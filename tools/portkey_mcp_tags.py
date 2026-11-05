#!/usr/bin/env python3
"""Add the pinned Portkey 8e5da0e MCP `tags` contract without replacing frozen roots.

Request and list-query validation uses cloned, versioned `portkey:8e5da0e:` roots
whose only difference from their frozen originals is the projected `tags`
property. Response validation keeps the historical tolerant roots. Generated
models expose `tags` as typed optional views of previously unknown JSON: a
non-conforming response value remains preserved in extensions.
"""
import argparse
import copy
import hashlib
import json
import re
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
LEDGER = ROOT / 'contracts/portkey-mcp-tags-overlay.json'
PIN = ROOT / 'contracts/vendor/portkey-mcp-tags/8e5da0e'
COMMIT = '8e5da0e756bc49ee3e82ca13df693b39fa541627'
PREFIX = 'portkey:8e5da0e:'
P = 's_pk8e5_'
# Frozen request/query nodes whose complete ancestor closure is cloned.
CLONES = ('s000711', 's000769', 's000826', 's006420', 's006433', 's006434')
CURRENT_ROOTS = {
    'McpIntegrationCreateRequestSchema': 's006420',
    'McpIntegrationUpdateRequestSchema': 's006433',
    'GatewayMcpServersClientCreateRequestSchema': 's000769',
    'GatewayMcpServersClientUpdateRequestSchema': 's000826',
    'GatewayMcpServersClientListOptionsSchema': 's000711',
}
NEW_TYPES = {'GatewayMcpIntegrationsClientListOptions': P + 'mcp_integrations_list_options'}
WRITE_TAGS = P + 'write_tags_optional'
READ_TAGS = P + 'read_tags_optional'
FILTER = P + 'tags_filter_optional'
# (frozen schema, upstream schema, projected field value schema)
REQUEST_PROJECTIONS = (
    ('s006420', 'CreateMcpIntegration', WRITE_TAGS),
    ('s006434', 'UpdateMcpIntegration', WRITE_TAGS),
    ('s000769', 'CreateMcpServer', WRITE_TAGS),
    ('s000826', 'UpdateMcpServer', WRITE_TAGS),
    ('s000711', '/mcp-servers', FILTER),
)
RESPONSE_PROJECTIONS = (
    ('s007458', 'McpIntegration', READ_TAGS),
    ('s007421', 'McpIntegrationListItem', READ_TAGS),
    ('s000786', 'McpServer', READ_TAGS),
    ('s000723', 'McpServerListItem', READ_TAGS),
)
# Equivalent to upstream's documented key rule; reuses the exact reviewed source in
# validation.rs's fail-closed regex registry.
TAG_KEY = r'^[a-zA-Z0-9_-]+$'
WIRE_TESTS = (
    'tags_write_bodies_preserve_omission_null_and_value_for_all_four_writes',
    'invalid_tags_and_filters_are_rejected_before_authentication',
    'list_filters_send_exact_queries_and_frozen_list_is_unchanged',
    'responses_type_conforming_tags_and_preserve_nonconforming_values',
)


def semantic_aliases():
    """Cloned semantic nodes reuse their frozen originals' reviewed bindings."""
    return {P + 's006433': 's006433'}


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


def string(*checks):
    return {'kind': 'ZodString', 'checks': list(checks), 'coerce': False}


def optional(inner):
    return {'kind': 'ZodOptional', 'inner': {'$ref': inner}}


def integer(minimum, maximum=None):
    checks = [{'kind': 'finite', 'message': {'$undefined': True}}, {'kind': 'int', 'message': {'$undefined': True}},
              {'kind': 'min', 'value': minimum, 'inclusive': True, 'message': {'$undefined': True}}]
    if maximum is not None:
        checks.append({'kind': 'max', 'value': maximum, 'inclusive': True, 'message': {'$undefined': True}})
    return {'kind': 'ZodNumber', 'checks': checks, 'coerce': False}


def accepts(schema, null='rejects'):
    return {'schema': {'$ref': schema}, 'inputPresence': 'accepts', 'inputNull': null}


