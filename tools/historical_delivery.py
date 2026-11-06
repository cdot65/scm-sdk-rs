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
import io
import json
import os
import pathlib
import re
import subprocess
import sys
import tarfile
import urllib.request

def _root(argv):
    """Repository root: `--root DIR` (for an evidence-only copy run inside a historical
    release tree) or the checkout containing tools/historical_delivery.py."""
    if '--root' in argv[:-1]:
        return pathlib.Path(argv[argv.index('--root') + 1]).resolve()
    return pathlib.Path(__file__).resolve().parents[1]


ROOT = _root(sys.argv)
# Fingerprint and CI-matrix rules come from the repository being verified, so a copy run
# inside a historical release tree applies that release's own definitions.
sys.path.insert(0, str(ROOT / 'tools'))
from release_delivery import GITHUB_REMOTE, REMOTE, commit_fingerprint, expected_contexts  # noqa: E402

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
              'proofs': 'docs/checks/phase-28-published-package-{toolchain}.json'},
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


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument('command', choices=['verify', 'observe-registry', 'observe-ci', 'observe-branch'])
    parser.add_argument('--version', required=True, choices=sorted(RELEASES))
    parser.add_argument('--commit')
    parser.add_argument('--run', type=int)
    parser.add_argument('--root', type=pathlib.Path, help='Repository root (default: this checkout)')
    args = parser.parse_args(argv)
    if args.command == 'verify':
        print(json.dumps(verify(ROOT, args.version)))
        return 0
    if args.command == 'observe-registry':
        path = observe_registry(ROOT, args.version)
    elif args.command == 'observe-branch':
        path = observe_branch(ROOT, args.version)
    else:
        token = os.environ.get('FORGEJO_TOKEN') or os.environ.get('CDOT_ENV_FORGEJO_TOKEN')
        require(token, 'Existing Forgejo read credential is unavailable')
        path = observe_ci(ROOT, args.version, args.commit or '', args.run or 0, token)
    print(json.dumps({'receipt': str(path.relative_to(ROOT)), 'observed': True}))
    return 0


if __name__ == '__main__':
    sys.exit(main())
