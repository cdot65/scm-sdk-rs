#!/usr/bin/env python3
"""Request and record a source-bound Codex CLI review for phases 29–31.

The maintainer selected Codex as the independent reviewer for the Portkey MCP
tags track (docs/portkey-mcp-tags-plan.md). Each chunk and phase review scores
1–10; acceptance needs at least 9 and no blocker/high finding. Every attempt
retains its prompt, check snapshot, Codex final message and outcome. Codex runs
in its read-only sandbox from the repository root. Raw event logs stay under
ignored .local/reviews; the committed record binds their SHA-256.
"""
import argparse
import datetime
import fcntl
import os
import hashlib
import json
import pathlib
import shutil
import subprocess
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from check_phase import commands_for  # noqa: E402
from review_phase import SCHEMA, check_evidence, fingerprint, validate_review  # noqa: E402

ROOT = pathlib.Path(__file__).resolve().parents[1]
PHASES = (29, 30, 31)
REVIEWER = 'Codex CLI'
CODEX = 'codex'
# Chunk identities are reviewed source: phase-status.json is excluded from the
# fingerprint, so its declarations alone cannot define which reviews are required.
REQUIRED_CHUNKS = {29: ('plan-and-provenance',), 30: ('models-and-validation', 'clients-and-wire'),
                   31: ('release-readiness', 'remote-delivery', 'docs-site')}
MINIMUM_SCORE = 9
MAX_ATTEMPTS = 5
TIMEOUT_SECONDS = 3600
# The chunk reviewed before publication stays bound to the published source; later
# governance-only successors are verified by tools/release_delivery.py.
PUBLICATION_CHUNKS = {31: 'release-readiness'}
# Chunks whose implementation is the separately deployed documentation site.
SITE_BINDINGS = {(31, 'docs-site'): '0.2.5'}
# Attempts before enforcement was introduced remain valid unbound (rejected) history.
SITE_BINDING_FROM_ATTEMPT = {(31, 'docs-site'): 3}
FAILURE_REASONS = {'timeout', 'process_error', 'invalid_output', 'launch_error', 'interrupted'}
# Codex structured output requires every property to be required and closed.
OUTPUT_SCHEMA = json.loads(json.dumps(SCHEMA))
for _definition in (OUTPUT_SCHEMA, OUTPUT_SCHEMA['properties']['findings']['items']):
    _definition['required'] = list(_definition['properties'])