class Overlay:
    def __init__(self):
        inventory = json.loads((ROOT / 'contracts/typescript.json').read_text())
        self.baseline = inventory['schemas']['definitions']
        self.contract = json.loads((PIN / 'contract.json').read_text())
        if self.contract['commit'] != COMMIT:
            raise ValueError('Portkey contract identity drift')
        self.mapping = {sid: P + sid for sid in CLONES}
        closure = {sid for sid, _, _ in REQUEST_PROJECTIONS}
        while True:
            expanded = closure | {sid for sid, node in self.baseline.items() if closure.intersection(references(node))}
            if expanded == closure:
                break
            closure = expanded
        if closure != set(CLONES):
            raise ValueError('Unreviewed Portkey MCP ancestor closure')
        for name, sid in CURRENT_ROOTS.items():
            if not any(entry['name'] == name and entry.get('schema') == {'$ref': sid} for entry in inventory['exports']):
                raise ValueError('Frozen MCP root identity changed: ' + name)
        for sid, _, _ in REQUEST_PROJECTIONS + RESPONSE_PROJECTIONS:
            if 'tags' in self.baseline[sid].get('properties', {}):
                raise ValueError('Frozen MCP schema already declares tags: ' + sid)
        self.verify_contract()
        record = lambda key: {'kind': 'ZodRecord', 'key': {'$ref': key}, 'value': {'$ref': P + 'tag_value'}}
        query = self.contract['paths']['/mcp-integrations']['get']['parameters']
        enum = next(row for row in query if row['name'] == 'type')['schema']['enum']
        self.nodes = {
            P + 'write_tag_key': string({'kind': 'regex', 'regex': {'$regexp': TAG_KEY, 'flags': ''}}),
            P + 'read_tag_key': string(),
            P + 'tag_value': string(),
            P + 'write_tags': record(P + 'write_tag_key'),
            P + 'read_tags': record(P + 'read_tag_key'),
            P + 'write_tags_nullable': {'kind': 'ZodNullable', 'inner': {'$ref': P + 'write_tags'}},
            P + 'read_tags_nullable': {'kind': 'ZodNullable', 'inner': {'$ref': P + 'read_tags'}},
            WRITE_TAGS: optional(P + 'write_tags_nullable'),
            READ_TAGS: optional(P + 'read_tags_nullable'),
            P + 'tags_filter': string(),
            FILTER: optional(P + 'tags_filter'),
            P + 'uuid': string({'kind': 'uuid'}),
            P + 'integration_type': {'kind': 'ZodEnum', 'values': list(enum)},
            P + 'page_size': integer(1, 1000),
            P + 'current_page': integer(0),
            P + 'text': string(),
            **{P + 'optional_' + name: optional(P + name) for name in ('uuid', 'integration_type', 'page_size', 'current_page', 'text')},
            P + 'never': {'kind': 'ZodNever'},
            NEW_TYPES['GatewayMcpIntegrationsClientListOptions']: {
                'kind': 'ZodObject', 'unknownFields': 'strict', 'catchall': {'$ref': P + 'never'},
                'properties': {
                    'organisation_id': accepts(P + 'optional_uuid'),
                    'type': accepts(P + 'optional_integration_type'),
                    'workspace_id': accepts(P + 'optional_text'),
                    'page_size': accepts(P + 'optional_page_size'),
                    'current_page': accepts(P + 'optional_current_page'),
                    'search': accepts(P + 'optional_text'),
                    'tags': accepts(FILTER),
                }},
        }
        self.roots = {PREFIX + name: self.mapping[sid] for name, sid in CURRENT_ROOTS.items()}
        self.roots.update({name + 'Schema': sid for name, sid in NEW_TYPES.items()})

    def verify_contract(self):
        """The overlay encodes exactly the pinned upstream fragments."""
        schemas = self.contract['components']['schemas']
        for _, name, _ in REQUEST_PROJECTIONS[:4] + RESPONSE_PROJECTIONS:
            tags = schemas[name]['properties']['tags']
            if (tags['type'], tags['additionalProperties'], tags['nullable']) != ('object', {'type': 'string'}, True):
                raise ValueError('Pinned tags shape changed: ' + name)
        for name in ('CreateMcpIntegration', 'CreateMcpServer'):
            if 'alphanumeric characters, underscores and hyphens' not in schemas[name]['properties']['tags']['description']:
                raise ValueError('Pinned tag key rule changed: ' + name)
        query = {path: {row['name']: row for row in op['get']['parameters']} for path, op in self.contract['paths'].items()}
        if set(query['/mcp-integrations']) != {'organisation_id', 'type', 'workspace_id', 'page_size', 'current_page', 'search', 'tags'}:
            raise ValueError('Pinned integration list query changed')
        rows = query['/mcp-integrations']
        if (rows['page_size']['schema'] != {'type': 'integer', 'minimum': 1, 'maximum': 1000, 'default': 100}
                or rows['current_page']['schema'] != {'type': 'integer', 'minimum': 0, 'default': 0}
                or rows['organisation_id']['schema'] != {'type': 'string', 'format': 'uuid'}):
            raise ValueError('Pinned integration list bounds changed')
        if any(query[path]['tags']['schema'] != {'type': 'string'} for path in query):
            raise ValueError('Pinned tags filter changed')

    def check_baseline_nodes(self, nodes, compact=False):
        for sid in CLONES:
            expected = self.baseline[sid]
            if compact:
                original = expected
                expected = {key: value for key, value in expected.items()
                            if key not in {'semanticSource', 'description', 'semanticPortability', 'provider', 'effect'}}
                if 'effect' in original:
                    expected['effect'] = {'type': original['effect']['type']}
                if expected['kind'] in {'ZodDefault', 'ZodEffects'}:
                    expected['requiresReviewedSemantics'] = True
            if nodes.get(sid) != expected:
                raise ValueError('Frozen MCP node identity changed: ' + sid)

    def extend_graph(self, graph):
        self.check_baseline_nodes(graph['nodes'], compact=True)
        for name, sid in CURRENT_ROOTS.items():
            if graph['roots'].get(name) != sid:
                raise ValueError('Frozen MCP root identity changed: ' + name)
        result = copy.deepcopy(graph)
        if set(self.nodes).intersection(result['nodes']) or set(self.roots).intersection(result['roots']):
            raise ValueError('Portkey overlay identity collision')
        for sid in CLONES:
            if self.mapping[sid] in result['nodes']:
                raise ValueError('Portkey clone identity collision')
            result['nodes'][self.mapping[sid]] = remap(graph['nodes'][sid], self.mapping)
        result['nodes'].update(copy.deepcopy(self.nodes))
        for sid, _, child in REQUEST_PROJECTIONS:
            properties = result['nodes'][self.mapping[sid]]['properties']
            if 'tags' in properties:
                raise ValueError('Portkey projection replaces a frozen field')
            properties['tags'] = accepts(child, 'accepts' if child == WRITE_TAGS else 'rejects')
        result['roots'].update(self.roots)
        if any(result['nodes'][sid] != node for sid, node in graph['nodes'].items()):
            raise ValueError('Portkey overlay changed a historical node')
        return result

    def augment_models(self, generator):
        self.check_baseline_nodes(generator.nodes)
        if set(self.nodes).intersection(generator.nodes):
            raise ValueError('Portkey model node identity collision')
        for name, sid in NEW_TYPES.items():
            if name in generator.used or sid in generator.names or name in generator.declarations or name in generator.aliases:
                raise ValueError('Portkey model identity collision')
        generator.nodes.update(copy.deepcopy(self.nodes))
        for name, sid in NEW_TYPES.items():
            generator.names[sid] = name
            generator.used.add(name)
            generator.family_exports['gateway'].add(name)
        for sid, _, child in REQUEST_PROJECTIONS + RESPONSE_PROJECTIONS:
            generator.nodes[sid] = copy.deepcopy(generator.nodes[sid])
            properties = generator.nodes[sid]['properties']
            if 'tags' in properties:
                raise ValueError('Portkey projection replaces a model field')
            properties['tags'] = {'schema': {'$ref': child}, 'vendorProjection': True}
        # Typed request/query models validate against their versioned clone, so
        # `parse`/`build_validated` accept exactly what the clients accept. The
        # frozen standalone schema names keep their historical acceptance.
        for sid in CURRENT_ROOTS.values():
            if sid in generator.schema_overrides:
                raise ValueError('Portkey schema override collision: ' + sid)
            generator.schema_overrides[sid] = self.mapping[sid]
        for name, sid in NEW_TYPES.items():
            generator.emit(sid, name)

    def supplemental_schemas(self):
        return [{'export': name + 'Schema', 'schema': sid, 'rustType': name, 'namespace': 'airs::gateway',
                 'status': 'typed_model', 'validator': 'crate::validation::parse'} for name, sid in NEW_TYPES.items()]

    def supplemental_models(self):
        return [{'export': name, 'rustType': name, 'status': 'typed_model'} for name in NEW_TYPES]

    def ledger(self):
        wire = 'crates/scm-sdk/tests/portkey_mcp_tags_contracts.rs'
        text = (ROOT / wire).read_text()
        for name in WIRE_TESTS:
            if len(re.findall(rf'^async fn {name}\(', text, re.MULTILINE)) != 1:
                raise ValueError('Portkey MCP wire test missing or duplicated: ' + name)
        bindings = ('tools/portkey_mcp_tags.py', 'tools/generate_models.py', 'tools/portkey-mcp-tags.mjs',
                    'contracts/vendor/portkey-mcp-tags/8e5da0e/contract.json',
                    'crates/scm-sdk/src/airs/gateway/mcp_integrations.rs', 'crates/scm-sdk/src/airs/gateway/mcp_servers.rs',
                    wire, 'crates/scm-sdk/tests/portkey_mcp_tags_models.rs')
        return {
            'schemaVersion': 1, 'upstreamCommit': COMMIT, 'baselineCommit': '4a80dbdb3dc14365e1c25d838ac0bd139485d48c',
            'sourceBindings': {path: sha((ROOT / path).read_bytes()) for path in bindings},
            'counts': {'versionedRoots': len(CURRENT_ROOTS), 'clonedNodes': len(CLONES), 'addedModels': len(NEW_TYPES),
                       'requestProjections': len(REQUEST_PROJECTIONS), 'responseProjections': len(RESPONSE_PROJECTIONS)},
            'versionedRoots': {key: value for key, value in self.roots.items() if key.startswith(PREFIX)},
            'clonedNodes': self.mapping,
            'requestProjections': [{'baselineSchema': sid, 'currentSchema': self.mapping[sid], 'upstream': up, 'field': 'tags',
                                    'valueSchema': child} for sid, up, child in REQUEST_PROJECTIONS],
            'responseProjections': [{'baselineSchema': sid, 'upstream': up, 'field': 'tags', 'valueSchema': child}
                                    for sid, up, child in RESPONSE_PROJECTIONS],
            'schemas': self.supplemental_schemas(), 'modelExports': self.supplemental_models(),
            'wireTests': list(WIRE_TESTS),
            'compatibility': {
                'historicalRoots': 'Frozen request, query and response roots are byte-identical and keep 0.34.0 acceptance.',
                'currentRoots': 'Versioned roots add only tags: write keys match ^[a-zA-Z0-9_-]+$, values are strings, null is accepted; filters are strings that clients additionally require to be a JSON object of string values.',
                'responses': 'Historical tolerant response roots; conforming tags decode typed, nonconforming JSON stays in extensions.',
                'liveAcceptance': 'Not established by generated contracts or local fixtures.',
            },
        }


