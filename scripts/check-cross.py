#!/usr/bin/env python3
"""Record compile-only qualification of transport-independent SDK features."""
import argparse
import datetime
import importlib.util
import json
import os
import pathlib
import re
import subprocess
import sys
import time

ROOT = pathlib.Path(__file__).resolve().parents[1]
TARGETS = ('x86_64-unknown-linux-gnu', 'aarch64-unknown-linux-gnu', 'aarch64-apple-darwin', 'x86_64-pc-windows-msvc')
FEATURES = 'models,scan,management,dlp,iam,gateway,agentguard,realtime,model-security,red-team,tracing'
FORBIDDEN = ('reqwest ', 'tokio-websockets ', 'rustls ', 'aws-lc-rs ')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--target', choices=TARGETS, action='append')
    parser.add_argument('--toolchain', default='1.99.0')
    parser.add_argument('--output', type=pathlib.Path)
    args = parser.parse_args()
    if not re.fullmatch(r'\d+\.\d+\.\d+', args.toolchain):
        parser.error('Use an exact Rust toolchain version')
    spec = importlib.util.spec_from_file_location('runner', ROOT / 'scripts/check-native-exports.py')
    runner = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(runner)
    cargo = runner.cargo_path()
    rustc = cargo.with_name('rustc.exe' if os.name == 'nt' else 'rustc')
    rustup = cargo.with_name('rustup.exe' if os.name == 'nt' else 'rustup')
    env = {k:v for k,v in os.environ.items() if not k.startswith(('PANW_', 'AIRS_', 'SCM_'))}
    env['CARGO_TERM_COLOR'] = 'never'
    version = subprocess.check_output([str(rustc),'+'+args.toolchain,'-vV'],env=env,text=True)
    installed = subprocess.check_output([str(rustup),'target','list','--installed','--toolchain',args.toolchain],env=env,text=True).splitlines()
    targets = args.target or list(TARGETS)
    if any(target not in installed for target in targets):
        raise ValueError('Install the requested Rust standard-library targets with rustup target add first')
    checks = []
    for target in targets:
        for selection, features in [('minimal', []), ('products-and-caller-transports', ['--features',FEATURES])]:
            tree_command = [str(cargo),'+'+args.toolchain,'tree','-p','scm-sdk','--locked','--offline','--no-default-features',
                            '--target',target,'--edges','normal,build','--prefix','none'] + features
            tree = subprocess.run(tree_command,cwd=ROOT,env=env,text=True,capture_output=True,timeout=120)
            if tree.returncode or any(crate in tree.stdout for crate in FORBIDDEN):
                raise ValueError('Transport-independent target graph includes a native HTTP/TLS/WebSocket dependency: '+tree.stderr)
            command = [str(cargo),'+'+args.toolchain,'check','-p','scm-sdk','--lib','--locked','--offline',
                       '--no-default-features','--target',target] + features
            start = time.monotonic()
            result = subprocess.run(command,cwd=ROOT,env=env,text=True,capture_output=True,timeout=300)
            checks.append({'target':target,'selection':selection,'features':FEATURES.split(',') if features else [],
                           'command':command,'exit_code':result.returncode,'stdout':result.stdout,'stderr':result.stderr,
                           'dependencyTree':tree.stdout,'durationSeconds':round(time.monotonic()-start,3)})
            if result.returncode:
                break
    passed = len(checks)==len(targets)*2 and all(check['exit_code']==0 for check in checks)
    report = {'schemaVersion':1,'checkedAt':datetime.datetime.now(datetime.timezone.utc).isoformat(),
              'passed':passed,'toolchain':args.toolchain,'rustcVersion':version,'installedTargets':installed,
              'qualification':'compile_only_no_native_transports','checks':checks,
              'limitations':['No target binaries are linked or executed by cargo check.',
                 'Reqwest, native WebSocket, TLS, proxies, roots and native cryptography are excluded; their non-musl platform behavior remains unqualified.']}
    output = args.output or ROOT / '.local/checks' / ('cross-'+('-'.join(targets) if args.target else 'all')+'.json')
    output.parent.mkdir(parents=True,exist_ok=True)
    output.write_text(json.dumps(report,indent=2)+'\n')
    print(json.dumps({'passed':passed,'targets':targets,'qualification':report['qualification'],'evidence':str(output)}))
    if not passed:
        print(checks[-1]['stderr'],file=sys.stderr)
    return 0 if passed else 1


if __name__=='__main__':
    try:
        sys.exit(main())
    except (ValueError,OSError,subprocess.TimeoutExpired) as error:
        sys.exit('Cross compilation qualification failed: '+str(error))
