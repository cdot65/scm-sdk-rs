#!/usr/bin/env python3
"""Add the reviewed management 0.86.0 branches without replacing frozen contracts.

Runtime graph changes are limited to two vocabularies and two appended union
branches. Former extension fields become optional model-only typed projections.
"""
import copy
import hashlib
import json
from functools import lru_cache
from pathlib import Path

from red_team_dataplane import read_yaml

ROOT = Path(__file__).resolve().parents[1]
VENDOR = ROOT / 'contracts/vendor/red-team-management/0.86.0'
SOURCE_SHA256 = '5fbb533bd5f0156bf85c0a549b962aaf2f96cf09d0b6d9d82ac2d2afe87509a0'
EXTENSIONS = ROOT / 'contracts/red-team-management-union-extensions.json'
OVERLAY = ROOT / 'contracts/red-team-management-model-overlay.json'
ROOT_COMPONENTS = ('VertexAIConnectionParams', 'VertexAIConnectionRedactParams', 'MultiTurnClientSessionConfig')
PATTERNS = {'^[a-z0-9][a-z0-9-]{0,62}$', '^[a-z0-9-]+$', '^[0-9]+$'}
PROJECTIONS = {
    's010721': ('BasicAuthAuthConfig', ('basic_username', 'basic_password')),
    's010755': ('BasicAuthAuthConfig', ('basic_username', 'basic_password')),
    's010789': ('RestConnectionParamsBase', ('audio_request_json', 'audio_curl')),
    's010821': ('StreamingConnectionParamsBase', ('audio_request_json', 'audio_curl')),
    's010824': ('WebSocketConnectionParamsBase', ('audio_request_json', 'audio_curl')),
    's010829': ('WebSocketConnectionParamsBase', ('audio_request_json', 'audio_curl')),
}
# Every vendor declaration of a projected key must have an explicit owner,
# including masked counterparts, even though baseline types share their shape.
PROJECTION_COUNTERPARTS = {
    'BasicAuthAuthConfig': 'BasicAuthAuthConfigRedact',
    'RestConnectionParamsBase': 'RestConnectionParamsRedactBase',
    'StreamingConnectionParamsBase': 'StreamingConnectionParamsRedactBase',
    'WebSocketConnectionParamsBase': 'WebSocketConnectionParamsRedactBase',
}
PROJECTED_KEYS = {'basic_username', 'basic_password', 'audio_request_json', 'audio_curl'}
UNION_BRANCHES = {'s010462': ('VertexAIConnectionParams', 4), 's010504': ('MultiTurnClientSessionConfig', 2)}

BASELINE_NAMES = {
    's010721': 'BasicAuthAuthConfig', 's010755': 'AuthConfigRequestVariant3',
    's010789': 'RestConnectionParams', 's010821': 'StreamingConnectionParams',
    's010824': 'WebSocketConnectionParams', 's010829': 'ConnectionParamsRequestVariant2',
    's010462': 'StreamingConnectionParamsTargetConnectionConfig',
    's010504': 'StreamingConnectionParamsMultiTurnConfig',
}
ENUM_COMPONENTS = {'s012061': 'TargetConnectionType', 's012067': 'ResponseMode'}


@lru_cache(maxsize=1)
def baseline_nodes():
    inventory = json.loads((ROOT / 'contracts/typescript.json').read_text())
    if inventory['baseline']['commit'] != '4a80dbdb3dc14365e1c25d838ac0bd139485d48c':
        raise ValueError('Unexpected management overlay baseline')
    return inventory['schemas']['definitions']


def replace_once(source, old, new):
    if source.count(old) != 1:
        raise ValueError('Management generated declaration/header/import identity changed: ' + old)
    return source.replace(old, new)


def check_projection_completeness(nodes, projections):
    # Search the complete frozen graph, independently of the hand-reviewed IDs.
    candidates = {sid for sid, node in nodes.items()
        if 'basic_auth_location' in node.get('properties', {})
        or {'api_endpoint', 'request_json', 'multi_turn_config'} <= node.get('properties', {}).keys()}
    if candidates != set(projections):
        raise ValueError('Management projection baseline coverage is incomplete')


