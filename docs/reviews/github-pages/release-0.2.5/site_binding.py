#!/usr/bin/env python3
"""Bind the phase 31 docs-site chunk to the exact deployed site source and its receipts.

Evidence-only helper for the 0.2.5 documentation release. It lives beside the
receipts it binds (outside the SDK source fingerprint) because the published
0.2.5 delivery requires the reviewed SDK fingerprint to stay unchanged; automated
enforcement inside tools/codex_review.py is deferred to a follow-up governance
phase (see the phase 31 review_findings_disposition).

  --write   record binding.json for the current docs worktree HEAD (after deployment)
  --check   offline: recompute every binding from local git objects and the retained
            receipts and fail on any difference (site-only change, replaced or
            mismatched deployment evidence, unmerged release source).
"""
import argparse
import hashlib
import json
import pathlib
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[4]
HERE = pathlib.Path(__file__).resolve().parent
SITE = ROOT / '.local/docs-site-worktree'
RELEASE_SOURCE = '4adbe971bce30b129bbd9a1a858540f4bca6c22b'
SOURCE_SHA256 = 'adbdec87ffba019d14c68d44c509e5cf6ab5cea1a3b638bc62565cb59a31612d'
BINDING = HERE / 'binding.json'


def git(*args, cwd=ROOT):
    return subprocess.check_output(['git', *args], cwd=cwd, text=True).strip()


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def compute(commit):
    receipts = HERE / commit
    deployment, push = receipts / 'deployment.json', receipts / 'push.json'
    record = json.loads(deployment.read_text())
    pushed = json.loads(push.read_text())
    workflow = record['workflow']
    if not (record['deploymentCommit'] == commit == workflow['headSha'] == pushed['after']
            and workflow['conclusion'] == 'success' and workflow['headBranch'] == 'docs/scm-github-pages'
            and record['version'] == '0.2.5' and record.get('artifactZipSha256Verified') is True
            and all(page['matchesArtifact'] for page in record['pages'].values())
            and all(image['matchesArtifact'] and image['matchesProtectedBaseline'] for image in record['protectedImages'].values())):
        raise ValueError('Deployment receipts do not prove a successful 0.2.5 deployment of this commit')
    for name, expected in record['protectedSourceAssets'].items():
        if hashlib.sha256(subprocess.check_output(['git', 'show', f'{commit}:{name}'], cwd=ROOT)).hexdigest() != expected['sha256']:
            raise ValueError('Protected site asset differs from its deployment receipt: ' + name)
    if subprocess.run(['git', 'merge-base', '--is-ancestor', RELEASE_SOURCE, commit], cwd=ROOT).returncode:
        raise ValueError('Deployed site does not contain the published 0.2.5 release source')
    return {
        'schemaVersion': 1,
        'version': '0.2.5',
        'phase': 31,
        'chunk': 'docs-site',
        'sdkSourceSha256': SOURCE_SHA256,
        'releaseSource': RELEASE_SOURCE,
        'siteBranch': 'docs/scm-github-pages',
        'siteCommit': commit,
        'siteTree': git('rev-parse', f'{commit}^{{tree}}'),
        'docsSiteTree': git('rev-parse', f'{commit}:docs-site'),
        'deployWorkflowBlob': git('rev-parse', f'{commit}:.github/workflows/docs-deploy.yml'),
        'deploymentRun': workflow['url'],
        'deploymentReceipt': deployment.relative_to(ROOT).as_posix(),
        'deploymentReceiptSha256': digest(deployment),
        'pushReceipt': push.relative_to(ROOT).as_posix(),
        'pushReceiptSha256': digest(push),
        'artifactDigest': record['artifact']['digest'],
        'releaseSourceMerged': True,
    }


def worktree_head():
    if git('status', '--porcelain', '--untracked-files=no', cwd=SITE):
        raise ValueError('Docs worktree has uncommitted tracked changes')
    return git('rev-parse', 'HEAD', cwd=SITE)


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument('--write', action='store_true')
    mode.add_argument('--check', action='store_true')
    args = parser.parse_args(argv)
    if args.write:
        binding = compute(worktree_head())
        with BINDING.open('x') as stream:
            stream.write(json.dumps(binding, indent=2) + '\n')
    else:
        binding = json.loads(BINDING.read_text())
        if compute(binding['siteCommit']) != binding:
            raise ValueError('Docs-site binding differs from recomputed site source or receipts')
        if SITE.is_dir() and worktree_head() != binding['siteCommit']:
            raise ValueError('Docs worktree HEAD differs from the bound deployed site commit')
    print(json.dumps({'bound': True, 'siteCommit': binding['siteCommit'], 'docsSiteTree': binding['docsSiteTree']}))


if __name__ == '__main__':
    sys.exit(main())
