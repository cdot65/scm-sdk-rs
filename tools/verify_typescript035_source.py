#!/usr/bin/env python3
"""Verify isolated TypeScript 0.35.0 source inputs before oracle execution."""
import argparse
import hashlib
import json
import re
import importlib.util
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "contracts/sources/typescript-0.35.0"
MANIFEST_SHA256 = "0c77c29a564771ad07f482027569f137c9d6ea3db8cc78cc8e9dd76ec796b2b7"
SDK_MAINTENANCE_BASELINE = '578f2e631cbf649c073f9f8940a9ce60216c58ed'
NEW_PROFILE_DECLARATIONS = {
    'ContentTypeConfigurationSchema': (326, 'bd7bc7996b5ab309c60d3671a53e3527feb9eff99c4b5e24600f33405704fc79'),
    'ContentTypeConfigurationsSchema': (343, '326ffb36bb8ab7dfe09d173e094699ab93844cc4e6820bf9ab70231e4871b82d'),
}


def verify_model_imports(text):
    """Allow exactly the reviewed Zod import before executing the pinned model.

    This intentionally conservative check rejects other import/require tokens,
    including dynamic forms, rather than trying to interpret arbitrary TypeScript.
    The independent whole-file and Git-blob pins remain mandatory.
    """
    allowed = "import { z } from 'zod';\n"
    if not text.startswith(allowed) or re.search(r"\b(?:import|require)\b|\bfrom\s*['\"]", text[len(allowed):]):
        raise ValueError('Pinned profile model must import only the reviewed Zod module')


def verify_new_profile_declarations(text):
    """Pin new expression spans and derive optional property keys from their text."""
    properties = {}
    for name, (length, digest) in NEW_PROFILE_DECLARATIONS.items():
        matches = list(re.finditer(r'^export const (?=' + re.escape(name) + r'\b)', text, re.MULTILINE))
        if len(matches) != 1:
            raise ValueError('New profile declaration identity changed: ' + name)
        start = matches[0].end()
        expression = text[start:start + length]
        if (hashlib.sha256(expression.encode()).hexdigest() != digest
                or text[start + length:start + length + 1] != ';'):
            raise ValueError('New profile declaration changed: ' + name)
        keys = re.findall(r"^    (?:'([^']+)'|(\w+)): .+\.optional\(\),$", expression, re.MULTILINE)
        properties[name] = tuple(quoted or plain for quoted, plain in keys)
        if len(properties[name]) != 4 or len(set(properties[name])) != 4 or not expression.endswith('.passthrough()'):
            raise ValueError('New profile property extraction changed: ' + name)
    return properties


def verify_model_export_delta(text, inventory, changed_runtime_sources):
    """Parse the pinned barrel, resolving only unchanged wildcard modules.

    The inventory contains the resolved public exports of the frozen baseline.
    Wildcards may reuse that inventory only when Git provenance proves their
    source modules unchanged; changed modules must be explicitly enumerated.
    """
    baseline = {row['name']: row for row in inventory['exports']
                if row['source']['file'].startswith('src/models/')}
    statement = re.compile(r"export(?:\s+type)?\s+(\*|\{[^}]*\})\s+from\s+'\./([^']+)\.js';")
    names, position = set(), 0
    for match in statement.finditer(text):
        if text[position:match.start()].strip():
            raise ValueError('Unparsed model index export syntax')
        position = match.end()
        body, module = match.groups()
        if not re.fullmatch(r'[A-Za-z0-9_-]+', module):
            raise ValueError('Unreviewed model index module')
        filename = 'src/models/' + module + '.ts'
        known = {name for name, row in baseline.items() if row['source']['file'] == filename}
        if not known:
            raise ValueError('Model index module absent from frozen inventory: ' + filename)
        if body == '*':
            if filename in changed_runtime_sources:
                raise ValueError('Changed wildcard model exports require independent extraction: ' + filename)
            exported = known
        else:
            exported = set()
            members = body[1:-1].split(',')
            for index, member in enumerate(members):
                if not member.strip() and index == len(members) - 1:
                    continue
                item = re.fullmatch(r'\s*(?:type\s+)?([A-Za-z_$][\w$]*)(?:\s+as\s+([A-Za-z_$][\w$]*))?\s*', member)
                if not item:
                    raise ValueError('Unparsed model export member')
                name = item.group(2) or item.group(1)
                if name in exported:
                    raise ValueError('Duplicate model export: ' + name)
                if name in baseline and name not in known:
                    raise ValueError('Model export source changed: ' + name)
                if name not in baseline and (filename != 'src/models/mgmt-security-profile.ts'
                                             or name != item.group(1)):
                    raise ValueError('New model export source or alias differs: ' + name)
                exported.add(name)
        if names.intersection(exported):
            raise ValueError('Duplicate model index export')
        names.update(exported)
    if text[position:].strip() or not position:
        raise ValueError('Unparsed model index export syntax')
    added, removed = names - baseline.keys(), baseline.keys() - names
    expected = {'ContentTypeConfiguration', 'ContentTypeConfigurationSchema',
                'ContentTypeConfigurations', 'ContentTypeConfigurationsSchema'}
    if added != expected or removed:
        raise ValueError('Unexpected model export delta')
    return {'baselineExports': len(baseline), 'currentExports': len(names),
            'added': sorted(added), 'removed': sorted(removed)}

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


