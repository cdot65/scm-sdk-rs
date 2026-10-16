#!/usr/bin/env python3
"""Request an independent Claude Code phase review and verify its source binding."""

import argparse
import datetime
import hashlib
import json
import pathlib
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
EXCLUDED_DIRECTORIES = ("docs/reviews/", "docs/checks/")
EXCLUDED_FILES = {"docs/phase-status.json"}
SCHEMA = {
    "type": "object",
    "required": ["score", "summary", "findings", "evidence", "limitations"],
    "additionalProperties": False,
    "properties": {
        "score": {"type": "number", "minimum": 1, "maximum": 10},
        "summary": {"type": "string"},
        "findings": {
            "type": "array",
            "items": {
                "type": "object",
                "required": ["severity", "path", "line", "description", "required_fix"],
                "additionalProperties": False,
                "properties": {
                    "severity": {"type": "string", "enum": ["blocker", "high", "medium", "low"]},
                    "path": {"type": "string"},
                    "line": {"type": "integer", "minimum": 0},
                    "description": {"type": "string"},
                    "required_fix": {"type": "string"},
                },
            },
        },
        "evidence": {"type": "array", "minItems": 1, "items": {"type": "string", "minLength": 1}},
        "limitations": {"type": "array", "items": {"type": "string"}},
    },
}


def fingerprint(root=None):
    """Hash all tracked and nonignored inputs except review bookkeeping."""
    root = ROOT if root is None else root
    output = subprocess.check_output(
        ["git", "ls-files", "--cached", "--others", "--exclude-standard", "-z"], cwd=root
    )
    digest = hashlib.sha256()
    paths = sorted(set(output.decode().split("\0")) - {""})
    for name in paths:
        if name in EXCLUDED_FILES or name.startswith(EXCLUDED_DIRECTORIES):
            continue
        path = root / name
        if path.is_symlink():
            raise ValueError(f"Review inputs must be regular files: {name}")
        if not path.is_file():
            raise ValueError(f"Missing review input: {name}")
        digest.update(name.encode() + b"\0" + hashlib.sha256(path.read_bytes()).digest())
    return digest.hexdigest()


def validate_review(review):
    """Reject malformed or fabricated pass records before applying the gate."""
    if not isinstance(review, dict):
        raise ValueError("Claude must return a structured review object")
    if set(review) != set(SCHEMA["required"]):
        raise ValueError("Unexpected review fields")
    score = review["score"]
    if isinstance(score, bool) or not isinstance(score, (int, float)) or not 1 <= score <= 10:
        raise ValueError("Review score must be a number between 1 and 10")
    if not isinstance(review["summary"], str) or not review["summary"].strip():
        raise ValueError("Review summary is required")
    for key in ("evidence", "limitations"):
        if not isinstance(review[key], list) or not all(isinstance(x, str) and x.strip() for x in review[key]):
            raise ValueError(f"Invalid {key}")
    if not review["evidence"]:
        raise ValueError("Review must identify evidence")
    if not isinstance(review["findings"], list):
        raise ValueError("Invalid findings")
    expected = {"severity", "path", "line", "description", "required_fix"}
    for finding in review["findings"]:
        if not isinstance(finding, dict) or set(finding) != expected:
            raise ValueError("Malformed finding")
        if finding["severity"] not in {"blocker", "high", "medium", "low"}:
            raise ValueError("Invalid finding severity")
        if isinstance(finding["line"], bool) or not isinstance(finding["line"], int) or finding["line"] < 0:
            raise ValueError("Invalid finding line")
        if not all(isinstance(finding[k], str) and finding[k].strip()
                   for k in ("path", "description", "required_fix")):
            raise ValueError("Invalid finding text")
    return score >= 9 and not any(x["severity"] in {"blocker", "high"} for x in review["findings"])


