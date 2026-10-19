#!/usr/bin/env python3
"""Request an independent Claude Code phase review and verify its source binding."""

import argparse
import copy
import re
import datetime
import hashlib
import json
import pathlib
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
EXCLUDED_DIRECTORIES = ("docs/reviews/", "docs/checks/")
EXCLUDED_FILES = {"docs/phase-status.json"}
SCOPE_BOUND_PHASES = frozenset({20, 21, 22, 23, 24, 25})
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


CATEGORIES = ("implementation", "code_quality", "supportability", "architecture", "feature_delivery")
RUBRIC_SCHEMA = copy.deepcopy(SCHEMA)
RUBRIC_SCHEMA["required"] += ["categories", "total"]
RUBRIC_SCHEMA["properties"].update({
    "categories": {"type": "object", "required": list(CATEGORIES), "additionalProperties": False,
                   "properties": {key: {"type": "number", "minimum": 0, "maximum": 10}
                                  for key in CATEGORIES}},
    "total": {"type": "number", "minimum": 0, "maximum": 50},
})


def uses_rubric(phase):
    # 90–92 are already accepted historical maintenance phases.
    return phase >= 19 and phase not in {90, 91, 92}


def review_destination(root, phase, attempt, chunk=None):
    if chunk is not None and not re.fullmatch(r"[a-z0-9]+(?:-[a-z0-9]+)*", chunk):
        raise ValueError("Chunk name must contain lowercase words separated by hyphens")
    suffix = f"-chunk-{chunk}" if chunk else ""
    return root / f"docs/reviews/phase-{phase:02d}{suffix}-attempt-{attempt:02d}.json"


def gate_decision(review, *, rubric=False, chunk=False, attempt=1, phase=None, completed_prior=0):
    if phase == 21 and not chunk and review["score"] < 9:
        return "rejected"
    if not rubric:
        return "accepted" if review["score"] >= 9 and not any(
            x["severity"] in {"blocker", "high"} for x in review["findings"]) else "rejected"
    if type(attempt) is not int or not 1 <= attempt <= 5:
        raise ValueError("The review loop allows at most five attempts")
    if any(x["severity"] in {"blocker", "high"} for x in review["findings"]):
        return "rejected"
    target = 42 if chunk else 46
    if review["total"] >= target and all(value >= 8 for value in review["categories"].values()):
        return "accepted"
    if (attempt == 5 and completed_prior == 4 and review["total"] > 40
            and all(value >= 8 for value in review["categories"].values())
            and (review["findings"] or review["limitations"])):
        return "accepted_with_deficiencies"
    return "rejected"


def verify_attempt_sequence(root, phase, attempt, chunk=None):
    """An exception cannot be obtained by jumping straight to attempt five."""
    completed = 0
    for previous in range(1, attempt):
        path = review_destination(root, phase, previous, chunk)
        failure = path.with_suffix(".failed.json")
        if not path.exists() and failure.exists():
            record = json.loads(failure.read_text())
            if (failure.is_symlink() or record.get("phase") != phase
                    or record.get("attempt") != previous or record.get("chunk") != chunk
                    or record.get("reviewer") != "Claude Code" or record.get("status") != "invocation_failed"
                    or record.get("reason") not in {"timeout", "process_error", "invalid_output", "launch_error"}):
                raise ValueError("Previous failed invocation is invalid")
            for key, suffix in (("prompt_sha256", ".prompt.md"), ("checks_sha256", ".checks.json")):
                evidence = path.with_suffix(suffix)
                if not evidence.is_file() or evidence.is_symlink() or sha256(evidence) != record.get(key):
                    raise ValueError("Previous failed invocation evidence has changed")
            continue
        if not path.is_file():
            raise ValueError("Previous review evidence has changed")
        record = json.loads(path.read_text())
        if (record.get("phase") != phase or record.get("attempt") != previous
                or record.get("chunk") != chunk or record.get("reviewer") != "Claude Code"
                or record.get("source_unchanged") is not True or path.is_symlink()):
            raise ValueError("Previous review attempt is invalid")
        validate_review(record.get("review"), rubric=True, chunk=bool(chunk), attempt=previous)
        for key, suffix in (("prompt_sha256", ".prompt.md"), ("cli_output_sha256", ".cli.json")):
            evidence = path.with_suffix(suffix)
            if not evidence.is_file() or evidence.is_symlink() or sha256(evidence) != record.get(key):
                raise ValueError("Previous review evidence has changed")
        envelope = json.loads(path.with_suffix(".cli.json").read_text())
        output = envelope.get("structured_output")
        if output is None:
            output = json.loads(envelope.get("result", "{}"))
        if envelope.get("is_error") is not False or output != record["review"]:
            raise ValueError("Previous review does not match actual Claude output")
        if chunk and phase in SCOPE_BOUND_PHASES - {20}:
            if not matches_declared_scope(root, phase, chunk, record.get("scope_evidence"), previous):
                raise ValueError("Previous review scope differs from its declared revision")
            prompt_scope = "Chunk scope artifacts: " + json.dumps(record["scope_evidence"]) + "."
            if prompt_scope not in path.with_suffix(".prompt.md").read_text():
                raise ValueError("Previous review scope differs from its preserved prompt")
        completed += 1
    return completed


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