def sha256(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def destination(root, phase, attempt, chunk=None):
    suffix = f'-chunk-{chunk}' if chunk else ''
    return root / f'docs/reviews/phase-{phase:02d}{suffix}-codex-attempt-{attempt:02d}.json'


def decide(review):
    if any(finding['severity'] in {'blocker', 'high'} for finding in review['findings']):
        return 'rejected'
    return 'accepted' if review['score'] >= MINIMUM_SCORE else 'rejected'


def phase_row(status, phase):
    row = next((item for item in status['phases'] if item['id'] == phase), None)
    if row is None or row.get('review_policy', {}).get('reviewer') != 'codex':
        raise ValueError('Phase is not registered for Codex review')
    if row['review_policy'].get('minimum_score') != MINIMUM_SCORE:
        raise ValueError('Codex review threshold differs from the maintainer instruction')
    return row


def scope_for(row, chunk):
    if chunk is not None and chunk not in REQUIRED_CHUNKS[row['id']]:
        raise ValueError('Undeclared chunk')
    if chunk is None:
        return row['scope_evidence']
    match = next((item for item in row.get('chunks', []) if item['id'] == chunk), None)
    if match is None:
        raise ValueError('Undeclared chunk')
    return match['scope_evidence']


def slot_files(root, phase, attempt, chunk):
    """Every artifact of one attempt slot shares its stem; any one occupies the slot."""
    stem = destination(root, phase, attempt, chunk).stem
    directory = root / 'docs/reviews'
    return sorted(path for path in directory.glob(stem + '.*')) if directory.is_dir() else []


def lock_path(root, phase, attempt, chunk):
    return root / '.local/reviews' / (destination(root, phase, attempt, chunk).stem + '.lock')


def verify_attempt(root, phase, attempt, chunk):
    """Return the verified record of one completed or failed attempt."""
    path = destination(root, phase, attempt, chunk)
    failed = path.with_suffix('.failed.json')
    if path.is_file() == failed.is_file():
        if not path.is_file() and path.with_suffix('.prompt.md').is_file():
            raise ValueError(f'Attempt {attempt} is in flight or was interrupted; record it with --mark-interrupted {attempt}')
        raise ValueError('Review attempt evidence is missing or ambiguous')
    record_path = path if path.is_file() else failed
    if record_path.is_symlink():
        raise ValueError('Review evidence must be a regular file')
    record = json.loads(record_path.read_text())
    if (record.get('phase'), record.get('attempt'), record.get('chunk'), record.get('reviewer')) != (phase, attempt, chunk, REVIEWER):
        raise ValueError('Review attempt record is invalid')
    prompt, checks = path.with_suffix('.prompt.md'), path.with_suffix('.checks.json')
    if not prompt.is_file() or prompt.is_symlink() or sha256(prompt) != record.get('prompt_sha256'):
        raise ValueError('Previous review evidence has changed')
    snapshot_missing = record.get('checks_sha256') is None
    if record_path == failed:
        if record.get('status') != 'invocation_failed' or record.get('reason') not in FAILURE_REASONS:
            raise ValueError('Failed review record is invalid')
        # Only an interruption between slot reservation and snapshot copy lacks checks.
        if snapshot_missing:
            if record['reason'] != 'interrupted' or checks.exists():
                raise ValueError('Failed review record omits existing check evidence')
            return record
    if snapshot_missing or not checks.is_file() or checks.is_symlink() or sha256(checks) != record.get('checks_sha256'):
        raise ValueError('Previous review evidence has changed')
    if record_path == failed:
        # Failed invocations never confer acceptance; their evidence only needs integrity.
        return record
    verify_bindings(prompt, checks, record)
    output = path.with_suffix('.codex.json')
    if not output.is_file() or output.is_symlink() or sha256(output) != record.get('output_sha256'):
        raise ValueError('Previous Codex output has changed')
    if json.loads(output.read_text()) != record.get('review'):
        raise ValueError('Previous review differs from the actual Codex output')
    validate_review(record['review'])
    unchanged = record.get('source_unchanged') is True and record.get('site_unchanged', True) is True
    expected = decide(record['review']) if unchanged else 'rejected'
    if record.get('status') != 'completed' or record.get('decision') != expected:
        raise ValueError('Review decision does not follow from its preserved output')
    return record


def verify_bindings(prompt, checks, record):
    """The preserved prompt and check snapshot themselves identify the record's source and scope."""
    phase, chunk, attempt, source = record['phase'], record['chunk'], record['attempt'], record.get('source_sha256')
    text = prompt.read_text()
    target = f'chunk `{chunk}` of phase {phase} ' if chunk else f'the complete phase {phase} '
    required = (f'You are the independent reviewer for {target}', f'Review attempt {attempt} of at most {MAX_ATTEMPTS}.',
                f'Reviewed source fingerprint: {source}.', 'Scope artifacts: ' + json.dumps(record.get('scope_evidence')) + '.')
    if (phase, chunk) in SITE_BINDINGS and attempt >= SITE_BINDING_FROM_ATTEMPT[(phase, chunk)]:
        site = record.get('site_binding')
        if not isinstance(site, dict) or record.get('site_unchanged') not in (True, False):
            raise ValueError('Site-bound review lacks its recorded site binding')
        required += (site_prompt(site),)
    elif 'site_binding' in record or 'site_unchanged' in record:
        raise ValueError('Unexpected site binding on a source-only review')
    if not isinstance(source, str) or any(fragment not in text for fragment in required):
        raise ValueError('Preserved prompt does not bind the recorded source and scope')
    report = json.loads(checks.read_text())
    rows = report.get('checks')
    if not (report.get('phase') == phase and report.get('passed') is True and report.get('source_sha256') == source
            and report.get('source_unchanged') is True and report.get('scope') is None and isinstance(rows, list) and rows
            and all(type(row.get('exit_code')) is int and row['exit_code'] == 0 for row in rows)):
        raise ValueError('Preserved check snapshot does not pass for the recorded source')
    return report


def verify_prior_attempts(root, phase, attempt, chunk):
    """Attempts are sequential and their preserved evidence is unchanged."""
    return [verify_attempt(root, phase, previous, chunk) for previous in range(1, attempt)]


def next_attempt(root, phase, chunk):
    attempt = 1
    while slot_files(root, phase, attempt, chunk):
        attempt += 1
    if attempt > MAX_ATTEMPTS:
        raise ValueError('The five-attempt review budget is exhausted; explicit maintainer authorization is required')
    return attempt


def latest_attempt(root, phase, chunk):
    attempt = 0
    while slot_files(root, phase, attempt + 1, chunk):
        attempt += 1
    return attempt


def reserve(path, content):
    """Atomically claim an attempt slot; an existing prompt is never overwritten."""
    descriptor = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o644)
    with os.fdopen(descriptor, 'w') as handle:
        handle.write(content)


