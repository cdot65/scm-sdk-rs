#!/usr/bin/env python3
"""Derive independent management compatibility specimens from pinned vendor schemas.

This is an offline compatibility corpus, not evidence of service acceptance.
Union acceptance expectations are reviewed authored values derived from vendor
schemas, not outputs from an independent validator oracle.
It deliberately does not import the management model compiler or its ledgers.
"""
import argparse
import copy
import hashlib
import json
from pathlib import Path

from red_team_dataplane import read_yaml

ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / 'contracts/native-fixtures/red-team-management.json'
VENDOR = ROOT / 'contracts/vendor/red-team-management/0.86.0'
PIN = '5fbb533bd5f0156bf85c0a549b962aaf2f96cf09d0b6d9d82ac2d2afe87509a0'
# Independently reviewed mapping: six frozen objects, twelve formerly raw keys.
MODELS = [
    ('s010721', 'BasicAuthAuthConfig', 'BasicAuthAuthConfig', {'basic_auth_location': 'HEADER'}),
    ('s010755', 'AuthConfigRequestVariant3', 'BasicAuthAuthConfig', {}),
    ('s010789', 'RestConnectionParams', 'RestConnectionParamsBase', {}),
    ('s010821', 'StreamingConnectionParams', 'StreamingConnectionParamsBase', {}),
    ('s010824', 'WebSocketConnectionParams', 'WebSocketConnectionParamsBase', {'ws_response_timeout': 110}),
    ('s010829', 'ConnectionParamsRequestVariant2', 'WebSocketConnectionParamsBase', {}),
]


def digest(data):
    return hashlib.sha256(data).hexdigest()


