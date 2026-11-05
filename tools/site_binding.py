#!/usr/bin/env python3
"""Bind a release documentation-site review to its exact deployed site source.

The Docusaurus site lives on a separate branch and worktree, and its deployment
receipts live under docs/reviews/: both are outside the SDK source fingerprint.
A site-bound chunk (tools/codex_review.py SITE_BINDINGS) is therefore accepted
only while this binding verifies. It binds:

- the deployed site commit, its whole tree, its docs-site tree and its deploy workflow blob;
- the per-commit deployment and push receipts (by SHA-256) and the Pages artifact digest;
- the published release identity from the verified registry receipt; and
- the merge of that release source into the deployed site.

`--check` recomputes everything offline from local git objects and retained
receipts. Any site-only change, replaced or mismatched receipt, unmerged release
source, or a docs worktree HEAD other than the bound commit fails.
`--write` records the binding for the current docs worktree HEAD after deployment.
"""
import argparse
import hashlib
import json
import pathlib
import re
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
WORKTREE = '.local/docs-site-worktree'
SITE_BRANCH = 'docs/scm-github-pages'
RELEASES = {'0.2.5': 'docs/reviews/github-pages/release-0.2.5/binding.json'}
FIELDS = ('schemaVersion', 'version', 'sdkSourceSha256', 'releaseSource', 'siteBranch', 'siteCommit', 'siteTree',
          'docsSiteTree', 'deployWorkflowBlob', 'deploymentRun', 'deploymentReceipt', 'deploymentReceiptSha256',
          'pushReceipt', 'pushReceiptSha256', 'artifactDigest', 'releaseSourceMerged')


def require(condition, message):
    if not condition:
        raise ValueError(message)


def git(root, *args):
    return subprocess.check_output(['git', *args], cwd=root, text=True).strip()


def regular(root, relative):
    path = root / relative
    require(not any(item.is_symlink() for item in (path, *path.parents) if item != item.parent)
            and path.resolve().is_relative_to(root.resolve()) and path.is_file(),
            'Site binding evidence must be a regular repository file: ' + relative)
    return path


def published(root, version):
    """The release identity is the verified registry receipt, never the binding's own claim."""
    registry = json.loads(regular(root, f'docs/checks/release-registry-{version}.json').read_text())
    readiness = registry.get('releaseReadiness', {})
    require(registry.get('published') is not False and re.fullmatch('[0-9a-f]{40}', str(registry.get('sourceCommit')))
            and re.fullmatch('[0-9a-f]{64}', str(readiness.get('sourceSha256'))),
            'Registry receipt lacks the published release identity')
    return registry['sourceCommit'], readiness['sourceSha256']


def compute(root, binding_path, version, commit):
    require(re.fullmatch('[0-9a-f]{40}', str(commit)), 'Site commit must be an exact commit')
    receipts = binding_path.parent / commit
    deployment_path = regular(root, (receipts / 'deployment.json').relative_to(root).as_posix())
    push_path = regular(root, (receipts / 'push.json').relative_to(root).as_posix())
    deployment, push = json.loads(deployment_path.read_text()), json.loads(push_path.read_text())
    workflow = deployment.get('workflow', {})
    require(deployment.get('deploymentCommit') == commit == workflow.get('headSha') == push.get('after')
            and workflow.get('conclusion') == 'success' and workflow.get('headBranch') == SITE_BRANCH
            and push.get('ref') == 'refs/heads/' + SITE_BRANCH and push.get('exitCode') == 0
            and deployment.get('version') == version and deployment.get('artifactZipSha256Verified') is True
            and deployment.get('pages') and all(page.get('matchesArtifact') is True for page in deployment['pages'].values())
            and all(image.get('matchesArtifact') is True and image.get('matchesProtectedBaseline') is True
                    for image in deployment.get('protectedImages', {}).values()),
            'Deployment receipts do not prove a successful deployment of the bound site commit')
    for name, expected in deployment.get('protectedSourceAssets', {}).items():
        blob = subprocess.check_output(['git', 'show', f'{commit}:{name}'], cwd=root)
        require(hashlib.sha256(blob).hexdigest() == expected.get('sha256'),
                'Protected site asset differs from its deployment receipt: ' + name)
    release_source, source_sha256 = published(root, version)
    merged = subprocess.run(['git', 'merge-base', '--is-ancestor', release_source, commit], cwd=root).returncode == 0
    require(merged, 'Deployed site does not contain the published release source')
    return {
        'schemaVersion': 1, 'version': version, 'sdkSourceSha256': source_sha256, 'releaseSource': release_source,
        'siteBranch': SITE_BRANCH, 'siteCommit': commit, 'siteTree': git(root, 'rev-parse', f'{commit}^{{tree}}'),
        'docsSiteTree': git(root, 'rev-parse', f'{commit}:docs-site'),
        'deployWorkflowBlob': git(root, 'rev-parse', f'{commit}:.github/workflows/docs-deploy.yml'),
        'deploymentRun': workflow.get('url'),
        'deploymentReceipt': deployment_path.relative_to(root).as_posix(),
        'deploymentReceiptSha256': hashlib.sha256(deployment_path.read_bytes()).hexdigest(),
        'pushReceipt': push_path.relative_to(root).as_posix(),
        'pushReceiptSha256': hashlib.sha256(push_path.read_bytes()).hexdigest(),
        'artifactDigest': deployment.get('artifact', {}).get('digest'), 'releaseSourceMerged': True,
    }


def worktree_head(root):
    worktree = root / WORKTREE
    require(worktree.is_dir(), 'The docs-site worktree is required to verify the reviewed site checkout')
    require(not git(worktree, 'status', '--porcelain', '--untracked-files=no'), 'Docs worktree has uncommitted tracked changes')
    return git(worktree, 'rev-parse', 'HEAD')


def check(root, version):
    """Return the verified binding and its SHA-256; raise on any difference."""
    relative = RELEASES.get(version)
    require(relative is not None, 'Unreviewed documentation release')
    path = regular(root, relative)
    binding = json.loads(path.read_text())
    require(isinstance(binding, dict) and tuple(binding) == FIELDS, 'Unexpected site binding fields')
    require(compute(root, path, version, binding['siteCommit']) == binding,
            'Site binding differs from the recomputed site source or receipts')
    require(worktree_head(root) == binding['siteCommit'], 'Docs worktree HEAD differs from the bound deployed site commit')
    return binding, hashlib.sha256(path.read_bytes()).hexdigest()


def write(root, version):
    path = root / RELEASES[version]
    binding = compute(root, path, version, worktree_head(root))
    with path.open('x') as stream:
        stream.write(json.dumps(binding, indent=2) + '\n')
    return binding


def main(argv=None, root=ROOT):
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument('--version', required=True, choices=sorted(RELEASES))
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument('--write', action='store_true')
    mode.add_argument('--check', action='store_true')
    args = parser.parse_args(argv)
    if args.write:
        binding = write(root, args.version)
    else:
        binding, _ = check(root, args.version)
    print(json.dumps({'bound': True, 'siteCommit': binding['siteCommit'], 'docsSiteTree': binding['docsSiteTree']}))
    return 0


if __name__ == '__main__':
    sys.exit(main())
