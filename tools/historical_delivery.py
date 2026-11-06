#!/usr/bin/env python3
"""Observe and verify delivery of releases published before their phases completed.

Phases 24 (0.2.3) and 28 (0.2.4) published to crates.io but never recorded
terminal delivery. Their original verifier (tools/release_delivery.py) requires
the current source to equal the release and remote `main` to equal the CI commit;
`main` has since advanced, so that rule can never hold again and history is not
rewritten to force it. Instead each release has a `delivery/vX.Y.Z` branch: the
release commit plus evidence-only commits (docs/checks, docs/reviews,
docs/phase-status.json are outside the source fingerprint), so every branch
commit has exactly the published source fingerprint.

`verify` recomputes, offline, from git objects and retained artifacts:
- the release commit's fingerprint and its annotated tags on the canonical remote
  (and the GitHub mirror where required);
- the crates.io archive: identity, checksum, retained bytes, `.cargo_vcs_info.json`
  naming the release commit, and its file inventory;
- both compiler clean package proofs: passed, qualifying the release fingerprint,
  and byte-identical to the published files except `.cargo_vcs_info.json`;
- the terminal CI run: every configured context's newest status, on a branch
  commit whose fingerprint equals the release and which the observed remote
  delivery branch contains.
A CI context may fail only if an exception record names that exact context and
its cause; such a delivery is reported as `delivered_with_exception`, never as
a clean pass.

Observation commands contact only crates.io, the canonical Forgejo API and the
two git remotes, read-only. Credentials are never written to receipts.
"""
import argparse
import collections
import datetime
import hashlib
import importlib.util
import io
import json
import os
import pathlib
import re
import subprocess
import sys
import tarfile
import urllib.request

ROOT = pathlib.Path(__file__).resolve().parents[1]
RULES = None  # path of the release_delivery module whose rules are bound


def bind_rules(root):
    """Bind fingerprint and CI-matrix rules from `<root>/tools/release_delivery.py`.

    A copy run inside a historical release tree (`--root DIR`) therefore applies that
    release's own definitions rather than those of the checkout it was copied from.
    """
    global GITHUB_REMOTE, REMOTE, commit_fingerprint, expected_contexts, RULES
    path = pathlib.Path(root).resolve() / 'tools/release_delivery.py'
    spec = importlib.util.spec_from_file_location(f'release_delivery_{abs(hash(str(path)))}', path)
    module = importlib.util.module_from_spec(spec)
    sys.path.insert(0, str(path.parent))
    try:
        spec.loader.exec_module(module)
    finally:
        sys.path.remove(str(path.parent))
    GITHUB_REMOTE, REMOTE = module.GITHUB_REMOTE, module.REMOTE
    commit_fingerprint, expected_contexts = module.commit_fingerprint, module.expected_contexts
    RULES = path
    return path


_BOUND_ROOT = pathlib.Path(os.environ.get('HISTORICAL_DELIVERY_ROOT') or ROOT)
if (_BOUND_ROOT / 'tools/release_delivery.py').is_file():
    bind_rules(_BOUND_ROOT)

FORGEJO = 'https://git.cdot.io/api/v1/repos/cdot/scm-sdk-rs'
RUN_URL = 'https://git.cdot.io/cdot/scm-sdk-rs/actions/runs/'
CRATES = 'https://crates.io/api/v1/crates/scm-sdk'
OWNER = 'cdot65'
VCS_INFO = '.cargo_vcs_info.json'
RELEASES = {
    '0.2.3': {'phase': 24, 'release': '14a67541f5a57d411c4c4a64c91ced5ba170dafd', 'branch': 'delivery/v0.2.3',
              'github_tag': False, 'registry': 'docs/checks/release-registry-0.2.3.json',
              'proofs': 'docs/checks/phase-24-release-package-{toolchain}.json'},
    '0.2.4': {'phase': 28, 'release': '6f18529a47cc273feca1000af6e4fc0d96da757e', 'branch': 'delivery/v0.2.4',
              'github_tag': True, 'registry': 'docs/checks/release-registry-0.2.4.json',
              'proofs': 'docs/checks/phase-28-published-package-{toolchain}.json',
              'publication_exception_required': True},
}
TOOLCHAINS = ('1.99.0', '1.89.0')