def expected():
    yaml = VENDOR / 'mp-openapi.yaml'
    if digest(yaml.read_bytes()) != PIN:
        raise ValueError('Management fixture source pin changed')
    spec = json.loads((VENDOR / 'openapi.json').read_text())
    if spec != read_yaml(yaml):
        raise ValueError('Management fixture normalized source differs')
    definitions = json.loads((ROOT / 'contracts/typescript.json').read_text())['schemas']['definitions']
    schemas = spec['components']['schemas']
    occurrences, cases = [], []
    values = [('omitted', None), ('null', None), ('string', 'synthetic-private-fixture'),
              ('number', 17), ('boolean', True), ('array', ['synthetic-private-fixture']),
              ('object', {'future': 'synthetic-private-fixture'})]
    for sid, model, component, defaults in MODELS:
        fields = ('basic_username', 'basic_password') if component == 'BasicAuthAuthConfig' else ('audio_request_json', 'audio_curl')
        base = {'response_stop_key': 'done', 'response_stop_value': 'yes'} if model == 'StreamingConnectionParams' else {}
        for field in fields:
            fragment = schemas[component]['properties'][field]
            inner = fragment.get('anyOf', [fragment])[0]
            wire_type = inner['type']
            if wire_type not in ('string', 'object') or field in definitions[sid]['properties']:
                raise ValueError('Projection shape or frozen passthrough identity changed')
            occurrences.append({'schema': sid, 'rustType': model, 'field': field,
                'vendorPointer': f'#/components/schemas/{component}/properties/{field}', 'vendorSchema': fragment})
            for label, value in values:
                document = copy.deepcopy(base)
                if label != 'omitted':
                    document[field] = value
                ordinary = {**defaults, **document}
                typed = 'unset' if label == 'omitted' else 'null' if label == 'null' else 'value' if label == wire_type else 'unset'
                cases.append({'schema': sid, 'rustType': model, 'field': field, 'case': label,
                    'input': document, 'expected': ordinary, 'withoutDefaults': document, 'typedPresence': typed})
    # Vendor schema drives required fields and pattern probes, with an independent
    # explicit witness for each reviewed format. Expected values never call SDK parsing.
    vertex = {'project_id': 'example-project', 'location': 'us-central1', 'engine_id': '12345', 'customer_sa_email': 'service@example.invalid'}
    union_cases = [{'schema': 's010462', 'variant': 5, 'case': 'valid', 'input': vertex, 'requestAccepted': True, 'responseAccepted': True}]
    for field in schemas['VertexAIConnectionParams']['required']:
        missing = dict(vertex)
        missing.pop(field)
        union_cases.append({'schema': 's010462', 'variant': 5, 'case': 'missing-' + field, 'input': missing, 'requestAccepted': False, 'responseAccepted': False})
        wrong = {**vertex, field: 17}
        union_cases.append({'schema': 's010462', 'variant': 5, 'case': 'wrong-type-' + field, 'input': wrong, 'requestAccepted': False, 'responseAccepted': False})
    for field, fragment in schemas['VertexAIConnectionParams']['properties'].items():
        if 'pattern' in fragment:
            union_cases.append({'schema': 's010462', 'variant': 5, 'case': 'loose-format-' + field,
                'input': {**vertex, field: 'UPPER / opaque'}, 'requestAccepted': False, 'responseAccepted': True})
    union_cases += [
        {'schema': 's010504', 'variant': 3, 'case': 'explicit-client-session', 'input': {'type': schemas['MultiTurnClientSessionConfig']['properties']['type']['const'], 'future': None}, 'requestAccepted': True, 'responseAccepted': True},
        {'schema': 's010504', 'variant': 2, 'case': 'missing-tag-stateless', 'input': {}, 'requestAccepted': True, 'responseAccepted': True},
        {'schema': 's010504', 'variant': 1, 'case': 'missing-tag-stateful', 'input': {'request_id_field':'previous','response_id_field':'next'}, 'requestAccepted': True, 'responseAccepted': True},
    ]
    enums = []
    for sid, component in [('s012061', 'TargetConnectionType'), ('s012067', 'ResponseMode')]:
        for value in schemas[component]['enum']:
            if value not in definitions[sid]['values']:
                enums.append({'schema': sid, 'value': value})
    if len(occurrences) != 12 or len(enums) != 3:
        raise ValueError('Management fixture coverage changed and requires review')
    sources = ['tools/management_fixtures.py', 'tools/test_management_fixtures.py',
        'crates/scm-sdk/tests/red_team_management_fixtures.rs', 'contracts/typescript.json',
        'contracts/vendor/red-team-management/0.86.0/mp-openapi.yaml',
        'contracts/vendor/red-team-management/0.86.0/openapi.json']
    return {'schemaVersion': 1, 'evidenceBoundary': 'Offline vendor-derived shape and independent preservation witnesses only; no service acceptance or live provider execution is established.',
        'expectationsOrigin': 'Union acceptance expectations are reviewed, vendor-schema-derived authored values, not independent validator-oracle outputs. Raw preservation and default expectations are explicit witnesses independent of SDK parsing.',
        'sourceBindings': [{'path': path, 'sha256': digest((ROOT / path).read_bytes())} for path in sources],
        'projectionOccurrences': occurrences, 'projectionCases': cases,
        'unionTails': [{'schema': 's010462', 'variant': 5, 'vendorComponent': 'VertexAIConnectionParams'},
                       {'schema': 's010504', 'variant': 3, 'vendorComponent': 'MultiTurnClientSessionConfig'}],
        'unionCases': union_cases, 'enumAdditions': enums}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    rendered = json.dumps(expected(), indent=2, ensure_ascii=False) + '\n'
    if args.check:
        if not OUTPUT.is_file() or OUTPUT.read_text() != rendered:
            raise SystemExit('Management fixtures stale; run python3 tools/management_fixtures.py')
    else:
        OUTPUT.write_text(rendered)
    print(json.dumps({'checked': args.check, 'projectionOccurrences': 12, 'models': 6, 'unionTails': 2, 'enumAdditions': 3}))


if __name__ == '__main__':
    main()