def verify_version_identity(manifest, package, constants):
    declarations = re.findall(r"^export const SDK_VERSION = ['\"]([^'\"]+)['\"];", constants, re.MULTILINE)
    if package.get('version') != manifest['version'] or declarations != [manifest['version']]:
        raise ValueError('Pinned package and SDK constant versions must match the source manifest')


def verify_license(manifest, baseline):
    licenses = [row for row in baseline['licenses'] if row['id'] == 'typescript']
    pinned = [row for row in manifest['files'] if row['upstreamPath'] == 'LICENSE']
    if (manifest.get('license') != {'id': 'typescript', 'spdx': 'Apache-2.0'}
            or len(licenses) != 1 or len(pinned) != 1
            or licenses[0]['spdx'] != 'Apache-2.0'
            or any(pinned[0][key] != licenses[0][key] for key in ('sha256', 'gitBlobSha1', 'bytes'))):
        raise ValueError('Pinned TypeScript license differs from frozen Apache attribution')



def verify_reused_profile_declarations(text, inventory):
    """Prove every reused declaration and its schema dependencies stayed exact.

    Spans are located by their unique export declaration, then compared at the
    frozen text length and terminating semicolon. Source line shifts are harmless;
    edits, additions to an expression, missing exports and duplicates fail closed.
    """
    filename = 'src/models/mgmt-security-profile.ts'
    declarations = {row['name']: row for row in inventory['exports']
                    if row['kind'] == 'schema' and row['source']['file'] == filename}
    pending = ['DataProtectionSchema', 'PolicyAppProtectionSchema',
               'ModelProtectionItemSchema', 'AgentProtectionItemSchema',
               'PolicyLatencySchema', 'DlpDataProfilePolicySchema']
    checked = {}
    while pending:
        name = pending.pop()
        if name in checked:
            continue
        row = declarations.get(name)
        if row is None or len(row['declarations']) != 1:
            raise ValueError('Reused profile declaration missing: ' + name)
        frozen = row['declarations'][0]
        expected = frozen['text']
        digest = hashlib.sha256(expected.encode()).hexdigest()
        if frozen['file'] != filename or digest != frozen['sha256']:
            raise ValueError('Frozen profile declaration hash changed: ' + name)
        matches = list(re.finditer(r'^export const (?=' + re.escape(name) + r'\b)', text, re.MULTILINE))
        if len(matches) != 1:
            raise ValueError('Reused profile declaration identity changed: ' + name)
        start = matches[0].end()
        actual = text[start:start + len(expected)]
        if (actual != expected or hashlib.sha256(actual.encode()).hexdigest() != digest
                or text[start + len(expected):start + len(expected) + 1] != ';'):
            raise ValueError('Reused profile declaration changed: ' + name)
        checked[name] = digest
        pending.extend(set(re.findall(r'\b[A-Za-z_]\w*Schema\b', expected)) - {name})
    return checked

def verify(source=SOURCE):
    if source.is_symlink() or any(p.is_symlink() for p in source.rglob('*')):
        raise ValueError("Pinned source cannot contain symlinks")
    raw = (source / "manifest.json").read_bytes()
    if hashlib.sha256(raw).hexdigest() != MANIFEST_SHA256:
        raise ValueError("TypeScript 0.35.0 manifest does not match reviewed pin")
    manifest = json.loads(raw)
    verify_license(manifest, json.loads((ROOT / "contracts/manifest.json").read_text()))
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
    verify_model_imports((source / 'src/models/mgmt-security-profile.ts').read_text())
    verify_reused_profile_declarations(
        (source / 'src/models/mgmt-security-profile.ts').read_text(),
        json.loads((ROOT / 'contracts/typescript.json').read_text()))
    verify_new_profile_declarations((source / 'src/models/mgmt-security-profile.ts').read_text())
    verify_model_export_delta((source / 'src/models/index.ts').read_text(),
                              json.loads((ROOT / 'contracts/typescript.json').read_text()), changed)
    package = json.loads((source / 'package.json').read_text())
    verify_version_identity(manifest, package, (source / 'src/constants.ts').read_text())
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
