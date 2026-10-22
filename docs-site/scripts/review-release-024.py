#!/usr/bin/env python3
"""Independent Pages 0.2.4 review runner; never invokes an SDK phase review.

Run only after verified registry/publication delivery and its evidence merge. Actual
Claude invocations consume one of five attempts, including failures. This helper
reads only an allowlisted repository source set and launches Read/Glob/Grep-only
Claude with no MCP servers. It does not publish, push, commit or change phase state.
"""
import argparse
import datetime
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import unittest

ROOT = Path(__file__).resolve().parents[2]
SCOPE = ROOT / 'docs/reviews/github-pages/release-0.2.4'
PRIVATE = ROOT / '.local/reviews/github-pages-release-0.2.4'
TEMPLATE = Path(__file__).with_name('review-release-024.prompt.md')
BASE = '382d9a0876c569c8349cd0db0096635f2494389b'
CATEGORIES = ('implementation', 'code_quality', 'supportability', 'architecture', 'delivery')
FINDING = {'type': 'object', 'properties': {
    'severity': {'type': 'string', 'enum': ['blocker', 'high', 'medium', 'low']},
    'path': {'type': 'string'}, 'line': {'type': 'integer', 'minimum': 1},
    'description': {'type': 'string'}, 'required_fix': {'type': 'string'},
}, 'required': ['severity', 'path', 'line', 'description', 'required_fix'], 'additionalProperties': False}
SCHEMA = {'type': 'object', 'properties': {
    'categories': {'type': 'object', 'properties': {key: {'type': 'integer', 'minimum': 0, 'maximum': 10} for key in CATEGORIES},
                   'required': list(CATEGORIES), 'additionalProperties': False},
    'total': {'type': 'integer', 'minimum': 0, 'maximum': 50},
    'overall': {'type': 'integer', 'minimum': 0, 'maximum': 10},
    'blocking_findings': {'type': 'array', 'items': FINDING},
    'nonblocking_findings': {'type': 'array', 'items': FINDING},
    'evidence_limitations': {'type': 'array', 'items': {'type': 'string'}},
}, 'required': ['categories', 'total', 'overall', 'blocking_findings', 'nonblocking_findings', 'evidence_limitations'],
    'additionalProperties': False}
ASSETS = (
    'docs-site/static/img/brand-logo.png', 'docs-site/static/img/logo.svg', 'docs-site/static/img/social-card.png',
    'docs-site/src/css/custom.css', 'docs-site/src/css/prism-airs.ts', 'docs-site/src/pages/index.module.css',
    'docs-site/src/theme/DocItem/Layout/index.tsx', 'docs-site/src/theme/DocItem/Layout/styles.module.css',
    'docs-site/docusaurus.config.ts', '.github/workflows/docs-deploy.yml',
)
REFERENCES = (
    'AGENTS.md', 'CLAUDE.md', 'docs/PHASES.md', 'docs/GITHUB_MIRROR.md',
    'docs/DIRECTIONAL_PROFILES.md', 'docs/STABILITY.md', 'docs/SECURITY.md', 'CHANGELOG.md',
    'docs/checks/release-registry-0.2.4.json',
    'docs/reviews/github-pages/release-0.2.4/remote-branches-before-deployment.json',
)


def sha(data):
    return hashlib.sha256(data).hexdigest()


def git(*args):
    return subprocess.check_output(['git', *args], cwd=ROOT)


def read(relative):
    path = ROOT / relative
    if path.is_symlink() or not path.is_file() or not path.resolve().is_relative_to(ROOT):
        raise ValueError('Review input is not a confined regular file: ' + relative)
    return path.read_bytes()


def phase28(status):
    phases = status.get('phases') if isinstance(status, dict) else None
    if not isinstance(phases, list) or any(not isinstance(p, dict) for p in phases):
        raise ValueError('phase-status phases is missing or malformed')
    phase = next((p for p in phases if p.get('id') == 28), None)
    if phase is None:
        raise ValueError('phase-status phases lacks phase28')
    return phase