def check_vendor_projection_completeness(spec, projections):
    schemas = spec['components']['schemas']
    declared = {}
    for component, fields in projections.values():
        if component in declared and declared[component] != set(fields):
            raise ValueError('Unreviewed conflicting projection field ownership')
        declared[component] = set(fields)
    if set(declared) != set(PROJECTION_COUNTERPARTS):
        raise ValueError('Unreviewed vendor projection component coverage')
    candidates = {name: PROJECTED_KEYS & schema.get('properties', {}).keys()
        for name, schema in schemas.items()
        if PROJECTED_KEYS & schema.get('properties', {}).keys()}
    expected = {**declared, **{masked: declared[plain]
        for plain, masked in PROJECTION_COUNTERPARTS.items()}}
    if candidates != expected:
        raise ValueError('Unreviewed vendor projection component/field coverage')
    for plain, masked in PROJECTION_COUNTERPARTS.items():
        for field in declared[plain]:
            if (schemas[plain]['properties'][field] != schemas[masked]['properties'][field]
                    or (field in schemas[plain].get('required', []))
                    != (field in schemas[masked].get('required', []))):
                raise ValueError('Unreviewed divergent masked projection: ' + masked + '.' + field)


def sha(data):
    return hashlib.sha256(data).hexdigest()


def ident(pointer):
    return 'smp_' + sha(pointer.encode())[:24]


def load_spec():
    yaml = VENDOR / 'mp-openapi.yaml'
    if yaml.is_symlink() or sha(yaml.read_bytes()) != SOURCE_SHA256:
        raise ValueError('Management model source differs from reviewed pin')
    spec = json.loads((VENDOR / 'openapi.json').read_text())
    if spec != read_yaml(yaml):
        raise ValueError('Management normalized JSON differs from pinned YAML')
    return spec


