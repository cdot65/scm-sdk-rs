#!/usr/bin/env python3
"""Report Prisma AIRS upstream (pan.dev) drift against the pinned baseline; never adopt it.

Offline modes (no network):
  airs_drift.py --check                 verify the pinned baseline hash chain and that
                                        contracts/airs-drift.json equals regeneration
  airs_drift.py [--baseline-tree DIR]   regenerate contracts/airs-drift.json
  airs_drift.py compare --candidate DIR [--output PATH]
                                        classify a verified candidate tree by source;
                                        exit 0 unchanged/byte_only, 2 review_required,
                                        1 classification failure, tampering or path error

Online mode (the only networked path, explicitly invoked):
  airs_drift.py refresh --commit <40-hex> [--output-dir .local/airs-drift/<commit>]
                                        fetch the complete, non-truncated AIRS product
                                        trees at an exact pan.dev commit

Every change is classified from the parsed source, never from version strings. An
unknown change raises with its identity and digest and writes no receipt. Drift is
reported, never adopted: adoption requires a new reviewed pin through the
REVIEWED_DELTAS process in tools/red_team_management.py, and upstream removals never
authorize SDK removal.
"""
import argparse
import hashlib
import importlib.util
import json
import os
import re
import sys
from functools import lru_cache
from pathlib import Path

import red_team_management as rtm
from red_team_dataplane import read_yaml

CLASSIFIER_VERSION = 3
REPOSITORY = 'https://github.com/PaloAltoNetworks/pan.dev'
GITHUB_API = 'https://api.github.com/repos/PaloAltoNetworks/pan.dev'
RAW_BASE = 'https://raw.githubusercontent.com/PaloAltoNetworks/pan.dev'
LEDGER = Path('contracts/airs-drift.json')
ACTIVE_SOURCES = Path('contracts/airs-active-sources.json')
RETAINED = Path('contracts/airs-retained-capabilities.json')
DECISIONS = rtm.VENDOR / 'compatibility-decisions.json'
MANAGEMENT_UPSTREAM = 'openapi-specs/prisma-airs-redteam/management/mp-openapi.yaml'
PRODUCT_DIRECTORIES = ('openapi-specs/prisma-airs', 'openapi-specs/prisma-airs-redteam',
                       'openapi-specs/prisma-airs-model-security')
BINDINGS = (ACTIVE_SOURCES, DECISIONS, rtm.VENDOR / 'source.json', RETAINED, rtm.RETENTION_SEED)
OUTPUT_ROOTS = (Path('docs/checks/airs-drift'), Path('.local/airs-drift'))
SPEC_SUFFIXES = ('.yaml', '.yml', '.json')
MAX_API_BYTES = 16 * 1024 * 1024
MAX_BLOB_BYTES = 8 * 1024 * 1024
POLICY = ('Drift is reported, never adopted. Adoption requires a new reviewed pin via '
          'REVIEWED_DELTAS; upstream removals never authorize SDK removal.')

# Each class lists its permitted dispositions. "unknown" is never emitted: it fails
# classification, writes no receipt and leaves the SDK unchanged.
CHANGE_CLASSES = {
    'source_added': {'dispositions': ['additive'], 'meaning': 'New OpenAPI document under an AIRS product directory.'},
    'source_removed': {'dispositions': ['breaking'], 'meaning': 'Pinned document absent upstream; SDK capability is retained.'},
    'byte_only': {'dispositions': ['annotation'], 'meaning': 'Bytes differ but the semantic model is identical (formatting, key or set order).'},
    'version_label_changed': {'dispositions': ['annotation'], 'meaning': 'info.version differs; never used to classify content.'},
    'annotation_changed': {'dispositions': ['annotation'], 'meaning': 'description/title/example(s)/summary/tags documentation only.'},
    'license_changed': {'dispositions': ['breaking'], 'meaning': 'info.license differs; redistribution terms need review.'},
    'operation_added': {'dispositions': ['additive'], 'meaning': 'New METHOD path; no native binding exists yet.'},
    'operation_removed': {'dispositions': ['breaking'], 'meaning': 'METHOD path absent upstream; native binding and retained capability remain.'},
    'operation_id_changed': {'dispositions': ['breaking'], 'meaning': 'operationId differs for the same METHOD path.'},
    'parameter_added': {'dispositions': ['additive', 'breaking'], 'meaning': 'New parameter; breaking when required.'},
    'parameter_removed': {'dispositions': ['breaking'], 'meaning': 'Parameter (in, name) absent upstream.'},
    'request_body_added': {'dispositions': ['additive', 'breaking'], 'meaning': 'New request body; breaking when required.'},
    'request_body_removed': {'dispositions': ['breaking'], 'meaning': 'Request body absent upstream.'},
    'media_type_added': {'dispositions': ['additive'], 'meaning': 'New request or response media type.'},
    'media_type_removed': {'dispositions': ['breaking'], 'meaning': 'Request or response media type absent upstream.'},
    'response_status_added': {'dispositions': ['additive'], 'meaning': 'New response status.'},
    'response_status_removed': {'dispositions': ['breaking'], 'meaning': 'Response status absent upstream.'},
    'component_added': {'dispositions': ['additive'], 'meaning': 'New named component.'},
    'component_removed': {'dispositions': ['breaking'], 'meaning': 'Named component absent upstream; public names are retained.'},
    'component_renamed': {'dispositions': ['breaking'], 'meaning': 'Exactly one removed and one added component with equal bodies after reference substitution.'},
    'property_added': {'dispositions': ['additive'], 'meaning': 'New schema property.'},
    'property_removed': {'dispositions': ['breaking'], 'meaning': 'Schema property absent upstream.'},
    'requiredness_changed': {'dispositions': ['additive', 'breaking'], 'meaning': 'Becomes-required is breaking; becomes-optional is additive.'},
    'type_changed': {'dispositions': ['breaking'], 'meaning': 'type/format/nullable/$ref/content type or schema shape differs.'},
    'constraint_changed': {'dispositions': ['breaking'], 'meaning': 'Validation constraint, default or serialization differs.'},
    'enum_value_added': {'dispositions': ['additive'], 'meaning': 'New enum value.'},
    'enum_value_removed': {'dispositions': ['breaking'], 'meaning': 'Enum value absent upstream.'},
    'enum_order_changed': {'dispositions': ['annotation'], 'meaning': 'Same enum value set in a different order.'},
    'union_variant_appended': {'dispositions': ['additive'], 'meaning': 'Old branches, ignoring a trailing {type: null}, remain an exact prefix.'},
    'union_variant_removed': {'dispositions': ['breaking'], 'meaning': 'anyOf/oneOf branch absent upstream.'},
    'union_order_changed': {'dispositions': ['breaking'], 'meaning': 'Same branches in a different precedence order.'},
    'union_variant_inserted': {'dispositions': ['breaking'], 'meaning': 'New branch before an existing branch changes precedence.'},
    'security_changed': {'dispositions': ['breaking'], 'meaning': 'Effective operation security requirement differs.'},
    'security_scheme_changed': {'dispositions': ['breaking'], 'meaning': 'Security scheme definition differs.'},
    'server_changed': {'dispositions': ['breaking'], 'meaning': 'Effective operation server URL or variables differ.'},
    'unknown': {'dispositions': ['fail_closed'], 'meaning': 'Residual/extension/webhook/discriminator/not/external or dangling $ref/ambiguous rename: raises, no receipt.'},
}

ANNOTATIONS = rtm.ANNOTATIONS
SCHEMA_MAPS = {'properties', 'patternProperties', '$defs', 'definitions', 'dependentSchemas'}
SCHEMA_ONE = {'items', 'additionalProperties', 'not', 'if', 'then', 'else', 'contains', 'propertyNames',
              'unevaluatedProperties', 'unevaluatedItems', 'additionalItems'}
SCHEMA_LISTS = {'anyOf', 'oneOf', 'allOf', 'prefixItems'}
TYPE_KEYWORDS = {'type', 'format', 'nullable', '$ref', 'contentMediaType', 'contentEncoding'}
CONSTRAINT_KEYWORDS = {'default', 'const', 'minimum', 'maximum', 'exclusiveMinimum', 'exclusiveMaximum',
                       'multipleOf', 'minLength', 'maxLength', 'pattern', 'minItems', 'maxItems',
                       'uniqueItems', 'minProperties', 'maxProperties', 'readOnly', 'writeOnly'}
SERIALIZATION = {'style', 'explode', 'allowEmptyValue', 'allowReserved'}
OPERATION_ANNOTATIONS = {'summary', 'description', 'tags', 'externalDocs'}
OPERATION_KNOWN = OPERATION_ANNOTATIONS | {'operationId', 'parameters', 'requestBody', 'responses', 'security', 'servers'}
PATH_KNOWN = rtm.METHODS | {'parameters', 'servers', 'summary', 'description'}
INFO_ANNOTATIONS = {'title', 'summary', 'description', 'contact', 'termsOfService'}
DOCUMENT_ANNOTATIONS = {'tags', 'ExternalTags', 'externalDocs'}
DOCUMENT_KNOWN = {'openapi', 'info', 'paths', 'components', 'security', 'servers'}
# Keywords each compare method classifies; anything else fails closed. Added content is
# scanned against the same sets so new structures cannot bypass classification.
SCHEMA_KNOWN = (ANNOTATIONS | TYPE_KEYWORDS | CONSTRAINT_KEYWORDS
                | {'properties', 'required', 'enum', 'anyOf', 'oneOf', 'items', 'additionalProperties',
                   'propertyNames', 'allOf', 'prefixItems'})
