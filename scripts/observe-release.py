#!/usr/bin/env python3
"""Retain read-only remote tag and terminal Forgejo CI observations for phase 24.

Only the fixed canonical SDK remote or its GitHub mirror is contacted. No credentials are written to receipts.
This observes delivery; it never pushes, merges, approves jobs or publishes.
"""
import argparse
import collections
import datetime
import json
import os
import pathlib
import re
import subprocess
import sys
import urllib.error
import urllib.request

ROOT = pathlib.Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / 'tools'))
from release_delivery import REMOTE, GITHUB_REMOTE, commit_fingerprint, expected_contexts
from review_phase import fingerprint
BASE = 'https://git.cdot.io/api/v1/repos/cdot/scm-sdk-rs'


class NoRedirect(urllib.request.HTTPRedirectHandler):
    def redirect_request(self, req, fp, code, msg, headers, newurl):
        return None


def remote_tag(source, output, version="0.2.3", tag_remote="canonical"):
    remotes = {"canonical": REMOTE, "github": GITHUB_REMOTE}
    if tag_remote not in remotes:
        raise ValueError("Unreviewed tag remote")
    remote = remotes[tag_remote]
    if version not in {"0.2.3", "0.2.4"}:
        raise ValueError("Unreviewed release version")
    tag_ref = "refs/tags/v" + version
    if not re.fullmatch('[0-9a-f]{40}', source):
        raise ValueError('Expected exact source commit')
    result = subprocess.run(['git', 'ls-remote', remote, 'refs/heads/main',
                             tag_ref, tag_ref + '^{}'], cwd=ROOT,
                            capture_output=True, text=True, timeout=60, check=True)
    refs = {}
    for line in result.stdout.splitlines():
        oid, ref = line.split()
        if not re.fullmatch('[0-9a-f]{40}', oid) or ref in refs:
            raise ValueError('Unexpected remote reference')
        refs[ref] = oid
    if (refs.get(tag_ref + '^{}') != source or
            tag_ref not in refs or 'refs/heads/main' not in refs):
        raise ValueError('Annotated remote tag is absent or differs from qualified source')
    receipt = {'passed': True, 'remote': remote, 'tag': 'v' + version,
               'sourceCommit': source, 'tagObject': refs[tag_ref],
               'remoteRefs': refs}
    write(output, receipt)


def terminal_ci(source, run_number, output):
    if not re.fullmatch('[0-9a-f]{40}', source) or run_number < 1:
        raise ValueError('Expected exact source commit and positive run number')
    source_hash = fingerprint(ROOT)
    if commit_fingerprint(ROOT, source) != source_hash:
        raise ValueError('CI commit differs from current reviewed source')
    token = os.environ.get('FORGEJO_TOKEN') or os.environ.get('CDOT_ENV_FORGEJO_TOKEN')
    if not token:
        raise ValueError('Existing Forgejo read credential is unavailable')
    opener = urllib.request.build_opener(NoRedirect())

    def get(path):
        request = urllib.request.Request(BASE + path, headers={'Authorization': 'token ' + token,
                    'Accept': 'application/json', 'User-Agent': 'scm-sdk-release-observation'})
        with opener.open(request, timeout=30) as response:
            data = response.read(8 * 1024 * 1024 + 1)
        if len(data) > 8 * 1024 * 1024:
            raise ValueError('Forgejo response exceeded bound')
        return json.loads(data)

    # Page through bounded history; repo index and database ID are not interchangeable.
    run = None
    for page in range(1, 21):
        runs = get(f'/actions/runs?limit=50&page={page}').get('workflow_runs', [])
        matching = [item for item in runs if item.get('index_in_repo') == run_number]
        if matching:
            if len(matching) != 1:
                raise ValueError('Ambiguous workflow identity')
            run = matching[0]
            break
        if len(runs) < 50:
            break
    if not run or run.get('commit_sha') != source or run.get('status') != 'success':
        raise ValueError('Requested workflow is absent, unsuccessful or for another source')
    expected_url = f'https://git.cdot.io/cdot/scm-sdk-rs/actions/runs/{run_number}'
    if run.get('html_url') != expected_url:
        raise ValueError('Workflow URL differs')
    contexts = expected_contexts((ROOT / '.forgejo/workflows/ci.yml').read_text())
    prefix = f'/cdot/scm-sdk-rs/actions/runs/{run_number}/jobs/'
    jobs = {}
    for page in range(1, 21):
        rows = get(f'/statuses/{source}?limit=50&page={page}')
        for row in rows:
            target = row.get('target_url', '')
            if not re.fullmatch(re.escape(prefix) + r'\d+', target):
                continue
            context = row.get('context')
            if context not in jobs:
                jobs[context] = {key: row.get(key) for key in ('context', 'status', 'target_url')}
        if len(rows) < 50:
            break
    else:
        raise ValueError('CI status history exceeded the bounded pagination limit')
    if set(jobs) != contexts or any(row['status'] != 'success' for row in jobs.values()):
        raise ValueError('All configured CI contexts must report success for this run')
    if len({row['target_url'] for row in jobs.values()}) != len(jobs):
        raise ValueError('Duplicate CI job identity')
    if fingerprint(ROOT) != source_hash:
        raise ValueError('Source changed during observation')
    receipt = {'sourceCommit': source, 'reviewedSourceSha256': source_hash,
               'run': {key: run[key] for key in ('index_in_repo', 'html_url', 'status', 'commit_sha')},
               'jobs': sorted(jobs.values(), key=lambda row: row['context']),
               'expectedJobCount': len(contexts), 'counts': dict(collections.Counter(row['status'] for row in jobs.values())),
               'contextsMatchConfiguredMatrix': True}
    write(output, receipt)


def write(path, receipt):
    receipt['checkedAt'] = datetime.datetime.now(datetime.timezone.utc).isoformat()
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open('x') as stream:
        stream.write(json.dumps(receipt, indent=2) + '\n')
    print(json.dumps({'receipt': str(path), 'observed': True}))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('kind', choices=['tag', 'ci'])
    parser.add_argument('--source', required=True)
    parser.add_argument('--version', choices=('0.2.3', '0.2.4'), default='0.2.3')
    parser.add_argument('--tag-remote', choices=('canonical', 'github'), default='canonical')
    parser.add_argument('--run', type=int)
    parser.add_argument('--output', required=True, type=pathlib.Path)
    args = parser.parse_args()
    try:
        if args.kind != 'tag' and args.tag_remote != 'canonical':
            raise ValueError('--tag-remote applies only to tag observations')
        if args.kind == 'tag':
            remote_tag(args.source, args.output, args.version, args.tag_remote)
        elif args.run is None:
            raise ValueError('CI observation requires --run')
        else:
            terminal_ci(args.source, args.run, args.output)
    except urllib.error.HTTPError as error:
        sys.exit('Forgejo observation failed with HTTP ' + str(error.code))
    except (ValueError, OSError, KeyError, TypeError, subprocess.SubprocessError):
        sys.exit('Remote delivery observation failed; no success receipt was produced')