def verify_chunk_acceptance(root, row, source):
    """Each required chunk's latest attempt is an accepted, unchanged review of this source and scope."""
    declared = row.get('chunks')
    if not isinstance(declared, list) or tuple(item.get('id') for item in declared) != REQUIRED_CHUNKS[row['id']]:
        raise ValueError('Declared chunks differ from the reviewed required chunk set')
    identity = published_identity(root, row['id'])
    for chunk in declared:
        latest = latest_attempt(root, row['id'], chunk['id'])
        expected = destination(root, row['id'], latest, chunk['id']).relative_to(root).as_posix() if latest else None
        if chunk.get('status') != 'accepted' or not latest or chunk.get('accepted_review') != expected:
            raise ValueError('Every declared chunk must have an accepted latest review: ' + chunk['id'])
        record = verify_attempt(root, row['id'], latest, chunk['id'])
        snapshot = json.loads(destination(root, row['id'], latest, chunk['id']).with_suffix('.checks.json').read_text())
        commands = [item['command'] for item in snapshot['checks']]
        expected_source = source
        if identity and PUBLICATION_CHUNKS.get(row['id']) == chunk['id']:
            # Accepted before publication: bound to the published source and its
            # pre-publication gate (clean packages, then the readiness receipt last).
            expected_source = identity[1]
            if not (commands and commands[-1][:2] == ['python', 'scripts/check-access-release.py']
                    and any('scripts/check-management-package.py' in command for command in commands)):
                raise ValueError('Publication chunk acceptance lacks its pre-publication gate: ' + chunk['id'])
        elif commands != commands_for(row['id']):
            raise ValueError('Chunk acceptance used a different check command set: ' + chunk['id'])
        if (row['id'], chunk['id']) in SITE_BINDINGS:
            binding, digest = verify_site(root, row['id'], chunk['id'])
            if record.get('site_unchanged') is not True or record.get('site_binding') != site_summary(binding, digest):
                raise ValueError('Site-bound chunk acceptance differs from the current site binding: ' + chunk['id'])
        if (record.get('decision') != 'accepted' or record.get('source_sha256') != expected_source
                or chunk.get('accepted_source_sha256') != expected_source or record.get('scope_evidence') != chunk['scope_evidence']
                or chunk.get('accepted_score') != record['review']['score']):
            raise ValueError('Chunk acceptance is stale, superseded or inconsistent: ' + chunk['id'])


def published_identity(root, phase):
    """(release commit, published source fingerprint) once the release is published, else None."""
    if phase not in RELEASE_PHASES:
        return None
    from release_delivery import published_source
    return published_source(root, RELEASE_PHASES[phase])


def verify_site(root, phase, chunk):
    import site_binding
    return site_binding.check(root, SITE_BINDINGS[(phase, chunk)])


def site_summary(binding, digest):
    return {'version': binding['version'], 'siteCommit': binding['siteCommit'], 'docsSiteTree': binding['docsSiteTree'],
            'binding': site_binding_path(binding['version']), 'bindingSha256': digest}


def site_binding_path(version):
    import site_binding
    return site_binding.RELEASES[version]


def site_prompt(site):
    return ('Site binding: ' + json.dumps(site, sort_keys=True) + ' (verified by tools/site_binding.py --check; '
            'the docs worktree HEAD is the bound deployed site commit).')