SUPPLEMENTAL = (
    # (public Rust item, kind, source, evidence substring, test "file::fn", delta items)
    ('scm_sdk::airs::gateway::AIGatewayMcpIntegrationsClient::list_with_options', 'method',
     'crates/scm-sdk/src/airs/gateway/mcp_integrations.rs', 'pub async fn list_with_options(',
     'crates/scm-sdk/tests/portkey_mcp_tags_contracts.rs::list_filters_send_exact_queries_and_frozen_list_is_unchanged',
     ('path:get:/mcp-integrations:changed:parameters',)),
    ('scm_sdk::airs::gateway::mcp_tags_filter', 'function',
     'crates/scm-sdk/src/airs/gateway/mcp_tags.rs', 'pub fn mcp_tags_filter<',
     'crates/scm-sdk/tests/portkey_mcp_tags_contracts.rs::list_filters_send_exact_queries_and_frozen_list_is_unchanged',
     ('path:get:/mcp-integrations:changed:parameters', 'path:get:/mcp-servers:changed:parameters')),
    ('scm_sdk::airs::gateway::models::GatewayMcpIntegrationsClientListOptions', 'model',
     'crates/scm-sdk/src/models/generated.rs', 'pub struct GatewayMcpIntegrationsClientListOptions {',
     'crates/scm-sdk/tests/portkey_mcp_tags_models.rs::list_option_roots_carry_the_pinned_query_contract',
     ('path:get:/mcp-integrations:changed:parameters',)),
    ('scm_sdk::data::TypedMap', 'type_alias', 'crates/scm-sdk/src/data.rs', 'pub use crate::models::{Extensions, TypedMap};',
     'crates/scm-sdk/tests/portkey_mcp_tags_models.rs::typed_model_validation_paths_accept_exactly_what_clients_accept', ()),
    ('scm_sdk::data::Extensions', 'type', 'crates/scm-sdk/src/data.rs', 'pub use crate::models::{Extensions, TypedMap};',
     'crates/scm-sdk/tests/portkey_mcp_tags_models.rs::typed_request_models_preserve_omission_null_and_value', ()),
)


