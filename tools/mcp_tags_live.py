#!/usr/bin/env python3
"""Run or verify the bounded Portkey MCP tags live workflow (phases 30–31).

`--run` is the only mode that contacts a tenant. It requires an explicitly
selected external registration, the expected tenant identifier and a new attempt
number. The example's sanitized report is wrapped with the exact runtime inputs,
binary digest and execution time. `--check` is credential-free: it validates the
receipt schema, privacy allowlist and that its runtime fingerprint is current.
"""
import argparse
import datetime
import hashlib
import json
import os
import pathlib
import re
import subprocess
import sys
import tempfile

ROOT = pathlib.Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / 'tools'))
from live_evidence import runtime_fingerprint  # noqa: E402

PHASES = (30, 31)
HOSTS = {'x86_64-unknown-linux-gnu', 'x86_64-unknown-linux-musl'}
TOOLCHAIN = '1.99.0'
OPERATIONS = {
    'mcp_integrations.create', 'mcp_integrations.list_with_options.tags_filter_includes_fixture',
    'mcp_integrations.list_with_options.tags_filter_excludes_others', 'mcp_integrations.get.tags_created',
    'mcp_integrations.update.tags_replace', 'mcp_integrations.get.tags_replaced', 'mcp_integrations.update.tags_null_clear',
    'mcp_integrations.get.tags_cleared', 'workspaces.get.selected', 'mcp_integrations.set_workspaces.enable_selected', 'mcp_servers.create', 'mcp_servers.get.tags_created',
    'mcp_servers.list.tags_filter_includes_fixture', 'mcp_servers.update.tags_null_clear', 'mcp_servers.get.tags_cleared',
    'mcp_servers.update.tags_replace', 'mcp_servers.get.tags_replaced', 'mcp_integrations.create.reconcile',
    'mcp_servers.create.reconcile',
    'mcp_servers.delete', 'mcp_integrations.delete',
}
# Schema 2 adds uncertain-fixture accounting; schema 1 receipts are historical only.
REPORT_KEYS = {'schemaVersion', 'crateVersion', 'upstreamCommit', 'tenantScopeMatched', 'serverWorkflowSelected', 'steps',
               'fixturesCreated', 'fixturesUncertain', 'fixturesDeleted', 'cleanupState', 'limitations'}
STEP_KEYS = {'operation', 'outcome'}
STEP_OPTIONAL = {'errorKind', 'status', 'verified', 'serviceCode'}


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def crate_version():
    text = (ROOT / 'crates/scm-sdk/Cargo.toml').read_text()
    return re.search(r'^version = "([^"]+)"', text, re.MULTILINE).group(1)


