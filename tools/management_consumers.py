#!/usr/bin/env python3
"""Verify immutable published 0.2.2 consumer sources without network access.

Execution against the candidate archive belongs to scripts/check-package.py.
This check proves the sources executed there remain the original consumers.
"""
import argparse
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
MANIFEST = 'contracts/compatibility/management-old-consumers.json'
MANIFEST_SHA256 = '10cecd5ed36db9db7f96934f47fa3d3cde49e438cdbb57fc8e82f17c0d6e12f4'
FROZEN_ROOT = 'contracts/compatibility/management-old-consumers/0.2.2'
ARCHIVE_SHA256 = 'c908cad8b27863d0e2d3987f5554b196313877788ac8cfef35c16d589910db7c'
RELEASE_COMMIT = '0716cfdcdbd522bb8fa63072d944e8fc15ba1824'
EXPECTED_NAMES = {'response_defaults', 'response_compatibility', 'red_team_dataplane', 'native-probes'}


def digest(data):
    return hashlib.sha256(data).hexdigest()


def read(root, relative):
    relative = Path(relative)
    if relative.is_absolute() or '..' in relative.parts:
        raise ValueError('Unsafe consumer path')
    path = root / relative
    for parent in (path, *path.parents):
        if parent == root:
            break
        if parent.is_symlink():
            raise ValueError('Symlink in consumer path')
    return path.read_bytes()


def verify(root=ROOT):
    root = Path(root).resolve()
    raw = read(root, MANIFEST)
    if digest(raw) != MANIFEST_SHA256:
        raise ValueError('Published consumer manifest pin differs')
    manifest = json.loads(raw)
    if (manifest['schemaVersion'] != 1 or manifest['package'] != 'scm-sdk'
            or manifest['version'] != '0.2.2' or manifest['releaseCommit'] != RELEASE_COMMIT
            or manifest['registryArchiveSha256'] != ARCHIVE_SHA256):
        raise ValueError('Published consumer origin differs')
    receipt_data = read(root, manifest['receipt']['path'])
    if digest(receipt_data) != manifest['receipt']['sha256']:
        raise ValueError('Published consumer receipt pin differs')
    receipt = json.loads(receipt_data)
    if (receipt['passed'] is not True or receipt['requireClean'] is not True
            or receipt['effectiveDirty'] is not False
            or receipt['releaseCommit'] != RELEASE_COMMIT
            or receipt['archive_sha256'] != ARCHIVE_SHA256
            or receipt['archive_bytes'] != manifest['registryArchiveBytes']):
        raise ValueError('Published consumer receipt origin differs')
    consumers = manifest['consumers']
    if len(consumers) != len(EXPECTED_NAMES) or {x['name'] for x in consumers} != EXPECTED_NAMES:
        raise ValueError('Published consumer inventory differs')
    archive_members = {row['path']: row for row in receipt['files']}
    for row in consumers:
        frozen = read(root, row['frozenPath'])
        if len(frozen) != row['bytes'] or digest(frozen) != row['sha256']:
            raise ValueError('Frozen published consumer differs: ' + row['name'])
        if read(root, row['path']) != frozen:
            raise ValueError('Current consumer differs from published bytes: ' + row['name'])
        if row['origin'] == 'published-registry-archive':
            original = archive_members[row['archiveMember']]
            if original['sha256'] != row['sha256'] or original['bytes'] != row['bytes']:
                raise ValueError('Published archive member identity differs')
        elif row['origin'] == 'published-release-package-receipt':
            if row['name'] != 'native-probes' or receipt[row['receiptField']] != row['sha256']:
                raise ValueError('Published native consumer identity differs')
        else:
            raise ValueError('Unreviewed consumer origin')
    actual = set()
    for path in (root / FROZEN_ROOT).rglob('*'):
        if path.is_symlink():
            raise ValueError('Symlink in frozen consumer corpus')
        if path.is_file():
            actual.add(path.relative_to(root).as_posix())
    if actual != {row['frozenPath'] for row in consumers}:
        raise ValueError('Unowned or missing frozen consumer files')
    return manifest


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--check', action='store_true', required=True)
    parser.parse_args()
    manifest = verify()
    print(json.dumps({'status': 'verified', 'publishedVersion': manifest['version'],
        'consumers': len(manifest['consumers']), 'registryArchiveSha256': ARCHIVE_SHA256,
        'executionClaimed': False}))


if __name__ == '__main__':
    try:
        main()
    except (ValueError, KeyError, TypeError, OSError) as error:
        raise SystemExit('Management consumer verification failed: ' + str(error))