def prior_review_name(number):
    stem = f'attempt-{number:02d}'
    return stem + ('.failed.json' if (SCOPE / (stem + '.failed.json')).is_file() else '.json')


def prior_review_sources(attempt):
    prefix = 'docs/reviews/github-pages/release-0.2.4/'
    return [prefix + name for number in range(1, attempt)
            for name in (prior_review_name(number), f'attempt-{number:02d}.disposition.json')]


def render_prompt(attempt):
    current = f'attempt-{attempt:02d}'
    prior = f'attempt-{attempt - 1:02d}'
    paragraphs = TEMPLATE.read_text().split('\n\n')
    if attempt == 1:
        paragraphs = [paragraph for paragraph in paragraphs
                      if not any(token in paragraph for token in ('{prior}', '{prior_review}'))]
    prompt = '\n\n'.join(paragraphs).replace('{current}', current)
    if attempt > 1:
        prompt = prompt.replace('{prior_review}', prior_review_name(attempt - 1)).replace('{prior}', prior)
    return prompt


def delivery_records():
    status = json.loads(read('docs/phase-status.json'))
    phase = phase28(status)
    registry = json.loads(read('docs/checks/release-registry-0.2.4.json'))
    delivery = phase.get('delivery', {})
    canonical_ci = delivery.get('ci_receipt')
    if (not isinstance(canonical_ci, str)
            or not re.fullmatch(r'docs/checks/phase-28-remote-ci-run-\d+\.json', canonical_ci)):
        raise ValueError('Merged canonical ci_receipt is missing or malformed')
    ci = delivery.get('ci_observation', canonical_ci)
    review = registry.get('releaseReadiness', {}).get('review', '')
    if (not re.fullmatch(r'docs/checks/phase-28-remote-ci-run-\d+(?:-observation-\d+)?\.json', ci)
            or not re.fullmatch(r'docs/reviews/phase-28-attempt-\d+\.json', review)):
        raise ValueError('Merged current CI and actual readiness review references are required')
    stem = review.removesuffix('.json')
    files = [
        'docs/checks/release-registry-0.2.4.json', 'docs/checks/release-tag-v0.2.4.json',
        'docs/checks/release-tag-github-v0.2.4.json', ci,
        canonical_ci,
        'docs/checks/phase-28-release-package-1.89.0.json',
        'docs/checks/phase-28-release-package-1.99.0.json',
        review, stem + '.checks.json', stem + '.prompt.md', stem + '.cli.json',
    ]
    for field in ('tag_observation', 'github_tag_observation'):
        relative = delivery.get(field)
        if relative is not None:
            if not re.fullmatch(r'docs/checks/release-tag-[A-Za-z0-9.-]+\.json', relative):
                raise ValueError('Invalid governance tag evidence path')
            files.append(relative)
    return phase, files


