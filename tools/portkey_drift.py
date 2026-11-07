#!/usr/bin/env python3
"""Portkey AI Gateway upstream drift detection (github.com/Portkey-AI/openapi).

Companion to tools/airs_drift.py (pan.dev). The SDK's Portkey baseline is the
vendored pin contracts/vendor/portkey-mcp-tags/8e5da0e (source.json). Watched
upstream paths, all at the repository root:

- openapi.yaml   the AI Gateway specification, classified with the same
                 fail-closed semantic classifier as pan.dev (airs_drift);
- LICENSE        any change is reported as license_changed;
- the root file listing: added, removed or changed top-level files are reported
  so new specification files are never missed.

Operation code samples (x-code-samples / x-codeSamples) are documentation: their
changes are reported as code_samples_changed annotations rather than failing as
unknown keywords. Drift is reported, never adopted.

  --check                         offline: pinned bytes, ledger and self-compare
  (no command)                    regenerate contracts/portkey-drift.json
  --baseline-tree DIR             regenerate, recording a verified refresh of the pin
  compare --candidate DIR [--output docs/checks/portkey-drift/<commit>.json]
  refresh --commit <40-hex> [--output-dir .local/portkey-drift/<commit>]   (only networked path)

Exit codes for compare: 0 unchanged, byte_only or annotation_only; 2 review_required;
1 classification failure, tampering or path error.
"""
import argparse
import copy
import json
import os
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import airs_drift as drift  # noqa: E402
import red_team_management as rtm  # noqa: E402

ROOT = rtm.ROOT
REPOSITORY = 'https://github.com/Portkey-AI/openapi'
GITHUB_API = 'https://api.github.com/repos/Portkey-AI/openapi'
RAW_BASE = 'https://raw.githubusercontent.com/Portkey-AI/openapi'
PIN_DIR = Path('contracts/vendor/portkey-mcp-tags/8e5da0e')
SPEC, LICENSE = 'openapi.yaml', 'LICENSE'
WATCHED = (SPEC, LICENSE)
LEDGER = Path('contracts/portkey-drift.json')
OUTPUT_ROOTS = (Path('docs/checks/portkey-drift'), Path('.local/portkey-drift'))
CODE_SAMPLE_KEYS = ('x-code-samples', 'x-codeSamples')
HEX40 = re.compile(r'[0-9a-f]{40}')
MAX_BYTES = 16 * 1024 * 1024
POLICY = drift.POLICY


def require(condition, message):
    if not condition:
        raise ValueError(message)


def pin(root=ROOT):
    """The vendored baseline: commit and per-file digests, verified against the bytes."""
    source = json.loads((root / PIN_DIR / 'source.json').read_text())
    require(HEX40.fullmatch(str(source.get('commit'))) and source.get('url') == REPOSITORY,
            'Portkey pin identity is invalid')
    files = {}
    for name in WATCHED:
        expected = source['files'][name]
        data = (root / PIN_DIR / name).read_bytes()
        require(drift.sha(data) == expected['sha256'] and drift.git_blob_sha1(data) == expected['gitBlobSha1']
                and len(data) == expected['bytes'], f'Pinned Portkey bytes changed: {name}')
        files[name] = {'sha256': expected['sha256'], 'gitBlobSha1': expected['gitBlobSha1'], 'bytes': expected['bytes']}
    return {'repository': REPOSITORY, 'commit': source['commit'], 'files': files,
            'path': (PIN_DIR / SPEC).as_posix()}


def strip_code_samples(spec):
    """Remove operation code samples; return the stripped spec and {pointer: samples}."""
    spec = copy.deepcopy(spec)
    samples = {}
    for route, item in (spec.get('paths') or {}).items():
        if not isinstance(item, dict):
            continue
        for method, operation in item.items():
            if method in rtm.METHODS and isinstance(operation, dict):
                for key in CODE_SAMPLE_KEYS:
                    if key in operation:
                        samples[drift.pointer(['paths', route, method, key])] = operation.pop(key)
    return spec, samples


def classify(old_path, new_path, old_bytes, new_bytes):
    """Classify the specification pair; code-sample differences become annotations."""
    old_spec, old_samples = strip_code_samples(drift.parse(old_path, old_bytes))
    new_spec, new_samples = strip_code_samples(drift.parse(new_path, new_bytes))
    result = drift.compare_sources({SPEC: {'bytes': old_bytes, 'spec': old_spec}},
                                   {SPEC: {'bytes': new_bytes, 'spec': new_spec}})
    samples = []
    for location in sorted(old_samples.keys() | new_samples.keys()):
        before, after = old_samples.get(location), new_samples.get(location)
        if rtm.encoded(before) != rtm.encoded(after):
            identity = {'path': [SPEC, location], 'kind': 'code_samples'}
            samples.append({'id': drift.digest({**identity, 'old': before, 'new': after}), 'source': SPEC,
                            'class': 'code_samples_changed', 'disposition': 'annotation', 'pointer': location})
    return result, samples


