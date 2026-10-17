#!/usr/bin/env python3
"""Fresh pinned RustSec audit plus an enforced resolved license/source policy."""
import argparse
import datetime
import hashlib
import importlib.util
import json
import os
import pathlib
import re
import shutil
import subprocess
import sys
import tempfile
import time
import tomllib

ROOT = pathlib.Path(__file__).resolve().parents[1]
DATABASE = 'https://github.com/RustSec/advisory-db.git'
VERSIONS = {'cargo-audit':'0.22.2', 'cargo-deny':'0.20.2'}
ALLOWED_LICENSES = {'Apache-2.0', 'MIT', 'BSD-3-Clause', 'ISC', 'Zlib', 'Unicode-3.0', 'CDLA-Permissive-2.0', '0BSD', 'Unlicense', 'BSL-1.0', 'CC0-1.0', 'MIT-0'}
OUTPUT_TAIL_LIMIT = 16384


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def check_lock(lock):
    for package in lock['package']:
        if package['name'] == 'scm-sdk' and 'source' not in package:
            continue
        if package.get('source') != 'registry+https://github.com/rust-lang/crates.io-index':
            raise ValueError('Unapproved registry/git/path dependency: ' + package['name'])
        if not re.fullmatch('[a-f0-9]{64}', package.get('checksum', '')):
            raise ValueError('Missing registry checksum: ' + package['name'])


def check_policy(policy):
    if policy.get('advisories',{}).get('ignore') or policy.get('licenses',{}).get('exceptions') or policy.get('licenses',{}).get('clarify'):
        raise ValueError('No advisory or license exceptions are authorized')
    if (policy.get('graph',{}).get('exclude') or policy.get('graph',{}).get('targets')
            or not policy.get('graph',{}).get('all-features')):
        raise ValueError('Policy must include the complete all-feature graph')
    licenses = policy.get('licenses',{})
    if (set(licenses.get('allow',[])) != ALLOWED_LICENSES or licenses.get('confidence-threshold') != 0.95
            or licenses.get('private',{}).get('ignore') is not False
            or policy.get('bans',{}).get('wildcards') != 'deny'):
        raise ValueError('License/checksum source policy may not be relaxed')
    source = policy.get('sources',{})
    if source.get('unknown-registry') != 'deny' or source.get('unknown-git') != 'deny' or source.get('allow-git'):
        raise ValueError('Policy must reject unapproved dependency sources')
    if source.get('allow-registry') != ['https://github.com/rust-lang/crates.io-index']:
        raise ValueError('Only crates.io is authorized')


def output_text(value):
    if isinstance(value, bytes):
        return value.decode('utf-8', errors='replace')
    return value or ''


def output_tail(value):
    text = output_text(value)
    # A bounded scheme match avoids quadratic scans through long non-URL output.
    text = re.sub(r'(?i)([a-z][a-z0-9+.-]{0,15}://)[^/\s@]+@', r'\1[REDACTED]@', text)
    text = re.sub(r'''(?i)([?&](?:access_token|token|api[_-]?key|password|secret|client_secret)=)[^&#\s"']+''',
                  r'\1[REDACTED]', text)
    text = re.sub(r'(?im)(\b(?:proxy-)?authorization[ \t]*:)[^\r\n]*', r'\1 [REDACTED]', text)
    return text[-OUTPUT_TAIL_LIMIT:]


def write_report(args, report):
    output = args.output or ROOT / '.local/checks/dependencies.json'
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(report, indent=2) + '\n')
    return output


def print_failure_diagnostics(checks):
    failed = [item for item in checks if item['exit_code'] != 0] or checks[-1:]
    for item in failed:
        print(json.dumps({'failed_command': item['command'], 'exit_code': item['exit_code'],
                          'timeout': item.get('timeout', False),
                          'stdout_tail': item['stdout'], 'stderr_tail': item['stderr']}),
              file=sys.stderr)