def require(condition, message):
    if not condition:
        raise ValueError(message)


def git(root, *args, text=True):
    return subprocess.check_output(['git', *args], cwd=root, text=text)


def evidence_dir(version):
    return f'docs/checks/historical-delivery/{version}'


def read_json(root, relative):
    path = root / relative
    require(path.resolve().is_relative_to(root.resolve()) and not any(
        p.is_symlink() for p in (path, *path.parents) if p != p.parent), 'Evidence must be a regular repository file')
    return json.loads(path.read_text())


def blob_json(root, commit, relative):
    return json.loads(git(root, 'show', f'{commit}:{relative}'))


def sha256(data):
    return hashlib.sha256(data).hexdigest()


def archive_rows(data, version):
    """Sorted file inventory of a .crate; returns (rows, vcs_info)."""
    prefix = f'scm-sdk-{version}/'
    rows, vcs = [], None
    with tarfile.open(fileobj=io.BytesIO(data), mode='r:gz') as archive:
        for member in archive.getmembers():
            require(member.name.startswith(prefix) and '..' not in member.name.split('/'), 'Unexpected archive path')
            require(member.isfile() or member.isdir(), 'Archive contains a non-regular member')
            if member.isfile():
                content = archive.extractfile(member).read()
                name = member.name[len(prefix):]
                rows.append({'path': name, 'sha256': sha256(content), 'bytes': len(content)})
                if name == VCS_INFO:
                    vcs = json.loads(content)
    require(vcs is not None, 'Archive lacks VCS identity')
    return sorted(rows, key=lambda row: row['path']), vcs


def proof_rows(proof, version):
    prefix = f'scm-sdk-{version}/'
    rows = [{'path': row['path'][len(prefix):] if row['path'].startswith(prefix) else row['path'],
             'sha256': row['sha256']} for row in proof['files']]
    return sorted(rows, key=lambda row: row['path'])


def without_vcs(rows):
    return [{'path': row['path'], 'sha256': row['sha256']} for row in rows if row['path'] != VCS_INFO]


# ---------------------------------------------------------------- observation

def fetch(url, token=None, limit=64 * 1024 * 1024):
    headers = {'User-Agent': 'scm-sdk-release-observation', 'Accept': 'application/json'}
    if token:
        headers['Authorization'] = 'token ' + token
    with urllib.request.urlopen(urllib.request.Request(url, headers=headers), timeout=60) as response:
        data = response.read(limit + 1)
    require(len(data) <= limit, 'Response exceeded bound')
    return data


def write_new(root, relative, record):
    path = root / relative
    path.parent.mkdir(parents=True, exist_ok=True)
    record['checkedAt'] = datetime.datetime.now(datetime.timezone.utc).isoformat()
    with path.open('x') as stream:
        stream.write(json.dumps(record, indent=2) + '\n')
    return path


