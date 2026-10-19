#!/usr/bin/env python3
"""Qualify management consumers from a clean, identical committed checkout."""
import argparse
import hashlib
import importlib.util
import json
import pathlib
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / 'tools'))
from review_phase import fingerprint
import management_consumers


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def package_module(root):
    spec = importlib.util.spec_from_file_location('management_package_policy', root / 'scripts/check-package.py')
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def verify_receipt(root, path, toolchain, source_sha=None):
    root, path = pathlib.Path(root).absolute(), pathlib.Path(path)
    if any(part.is_symlink() for part in (path, *path.parents)):
        raise ValueError('Package receipt must have a regular path')
    report = json.loads(path.read_text())
    expected = source_sha or fingerprint(root)
    if (report.get('schemaVersion') != 3 or report.get('sourceSha256') != expected
            or report.get('toolchain') != toolchain or report.get('passed') is not True
            or report.get('requireClean') is not True or report.get('effectiveDirty') is not False):
        raise ValueError('Package receipt is stale, dirty or failed')
    for key, name in [('packageScriptSha256', 'scripts/check-package.py'),
                      ('wrapperSha256', 'scripts/check-management-package.py'),
                      ('oldConsumerManifestSha256', management_consumers.MANIFEST)]:
        if report.get(key) != sha(root / name):
            raise ValueError('Package receipt input differs: ' + name)
    archive = pathlib.Path(report['retainedArchive'])
    if not archive.is_absolute():
        archive = root / archive
    if (any(part.is_symlink() for part in (archive, *archive.parents)) or not archive.is_file()
            or sha(archive) != report.get('archive_sha256')):
        raise ValueError('Retained clean archive differs')
    policy = package_module(root)
    _, rustc_version, actual_host = policy.compiler_identity(root, toolchain)
    if report.get('hostTarget') != actual_host or report.get('rustcVersion') != rustc_version:
        raise ValueError('Package receipt compiler host or version differs from local toolchain')
    import tomllib
    version = tomllib.loads((root / 'crates/scm-sdk/Cargo.toml').read_text())['package']['version']
    files = policy.inspect_archive(archive.read_bytes(), f'scm-sdk-{version}/')
    vcs = policy.validate_vcs(files, report['releaseCommit'], True)
    if vcs != report.get('vcs') or len(archive.read_bytes()) != report.get('archive_bytes'):
        raise ValueError('Archive metadata differs')
    actual_files = [{'path': name, 'sha256': hashlib.sha256(data).hexdigest(), 'bytes': len(data)}
                    for name, data in sorted(files.items())]
    if report.get('files') != actual_files:
        raise ValueError('Archive member ledger differs')
    actual_sources = {item.relative_to(root / 'crates/scm-sdk').as_posix()
                      for directory in ('src', 'examples')
                      for item in (root / 'crates/scm-sdk' / directory).rglob('*') if item.is_file()}
    archived_sources = {name for name in files if name.startswith(('src/', 'examples/'))}
    if actual_sources != archived_sources:
        raise ValueError('Archive source inventory differs from reviewed source')
    for name, data in files.items():
        source_path = (root / 'crates/scm-sdk' / name) if name.startswith(('src/', 'examples/')) else None
        if name in {'LICENSE', 'NOTICE', 'README.md'}:
            source_path = root / name
        if name == 'Cargo.toml.orig':
            source_path = root / 'crates/scm-sdk/Cargo.toml'
        if source_path is not None and source_path.read_bytes() != data:
            raise ValueError('Archive source differs from reviewed source: ' + name)
    manifest = management_consumers.verify(root=root)
    expected_old = {row['path']: row['sha256'] for row in manifest['consumers']}
    if report.get('oldConsumerSources') != expected_old:
        raise ValueError('Old consumer identities differ')
    checks = report.get('checks', [])
    if not checks or any(row.get('exit_code') != 0 for row in checks):
        raise ValueError('Package commands are missing or failed')
    host = report['hostTarget']
    commands = [row['command'][2:] for row in checks]
    extracted = pathlib.Path(report.get('extractedArchiveRoot', ''))
    consumer_root = pathlib.Path(report.get('externalConsumerRoot', ''))
    if (not extracted.is_absolute() or extracted.name != 'scm-sdk'
            or extracted in (root, root / 'crates/scm-sdk')
            or '..' in extracted.parts or consumer_root != extracted.parent / 'consumer'):
        raise ValueError('Package execution roots differ from extracted archive layout')

    def archive_command_executed(command):
        matches = [row for row in checks if row['command'][2:] == command]
        return bool(matches) and all(pathlib.Path(row['cwd']) == extracted for row in matches)

    required = [['test', '--lib', '--no-default-features'], ['test', '--lib', '--all-features'],
                ['check', '--examples', '--all-features'], ['test', '--doc', '--all-features']]
    for command in required:
        if not archive_command_executed(command + ['--locked', '--offline', '--target', host]):
            raise ValueError('Missing package verification command')
    executables = {row['command'][0] for row in checks if row.get('command')}
    if len(executables) != 1 or pathlib.Path(next(iter(executables))).name not in {'cargo', 'cargo.exe'}:
        raise ValueError('Package command executable differs')
    for row in checks:
        if len(row['command']) < 3 or row['command'][1] != '+' + toolchain:
            raise ValueError('Package command compiler differs')
    manifest_text = report.get('externalConsumerManifest', '')
    if (hashlib.sha256(manifest_text.encode()).hexdigest() != report.get('externalConsumerManifestSha256')
            or tomllib.loads(manifest_text).get('features') != {'red-team': ['scm-sdk/red-team']}):
        raise ValueError('External consumer feature manifest differs')
    consumers = report.get('externalExampleConsumers', {})
    indices = [entry.get('checkIndex') for entry in consumers.values()]
    if len(indices) != 4 or any(type(index) is not int for index in indices) or len(set(indices)) != 4:
        raise ValueError('Consumer execution indices must be unique')
    probe_index = report.get('frozenProbeCheckIndex')
    if (type(probe_index) is not int or not 0 <= probe_index < len(checks) or probe_index in indices
            or commands[probe_index] != ['run', '--locked', '--offline', '--target', host]
            or pathlib.Path(checks[probe_index]['cwd']) != consumer_root):
        raise ValueError('Frozen capability consumer execution is missing')
    for name in ('response_defaults', 'red_team_dataplane', 'response_compatibility', 'red_team_management'):
        key = f'examples/{name}.rs'
        digest = hashlib.sha256(files[key]).hexdigest()
        if digest != sha(root / 'crates/scm-sdk' / key):
            raise ValueError('Packaged consumer differs from reviewed source')
        binding = consumers.get(key, {})
        index = binding.get('checkIndex')
        if (binding.get('sha256') != digest or type(index) is not int
                or not 0 <= index < len(checks)
                or binding.get('selectedFeatures') != ['red-team']
                or commands[index] != ['run', '--features', 'red-team', '--locked', '--offline', '--target', host]
                or pathlib.Path(checks[index]['cwd']) != consumer_root):
            raise ValueError('External consumer execution binding differs')
        if not archive_command_executed(['run', '--example', name, '--all-features', '--locked', '--offline', '--target', host]):
            raise ValueError('Packaged example did not execute')
    if report.get('managementConsumerSourceSha256') != sha(root / 'crates/scm-sdk/examples/red_team_management.rs'):
        raise ValueError('Management consumer binding differs')
    if report.get('publicConsumerSourceSha256') != expected_old['contracts/native-probes/src/main.rs']:
        raise ValueError('Frozen capability consumer differs')
    return report


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--toolchain', choices=['1.89.0', '1.99.0'], required=True)
    parser.add_argument('--output', type=pathlib.Path, required=True)
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    output = args.output.resolve()
    if args.check:
        verify_receipt(ROOT, output, args.toolchain)
    else:
        source = fingerprint(ROOT)
        manifest = management_consumers.verify(root=ROOT)
        commit = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip()
        worktree = ROOT / '.local/package-worktrees' / commit
        if not worktree.exists():
            worktree.parent.mkdir(parents=True, exist_ok=True)
            subprocess.run(['git', 'worktree', 'add', '--detach', str(worktree), commit], cwd=ROOT, check=True)
        if (subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=worktree, text=True).strip() != commit
                or fingerprint(worktree) != source):
            raise ValueError('Commit candidate source before clean package qualification')
        # check-package independently requires a completely clean checkout before
        # and after packaging. Receipts are written outside that checkout.
        subprocess.run([sys.executable, str(worktree / 'scripts/check-package.py'),
                        '--toolchain', args.toolchain, '--require-clean', '--output', str(output)],
                       cwd=worktree, check=True)
        if fingerprint(ROOT) != source or fingerprint(worktree) != source:
            raise ValueError('Source changed during package qualification')
        report = json.loads(output.read_text())
        report.update(schemaVersion=3, sourceSha256=source,
                      packageScriptSha256=sha(ROOT / 'scripts/check-package.py'),
                      wrapperSha256=sha(pathlib.Path(__file__)),
                      oldConsumerManifestSha256=sha(ROOT / management_consumers.MANIFEST),
                      oldConsumerSources={row['path']: row['sha256'] for row in manifest['consumers']})
        output.write_text(json.dumps(report, indent=2) + '\n')
        verify_receipt(ROOT, output, args.toolchain, source)
    print(json.dumps({'passed': True, 'toolchain': args.toolchain, 'receipt': str(output)}))


if __name__ == '__main__':
    try:
        main()
    except (ValueError, OSError, KeyError, subprocess.CalledProcessError) as error:
        sys.exit('Management package qualification failed: ' + str(error))
