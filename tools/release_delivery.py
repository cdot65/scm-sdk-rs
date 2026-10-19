#!/usr/bin/env python3
"""Verify recorded registry, tag and terminal CI evidence before phase completion.

Online observations are collected by verify-publication.py and the maintainer's
Forgejo/SSH read. This offline guard binds their retained evidence to qualified
archives and the exact reviewed Git trees; repository writers remain trusted.
"""
import argparse
import hashlib
import importlib.util
import json
import pathlib
import re
import subprocess

from review_phase import EXCLUDED_DIRECTORIES, EXCLUDED_FILES, fingerprint

ROOT = pathlib.Path(__file__).resolve().parents[1]
REMOTE = 'ssh://git@git-ssh.cdot.io/cdot/scm-sdk-rs.git'
VERSION = '0.2.2'
PHASE = 20


def require(condition, message):
    if not condition:
        raise ValueError(message)


def read_record(root, relative):
    path = root / relative
    require(path.resolve().is_relative_to(root.resolve()) and not path.is_symlink(),
            'Delivery evidence must be a regular repository file')
    return json.loads(path.read_text())


def git(root, *arguments):
    return subprocess.check_output(['git', *arguments], cwd=root)


def commit_fingerprint(root, commit):
    require(isinstance(commit, str) and re.fullmatch('[0-9a-f]{40}', commit), 'Invalid source commit')
    digest = hashlib.sha256()
    listing = git(root, 'ls-tree', '-r', '-z', commit)
    for row in sorted(listing.split(b'\0')[:-1], key=lambda row: row.split(b'\t', 1)[1]):
        metadata, raw_name = row.split(b'\t', 1)
        mode, kind, object_id = metadata.decode().split()
        name = raw_name.decode()
        if name in EXCLUDED_FILES or name.startswith(EXCLUDED_DIRECTORIES):
            continue
        require(mode in {'100644', '100755'} and kind == 'blob', 'Nonregular delivery source input')
        digest.update(raw_name + b'\0' + hashlib.sha256(git(root, 'cat-file', 'blob', object_id)).digest())
    return digest.hexdigest()


def expected_contexts(workflow):
    """Read the deliberately inline JSON matrix without a YAML dependency."""
    names = re.findall(r'^name: (.+)$', workflow, re.M)
    require(len(names) == 1, 'Workflow must declare exactly one name')
    def matrix(key):
        values = re.findall(r'^\s+' + key + r': (\[.*\])$', workflow, re.M)
        require(len(values) == 1, 'Workflow matrix declaration is ambiguous: ' + key)
        values = json.loads(values[0])
        require(isinstance(values, list) and values and all(isinstance(x, str) and x for x in values)
                and len(values) == len(set(values)), 'Workflow matrix values must be unique strings')
        return values
    return {names[0] + ' / contracts-and-package (push)'} | {
        names[0] + f' / rust ({feature}, {toolchain}) (push)'
        for feature in matrix('features') for toolchain in matrix('toolchain')}