def observe_registry(root, version):
    """Retain the crates.io archive and write a registry receipt (0.2.4 only; 0.2.3 has one)."""
    spec = RELEASES[version]
    require(not (root / spec['registry']).exists(), 'Registry receipt already exists')
    meta = json.loads(fetch(f'{CRATES}/{version}'))['version']
    owners = sorted(user['login'] for user in json.loads(fetch(f'{CRATES}/owners'))['users'])
    data = fetch(f'https://static.crates.io/crates/scm-sdk/scm-sdk-{version}.crate')
    require(sha256(data) == meta['checksum'], 'Downloaded archive differs from the registry checksum')
    rows, vcs = archive_rows(data, version)
    require(vcs.get('git', {}).get('sha1') == spec['release'], 'Published archive was not built from the release commit')
    retained = pathlib.Path('.local/artifacts') / f'scm-sdk-{version}-registry-{sha256(data)[:16]}.crate'
    (root / retained).parent.mkdir(parents=True, exist_ok=True)
    if not (root / retained).exists():
        (root / retained).write_bytes(data)
    receipt = {'schemaVersion': 1, 'registry': 'crates.io', 'package': 'scm-sdk', 'version': version,
               'yanked': meta['yanked'], 'publishedAt': meta['created_at'], 'ownerLogins': owners,
               'archiveSha256': sha256(data), 'archiveBytes': len(data), 'retainedArchive': retained.as_posix(),
               'sourceCommit': spec['release'], 'sourceSha256': commit_fingerprint(root, spec['release']),
               'vcs': vcs, 'files': rows,
               'observation': 'post-publication; the release was published before its delivery was recorded'}
    return write_new(root, spec['registry'], receipt)


def observe_ci(root, version, commit, run_number, token):
    spec = RELEASES[version]
    require(re.fullmatch('[0-9a-f]{40}', commit) and run_number > 0, 'Expected exact commit and run number')
    require(commit_fingerprint(root, commit) == commit_fingerprint(root, spec['release']),
            'CI commit fingerprint differs from the published release')
    run = None
    for page in range(1, 21):
        runs = json.loads(fetch(f'{FORGEJO}/actions/runs?limit=50&page={page}', token)).get('workflow_runs', [])
        matching = [item for item in runs if item.get('index_in_repo') == run_number]
        if matching:
            require(len(matching) == 1, 'Ambiguous workflow identity')
            run = matching[0]
            break
        if len(runs) < 50:
            break
    require(run and run.get('commit_sha') == commit and run.get('status') in ('success', 'failure'),
            'Requested workflow is absent, not terminal, or for another commit')
    require(run.get('html_url') == RUN_URL + str(run_number), 'Workflow URL differs')
    contexts = expected_contexts(git(root, 'show', f'{commit}:.forgejo/workflows/ci.yml'))
    prefix = f'/cdot/scm-sdk-rs/actions/runs/{run_number}/jobs/'
    newest = {}
    for page in range(1, 21):
        rows = json.loads(fetch(f'{FORGEJO}/statuses/{commit}?limit=50&page={page}', token))
        for row in rows:
            if not re.fullmatch(re.escape(prefix) + r'\d+', row.get('target_url', '')):
                continue
            require(type(row.get('id')) is int and row['id'] > 0, 'CI status lacks a positive id')
            if row['context'] not in newest or row['id'] > newest[row['context']]['id']:
                newest[row['context']] = {key: row.get(key) for key in ('id', 'context', 'status', 'target_url')}
        if len(rows) < 50:
            break
    else:
        raise ValueError('CI status history exceeded the bounded pagination limit')
    require(set(newest) == contexts, 'Observed CI contexts differ from the configured matrix')
    jobs = sorted(newest.values(), key=lambda row: row['context'])
    receipt = {'schemaVersion': 1, 'version': version, 'sourceCommit': commit,
               'sourceSha256': commit_fingerprint(root, commit),
               'run': {key: run[key] for key in ('index_in_repo', 'html_url', 'status', 'commit_sha')},
               'jobs': jobs, 'expectedJobCount': len(contexts),
               'counts': dict(collections.Counter(job['status'] for job in jobs))}
    return write_new(root, f'{evidence_dir(version)}/ci-run-{run_number}.json', receipt)


def observe_branch(root, version):
    spec = RELEASES[version]
    line = git(root, 'ls-remote', REMOTE, 'refs/heads/' + spec['branch']).split()
    require(len(line) == 2 and re.fullmatch('[0-9a-f]{40}', line[0]), 'Remote delivery branch is absent')
    receipt = {'schemaVersion': 1, 'version': version, 'remote': REMOTE, 'ref': 'refs/heads/' + spec['branch'],
               'commit': line[0]}
    return write_new(root, f'{evidence_dir(version)}/branch.json', receipt)


