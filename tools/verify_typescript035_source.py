#!/usr/bin/env python3
"""Verify isolated TypeScript 0.35.0 source inputs before oracle execution."""
import argparse
import hashlib
import json
import importlib.util
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "contracts/sources/typescript-0.35.0"
MANIFEST_SHA256 = "89831a8c173665e1a84f7d985db50ba78548b6f1595e6f10db0fc9d2d0a7ba6b"

def baseline_ledger(root=ROOT):
    manifest = json.loads((root / 'contracts/manifest.json').read_text())
    raw = (root / 'contracts/sources/typescript/git-tree.json').read_bytes()
    if hashlib.sha256(raw).hexdigest() != manifest['baseline']['gitTreeLedgerSha256']:
        raise ValueError('Frozen baseline tree ledger changed')
    ledger = json.loads(raw)
    if ledger['commit'] != '4a80dbdb3dc14365e1c25d838ac0bd139485d48c':
        raise ValueError('Frozen baseline tree commit changed')
    return ledger


def verify_zod_versions(package, lock):
    version = lock['packages']['node_modules/zod']['version']
    if package['dependencies']['zod'] != '^3.24.0' or not (3, 24, 0) <= tuple(map(int, version.split('.'))) < (4, 0, 0):
        raise ValueError('Oracle Zod version does not satisfy the pinned source')


def verify(source=SOURCE):
    if source.is_symlink() or any(p.is_symlink() for p in source.rglob('*')):
        raise ValueError("Pinned source cannot contain symlinks")
    raw = (source / "manifest.json").read_bytes()
    if hashlib.sha256(raw).hexdigest() != MANIFEST_SHA256:
        raise ValueError("TypeScript 0.35.0 manifest does not match reviewed pin")
    manifest = json.loads(raw)
    if manifest["commit"] != "780c985cac90144ea1884fe6cffa58b5deec9ac4" or manifest["version"] != "0.35.0":
        raise ValueError("Unexpected TypeScript source identity")
    helper_spec = importlib.util.spec_from_file_location('frozen_contract_verifier', ROOT / 'tools/verify-contracts.py')
    helper = importlib.util.module_from_spec(helper_spec)
    helper_spec.loader.exec_module(helper)
    expected = {"manifest.json"}
    for proof in manifest['proofFiles']:
        path = source / proof['path']
        if proof['path'] not in {'commit.txt', 'git-tree.json'} or hashlib.sha256(path.read_bytes()).hexdigest() != proof['sha256']:
            raise ValueError('Git provenance proof changed')
        expected.add(proof['path'])
    commit = (source / 'commit.txt').read_bytes()
    ledger = json.loads((source / 'git-tree.json').read_text())
    if helper.git_hash('commit', commit) != manifest['commit'] or ledger['commit'] != manifest['commit']:
        raise ValueError('Git commit identity differs')
    tree = helper.git_tree_hash(ledger['files'])
    if commit.splitlines()[0].decode() != 'tree ' + tree:
        raise ValueError('Git tree identity differs')
    upstream = {r['path']: r for r in ledger['files']}
    baseline = baseline_ledger()
    old = {r['path']: r['gitBlobSha1'] for r in baseline['files'] if r['path'].startswith('src/')}
    new = {r['path']: r['gitBlobSha1'] for r in ledger['files'] if r['path'].startswith('src/')}
    changed = sorted(k for k in old.keys() | new.keys() if old.get(k) != new.get(k))
    if changed != manifest['changedRuntimeSources']:
        raise ValueError('Unexpected runtime source delta')
    for row in manifest["files"]:
        relative = Path(row["upstreamPath"])
        if relative.is_absolute() or ".." in relative.parts:
            raise ValueError("Invalid pinned source path")
        expected.add(relative.as_posix())
        path = source / relative
        if path.is_symlink() or not path.is_file():
            raise ValueError("Pinned source must be a regular file")
        data = path.read_bytes()
        origin = upstream.get(relative.as_posix(), {})
        if origin.get('gitBlobSha1') != row['gitBlobSha1'] or helper.git_hash('blob', data) != row['gitBlobSha1'] or origin.get('bytes') != len(data):
            raise ValueError('Pinned file does not match Git origin')
        if len(data) != row["bytes"] or hashlib.sha256(data).hexdigest() != row["sha256"]:
            raise ValueError("Pinned TypeScript source changed: " + relative.as_posix())
    actual = {p.relative_to(source).as_posix() for p in source.rglob("*") if p.is_file() or p.is_symlink()}
    if actual != expected:
        raise ValueError("Unexpected pinned source inventory")
    if not set(changed) <= {r['upstreamPath'] for r in manifest['files']}:
        raise ValueError('Changed runtime source not bundled')
    package = json.loads((source / 'package.json').read_text())
    verify_zod_versions(package, json.loads((ROOT / 'tools/package-lock.json').read_text()))
    if package['repository']['url'] != 'git+' + manifest['repository'] + '.git':
        raise ValueError('Canonical repository differs from published metadata')
    # Public, origin-verified fixture identifiers only; never load registrations.
    fixture = json.loads((source / 'test/models/fixtures/directional-security-profile.json').read_text())
    identifiers = {k: v for k, v in fixture.items() if k in {'dlp_tenant_id', 'tsg_id', 'csp_id', 'profile_id'}}
    if identifiers != {'dlp_tenant_id': '8259670718644550000', 'tsg_id': '100', 'csp_id': '123456', 'profile_id': '550e8400-e29b-41d4-a716-446655440000'}:
        raise ValueError('Unreviewed public fixture identifiers')
    return manifest

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", required=True)
    parser.parse_args()
    print("Verified TypeScript", verify()["version"], "source inputs")