def supplemental_capabilities(root=ROOT):
    """Portkey-delta public capabilities, recorded outside the frozen parity denominator.

    Every record is verified against its source and test; field-level `tags`
    projections are linked through the Portkey implementation-binding ledger.
    """
    operations = root / 'contracts/vendor/portkey-mcp-tags/8e5da0e/operations.json'
    ledger = json.loads(operations.read_text())
    receipt = json.loads((root / ledger['liveReceipt']).read_text())
    passed = {step['operation'] for step in receipt['report']['steps']
              if step['outcome'] == 'ok' and step.get('verified', True)}
    records = []
    for item, kind, source, evidence, test, deltas in SUPPLEMENTAL:
        test_file, test_name = test.split('::')
        if evidence not in (root / source).read_text():
            raise ValueError('Supplemental capability source drift: ' + item)
        if not re.search(rf'fn {test_name}\(', (root / test_file).read_text()):
            raise ValueError('Supplemental capability test missing: ' + item)
        steps = sorted({step for row in ledger['bindings'] if row['delta'] in deltas for step in row['liveSteps']})
        records.append({'item': item, 'kind': kind, 'source': source, 'test': test, 'deltaItems': list(deltas),
                        'liveAcceptance': 'verified' if steps and all(step in passed for step in steps) else 'not_applicable' if not deltas else 'unverified',
                        'liveSteps': steps})
    return {'upstreamCommit': COMMIT, 'implementationBindings': operations.relative_to(root).as_posix(),
            'implementationBindingsSha256': sha(operations.read_bytes()), 'liveReceipt': ledger['liveReceipt'],
            'frozenParityDenominatorUnchanged': True, 'fieldProjections': len(ledger['bindings']) - 2,
            'records': records}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    overlay = Overlay()
    content = json.dumps(overlay.ledger(), indent=2) + '\n'
    if args.check:
        if LEDGER.read_text() != content:
            raise ValueError('Portkey MCP tags overlay ledger is stale')
        graph = json.loads((ROOT / 'crates/scm-sdk/src/validation/schema_graph.json').read_text())
        historical = copy.deepcopy(graph)
        for sid in set(overlay.mapping.values()) | set(overlay.nodes):
            historical['nodes'].pop(sid, None)
        for name in overlay.roots:
            historical['roots'].pop(name, None)
        if overlay.extend_graph(historical) != graph:
            raise ValueError('Portkey MCP runtime graph differs')
        native = json.loads((ROOT / 'contracts/native-models.json').read_text())
        if (native.get('portkeyMcpTagsSchemas') != overlay.supplemental_schemas()
                or native.get('portkeyMcpTagsModelExports') != overlay.supplemental_models()):
            raise ValueError('Portkey MCP model ledger differs')
    else:
        LEDGER.write_text(content)
    print(json.dumps({'checked': args.check, **overlay.ledger()['counts']}))


if __name__ == '__main__':
    main()