def validate_receipts(registry, tag, ci, proofs, contexts, source_sha256):
    """Cross-check independently recorded observations; never trust summary flags alone."""
    require(len(proofs) == 2 and [p.get('toolchain') for p in proofs] == ['1.89.0', '1.99.0'],
            'Both compiler package proofs are required')
    current = proofs[1]
    require(all(p.get('releaseCommit') == current.get('releaseCommit') and
                p.get('files') == current.get('files') and p.get('vcs') == current.get('vcs')
                for p in proofs), 'Compiler proofs must qualify identical files and source')
    require(registry.get('registry') == 'crates.io' and registry.get('package') == 'scm-sdk'
            and registry.get('version') == VERSION and registry.get('yanked') is False
            and 'cdot65' in registry.get('ownerLogins', []), 'Registry identity or ownership differs')
    require(registry.get('identicalFileContentsQualifiedOnMsrv') is True
            and registry.get('qualifiedToolchain') == '1.99.0', 'Registry lacks both compiler qualifications')
    for receipt_key, proof_key in [('sourceCommit', 'releaseCommit'), ('archiveSha256', 'archive_sha256'),
                                   ('archiveBytes', 'archive_bytes'), ('files', 'files'), ('vcs', 'vcs')]:
        require(registry.get(receipt_key) == current.get(proof_key), 'Registry differs from clean package: ' + receipt_key)
    refs = tag.get('remoteRefs', {})
    require(tag.get('passed') is True and tag.get('remote') == REMOTE and tag.get('tag') == 'v' + VERSION
            and tag.get('sourceCommit') == registry['sourceCommit']
            and refs.get('refs/tags/v' + VERSION + '^{}') == registry['sourceCommit']
            and isinstance(tag.get('tagObject'), str) and re.fullmatch('[0-9a-f]{40}', tag['tagObject'])
            and refs.get('refs/tags/v' + VERSION) == tag['tagObject'], 'Remote annotated tag does not match publication')
    run = ci.get('run', {})
    require(ci.get('reviewedSourceSha256') == source_sha256
            and run.get('status') == 'success' and run.get('commit_sha') == ci.get('sourceCommit')
            and refs.get('refs/heads/main') == ci.get('sourceCommit'), 'CI source, remote main, or terminal run differs')
    run_number = run.get('index_in_repo')
    require(type(run_number) is int and run_number > 0, 'Invalid workflow run identity')
    require(run.get('html_url') == f'https://git.cdot.io/cdot/scm-sdk-rs/actions/runs/{run_number}',
            'Unexpected workflow run URL')
    jobs = ci.get('jobs', [])
    require(isinstance(jobs, list) and all(isinstance(job, dict) for job in jobs), 'Invalid CI jobs')
    identities = [job.get('context') for job in jobs]
    require(len(identities) == len(contexts) and set(identities) == contexts,
            'CI contexts must exactly match the configured matrix without duplicates')
    for job in jobs:
        require(job.get('status') == 'success', 'CI job is not terminal success')
        target = job.get('target_url', '')
        prefix = f'/cdot/scm-sdk-rs/actions/runs/{run_number}/jobs/'
        require(isinstance(target, str) and re.fullmatch(re.escape(prefix) + r'\d+', target),
                'CI job belongs to another workflow run')
    require(len({job['target_url'] for job in jobs}) == len(jobs), 'CI job identities are duplicated')
    require(ci.get('expectedJobCount') == len(contexts) and ci.get('counts') == {'success': len(contexts)}
            and ci.get('contextsMatchConfiguredMatrix') is True, 'CI summary does not match verified jobs')
    return {'version': VERSION, 'sourceCommit': registry['sourceCommit'],
            'ciCommit': ci['sourceCommit'], 'jobs': len(contexts), 'passed': True}


def verify_source_commits(root, result, source_sha256):
    for commit in (result['sourceCommit'], result['ciCommit']):
        require(commit_fingerprint(root, commit) == source_sha256, 'Published or CI source differs from reviewed inputs')
        subprocess.run(['git', 'merge-base', '--is-ancestor', commit, 'HEAD'], cwd=root, check=True)
    subprocess.run(['git', 'merge-base', '--is-ancestor', result['sourceCommit'], result['ciCommit']],
                   cwd=root, check=True)


def verify(root, source_sha256=None):
    root = root.resolve()
    source_sha256 = source_sha256 or fingerprint(root)
    require(fingerprint(root) == source_sha256, 'Current source differs from reviewed source')
    status = read_record(root, 'docs/phase-status.json')
    phase = next(item for item in status['phases'] if item['id'] == PHASE)
    delivery = phase.get('delivery', {})
    registry_path = f'docs/checks/release-registry-{VERSION}.json'
    tag_path = f'docs/checks/release-tag-v{VERSION}.json'
    require(delivery.get('registry_receipt') == registry_path and delivery.get('tag_receipt') == tag_path,
            'Phase delivery must identify canonical registry and tag receipts')
    ci_path = delivery.get('ci_receipt', '')
    require(isinstance(ci_path, str) and re.fullmatch(r'docs/checks/phase-20-remote-ci-run-\d+\.json', ci_path),
            'Phase delivery must identify its terminal CI receipt')
    registry, tag, ci = [read_record(root, path) for path in (registry_path, tag_path, ci_path)]
    proofs = [read_record(root, f'docs/checks/phase-20-release-package-{compiler}.json')
              for compiler in ('1.89.0', '1.99.0')]
    spec = importlib.util.spec_from_file_location('delivery_publication', root / 'scripts/verify-publication.py')
    publication = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(publication)
    for proof in proofs:
        publication.require_proof(proof)
        archive = pathlib.Path(proof['retainedArchive'])
        require(archive.is_absolute() and archive.parent == root / '.local/artifacts'
                and not any(path.is_symlink() for path in (archive, *archive.parents)),
                'Retained archive must be a regular repository artifact')
        data = archive.read_bytes()
        require(hashlib.sha256(data).hexdigest() == proof['archive_sha256'] and len(data) == proof['archive_bytes'],
                'Retained package differs from qualified archive')
        files = publication.policy.inspect_archive(data, f'scm-sdk-{VERSION}/')
        require(publication.file_rows(files) == proof['files']
                and publication.policy.validate_vcs(files, proof['releaseCommit'], True) == proof['vcs'],
                'Retained archive files or VCS differ from clean proof')
    result = validate_receipts(registry, tag, ci, proofs,
                               expected_contexts((root / '.forgejo/workflows/ci.yml').read_text()), source_sha256)
    verify_source_commits(root, result, source_sha256)
    return result


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.parse_args()
    print(json.dumps(verify(ROOT)))
