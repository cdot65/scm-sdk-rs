#!/usr/bin/env python3
"""Extract the real crate archive and qualify its self-contained consumer source."""
import argparse
import datetime
import hashlib
import importlib.util
import io
import json
import os
import pathlib
import re
import shutil
import subprocess
import sys
import tarfile
import tempfile
import time
import tomllib

ROOT = pathlib.Path(__file__).resolve().parents[1]


def digest(data):
    return hashlib.sha256(data).hexdigest()


def execute_package_check(command, cwd, env, timeout=600, consumer_main=None):
    """Bind an external run to the main.rs bytes present during execution."""
    source = pathlib.Path(consumer_main) if consumer_main is not None else None
    if source is not None and (source.is_symlink() or not source.is_file()):
        raise ValueError('Consumer source must be a regular file')
    before = digest(source.read_bytes()) if source is not None else None
    start = time.monotonic()
    completed = subprocess.run(command, cwd=cwd, env=env, text=True,
                               capture_output=True, timeout=timeout)
    row = {'command': command, 'cwd': str(cwd), 'exit_code': completed.returncode,
           'stdout': completed.stdout, 'stderr': completed.stderr,
           'durationSeconds': round(time.monotonic() - start, 3)}
    unchanged = True
    if source is not None:
        try:
            unchanged = not source.is_symlink() and digest(source.read_bytes()) == before
        except OSError:
            unchanged = False
        row.update(consumerSourceSha256=before, consumerSourceUnchanged=unchanged)
    row['passed'] = completed.returncode == 0 and unchanged
    return row


def inspect_archive(content, prefix):
    """Accept only bounded unique ordinary files under the exact crate directory."""
    files = {}
    total = 0
    with tarfile.open(fileobj=io.BytesIO(content), mode='r:gz') as packed:
        for member in packed:
            path = pathlib.PurePosixPath(member.name)
            if (not member.isfile() or not member.name.startswith(prefix) or path.is_absolute()
                    or '..' in path.parts or '\\' in member.name or ':' in member.name
                    or any(ord(character) < 32 or ord(character) == 127 for character in member.name)
                    or str(path) != member.name):
                raise ValueError('Unsafe archive member: ' + member.name)
            name = member.name[len(prefix):]
            relative = pathlib.PurePosixPath(name)
            if not name or relative.is_absolute() or str(relative) != name or name in files:
                raise ValueError('Invalid or duplicate archive path')
            root_files = {'Cargo.toml', 'Cargo.toml.orig', 'Cargo.lock', 'README.md', 'LICENSE', 'NOTICE', '.cargo_vcs_info.json'}
            if name.startswith(('.local/', 'contracts/', 'tools/', 'node_modules/')) or name.endswith(('.pem', '.key', '.env', '.ts', '.mts', '.js', '.yaml', '.yml')):
                raise ValueError('Development/credential input in published archive')
            if name not in root_files and not name.startswith(('src/', 'examples/')):
                raise ValueError('Unexpected file outside published source/include policy')
            total += member.size
            if member.size > 16 * 1024 * 1024 or total > 64 * 1024 * 1024 or len(files) >= 2048:
                raise ValueError('Archive resource limit exceeded')
            stream = packed.extractfile(member)
            data = stream.read(member.size + 1)
            if len(data) != member.size:
                raise ValueError('Truncated archive member')
            files[name] = data
    required = {'Cargo.toml', 'Cargo.lock', 'README.md', 'LICENSE', 'NOTICE', 'src/lib.rs',
                '.cargo_vcs_info.json', 'src/fixtures/model-regressions.json'}
    if not required <= files.keys():
        raise ValueError('Package is missing required self-contained source or release metadata')
    return files


def extract_archive(files, destination):
    # Do not use tar.extractall: no links, device files, modes or path traversal
    # from the archive are honored. The destination is a fresh private directory.
    for name, content in files.items():
        path = destination.joinpath(*pathlib.PurePosixPath(name).parts)
        path.parent.mkdir(parents=True, exist_ok=True)
        with path.open('xb') as output:
            output.write(content)