PARAMETER_KNOWN = ANNOTATIONS | SERIALIZATION | {'in', 'name', 'required', 'schema', 'content'}
MEDIA_KNOWN = ANNOTATIONS | {'schema'}
SCHEMA_SUBTREES = {'anyOf', 'oneOf', 'allOf', 'prefixItems', 'items', 'additionalProperties', 'propertyNames',
                   'schema'}
SECURITY_SCHEME_KNOWN = {'type', 'description', 'name', 'in', 'scheme', 'bearerFormat', 'flows', 'openIdConnectUrl'}
COMPONENT_SECTIONS = {'schemas', 'parameters', 'requestBodies', 'responses', 'securitySchemes', 'examples'}
MISSING = object()
HEX40 = re.compile(r'[0-9a-f]{40}')


def sha(data):
    return rtm.sha(data)


def digest(value):
    return sha(rtm.encoded(value).encode())


def git_blob_sha1(data):
    return hashlib.sha1(b'blob ' + str(len(data)).encode() + b'\0' + data).hexdigest()


@lru_cache(maxsize=1)
def _contracts_verifier():
    path = rtm.ROOT / 'tools/verify-contracts.py'
    spec = importlib.util.spec_from_file_location('airs_drift_contract_verifier', path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def git_tree_hash(entries):
    return _contracts_verifier().git_tree_hash(entries)


def flat_tree_hash(entries):
    """Git tree object hash of one non-recursive GitHub tree listing."""
    raw = []
    for entry in entries:
        name, mode, kind = entry['path'], entry['mode'].lstrip('0'), entry['type']
        if '/' in name or not name or not HEX40.fullmatch(entry.get('sha', '')):
            raise ValueError('Malformed upstream tree listing entry')
        key = name.encode() + (b'/' if kind == 'tree' else b'')
        raw.append((key, mode.encode() + b' ' + name.encode() + b'\0' + bytes.fromhex(entry['sha'])))
    body = b''.join(value for _, value in sorted(raw))
    return hashlib.sha1(b'tree ' + str(len(body)).encode() + b'\0' + body).hexdigest()


_PARSED = {}


def parse(path, data):
    """Parse with the repository's pinned js-yaml; identical bytes parse identically, so cache by digest."""
    key = sha(data)
    if key not in _PARSED:
        spec = read_yaml(path)
        if sha(Path(path).read_bytes()) != key:
            raise ValueError(f'Source bytes changed while parsing: {path}')
        _PARSED[key] = json.dumps(spec)
    return json.loads(_PARSED[key])


def pointer(parts):
    return '/' + '/'.join(str(part).replace('~', '~0').replace('/', '~1') for part in parts)


def identity_of(source, parts, kind, old=MISSING, new=MISSING):
    identity = {'path': [source, *parts], 'kind': kind}
    if old is not MISSING:
        identity['old'] = old
    if new is not MISSING:
        identity['new'] = new
    return identity


def unknown(source, parts, kind, old=MISSING, new=MISSING, reason='Unclassified upstream change'):
    identity = identity_of(source, parts, kind, old, new)
    raise ValueError(f'{reason} (no receipt written): identity={rtm.encoded(identity).strip()} '
                     f'digest={digest(identity)}')


# ---------------------------------------------------------------------------
# Reference validation and semantic model


def resolve(spec, ref):
    target = spec
    for part in ref[2:].split('/'):
        part = part.replace('~1', '/').replace('~0', '~')
        if isinstance(target, dict) and part in target:
            target = target[part]
        elif isinstance(target, list) and part.isdigit() and int(part) < len(target):
            target = target[int(part)]
        else:
            return MISSING
    return target


def validate_references(source, spec):
    for ref in sorted(rtm.refs(spec), key=str):
        if not isinstance(ref, str) or not ref.startswith('#/'):
            unknown(source, ['$ref'], 'changed', new=ref, reason='Unreviewed external reference')
        if resolve(spec, ref) is MISSING:
            unknown(source, ['$ref'], 'changed', new=ref, reason='Dangling local reference')


def schema_model(schema):
    """Keywords kept (refs not inlined); required as a set; enum and unions ordered."""
    if not isinstance(schema, dict):
        return schema
    result = {}
    for key, value in schema.items():
        if key == 'required' and isinstance(value, list) and all(isinstance(x, str) for x in value):
            result[key] = sorted(set(value))
        elif key == 'type' and isinstance(value, list) and all(isinstance(x, str) for x in value):
            result[key] = sorted(set(value))
        elif key in SCHEMA_MAPS and isinstance(value, dict):
            result[key] = {name: schema_model(item) for name, item in value.items()}
        elif key in SCHEMA_ONE:
            result[key] = schema_model(value)
        elif key in SCHEMA_LISTS and isinstance(value, list):
            result[key] = [schema_model(item) for item in value]
        else:
            result[key] = value
    return result


def strip_annotations(schema):
    if not isinstance(schema, dict):
        return schema
    result = {}
    for key, value in schema.items():
        if key in ANNOTATIONS:
            continue
        if key in SCHEMA_MAPS and isinstance(value, dict):
            value = {name: strip_annotations(item) for name, item in value.items()}
        elif key in SCHEMA_ONE:
            value = strip_annotations(value)
        elif key in SCHEMA_LISTS and isinstance(value, list):
            value = [strip_annotations(item) for item in value]
        result[key] = value
    return result


def content_model(content):
    if not isinstance(content, dict):
        return content
    return {media: ({**value, 'schema': schema_model(value['schema'])} if isinstance(value, dict) and 'schema' in value else value)
            for media, value in content.items()}


def parameter_model(parameter):
    if not isinstance(parameter, dict) or '$ref' in parameter:
        return parameter
    result = dict(parameter)
    if 'schema' in result:
        result['schema'] = schema_model(result['schema'])
    if 'content' in result:
        result['content'] = content_model(result['content'])
    return result


def body_model(body):
    if not isinstance(body, dict) or '$ref' in body:
        return body
    result = dict(body)
    if 'content' in result:
        result['content'] = content_model(result['content'])
    return result


def parameter_key(source, spec, parameter):
    target = parameter
    if isinstance(parameter, dict) and '$ref' in parameter:
        target = resolve(spec, parameter['$ref'])
    if not isinstance(target, dict) or not isinstance(target.get('in'), str) or not isinstance(target.get('name'), str):
        unknown(source, ['parameters'], 'changed', new=parameter, reason='Unidentifiable parameter')
    return target['in'] + ':' + target['name']


def security_model(security):
    if not isinstance(security, list):
        return security
    rows = [{name: sorted(scopes) if isinstance(scopes, list) else scopes for name, scopes in row.items()}
            if isinstance(row, dict) else row for row in security]
    return sorted(rows, key=rtm.encoded)


def operation_models(source, spec):
    result = {}
    for route, item in spec.get('paths', {}).items():
        for method, operation in item.items():
            if method not in rtm.METHODS:
                continue
            parameters = {}
            for parameter in item.get('parameters', []) + operation.get('parameters', []):
                parameters[parameter_key(source, spec, parameter)] = parameter_model(parameter)
            result[method.upper() + ' ' + route] = {
                'operationId': operation.get('operationId'),
                'annotations': {k: operation[k] for k in OPERATION_ANNOTATIONS if k in operation},
                'parameters': parameters,
                'requestBody': body_model(operation['requestBody']) if 'requestBody' in operation else None,
                'responses': {str(status): body_model(response) for status, response in operation.get('responses', {}).items()},
                'security': security_model(operation['security'] if 'security' in operation else spec.get('security')),
                'servers': operation.get('servers', item.get('servers', spec.get('servers'))),
                'residual': {k: v for k, v in operation.items() if k not in OPERATION_KNOWN},
                'pathAnnotations': {k: item[k] for k in ('summary', 'description') if k in item},
                'pathResidual': {k: v for k, v in item.items() if k not in PATH_KNOWN},
            }
    return result


def document_model(source, spec):
    if not isinstance(spec, dict) or not isinstance(spec.get('paths'), dict) or 'openapi' not in spec:
        unknown(source, [], 'changed', reason='Not an OpenAPI document')
    validate_references(source, spec)
    normalizers = {'schemas': schema_model, 'parameters': parameter_model, 'requestBodies': body_model,
                   'responses': body_model}
    components = {}
    for section, entries in spec.get('components', {}).items():
        normalize = normalizers.get(section, lambda value: value)
        components[section] = ({name: normalize(value) for name, value in entries.items()}
                               if isinstance(entries, dict) else entries)
    return {'openapi': spec.get('openapi'), 'info': spec.get('info'),
            'operations': operation_models(source, spec), 'components': components,
            'rootSecurity': security_model(spec.get('security')), 'rootServers': spec.get('servers'),
            'document': {k: v for k, v in spec.items() if k not in DOCUMENT_KNOWN}}


def semantic_model_sha256(source, spec):
    return digest(document_model(source, spec))


def substitute(value, mapping):
    if isinstance(value, dict):
        return {k: (mapping.get(v, v) if k == '$ref' and isinstance(v, str) else substitute(v, mapping))
                for k, v in value.items()}
    if isinstance(value, list):
        return [substitute(item, mapping) for item in value]
    return value


# ---------------------------------------------------------------------------
# Retained capability and prior decision context


@lru_cache(maxsize=1)
def _context():
    operations = rtm.load('contracts/airs-operations.json')['operations']
    bindings = {(row['source'], row['method'], row['path']): row['nativeBindings'] for row in operations}
    retained = rtm.load(RETAINED)
    by_method = {}
    for section in ('methods', 'redTeamExtraMethods'):
        for row in retained[section]:
            for rust in row['rustMethods']:
                by_method.setdefault('::'.join(rust.split('::')[-2:]), set()).add(row['baselineId'])
    post_baseline = {(row['method'], row['path']) for row in retained['postBaselineRedTeamOperations']}
    decisions = [row for row in rtm.load(DECISIONS)['deltas']]
    return bindings, by_method, post_baseline, decisions


def native_bindings(source, operation_key):
    method, route = operation_key.split(' ', 1)
    return _context()[0].get((source, method, route), [])


def retained_capability(source, operation_key):
    _, by_method, post_baseline, _ = _context()
    method, route = operation_key.split(' ', 1)
    result = set()
    for binding in native_bindings(source, operation_key):
        result |= by_method.get(binding['class'] + '::' + binding['method'], set())
    if source == 'openapi-specs/prisma-airs-redteam/data-plane/dp-openapi.yaml' and (method, route) in post_baseline:
        result.add(f'postBaselineRedTeamOperations:{method} {route}')
    return result


def prior_decision(source, parts):
    if source != MANAGEMENT_UPSTREAM:
        return None
    best = None
    for row in _context()[3]:
        path = row['path']
        if list(parts[:len(path)]) == path and (best is None or len(path) > len(best['path'])):
            best = row
    return None if best is None else {'path': best['path'], 'category': best['category'], 'decision': best['decision']}


# ---------------------------------------------------------------------------
# Comparison


ADDED_SCANS = {'component_added': 'scan_component', 'property_added': 'scan_schema',
               'union_variant_appended': 'scan_schema', 'union_variant_inserted': 'scan_schema',
               'parameter_added': 'scan_parameter', 'request_body_added': 'scan_body',
               'response_status_added': 'scan_response', 'media_type_added': 'scan_media',
               'operation_added': 'scan_operation'}


class SourceComparison:
    def __init__(self, source, old_spec, new_spec):
        self.source = source
        self.old_spec, self.new_spec = old_spec, new_spec
        self.old = document_model(source, old_spec)
        self.new = document_model(source, new_spec)
        self.changes = []
        self.renames = {}
        self._closures = {}

    # -- emission -----------------------------------------------------------
    def emit(self, cls, kind, parts, old=MISSING, new=MISSING, disposition=None, **extra):
        allowed = CHANGE_CLASSES[cls]['dispositions']
        disposition = disposition or allowed[0]
        if disposition not in allowed or cls == 'unknown':
            raise ValueError(f'Invalid disposition for {cls}: {disposition}')
        if new is not MISSING and cls in ADDED_SCANS:
            getattr(self, ADDED_SCANS[cls])(new, parts)
        elif new is not MISSING and cls in ('type_changed', 'constraint_changed') and parts and (
                parts[-1] in SCHEMA_SUBTREES or isinstance(parts[-1], int)):
            for index, item in (enumerate(new) if isinstance(new, list) else [(None, new)]):
                self.scan_schema(item, parts if index is None else parts + [index])
        identity = identity_of(self.source, parts, kind, old, new)
        affected = sorted(self.affected(parts))
        retained = set()
        for key in affected:
            retained |= retained_capability(self.source, key)
        change = {'id': digest(identity), 'source': self.source, 'class': cls, 'disposition': disposition,
                  'pointer': pointer(parts), 'kind': kind, 'affectedOperations': affected,
                  'priorDecision': prior_decision(self.source, parts),
                  'retainedCapability': sorted(retained), **extra}
        if old is not MISSING:
            change['old'] = old
        if new is not MISSING:
            change['new'] = new
        self.changes.append(change)

    # -- added-content scans (fail closed on keywords no compare method classifies) --------
    def scan_schema(self, schema, parts):
        if not isinstance(schema, dict):
            return
        for key, value in schema.items():
            at = parts + [key]
            if key not in SCHEMA_KNOWN:
                self.fail(at, new=value, reason='Unclassified keyword in added content')
            if key == 'properties':
                if not isinstance(value, dict):
                    self.fail(at, new=value, reason='Unclassified keyword in added content')
                for name, item in value.items():
                    self.scan_schema(item, at + [name])
            elif key in ('items', 'additionalProperties', 'propertyNames'):
                self.scan_schema(value, at)
            elif key in ('anyOf', 'oneOf', 'allOf', 'prefixItems') and isinstance(value, list):
                for index, item in enumerate(value):
                    self.scan_schema(item, at + [index])

    def scan_media(self, media, parts):
        if not isinstance(media, dict):
            return
        for key, value in media.items():
            if key not in MEDIA_KNOWN:
                self.fail(parts + [key], new=value, reason='Unclassified keyword in added content')
        self.scan_schema(media.get('schema'), parts + ['schema'])

    def scan_content(self, content, parts):
        if content is None:
            return
        if not isinstance(content, dict):
            self.fail(parts, new=content, reason='Unclassified keyword in added content')
        for media, value in content.items():
            self.scan_media(value, parts + [media])

    def scan_parameter(self, parameter, parts):
        if not isinstance(parameter, dict) or '$ref' in parameter:
            return
        for key, value in parameter.items():
            if key not in PARAMETER_KNOWN:
                self.fail(parts + [key], new=value, reason='Unclassified keyword in added content')
        self.scan_schema(parameter.get('schema'), parts + ['schema'])
        self.scan_content(parameter.get('content'), parts + ['content'])

    def scan_body(self, body, parts, response=False):
        if not isinstance(body, dict) or '$ref' in body:
            return
        known = {'content', 'description'} | (set() if response else {'required'})
        for key, value in body.items():
            if key not in known:
                self.fail(parts + [key], new=value, reason='Unclassified keyword in added content')
        self.scan_content(body.get('content'), parts + ['content'])

    def scan_response(self, body, parts):
        self.scan_body(body, parts, response=True)

    def scan_operation(self, operation, parts):
        for field, at in (('residual', parts), ('pathResidual', parts[:2])):
            if operation[field]:
                self.fail(at, new=operation[field], reason='Unclassified keyword in added content')
        for key, parameter in operation['parameters'].items():
            location, name = key.split(':', 1)
            self.scan_parameter(parameter, parts + ['parameters', location, name])
        if operation['requestBody'] is not None:
            self.scan_body(operation['requestBody'], parts + ['requestBody'])
        for status, response in operation['responses'].items():
            self.scan_response(response, parts + ['responses', status])

    def scan_component(self, value, parts):
        section = parts[1]
        if section == 'schemas':
            self.scan_schema(value, parts)
        elif section == 'parameters':
            self.scan_parameter(value, parts)
        elif section == 'requestBodies':
            self.scan_body(value, parts)
        elif section == 'responses':
            self.scan_response(value, parts)
        elif section == 'securitySchemes' and isinstance(value, dict):
            for key, item in value.items():
                if key not in SECURITY_SCHEME_KNOWN:
                    self.fail(parts + [key], new=item, reason='Unclassified keyword in added content')

    def fail(self, parts, old=MISSING, new=MISSING, reason='Unclassified upstream change'):
        kind = 'added' if old is MISSING else 'removed' if new is MISSING else 'changed'
        unknown(self.source, parts, kind, old, new, reason)

    # -- affected operations -------------------------------------------------
    def closures(self, side):
        if side not in self._closures:
            spec = self.old_spec if side == 'old' else self.new_spec
            result = {}
            for route, item in spec.get('paths', {}).items():
                for method, operation in item.items():
                    if method in rtm.METHODS:
                        result[method.upper() + ' ' + route] = set(rtm.closure(
                            {'path': item.get('parameters', []), 'operation': operation}, spec))
            self._closures[side] = result
        return self._closures[side]

    def affected(self, parts):
        operations = set(self.old['operations']) | set(self.new['operations'])
        if not parts:
            return set()
        if parts[0] == 'paths' and len(parts) >= 2:
            if len(parts) >= 3 and parts[2] in rtm.METHODS:
                return {parts[2].upper() + ' ' + parts[1]}
            return {key for key in operations if key.split(' ', 1)[1] == parts[1]}
        if parts[0] == 'components' and len(parts) >= 3:
            section, name = parts[1], parts[2]
            if section == 'securitySchemes':
                result = set()
                for model in (self.old, self.new):
                    for key, operation in model['operations'].items():
                        if any(isinstance(row, dict) and name in row for row in operation['security'] or []):
                            result.add(key)
                return result
            names = {name, self.renames.get(section, {}).get(name, name)}
            names |= {old for old, new in self.renames.get(section, {}).items() if new == name}
            refs = {f'#/components/{section}/{n}' for n in names}
            return {key for side in ('old', 'new') for key, refs_ in self.closures(side).items() if refs & refs_}
        return set()

    # -- top level ----------------------------------------------------------
    def compare(self):
        old, new = self.old, self.new
        if old['openapi'] != new['openapi']:
            self.fail(['openapi'], old['openapi'], new['openapi'])
        self.compare_info(old['info'] or {}, new['info'] or {})
        for key in sorted(old['document'].keys() | new['document'].keys()):
            before, after = old['document'].get(key, MISSING), new['document'].get(key, MISSING)
            if before is not MISSING and after is not MISSING and rtm.encoded(before) == rtm.encoded(after):
                continue
            if key not in DOCUMENT_ANNOTATIONS:
                self.fail([key], before, after)
            self.emit('annotation_changed', self.kind(before, after), [key], before, after)
        mapping = self.detect_renames()
        new_components = substitute(new['components'], mapping)
        new_operations = substitute(new['operations'], mapping)
        self.compare_components(old['components'], new_components)
        self.compare_operations(old['operations'], new_operations)
        if not self.changes and rtm.encoded(old) != rtm.encoded(new):
            self.fail([], digest(old), digest(new), reason='Semantic model differs without a classified change')
        return self.changes

    @staticmethod
    def kind(before, after):
        return 'added' if before is MISSING else 'removed' if after is MISSING else 'changed'

    def compare_info(self, old, new):
        for key in sorted(old.keys() | new.keys()):
            before, after = old.get(key, MISSING), new.get(key, MISSING)
            if before is not MISSING and after is not MISSING and rtm.encoded(before) == rtm.encoded(after):
                continue
            parts, kind = ['info', key], self.kind(before, after)
            if key == 'version':
                self.emit('version_label_changed', kind, parts, before, after)
            elif key == 'license':
                self.emit('license_changed', kind, parts, before, after)
            elif key in INFO_ANNOTATIONS:
                self.emit('annotation_changed', kind, parts, before, after)
            else:
                self.fail(parts, before, after)

    # -- components ---------------------------------------------------------
    def detect_renames(self):
        mapping = {}
        for section in sorted(self.old['components'].keys() & self.new['components'].keys()):
            old, new = self.old['components'][section], self.new['components'][section]
            if section not in COMPONENT_SECTIONS - {'examples'} or not isinstance(old, dict) or not isinstance(new, dict):
                continue
            removed, added = sorted(old.keys() - new.keys()), sorted(new.keys() - old.keys())
            prefix = f'#/components/{section}/'
            candidates = {}
            for before in removed:
                for after in added:
                    body = substitute(new[after], {prefix + after: prefix + before})
                    if rtm.encoded(body) == rtm.encoded(old[before]):
                        candidates.setdefault(before, []).append(after)
            reverse = {}
            for before, names in candidates.items():
                for after in names:
                    reverse.setdefault(after, []).append(before)
            for before, names in sorted(candidates.items()):
                if len(names) != 1 or len(reverse[names[0]]) != 1:
                    self.fail(['components', section, before], old[before], sorted(names),
                              reason='Ambiguous component rename')
                self.renames.setdefault(section, {})[before] = names[0]
                mapping[prefix + names[0]] = prefix + before
        return mapping

    def compare_components(self, old, new):
        for section in sorted(old.keys() | new.keys()):
            before, after = old.get(section, {}), new.get(section, {})
            if section not in COMPONENT_SECTIONS or not isinstance(before, dict) or not isinstance(after, dict):
                if rtm.encoded(before) != rtm.encoded(after):
                    self.fail(['components', section], before, after)
                continue
            renamed = self.renames.get(section, {})
            renamed_to = set(renamed.values())
            for name in sorted(before.keys() | after.keys()):
                parts = ['components', section, name]
                if name in renamed:
                    self.emit('component_renamed', 'changed', parts, name, renamed[name], renamedFrom=name)
                    continue
                if name in renamed_to:
                    continue
                o, n = before.get(name, MISSING), after.get(name, MISSING)
                if section == 'examples':
                    if o is MISSING or n is MISSING or rtm.encoded(o) != rtm.encoded(n):
                        self.emit('annotation_changed', self.kind(o, n), parts, o, n)
                elif o is MISSING:
                    self.emit('component_added', 'added', parts, new=n)
                elif n is MISSING:
                    self.emit('component_removed', 'removed', parts, old=o)
                elif section == 'schemas':
                    self.compare_schema(o, n, parts)
                elif section == 'parameters':
                    self.compare_parameter(o, n, parts)
                elif section == 'requestBodies':
                    self.compare_body(o, n, parts)
                elif section == 'responses':
                    self.compare_response(o, n, parts)
                else:
                    self.compare_security_scheme(o, n, parts)

    def compare_security_scheme(self, old, new, parts):
        if not isinstance(old, dict) or not isinstance(new, dict):
            if old != new:
                self.emit('security_scheme_changed', 'changed', parts, old, new)
            return
        for key in sorted(old.keys() | new.keys()):
            before, after = old.get(key, MISSING), new.get(key, MISSING)
            if before is not MISSING and after is not MISSING and rtm.encoded(before) == rtm.encoded(after):
                continue
            self.emit('annotation_changed' if key == 'description' else 'security_scheme_changed',
                      self.kind(before, after), parts + [key], before, after)

    # -- schemas ------------------------------------------------------------
    def compare_schema(self, old, new, parts):
        if rtm.encoded(old) == rtm.encoded(new):
            return
        if not isinstance(old, dict) or not isinstance(new, dict):
            self.emit('type_changed', 'changed', parts, old, new)
            return
        for key in sorted(old.keys() | new.keys()):
            before, after = old.get(key, MISSING), new.get(key, MISSING)
            if before is not MISSING and after is not MISSING and rtm.encoded(before) == rtm.encoded(after):
                continue
            at, kind = parts + [key], self.kind(before, after)
            if key in ANNOTATIONS:
                self.emit('annotation_changed', kind, at, before, after)
            elif key == 'properties':
                self.compare_properties({} if before is MISSING else before, {} if after is MISSING else after, at)
            elif key == 'required':
                self.compare_required([] if before is MISSING else before, [] if after is MISSING else after, at)
            elif key == 'enum':
                self.compare_enum(before, after, at)
            elif key in ('anyOf', 'oneOf'):
                if before is MISSING or after is MISSING:
                    self.emit('type_changed', kind, at, before, after)
                else:
                    self.compare_union(before, after, at)
            elif key in ('items', 'additionalProperties', 'propertyNames'):
                if isinstance(before, dict) and isinstance(after, dict):
                    self.compare_schema(before, after, at)
                else:
                    self.emit('constraint_changed' if key == 'additionalProperties' else 'type_changed', kind, at, before, after)
            elif key in ('allOf', 'prefixItems'):
                if isinstance(before, list) and isinstance(after, list) and len(before) == len(after):
                    for index, (left, right) in enumerate(zip(before, after)):
                        self.compare_schema(left, right, at + [index])
                else:
                    self.emit('type_changed', kind, at, before, after)
            elif key in TYPE_KEYWORDS:
                self.emit('type_changed', kind, at, before, after)
            elif key in CONSTRAINT_KEYWORDS:
                self.emit('constraint_changed', kind, at, before, after)
            else:
                self.fail(at, before, after)

    def compare_properties(self, old, new, parts):
        if not isinstance(old, dict) or not isinstance(new, dict):
            self.fail(parts, old, new)
        for name in sorted(old.keys() | new.keys()):
            if name not in old:
                self.emit('property_added', 'added', parts + [name], new=new[name])
            elif name not in new:
                self.emit('property_removed', 'removed', parts + [name], old=old[name])
            else:
                self.compare_schema(old[name], new[name], parts + [name])

    def compare_required(self, old, new, parts):
        if not isinstance(old, list) or not isinstance(new, list):
            self.fail(parts, old, new)
        for name in sorted(set(new) - set(old)):
            self.emit('requiredness_changed', 'added', parts + [name], new=True, disposition='breaking')
        for name in sorted(set(old) - set(new)):
            self.emit('requiredness_changed', 'removed', parts + [name], old=True, disposition='additive')

    def compare_enum(self, old, new, parts):
        if not isinstance(old, list) or not isinstance(new, list):
            self.emit('constraint_changed', self.kind(old, new), parts, old, new)
            return
        old_keys, new_keys = [rtm.encoded(x) for x in old], [rtm.encoded(x) for x in new]
        added = [value for value, key in zip(new, new_keys) if key not in old_keys]
        removed = [value for value, key in zip(old, old_keys) if key not in new_keys]
        for value in added:
            self.emit('enum_value_added', 'added', parts, new=value)
        for value in removed:
            self.emit('enum_value_removed', 'removed', parts, old=value)
        if not added and not removed:
            self.emit('enum_order_changed', 'changed', parts, old, new)

    def compare_union(self, old, new, parts):
        if not isinstance(old, list) or not isinstance(new, list):
            self.fail(parts, old, new)
        old_keys = [rtm.encoded(strip_annotations(x)) for x in old]
        new_keys = [rtm.encoded(strip_annotations(x)) for x in new]
        if old_keys == new_keys:
            for index, (left, right) in enumerate(zip(old, new)):
                self.compare_schema(left, right, parts + [index])
            return
        if len(old_keys) == len(new_keys):
            differing = [index for index, (left, right) in enumerate(zip(old_keys, new_keys)) if left != right]
            if len(differing) == 1 and new_keys[differing[0]] not in old_keys and old_keys[differing[0]] not in new_keys:
                for index, (left, right) in enumerate(zip(old, new)):
                    self.compare_schema(left, right, parts + [index])
                return
        null = rtm.encoded({'type': 'null'})
        core = old_keys[:-1] if old_keys and old_keys[-1] == null else old_keys
        removed = [branch for branch, key in zip(old, old_keys) if key not in new_keys]
        extra = [branch for branch, key in zip(new, new_keys) if key not in old_keys]
        for branch in removed:
            self.emit('union_variant_removed', 'removed', parts, old=branch)
        if not removed and new_keys[:len(core)] == core and len(new_keys) > len(core):
            for index in range(len(core)):
                self.compare_schema(old[index], new[index], parts + [index])
            for branch in extra:
                self.emit('union_variant_appended', 'added', parts, new=branch)
        elif not removed and not extra:
            self.emit('union_order_changed', 'changed', parts, old, new)
        else:
            for branch in extra:
                self.emit('union_variant_inserted', 'added', parts, new=branch)

    # -- operations ---------------------------------------------------------
    def compare_operations(self, old, new):
        for key in sorted(old.keys() | new.keys()):
            method, route = key.split(' ', 1)
            parts = ['paths', route, method.lower()]
            if key not in old:
                self.emit('operation_added', 'added', parts, new=new[key], nativeBindings=[])
            elif key not in new:
                self.emit('operation_removed', 'removed', parts, old=old[key],
                          nativeBindings=native_bindings(self.source, key), sdkRetention='retained')
            else:
                self.compare_operation(old[key], new[key], parts)

    def compare_operation(self, old, new, parts):
        if rtm.encoded(old) == rtm.encoded(new):
            return
        if old['operationId'] != new['operationId']:
            self.emit('operation_id_changed', 'changed', parts + ['operationId'], old['operationId'], new['operationId'])
        for field, at in (('residual', parts), ('pathResidual', parts[:2])):
            if rtm.encoded(old[field]) != rtm.encoded(new[field]):
                self.fail(at, old[field], new[field])
        for field, at in (('annotations', parts), ('pathAnnotations', parts[:2])):
            for key in sorted(old[field].keys() | new[field].keys()):
                before, after = old[field].get(key, MISSING), new[field].get(key, MISSING)
                if before is MISSING or after is MISSING or rtm.encoded(before) != rtm.encoded(after):
                    self.emit('annotation_changed', self.kind(before, after), at + [key], before, after)
        for key in sorted(old['parameters'].keys() | new['parameters'].keys()):
            location, name = key.split(':', 1)
            at = parts + ['parameters', location, name]
            before, after = old['parameters'].get(key, MISSING), new['parameters'].get(key, MISSING)
            if before is MISSING:
                required = isinstance(after, dict) and after.get('required') is True
                self.emit('parameter_added', 'added', at, new=after, disposition='breaking' if required else 'additive')
            elif after is MISSING:
                self.emit('parameter_removed', 'removed', at, old=before, sdkRetention='retained')
            else:
                self.compare_parameter(before, after, at)
        before, after = old['requestBody'], new['requestBody']
        if before is None and after is not None:
            required = isinstance(after, dict) and after.get('required') is True
            self.emit('request_body_added', 'added', parts + ['requestBody'], new=after,
                      disposition='breaking' if required else 'additive')
        elif before is not None and after is None:
            self.emit('request_body_removed', 'removed', parts + ['requestBody'], old=before, sdkRetention='retained')
        elif before is not None:
            self.compare_body(before, after, parts + ['requestBody'])
        for status in sorted(old['responses'].keys() | new['responses'].keys()):
            at = parts + ['responses', status]
            if status not in old['responses']:
                self.emit('response_status_added', 'added', at, new=new['responses'][status])
            elif status not in new['responses']:
                self.emit('response_status_removed', 'removed', at, old=old['responses'][status])
            else:
                self.compare_response(old['responses'][status], new['responses'][status], at)
        if rtm.encoded(old['security']) != rtm.encoded(new['security']):
            self.emit('security_changed', 'changed', parts + ['security'], old['security'], new['security'])
        self.compare_servers(old['servers'], new['servers'], parts + ['servers'])

    def compare_servers(self, old, new, parts):
        if rtm.encoded(old) == rtm.encoded(new):
            return
        def bare(servers):
            if not isinstance(servers, list):
                return servers
            return [{k: v for k, v in s.items() if k != 'description'} if isinstance(s, dict) else s for s in servers]
        if rtm.encoded(bare(old)) != rtm.encoded(bare(new)):
            self.emit('server_changed', 'changed', parts, old, new)
        else:
            self.emit('annotation_changed', 'changed', parts, old, new)

    def compare_reference(self, old, new, parts):
        """True when either side is a $ref (kept, not inlined)."""
        if isinstance(old, dict) and '$ref' in old or isinstance(new, dict) and '$ref' in new:
            if rtm.encoded(old) != rtm.encoded(new):
                self.emit('type_changed', 'changed', parts, old, new)
            return True
        if not isinstance(old, dict) or not isinstance(new, dict):
            self.fail(parts, old, new)
        return False

    def compare_parameter(self, old, new, parts):
        if rtm.encoded(old) == rtm.encoded(new) or self.compare_reference(old, new, parts):
            return
        for key in sorted(old.keys() | new.keys()):
            before, after = old.get(key, MISSING), new.get(key, MISSING)
            if before is not MISSING and after is not MISSING and rtm.encoded(before) == rtm.encoded(after):
                continue
            at, kind = parts + [key], self.kind(before, after)
            if key == 'required':
                if (before is True) != (after is True):
                    self.emit('requiredness_changed', kind, at, before, after,
                              disposition='breaking' if after is True else 'additive')
            elif key == 'schema':
                if before is MISSING or after is MISSING:
                    self.emit('type_changed', kind, at, before, after)
                else:
                    self.compare_schema(before, after, at)
            elif key == 'content':
                self.compare_content({} if before is MISSING else before, {} if after is MISSING else after, at)
            elif key in ANNOTATIONS:
                self.emit('annotation_changed', kind, at, before, after)
            elif key in SERIALIZATION:
                self.emit('constraint_changed', kind, at, before, after)
            else:
                self.fail(at, before, after)

    def compare_body(self, old, new, parts, response=False):
        if rtm.encoded(old) == rtm.encoded(new) or self.compare_reference(old, new, parts):
            return
        for key in sorted(old.keys() | new.keys()):
            before, after = old.get(key, MISSING), new.get(key, MISSING)
            if before is not MISSING and after is not MISSING and rtm.encoded(before) == rtm.encoded(after):
                continue
            at, kind = parts + [key], self.kind(before, after)
            if key == 'required' and not response:
                if (before is True) != (after is True):
                    self.emit('requiredness_changed', kind, at, before, after,
                              disposition='breaking' if after is True else 'additive')
            elif key == 'content':
                self.compare_content({} if before is MISSING else before, {} if after is MISSING else after, at)
            elif key == 'description':
                self.emit('annotation_changed', kind, at, before, after)
            else:
                self.fail(at, before, after)

    def compare_response(self, old, new, parts):
        self.compare_body(old, new, parts, response=True)

    def compare_content(self, old, new, parts):
        if not isinstance(old, dict) or not isinstance(new, dict):
            self.fail(parts, old, new)
        for media in sorted(old.keys() | new.keys()):
            at = parts + [media]
            if media not in old:
                self.emit('media_type_added', 'added', at, new=new[media])
            elif media not in new:
                self.emit('media_type_removed', 'removed', at, old=old[media])
            else:
                self.compare_media(old[media], new[media], at)

    def compare_media(self, old, new, parts):
        if rtm.encoded(old) == rtm.encoded(new):
            return
        if not isinstance(old, dict) or not isinstance(new, dict):
            self.fail(parts, old, new)
        for key in sorted(old.keys() | new.keys()):
            before, after = old.get(key, MISSING), new.get(key, MISSING)
            if before is not MISSING and after is not MISSING and rtm.encoded(before) == rtm.encoded(after):
                continue
            at, kind = parts + [key], self.kind(before, after)
            if key == 'schema':
                if before is MISSING or after is MISSING:
                    self.emit('type_changed', kind, at, before, after)
                else:
                    self.compare_schema(before, after, at)
            elif key in ANNOTATIONS:
                self.emit('annotation_changed', kind, at, before, after)
            else:
                self.fail(at, before, after)


def source_summary(spec):
    components = spec.get('components', {})
    return {'version': spec.get('info', {}).get('version'), 'operationCount': len(rtm.operations(spec)),
            'componentCount': sum(len(v) for v in components.values() if isinstance(v, dict))}


def compare_sources(old_sources, new_sources):
    """Deterministic classification of two {upstreamPath: {bytes, spec}} sets."""
    sources, changes = [], []
    for upstream in sorted(old_sources.keys() | new_sources.keys()):
        old, new = old_sources.get(upstream), new_sources.get(upstream)
        if new is None:
            comparison = SourceComparison(upstream, old['spec'], old['spec'])
            retained, bindings = set(), {}
            for key in comparison.old['operations']:
                retained |= retained_capability(upstream, key)
                for binding in native_bindings(upstream, key):
                    bindings[rtm.encoded(binding)] = binding
            identity = identity_of(upstream, [], 'removed', old={'sha256': sha(old['bytes'])})
            changes.append({'id': digest(identity), 'source': upstream, 'class': 'source_removed',
                            'disposition': 'breaking', 'pointer': '', 'kind': 'removed',
                            'old': {'sha256': sha(old['bytes'])}, 'affectedOperations': sorted(comparison.old['operations']),
                            'priorDecision': None, 'retainedCapability': sorted(retained), 'sdkRetention': 'retained',
                            'nativeBindings': [bindings[key] for key in sorted(bindings)]})
            sources.append({'upstreamPath': upstream, 'status': 'removed', 'oldSha256': sha(old['bytes']),
                            'oldVersion': source_summary(old['spec'])['version']})
            continue
        if old is None:
            # Classify every structure of the new source against an empty document of the same
            # OpenAPI version, so document, info, component-section and addition rules all apply.
            empty = {'openapi': new['spec'].get('openapi') if isinstance(new['spec'], dict) else None,
                     'info': {}, 'paths': {}}
            SourceComparison(upstream, empty, new['spec']).compare()
            summary = source_summary(new['spec'])
            value = {'sha256': sha(new['bytes']), **summary}
            identity = identity_of(upstream, [], 'added', new=value)
            changes.append({'id': digest(identity), 'source': upstream, 'class': 'source_added',
                            'disposition': 'additive', 'pointer': '', 'kind': 'added', 'new': value,
                            'affectedOperations': [], 'priorDecision': None, 'retainedCapability': []})
            sources.append({'upstreamPath': upstream, 'status': 'added', 'newSha256': sha(new['bytes']),
                            'newVersion': summary['version']})
            continue
        old_version = source_summary(old['spec'])['version']
        new_version = source_summary(new['spec'])['version']
        row = {'upstreamPath': upstream, 'oldSha256': sha(old['bytes']), 'newSha256': sha(new['bytes']),
               'oldVersion': old_version, 'newVersion': new_version,
               'sameVersionContentChange': old_version == new_version and old['bytes'] != new['bytes']}
        if old['bytes'] == new['bytes']:
            row['status'] = 'unchanged'
            sources.append(row)
            continue
        comparison = SourceComparison(upstream, old['spec'], new['spec'])
        if rtm.encoded(comparison.old) == rtm.encoded(comparison.new):
            value = {'oldSha256': row['oldSha256'], 'newSha256': row['newSha256']}
            identity = identity_of(upstream, [], 'changed', old=row['oldSha256'], new=row['newSha256'])
            changes.append({'id': digest(identity), 'source': upstream, 'class': 'byte_only',
                            'disposition': 'annotation', 'pointer': '', 'kind': 'changed',
                            'old': value['oldSha256'], 'new': value['newSha256'],
                            'affectedOperations': [], 'priorDecision': None, 'retainedCapability': []})
            row['status'] = 'byte_only'
        else:
            changes.extend(comparison.compare())
            row['status'] = 'changed'
        sources.append(row)
    unique = {}
    for change in changes:
        unique.setdefault(change['id'], change)
    changes = sorted(unique.values(), key=lambda c: (c['source'], c['pointer'], c['class'], c['id']))
    if not changes:
        status = 'unchanged'
    elif all(change['class'] == 'byte_only' for change in changes):
        status = 'byte_only'
    else:
        status = 'review_required'
    summary = {'changes': len(changes), 'byClass': {}, 'byDisposition': {}}
    for change in changes:
        summary['byClass'][change['class']] = summary['byClass'].get(change['class'], 0) + 1
        summary['byDisposition'][change['disposition']] = summary['byDisposition'].get(change['disposition'], 0) + 1
    return {'status': status, 'sources': sources, 'summary': summary, 'changes': changes}


# ---------------------------------------------------------------------------
# Baseline, ledger and candidate verification


def baseline():
    """Verify the pinned seven-source chain and return {upstreamPath: {bytes, spec}}."""
    active = rtm.load(ACTIVE_SOURCES)
    if active.get('snapshotCommit') != rtm.COMMIT:
        raise ValueError('Active source snapshot commit differs from the reviewed pin')
    scope = rtm.load(rtm.AUDIT / 'inventory.json')['scope']['productDirectories']
    if list(scope) != list(PRODUCT_DIRECTORIES):
        raise ValueError('AIRS product directories differ from the pinned inventory scope')
    manifest_sources = {row['id']: row for row in rtm.load('contracts/manifest.json')['sources']}
    rows = active['sources']
    paths = [row['upstreamPath'] for row in rows]
    if len(paths) != len(set(paths)) or set(paths) != set(rtm.COUNTS):
        raise ValueError('Active source rows differ from the seven pinned AIRS sources')
    result = {}
    for row in rows:
        upstream = row['upstreamPath']
        if row.get('observedCommit') != rtm.COMMIT or not any(upstream.startswith(d + '/') for d in PRODUCT_DIRECTORIES):
            raise ValueError(f'Active source outside the pinned AIRS tree: {upstream}')
        data = rtm.checked_bytes(row['activeSpecificationPath'], row['activeSpecificationSha256'])
        rtm.resolve_active_identity(row, manifest_sources)
        spec = parse(rtm.safe_path(row['activeSpecificationPath']), data)
        canonical = sha(json.dumps(spec, sort_keys=True, separators=(',', ':'), ensure_ascii=False).encode())
        if canonical != row['canonicalSemanticJsonSha256']:
            raise ValueError(f'Pinned semantic JSON digest differs: {upstream}')
        if upstream == MANAGEMENT_UPSTREAM:
            rtm.verify_management_metadata(rtm.load(rtm.VENDOR / 'source.json'), manifest_sources[rtm.SOURCE_ID], spec)
        if len(rtm.operations(spec)) != rtm.COUNTS[upstream] or row['operationCount'] != rtm.COUNTS[upstream]:
            raise ValueError(f'Pinned operation count differs: {upstream}')
        result[upstream] = {'bytes': data, 'spec': spec, 'row': row}
    return result


def safe_relative(value):
    if (not isinstance(value, str) or not value or value.startswith('/') or '\\' in value
            or any(part in ('', '.', '..') for part in value.split('/'))):
        raise ValueError(f'Unsafe upstream path: {value!r}')
    return value


def content_identity(tree):
    trees = [{'path': t['path'], 'treeSha': t['treeSha'], 'truncated': t['truncated'],
              'entries': sorted(t['entries'], key=lambda e: e['path'])}
             for t in sorted(tree['productTrees'], key=lambda t: t['path'])]
    return {'schemaVersion': tree['schemaVersion'], 'repository': tree['repository'], 'commit': tree['commit'],
            'commitTreeSha': tree['commitTreeSha'], 'productTrees': trees}


def verify_tree_document(tree):
    """Structural, hash-chain and digest checks of tree.json (no file bytes)."""
    if not isinstance(tree, dict) or set(tree) - {'observation'} != {
            'schemaVersion', 'repository', 'commit', 'commitTreeSha', 'productTrees', 'contentSha256'}:
        raise ValueError('Candidate tree.json has unexpected or missing fields')
    if tree['schemaVersion'] != 1:
        raise ValueError('Unsupported candidate tree schema version')
    if tree['repository'] != REPOSITORY:
        raise ValueError('Foreign candidate repository rejected: ' + str(tree['repository']))
    for key in ('commit', 'commitTreeSha'):
        if not isinstance(tree[key], str) or not HEX40.fullmatch(tree[key]):
            raise ValueError(f'Candidate {key} must be 40 lowercase hex characters')
    trees = tree['productTrees']
    if not isinstance(trees, list) or sorted(t.get('path') for t in trees if isinstance(t, dict)) != sorted(PRODUCT_DIRECTORIES) \
            or len(trees) != len(PRODUCT_DIRECTORIES):
        raise ValueError('Candidate must contain exactly the three AIRS product trees')
    files = {}
    for product in trees:
        if set(product) != {'path', 'treeSha', 'truncated', 'entries'} or product['truncated'] is not False:
            raise ValueError('Candidate product tree is truncated or malformed: ' + str(product.get('path')))
        if not isinstance(product['treeSha'], str) or not HEX40.fullmatch(product['treeSha']):
            raise ValueError('Candidate product tree hash is malformed')
        for entry in product['entries']:
            if not isinstance(entry, dict) or set(entry) != {'path', 'mode', 'gitBlobSha1', 'sha256', 'bytes'}:
                raise ValueError('Candidate tree entry is malformed')
            if entry['mode'] != '100644':
                raise ValueError('Unsupported upstream file mode (symlink, executable or submodule): ' + str(entry['path']))
            upstream = product['path'] + '/' + safe_relative(entry['path'])
            if upstream in files:
                raise ValueError('Duplicate candidate tree path: ' + upstream)
            if (not isinstance(entry['sha256'], str) or not re.fullmatch(r'[0-9a-f]{64}', entry['sha256'])
                    or type(entry['bytes']) is not int or entry['bytes'] < 0):
                raise ValueError('Candidate tree entry digest is malformed: ' + upstream)
            files[upstream] = entry
        if git_tree_hash(product['entries']) != product['treeSha']:
            raise ValueError('Candidate product tree hash does not match its entries: ' + product['path'])
    if tree['contentSha256'] != digest(content_identity(tree)):
        raise ValueError('Candidate contentSha256 does not match its tree')
    return files


def verify_candidate(directory, base=None, *, parse_sources=True):
    """Return verified ({upstreamPath: {bytes, spec}}, identity, tree) for a candidate directory.

    parse_sources=False checks integrity only (used by refresh before classification)."""
    directory = Path(directory)
    if directory.is_symlink() or not directory.is_dir():
        raise ValueError('Candidate must be a real directory')
    top = sorted(path.name for path in directory.iterdir())
    if top != ['files', 'tree.json']:
        raise ValueError('Candidate directory must contain exactly tree.json and files/')
    tree_path, files_root = directory / 'tree.json', directory / 'files'
    if tree_path.is_symlink() or not tree_path.is_file() or files_root.is_symlink() or not files_root.is_dir():
        raise ValueError('Candidate tree.json and files/ must be regular, non-symlink entries')
    tree = json.loads(tree_path.read_bytes())
    expected = verify_tree_document(tree)
    present = set()
    for current, directories, names in os.walk(files_root, followlinks=False):
        for name in directories + names:
            path = Path(current) / name
            if path.is_symlink():
                raise ValueError('Symlink in candidate files rejected: ' + path.relative_to(files_root).as_posix())
        for name in names:
            path = Path(current) / name
            if not path.is_file():
                raise ValueError('Non-regular candidate file rejected')
            present.add(path.relative_to(files_root).as_posix())
    if present != set(expected):
        extra, missing = sorted(present - set(expected)), sorted(set(expected) - present)
        raise ValueError(f'Candidate file set differs from its tree: extra={extra} missing={missing}')
    contents = {}
    for upstream, entry in sorted(expected.items()):
        data = (files_root / upstream).read_bytes()
        if len(data) != entry['bytes'] or sha(data) != entry['sha256'] or git_blob_sha1(data) != entry['gitBlobSha1']:
            raise ValueError('Candidate file bytes do not match the tree: ' + upstream)
        contents[upstream] = data
    if tree['commit'] == rtm.COMMIT:
        base = baseline() if base is None else base
        if set(contents) != set(base) or any(contents[k] != base[k]['bytes'] for k in base):
            raise ValueError('Source tampering: candidate claims the pinned commit with differing bytes')
    result = {}
    for upstream, data in contents.items():
        if not parse_sources:
            break
        if not upstream.endswith(SPEC_SUFFIXES):
            unknown(upstream, [], 'added', new={'sha256': sha(data)}, reason='Unclassified non-specification upstream file')
        spec = parse(files_root / upstream, data)
        if not isinstance(spec, dict) or 'openapi' not in spec or not isinstance(spec.get('paths'), dict):
            unknown(upstream, [], 'added', new={'sha256': sha(data)}, reason='Unclassified non-OpenAPI upstream file')
        result[upstream] = {'bytes': data, 'spec': spec}
    identity = {'repository': tree['repository'], 'commit': tree['commit'],
                'commitTreeSha': tree['commitTreeSha'], 'contentSha256': tree['contentSha256']}
    return result, identity, tree


def verify_baseline_tree(tree, base):
    files = verify_tree_document(tree)
    if tree['commit'] != rtm.COMMIT:
        raise ValueError('Ledger baselineTree must describe the pinned commit')
    if set(files) != set(base):
        raise ValueError('Ledger baselineTree file set differs from the pinned sources')
    for upstream, entry in files.items():
        data = base[upstream]['bytes']
        if entry['gitBlobSha1'] != git_blob_sha1(data) or entry['sha256'] != sha(data) or entry['bytes'] != len(data):
            raise ValueError('Ledger baselineTree blob differs from the pinned source: ' + upstream)


def current_ledger_text():
    path = rtm.safe_path(LEDGER)
    return path.read_text() if path.is_file() else None


def ledger(baseline_tree=None):
    base = baseline()
    if baseline_tree is None:
        text = current_ledger_text()
        if text is not None:
            baseline_tree = json.loads(text).get('baselineTree')
    rows = []
    for upstream, item in sorted(base.items()):
        summary = source_summary(item['spec'])
        rows.append({'upstreamPath': upstream,
                     'activeSpecificationPath': item['row']['activeSpecificationPath'],
                     'activeSpecificationSha256': sha(item['bytes']),
                     'gitBlobSha1': git_blob_sha1(item['bytes']),
                     'version': summary['version'], 'operationCount': summary['operationCount'],
                     'componentCount': summary['componentCount'],
                     'semanticModelSha256': semantic_model_sha256(upstream, item['spec'])})
    document = {
        'schemaVersion': 1, 'classifierVersion': CLASSIFIER_VERSION, 'repository': REPOSITORY,
        'snapshotCommit': rtm.COMMIT, 'productDirectories': list(PRODUCT_DIRECTORIES),
        'bindings': [{'path': str(path), 'sha256': sha(rtm.safe_path(path).read_bytes())} for path in BINDINGS],
        'sources': rows, 'changeClasses': CHANGE_CLASSES, 'reviewedCandidateChanges': [], 'policy': POLICY,
        'evidenceLimit': 'Offline comparison of committed pins and verified candidate trees; refresh is explicit and '
                         'never runs in --check; reported drift is not adopted, live or runtime evidence.',
    }
    if baseline_tree is not None:
        verify_baseline_tree(baseline_tree, base)
        document['baselineTree'] = baseline_tree
    return document


def verify_ledger_bindings(document):
    for row in document.get('bindings', []):
        rtm.checked_bytes(row['path'], row['sha256'])
    if [row['path'] for row in document.get('bindings', [])] != [str(p) for p in BINDINGS]:
        raise ValueError('Drift ledger bindings differ from the reviewed binding set')


def check():
    text = current_ledger_text()
    if text is None:
        raise ValueError(f'Missing drift ledger: {LEDGER}')
    current = json.loads(text)
    verify_ledger_bindings(current)
    expected = rtm.encoded(ledger(current.get('baselineTree')))
    if text != expected:
        raise ValueError(f'Stale or tampered drift ledger: {LEDGER}')
    return current


def verify(root=None):
    """Phase 25 gate: ledger equals regeneration and the baseline self-compares unchanged."""
    if root is not None and Path(root).resolve() != rtm.ROOT.resolve():
        raise ValueError('Drift verification must run against the repository root')
    check()
    base = baseline()
    report = compare_sources(base, base)
    if report['status'] != 'unchanged' or report['changes']:
        raise ValueError('Pinned baseline does not self-compare as unchanged')
    return report


def compare_candidate(directory):
    document = check()
    base = baseline()
    for row in document['sources']:
        if semantic_model_sha256(row['upstreamPath'], base[row['upstreamPath']]['spec']) != row['semanticModelSha256']:
            raise ValueError('Baseline semantic model differs from the drift ledger')
    candidate, identity, _ = verify_candidate(directory, base)
    report = compare_sources(base, candidate)
    return {'schemaVersion': 1, 'classifierVersion': CLASSIFIER_VERSION, 'policy': POLICY,
            'adoption': 'not_adopted',
            'baseline': {'repository': REPOSITORY, 'commit': rtm.COMMIT,
                         'ledgerSha256': sha(rtm.encoded(document).encode())},
            'candidate': identity, **report}


# ---------------------------------------------------------------------------
# Output guard and receipts


def guarded_output(path):
    """Resolve a write target strictly beneath docs/checks/airs-drift/ or .local/airs-drift/."""
    path = Path(path)
    if path.is_absolute():
        try:
            path = Path(os.path.abspath(path)).relative_to(rtm.ROOT)
        except ValueError:
            raise ValueError(f'Refusing write outside drift output roots: {path}') from None
    if '..' in path.parts or not any(path.parts[:len(r.parts)] == r.parts and len(path.parts) > len(r.parts)
                                     for r in OUTPUT_ROOTS):
        raise ValueError(f'Refusing write outside drift output roots: {path}')
    current = rtm.ROOT
    for part in path.parts:
        current = current / part
        if current.is_symlink():
            raise ValueError(f'Refusing write through a symlink: {path}')
    target = rtm.ROOT / path
    if not target.resolve(strict=False).is_relative_to(rtm.ROOT.resolve()):
        raise ValueError(f'Refusing write outside drift output roots: {path}')
    return target


def write_new(target, data):
    """Create a new file (mode 'x'); never overwrite evidence."""
    target.parent.mkdir(parents=True, exist_ok=True)
    with open(target, 'xb') as handle:
        handle.write(data.encode('utf-8') if isinstance(data, str) else data)


# ---------------------------------------------------------------------------
# Online refresh (explicit; the only networked path)


def urllib_fetcher():
    import urllib.error
    import urllib.request

    class NoRedirect(urllib.request.HTTPRedirectHandler):
        def redirect_request(self, *args, **kwargs):
            return None

    opener = urllib.request.build_opener(NoRedirect)

    def fetch(url, headers, limit):
        if not url.startswith(('https://api.github.com/', 'https://raw.githubusercontent.com/')):
            raise ValueError('Refusing unexpected upstream URL')
        request = urllib.request.Request(url, headers=headers)
        try:
            response = opener.open(request, timeout=60)
        except urllib.error.HTTPError as error:
            return error.code, url, b''
        with response:
            return response.status, response.geturl(), response.read(limit + 1)
    return fetch


def refresh(commit, output_dir=None, *, fetch=None, token=None, clock=None):
    """Fetch the complete AIRS product trees at an exact commit into a new directory."""
    if not isinstance(commit, str) or not HEX40.fullmatch(commit):
        raise ValueError('Refresh requires a full 40-character lowercase hex commit')
    target = guarded_output(output_dir if output_dir is not None else OUTPUT_ROOTS[1] / commit)
    if target.exists() or target.is_symlink():
        raise ValueError(f'Refusing to overwrite existing refresh output: {target}')
    fetch = fetch or urllib_fetcher()

    def get(url, limit, api):
        headers = {'Accept': 'application/vnd.github+json' if api else 'application/octet-stream',
                   'User-Agent': 'scm-sdk-rs-airs-drift'}
        if api and token:
            headers['Authorization'] = 'Bearer ' + token
        status, final_url, body = fetch(url, headers, limit)
        if status != 200 or final_url != url:
            raise ValueError(f'Refusing redirected or failed upstream response ({status}): {url}')
        if len(body) > limit:
            raise ValueError(f'Upstream response exceeds the size limit: {url}')
        return body

    def get_json(url):
        return json.loads(get(url, MAX_API_BYTES, True))

    def subtree(listing, name):
        if listing.get('truncated') is not False:
            raise ValueError('Upstream tree listing is truncated')
        if flat_tree_hash(listing['tree']) != listing['sha']:
            raise ValueError('Upstream tree listing does not hash to its object id')
        matches = [e for e in listing['tree'] if e['path'] == name and e['type'] == 'tree']
        if len(matches) != 1:
            raise ValueError(f'Upstream directory missing: {name}')
        return matches[0]['sha']

    commit_object = get_json(f'{GITHUB_API}/git/commits/{commit}')
    if commit_object.get('sha') != commit:
        raise ValueError('Upstream commit identity differs from the requested commit')
    root_sha = commit_object['tree']['sha']
    root = get_json(f'{GITHUB_API}/git/trees/{root_sha}')
    if root.get('sha') != root_sha:
        raise ValueError('Upstream root tree identity differs')
    specs_sha = subtree(root, 'openapi-specs')
    specs = get_json(f'{GITHUB_API}/git/trees/{specs_sha}')
    if specs.get('sha') != specs_sha:
        raise ValueError('Upstream openapi-specs tree identity differs')
    products, blobs = [], {}
    for product in PRODUCT_DIRECTORIES:
        product_sha = subtree(specs, product.split('/', 1)[1])
        listing = get_json(f'{GITHUB_API}/git/trees/{product_sha}?recursive=1')
        if listing.get('truncated') is not False or listing.get('sha') != product_sha:
            raise ValueError(f'Upstream product tree is truncated or differs: {product}')
        entries = []
        for entry in listing['tree']:
            if entry['type'] == 'tree':
                continue
            if entry['type'] != 'blob' or entry['mode'] != '100644':
                raise ValueError(f'Unsupported upstream entry (symlink, executable or submodule): {entry["path"]}')
            relative = safe_relative(entry['path'])
            data = get(f'{RAW_BASE}/{commit}/{product}/{relative}', MAX_BLOB_BYTES, False)
            if git_blob_sha1(data) != entry['sha'] or ('size' in entry and entry['size'] != len(data)):
                raise ValueError(f'Upstream blob hash mismatch: {product}/{relative}')
            entries.append({'path': relative, 'mode': entry['mode'], 'gitBlobSha1': entry['sha'],
                            'sha256': sha(data), 'bytes': len(data)})
            blobs[product + '/' + relative] = data
        entries.sort(key=lambda e: e['path'])
        if git_tree_hash(entries) != product_sha:
            raise ValueError(f'Upstream product listing is incomplete: {product}')
        products.append({'path': product, 'treeSha': product_sha, 'truncated': False, 'entries': entries})
    tree = {'schemaVersion': 1, 'repository': REPOSITORY, 'commit': commit, 'commitTreeSha': root_sha,
            'productTrees': products}
    tree['contentSha256'] = digest(content_identity(tree))
    tree['observation'] = {'method': 'GitHub REST git commit/trees (non-truncated; root, openapi-specs and '
                                     'product tree hashes recomputed) and raw blobs at the exact commit; '
                                     'redirects refused; authentication token never recorded.',
                           'apiBase': GITHUB_API, 'rawBase': RAW_BASE}
    if clock is not None:
        tree['observation']['observedAt'] = clock()
    write_new(target / 'tree.json', rtm.encoded(tree))
    for upstream, data in sorted(blobs.items()):
        write_new(target / 'files' / upstream, data)
    verify_candidate(target, parse_sources=False)
    return target


# ---------------------------------------------------------------------------
# CLI


def main(argv=None):
    argv = list(sys.argv[1:] if argv is None else argv)
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    if argv and argv[0] in ('compare', 'refresh'):
        sub = parser.add_subparsers(dest='command', required=True)
        compare = sub.add_parser('compare', help='Offline classification of a verified candidate tree')
        compare.add_argument('--candidate', required=True)
        compare.add_argument('--output', help='Receipt path under docs/checks/airs-drift/ or .local/airs-drift/')
        online = sub.add_parser('refresh', help='Explicit online inventory refresh at an exact commit')
        online.add_argument('--commit', required=True)
        online.add_argument('--output-dir')
    else:
        parser.add_argument('--check', action='store_true')
        parser.add_argument('--baseline-tree', help='Embed a verified refresh directory for the pinned commit')
    args = parser.parse_args(argv)
    try:
        command = getattr(args, 'command', None)
        if command == 'compare':
            if args.output is not None:
                target = guarded_output(args.output)
                if target.exists() or target.is_symlink():
                    raise ValueError(f'Refusing to overwrite existing receipt: {target}')
            receipt = compare_candidate(args.candidate)
            text = rtm.encoded(receipt)
            if args.output is not None:
                write_new(target, text)
            else:
                sys.stdout.write(text)
            print(f"AIRS drift: {receipt['status']}; {receipt['summary']['changes']} classified changes; not adopted",
                  file=sys.stderr)
            return {'unchanged': 0, 'byte_only': 0}.get(receipt['status'], 2)
        if command == 'refresh':
            target = refresh(args.commit, args.output_dir, token=os.environ.get('GITHUB_TOKEN') or None)
            print(f'AIRS drift refresh written: {target}')
            return 0
        if args.check:
            if args.baseline_tree:
                raise ValueError('--baseline-tree regenerates; it cannot be combined with --check')
            verify(rtm.ROOT)
            print('AIRS drift: pinned baseline verified; ledger current; baseline self-compare unchanged')
            return 0
        tree = None
        if args.baseline_tree:
            _, identity, tree = verify_candidate(args.baseline_tree)
            if identity['commit'] != rtm.COMMIT:
                raise ValueError('--baseline-tree must be a refresh of the pinned commit')
            tree = {k: v for k, v in tree.items() if k != 'observation'}
        document = ledger(tree)
        rtm.safe_path(LEDGER).write_text(rtm.encoded(document))
        print(f'AIRS drift ledger regenerated: {LEDGER}')
        return 0
    except (ValueError, KeyError, TypeError, OSError, json.JSONDecodeError) as error:
        print(str(error), file=sys.stderr)
        return 1
    except Exception as error:  # parser failures from the pinned js-yaml subprocess
        if type(error).__name__ == 'CalledProcessError':
            print('Upstream document failed to parse with the pinned YAML parser', file=sys.stderr)
            return 1
        raise


if __name__ == '__main__':
    sys.exit(main())