# ---------------------------------------------------------------- verification

def verify_tag(receipt, remote, version, release):
    refs = receipt.get('remoteRefs', {})
    require(receipt.get('passed') is True and receipt.get('remote') == remote and receipt.get('tag') == 'v' + version
            and receipt.get('sourceCommit') == release and refs.get(f'refs/tags/v{version}^{{}}') == release
            and re.fullmatch('[0-9a-f]{40}', str(receipt.get('tagObject')))
            and refs.get(f'refs/tags/v{version}') == receipt['tagObject'],
            'Annotated tag receipt does not peel to the release commit: ' + remote)
    return receipt['tagObject']


def verify(root, version):
    spec = RELEASES[version]
    root = root.resolve()
    release = spec['release']
    published = commit_fingerprint(root, release)
    evidence = evidence_dir(version)
    manifest = read_json(root, f'{evidence}/manifest.json')
    require(manifest.get('version') == version and manifest.get('phase') == spec['phase']
            and manifest.get('releaseCommit') == release, 'Historical delivery manifest identity differs')

    # Registry archive.
    registry = read_json(root, spec['registry'])
    require(registry.get('registry') == 'crates.io' and registry.get('package') == 'scm-sdk'
            and registry.get('version') == version and registry.get('yanked') is False
            and OWNER in registry.get('ownerLogins', []) and registry.get('sourceCommit') == release,
            'Registry identity, ownership or source differs')
    retained = root / registry['retainedArchive']
    require(retained.resolve().is_relative_to((root / '.local/artifacts').resolve()) and retained.is_file(),
            'Registry archive must be retained under .local/artifacts')
    data = retained.read_bytes()
    require(sha256(data) == registry['archiveSha256'] and len(data) == registry['archiveBytes'],
            'Retained registry archive changed')
    rows, vcs = archive_rows(data, version)
    require(vcs.get('git', {}).get('sha1') == release, 'Published archive VCS identity is not the release commit')
    recorded = sorted(({'path': row['path'][len(f'scm-sdk-{version}/'):] if row['path'].startswith(f'scm-sdk-{version}/')
                        else row['path'], 'sha256': row['sha256']} for row in registry['files']), key=lambda row: row['path'])
    require(without_vcs(rows) == without_vcs(recorded), 'Registry file inventory differs from the retained archive')

    # Clean package proofs, read from the recorded evidence commit.
    proof_commit = manifest.get('proofCommit')
    require(re.fullmatch('[0-9a-f]{40}', str(proof_commit)), 'Proof commit must be exact')
    require(any(subprocess.run(['git', 'merge-base', '--is-ancestor', proof_commit, tip], cwd=root).returncode == 0
                for tip in ('HEAD', spec['branch'])), 'Proof commit is not retained on main or the delivery branch')
    for toolchain in TOOLCHAINS:
        proof = blob_json(root, proof_commit, spec['proofs'].format(toolchain=toolchain))
        qualified = proof.get('releaseCommit')
        require(proof.get('passed') is True and proof.get('toolchain') == toolchain
                and proof.get('requireClean') is True and proof.get('effectiveDirty') is False
                and proof.get('sourceSha256') == published and re.fullmatch('[0-9a-f]{40}', str(qualified))
                and commit_fingerprint(root, qualified) == published
                and subprocess.run(['git', 'merge-base', '--is-ancestor', release, qualified], cwd=root).returncode == 0,
                f'Clean {toolchain} package proof does not qualify the published source')
        require(without_vcs(proof_rows(proof, version)) == without_vcs(rows),
                f'Published files differ from the {toolchain} clean package')

    # Tags.
    canonical = verify_tag(read_json(root, f'{evidence}/tag.json'), REMOTE, version, release)
    if spec['github_tag']:
        require(verify_tag(read_json(root, f'{evidence}/tag-github.json'), GITHUB_REMOTE, version, release) == canonical,
                'GitHub tag object differs from the canonical tag')

    # CI on a fingerprint-identical branch commit contained in the observed remote delivery branch.
    branch = read_json(root, f'{evidence}/branch.json')
    require(branch.get('remote') == REMOTE and branch.get('ref') == 'refs/heads/' + spec['branch'],
            'Delivery branch observation differs')
    ci = read_json(root, f"{evidence}/ci-run-{manifest.get('ciRun')}.json")
    commit = ci.get('sourceCommit')
    require(ci.get('version') == version and ci.get('run', {}).get('commit_sha') == commit
            and ci['run'].get('index_in_repo') == manifest.get('ciRun')
            and ci['run'].get('html_url') == RUN_URL + str(manifest.get('ciRun')), 'CI receipt identity differs')
    require(commit_fingerprint(root, commit) == published == ci.get('sourceSha256'),
            'CI commit fingerprint differs from the published release')
    for ancestor, descendant in ((release, commit), (commit, branch['commit'])):
        require(subprocess.run(['git', 'merge-base', '--is-ancestor', ancestor, descendant], cwd=root).returncode == 0,
                'Delivery ancestry differs: ' + ancestor[:7] + ' -> ' + descendant[:7])
    contexts = expected_contexts(git(root, 'show', f'{commit}:.forgejo/workflows/ci.yml'))
    jobs = ci.get('jobs', [])
    require({job.get('context') for job in jobs} == contexts and len(jobs) == len(contexts) == ci.get('expectedJobCount'),
            'CI contexts differ from the configured matrix')
    prefix = f"/cdot/scm-sdk-rs/actions/runs/{manifest['ciRun']}/jobs/"
    require(all(re.fullmatch(re.escape(prefix) + r'\d+', str(job.get('target_url'))) for job in jobs)
            and len({job['target_url'] for job in jobs}) == len(jobs), 'CI job identities differ')
    failed = sorted(job['context'] for job in jobs if job.get('status') != 'success')
    exceptions = manifest.get('ciExceptions', [])
    require(isinstance(exceptions, list) and all(isinstance(item, dict) and item.get('context') and item.get('cause')
                                                 and item.get('evidence') for item in exceptions),
            'CI exceptions must name a context, cause and evidence')
    require(sorted(item['context'] for item in exceptions) == failed,
            'Every failed CI context, and only those, must carry a recorded exception')
    for item in exceptions:
        require((root / item['evidence']).is_file(), 'CI exception evidence is missing: ' + item['evidence'])
    require((ci['run'].get('status') == 'success') == (not failed), 'CI run status disagrees with its jobs')
    return {'version': version, 'phase': spec['phase'], 'releaseCommit': release, 'sourceSha256': published,
            'ciCommit': commit, 'ciRun': manifest['ciRun'], 'jobs': len(jobs), 'failedContexts': failed,
            'status': 'delivered_with_exception' if failed else 'delivered'}