def verify_release_readiness(root, path, source):
    """Phase 31 publication guard: the latest release-readiness chunk attempt is an
    accepted, unchanged Codex record of exactly this source."""
    phase, chunk = 31, 'release-readiness'
    latest = latest_attempt(root, phase, chunk)
    if not latest or path.resolve() != destination(root, phase, latest, chunk).resolve() or path.is_symlink():
        return False
    record = verify_attempt(root, phase, latest, chunk)
    return (record.get('decision') == 'accepted' and record.get('source_unchanged') is True
            and record.get('source_sha256') == source)


RELEASE_PHASES = {31: '0.2.5'}


def verify_release_delivery(root, phase, source):
    """A release phase completes only after its registry, tag and CI delivery verify."""
    if phase not in RELEASE_PHASES:
        return
    from release_delivery import verify as verify_delivery
    verify_delivery(root, source, phase=phase, version=RELEASE_PHASES[phase])


def mark_interrupted(root, phase, attempt, chunk):
    """Record an orphaned slot as a consumed failed attempt once no invocation holds its lock."""
    path = destination(root, phase, attempt, chunk)
    if path.exists() or path.with_suffix('.failed.json').exists() or not path.with_suffix('.prompt.md').is_file():
        raise ValueError('Only an orphaned attempt with a prompt and no record can be marked interrupted')
    lock = lock_path(root, phase, attempt, chunk)
    lock.parent.mkdir(parents=True, exist_ok=True)
    with lock.open('a') as handle:
        try:
            fcntl.flock(handle, fcntl.LOCK_EX | fcntl.LOCK_NB)
        except BlockingIOError:
            raise ValueError('Attempt is still in flight') from None
        checks = path.with_suffix('.checks.json')
        events = root / '.local/reviews' / (path.stem + '.events.jsonl')
        record = {'phase': phase, 'chunk': chunk, 'attempt': attempt, 'reviewer': REVIEWER, 'status': 'invocation_failed',
                  'reason': 'interrupted', 'prompt_sha256': sha256(path.with_suffix('.prompt.md')),
                  'checks_sha256': sha256(checks) if checks.is_file() else None,
                  'events_sha256': sha256(events) if events.is_file() else None,
                  'recorded_at': datetime.datetime.now(datetime.timezone.utc).isoformat()}
        path.with_suffix('.failed.json').write_text(json.dumps(record, indent=2) + '\n')
    return record


def build_prompt(row, chunk, scope, source, checks_name, attempt, site=None):
    target = f"chunk `{chunk}` of phase {row['id']}" if chunk else f"the complete phase {row['id']}"
    criteria = row.get('acceptance_criteria', [])
    lines = [
        f"You are the independent reviewer for {target} ({row['name']}) of the scm-sdk Rust SDK at the current working directory.",
        'Follow AGENTS.md and CLAUDE.md review context. Read docs/portkey-mcp-tags-plan.md (the accepted plan for phases 29-31), '
        'docs/PHASES.md and the phase entry in docs/phase-status.json before reviewing.',
        'Keep the checkout read-only: do not edit files, commit, push, contact tenants, read credential files or tenant registrations, '
        'or make network requests. You may run read-only inspection commands.',
        f'Review attempt {attempt} of at most {MAX_ATTEMPTS}. Reviewed source fingerprint: {source}. '
        f'Passing check report for this exact source: {checks_name} (inspect its commands and results).',
        'Scope artifacts: ' + json.dumps(scope) + '.',
        *([site_prompt(site)] if site else []),
        'Acceptance criteria: ' + json.dumps(criteria) + '.',
        'Assess the actual code, tests, provenance, security, functionality, ergonomics and maintainability against the plan. '
        'Do not reward claimed coverage that is not supported by implemented methods, tests or recorded evidence. '
        'Distinguish verified test evidence from unexecuted checks and service limitations. Later phases may remain unimplemented.',
        f'Return the structured result: an integer score from 1 to 10 (acceptance requires at least {MINIMUM_SCORE} and no blocker/high finding), '
        'a summary, concrete findings with repository-relative path, line (0 if not line-specific), severity '
        '(blocker|high|medium|low), description and required_fix, the evidence you actually inspected, and limitations.',
    ]
    if row.get('review_findings_disposition'):
        lines.append('Disposition of earlier review findings: ' + json.dumps(row['review_findings_disposition']) + '.')
    return '\n\n'.join(lines) + '\n'