def sha256(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def check_evidence(root, phase, source_sha256):
    """Accept the complete command set only when checked against unchanged inputs."""
    from check_phase import commands_for

    path = root / f"docs/checks/phase-{phase:02d}.json"
    report = json.loads(path.read_text())
    checks = report.get("checks", [])
    if not (report.get("phase") == phase and report.get("passed") is True
            and report.get("source_sha256") == source_sha256
            and report.get("source_unchanged") is True
            and isinstance(checks, list)
            and [item.get("command") for item in checks] == commands_for(phase)
            and all(type(item.get("exit_code")) is int and item["exit_code"] == 0 for item in checks)):
        raise ValueError("Phase checks are incomplete, stale, or failing; rerun tools/check_phase.py")
    return path


def verify_record(root, path, phase, source_sha256):
    """Bind the score to the preserved prompt, actual CLI result, and passing checks."""
    path = path.resolve()
    record = json.loads(path.read_text())
    attempt = record.get("attempt")
    if type(attempt) is not int or attempt < 1:
        return False
    expected = root / f"docs/reviews/phase-{phase:02d}-attempt-{attempt:02d}.json"
    if path != expected or path.is_symlink():
        return False
    if not (record.get("reviewer") == "Claude Code" and record.get("phase") == phase
            and record.get("source_sha256") == source_sha256
            and record.get("source_unchanged") is True and record.get("gate_passed") is True
            and validate_review(record.get("review"))):
        return False
    for key, suffix in (("prompt_sha256", ".prompt.md"), ("cli_output_sha256", ".cli.json")):
        evidence = path.with_suffix(suffix)
        if not evidence.is_file() or evidence.is_symlink() or sha256(evidence) != record.get(key):
            return False
    envelope = json.loads(path.with_suffix(".cli.json").read_text())
    output = envelope.get("structured_output")
    if output is None:
        output = json.loads(envelope.get("result", "{}"))
    if envelope.get("is_error") is not False or output != record["review"]:
        return False
    checks = check_evidence(root, phase, source_sha256)
    return sha256(checks) == record.get("checks_sha256")


def validate_predecessors(root, status, phase):
    for item in status["phases"]:
        if item["id"] >= phase:
            continue
        if item["status"] != "complete" or not item.get("reviews"):
            raise ValueError("Earlier phases have not passed their gates")
        review_path = root / item["reviews"][-1]
        if not verify_record(root, review_path, item["id"], item.get("accepted_source_sha256")):
            raise ValueError("An earlier phase's accepted review evidence is invalid")


def main(argv=None, root=None):
    root = ROOT if root is None else root.resolve()
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--phase", type=int, required=True)
    parser.add_argument("--attempt", type=int, default=1)
    parser.add_argument("--verify", type=pathlib.Path)
    parser.add_argument("--advance", action="store_true",
                        help="Advance only after verifying an accepted current-source review")
    args = parser.parse_args(argv)
    status = json.loads((root / "docs/phase-status.json").read_text())
    phases = {item["id"]: item for item in status["phases"]}
    if args.phase not in phases or args.attempt < 1:
        raise ValueError("Unknown phase or invalid attempt")
    if args.advance and not args.verify:
        raise ValueError("Advancement requires --verify with the actual Claude review artifact")
    validate_predecessors(root, status, args.phase)
    before = fingerprint(root)
    if args.verify:
        review_path = args.verify if args.verify.is_absolute() else root / args.verify
        valid = verify_record(root, review_path, args.phase, before)
        print(json.dumps({"phase": args.phase, "valid_for_current_source": valid}))
        if valid and args.advance:
            if status["active_phase"] != args.phase:
                raise ValueError("Only the active phase can advance")
            phases[args.phase]["status"] = "complete"
            record = json.loads(review_path.read_text())
            phases[args.phase]["reviews"].append(str(review_path.relative_to(root)))
            phases[args.phase]["accepted_source_sha256"] = before
            phases[args.phase]["accepted_score"] = record["review"]["score"]
            milestones = phases[args.phase].setdefault("milestones", {})
            milestones["implemented_scope"] = {
                "status": "accepted_by_review", "evidence": [str(review_path.relative_to(root))]}
            milestones["independent_verification"] = {
                "status": "passed", "evidence": [f"docs/checks/phase-{args.phase:02d}.json"],
                "source_sha256": before}
            milestones["claude_review"] = {
                "status": "accepted", "score": record["review"]["score"],
                "evidence": [str(review_path.relative_to(root))]}
            status["active_phase"] = args.phase + 1 if args.phase + 1 in phases else None
            if status["active_phase"] is not None:
                phases[status["active_phase"]]["status"] = "in_progress"
            (root / "docs/phase-status.json").write_text(json.dumps(status, indent=2) + "\n")
        return 0 if valid else 1
    if status["active_phase"] != args.phase:
        raise ValueError("Review the active phase")
    checks = check_evidence(root, args.phase, before)
    destination = root / f"docs/reviews/phase-{args.phase:02d}-attempt-{args.attempt:02d}.json"
    if destination.exists() or destination.with_suffix(".prompt.md").exists():
        raise ValueError("Review attempt already exists; preserve it and use a new attempt")
    prompt = f"""Independently review phase {args.phase}: {phases[args.phase]['name']}.
Read AGENTS.md, CLAUDE.md, docs/PHASES.md, docs/implementation-plan.md and
docs/phase-status.json, then inspect all actual artifacts relevant to this phase.
Assess functionality, security, contract completeness, maintainability and check evidence.
Give a candid score from 1 to 10. The threshold is 9, but do not inflate the score to
meet it. Report concrete required fixes with paths and lines. Judge this phase's
acceptance criteria, not unimplemented later phases. Verify evidence rather than
trusting completion prose. You have read-only Read/Glob/Grep tools: inspect recorded
checks and distinguish unexecuted checks. Do not read any credential/tenant-registration
files or anything outside this repository. .local/typescript is a frozen nonsecret source
reference and may be inspected. Do not modify files or perform network calls.
Reviewed source SHA256: {before}.
Passing check evidence: {checks.relative_to(root)}, SHA256 {sha256(checks)}.
Return the requested structured review with score, summary, findings, evidence, limitations.
"""
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.with_suffix(".prompt.md").write_text(prompt)
    local = root / ".local/reviews"
    local.mkdir(parents=True, exist_ok=True, mode=0o700)
    def diagnostics(stdout, stderr):
        for suffix, output in (("json", stdout), ("stderr.txt", stderr)):
            path = local / f"phase-{args.phase:02d}-attempt-{args.attempt:02d}.{suffix}"
            path.write_bytes(output if isinstance(output, bytes) else (output or "").encode())
            path.chmod(0o600)
    try:
        result = subprocess.run(
            ["claude", "-p", "--restricted", "--permission-mode", "plan", "--tools", "Read,Glob,Grep",
             "--strict-mcp-config", "--mcp-config", '{"mcpServers":{}}',
             "--max-budget-usd", "25", "--output-format", "json", "--json-schema", json.dumps(SCHEMA)],
            cwd=root, input=prompt, text=True, capture_output=True, timeout=1800,
        )
    except subprocess.TimeoutExpired as error:
        diagnostics(error.stdout, error.stderr)
        raise RuntimeError("Claude Code review timed out; prompt and private diagnostics preserved") from None
    diagnostics(result.stdout, result.stderr)
    if result.returncode:
        raise RuntimeError(f"Claude Code exited {result.returncode}; see local review output")
    envelope = json.loads(result.stdout)
    if envelope.get("is_error") is not False:
        raise RuntimeError("Claude Code returned an error; see local review output")
    review = envelope.get("structured_output")
    if review is None:
        review = json.loads(envelope.get("result", "{}"))
    passed = validate_review(review)
    unchanged = before == fingerprint(root)
    destination.with_suffix(".cli.json").write_text(result.stdout)
    record = {
        "reviewer": "Claude Code", "phase": args.phase, "attempt": args.attempt,
        "reviewed_at": datetime.datetime.now(datetime.timezone.utc).isoformat(),
        "source_sha256": before, "source_unchanged": unchanged,
        "checks_sha256": sha256(checks),
        "prompt_sha256": sha256(destination.with_suffix(".prompt.md")),
        "cli_output_sha256": sha256(destination.with_suffix(".cli.json")),
        "gate_passed": passed and unchanged, "review": review,
    }
    destination.write_text(json.dumps(record, indent=2) + "\n")
    print(json.dumps({"phase": args.phase, "score": review["score"], "gate_passed": record["gate_passed"],
                      "review": str(destination.relative_to(root)), "summary": review["summary"]}))
    return 0 if record["gate_passed"] else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (ValueError, RuntimeError, OSError, subprocess.SubprocessError) as error:
        print(str(error), file=sys.stderr)
        sys.exit(2)