def verify_release_delivery(release_repo, qualified_release_repo=None):
    release_repo = release_repo.resolve()
    phase, files = delivery_records()
    primary_status_path = release_repo / 'docs/phase-status.json'
    if primary_status_path.is_symlink() or not primary_status_path.is_file():
        raise ValueError('Primary phase evidence is not a regular source file')
    primary_status = json.loads(primary_status_path.read_bytes())
    primary_phase = phase28(primary_status)
    if phase != primary_phase:
        raise ValueError('Merged phase28 records differ from primary records; do not fabricate completion')
    digests = {}
    for name in files:
        primary = release_repo / name
        if primary.is_symlink() or not primary.resolve().is_relative_to(release_repo):
            raise ValueError('Primary delivery evidence must be a confined regular file')
        if read(name) != primary.read_bytes():
            raise ValueError('Merged delivery evidence differs from primary: ' + name)
        digests[name] = sha(read(name))
    # Run the unmodified primary guard where its retained archives and original
    # source fingerprint live. A docs merge changes the source fingerprint, so
    # running this SDK guard against the documentation tree would be incorrect.
    command = [sys.executable, 'tools/release_delivery.py', '--phase', '28', '--version', '0.2.4']
    if qualified_release_repo is not None:
        if qualified_release_repo.resolve() == release_repo:
            raise ValueError('Governance and qualified release roots must be separate')
        command += ['--post-publication-governance', '--qualified-release-root', str(qualified_release_repo.resolve())]
    result = subprocess.run(command, cwd=release_repo, capture_output=True, text=True, timeout=300)
    if result.returncode:
        raise ValueError('Primary actual release_delivery.verify has not passed')
    verified = json.loads(result.stdout)
    if not isinstance(verified, dict):
        raise ValueError('Primary guard result is not an object')
    governance = None
    registry = json.loads(read('docs/checks/release-registry-0.2.4.json'))
    published_source = registry.get('sourceCommit') if isinstance(registry, dict) else None
    if not isinstance(published_source, str) or not published_source:
        raise ValueError('Registry sourceCommit is missing or malformed')
    if qualified_release_repo is not None:
        governance = verified
        if (governance.get('scope') != 'post_publication_governance'
                or governance.get('passed') is not True or not governance.get('source_sha256')
                or not governance.get('qualifiedSourceSha256')
                or governance.get('source_sha256') in (governance.get('qualifiedSourceSha256'), registry.get('archiveSha256'))
                or governance.get('qualifiedReleaseRoot') != str(qualified_release_repo.resolve())):
            raise ValueError('Governance delivery proof lacks distinct source bindings')
        canonical = governance.get('canonicalGuard')
        verified = canonical.get('result') if isinstance(canonical, dict) else None
        if not isinstance(verified, dict):
            raise ValueError('Governance canonicalGuard result is missing or malformed')
        if governance.get('publishedSourceCommit') != verified.get('sourceCommit'):
            raise ValueError('Governance published source differs from qualified guard')
        for name, digest in governance.get('tagObservations', {}).items():
            if digests.get(name) != digest:
                raise ValueError('Governance tag observation is not bound to merged evidence')
        if digests.get(governance.get('currentCiPath')) != governance.get('currentCiSha256'):
            raise ValueError('Governance CI observation is not bound to merged evidence')
    for field in ('sourceCommit', 'ciCommit'):
        if not isinstance(verified.get(field), str) or not verified.get(field):
            raise ValueError('Primary guard ' + field + ' is missing or malformed')
    if (verified.get('passed') is not True or verified.get('version') != '0.2.4'
            or verified.get('jobs') != 49 or verified.get('sourceCommit') != published_source):
        raise ValueError('Primary delivery guard lacks exact publication and terminal49-CI proof')
    for commit in (verified.get('sourceCommit'), verified.get('ciCommit')):
        subprocess.run(['git', 'merge-base', '--is-ancestor', commit, 'HEAD'], cwd=ROOT, check=True)
    return {'releaseRepo': str(release_repo),
            'qualifiedReleaseRepo': str(qualified_release_repo.resolve()) if qualified_release_repo else None,
            'guard': verified, 'governanceGuard': governance, 'evidenceSha256': digests,
            'sdkPhase28Status': phase.get('status'),
            'sdkPhase28Sha256': sha(json.dumps(phase, sort_keys=True, separators=(',', ':')).encode()),
            'sdkFinalReview': 'Not granted or replaced by independent documentation review.'}


def documentation_metadata_sha(status):
    delivery = status.get('independent_documentation_delivery')
    if not isinstance(delivery, dict) or not delivery:
        raise ValueError('Historical documentation delivery metadata is required')
    if not isinstance(delivery.get('release_updates', []), list):
        raise ValueError('Existing documentation release_updates is not a list')
    return sha(json.dumps(delivery, sort_keys=True, separators=(',', ':')).encode())


def append_documentation_update(status, expected_sha, update):
    if documentation_metadata_sha(status) != expected_sha:
        raise ValueError('Historical documentation delivery metadata changed during review')
    status['independent_documentation_delivery'].setdefault('release_updates', []).append(update)