def update_status(root, status, row, chunk, record_path, review, source, decision):
    """Apply one decision to a freshly read status under an exclusive lock, so
    concurrent chunk reviews cannot overwrite each other's bookkeeping."""
    lock = (root / '.local/reviews/phase-status.lock').open('a')
    fcntl.flock(lock, fcntl.LOCK_EX)
    try:
        status = json.loads((root / 'docs/phase-status.json').read_text())
        row = next(item for item in status['phases'] if item['id'] == row['id'])
        _apply_status(root, status, row, chunk, record_path, review, source, decision)
    finally:
        lock.close()


def _apply_status(root, status, row, chunk, record_path, review, source, decision):
    relative = record_path.relative_to(root).as_posix()
    if chunk:
        target = next(item for item in row['chunks'] if item['id'] == chunk)
        for key in ('accepted_review', 'accepted_score', 'accepted_source_sha256', 'latest_review'):
            target.pop(key, None)
        if decision == 'accepted':
            target.update(status='accepted', review_decision='accepted', accepted_review=relative,
                          accepted_score=review['score'], accepted_source_sha256=source)
        else:
            # A later rejection supersedes any earlier acceptance of this chunk.
            target.update(status='pending', review_decision='rejected', latest_review=relative)
    elif decision != 'accepted':
        row['latest_rejected_review'] = relative
    else:
        verify_chunk_acceptance(root, row, source)
        verify_release_delivery(root, row['id'], source)
        row.update(status='complete', accepted_source_sha256=source, accepted_score=review['score'])
        row.setdefault('reviews', []).append(relative)
        row['milestones'] = {
            'implemented_scope': {'status': 'accepted_by_review', 'evidence': [relative]},
            'independent_verification': {'status': 'passed', 'evidence': [relative.replace('.json', '.checks.json')],
                                         'source_sha256': source},
            'codex_review': {'status': 'accepted', 'score': review['score'], 'evidence': [relative]},
        }
        later = [item['id'] for item in status['phases'] if item['id'] in PHASES and item['id'] > row['id'] and item['status'] != 'complete']
        status['active_phase'] = min(later) if later else row['id']
        for item in status['phases']:
            if later and item['id'] == min(later):
                item['status'] = 'in_progress'
    (root / 'docs/phase-status.json').write_text(json.dumps(status, indent=2) + '\n')


def codex_version():
    """Record the reviewer CLI version; a host without it records None and the review invocation fails closed."""
    try:
        return subprocess.run([CODEX, '--version'], capture_output=True, text=True).stdout.strip() or None
    except FileNotFoundError:
        return None


def invoke(prompt_path, output_path, events_path, schema_path, root):
    command = [CODEX, 'exec', '--sandbox', 'read-only', '--cd', str(root), '--ephemeral', '--json',
               '--output-schema', str(schema_path), '--output-last-message', str(output_path), '-']
    with prompt_path.open('rb') as stdin, events_path.open('wb') as events:
        return subprocess.run(command, stdin=stdin, stdout=events, stderr=subprocess.STDOUT, timeout=TIMEOUT_SECONDS).returncode