def validate_vcs(files, commit, require_clean):
    vcs = json.loads(files['.cargo_vcs_info.json'])
    git = vcs.get('git', {})
    if git.get('sha1') != commit or vcs.get('path_in_vcs') != 'crates/scm-sdk':
        raise ValueError('Archive VCS commit/path does not match the release repository')
    # Cargo's GitVcsInfo omits false via skip_serializing_if=Not::not.
    dirty = git.get('dirty', False)
    if type(dirty) is not bool:
        raise ValueError('Archive VCS dirty flag must be boolean when present')
    if require_clean and dirty:
        raise ValueError('Clean release archive has dirty source metadata')
    return vcs


def compiler_identity(root, toolchain):
    """Resolve the requested compiler locally instead of trusting receipt metadata."""
    helper_spec = importlib.util.spec_from_file_location('export_runner', root / 'scripts/check-native-exports.py')
    helper = importlib.util.module_from_spec(helper_spec)
    helper_spec.loader.exec_module(helper)
    cargo = helper.cargo_path()
    rustc = cargo.with_name('rustc.exe' if os.name == 'nt' else 'rustc')
    env = {key: value for key, value in os.environ.items() if not key.startswith(('PANW_', 'AIRS_', 'SCM_'))}
    version = subprocess.check_output([str(rustc), '+' + toolchain, '-vV'], env=env, text=True)
    host = next(line[6:] for line in version.splitlines() if line.startswith('host: '))
    return cargo, version, host