def ledger(baseline_root=None, root=ROOT):
    pinned = pin(root)
    data = (root / pinned['path']).read_bytes()
    spec = drift.parse(root / pinned['path'], data)
    document = {'schemaVersion': 1, 'classifierVersion': drift.CLASSIFIER_VERSION, 'repository': REPOSITORY,
                'snapshotCommit': pinned['commit'], 'watched': list(WATCHED), 'watchedRootListing': True,
                'pinnedFiles': pinned['files'],
                'specification': {'path': pinned['path'], 'version': (spec.get('info') or {}).get('version'),
                                  'operationCount': len(rtm.operations(spec)),
                                  'semanticModelSha256': drift.digest(drift.document_model(SPEC, strip_code_samples(spec)[0]))},
                'codeSampleKeys': list(CODE_SAMPLE_KEYS), 'policy': POLICY}
    if baseline_root is not None:
        require(baseline_root.get('commit') == pinned['commit'], 'Baseline root listing is not for the pinned commit')
        document['baselineRoot'] = baseline_root
    return document


def ledger_text(document):
    return rtm.encoded(document) + '\n'


def check(root=ROOT):
    path = root / LEDGER
    require(path.is_file() and not path.is_symlink(), 'Portkey drift ledger is missing')
    current = json.loads(path.read_text())
    expected = ledger(current.get('baselineRoot'), root)
    require(path.read_text() == ledger_text(expected), 'Stale or tampered Portkey drift ledger')
    pinned = pin(root)
    data = (root / pinned['path']).read_bytes()
    result, samples = classify(root / pinned['path'], root / pinned['path'], data, data)
    require(result['status'] == 'unchanged' and not result['changes'] and not samples,
            'Portkey baseline self-compare is not unchanged')
    return current


# ---------------------------------------------------------------- candidates

def verify_candidate(directory, root=ROOT):
    directory = Path(directory)
    tree = json.loads((directory / 'tree.json').read_text())
    require(tree.get('schemaVersion') == 1 and tree.get('repository') == REPOSITORY
            and HEX40.fullmatch(str(tree.get('commit'))), 'Candidate is not a Portkey-AI/openapi tree')
    entries = tree.get('rootEntries')
    require(isinstance(entries, list) and all(isinstance(e, dict) and e.get('path') and e.get('type') in ('blob', 'tree')
                                              and HEX40.fullmatch(str(e.get('sha'))) for e in entries),
            'Candidate root listing is malformed')
    names = {entry['path']: entry for entry in entries}
    files = {}
    for name in WATCHED:
        path = directory / 'files' / name
        require(path.is_file() and not path.is_symlink(), f'Candidate file missing: {name}')
        data = path.read_bytes()
        require(name in names and names[name]['type'] == 'blob' and drift.git_blob_sha1(data) == names[name]['sha'],
                f'Candidate file does not match its listed blob: {name}')
        files[name] = {'path': path, 'bytes': data}
    extra = {p.relative_to(directory / 'files').as_posix() for p in (directory / 'files').rglob('*') if p.is_file()}
    require(extra == set(WATCHED), 'Candidate holds unlisted files')
    pinned = pin(root)
    if tree['commit'] == pinned['commit']:
        for name in WATCHED:
            require(drift.sha(files[name]['bytes']) == pinned['files'][name]['sha256'],
                    'Same revision as the pin with different bytes: source tampering')
    return tree, files


def compare_candidate(directory, root=ROOT):
    tree, files = verify_candidate(directory, root)
    pinned = pin(root)
    base_path = root / pinned['path']
    result, samples = classify(base_path, files[SPEC]['path'], base_path.read_bytes(), files[SPEC]['bytes'])
    repository = []
    if drift.sha(files[LICENSE]['bytes']) != pinned['files'][LICENSE]['sha256']:
        repository.append({'class': 'license_changed', 'disposition': 'breaking', 'pointer': '/' + LICENSE})
    baseline = json.loads((root / LEDGER).read_text()).get('baselineRoot')
    if baseline:
        before = {e['path']: e['sha'] for e in baseline['rootEntries']}
        after = {e['path']: e['sha'] for e in tree['rootEntries']}
        for name in sorted(before.keys() | after.keys()):
            if name in WATCHED or before.get(name) == after.get(name):
                continue
            kind = 'added' if name not in before else 'removed' if name not in after else 'changed'
            repository.append({'class': f'repository_file_{kind}', 'disposition': 'review', 'pointer': '/' + name})
    status = result['status']
    if status == 'unchanged' and (samples or repository):
        status = 'annotation_only' if not repository else 'review_required'
    elif status == 'byte_only' and repository:
        status = 'review_required'
    elif status == 'byte_only' and samples:
        status = 'annotation_only'
    return {'schemaVersion': 1, 'tool': 'tools/portkey_drift.py', 'classifierVersion': drift.CLASSIFIER_VERSION,
            'baseline': {'repository': REPOSITORY, 'commit': pinned['commit']},
            'candidate': {'repository': REPOSITORY, 'commit': tree['commit'], 'rootTreeSha': tree.get('rootTreeSha')},
            'status': status, 'adoption': 'not_adopted', 'specification': result, 'codeSamples': samples,
            'repositoryFiles': repository, 'policy': POLICY}