def run_suite(command, root, env):
    return subprocess.run(command, cwd=root, capture_output=True, text=True, env=env, timeout=600)


def execute(root, version, output, label):
    """Run verify and this tool's own regression suite, then write a bound receipt."""
    root = pathlib.Path(root).resolve()
    relative = pathlib.PurePosixPath(output)
    require(str(relative).startswith(f'{evidence_dir(version)}/execution-') and relative.suffix == '.json'
            and '..' not in relative.parts, 'Execution receipts belong in the historical delivery evidence directory')
    tool = pathlib.Path(__file__).resolve()
    tests = tool.with_name('test_historical_delivery.py')
    status = git(root, 'status', '--porcelain', '--untracked-files=no')
    try:
        result, error = verify(root, version), None
    except (ValueError, OSError, subprocess.CalledProcessError) as exc:
        result, error = None, str(exc)
    env = {**os.environ, 'HISTORICAL_DELIVERY_ROOT': str(root)}
    run = run_suite([sys.executable, '-m', 'unittest', 'discover', '-s', str(tool.parent), '-p', tests.name], root, env)
    summary = [line for line in run.stderr.strip().splitlines() if line.startswith(('Ran ', 'OK', 'FAILED'))]
    receipt = {'schemaVersion': 1, 'label': label, 'version': version,
               'executedAt': datetime.datetime.now(datetime.timezone.utc).isoformat(),
               'headCommit': git(root, 'rev-parse', 'HEAD').strip(), 'workingTreeClean': not status.strip(),
               'rules': {'path': RULES.relative_to(root).as_posix() if RULES and RULES.is_relative_to(root) else str(RULES),
                         'sha256': sha256(RULES.read_bytes()) if RULES else None},
               'tool': {'path': tool.relative_to(root).as_posix() if tool.is_relative_to(root) else tool.name,
                        'sha256': sha256(tool.read_bytes())},
               'tests': {'path': tests.relative_to(root).as_posix() if tests.is_relative_to(root) else tests.name,
                         'sha256': sha256(tests.read_bytes()), 'exitCode': run.returncode, 'summary': summary},
               'verify': {'result': result, 'error': error}}
    path = root / relative
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open('x') as stream:
        stream.write(json.dumps(receipt, indent=2) + '\n')
    require(error is None and run.returncode == 0, 'Historical delivery execution failed; receipt retained: ' + str(relative))
    return path