def external_consumer_manifest(manifest):
    """Enable the frozen example's cfg without changing its published source."""
    manifest = manifest.replace('path = "../../crates/scm-sdk"', 'path = "../scm-sdk"')
    if 'path = "../scm-sdk"' not in manifest or manifest.count('[dependencies]') != 1:
        raise ValueError('External probe manifest path adapter is missing or ambiguous')
    if 'features' in tomllib.loads(manifest):
        raise ValueError('External probe feature adapter requires an unambiguous manifest')
    return manifest.replace('[dependencies]', '[features]\nred-team = ["scm-sdk/red-team"]\n\n[dependencies]')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--toolchain', default='1.99.0')
    parser.add_argument('--output', type=pathlib.Path)
    parser.add_argument('--require-clean', action='store_true')
    args = parser.parse_args()
    if not re.fullmatch(r'\d+\.\d+\.\d+', args.toolchain):
        parser.error('Use an exact Rust toolchain release')
    cargo, version, host = compiler_identity(ROOT, args.toolchain)
    env = {k: v for k, v in os.environ.items() if not k.startswith(('PANW_', 'AIRS_', 'SCM_'))}
    env['CARGO_TERM_COLOR'] = 'never'
    if env.get('CARGO_BUILD_TARGET') and env['CARGO_BUILD_TARGET'] != host:
        raise ValueError('Package execution requires the actual compiler host target')
    base = [str(cargo), '+' + args.toolchain]
    commit = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip()
    status_before = subprocess.check_output(['git', 'status', '--porcelain', '--untracked-files=all'], cwd=ROOT, text=True)
    if args.require_clean and status_before:
        raise ValueError('--require-clean requires a committed, clean repository including untracked files')
    checks = []
    def run(arguments, cwd, timeout=600, consumer_main=None):
        command = base + arguments
        row = execute_package_check(command, cwd, env, timeout, consumer_main)
        checks.append(row)
        return row['passed']
    package_args = ['package', '-p', 'scm-sdk', '--locked', '--offline', '--no-verify', '--target', host]
    if not args.require_clean:
        package_args.append('--allow-dirty')
    if not run(package_args, ROOT):
        raise ValueError('cargo package failed: ' + checks[-1]['stderr'])
    metadata = json.loads(subprocess.check_output(base + ['metadata', '--no-deps', '--locked', '--offline', '--format-version', '1'], cwd=ROOT, env=env))
    package = next(item for item in metadata['packages'] if item['name'] == 'scm-sdk')
    archive = pathlib.Path(metadata['target_directory']) / 'package' / f"scm-sdk-{package['version']}.crate"
    content = archive.read_bytes()
    files = inspect_archive(content, f"scm-sdk-{package['version']}/")
    for name in ('LICENSE', 'NOTICE'):
        if files[name] != (ROOT / name).read_bytes():
            raise ValueError('Packaged legal notice differs from repository: ' + name)
    vcs = validate_vcs(files, commit, args.require_clean)
    retained = ROOT / '.local/artifacts' / f"scm-sdk-{package['version']}-{digest(content)[:16]}.crate"
    retained.parent.mkdir(parents=True, exist_ok=True)
    retained.write_bytes(content)
    with tempfile.TemporaryDirectory(prefix='scm-sdk-archive-') as temporary:
        directory = pathlib.Path(temporary)
        extracted = directory / 'scm-sdk'
        extracted.mkdir()
        extract_archive(files, extracted)
        commands = [
            ['test', '--lib', '--no-default-features'],
            ['test', '--lib', '--all-features'],
            ['check', '--examples', '--all-features'],
            ['test', '--doc', '--all-features'],
        ]
        if 'examples/response_defaults.rs' in files:
            commands.append(['run', '--example', 'response_defaults', '--all-features'])
        if 'examples/red_team_dataplane.rs' in files:
            commands.append(['run', '--example', 'red_team_dataplane', '--all-features'])
        if 'examples/access_policies.rs' in files:
            commands.append(['run', '--example', 'access_policies', '--all-features'])
        if 'examples/response_compatibility.rs' in files:
            commands.append(['run', '--example', 'response_compatibility', '--all-features'])
        if 'examples/red_team_management.rs' in files:
            commands.append(['run', '--example', 'red_team_management', '--all-features'])
        if 'examples/directional_profiles.rs' in files:
            commands.append(['run', '--example', 'directional_profiles', '--all-features'])
        passed = True
        for command in commands:
            if not run(command + ['--locked', '--offline', '--target', host], extracted):
                passed = False
                break
        consumer = directory / 'consumer'
        (consumer / 'src').mkdir(parents=True)
        original = ROOT / 'contracts/native-probes'
        manifest = external_consumer_manifest((original / 'Cargo.toml').read_text())
        (consumer / 'Cargo.toml').write_text(manifest)
        shutil.copyfile(original / 'Cargo.lock', consumer / 'Cargo.lock')
        shutil.copyfile(original / 'src/main.rs', consumer / 'src/main.rs')
        frozen_probe_check_index = None
        if passed:
            passed = run(['run', '--locked', '--offline', '--target', host], consumer, consumer_main=consumer / 'src/main.rs')
            frozen_probe_check_index = len(checks) - 1
        if passed and 'examples/access_policies.rs' in files:
            # Execute all five new public methods from a separate crate against
            # the extracted package, using the packaged offline transport example.
            locked = tomllib.loads((consumer / 'Cargo.lock').read_text())['package']
            versions = {row['name']: row['version'] for row in locked if row['name'] in {'bytes', 'http', 'tokio'}}
            extra = ''.join(f'{name} = "={versions[name]}"\n' for name in ('bytes', 'http'))
            extra += f'tokio = {{ version = "={versions["tokio"]}", features = ["macros", "rt"] }}\n'
            (consumer / 'Cargo.toml').write_text((consumer / 'Cargo.toml').read_text() + extra)
            lock = (consumer / 'Cargo.lock').read_text()
            marker = 'name = "scm-sdk-export-probes"\nversion = "0.0.0"\ndependencies = [\n'
            if lock.count(marker) != 1:
                raise ValueError('Consumer lock identity is ambiguous')
            lock = lock.replace(marker, marker + ' "bytes",\n "http",\n "tokio",\n')
            (consumer / 'Cargo.lock').write_text(lock)
            (consumer / 'src/main.rs').write_bytes(files['examples/access_policies.rs'])
            passed = run(['run', '--locked', '--offline', '--target', host], consumer, consumer_main=consumer / 'src/main.rs')
        consumer_manifest = (consumer / 'Cargo.toml').read_text()
        consumer_sources = {}
        for name in ('response_defaults', 'red_team_dataplane', 'response_compatibility', 'red_team_management'):
            key = f'examples/{name}.rs'
            if passed and key in files:
                (consumer / 'src/main.rs').write_bytes(files[key])
                passed = run(['run', '--features', 'red-team', '--locked', '--offline', '--target', host], consumer, consumer_main=consumer / 'src/main.rs')
                consumer_sources[key] = {'sha256': digest(files[key]), 'checkIndex': len(checks) - 1,
                                         'selectedFeatures': ['red-team']}
        key = 'examples/directional_profiles.rs'
        if passed and key in files:
            (consumer / 'src/main.rs').write_bytes(files[key])
            passed = run(['run', '--locked', '--offline', '--target', host], consumer, consumer_main=consumer / 'src/main.rs')
            consumer_sources[key] = {'sha256': digest(files[key]), 'checkIndex': len(checks) - 1,
                                     'selectedFeatures': []}
    if args.require_clean:
        current = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip()
        current_status = subprocess.check_output(['git', 'status', '--porcelain', '--untracked-files=all'], cwd=ROOT, text=True)
        if current != commit or current_status:
            passed = False
    report = {'schemaVersion': 2, 'checkedAt': datetime.datetime.now(datetime.timezone.utc).isoformat(),
              'passed': passed, 'toolchain': args.toolchain, 'rustcVersion': version, 'hostTarget': host,
              'releaseCommit': commit, 'requireClean': args.require_clean, 'vcs': vcs,
              'effectiveDirty': vcs['git'].get('dirty', False), 'dirtyFieldPresent': 'dirty' in vcs['git'],
              'archive_sha256': digest(content), 'archive_bytes': len(content), 'retainedArchive': str(retained),
              'files': [{'path': name, 'sha256': digest(data), 'bytes': len(data)} for name, data in sorted(files.items())],
              'publicConsumerSourceSha256': digest((ROOT / 'contracts/native-probes/src/main.rs').read_bytes()),
              'accessPolicyConsumerSourceSha256': digest(files['examples/access_policies.rs']) if 'examples/access_policies.rs' in files else None,
              'responseCompatibilityConsumerSourceSha256': digest(files['examples/response_compatibility.rs']) if 'examples/response_compatibility.rs' in files else None,
              'extractedArchiveRoot': str(extracted), 'externalConsumerRoot': str(consumer),
              'externalExampleConsumers': consumer_sources,
              'externalConsumerManifest': consumer_manifest,
              'externalConsumerManifestSha256': digest(consumer_manifest.encode()),
              'frozenProbeCheckIndex': frozen_probe_check_index,
              'managementConsumerSourceSha256': digest(files['examples/red_team_management.rs']) if 'examples/red_team_management.rs' in files else None,
              'checks': checks, 'limitations': ['Only the recorded host executes code; examples compile without contacting services. The response_defaults example, when present, also executes offline.',
                  'The full development integration corpus remains repository-only; the archive runs its packaged unit/doctests and external public capability consumer.']}
    output = args.output or ROOT / f'.local/checks/package-{args.toolchain}.json'
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps({'passed': passed, 'toolchain': args.toolchain, 'archive_sha256': report['archive_sha256'], 'evidence': str(output)}))
    if not passed:
        print(checks[-1]['stderr'], file=sys.stderr)
    return 0 if passed else 1


if __name__ == '__main__':
    try:
        sys.exit(main())
    except (ValueError, OSError, subprocess.TimeoutExpired) as error:
        sys.exit('Package qualification failed: ' + str(error))