def validate_report(report, expected_version=None):
    """Allowlist every exported value; unexpected fields could carry tenant data."""
    if not isinstance(report, dict) or set(report) != REPORT_KEYS:
        raise ValueError('Unexpected MCP tags report fields')
    if (report['schemaVersion'] != 2 or report['crateVersion'] != (expected_version or report['crateVersion'])
            or not re.fullmatch(r'\d+\.\d+\.\d+', str(report['crateVersion']))
            or report['upstreamCommit'] != '8e5da0e756bc49ee3e82ca13df693b39fa541627' or report['tenantScopeMatched'] is not True
            or not isinstance(report['serverWorkflowSelected'], bool)
            or report['cleanupState'] not in {'not_needed', 'complete', 'incomplete'}):
        raise ValueError('Invalid MCP tags report identity')
    created, uncertain, deleted = report['fixturesCreated'], report['fixturesUncertain'], report['fixturesDeleted']
    if not all(type(value) is int and 0 <= value <= 2 for value in (created, uncertain, deleted)) or deleted > created \
            or created + uncertain > 2:
        raise ValueError('Invalid fixture accounting')
    # Mirrors rules::Accounting in the example: uncertainty never reports complete cleanup.
    expected_cleanup = ('not_needed' if created == 0 and uncertain == 0
                        else 'complete' if uncertain == 0 and deleted == created else 'incomplete')
    if report['cleanupState'] != expected_cleanup:
        raise ValueError('Cleanup state does not follow fixture accounting')
    steps = report['steps']
    if not isinstance(steps, list) or not 1 <= len(steps) <= 32:
        raise ValueError('Invalid MCP tags steps')
    for step in steps:
        if not isinstance(step, dict) or not STEP_KEYS <= set(step) <= STEP_KEYS | STEP_OPTIONAL:
            raise ValueError('Unexpected MCP tags step fields')
        if step['operation'] not in OPERATIONS or step['outcome'] not in {'ok', 'error', 'verification_failed'}:
            raise ValueError('Unexpected MCP tags step value')
        if 'errorKind' in step and not re.fullmatch(r'[A-Za-z]{1,32}', step['errorKind']):
            raise ValueError('Unexpected error kind')
        if 'status' in step and not (type(step['status']) is int and 100 <= step['status'] <= 599):
            raise ValueError('Unexpected status')
        if 'serviceCode' in step and not re.fullmatch(r'[A-Za-z0-9_.:-]{1,64}', step['serviceCode']):
            raise ValueError('Unexpected service code')
        if 'verified' in step and not isinstance(step['verified'], bool):
            raise ValueError('Unexpected verification value')
        if (step['outcome'] == 'error') != ('errorKind' in step):
            raise ValueError('Error outcome must carry exactly its error kind')
        if (step['outcome'] == 'verification_failed') != (step.get('verified') is False):
            raise ValueError('Verification outcome differs from its recorded result')
    if not all(isinstance(text, str) and len(text) <= 200 for text in report['limitations']):
        raise ValueError('Invalid limitations')
    return report


def summarize(report):
    steps = {step['operation']: step for step in report['steps']}
    verified = sorted(op for op, step in steps.items() if step['outcome'] == 'ok' and step.get('verified', True))
    return {'verifiedSteps': verified, 'failedSteps': sorted(set(steps) - set(verified)),
            'cleanupState': report['cleanupState']}


def check(path, current=True):
    envelope = json.loads(path.read_text())
    expected = {'schemaVersion', 'phase', 'attempt', 'executedAt', 'runtimeInputsSha256', 'binarySha256', 'toolchain',
                'hostTarget', 'sourceUnchanged', 'exitCode', 'workspaceSelected', 'report', 'summary'}
    if set(envelope) != expected or envelope['phase'] not in PHASES or envelope['toolchain'] != TOOLCHAIN or envelope['hostTarget'] not in HOSTS:
        raise ValueError('Unexpected MCP tags receipt envelope')
    if envelope['sourceUnchanged'] is not True or envelope['exitCode'] != 0 or not re.fullmatch(r'[0-9a-f]{64}', envelope['binarySha256']):
        raise ValueError('MCP tags receipt lacks clean execution evidence')
    if current and envelope['runtimeInputsSha256'] != runtime_fingerprint(ROOT):
        raise ValueError('MCP tags receipt is stale for the current runtime')
    # A current-runtime receipt must also name the current crate version.
    expected = crate_version() if current else None
    if envelope['summary'] != summarize(validate_report(envelope['report'], expected)):
        raise ValueError('MCP tags receipt summary differs from its report')
    executed = datetime.datetime.fromisoformat(envelope['executedAt'])
    if executed.utcoffset() != datetime.timedelta(0):
        raise ValueError('Receipt execution time must be explicit UTC')
    return envelope