def snapshot(attempt=1):
    listed = git('ls-files', '--cached', '--others', '--exclude-standard', '-z', '--', 'docs-site').decode().split('\0')
    phase, evidence = delivery_records()
    files = sorted(set(listed) - {''} | set(REFERENCES) | set(evidence) | set(prior_review_sources(attempt)) | {'.github/workflows/docs-deploy.yml'})
    digests = {name: sha(read(name)) for name in files}
    sdk_phase_sha = sha(json.dumps(phase, sort_keys=True, separators=(',', ':')).encode())
    docs_sha = documentation_metadata_sha(json.loads(read('docs/phase-status.json')))
    fingerprint = sha(json.dumps({'files': digests, 'sdkPhase28Sha256': sdk_phase_sha, 'priorDocumentationMetadataSha256': docs_sha}, sort_keys=True, separators=(',', ':')).encode())
    return {'headCommit': git('rev-parse', 'HEAD').decode().strip(), 'baselineCommit': BASE,
            'files': digests, 'priorDocumentationMetadataSha256': docs_sha, 'sdkPhase28Sha256': sdk_phase_sha, 'sdkPhase28Status': phase.get('status'), 'sourceSha256': fingerprint}


def prerequisites():
    if git('branch', '--show-current').decode().strip() != 'docs/scm-github-pages':
        raise ValueError('Run only in the isolated documentation branch')
    receipt = json.loads(read('docs/checks/release-registry-0.2.4.json'))
    commit = receipt.get('sourceCommit', '')
    if (receipt.get('registry') != 'crates.io' or receipt.get('package') != 'scm-sdk'
            or receipt.get('version') != '0.2.4' or receipt.get('yanked') is not False
            or not receipt.get('publishedAt') or not re.fullmatch('[0-9a-f]{40}', commit)
            or not re.fullmatch('[0-9a-f]{64}', receipt.get('archiveSha256', ''))
            or receipt.get('vcs', {}).get('git', {}).get('sha1') != commit):
        raise ValueError('Verified published 0.2.4 registry identity is required')
    if git('rev-parse', 'v0.2.4^{commit}').decode().strip() != commit:
        raise ValueError('Release tag differs from registry source')
    subprocess.run(['git', 'merge-base', '--is-ancestor', commit, 'HEAD'], cwd=ROOT, check=True)
    manifest = json.loads(read('docs-site/DESIGN_SOURCE.json'))
    binding = manifest.get('directionalProfilesRelease', {})
    if (binding.get('sourceCommit') != commit or binding.get('version') != '0.2.4'
            or binding.get('registryReceiptSha256') != sha(read('docs/checks/release-registry-0.2.4.json'))
            or binding.get('archiveSha256') != receipt['archiveSha256']
            or manifest.get('derivedFrom', {}).get('docs/airs/directional-profiles.md') != 'docs/DIRECTIONAL_PROFILES.md'
            or binding.get('sourceSha256') != sha(git('show', commit + ':docs/DIRECTIONAL_PROFILES.md'))):
        raise ValueError('Receipt-bound directional guide provenance is missing or stale')
    status = json.loads(read('docs/phase-status.json'))
    phase = phase28(status)
    if not phase or not status.get('independent_documentation_delivery'):
        raise ValueError('Actual phase28 records and historical docs delivery must both be merged')
    documentation_metadata_sha(status)
    return receipt


def validate_review(review):
    if not isinstance(review, dict) or set(review) != set(SCHEMA['required']):
        raise ValueError('Review output shape is invalid')
    scores = review['categories']
    if not isinstance(scores, dict) or set(scores) != set(CATEGORIES):
        raise ValueError('Review categories are incomplete')
    if any(type(value) is not int or not 0 <= value <= 10 for value in scores.values()):
        raise ValueError('Review category scores are invalid')
    if type(review['total']) is not int or review['total'] != sum(scores.values()):
        raise ValueError('Review total differs from category sum')
    if type(review['overall']) is not int or not 0 <= review['overall'] <= 10:
        raise ValueError('Review overall score is invalid')
    for name in ('blocking_findings', 'nonblocking_findings'):
        if not isinstance(review[name], list):
            raise ValueError('Review findings must be arrays')
        for item in review[name]:
            if (not isinstance(item, dict) or set(item) != set(FINDING['required'])
                    or item['severity'] not in ('blocker', 'high', 'medium', 'low')
                    or type(item['line']) is not int or item['line'] < 1
                    or any(not isinstance(item[key], str) or not item[key] for key in ('path', 'description', 'required_fix'))):
                raise ValueError('Review finding is invalid')
    if not isinstance(review['evidence_limitations'], list) or not all(isinstance(x, str) for x in review['evidence_limitations']):
        raise ValueError('Review limitations are invalid')
    return (min(scores.values()) >= 8 and review['total'] >= 46 and not review['blocking_findings']
            and not any(x['severity'] in ('blocker', 'high') for x in review['nonblocking_findings']))