RELEASE_TREE_CHECK = r"""
import json, pathlib, subprocess, sys
root = pathlib.Path('.').resolve()
sys.path.insert(0, str(root / 'tools'))
import review_phase as rp
phase = int(sys.argv[1])
source = rp.fingerprint(root)
status = json.loads((root / 'docs/phase-status.json').read_text())
row = next(item for item in status['phases'] if item['id'] == phase)
attempts = sorted(root.glob(f'docs/reviews/phase-{phase:02d}-attempt-*.json'))
attempts = [path for path in attempts if path.name.count('.') == 1]
review = max(attempts, key=lambda path: int(path.stem.rsplit('-', 1)[1]))
record = json.loads(review.read_text())
valid = rp.verify_record(root, review, phase, source)
chunks = rp.accepted_chunks(root, phase, row['chunks'], source, phase_review=review)
print(json.dumps({'source_sha256': source, 'phase_review': review.relative_to(root).as_posix(), 'phase_review_valid': valid,
                  'scope': record.get('review_scope'), 'decision': record.get('decision'), 'review': record['review'],
                  'chunks': chunks, 'publication_exception': row.get('publication_exception'),
                  'limitations': row.get('limitations', []), 'review_dispositions': row.get('review_dispositions', []),
                  'head': subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(),
                  'clean': not subprocess.check_output(['git', 'status', '--porcelain', '--untracked-files=no'], text=True).strip()}))
"""