# ---------------------------------------------------------------- output and network

def guarded(path):
    path = Path(path)
    target = (path if path.is_absolute() else ROOT / path).resolve()
    allowed = [(ROOT / item).resolve() for item in OUTPUT_ROOTS]
    require(any(target.is_relative_to(base) and target != base for base in allowed),
            'Output must be beneath docs/checks/portkey-drift/ or .local/portkey-drift/')
    return target


def refresh(commit, output_dir=None, *, fetch=None, token=None):
    """Fetch the watched root files at an exact commit into a new directory (only networked path)."""
    require(isinstance(commit, str) and HEX40.fullmatch(commit), 'Refresh requires a full 40-character lowercase hex commit')
    target = guarded(output_dir if output_dir is not None else OUTPUT_ROOTS[1] / commit)
    require(not target.exists(), 'Refresh output already exists')
    fetch = fetch or drift.urllib_fetcher()
    headers = {'User-Agent': 'scm-sdk-portkey-drift', 'Accept': 'application/vnd.github+json'}
    if token:
        headers['Authorization'] = 'Bearer ' + token

    def get(url, api=True):
        status, final, body = fetch(url, headers if api else {'User-Agent': headers['User-Agent']}, MAX_BYTES)
        require(status == 200 and final == url and len(body) <= MAX_BYTES, 'Upstream fetch failed or redirected: ' + url)
        return body
    meta = json.loads(get(f'{GITHUB_API}/commits/{commit}'))
    require(meta.get('sha') == commit, 'Upstream commit identity differs')
    root_tree = meta['commit']['tree']['sha']
    listing = json.loads(get(f'{GITHUB_API}/git/trees/{root_tree}'))
    require(listing.get('sha') == root_tree and listing.get('truncated') is False, 'Root listing is incomplete')
    entries = sorted(({'path': e['path'], 'type': e['type'], 'sha': e['sha']} for e in listing['tree']),
                     key=lambda e: e['path'])
    blobs = {e['path']: e['sha'] for e in entries if e['type'] == 'blob'}
    contents = {}
    for name in WATCHED:
        require(name in blobs, f'Upstream root lacks {name}')
        data = get(f'{RAW_BASE}/{commit}/{name}', api=False)
        require(drift.git_blob_sha1(data) == blobs[name], f'Fetched {name} does not match its blob')
        contents[name] = data
    for name, data in contents.items():
        drift.write_new(target / 'files' / name, data)
    tree = {'schemaVersion': 1, 'repository': REPOSITORY, 'commit': commit, 'rootTreeSha': root_tree,
            'committedAt': meta['commit']['committer']['date'], 'rootEntries': entries}
    drift.write_new(target / 'tree.json', (rtm.encoded(tree) + '\n').encode())
    verify_candidate(target)
    return target


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument('--check', action='store_true')
    parser.add_argument('--baseline-tree')
    sub = parser.add_subparsers(dest='command')
    compare = sub.add_parser('compare')
    compare.add_argument('--candidate', required=True)
    compare.add_argument('--output')
    fetch = sub.add_parser('refresh')
    fetch.add_argument('--commit', required=True)
    fetch.add_argument('--output-dir')
    args = parser.parse_args(argv)
    try:
        if args.command == 'refresh':
            target = refresh(args.commit, args.output_dir, token=os.environ.get('GITHUB_TOKEN'))
            print('Portkey drift refresh written: ' + str(target))
            return 0
        if args.command == 'compare':
            report = compare_candidate(args.candidate)
            text = rtm.encoded(report) + '\n'
            if args.output:
                drift.write_new(guarded(args.output), text.encode())
            print(f"Portkey drift: {report['status']}; {report['specification']['summary']['changes']} classified "
                  f"specification changes, {len(report['codeSamples'])} code-sample annotations, "
                  f"{len(report['repositoryFiles'])} repository file changes; not adopted")
            print(text, end='')
            return 2 if report['status'] == 'review_required' else 0
        if args.check:
            check()
            print('Portkey drift: pinned baseline verified; ledger current; baseline self-compare unchanged')
            return 0
        baseline_root = None
        if args.baseline_tree:
            tree, _ = verify_candidate(args.baseline_tree)
            baseline_root = {'commit': tree['commit'], 'rootTreeSha': tree['rootTreeSha'], 'rootEntries': tree['rootEntries']}
        (ROOT / LEDGER).write_text(ledger_text(ledger(baseline_root)))
        print('Portkey drift ledger regenerated: ' + LEDGER.as_posix())
        return 0
    except (ValueError, OSError, KeyError) as error:
        print('Portkey drift failed: ' + str(error), file=sys.stderr)
        return 1


if __name__ == '__main__':
    sys.exit(main())