def main(argv=None, root=ROOT, runner=invoke):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--phase', required=True, type=int, choices=PHASES)
    parser.add_argument('--chunk')
    parser.add_argument('--mark-interrupted', type=int, metavar='ATTEMPT',
                        help='Record an orphaned attempt slot as a consumed interrupted invocation')
    args = parser.parse_args(argv)
    if args.mark_interrupted is not None:
        print(json.dumps(mark_interrupted(root, args.phase, args.mark_interrupted, args.chunk)))
        return 1
    status = json.loads((root / 'docs/phase-status.json').read_text())
    if status['active_phase'] != args.phase:
        raise ValueError('Review the active phase')
    row = phase_row(status, args.phase)
    scope = scope_for(row, args.chunk)
    source = fingerprint(root)
    if args.chunk is None:
        verify_chunk_acceptance(root, row, source)
        verify_release_delivery(root, args.phase, source)
    checks = check_evidence(root, args.phase, source)
    site = site_summary(*verify_site(root, args.phase, args.chunk)) if (args.phase, args.chunk) in SITE_BINDINGS else None
    attempt = next_attempt(root, args.phase, args.chunk)
    verify_prior_attempts(root, args.phase, attempt, args.chunk)
    record_path = destination(root, args.phase, attempt, args.chunk)
    record_path.parent.mkdir(parents=True, exist_ok=True)
    prompt_path, checks_copy = record_path.with_suffix('.prompt.md'), record_path.with_suffix('.checks.json')
    private = root / '.local/reviews'
    private.mkdir(parents=True, exist_ok=True)
    lock = lock_path(root, args.phase, attempt, args.chunk).open('a')
    try:
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
    except BlockingIOError:
        raise ValueError('Another invocation holds this attempt slot') from None
    reserve(prompt_path, build_prompt(row, args.chunk, scope, source, checks.relative_to(root).as_posix(), attempt, site))
    shutil.copyfile(checks, checks_copy)
    stem = record_path.stem
    events_path, schema_path = private / f'{stem}.events.jsonl', private / f'{stem}.schema.json'
    output_path = record_path.with_suffix('.codex.json')
    schema_path.write_text(json.dumps(OUTPUT_SCHEMA))
    version = codex_version()
    base = {'phase': args.phase, 'chunk': args.chunk, 'attempt': attempt, 'reviewer': REVIEWER, 'codex_version': version,
            'source_sha256': source, 'scope_evidence': scope, 'prompt_sha256': sha256(prompt_path),
            'checks_sha256': sha256(checks_copy), **({'site_binding': site} if site else {}), 'requested_at': datetime.datetime.now(datetime.timezone.utc).isoformat()}

    def failed(reason):
        record = {**base, 'status': 'invocation_failed', 'reason': reason,
                  'events_sha256': sha256(events_path) if events_path.exists() else None}
        record_path.with_suffix('.failed.json').write_text(json.dumps(record, indent=2) + '\n')
        print(json.dumps({'decision': 'invocation_failed', 'reason': reason, 'attempt': attempt}))
        return 1

    try:
        code = runner(prompt_path, output_path, events_path, schema_path, root)
    except subprocess.TimeoutExpired:
        return failed('timeout')
    except OSError:
        return failed('launch_error')
    except BaseException:
        failed('interrupted')
        raise
    if code != 0:
        return failed('process_error')
    try:
        review = json.loads(output_path.read_text())
        validate_review(review)
        if isinstance(review['score'], float) and not review['score'].is_integer():
            raise ValueError('Codex score must be an integer')
        review['score'] = int(review['score'])
        output_path.write_text(json.dumps(review, indent=2) + '\n')
    except (OSError, ValueError, KeyError, TypeError):
        return failed('invalid_output')
    unchanged = fingerprint(root) == source
    site_fields = {}
    if site:
        try:
            site_fields['site_unchanged'] = site_summary(*verify_site(root, args.phase, args.chunk)) == site
        except (OSError, ValueError, subprocess.CalledProcessError):
            site_fields['site_unchanged'] = False
    decision = decide(review) if unchanged and site_fields.get('site_unchanged', True) else 'rejected'
    record = {**base, 'status': 'completed', 'source_unchanged': unchanged, **site_fields, 'review': review, 'decision': decision,
              'output_sha256': sha256(output_path), 'events_sha256': sha256(events_path),
              'completed_at': datetime.datetime.now(datetime.timezone.utc).isoformat()}
    record_path.write_text(json.dumps(record, indent=2) + '\n')
    update_status(root, status, row, args.chunk, record_path, review, source, decision)
    print(json.dumps({'decision': decision, 'score': review['score'], 'attempt': attempt,
                      'findings': [{k: f[k] for k in ('severity', 'path', 'line', 'description')} for f in review['findings']]}, indent=2))
    return 0 if decision == 'accepted' else 2


if __name__ == '__main__':
    sys.exit(main())