def complete(root, version, tree, attempt_note=None):
    """Record completion of a historically delivered phase from its release tree's own review gate.

    The release tree (the fingerprint-identical delivery branch worktree) validates its final
    phase review and every declared chunk with that release's tools/review_phase.py. This
    checkout's verify() must report `delivered`, a required publication exception must be
    present, and the reviewed records are copied byte-for-byte with their digests receipted.
    """
    root, tree = pathlib.Path(root).resolve(), pathlib.Path(tree).resolve()
    spec = RELEASES[version]
    delivered = verify(root, version)
    require(delivered['status'] == 'delivered', 'Delivery must verify without exceptions before completion')
    require(git(tree, 'rev-parse', '--abbrev-ref', 'HEAD').strip() == spec['branch'], 'Release tree is not the delivery branch')
    head = git(tree, 'rev-parse', 'HEAD').strip()
    require(commit_fingerprint(root, head) == delivered['sourceSha256'], 'Release tree does not carry the published source')
    result = json.loads(subprocess.run([sys.executable, '-c', RELEASE_TREE_CHECK, str(spec['phase'])], cwd=tree,
                                       capture_output=True, text=True, check=True, timeout=600).stdout)
    require(result['clean'] and result['head'] == head, 'Release tree must be clean at its committed head')
    require(result['source_sha256'] == delivered['sourceSha256'], 'Release tree review source differs from the published source')
    require(result['phase_review_valid'] is True and result['scope'] == 'phase'
            and result['decision'] in ('accepted', 'accepted_with_deficiencies'), 'Final phase review is not accepted')
    require(all(chunk.get('status') in ('accepted', 'accepted_with_deficiencies') for chunk in result['chunks']),
            'Every declared chunk must be accepted')
    exception = result.get('publication_exception')
    if spec.get('publication_exception_required'):
        require(isinstance(exception, dict) and exception.get('facts') and exception.get('authorization')
                and exception.get('remediation'), 'A structured publication exception is required for this release')
    # The release tree's attempt numbering is its own lineage, so its review artifacts are kept
    # in a per-release namespace and never overwrite same-named records on this checkout.
    namespace = f'docs/reviews/historical-delivery/{version}'
    copied = []
    for source_path in sorted((tree / 'docs/reviews').glob(f"phase-{spec['phase']:02d}-*")):
        target = root / namespace / source_path.name
        data = source_path.read_bytes()
        target.parent.mkdir(parents=True, exist_ok=True)
        if target.exists():
            require(target.read_bytes() == data, 'Review artifact differs from the retained copy: ' + source_path.name)
        else:
            target.write_bytes(data)
        copied.append({'path': f'{namespace}/{source_path.name}', 'sha256': sha256(data)})
    result['phase_review'] = f"{namespace}/{pathlib.PurePosixPath(result['phase_review']).name}"
    for chunk in result['chunks']:
        if chunk.get('accepted_review'):
            chunk['accepted_review'] = f"{namespace}/{pathlib.PurePosixPath(chunk['accepted_review']).name}"
    receipt = {'schemaVersion': 1, 'version': version, 'phase': spec['phase'], 'releaseTreeHead': head,
               'sourceSha256': delivered['sourceSha256'], 'delivery': delivered, 'phaseReview': result['phase_review'],
               'decision': result['decision'], 'total': result['review'].get('total'),
               'categories': result['review'].get('categories'),
               'chunks': [{'id': c['id'], 'status': c['status'], 'accepted_review': c.get('accepted_review'),
                           'accepted_total': c.get('accepted_total')} for c in result['chunks']],
               'publicationException': bool(exception), 'reviewArtifacts': copied, 'attemptNote': attempt_note}
    path = write_new(root, f'{evidence_dir(version)}/completion.json', receipt)
    status_path = root / 'docs/phase-status.json'
    status = json.loads(status_path.read_text())
    row = next(item for item in status['phases'] if item['id'] == spec['phase'])
    review_record = json.loads((root / result['phase_review']).read_text())
    row.update({'status': 'complete', 'chunks': result['chunks'], 'accepted_source_sha256': delivered['sourceSha256'],
                'accepted_score': review_record['review'].get('score'), 'accepted_total': review_record['review'].get('total'),
                'accepted_categories': review_record['review'].get('categories'), 'review_decision': result['decision'],
                'publication_exception': exception, 'limitations': result['limitations'],
                'review_dispositions': result['review_dispositions'],
                'historical_completion': path.relative_to(root).as_posix(),
                'review_attempt_note': attempt_note,
                'delivery': {'status': 'complete_historical', 'release_commit': spec['release'],
                             'ci_commit': delivered['ciCommit'], 'ci_run': delivered['ciRun'],
                             'registry_receipt': spec['registry'], 'evidence': evidence_dir(version)}})
    if result['phase_review'] not in row.setdefault('reviews', []):
        row['reviews'].append(result['phase_review'])
    status_path.write_text(json.dumps(status, indent=2) + '\n')
    return path