def find_tool(name, directory, query):
    candidate = directory / name if directory else None
    if not candidate or not candidate.is_file():
        found = shutil.which(name)
        candidate = pathlib.Path(found) if found else pathlib.Path.home() / '.local/share/scm-sdk-tools/bin' / name
    if not candidate.is_file():
        raise ValueError('Install pinned ' + name + ' ' + VERSIONS[name] + ' or pass --tools-dir')
    version = query([str(candidate), '--version'])
    if version != name + ' ' + VERSIONS[name]:
        raise ValueError('Wrong dependency-policy tool version: ' + version)
    return candidate


def qualify(args, checks):
    env = {k:v for k,v in os.environ.items() if not k.startswith(('PANW_', 'AIRS_', 'SCM_'))}
    spec = importlib.util.spec_from_file_location('runner',ROOT/'scripts/check-native-exports.py')
    runner = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(runner)
    cargo = runner.cargo_path()
    env['PATH'] = str(cargo.parent) + os.pathsep + env.get('PATH','')
    env['RUSTUP_TOOLCHAIN'] = '1.99.0'
    env['CARGO_TERM_COLOR'] = 'never'
    lock_path = ROOT/'Cargo.lock'
    before = sha(lock_path)
    lock = tomllib.loads(lock_path.read_text())
    check_lock(lock)
    check_policy(tomllib.loads((ROOT/'deny.toml').read_text()))
    def run(command,cwd=ROOT,timeout=300):
        command = [str(value) for value in command]
        start = time.monotonic()

        def record(exit_code, stdout, stderr, timed_out=False):
            checks.append({'command': command, 'exit_code': exit_code,
                           'stdout': output_tail(stdout), 'stderr': output_tail(stderr),
                           'stdout_truncated': len(output_text(stdout)) > OUTPUT_TAIL_LIMIT,
                           'stderr_truncated': len(output_text(stderr)) > OUTPUT_TAIL_LIMIT,
                           'timeout': timed_out, 'durationSeconds': round(time.monotonic() - start, 3)})

        try:
            result = subprocess.run(command, cwd=cwd, env=env, text=True,
                                    capture_output=True, timeout=timeout)
        except subprocess.TimeoutExpired as error:
            record(None, error.stdout, error.stderr, True)
            raise
        except OSError as error:
            record(None, '', str(error))
            raise
        record(result.returncode, result.stdout, result.stderr)
        return result

    def query(command, cwd=ROOT):
        result = run(command, cwd=cwd)
        if result.returncode:
            raise ValueError('Unable to query dependency tooling or advisory provenance')
        return result.stdout.strip()

    tools = {name: find_tool(name, args.tools_dir, query) for name in VERSIONS}
    cache=ROOT/'.local/security/advisory-db.git'
    cache.parent.mkdir(parents=True,exist_ok=True)
    if not cache.exists():
        result=run(['git','clone','--bare',DATABASE,cache],timeout=180)
        if result.returncode:
            raise ValueError('Unable to fetch the authoritative RustSec database')
    origin = query(['git', '-C', str(cache), 'remote', 'get-url', 'origin'])
    if origin!=DATABASE:
        raise ValueError('Advisory cache origin differs from authoritative RustSec source')
    result=run(['git','-C',cache,'fetch','--force','origin','HEAD'],timeout=180)
    if result.returncode:
        raise ValueError('Fresh RustSec fetch failed; stale scans cannot pass the release gate')
    current = query(['git', '-C', str(cache), 'rev-parse', 'FETCH_HEAD'])
    revision=args.advisory_revision or current
    if not re.fullmatch('[a-f0-9]{40}',revision):
        raise ValueError('Use an exact 40-character advisory commit')
    with tempfile.TemporaryDirectory(prefix='scm-rustsec-') as temporary:
        database=pathlib.Path(temporary)/'advisory-db'
        result=run(['git','clone','--shared','--no-checkout',cache,database],timeout=60)
        if result.returncode or run(['git','-C',database,'checkout','--detach',revision],timeout=60).returncode:
            raise ValueError('Pinned advisory revision could not be checked out')
        actual = query(['git', '-C', str(database), 'rev-parse', 'HEAD'])
        if actual!=revision:
            raise ValueError('Advisory checkout pin mismatch')
        commit_time = query(['git', '-C', str(database), 'show', '-s', '--format=%cI', 'HEAD'])
        audit=run([tools['cargo-audit'],'audit','--file',lock_path,'--db',database,'--no-fetch','--json','--deny','warnings'])
        try:
            audit_json=json.loads(audit.stdout)
        except json.JSONDecodeError:
            raise ValueError('cargo-audit did not produce its machine-readable report')
    metadata=run([cargo,'+1.99.0','metadata','--all-features','--locked','--offline','--format-version','1'])
    if metadata.returncode:
        raise ValueError('Unable to resolve locked all-feature dependency metadata')
    packages=json.loads(metadata.stdout)['packages']
    checks.pop()  # Store compact package evidence instead of huge absolute source paths.
    deny=run([tools['cargo-deny'],'--all-features','--locked','--offline','--format','json','check','bans','licenses','sources','--deny','warnings'])
    unchanged=before==sha(lock_path)
    vulnerabilities=audit_json.get('vulnerabilities',{})
    warnings=audit_json.get('warnings',{})
    zero=vulnerabilities.get('count')==0 and not any(warnings.values())
    passed=audit.returncode==0 and deny.returncode==0 and unchanged and zero
    report={'schemaVersion':1,'checkedAt':datetime.datetime.now(datetime.timezone.utc).isoformat(),'passed':passed,
            'lockfileSha256':before,'lockfileUnchanged':unchanged,'policySha256':sha(ROOT/'deny.toml'),
            'database':{'url':DATABASE,'revision':revision,'upstreamHeadAtFetch':current,'commitTime':commit_time,'freshHead':revision==current},
            'tools':[{'name':name,'version':VERSIONS[name],'binarySha256':sha(path)} for name,path in tools.items()],
            'audit':audit_json,'packages':[{'name':p['name'],'version':p['version'],'license':p['license'],'source':p['source'],
                'rustVersion':p.get('rust_version')} for p in sorted(packages,key=lambda p:(p['name'],p['version']))],
            'resolvedPackages':len(packages),'checks':checks,
            'limitations':['RustSec covers published known advisories at the recorded revision, not all possible flaws.',
                           'License/source policy checks package metadata and detected license files; downstream applications must audit their own resolved graphs.']}
    output = write_report(args, report)
    print(json.dumps({'passed':passed,'packages':len(packages),'advisoryRevision':revision,'freshHead':revision==current,'evidence':str(output)}))
    if not passed:
        print_failure_diagnostics(checks)
    return 0 if passed else 1


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=pathlib.Path)
    parser.add_argument('--tools-dir', type=pathlib.Path)
    parser.add_argument('--advisory-revision',
                        help='Exact revision for historical replay; default fetches current upstream HEAD')
    args = parser.parse_args()
    checks = []
    try:
        return qualify(args, checks)
    except (ValueError, OSError, subprocess.SubprocessError) as error:
        message = output_tail(str(error))
        report = {'schemaVersion': 1, 'checkedAt': datetime.datetime.now(datetime.timezone.utc).isoformat(),
                  'passed': False, 'partial': True, 'failure': message, 'checks': checks,
                  'outputTailLimit': OUTPUT_TAIL_LIMIT,
                  'limitations': ['A partial failure receipt does not establish successful dependency qualification.']}
        try:
            write_report(args, report)
        except OSError:
            print('The failed dependency receipt could not be written.', file=sys.stderr)
        print('Dependency qualification failed: ' + message, file=sys.stderr)
        print_failure_diagnostics(checks)
        return 1


if __name__=='__main__':
    try:
        sys.exit(main())
    except (ValueError,OSError,subprocess.TimeoutExpired) as error:
        sys.exit('Dependency qualification failed: '+str(error))
