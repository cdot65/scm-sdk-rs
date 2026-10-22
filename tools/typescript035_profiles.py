#!/usr/bin/env python3
"""Add pinned TypeScript 0.35.0 profile contracts without replacing frozen roots.

The runtime graph receives cloned ancestors of the five new field occurrences.
Models keep their original identities and expose these fields as optional views
of previously unknown JSON. Source verification is mandatory before generation.
"""
import argparse
import copy
import hashlib
import json
import re
from pathlib import Path

from verify_typescript035_source import verify, verify_new_profile_declarations, verify_model_export_delta, MANIFEST_SHA256

ROOT = Path(__file__).resolve().parents[1]
LEDGER = ROOT / 'contracts/typescript-035-parity.json'
PREFIX = 'typescript:0.35.0:'
CLONES = ('s009588', 's009599', 's009605', 's009606', 's009608', 's009609',
          's009612', 's009625', 's009632', 's009651', 's009653', 's009654')
CURRENT_ROOTS = {
    'ModelConfigurationSchema': 's009588', 'AiSecurityProfileSchema': 's009599',
    'PolicySchema': 's009606', 'SecurityProfileSchema': 's009612',
    'CreateSecurityProfileRequestSchema': 's009632',
    'SecurityProfileListResponseSchema': 's009651',
}
NEW_TYPES = {
    'ContentTypeConfiguration': 's_ts035_content_type_configuration',
    'ContentTypeConfigurations': 's_ts035_content_type_configurations',
}
DEFAULT_ALIAS = 's_ts035_s009653'
DEFAULT_ORIGINAL = 's009653'
WIRE_TESTS = (
    'directional_profiles_preserve_all_wire_paths_and_both_response_policies',
    'current_profile_requests_reject_new_invalid_fields_before_authentication',
    'valid_raw_directional_extensions_submit_and_decode_typed_responses',
    'profile_wire_defaults_and_empty_terminal_pages_follow_request_options',
)
FORMAT_CHECKS = {'uuid', 'rfc3339', 'datetime', 'date', 'duration', 'email', 'url', 'regex'}
PROJECTIONS = (
    ('s009588', 'enable-full-conversation-inspection', 's009590'),
    ('s009599', 'content-type-mode', 's009601'),
    ('s009599', 'content-type-configurations', 's_ts035_configurations_optional'),
    ('s009612', 'dlp_tenant_id', 's009601'),
    ('s009632', 'dlp_tenant_id', 's009601'),
)


def sha(data):
    return hashlib.sha256(data).hexdigest()


def references(value):
    if isinstance(value, dict):
        if '$ref' in value:
            yield value['$ref']
        for child in value.values():
            yield from references(child)
    elif isinstance(value, list):
        for child in value:
            yield from references(child)


def remap(value, mapping):
    if isinstance(value, dict):
        return {key: mapping.get(child, child) if key == '$ref' else remap(child, mapping)
                for key, child in value.items()}
    if isinstance(value, list):
        return [remap(child, mapping) for child in value]
    return value


def default_aliases():
    """Reviewed identity aliases; no additional default value is introduced."""
    return {DEFAULT_ALIAS: DEFAULT_ORIGINAL}


def verify_wire_tests(text):
    names = re.findall(r'^#\[tokio::test\]\s*\nasync fn (\w+)\(', text, re.MULTILINE)
    for name in WIRE_TESTS:
        if names.count(name) != 1:
            raise ValueError('Directional wire test missing or duplicated: ' + name)