def verify_completion(root, row):
    """Recheck a historical completion receipt: delivery, review digests and decisions."""
    root = pathlib.Path(root).resolve()
    receipt = read_json(root, row['historical_completion'])
    version = receipt.get('version')
    require(version in RELEASES and RELEASES[version]['phase'] == row['id'] == receipt.get('phase'),
            'Historical completion identity differs')
    delivered = verify(root, version)
    require(delivered['status'] == 'delivered' and delivered['sourceSha256'] == receipt['sourceSha256']
            == row.get('accepted_source_sha256'), 'Historical completion no longer verifies delivery')
    for item in receipt['reviewArtifacts']:
        path = root / item['path']
        require(path.is_file() and sha256(path.read_bytes()) == item['sha256'], 'Review artifact changed: ' + item['path'])
    record = json.loads((root / receipt['phaseReview']).read_text())
    require(record.get('decision') == receipt['decision'] in ('accepted', 'accepted_with_deficiencies')
            and record.get('source_sha256') == receipt['sourceSha256'] and receipt['phaseReview'] in row.get('reviews', []),
            'Historical completion phase review differs')
    if RELEASES[version].get('publication_exception_required'):
        require(receipt.get('publicationException') is True and row.get('publication_exception'),
                'Historical completion lacks its publication exception')
    return receipt


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument('command', choices=['verify', 'execute', 'complete', 'observe-registry', 'observe-ci', 'observe-branch'])
    parser.add_argument('--version', required=True, choices=sorted(RELEASES))
    parser.add_argument('--commit')
    parser.add_argument('--run', type=int)
    parser.add_argument('--root', type=pathlib.Path, help='Repository root whose rules and evidence apply')
    parser.add_argument('--output', help='execute: receipt path under the evidence directory')
    parser.add_argument('--label', default='main', help='execute: where this run happened')
    parser.add_argument('--release-tree', type=pathlib.Path, help='complete: the clean delivery-branch worktree')
    parser.add_argument('--attempt-note', help='complete: JSON file recording an attempt-numbering decision')
    args = parser.parse_args(argv)
    root = (args.root or ROOT).resolve()
    if (root / 'tools/release_delivery.py').is_file() and RULES != (root / 'tools/release_delivery.py'):
        bind_rules(root)
    require(RULES is not None, 'No release_delivery rules found under ' + str(root / 'tools'))
    if args.command == 'verify':
        print(json.dumps(verify(root, args.version)))
        return 0
    if args.command == 'execute':
        require(args.output, 'execute requires --output')
        path = execute(root, args.version, args.output, args.label)
        print(json.dumps({'receipt': path.relative_to(root).as_posix(), 'executed': True}))
        return 0
    if args.command == 'complete':
        require(args.release_tree, 'complete requires --release-tree')
        note = json.loads(pathlib.Path(args.attempt_note).read_text()) if args.attempt_note else None
        path = complete(root, args.version, args.release_tree, note)
        print(json.dumps({'receipt': path.relative_to(root).as_posix(), 'completed': True}))
        return 0
    if args.command == 'observe-registry':
        path = observe_registry(root, args.version)
    elif args.command == 'observe-branch':
        path = observe_branch(root, args.version)
    else:
        token = os.environ.get('FORGEJO_TOKEN') or os.environ.get('CDOT_ENV_FORGEJO_TOKEN')
        require(token, 'Existing Forgejo read credential is unavailable')
        path = observe_ci(root, args.version, args.commit or '', args.run or 0, token)
    print(json.dumps({'receipt': str(path.relative_to(root)), 'observed': True}))
    return 0


if __name__ == '__main__':
    sys.exit(main())