def fifth_attempt_exception(review, attempt):
    if validate_review(review) or attempt != 5:
        return None
    if (review['total'] <= 40 or min(review['categories'].values()) < 8
            or review['blocking_findings']
            or any(f['severity'] in ('blocker', 'high') for f in review['nonblocking_findings'])):
        return None
    prior = []
    for number in range(1, 5):
        path = SCOPE / f'attempt-{number:02d}.json'
        if not path.is_file() or (SCOPE / f'attempt-{number:02d}.failed.json').exists():
            return None
        record = json.loads(path.read_text())
        if (record.get('reviewer') != 'Claude Code' or record.get('attempt') != number
                or record.get('scope') != 'independent-docs-release-0.2.4'
                or record.get('sourceUnchanged') is not True or record.get('decision') != 'rejected'):
            return None
        if validate_review(record['review']):
            return None
        for key, suffix in [('sourceManifestSha256', 'source.json'), ('checksSha256', 'checks.json'),
                            ('promptSha256', 'prompt.md'), ('cliSha256', 'cli.json')]:
            evidence = SCOPE / f'attempt-{number:02d}.{suffix}'
            if not evidence.is_file() or sha(evidence.read_bytes()) != record.get(key):
                return None
        cli = json.loads((SCOPE / f'attempt-{number:02d}.cli.json').read_text())
        if cli.get('exitCode') != 0:
            return None
        prior.append({'attempt': number, 'reviewSha256': sha(path.read_bytes())})
    return {'rule': 'Fifth attempt only: total >40, every category >=8, no blocking/high findings, four prior substantive reviews.',
            'priorReviews': prior, 'remainingDeficiencies': review['nonblocking_findings'],
            'categoryScores': review['categories'], 'evidenceLimitations': review['evidence_limitations'],
            'normalThresholdShortfall': 46 - review['total']}


def parse_envelope(envelope):
    if envelope.get('is_error') is not False:
        raise ValueError('Claude returned an error')
    review = envelope.get('structured_output')
    if review is None:
        review = json.loads(envelope.get('result', '{}'))
    return review, validate_review(review)


def write_json(path, value):
    with path.open('x') as stream:
        json.dump(value, stream, indent=2)
        stream.write('\n')