class Overlay:
    def __init__(self):
        self.manifest = verify()
        inventory = json.loads((ROOT / 'contracts/typescript.json').read_text())
        self.export_delta = verify_model_export_delta(
            (ROOT / 'contracts/sources/typescript-0.35.0/src/models/index.ts').read_text(),
            inventory, self.manifest['changedRuntimeSources'])
        self.baseline = inventory['schemas']['definitions']
        self.mapping = {sid: 's_ts035_' + sid for sid in CLONES}
        closure = {sid for sid, _, _ in PROJECTIONS}
        while True:
            expanded = closure | {sid for sid, node in self.baseline.items()
                                  if closure.intersection(references(node))}
            if expanded == closure:
                break
            closure = expanded
        if closure != set(CLONES):
            raise ValueError('Unreviewed TypeScript profile ancestor closure')
        for name, sid in CURRENT_ROOTS.items():
            if not any(entry['name'] == name and entry.get('schema') == {'$ref': sid}
                       for entry in inventory['exports']):
                raise ValueError('Frozen profile root identity changed: ' + name)
        # The four protection fields are unchanged upstream. Reuse their exact
        # frozen schema references and hence their existing public Rust types.
        declarations = verify_new_profile_declarations(
            (ROOT / 'contracts/sources/typescript-0.35.0/src/models/mgmt-security-profile.ts').read_text())
        protections = declarations['ContentTypeConfigurationSchema']
        unmatched = set(protections) - self.baseline['s009588']['properties'].keys()
        if unmatched:
            raise ValueError('Unreviewed TypeScript protection keys absent from frozen ModelConfiguration: '
                             + ', '.join(sorted(unmatched)))
        config = NEW_TYPES['ContentTypeConfiguration']
        configs = NEW_TYPES['ContentTypeConfigurations']
        self.nodes = {
            config: {'kind': 'ZodObject', 'unknownFields': 'passthrough',
                     'catchall': copy.deepcopy(self.baseline['s009588']['catchall']),
                     'properties': {key: {**copy.deepcopy(self.baseline['s009588']['properties'][key]),
                                           'inputPresence': 'accepts', 'inputNull': 'rejects'}
                                    for key in protections}},
            configs: {'kind': 'ZodObject', 'unknownFields': 'passthrough',
                      'catchall': copy.deepcopy(self.baseline['s009588']['catchall']),
                      'properties': {key: {'schema': {'$ref': 's_ts035_configuration_optional'},
                                             'inputPresence': 'accepts', 'inputNull': 'rejects'}
                                     for key in declarations['ContentTypeConfigurationsSchema']}},
            's_ts035_configuration_optional': {'kind': 'ZodOptional', 'inner': {'$ref': config}},
            's_ts035_configurations_optional': {'kind': 'ZodOptional', 'inner': {'$ref': configs}},
        }
        self.roots = {PREFIX + name: self.mapping[sid] for name, sid in CURRENT_ROOTS.items()}
        self.roots.update({name + 'Schema': sid for name, sid in NEW_TYPES.items()})

    def check_baseline_nodes(self, nodes, compact=False):
        for sid in CLONES:
            expected = self.baseline[sid]
            if compact:
                expected = {key: value for key, value in expected.items()
                            if key not in {'semanticSource', 'description', 'semanticPortability', 'provider', 'effect'}}
                if expected['kind'] in {'ZodDefault', 'ZodEffects'}:
                    expected['requiresReviewedSemantics'] = True
            if nodes.get(sid) != expected:
                raise ValueError('Frozen profile node identity changed: ' + sid)

    def extend_graph(self, graph):
        self.check_baseline_nodes(graph['nodes'], compact=True)
        for name, sid in CURRENT_ROOTS.items():
            if graph['roots'].get(name) != sid:
                raise ValueError('Frozen profile root identity changed: ' + name)
        result = copy.deepcopy(graph)
        if set(self.nodes).intersection(result['nodes']) or set(self.roots).intersection(result['roots']):
            raise ValueError('TypeScript overlay identity collision')
        for sid in CLONES:
            target = self.mapping[sid]
            if target in result['nodes']:
                raise ValueError('TypeScript clone identity collision')
            result['nodes'][target] = remap(graph['nodes'][sid], self.mapping)
        result['nodes'].update(copy.deepcopy(self.nodes))
        for sid, field, child in PROJECTIONS:
            properties = result['nodes'][self.mapping[sid]]['properties']
            if field in properties:
                raise ValueError('TypeScript projection replaces a frozen field')
            properties[field] = {'schema': {'$ref': child}, 'inputPresence': 'accepts', 'inputNull': 'rejects'}
        result['roots'].update(self.roots)
        if any(result['nodes'][sid] != node for sid, node in graph['nodes'].items()):
            raise ValueError('TypeScript overlay changed a historical node')
        return result

    def augment_models(self, generator):
        self.check_baseline_nodes(generator.nodes)
        if set(self.nodes).intersection(generator.nodes):
            raise ValueError('TypeScript model node identity collision')
        for name, sid in NEW_TYPES.items():
            if (name in generator.used or sid in generator.names
                    or name in generator.declarations or name in generator.aliases):
                raise ValueError('TypeScript model identity collision')
        generator.nodes.update(copy.deepcopy(self.nodes))
        for name, sid in NEW_TYPES.items():
            generator.names[sid] = name
            generator.used.add(name)
            generator.family_exports['management'].add(name)
        for sid, field, child in PROJECTIONS:
            generator.nodes[sid] = copy.deepcopy(generator.nodes[sid])
            properties = generator.nodes[sid]['properties']
            if field in properties:
                raise ValueError('TypeScript projection replaces a model field')
            properties[field] = {'schema': {'$ref': child}, 'vendorProjection': True}
        # Both types are reachable through AiSecurityProfile; explicit emission
        # also makes development-only generation independent of that traversal.
        for name, sid in NEW_TYPES.items():
            generator.emit(sid, name)

    def supplemental_schemas(self):
        return [{'export': name + 'Schema', 'schema': sid, 'rustType': name,
                 'namespace': 'airs::management', 'status': 'typed_model',
                 'validator': 'crate::validation::parse'} for name, sid in NEW_TYPES.items()]

    def supplemental_models(self):
        return [{'export': name, 'rustType': name, 'status': 'typed_model'} for name in NEW_TYPES]

    def format_inventory(self):
        """Count unique reachable format nodes per current root, separately from G25."""
        nodes = {**self.baseline, **self.nodes,
                 **{self.mapping[sid]: remap(self.baseline[sid], self.mapping) for sid in CLONES}}
        for sid, field, child in PROJECTIONS:
            nodes[self.mapping[sid]]['properties'][field] = {'schema': {'$ref': child}}
        rows = []
        for name in sorted(CURRENT_ROOTS):
            pending, seen, sites = [self.roots[PREFIX + name]], set(), []
            while pending:
                sid = pending.pop()
                if sid in seen:
                    continue
                seen.add(sid)
                node = nodes[sid]
                sites.extend({'node': sid, 'kind': check['kind']} for check in node.get('checks', [])
                             if check['kind'] in FORMAT_CHECKS)
                pending.extend(references(node))
            rows.append({'root': PREFIX + name,
                         'formatSites': sorted(sites, key=lambda row: (row['node'], row['kind']))})
        return {'roots': len(rows), 'rootsWithFormats': sum(bool(row['formatSites']) for row in rows),
                'formatSites': sum(len(row['formatSites']) for row in rows), 'entries': rows}

    def ledger(self):
        source = 'contracts/sources/typescript-0.35.0/src/models/mgmt-security-profile.ts'
        wire = 'crates/scm-sdk/tests/management_contracts.rs'
        verify_wire_tests((ROOT / wire).read_text())
        return {
            'schemaVersion': 1, 'version': '0.35.0', 'commit': self.manifest['commit'],
            'baselineCommit': '4a80dbdb3dc14365e1c25d838ac0bd139485d48c',
            'sourceManifestSha256': MANIFEST_SHA256,
            'sourceBindings': {path: sha((ROOT / path).read_bytes()) for path in
                               (source, wire, 'tools/typescript035_profiles.py', 'tools/generate_models.py',
                                'tools/generate_validation.py', 'tools/response_defaults.py',
                                'tools/verify_typescript035_source.py',
                                'crates/scm-sdk/src/airs/management/profiles.rs',
                                'crates/scm-sdk/tests/typescript035_models.rs',
                                'crates/scm-sdk/examples/directional_profiles.rs')},
            'counts': {'addedExports': len(self.export_delta['added']), 'addedModels': len(NEW_TYPES),
                       'addedSchemas': len(self.supplemental_schemas()),
                       'projectedFields': len(PROJECTIONS), 'versionedRoots': len(CURRENT_ROOTS),
                       'clonedNodes': len(CLONES), 'defaultAliases': len(default_aliases())},
            'modelExportDelta': self.export_delta,
            'schemas': self.supplemental_schemas(), 'modelExports': self.supplemental_models(),
            'versionedRoots': {key: value for key, value in self.roots.items() if key.startswith(PREFIX)},
            'clonedNodes': self.mapping,
            'projections': [{'baselineSchema': sid, 'currentSchema': self.mapping[sid],
                             'field': field, 'valueSchema': child} for sid, field, child in PROJECTIONS],
            'defaultAliases': default_aliases(),
            'wireTests': list(WIRE_TESTS),
            'versionedFormatInventory': self.format_inventory(),
            'requestOperations': ['ProfilesClient.create', 'ProfilesClient.update'],
            'compatibility': {
                'historicalRoots': 'Unchanged 0.34.0 validation and response acceptance.',
                'currentRoots': 'New known fields reject null and wrong types as TypeScript 0.35.0 does.',
                'models': 'Five optional projections preserve unsupported JSON in extensions; typed values take precedence.',
                'responseDifference': 'Rust retains historical tolerant responses; TypeScript 0.35.0 rejects wrong-typed new fields.',
                'oracleScope': 'The actual Zod corpus is replayed for all eight roots in Mode::Request. The two newly named ContentType roots also replay that corpus in Mode::Response and Mode::ResponseWithoutDefaults; their current reachable schemas contain no enum, format, or default adaptations. The six versioned roots do not claim Zod response equivalence; tolerant profile response preservation has separate Rust regressions.',
                'liveAcceptance': 'Not established by generated contracts or local fixtures.',
            },
        }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    overlay = Overlay()
    content = json.dumps(overlay.ledger(), indent=2) + '\n'
    if args.check:
        if LEDGER.read_text() != content:
            raise ValueError('TypeScript 0.35.0 parity ledger is stale')
        graph = json.loads((ROOT / 'crates/scm-sdk/src/validation/schema_graph.json').read_text())
        # Reconstruct expected added graph from the existing historical portion.
        historical = copy.deepcopy(graph)
        for sid in set(overlay.mapping.values()) | set(overlay.nodes):
            historical['nodes'].pop(sid, None)
        for name in overlay.roots:
            historical['roots'].pop(name, None)
        if overlay.extend_graph(historical) != graph:
            raise ValueError('TypeScript 0.35.0 runtime graph differs')
        native = json.loads((ROOT / 'contracts/native-models.json').read_text())
        if (native.get('typescript035Schemas') != overlay.supplemental_schemas()
                or native.get('typescript035ModelExports') != overlay.supplemental_models()):
            raise ValueError('TypeScript 0.35.0 model ledger differs')
    else:
        LEDGER.write_text(content)
    print(json.dumps({'checked': args.check, **overlay.ledger()['counts']}))


if __name__ == '__main__':
    main()