def run(args):
    if not args.config or not args.expect_tsg or not 1 <= args.attempt <= 99 or args.phase not in PHASES:
        raise ValueError('Select an external config, expected tenant, phase and attempt 1–99')
    config = args.config.resolve()
    if config.is_relative_to(ROOT):
        raise ValueError('Registration must be outside the repository')
    destination = ROOT / f'docs/checks/phase-{args.phase}-mcp-tags-live-attempt-{args.attempt:02d}.json'
    if destination.exists():
        raise ValueError('Preserve previous attempts and select a new attempt number')
    cargo = pathlib.Path.home() / '.cargo/bin/cargo'
    native = subprocess.check_output([str(cargo.with_name('rustc')), f'+{TOOLCHAIN}', '-vV'], text=True)
    host = next(line[6:] for line in native.splitlines() if line.startswith('host: '))
    if host not in HOSTS:
        raise ValueError('Unreviewed host target for MCP tags live acceptance')
    env = {k: v for k, v in os.environ.items() if not k.startswith(('PANW_', 'AIRS_', 'SCM_'))}
    before = runtime_fingerprint(ROOT)
    build = [str(cargo), f'+{TOOLCHAIN}', 'build', '--locked', '-p', 'scm-sdk', '--features', 'gateway,reqwest-transport',
             '--target', host, '--example', 'mcp_tags_live']
    if subprocess.run(build, cwd=ROOT, env=env, capture_output=True, timeout=900).returncode:
        raise ValueError('MCP tags live application failed to build; no live requests were made')
    if before != runtime_fingerprint(ROOT):
        raise ValueError('Runtime source changed during build; no live requests were made')
    metadata = json.loads(subprocess.check_output([str(cargo), f'+{TOOLCHAIN}', 'metadata', '--no-deps', '--locked', '--offline',
                                                   '--format-version', '1'], cwd=ROOT, env=env, text=True))
    binary = pathlib.Path(metadata['target_directory']) / host / 'debug/examples/mcp_tags_live'
    private = pathlib.Path(tempfile.mkdtemp(prefix='scm-sdk-mcp-tags-live-'))
    raw = private / 'report.json'
    command = [str(binary), '--config', str(config), '--expect-tsg', args.expect_tsg, '--output', str(raw),
               '--recovery-output', str(private / 'recovery.json')]
    if args.workspace:
        command += ['--workspace', args.workspace]
    execution = subprocess.run(command, cwd=ROOT, env=env, text=True, capture_output=True, timeout=600)
    for name, value in (('stdout.txt', execution.stdout), ('stderr.txt', execution.stderr)):
        (private / name).write_text(value)
        (private / name).chmod(0o600)
    if execution.returncode or not raw.is_file():
        raise ValueError(f'MCP tags live workflow interrupted; private diagnostics retained at {private}')
    report = validate_report(json.loads(raw.read_text()), crate_version())
    envelope = {'schemaVersion': 1, 'phase': args.phase, 'attempt': args.attempt,
                'executedAt': datetime.datetime.now(datetime.timezone.utc).isoformat(),
                'runtimeInputsSha256': before, 'binarySha256': digest(binary), 'toolchain': TOOLCHAIN, 'hostTarget': host,
                'sourceUnchanged': before == runtime_fingerprint(ROOT), 'exitCode': execution.returncode,
                'workspaceSelected': bool(args.workspace), 'report': report, 'summary': summarize(report)}
    with destination.open('x') as stream:
        stream.write(json.dumps(envelope, indent=2) + '\n')
    print(json.dumps({'receipt': destination.relative_to(ROOT).as_posix(), **envelope['summary']}))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument('--run', action='store_true')
    mode.add_argument('--check', type=pathlib.Path, help='Credential-free current-runtime receipt verification')
    parser.add_argument('--config', type=pathlib.Path)
    parser.add_argument('--expect-tsg')
    parser.add_argument('--workspace', help='Nonproduction workspace slug for the optional MCP server workflow')
    parser.add_argument('--phase', type=int, choices=PHASES, default=30)
    parser.add_argument('--attempt', type=int, default=1)
    args = parser.parse_args()
    if args.run:
        run(args)
    else:
        envelope = check(ROOT / args.check)
        print(json.dumps({'checked': True, 'phase': envelope['phase'], **envelope['summary']}))


if __name__ == '__main__':
    main()