class Overlay:
    def __init__(self, spec=None):
        self.spec = spec if spec is not None else load_spec()
        self.nodes, self.roots = {}, {}
        for name in ROOT_COMPONENTS:
            self.roots[name] = self.compile(self.spec['components']['schemas'][name], '#/components/schemas/' + name,
                require_literal=name == 'MultiTurnClientSessionConfig')
        check_projection_completeness(baseline_nodes(), PROJECTIONS)
        check_vendor_projection_completeness(self.spec, PROJECTIONS)
        self.projections = []
        for legacy, (component, fields) in PROJECTIONS.items():
            for field in fields:
                schema = self.spec['components']['schemas'][component]['properties'][field]
                pointer = '#/components/schemas/' + component + '/properties/' + field
                child = self.compile(schema, pointer)
                optional = self.node(pointer + '/projectionOptional', {'kind': 'ZodOptional', 'inner': {'$ref': child}})
                self.projections.append({'legacySchema': legacy, 'field': field, 'schema': optional})

    def node(self, pointer, value):
        sid = ident(pointer)
        if sid in self.nodes and self.nodes[sid] != value:
            raise ValueError('Management overlay node collision')
        self.nodes[sid] = value
        return sid

    def compile(self, schema, pointer, require_literal=False):
        annotations = {'title', 'description', 'examples', 'example', 'writeOnly'}
        if 'default' in schema:
            if (pointer != '#/components/schemas/MultiTurnClientSessionConfig/properties/type'
                    or schema.get('const') != 'client_session'
                    or schema['default'] != 'client_session'):
                raise ValueError('Unreviewed management default: ' + pointer)
            annotations.add('default')
        allowed = annotations | {'type', 'properties', 'required', 'additionalProperties', 'pattern', 'format', 'const', 'anyOf'}
        if set(schema) - allowed:
            raise ValueError('Unreviewed management fragment vocabulary: ' + pointer)
        if 'anyOf' in schema:
            branches = schema['anyOf']
            if len(branches) != 2 or branches[-1] != {'type': 'null'}:
                raise ValueError('Only reviewed nullable projections may use anyOf')
            inner = self.compile(branches[0], pointer + '/anyOf/0')
            return self.node(pointer, {'kind': 'ZodNullable', 'inner': {'$ref': inner}})
        if 'const' in schema:
            if schema['const'] != 'client_session' or schema.get('default') != 'client_session':
                raise ValueError('Unreviewed management discriminator')
            # The new branch is opt-in: never consume an absent tag or inject default.
            return self.node(pointer, {'kind': 'ZodLiteral', 'value': 'client_session'})
        kind = schema.get('type')
        if kind == 'string':
            if schema.get('format') not in {None, 'password'}:
                raise ValueError('Unreviewed management string format')
            checks = []
            if 'pattern' in schema:
                if schema['pattern'] not in PATTERNS:
                    raise ValueError('Unreviewed management request pattern')
                checks.append({'kind': 'regex', 'regex': {'$regexp': schema['pattern'], 'flags': ''}})
            return self.node(pointer, {'kind': 'ZodString', 'checks': checks, 'coerce': False})
        if kind == 'object':
            if 'properties' not in schema:
                if schema.get('additionalProperties') is not True:
                    raise ValueError('Unreviewed management record')
                key = self.node(pointer + '/key', {'kind': 'ZodString', 'checks': [], 'coerce': False})
                value = self.node(pointer + '/value', {'kind': 'ZodUnknown'})
                return self.node(pointer, {'kind': 'ZodRecord', 'key': {'$ref': key}, 'value': {'$ref': value}})
            if 'additionalProperties' in schema and schema['additionalProperties'] is not True:
                raise ValueError('Unreviewed management object additionalProperties policy')
            required = set(schema.get('required', []))
            if require_literal:
                required.add('type')
            fields = {}
            for name, value in schema['properties'].items():
                child = self.compile(value, pointer + '/properties/' + name)
                if name not in required:
                    child = self.node(pointer + '/optional/' + name, {'kind': 'ZodOptional', 'inner': {'$ref': child}})
                fields[name] = {'schema': {'$ref': child}}
            catchall = self.node(pointer + '/catchall', {'kind': 'ZodNever'})
            return self.node(pointer, {'kind': 'ZodObject', 'properties': fields, 'unknownFields': 'passthrough', 'catchall': {'$ref': catchall}})
        raise ValueError('Unimplemented management fragment: ' + pointer)

    def extend_nodes(self, original):
        baseline = baseline_nodes()
        for sid in set(PROJECTIONS) | set(UNION_BRANCHES) | set(ENUM_COMPONENTS):
            # Model-only dataplane projections may decorate unrelated objects; these
            # management nodes must still be byte-for-byte frozen before adoption.
            if original.get(sid) != baseline.get(sid):
                raise ValueError('Management baseline node identity changed: ' + sid)
        nodes = copy.deepcopy(original)
        for sid, node in self.nodes.items():
            if sid in nodes:
                raise ValueError('Management overlay must be applied exactly once')
            nodes[sid] = copy.deepcopy(node)
        for sid, component in ENUM_COMPONENTS.items():
            values = self.spec['components']['schemas'][component]['enum']
            nodes[sid]['values'] = list(dict.fromkeys(nodes[sid]['values'] + values))
            if 'enumObject' in nodes[sid]:
                nodes[sid]['enumObject'].update({value: value for value in values})
        for sid, (component, count) in UNION_BRANCHES.items():
            if len(nodes[sid]['variants']) != count:
                raise ValueError('Legacy management union prefix changed')
            nodes[sid]['variants'].append({'$ref': self.roots[component]})
        return nodes

    def extend_graph(self, graph):
        result = copy.deepcopy(graph)
        result['nodes'] = self.extend_nodes(result['nodes'])
        for name, sid in self.roots.items():
            root = name + 'Schema'
            if root in result['roots']:
                raise ValueError('Management root name collision')
            result['roots'][root] = sid
            result['roots']['vendor:red-team-mp:0.86.0:' + name] = sid
        return result

    def model_outputs(self, generator_class):
        inventory = {'schemas': {'definitions': self.nodes, 'roots': []}, 'exports': [
            {'name': name + 'Schema', 'kind': 'schema', 'schema': {'$ref': sid},
             'classification': {'namespace': 'airs::red_team'}} for name, sid in self.roots.items()]}
        gen = generator_class(inventory)
        source = gen.generate('red_team')[ROOT / 'crates/scm-sdk/src/models/generated.rs']
        source = replace_once(source, 'use crate::data::{Optional,Nullable,Patch,FiniteNumber,JsonValue,JsonObject};', 'use crate::data::JsonValue;')
        source = replace_once(source, '//! Generated native models. Regenerate with tools/generate_models.py.',
            '//! Management 0.86.0 additive models derived from the MIT pan.dev specification.\n//! Regenerate with tools/generate_models.py; see NOTICE.')
        declaration = '#[derive(Clone, PartialEq, Deserialize)]\n#[non_exhaustive]\npub struct VertexAIConnectionRedactParams {'
        source = replace_once(source, declaration,
            '/// Declaration-only typed view of the vendor masked shape. Target GET responses\n'
            '/// use the shared `VertexAIConnectionParams` union branch; response validation\n'
            '/// preserves masked strings and does not enforce request patterns.\n'
            + declaration)
        values = self.spec['components']['schemas']['ProfilingStatusFilter']['enum']
        if not values or len(values) != len(set(values)) or not all(isinstance(v, str) for v in values):
            raise ValueError('Invalid profiling filter vocabulary')
        source += '\n#[cfg(feature = "red-team")]\npub(crate) const TARGET_PROFILING_FILTER_VALUES: &[&str] = &[' + ','.join(json.dumps(v) for v in values) + '];\n'
        return source, gen

    def augment_models(self, gen, new_gen):
        gen.nodes = self.extend_nodes(gen.nodes)
        gen.external_types.update({sid: 'super::red_team_management::' + name for sid, name in new_gen.names.items()})
        for projection in self.projections:
            node = gen.nodes[projection['legacySchema']]
            field = projection['field']
            if field in node['properties']:
                raise ValueError('Management projection would overwrite a baseline field')
            node['properties'][field] = {'schema': {'$ref': projection['schema']}, 'vendorProjection': True}

    def extension_ledger(self):
        return {'schemaVersion': 1, 'sourceSha256': SOURCE_SHA256,
            'baselinePins': 'contracts/native-union-variants.json',
            'baselinePinsSha256': sha((ROOT / 'contracts/native-union-variants.json').read_bytes()),
            'extensions': [{'schema': sid, 'legacyBranches': count, 'appendedSchema': self.roots[component],
                'rustType': 'super::red_team_management::' + component, 'rustVariant': 'Variant' + str(count + 1)}
                for sid, (component, count) in UNION_BRANCHES.items()]}

    def verify_union_pins(self, actual, baseline, strict_verifier):
        expected = {row['schema']: row for row in self.extension_ledger()['extensions']}
        trimmed = copy.deepcopy(actual)
        seen = set()
        for row in trimmed['unions']:
            extension = expected.get(row.get('schema'))
            if extension:
                seen.add(row['schema'])
                count = extension['legacyBranches']
                tail = {'sourceIndex': count + 1, 'schema': extension['appendedSchema'],
                        'rustVariant': extension['rustVariant'], 'rustType': extension['rustType']}
                if len(row['branches']) != count + 1 or row['branches'][-1] != tail:
                    raise ValueError('Unreviewed management positional union extension')
                row['branches'].pop()
        if seen != set(expected):
            raise ValueError('Management union extension not emitted')
        strict_verifier(trimmed, baseline)

    def verify_model_names(self, gen):
        for sid, expected in BASELINE_NAMES.items():
            if gen.names.get(sid) != expected:
                raise ValueError('Management baseline model identity changed: ' + sid)

    def ledger(self, gen=None):
        return {'schemaVersion': 1, 'sourceSha256': SOURCE_SHA256,
            'generator': 'tools/red_team_management_models.py',
            'generatorSha256': sha(Path(__file__).read_bytes()),
            'runtimeEnumExtensions': {sid: [value for value in dict.fromkeys(self.spec['components']['schemas'][component]['enum'])
                if value not in baseline_nodes()[sid]['values']] for sid, component in ENUM_COMPONENTS.items()},
            'enumCompatibility': {name: {'frozenParityType': name, 'schemaBoundType': 'TargetCreateRequest' + ('ConnectionType' if name == 'TargetConnectionType' else name),
                'policy': 'Frozen TypeScript constants unchanged; schema-bound enums declare current values; response unknown strings preserved.'}
                for name in ('TargetConnectionType', 'ResponseMode')},
            'sharedRedactBranch': {'declarationOnlyModel': 'VertexAIConnectionRedactParams',
                'targetResponseBranch': 'VertexAIConnectionParams', 'reason': 'Frozen shared union retained; masked response strings bypass request patterns.'},
            'runtimeUnionExtensions': self.extension_ledger()['extensions'],
            'unionDecodeTransitions': [
                {'schema': sid, 'rustType': BASELINE_NAMES[sid],
                 'previousVariant': 'Unknown(JsonValue)',
                 'currentVariant': 'Variant' + str(count + 1) + '(' + component + ')',
                 'previouslyUnknownShape': ('An object with string project_id, location, engine_id and customer_sa_email, and no matching legacy provider shape.'
                    if component == 'VertexAIConnectionParams' else 'An object with explicit type: "client_session".'),
                 'precedence': 'Legacy branch order and precedence remain unchanged; payloads matching an earlier branch still select it.'}
                for sid, (component, count) in UNION_BRANCHES.items()],
            'modelOnlyProjections': [{**row, 'rustType': gen.names[row['legacySchema']] if gen else BASELINE_NAMES[row['legacySchema']]} for row in self.projections], 'publicModels': self.roots,
            'defaultPolicy': 'No added default nodes; frozen default bindings unchanged; client_session requires explicit tag.',
            'responsePolicy': 'Frozen roots retained; new patterns request-only; passthrough fields remain unvalidated optional typed projections.'}