def validate_review(review, *, rubric=False, chunk=False, attempt=1, completed_prior=0, phase=None):
    """Reject malformed or fabricated pass records before applying the gate."""
    if not isinstance(review, dict):
        raise ValueError("Claude must return a structured review object")
    if set(review) != set((RUBRIC_SCHEMA if rubric else SCHEMA)["required"]):
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
    if rubric:
        categories = review["categories"]
        if not isinstance(categories, dict) or set(categories) != set(CATEGORIES):
            raise ValueError("Exactly five review categories are required")
        for value in categories.values():
            if isinstance(value, bool) or not isinstance(value, (int, float)) or not 0 <= value <= 10:
                raise ValueError("Category scores must be numbers between 0 and 10")
        total = review["total"]
        if isinstance(total, bool) or not isinstance(total, (int, float)) or total != sum(categories.values()):
            raise ValueError("Review total must equal the sum of its five categories")
    return gate_decision(review, rubric=rubric, chunk=chunk, attempt=attempt, completed_prior=completed_prior, phase=phase) != "rejected"


def sha256(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def check_evidence(root, phase, source_sha256, path=None):
    """Accept the complete command set only when checked against unchanged inputs."""
    from check_phase import commands_for, verify_checkpoint_bindings

    path = path if path is not None else root / f"docs/checks/phase-{phase:02d}.json"
    if path.is_symlink():
        raise ValueError("Check evidence must be a regular file")
    report = json.loads(path.read_text())
    checks = report.get("checks", [])
    if not (report.get("phase") == phase and report.get("passed") is True
            and report.get("scope") is None
            and report.get("source_sha256") == source_sha256
            and report.get("source_unchanged") is True
            and isinstance(checks, list)
            and [item.get("command") for item in checks] == commands_for(phase)
            and all(type(item.get("exit_code")) is int and item["exit_code"] == 0 for item in checks)):
        raise ValueError("Phase checks are incomplete, stale, or failing; rerun tools/check_phase.py")
    # Historical accepted snapshots retain their own evidence hashes. Current
    # qualification must additionally prove the referenced files still match.
    if phase in (23, 24, 25) and source_sha256 == fingerprint(root):
        verify_checkpoint_bindings(checks, root)
    return path


def declared_chunk_scope(root, phase, name, attempt=None):
    status = json.loads((root / "docs/phase-status.json").read_text())
    phases = [item for item in status["phases"] if item["id"] == phase]
    declarations = [item for item in phases[0].get("chunks", []) if item.get("id") == name] if len(phases) == 1 else []
    if len(declarations) != 1:
        raise ValueError("Chunk review needs exactly one matching declaration")
    scope = declarations[0].get("scope_evidence")
    if phase in SCOPE_BOUND_PHASES - {20}:
        revisions = declarations[0].get("scope_revisions")
        if not isinstance(revisions, list) or not revisions:
            raise ValueError("Chunk declaration requires versioned scope revisions")
        starts = [revision.get("from_attempt") for revision in revisions if isinstance(revision, dict)]
        if (len(starts) != len(revisions) or any(type(n) is not int or n < 1 or n > 5 for n in starts)
                or starts != sorted(set(starts)) or starts[0] != 1
                or revisions[-1].get("scope_evidence") != scope):
            raise ValueError("Chunk scope revisions are invalid or differ from current scope")
        for revision in revisions:
            paths = revision.get("scope_evidence")
            if (not isinstance(paths, list) or not paths
                    or not all(isinstance(path, str) and path for path in paths)
                    or len(set(paths)) != len(paths)):
                raise ValueError("Chunk scope revisions require nonempty unique paths")
        if attempt is not None:
            scope = next(revision["scope_evidence"] for revision in reversed(revisions)
                         if revision["from_attempt"] <= attempt)
    if (not isinstance(scope, list) or not scope or not all(isinstance(path, str) and path for path in scope)
            or len(set(scope)) != len(scope)):
        raise ValueError("Chunk declaration needs nonempty, unique scope_evidence paths")
    return scope


def matches_declared_scope(root, phase, name, scope, attempt=None):
    declared = declared_chunk_scope(root, phase, name, attempt)
    return (isinstance(scope, list) and all(isinstance(path, str) for path in scope)
            and len(scope) == len(declared) and set(scope) == set(declared))


def verify_record(root, path, phase, source_sha256, chunk=None):
    """Bind the score to the preserved prompt, actual CLI result, and passing checks."""
    path = path.resolve()
    record = json.loads(path.read_text())
    attempt = record.get("attempt")
    if type(attempt) is not int or attempt < 1:
        return False
    expected = review_destination(root, phase, attempt, chunk)
    if path != expected or path.is_symlink():
        return False
    rubric = uses_rubric(phase)
    if rubric:
        validate_review(record.get("review"), rubric=True, chunk=bool(chunk), attempt=attempt, phase=phase)
        completed_prior = verify_attempt_sequence(root, phase, attempt, chunk)
        if (record.get("chunk") != chunk or record.get("rubric") != "five-category-v1"
                or record.get("decision") != gate_decision(record["review"], rubric=True,
                                                           chunk=bool(chunk), attempt=attempt, phase=phase, completed_prior=completed_prior)):
            return False
    if not (record.get("reviewer") == "Claude Code" and record.get("phase") == phase
            and record.get("source_sha256") == source_sha256
            and record.get("source_unchanged") is True and record.get("gate_passed") is True
            and validate_review(record.get("review"), rubric=rubric, chunk=bool(chunk), attempt=attempt,
                                completed_prior=completed_prior if rubric else 0, phase=phase)):
        return False
    for key, suffix in (("prompt_sha256", ".prompt.md"), ("cli_output_sha256", ".cli.json")):
        evidence = path.with_suffix(suffix)
        if not evidence.is_file() or evidence.is_symlink() or sha256(evidence) != record.get(key):
            return False
    if phase in SCOPE_BOUND_PHASES:
        scope = record.get("review_scope")
        allowed = {"chunk"} if chunk else ({"phase", "release_readiness"} if phase == 20 else {"phase"})
        if scope not in allowed or not path.with_suffix(".prompt.md").read_text().startswith(
                f"Review scope: {scope}.\n"):
            return False
        if chunk:
            evidence_scope = record.get("scope_evidence")
            if not matches_declared_scope(root, phase, chunk, evidence_scope, attempt):
                return False
            prompt_scope = "Chunk scope artifacts: " + json.dumps(evidence_scope) + "."
            if prompt_scope not in path.with_suffix(".prompt.md").read_text():
                return False
    envelope = json.loads(path.with_suffix(".cli.json").read_text())
    output = envelope.get("structured_output")
    if output is None:
        output = json.loads(envelope.get("result", "{}"))
    if envelope.get("is_error") is not False or output != record["review"]:
        return False
    checks = check_evidence(root, phase, source_sha256, path.with_suffix(".checks.json") if rubric else None)
    return sha256(checks) == record.get("checks_sha256")


def verify_release_readiness(root, path, source_sha256):
    """Publication helpers must require this accepted, current-source assessment."""
    record = json.loads(path.read_text())
    return (record.get("review_scope") == "release_readiness"
            and verify_record(root, path, 20, source_sha256))


def accepted_chunks(root, phase, declared, source_sha256, phase_review=None):
    """Verify every declared chunk's latest record before changing phase status."""
    if not isinstance(declared, list) or not declared:
        raise ValueError("Rubric phases must declare a nonempty chunks list")
    names = []
    for chunk in declared:
        if not isinstance(chunk, dict) or not isinstance(chunk.get("id"), str):
            raise ValueError("Each declared chunk needs an id")
        review_destination(root, phase, 1, chunk["id"])
        names.append(chunk["id"])
    reviewed_pattern = re.compile(rf"phase-{phase:02d}-chunk-([a-z0-9]+(?:-[a-z0-9]+)*)-attempt-[0-9]+\.json")
    reviewed = {match[1] for path in (root / "docs/reviews").glob(f"phase-{phase:02d}-chunk-*.json")
                if (match := reviewed_pattern.fullmatch(path.name))}
    if reviewed - set(names):
        raise ValueError("Reviewed chunks are missing declarations: " + ", ".join(sorted(reviewed - set(names))))
    accepted = copy.deepcopy(declared)
    seen = set()
    for chunk in accepted:
        if not isinstance(chunk, dict) or not isinstance(chunk.get("id"), str):
            raise ValueError("Each declared chunk needs an id")
        name = chunk["id"]
        review_destination(root, phase, 1, name)  # Validate before constructing a glob.
        if name in seen:
            raise ValueError("Declared chunk ids must be unique")
        seen.add(name)
        prefix = f"phase-{phase:02d}-chunk-{name}-attempt-"
        pattern = re.compile(re.escape(prefix) + r"([0-9]+)\.json")
        candidates = []
        for path in (root / "docs/reviews").glob(prefix + "*.json"):
            match = pattern.fullmatch(path.name)
            if match:
                candidates.append((int(match[1]), path))
        if not candidates:
            raise ValueError(f"Missing accepted chunk review: {name}")
        _, path = max(candidates, key=lambda item: (item[0], item[1].name))
        record = json.loads(path.read_text())
        if phase in SCOPE_BOUND_PHASES - {20} and not matches_declared_scope(
                root, phase, name, record.get("scope_evidence")):
            raise ValueError("Latest chunk review does not cover current declared scope")
        original_source = record.get("source_sha256")
        if original_source != source_sha256:
            if (record.get("attempt") != 5 or phase_review is None
                    or not verify_record(root, path, phase, original_source, name)
                    or not verify_record(root, phase_review, phase, source_sha256)):
                raise ValueError(f"Latest chunk review is rejected, stale, or invalid: {name}")
            coverage = json.loads(phase_review.read_text())
            if (coverage.get("decision") not in {"accepted", "accepted_with_deficiencies"}
                    or coverage.get("review_scope", "phase") != "phase"):
                raise ValueError("Chunk follow-up requires an accepted current complete-phase review")
            chunk["phase_followup_review"] = str(phase_review.relative_to(root))
            chunk["phase_followup_current_source_sha256"] = source_sha256
        elif not verify_record(root, path, phase, source_sha256, name):
            raise ValueError(f"Latest chunk review is rejected, stale, or invalid: {name}")
        if record.get("decision") not in {"accepted", "accepted_with_deficiencies"}:
            raise ValueError(f"Latest chunk review is not accepted: {name}")
        chunk.update({"status": record["decision"], "accepted_total": record["review"]["total"],
                      "accepted_categories": record["review"]["categories"],
                      "accepted_review": str(path.relative_to(root)),
                      "accepted_source_sha256": original_source,
                      "review_decision": record["decision"]})
    return accepted


def validate_predecessors(root, status, phase):
    for item in status["phases"]:
        if item["id"] >= phase:
            continue
        if item["status"] != "complete" or not item.get("reviews"):
            raise ValueError("Earlier phases have not passed their gates")
        review_path = root / item["reviews"][-1]
        if not verify_record(root, review_path, item["id"], item.get("accepted_source_sha256")):
            raise ValueError("An earlier phase's accepted review evidence is invalid")


def next_phase(phases, current):
    """Support separately numbered maintenance phases without bypassing a gate."""
    if phases[current].get("final_in_sequence") is True:
        return None
    return min((number for number in phases if number > current
                and phases[number].get("status") != "complete"), default=None)


def main(argv=None, root=None):
    root = ROOT if root is None else root.resolve()
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--phase", type=int, required=True)
    parser.add_argument("--attempt", type=int, default=1)
    parser.add_argument("--chunk", help="Named chunk review; cannot advance the phase")
    parser.add_argument("--scope-evidence", nargs="*", action="extend", default=[], help="Chunk artifact paths for the reviewer")
    parser.add_argument("--release-readiness", action="store_true",
                        help="Phase 20 prepublication readiness review; cannot complete the phase")
    parser.add_argument("--verify", type=pathlib.Path)
    parser.add_argument("--advance", action="store_true",
                        help="Advance only after verifying an accepted current-source review")
    args = parser.parse_args(argv)
    status = json.loads((root / "docs/phase-status.json").read_text())
    phases = {item["id"]: item for item in status["phases"]}
    if args.phase not in phases or args.attempt < 1:
        raise ValueError("Unknown phase or invalid attempt")
    rubric = uses_rubric(args.phase)
    completed_prior = 0
    if args.release_readiness and (args.phase != 20 or args.chunk or args.verify or args.advance):
        raise ValueError("Release readiness requires a new phase 20 review without chunk, verify, or advance")
    if args.chunk and (not rubric or args.advance):
        raise ValueError("Chunk reviews require the new rubric and cannot advance a phase")
    if rubric and not args.verify:
        if not 1 <= args.attempt <= 5:
            raise ValueError("The review loop allows at most five attempts")
        completed_prior = verify_attempt_sequence(root, args.phase, args.attempt, args.chunk)
    if args.advance and not args.verify:
        raise ValueError("Advancement requires --verify with the actual Claude review artifact")
    validate_predecessors(root, status, args.phase)
    before = fingerprint(root)
    if args.verify:
        review_path = args.verify if args.verify.is_absolute() else root / args.verify
        valid = verify_record(root, review_path, args.phase, before, args.chunk)
        print(json.dumps({"phase": args.phase, "valid_for_current_source": valid}))
        if valid and args.advance:
            if status["active_phase"] != args.phase:
                raise ValueError("Only the active phase can advance")
            if args.phase == 20 and json.loads(review_path.read_text()).get("review_scope") != "phase":
                raise ValueError("Release readiness does not complete delivery; obtain a final phase review")
            if rubric:
                phases[args.phase]["chunks"] = accepted_chunks(
                    root, args.phase, phases[args.phase].get("chunks"), before, phase_review=review_path)
            if args.phase == 20:
                from release_delivery import verify as verify_delivery
                verify_delivery(root, before)
            phases[args.phase]["status"] = "complete"
            record = json.loads(review_path.read_text())
            phases[args.phase]["reviews"].append(str(review_path.relative_to(root)))
            phases[args.phase]["accepted_source_sha256"] = before
            phases[args.phase]["accepted_score"] = record["review"]["score"]
            if rubric:
                phases[args.phase]["accepted_total"] = record["review"]["total"]
                phases[args.phase]["accepted_categories"] = record["review"]["categories"]
                phases[args.phase]["review_decision"] = record["decision"]
            milestones = phases[args.phase].setdefault("milestones", {})
            milestones["implemented_scope"] = {
                "status": "accepted_by_review", "evidence": [str(review_path.relative_to(root))]}
            milestones["independent_verification"] = {
                "status": "passed", "evidence": [str(review_path.with_suffix(".checks.json").relative_to(root))
                                                   if rubric else f"docs/checks/phase-{args.phase:02d}.json"],
                "source_sha256": before}
            milestones["claude_review"] = {
                "status": "accepted", "score": record["review"]["score"],
                "evidence": [str(review_path.relative_to(root))]}
            status["active_phase"] = next_phase(phases, args.phase)
            if status["active_phase"] is not None:
                phases[status["active_phase"]]["status"] = "in_progress"
            (root / "docs/phase-status.json").write_text(json.dumps(status, indent=2) + "\n")
        return 0 if valid else 1
    if status["active_phase"] != args.phase:
        raise ValueError("Review the active phase")
    checks = check_evidence(root, args.phase, before)
    destination = review_destination(root, args.phase, args.attempt, args.chunk)
    for scoped_path in args.scope_evidence:
        candidate = (root / scoped_path).resolve()
        if not candidate.is_relative_to(root) or not candidate.exists():
            raise ValueError("Chunk scope evidence must exist inside the repository")
    if destination.exists() or destination.with_suffix(".prompt.md").exists():
        raise ValueError("Review attempt already exists; preserve it and use a new attempt")
    if args.chunk and not args.scope_evidence:
        raise ValueError("Chunk reviews require --scope-evidence with scoped artifact paths")
    if args.phase in SCOPE_BOUND_PHASES and args.chunk and not matches_declared_scope(
            root, args.phase, args.chunk, args.scope_evidence):
        raise ValueError("Supplied chunk scope differs from its declared scope_evidence")
    if rubric:
        snapshot = destination.with_suffix(".checks.json")
        snapshot.parent.mkdir(parents=True, exist_ok=True)
        snapshot.write_bytes(checks.read_bytes())
        checks = snapshot
    followup = phases[args.phase].get("plan") or status.get("active_followup_plan") or "docs/implementation-plan.md"
    focus = f"\nActive follow-up plan: {followup}. Read it and this phase's scope_evidence.\n" if followup else ""
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
{focus}
Reviewed source SHA256: {before}.
Passing check evidence: {checks.relative_to(root)}, SHA256 {sha256(checks)}.
Scope evidence: {json.dumps(phases[args.phase].get('scope_evidence', []))}.
For a vendor follow-up phase, focus on the active milestones and relevant cumulative
regressions. Generated model correctness is assessed through the compiler, selected
generated declarations and independent fixture/check evidence. Later-phase operations
marked planned are not a missing deliverable of the contracts/models phase. Historical
release evidence does not establish new-source acceptance; avoid re-auditing unrelated
historical infrastructure when evaluating this scoped change.
Return the requested structured review with score, summary, findings, evidence, limitations.
"""
    if rubric:
        prompt += f"""
The maintainer's current five-category rubric supersedes the older /10 threshold.
Review {'chunk ' + args.chunk if args.chunk else 'the complete phase'}; attempt {args.attempt}/5.
Score implementation, code_quality, supportability, architecture and feature_delivery
separately from 0 to 10. Return categories and their exact sum as total (/50).
Retain score (/10) as a legacy summary only; it does not decide acceptance.
Each category must be >=8; chunks require >=42/50 (aim >=45); phases require >=46/50.
On attempt five only, a total strictly above 40 may proceed with every category >=8
and documented remaining findings or limitations. Blocker/high findings still block acceptance. Do not inflate scores.
Chunk scope artifacts: {json.dumps(args.scope_evidence)}.
"""
    if args.phase == 21 and not args.chunk:
        prompt += "\nThis phase is also a plan gate: score must be >=9/10 in addition to the five-category rubric.\n"
    if rubric and not args.chunk:
        prompt += """
Inspect the latest prior phase findings, phase-status review_findings_disposition,
follow-up scope_evidence, and all declared chunks. Where an accepted fifth chunk
review predates phase-requested corrections, explicitly assess those corrections
and their regressions against the complete current phase. The original chunk
review remains historical evidence; this phase review must cover current source.
"""
    if args.release_readiness:
        prompt += """
This is the prepublication RELEASE READINESS assessment, not phase completion.
Assess implementation, code quality, supportability, architecture and feature delivery
against the qualified package and readiness to publish. The phase threshold remains 46/50.
Actual registry publication, remote tag and terminal CI are subsequent delivery obligations;
identify them as pending limitations, not completed outcomes. This record cannot advance
phase 20. A fresh complete-phase review after delivery and both accepted chunks is required.
"""
    if args.phase in SCOPE_BOUND_PHASES:
        review_scope = "release_readiness" if args.release_readiness else ("chunk" if args.chunk else "phase")
        prompt = f"Review scope: {review_scope}.\n" + prompt
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.with_suffix(".prompt.md").write_text(prompt)
    local = root / ".local/reviews"
    local.mkdir(parents=True, exist_ok=True, mode=0o700)
    def diagnostics(stdout, stderr):
        for suffix, output in (("json", stdout), ("stderr.txt", stderr)):
            path = local / f"{destination.stem}.{suffix}"
            path.write_bytes(output if isinstance(output, bytes) else (output or "").encode())
            path.chmod(0o600)
    def failed_invocation(reason):
        # Sanitized public receipt; raw model/process output remains private.
        failure = {"reviewer": "Claude Code", "phase": args.phase, "attempt": args.attempt,
                   "chunk": args.chunk, "status": "invocation_failed", "reason": reason,
                   "source_sha256": before, "prompt_sha256": sha256(destination.with_suffix(".prompt.md")),
                   "checks_sha256": sha256(checks)}
        destination.with_suffix(".failed.json").write_text(json.dumps(failure, indent=2) + "\n")
    try:
        result = subprocess.run(
            ["claude", "-p", "--restricted", "--permission-mode", "plan", "--tools", "Read,Glob,Grep",
             "--strict-mcp-config", "--mcp-config", '{"mcpServers":{}}',
             "--max-budget-usd", "25", "--output-format", "json", "--json-schema", json.dumps(RUBRIC_SCHEMA if rubric else SCHEMA)],
            cwd=root, input=prompt, text=True, capture_output=True, timeout=1800,
        )
    except subprocess.TimeoutExpired as error:
        diagnostics(error.stdout, error.stderr)
        failed_invocation("timeout")
        raise RuntimeError("Claude Code review timed out; prompt and private diagnostics preserved") from None
    except OSError:
        failed_invocation("launch_error")
        raise
    diagnostics(result.stdout, result.stderr)
    if result.returncode:
        failed_invocation("process_error")
        raise RuntimeError(f"Claude Code exited {result.returncode}; see local review output")
    try:
        envelope = json.loads(result.stdout)
        if envelope.get("is_error") is not False:
            raise RuntimeError("Claude Code returned an error; see local review output")
        review = envelope.get("structured_output")
        if review is None:
            review = json.loads(envelope.get("result", "{}"))
        passed = validate_review(review, rubric=rubric, chunk=bool(args.chunk), attempt=args.attempt, completed_prior=completed_prior, phase=args.phase)
    except (ValueError, TypeError, KeyError, RuntimeError):
        failed_invocation("invalid_output")
        raise
    decision = gate_decision(review, rubric=rubric, chunk=bool(args.chunk),
                             attempt=args.attempt, phase=args.phase, completed_prior=completed_prior)
    passed = passed and decision != "rejected"
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
    if rubric:
        record.update({"rubric": "five-category-v1", "chunk": args.chunk,
                       "decision": decision})
    if args.phase in SCOPE_BOUND_PHASES:
        record["review_scope"] = "release_readiness" if args.release_readiness else ("chunk" if args.chunk else "phase")
        if args.chunk:
            record["scope_evidence"] = args.scope_evidence
    destination.write_text(json.dumps(record, indent=2) + "\n")
    print(json.dumps({"phase": args.phase, "score": review["score"], "total": review.get("total"), "gate_passed": record["gate_passed"],
                      "review": str(destination.relative_to(root)), "summary": review["summary"]}))
    return 0 if record["gate_passed"] else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (ValueError, RuntimeError, OSError, subprocess.SubprocessError) as error:
        print(str(error), file=sys.stderr)
        sys.exit(2)