def validate_attempt(number):
    if not 1 <= number <= 5:
        raise ValueError('Independent documentation scope has at most five attempts')
    if list(SCOPE.glob(f'attempt-{number:02d}.*')):
        raise ValueError('Attempt already has artifacts; never overwrite or restart it')
    for previous in range(1, number):
        paths = [SCOPE / f'attempt-{previous:02d}.json', SCOPE / f'attempt-{previous:02d}.failed.json']
        present = [path for path in paths if path.is_file()]
        if len(present) != 1:
            raise ValueError('Prior attempt has no unique terminal receipt')
        record = json.loads(present[0].read_text())
        for key, suffix in [('sourceManifestSha256', 'source.json'), ('checksSha256', 'checks.json'), ('promptSha256', 'prompt.md')]:
            path = SCOPE / f'attempt-{previous:02d}.{suffix}'
            if not path.is_file() or sha(path.read_bytes()) != record.get(key):
                raise ValueError('Prior attempt evidence is missing or altered')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--attempt', type=int, required=True)
    parser.add_argument('--release-repo', type=Path, required=True,
                        help='Primary release checkout with verified retained archives and delivery records')
    parser.add_argument('--qualified-release-repo', type=Path,
                        help='Immutable published checkout; required when release-repo is governance source')
    parser.add_argument('--run', action='store_true', help='Run checks and actual Claude after all publication/merge prerequisites')
    args = parser.parse_args()
    if not args.run:
        parser.error('Explicit --run is required; no review is launched by inspection')
    validate_attempt(args.attempt)
    receipt = prerequisites()
    verified_delivery = verify_release_delivery(args.release_repo, args.qualified_release_repo)
    before = snapshot(args.attempt)
    preserved = {}
    for name in ASSETS:
        actual = read(name)
        if actual != git('show', BASE + ':' + name):
            raise ValueError('Preserved logo/style/config/workflow changed: ' + name)
        preserved[name] = sha(actual)
    PRIVATE.mkdir(parents=True, exist_ok=True, mode=0o700)
    unit_test_count = unittest.TestLoader().discover(
        str(ROOT / 'docs-site/scripts'), pattern='test_review_release_024.py').countTestCases()
    if unit_test_count < 8:
        raise ValueError('Expected at least eight documentation review helper tests')
    commands = [[sys.executable, '-m', 'unittest', 'discover', '-s', 'docs-site/scripts',
                 '-p', 'test_review_release_024.py', '-v'],
                ['npm', '--prefix', 'docs-site', 'run', 'build'],
                ['npm', '--prefix', 'docs-site', 'run', 'typecheck'],
                ['git', 'diff', '--check', BASE, '--', 'docs-site', 'docs/SECURITY.md',
                 'docs/GITHUB_MIRROR.md', '.github/workflows/docs-deploy.yml']]
    checks = []
    for index, command in enumerate(commands):
        result = subprocess.run(command, cwd=ROOT, capture_output=True, timeout=900)
        log = PRIVATE / f'attempt-{args.attempt:02d}-check-{index}.log'
        log.write_bytes(result.stdout + b'\n' + result.stderr)
        log.chmod(0o600)
        checks.append({'command': command, 'exitCode': result.returncode, 'logSha256': sha(log.read_bytes())})
        if result.returncode:
            raise ValueError('Documentation check failed; no Claude invocation consumed')
    if (snapshot(args.attempt)['sourceSha256'] != before['sourceSha256']
            or verify_release_delivery(args.release_repo, args.qualified_release_repo) != verified_delivery):
        raise ValueError('Source changed while checking; no Claude invocation consumed')
    SCOPE.mkdir(parents=True, exist_ok=True)
    stem = f'attempt-{args.attempt:02d}'
    source_path, checks_path = SCOPE / (stem + '.source.json'), SCOPE / (stem + '.checks.json')
    write_json(source_path, before)
    write_json(checks_path, {'sourceSha256': before['sourceSha256'], 'priorDocumentationMetadataSha256': before['priorDocumentationMetadataSha256'], 'checks': checks, 'helperUnitTestCount': unit_test_count, 'preservedAssets': preserved, 'verifiedReleaseDelivery': verified_delivery,
                            'sdkTests': 'Not rerun for documentation-only scope; qualified runtime is reference evidence.',
                            'deployment': 'Not performed by this review helper.'})
    prompt = render_prompt(args.attempt)
    prompt += '\nReturn categories inside a categories object and conform exactly to the supplied JSON schema.\n'
    prompt_path = SCOPE / (stem + '.prompt.md')
    with prompt_path.open('x') as stream:
        stream.write(prompt)
    common = {'reviewer': 'Claude Code', 'scope': 'independent-docs-release-0.2.4', 'attempt': args.attempt,
              'reviewedAt': datetime.datetime.now(datetime.timezone.utc).isoformat(),
              'sourceSha256': before['sourceSha256'], 'sourceManifestSha256': sha(source_path.read_bytes()),
              'checksSha256': sha(checks_path.read_bytes()), 'promptSha256': sha(prompt_path.read_bytes()),
              'releaseSourceCommit': receipt['sourceCommit'],
              'runnerSha256': sha(Path(__file__).read_bytes()), 'templateSha256': sha(TEMPLATE.read_bytes())}
    command = ['claude', '-p', '--restricted', '--permission-mode', 'plan', '--tools', 'Read,Glob,Grep',
               '--strict-mcp-config', '--mcp-config', '{"mcpServers":{}}', '--max-budget-usd', '25',
               '--output-format', 'json', '--json-schema', json.dumps(SCHEMA)]
    reason = 'launch_error'
    try:
        result = subprocess.run(command, cwd=ROOT, input=prompt, text=True, capture_output=True, timeout=1800)
        for suffix, value in [('stdout.json', result.stdout), ('stderr.txt', result.stderr)]:
            path = PRIVATE / (stem + '.' + suffix)
            path.write_text(value)
            path.chmod(0o600)
        reason = 'process_error'
        if result.returncode:
            raise ValueError('Claude exited nonzero')
        reason = 'invalid_output'
        envelope = json.loads(result.stdout)
        review, accepted = parse_envelope(envelope)
        exception = fifth_attempt_exception(review, args.attempt)
        accepted = accepted or exception is not None
        reason = 'source_changed'
        if (snapshot(args.attempt)['sourceSha256'] != before['sourceSha256']
                or verify_release_delivery(args.release_repo, args.qualified_release_repo) != verified_delivery):
            raise ValueError('Source changed during review')
        cli = {'exitCode': 0, 'stdoutSha256': sha(result.stdout.encode()), 'stderrSha256': sha(result.stderr.encode())}
        write_json(SCOPE / (stem + '.cli.json'), cli)
        write_json(SCOPE / (stem + '.json'), {**common, 'sourceUnchanged': True,
                   'decision': 'accepted' if accepted else 'rejected', 'review': review,
                   'fifthAttemptException': exception,
                   'cliSha256': sha((SCOPE / (stem + '.cli.json')).read_bytes())})
        if accepted:
            status_path = ROOT / 'docs/phase-status.json'
            status = json.loads(read('docs/phase-status.json'))
            append_documentation_update(status, before['priorDocumentationMetadataSha256'], {'version': '0.2.4', 'scope': common['scope'],
                            'status': 'reviewed_ready_for_deployment', 'score': review['total'],
                            'fifth_attempt_exception': exception,
                            'review': (SCOPE / (stem + '.json')).relative_to(ROOT).as_posix(),
                            'source_manifest': source_path.relative_to(ROOT).as_posix(),
                            'checks': checks_path.relative_to(ROOT).as_posix(),
                            'deployment_status': 'pending', 'release_source_commit': receipt['sourceCommit'],
                            'sdk_phase28_status_at_review': verified_delivery['sdkPhase28Status'],
                            'sdk_final_review': 'Not granted or replaced by this independent docs review.'})
            status_path.write_text(json.dumps(status, indent=2) + '\n')
        print(json.dumps({'attempt': args.attempt, 'decision': 'accepted' if accepted else 'rejected', 'total': review['total']}))
        return 0 if accepted else 1
    except subprocess.TimeoutExpired as error:
        for suffix, value in [('stdout.json', error.stdout), ('stderr.txt', error.stderr)]:
            path = PRIVATE / (stem + '.' + suffix)
            path.write_bytes(value if isinstance(value, bytes) else (value or '').encode())
            path.chmod(0o600)
        reason = 'timeout'
    except KeyboardInterrupt:
        reason = 'interrupted'
    except (OSError, ValueError, KeyError, TypeError, subprocess.SubprocessError):
        pass
    if (SCOPE / (stem + '.json')).is_file():
        print(json.dumps({'attempt': args.attempt, 'status': 'review_recorded_metadata_failed', 'reason': reason}))
        return 1
    write_json(SCOPE / (stem + '.failed.json'), {**common, 'status': 'invocation_failed', 'reason': reason})
    print(json.dumps({'attempt': args.attempt, 'status': 'invocation_failed', 'reason': reason}))
    return 1


if __name__ == '__main__':
    try:
        sys.exit(main())
    except (ValueError, OSError, subprocess.SubprocessError) as error:
        sys.exit(str(error))
