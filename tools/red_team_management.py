#!/usr/bin/env python3
"""Reproduce the AIRS source/operation inventory and management adoption decisions.

This is static contract evidence, not a validator generator or live qualifier.
Only committed pins and the reviewed, committed native-binding audit are inputs.

To review any new delta, including a whole added or removed subtree, form identity from its path/kind/old/new keys
(omit absent keys), then SHA256(encoded(identity).encode('utf-8')). encoded is
sorted-key, indent-2, ensure_ascii=False JSON with a terminal newline. Review the
old/new meaning before registering its exact path and digest; never auto-approve.
The rejection diagnostic prints the identity and candidate digest for review.
"""
import argparse
import hashlib
import json
import re
from pathlib import Path
from functools import lru_cache

from red_team_dataplane import read_yaml

ROOT = Path(__file__).resolve().parents[1]
AUDIT = Path('docs/research/2026-10-19-prisma-airs-openapi')
VENDOR = Path('contracts/vendor/red-team-management/0.86.0')
COMMIT = '582e30543bf2adfd8b0ee457170d22b5c380dfbe'
SOURCE_ID = 'prisma-airs-redteam/management/mp-openapi.yaml@0.86.0+582e30543bf2'
METHODS = {'get', 'post', 'put', 'patch', 'delete', 'head', 'options', 'trace'}
ANNOTATIONS = {'description', 'title', 'example', 'examples'}
COUNTS = {'openapi-specs/prisma-airs-model-security/dataplane/data-plane.yml': 18, 'openapi-specs/prisma-airs-model-security/management/mgmt-plane.yml': 23, 'openapi-specs/prisma-airs-redteam/data-plane/dp-openapi.yaml': 47, 'openapi-specs/prisma-airs-redteam/management/mp-openapi.yaml': 42, 'openapi-specs/prisma-airs-redteam/network-broker/AIRS-Red-Teaming-Network-Broker.yaml': 5, 'openapi-specs/prisma-airs/management/mgmt-service_latest.yaml': 21, 'openapi-specs/prisma-airs/scan/scan-service_latest.yaml': 4}
RETENTION_SEED = Path('contracts/compatibility/airs-0.2.2-retention.json')
RETENTION_SHA256 = '25a772562bc033381c21099850b46bea3392d8cfdd8454f21ef7e977523a6e86'
RETENTION_COMMIT = '0a213fe79c7f0c5758f05f53d04a5c577a4f3f4e'
RETENTION_LEDGER_PATHS = ['contracts/native-exports.json', 'contracts/native-operations.json', 'contracts/response-defaults.json', 'contracts/vendor/portkey-access/a27feb9/operations.json', 'contracts/vendor/red-team-data-plane/0.93.0/model-additions.json', 'contracts/vendor/red-team-data-plane/0.93.0/operations.json']
AUDIT_PINS = {'inventory.json': '9dd21a2413574296373ee691a1d6ede19d077cebc97f581fc8c4331e511ff5ab', 'red-team-operations.json': 'b006c7db6a89866b5be205c69c85f3db1a8102c49eaab8c883f186946b364e72', 'model-security-operations.json': '5bd5529b8b4288b1b3fe2824e4f951be4ef19e83b06f061422874ad5f4d86d02', 'runtime-operation-mapping.json': '5d0f878c9e4d5b136d335579bbb9b2f44594cbd0d3ea19a3cbff2595772397a0'}
REVIEWED_DELTAS = {
    ('components', 'schemas', 'AudioFormat'): '84b4abe3604a45998b1e2aca2efd693fc78b3353c4824b3e7d35af9e09227d92',
    ('components', 'schemas', 'BasicAuthAuthConfig', 'description'): '99729dd1ce5e11babcff6657e84d38ea4bc716d9b4a8344baa9abc2b669f029a',
    ('components', 'schemas', 'BasicAuthAuthConfig', 'properties', 'basic_auth_header', 'description'): '58927e07c3f73280c069c9477ab4c689c05be5511abfacc13db9735ec18ab9a3',
    ('components', 'schemas', 'BasicAuthAuthConfig', 'properties', 'basic_password'): 'be4fa75f2efed20fe9301db9200adaec6aae568ee28dc18652a06f5ef0e8c234',
    ('components', 'schemas', 'BasicAuthAuthConfig', 'properties', 'basic_username'): '7add05f4db0f0443c9966cedea79bb72ad2f61ed9230ed1d919ba9cc4417f67d',
    ('components', 'schemas', 'BasicAuthAuthConfigRedact', 'properties', 'basic_auth_header', 'description'): '95e540783cce7c4ab8d69a5dc9842da2e2471b3b9541eebbb213536113361de7',
    ('components', 'schemas', 'BasicAuthAuthConfigRedact', 'properties', 'basic_password'): '85a5e8b8ffe5ee6813481b642d1a3059b9569f939b2fdf3d651118f1fc408be7',
    ('components', 'schemas', 'BasicAuthAuthConfigRedact', 'properties', 'basic_username'): '6921bf57c16821ac609ebf4799467ee9557755c9408489761aaf115d433b1b01',
    ('components', 'schemas', 'Body_upload_prompts_csv_v1_custom_attack_upload_custom_prompts_csv_post', 'properties', 'file', 'contentMediaType'): 'f09a84719c05286c27b08c963d477a7590cdabec96a75862bcbde9949bee5dee',
    ('components', 'schemas', 'Body_upload_prompts_csv_v1_custom_attack_upload_custom_prompts_csv_post', 'properties', 'file', 'format'): 'b4c46ede3a6b7a61c73190bb6c1d8de0d0410208a25f31a8eb06bf6a718ca851',
    ('components', 'schemas', 'LanguageDropdownOptionSchema'): '00c96fcfa763f9a7d3f859f20efb1179d76cad89d372c8542b203efceee07997',
    ('components', 'schemas', 'MultiTurnClientSessionConfig'): '1f86cf5e2ce16855e20e3b1eeebb286da147c7d7c27fca66bda3807623f39b37',
    ('components', 'schemas', 'NativeConnectionParamsBase'): '011b044e86f28f6b22d01b9bca0e0de996426634acc7fdb6fefcb13d6f05cc6b',
    ('components', 'schemas', 'NativeConnectionParamsBase-Input'): 'ef5aa6470f935f32e7b352e16f6f49064987c4c4d6ffcacff87db46c328475f6',
    ('components', 'schemas', 'NativeConnectionParamsBase-Output'): '166e85280bf9ec11742ffc5d55a473bc39d95260ba0fde1a1b98265ec3b4447a',
    ('components', 'schemas', 'NativeConnectionParamsRedactBase', 'properties', 'multi_turn_config', 'anyOf'): '215adfd83ae7eb4930e9d9a185ea77c9ec2966a5696550baeaf90f24e97b11f0',
    ('components', 'schemas', 'NativeConnectionParamsRedactBase', 'properties', 'target_connection_config', 'anyOf'): 'fe82a0f595ed898817e897ebec4350016891965dafe3e7de618b05cfe7ce519c',
    ('components', 'schemas', 'ProfilingStatus', 'enum'): 'edaaba8e904d5b76db3ff05354a0c9c328d2fc14ba7033dae3eb93418528bff8',
    ('components', 'schemas', 'ProfilingStatusFilter'): '1e87467a8571dfad320728078da8b7d42b01e3716b70294480419092c053226d',
    ('components', 'schemas', 'ResponseMode', 'enum'): '45ab1f969b5ca6464e23c76014f81fff58e15b867eaec6ff95e92ab5fb6c3285',
    ('components', 'schemas', 'RestConnectionParamsBase'): '5bebd082b19fffbf4bf68accb8f5d2103e8a9dc82ebafed3208e97cf4dac0f41',
    ('components', 'schemas', 'RestConnectionParamsBase-Input'): 'c1a5c5e3c206e9af004305222b7865843db9c5188f3a6658a836406f7455366d',
    ('components', 'schemas', 'RestConnectionParamsBase-Output'): '675c84f6a69a942d72c35fceee6bd82b5f52926a06e5279581cc5a1adcac7eec',
    ('components', 'schemas', 'RestConnectionParamsRedactBase', 'properties', 'audio_curl'): '4029e0b80617f35a0767207ecf3eeda9ac41e205afc4beb5f52314621c65d073',
    ('components', 'schemas', 'RestConnectionParamsRedactBase', 'properties', 'audio_request_json'): 'eec054582ac8d6eecfba0ec016b43e251f140eb505b801043286e14de7a55da7',
    ('components', 'schemas', 'RestConnectionParamsRedactBase', 'properties', 'multi_turn_config', 'anyOf'): 'bb799f0d1c46894a51268d70225fdca9c0b2e39d2732f58b7fa77691ded386d2',
    ('components', 'schemas', 'RestConnectionParamsRedactBase', 'properties', 'target_connection_config', 'anyOf'): '6fe67b88fd25a94c89b1edc44b62489d02ae999ca60fe986e846bfeb6c7fd24f',
    ('components', 'schemas', 'StreamingConnectionParamsBase'): 'cfb28d2d8e02e09878df5f7003f0a5329bc4bd700cf461c1763382ec0848c002',
    ('components', 'schemas', 'StreamingConnectionParamsBase-Input'): '9ea76f510521d83fa9d6a4bcd315396a4b56a12e2a0cc7992accd47ef4d16532',
    ('components', 'schemas', 'StreamingConnectionParamsBase-Output'): '4d0b7b3c3c46bef32c2d8ef3e86646154c68d32c4d368c5b1548eaeb2301a450',
    ('components', 'schemas', 'StreamingConnectionParamsRedactBase', 'properties', 'audio_curl'): 'd71cabdb18392283b7276cae99cfa8e8c9751bbf044a30e65df5fac3a71442d6',
    ('components', 'schemas', 'StreamingConnectionParamsRedactBase', 'properties', 'audio_request_json'): 'bfbbe2670348b07cd7ddb06d027fbf58d345600c07cbcdc340c0bfe9c45b8739',
    ('components', 'schemas', 'StreamingConnectionParamsRedactBase', 'properties', 'multi_turn_config', 'anyOf'): '6d595b7f21f4d0653ebcb73d77fcb1ac0089ab86afb03e1d67415e1e2a858f7a',
    ('components', 'schemas', 'StreamingConnectionParamsRedactBase', 'properties', 'target_connection_config', 'anyOf'): 'fb9d3cd297597dfa79774b310026e5ae3af837028af926c34d56635efcb43311',
    ('components', 'schemas', 'TargetConnectionType', 'enum'): '352500624e4390d56da10dfa3dea6b2be20ded01e2a38d0a6a5321b9f645c85b',
    ('components', 'schemas', 'TargetCreateRequestSchema', 'properties', 'connection_params', 'anyOf'): '73aa1b37dd470c45edcca1671bf3ea53e14a849755dce318ed02aa72e431f901',
    ('components', 'schemas', 'TargetListItemSchema', 'properties', 'profiling_status', 'description'): '374b8b6db8627938c2b9bbd63a3ade9c9e2a2512a5cee83ae80a8de14d740e89',
    ('components', 'schemas', 'TargetMetadata', 'description'): '4d487ffbcde0b9af74399fa81af7411906c255bcb445f9e25a6ccd6ee15950c0',
    ('components', 'schemas', 'TargetMetadata', 'properties', 'audio_format'): '89cac27dbebb6d9e002faad9ff6e3daef698ffc3f38a40f56ca3903e1758936c',
    ('components', 'schemas', 'TargetMetadata', 'properties', 'audio_multi_turn_probe_error'): '25abd64813224eec31def52d46f48810c0bdff611ffdbf88e94b47a2b1e0e85a',
    ('components', 'schemas', 'TargetMetadata', 'properties', 'audio_multi_turn_probe_passed'): 'acdf0b34cdc48c3030cb7528da6b41c093caea62c9285a04f490a33aff103cb5',
    ('components', 'schemas', 'TargetMetadata', 'properties', 'audio_probe_error'): '60f0ae05a90cd7a5e6dadea57b90d10199556343e74f81e153e2d02901bb8f68',
    ('components', 'schemas', 'TargetMetadata', 'properties', 'audio_probe_passed'): '5bb3f26a802c5a6862d74e6594c0e6632d1e97384f5dca5bf20131b8ea6c9709',
    ('components', 'schemas', 'TargetMetadata', 'properties', 'supports_audio'): 'e651d7316eca4b8512cff78b32e8f1010677e91b4764428b9d9bb5723b7924e6',
    ('components', 'schemas', 'TargetMetadata', 'properties', 'supports_text'): '6cc766eafe354db13d730074c767c67411b550867e3843c1033b7ac1c5580aaf',
    ('components', 'schemas', 'TargetProbeRequest', 'properties', 'connection_params', 'anyOf'): 'b71940b3f32aa862431195d429a9e8656b89882b0482fa056b0499ed5a2543a8',
    ('components', 'schemas', 'TargetRedactSchema', 'properties', 'profiling_status', 'description'): '94a1963091746aeb8d3615cebabbea493dac63f6a497b44cdd9dba13bc805486',
    ('components', 'schemas', 'TargetResponseSchema', 'properties', 'profiling_status', 'description'): '09e5c8ce9d5affeafed0f246bf450a3a7ce5defce84f3767f060bcdace27db2b',
    ('components', 'schemas', 'TargetSchema', 'description'): '1a54e6dfcb7ccd13942daa2de49e8a4d252213a94cfeebb71ba1d92f25d85e0d',
    ('components', 'schemas', 'TargetSchema', 'properties', 'connection_params', 'anyOf'): '50c7a41d7fb1024f7bae87504193e73816a77797775e66137e05fb6610c6c610',
    ('components', 'schemas', 'TargetSchema', 'properties', 'profiling_status', 'description'): 'b6d814f6988df3a46a47661e42da3851b7462ed1de04eb4e87cb2c4a6ec9d0b0',
    ('components', 'schemas', 'TargetStatusFilter', 'description'): '11cb608dbb2543bc2e38c5daf39b67824f118b83128424ef1c52d964b37f7b68',
    ('components', 'schemas', 'TargetStatusFilter', 'enum'): 'f93f0f7b080256c8a7efefcf02f90fc06d5abf98401c9ac8edaad92c6d9cc39a',
    ('components', 'schemas', 'TargetUpdateRequestSchema', 'properties', 'connection_params', 'anyOf'): 'ffa0a6b5b966e05faa742358993e147e7e41ed8ecdf87971359633426cda41a3',
    ('components', 'schemas', 'TenantLanguagesResponseSchema', 'properties', 'languages', 'items', '$ref'): '7af6574ceab729b210576e445bb1b27fb548efc12753a9131d464934e61f1e33',
    ('components', 'schemas', 'ValidationError', 'properties', 'ctx'): '42376988810bb05c78ed8ec88544706f7b2f337cd3c785f161eeef827a542e78',
    ('components', 'schemas', 'ValidationError', 'properties', 'input'): 'e90dde617c60c048f1dda134250014978a61fb1e0be3f20e61507e3b13adf00a',
    ('components', 'schemas', 'VertexAIConnectionParams'): '69a26b4823ab55b3efbc14f19e0142662e5e209a759acef6447d56f99dbcac1a',
    ('components', 'schemas', 'VertexAIConnectionRedactParams'): '243830eb12c466f51a30a08a4238c79bdc2e15c60afe3270ea0da05ae258718b',
    ('components', 'schemas', 'WebSocketConnectionParamsBase'): '59011fec5e1f48df428ac4b90a27aa24df5d0aa9e5a1ba7070b22939bb2342cc',
    ('components', 'schemas', 'WebSocketConnectionParamsBase-Input'): 'd1d550326de6988713f55cbeb2a2a83fa601bcda6d0fa8ab1bdc39112ff68872',
    ('components', 'schemas', 'WebSocketConnectionParamsBase-Output'): 'b8571b16a7d805d3402320b425265c332cf23c36511ac213c799b0ef40dbb105',
    ('components', 'schemas', 'WebSocketConnectionParamsRedactBase', 'properties', 'audio_curl'): 'dd4b23b058c68db79e1a831b0a687d79e0911694e6cc20274c2e27ace2b1291b',
    ('components', 'schemas', 'WebSocketConnectionParamsRedactBase', 'properties', 'audio_request_json'): '3382eb94c44b828f872eef53d6671a2dce7ec4be505c363f42615dfac1886a18',
    ('components', 'schemas', 'WebSocketConnectionParamsRedactBase', 'properties', 'multi_turn_config', 'anyOf'): 'b7914068a11ed030e62a7870355fd3bddf8935d604fc055e408d8019615a1917',
    ('components', 'schemas', 'WebSocketConnectionParamsRedactBase', 'properties', 'target_connection_config', 'anyOf'): '012610b564f4ec41546ccce24519189c4f445fce4ec7c7714072b983ce77e897',
    ('info', 'version'): 'f7e298a488835d88e2e51b95dee7ca18ce9c52b75e9bbbc48e9a524856ef965c',
    ('paths', '/v1/target', 'get', 'parameters'): 'c8368976cc0f936ecd8c94d2b93cfca839099c957714bc41de6585fe59aba3ba',
}
REVIEWED_SUCCESSORS = {
    'NativeConnectionParamsBase-Input': '853f13184bdf04378c8b359d87716813567aaaeedc4f6389292cc2347ce8d497',
    'NativeConnectionParamsBase-Output': '853f13184bdf04378c8b359d87716813567aaaeedc4f6389292cc2347ce8d497',
    'RestConnectionParamsBase-Input': '1b44ce22f1f59a29584e14c6943914cd340d9ded63c162023bda28ca56924412',
    'RestConnectionParamsBase-Output': '1b44ce22f1f59a29584e14c6943914cd340d9ded63c162023bda28ca56924412',
    'StreamingConnectionParamsBase-Input': 'a1023a85009002f0f6d70e0a79be95573418787bf0e727db2cc258b911898988',
    'StreamingConnectionParamsBase-Output': 'a1023a85009002f0f6d70e0a79be95573418787bf0e727db2cc258b911898988',
    'WebSocketConnectionParamsBase-Input': '3e6257a57a7eeb5c2edcc8b844fa5e5a45daa7bf8d012bc9386fcf185eef2df9',
    'WebSocketConnectionParamsBase-Output': '3e6257a57a7eeb5c2edcc8b844fa5e5a45daa7bf8d012bc9386fcf185eef2df9',
}
MODEL_SECURITY_OWNERS = {
    "client.rs": "ModelSecurityClient",
    "custom_rules.rs": "ModelSecurityCustomRulesClient",
    "groups.rs": "ModelSecurityGroupsClient",
    "scans.rs": "ModelSecurityScansClient",
}
COMPONENT_SUCCESSORS = {name + suffix: name for name in ('NativeConnectionParamsBase', 'RestConnectionParamsBase', 'StreamingConnectionParamsBase', 'WebSocketConnectionParamsBase') for suffix in ('-Input', '-Output')}


def sha(data):
    return hashlib.sha256(data).hexdigest()


def load(path):
    path = Path(path)
    if path.parent == AUDIT:
        if path.name not in AUDIT_PINS:
            raise ValueError(f'Unreviewed audit input: {path}')
        return json.loads(checked_bytes(path, AUDIT_PINS[path.name]))
    return json.loads(safe_path(path).read_text())


def encoded(value):
    return json.dumps(value, indent=2, ensure_ascii=False, sort_keys=True) + '\n'


def safe_path(path):
    path = Path(path)
    if path.is_absolute() or '..' in path.parts:
        raise ValueError(f'Unsafe pin path: {path}')
    target = ROOT / path
    current = ROOT
    for part in path.parts:
        current = current / part
        if current.is_symlink():
            raise ValueError(f'Unsafe pin path: {path}')
    if not target.resolve().is_relative_to(ROOT.resolve()):
        raise ValueError(f'Unsafe pin path: {path}')
    return target


def checked_bytes(path, expected):
    data = safe_path(path).read_bytes()
    if sha(data) != expected:
        raise ValueError(f'Pinned bytes changed: {path}')
    return data


def operations(spec):
    return {(method.upper(), path): op for path, item in spec['paths'].items()
            for method, op in item.items() if method in METHODS}


def refs(value):
    result = set()
    if isinstance(value, dict):
        if '$ref' in value:
            result.add(value['$ref'])
        for item in value.values():
            result.update(refs(item))
    elif isinstance(value, list):
        for item in value:
            result.update(refs(item))
    return result


def closure(value, spec):
    """Resolve all local JSON pointers, including recursive schemas and responses."""
    seen, pending = set(), list(refs(value))
    while pending:
        ref = pending.pop()
        if ref in seen:
            continue
        if not ref.startswith('#/'):
            raise ValueError(f'Unreviewed external reference: {ref}')
        seen.add(ref)
        target = spec
        for part in ref[2:].split('/'):
            target = target[part.replace('~1', '/').replace('~0', '~')]
        pending.extend(refs(target) - seen)
    return sorted(seen)


def differences(old, new, path=()):
    """Lossless structural delta; list order remains significant for union precedence."""
    if isinstance(old, dict) and isinstance(new, dict):
        result = []
        for key in sorted(old.keys() | new.keys()):
            at = (*path, key)
            if key not in old:
                result.append({'path': list(at), 'kind': 'added', 'new': new[key]})
            elif key not in new:
                result.append({'path': list(at), 'kind': 'removed', 'old': old[key]})
            else:
                result.extend(differences(old[key], new[key], at))
        return result
    if encoded(old) != encoded(new):
        return [{'path': list(path), 'kind': 'changed', 'old': old, 'new': new}]
    return []


def disposition(delta):
    """Fail closed on unclassified changes rather than silently adopting a new pin."""
    path = delta['path']
    kind = delta['kind']
    identity = {key: delta[key] for key in ('path', 'kind', 'old', 'new') if key in delta}
    digest = sha(encoded(identity).encode())
    if REVIEWED_DELTAS.get(tuple(path)) != digest:
        raise ValueError(f'Unclassified semantic or structural delta: {path}. Review old/new values and register the exact path and SHA256 {digest} in REVIEWED_DELTAS; identity={encoded(identity)}')
    if kind == 'removed':
        if len(path) == 3 and path[:2] == ['components', 'schemas'] and path[2] in COMPONENT_SUCCESSORS:
            return 'component_removed_or_renamed', 'Preserve old public aliases; adopt successor ' + COMPONENT_SUCCESSORS[path[2]] + ' without narrowing legacy acceptance.'
        if path == ['components', 'schemas', 'Body_upload_prompts_csv_v1_custom_attack_upload_custom_prompts_csv_post', 'properties', 'file', 'format'] and delta.get('old') == 'binary':
            return 'vendor_annotation', 'Retain binary multipart transport; contentMediaType replaces format:binary.'
        raise ValueError(f'Unclassified removal: {path}')
    if 'required' in path or (path[-1] in ANNOTATIONS and len(path) > 1 and path[-2] == 'properties'):
        raise ValueError(f'Unclassified structural change: {path}')
    if path[-1] in ANNOTATIONS or path == ['info', 'version']:
        return 'documentation', 'No runtime behavior change.'
    if path == ['paths', '/v1/target', 'get', 'parameters'] and kind == 'changed':
        old = {(p['in'], p['name']): p for p in delta.get('old', [])}
        new = {(p['in'], p['name']): p for p in delta.get('new', [])}
        if set(new) - set(old) != {('query', 'profiling_status')} or set(old) - set(new):
            raise ValueError('Unclassified target parameter addition/removal')
        for key, previous in old.items():
            changes = differences(previous, new[key])
            if any(change['path'] not in [['description'], ['schema', 'description']] for change in changes):
                raise ValueError('Unclassified target parameter contract change')
        added = new[('query', 'profiling_status')]
        if added.get('required') is not False or added.get('schema', {}).get('anyOf') != [{'$ref': '#/components/schemas/ProfilingStatusFilter'}, {'type': 'null'}]:
            raise ValueError('Unclassified profiling filter shape')
        return 'new_behavior', 'Add profiling_status; preserve old status wire values without rewriting.'
    if path[:2] != ['components', 'schemas']:
        raise ValueError(f'Unclassified non-schema delta: {path}')
    name = path[2]
    if name in {'AudioFormat', 'LanguageDropdownOptionSchema', 'TargetMetadata'}:
        return 'already_supported', 'Reuse data-plane audio/language projection; REST PCM16 remains excluded; verify all fields.'
    if name in {'BasicAuthAuthConfig', 'BasicAuthAuthConfigRedact'} or (len(path) > 4 and path[4] in {'audio_curl', 'audio_request_json'}):
        return 'typed_ergonomics', 'Preserve extension-map wire support; add named optional fields and secret-safe diagnostics.'
    if name == 'Body_upload_prompts_csv_v1_custom_attack_upload_custom_prompts_csv_post':
        return 'vendor_annotation', 'Retain binary multipart transport; contentMediaType replaces format:binary.'
    if name == 'ValidationError':
        return 'error_metadata', 'Retain sanitized shared error boundary; no arbitrary upstream body in diagnostics.'
    if name == 'TenantLanguagesResponseSchema':
        return 'component_alias', 'Bind LanguageDropdownOptionSchema to existing LanguageOption country_code projection.'
    if name in {'ProfilingStatus', 'TargetStatusFilter', 'ProfilingStatusFilter'}:
        return 'compatibility_exception', 'Retain removed legacy response values and existing status inputs; new profiling filters accept only declared values, without automatic mappings.'
    if name == 'ResponseMode':
        return 'compatibility_exception', 'Add REALTIME to schema-bound TargetCreateRequestResponseMode; keep frozen TypeScript-parity ResponseMode unchanged with Unknown(String).'
    if name == 'TargetConnectionType':
        return 'new_behavior', 'Add VERTEX_AI and N8N to schema-bound TargetCreateRequestConnectionType; keep frozen TypeScript-parity TargetConnectionType unchanged with Unknown(String).'
    if name in {'MultiTurnClientSessionConfig', 'VertexAIConnectionParams', 'VertexAIConnectionRedactParams'}:
        return 'new_behavior', 'Add declared request variants and precise nested response branches; preserve legacy branches/defaults.'
    if name in {base + suffix for base in ('NativeConnectionParams', 'RestConnectionParams', 'StreamingConnectionParams', 'WebSocketConnectionParams') for suffix in ('Base', 'Base-Input', 'Base-Output', 'RedactBase')} or name in {'TargetCreateRequestSchema', 'TargetProbeRequest', 'TargetSchema', 'TargetUpdateRequestSchema'}:
        return 'new_behavior_and_alias', 'Preserve old public names; reconcile input/output aliases and new union branches without narrowing frozen acceptance.'
    raise ValueError(f'Unclassified schema delta: {path}')


@lru_cache(maxsize=32)
def rust_code_tokens(text):
    """Single pass masks comments/literals before counting code braces.

    Supports nested block comments, raw strings, escaped normal/byte strings,
    char literals and lifetimes. This checks static impl ownership, not Rust types.
    """
    raw_pattern = re.compile(r'(?:br|cr|r)(\#*)"')
    char_pattern = re.compile(r"'(?:\\(?:u\{[0-9a-fA-F_]+\}|x[0-9a-fA-F]{2}|.)|[^'\\\n])'")
    output, i = [], 0
    while i < len(text):
        start = i
        if text.startswith('//', i):
            end = text.find('\n', i + 2)
            i = len(text) if end < 0 else end
        elif text.startswith('/*', i):
            depth, i = 1, i + 2
            while i < len(text) and depth:
                if text.startswith('/*', i):
                    depth, i = depth + 1, i + 2
                elif text.startswith('*/', i):
                    depth, i = depth - 1, i + 2
                else:
                    i += 1
            if depth:
                raise ValueError('Unterminated Rust block comment')
        else:
            raw = raw_pattern.match(text, i)
            char = char_pattern.match(text, i)
            if raw:
                close = '"' + raw.group(1)
                end = text.find(close, raw.end())
                if end < 0:
                    raise ValueError('Unterminated Rust raw string')
                i = end + len(close)
            elif text[i] == '"':
                i += 1
                while i < len(text):
                    if text[i] == '\\':
                        i += 2
                    elif text[i] == '"':
                        i += 1
                        break
                    else:
                        i += 1
                else:
                    raise ValueError('Unterminated Rust string')
            elif char:
                i = char.end()
            else:
                output.append(text[i])
                i += 1
                continue
        output.append(''.join('\n' if c == '\n' else ' ' for c in text[start:i]))
    return ''.join(output)


def impl_contains(text, class_name, method):
    """Check a named inherent impl's balanced body, excluding strings/comments."""
    text = rust_code_tokens(text)
    pattern = r'\bimpl(?:\s*<[^{}]*>)?\s+' + re.escape(class_name) + r'(?:\s*<[^{}]*>)?\s*\{'
    for match in re.finditer(pattern, text):
        depth, end = 1, match.end()
        while end < len(text) and depth:
            depth += (text[end] == '{') - (text[end] == '}')
            end += 1
        if depth == 0:
            body = text[match.end():end - 1]
            for function in re.finditer(r'\bpub\s+(?:async\s+)?(?:unsafe\s+)?(?:const\s+)?fn\s+' + re.escape(method) + r'\s*[(<]', body):
                prefix = body[:function.start()]
                if prefix.count('{') == prefix.count('}'):
                    return True
    return False


def verify_native_binding(binding):
    if not binding.get('class'):
        raise ValueError('Unqualified native binding')
    text = safe_path(binding['source']).read_text()
    if not impl_contains(text, binding['class'], binding['method']):
        raise ValueError(f'Missing Rust function: {binding}')


def bindings():
    """Reviewed mapping seeds only; current contracts are extracted independently below."""
    result = {}
    def add(key, value):
        if key in result:
            raise ValueError(f'Duplicate native binding: {key}')
        result[key] = value
    for row in load(AUDIT / 'red-team-operations.json')['operations']:
        source = f"openapi-specs/prisma-airs-redteam/{row['plane']}/" + {'data-plane': 'dp-openapi.yaml', 'management': 'mp-openapi.yaml', 'network-broker': 'AIRS-Red-Teaming-Network-Broker.yaml'}[row['plane']]
        add((source, row['method'], row['path']), [{'source': row['nativeSource'], 'method': row['nativeMethod'], 'class': row['nativeClass']}])
    for row in load(AUDIT / 'model-security-operations.json'):
        suffix = 'dataplane/data-plane.yml' if row['plane'] == 'dataplane' else 'management/mgmt-plane.yml'
        source = row['sdk']['source']
        if Path(source).parent != Path('crates/scm-sdk/src/airs/model_security'):
            raise ValueError('Unreviewed Model Security source owner')
        if Path(source).name == 'inventory.rs':
            owner = 'ModelSecurityRulesClient' if row['sdk']['rust_method'] in {'list', 'get', 'list_versions'} else 'ModelSecurityModelsClient'
        else:
            owner = MODEL_SECURITY_OWNERS[Path(source).name]
        add(('openapi-specs/prisma-airs-model-security/' + suffix, row['method'], row['path']), [{'source': source, 'method': row['sdk']['rust_method'], 'class': owner}])
    for row in load(AUDIT / 'runtime-operation-mapping.json')['operations']:
        add((row['spec_source'], row['http_method'], row['path']), [{'source': b['source'], 'method': b['method'].split('::')[-1], 'class': b['method'].split('::')[-2]} for b in row['native_bindings']])
    for key, methods in result.items():
        if not methods:
            raise ValueError(f'Missing binding: {key}')
        for binding in methods:
            verify_native_binding(binding)
    return result


def contains(current, required):
    """Compatibility subset: added keys/items are fine; scalar/default changes are not."""
    if isinstance(required, dict):
        return isinstance(current, dict) and all(k in current and contains(current[k], v) for k, v in required.items())
    if isinstance(required, list):
        return isinstance(current, list) and all(any(contains(item, wanted) for item in current) for wanted in required)
    return type(current) is type(required) and current == required


def retained_floor(current):
    """Never derive the required floor from the ledgers being checked."""
    floor = json.loads(checked_bytes(RETENTION_SEED, RETENTION_SHA256))
    if floor['auditedSourceCommit'] != RETENTION_COMMIT or floor['sdkVersion'] != '0.2.2':
        raise ValueError('Retention seed source identity changed')
    if set(floor.get('sourceLedgerSha256', {})) != set(RETENTION_LEDGER_PATHS) or not all(re.fullmatch(r'[0-9a-f]{64}', value) for value in floor['sourceLedgerSha256'].values()):
        raise ValueError('Retention source ledger provenance missing or malformed')
    # Index the large export/method sets so this check stays linear, not quadratic.
    for section in ('exports', 'methods', 'redTeamExtraMethods'):
        indexed = {x['baselineId']: x for x in current[section]}
        if len(indexed) != len(current[section]):
            raise ValueError(f'Duplicate retention identity: {section}')
        for row in floor[section]:
            if not contains(indexed.get(row['baselineId']), row):
                raise ValueError(f'Retained capability removed or changed: {section}:{row["baselineId"]}')
    for section in ('defaultBindings', 'modelSecurityExtraQueries', 'postBaselineRedTeamOperations',
                    'postBaselineRedTeamModelAdditions', 'additionalTargetTemplates'):
        if not contains(current[section], floor[section]):
            raise ValueError(f'Retained capability removed or changed: {section}')
    # Live receipts and test source positions may evolve; the public operation may not.
    fields = ('method', 'path', 'operationId', 'rustMethod', 'operationName')
    for row in floor['postBaselinePortkeyOperations']:
        expected = {key: row[key] for key in fields}
        if not any(contains(candidate, expected) for candidate in current['postBaselinePortkeyOperations']):
            raise ValueError('Retained Portkey operation removed or changed')
    # Emit current state, not a duplicate of the required floor. Additions must
    # change this artifact so --check detects stale inventories.
    return {**current, 'floorReference': {'path': str(RETENTION_SEED), 'sha256': RETENTION_SHA256, 'auditedSourceCommit': RETENTION_COMMIT},
            'preservationCheck': 'Current ledgers retain immutable 0.2.2 floor; additions allowed.'}


def verify_retention_origin(floor=None):
    """Phase 21-only digest proof; future phases retain the floor but may add rows."""
    if floor is None:
        floor = json.loads(checked_bytes(RETENTION_SEED, RETENTION_SHA256))
    if set(floor.get('sourceLedgerSha256', {})) != set(RETENTION_LEDGER_PATHS):
        raise ValueError('Retention source ledger provenance missing')
    for path, digest in floor['sourceLedgerSha256'].items():
        checked_bytes(path, digest)

def verify_management_metadata(pin, manifest_source, spec):
    path = str(VENDOR / 'mp-openapi.yaml')
    data = checked_bytes(path, manifest_source['sha256'])
    license_path = 'contracts/licenses/pan-dev-582e30543bf2.LICENSE'
    expected = {
        **{key: manifest_source[key] for key in ('id', 'family', 'repository', 'commit', 'upstreamPath', 'sourceUrl', 'license', 'redistributed', 'usedForGeneration')},
        'schemaVersion': 1, 'path': path, 'sha256': sha(data), 'bytes': len(data),
        'gitBlobSha1': hashlib.sha1(b'blob ' + str(len(data)).encode() + b'\0' + data).hexdigest(),
        'openapiVersion': spec['openapi'], 'apiVersion': spec['info']['version'],
        'expectedOperations': len(operations(spec)), 'licensePath': license_path,
        'licenseSha256': sha(safe_path(license_path).read_bytes()), 'licenseSpdx': 'MIT',
        'yamlParser': {'package': 'js-yaml', 'version': '4.3.2'},
    }
    if spec['openapi'] != '3.1.0' or spec['info']['version'] != '0.86.0' or len(operations(spec)) != 42:
        raise ValueError('Management OpenAPI identity changed')
    for key, value in expected.items():
        if encoded(pin.get(key)) != encoded(value):
            raise ValueError(f'Management source metadata mismatch: {key}')


def resolve_active_identity(row, sources):
    if row['identityKind'] == 'manifest_source':
        identity = sources[row['activeSourceId']]
    elif row['identityKind'] == 'source_metadata':
        identity = json.loads(checked_bytes(row['activeSourceId'], row['sourceMetadataSha256']))
    else:
        raise ValueError('Unrecognized active source identity kind')
    if identity['path'] != row['activeSpecificationPath'] or identity['sha256'] != row['activeSpecificationSha256']:
        raise ValueError('Active source identity does not resolve to pinned specification')
    return identity

def verify_component_successor(delta, spec):
    removed = delta['path'][2]
    successor = COMPONENT_SUCCESSORS[removed]
    current = spec['components']['schemas'].get(successor)
    if current is None or sha(encoded(current).encode()) != REVIEWED_SUCCESSORS[removed]:
        raise ValueError(f'Unreviewed component successor content: {removed} -> {successor}')

    def prefixes(old, new):
        if isinstance(old, dict) and isinstance(new, dict):
            for key in old.keys() & new.keys():
                if key in {'anyOf', 'oneOf'}:
                    before = old[key][:-1] if old[key] and old[key][-1] == {'type': 'null'} else old[key]
                    after = new[key][:-1] if new[key] and new[key][-1] == {'type': 'null'} else new[key]
                    if len(after) < len(before):
                        raise ValueError('Successor removes a legacy union branch')
                    for left, right in zip(before, after):
                        if ('$ref' in left or left == {'type': 'null'}) and left != right:
                            raise ValueError('Successor reorders a legacy union prefix')
                        prefixes(left, right)
                else:
                    prefixes(old[key], new[key])
    prefixes(delta['old'], current)
    return successor, REVIEWED_SUCCESSORS[removed]


def build():
    inventory = load(AUDIT / 'inventory.json')
    if not any(item['path'] == 'openapi-specs/prisma-airs-redteam/management/mp-openapi.yaml' for item in inventory['specs']):
        raise ValueError('Management source row missing from pinned inventory')
    if set(COUNTS) != {item['path'] for item in inventory['specs']}:
        raise ValueError('Unexpected source inventory paths')
    manifest = load('contracts/manifest.json')
    source_ids = {x['id']: x for x in manifest['sources']}
    if SOURCE_ID not in source_ids:
        raise ValueError('Management source must be explicitly registered first')
    mapping = bindings()
    source_rows, rows = [], []
    baseline = management_spec = None
    for item in inventory['specs']:
        upstream = item['path']
        management = upstream == 'openapi-specs/prisma-airs-redteam/management/mp-openapi.yaml'
        dataplane = upstream == 'openapi-specs/prisma-airs-redteam/data-plane/dp-openapi.yaml'
        path = str(VENDOR / 'mp-openapi.yaml') if management else item['activeSpecificationPath']
        checked_bytes(path, item['sha256'])
        spec = read_yaml(ROOT / path)
        if management:
            verify_management_metadata(load(VENDOR / 'source.json'), source_ids[SOURCE_ID], spec)
        ops = operations(spec)
        if len(ops) != COUNTS[upstream]:
            raise ValueError(f'Operation count changed: {upstream}')
        frozen_id = upstream.removeprefix('openapi-specs/')
        frozen = source_ids[frozen_id]
        checked_bytes(frozen['path'], item['frozenBaselineSha256'])
        source_rows.append({
            'upstreamPath': upstream, 'observedCommit': COMMIT,
            'frozenSourceId': frozen_id, 'frozenPath': frozen['path'], 'frozenSha256': item['frozenBaselineSha256'],
            'activeSpecificationPath': path, 'activeSpecificationSha256': item['sha256'],
            'activeSourceId': SOURCE_ID if management else ('contracts/vendor/red-team-data-plane/0.93.0/source.json' if dataplane else frozen_id),
            'identityKind': 'source_metadata' if dataplane else 'manifest_source',
            'version': spec['info']['version'], 'operationCount': len(ops),
            # Machine identifiers retain stable phase-number tokens; prose uses normal spacing.
            'adoptionState': management_adoption_tokens(source_ids[SOURCE_ID]['usedForGeneration'])[0] if management else 'already_adopted',
            'runtimeCompatibilityFloor': 'Frozen TypeScript 0.34.0 plus G10/G25; never infer precedence from usedForGeneration.',
            'owner': 'tools/verify-contracts.py' if not dataplane else 'tools/red_team_dataplane.py',
            'overlayOwner': ('tools/red_team_management_models.py + tools/generate_models.py' if source_ids[SOURCE_ID]['usedForGeneration'] else 'tools/red_team_management.py (plan); phase 22 implements overlay') if management else ('tools/red_team_dataplane.py + tools/generate_validation.py' if dataplane else 'frozen generator + reviewed semantics'),
            'canonicalSemanticJsonSha256': sha(json.dumps(spec, sort_keys=True, separators=(',', ':'), ensure_ascii=False).encode()),
        })
        if dataplane:
            source_rows[-1]['sourceMetadataSha256'] = sha(safe_path(source_rows[-1]['activeSourceId']).read_bytes())
        resolve_active_identity(source_rows[-1], source_ids)
        if management or dataplane:
            normalized_path = str(Path(path).with_name('openapi.json'))
            normalized_bytes = (ROOT / normalized_path).read_bytes()
            if json.loads(normalized_bytes) != spec:
                raise ValueError('Normalized vendor JSON differs from YAML')
            source_rows[-1].update(normalizedJsonPath=normalized_path, normalizedJsonSha256=sha(normalized_bytes))
        for (method, route), operation in sorted(ops.items()):
            key = (upstream, method, route)
            if key not in mapping:
                raise ValueError(f'Unmapped operation: {key}')
            item_path = spec['paths'][route]
            rows.append({'source': upstream, 'method': method, 'path': route, 'operationId': operation.get('operationId'),
                'nativeBindings': mapping[key], 'parameters': item_path.get('parameters', []) + operation.get('parameters', []),
                'requestBody': operation.get('requestBody'), 'responses': operation.get('responses', {}),
                'security': operation.get('security', spec.get('security')), 'servers': operation.get('servers', item_path.get('servers', spec.get('servers', []))),
                'reachableReferences': closure(operation, spec),
                'coverage': 'native_method_exists; static_mapping_only; not_live_acceptance'})
        if management:
            management_spec = spec
            baseline = read_yaml(ROOT / frozen['path'])
    if baseline is None or management_spec is None:
        raise ValueError('Management source row missing from pinned inventory')
    if len(rows) != 160 or set(mapping) != {(x['source'], x['method'], x['path']) for x in rows}:
        raise ValueError('AIRS mapping coverage changed')
    deltas = differences(baseline, management_spec)
    used_delta_paths = {tuple(delta['path']) for delta in deltas}
    if set(REVIEWED_DELTAS) - used_delta_paths:
        raise ValueError('Unused reviewed delta identities: ' + str(sorted(set(REVIEWED_DELTAS) - used_delta_paths)))
    used_successors = {delta['path'][2] for delta in deltas if delta['kind'] == 'removed' and len(delta['path']) == 3 and delta['path'][:2] == ['components', 'schemas']}
    if set(REVIEWED_SUCCESSORS) - used_successors:
        raise ValueError('Unused reviewed successor identities: ' + str(sorted(set(REVIEWED_SUCCESSORS) - used_successors)))
    baseline_operations = operations(baseline)
    management_rows = [x for x in rows if x['source'] == 'openapi-specs/prisma-airs-redteam/management/mp-openapi.yaml']
    baseline_closures = {(row['method'], row['path']): set(closure(baseline_operations[(row['method'], row['path'])], baseline)) for row in management_rows}
    for delta in deltas:
        category, decision = disposition(delta)
        delta.update(category=category, decision=decision)
        if category == 'component_removed_or_renamed':
            successor, successor_digest = verify_component_successor(delta, management_spec)
            delta['successorReference'] = '#/components/schemas/' + successor
            delta['successorSha256'] = successor_digest
            delta['legacyUnionPrefixPreserved'] = True
        ref = '#/components/schemas/' + delta['path'][2] if delta['path'][:2] == ['components', 'schemas'] else None
        delta['affectedOperations'] = [x['operationId'] for x in management_rows if ref in set(x['reachableReferences']) | baseline_closures[(x['method'], x['path'])] or (delta['path'][:1] == ['paths'] and x['path'] == delta['path'][1])]
    defaults = load('contracts/response-defaults.json')['bindings']
    if len(defaults) < 23:
        raise ValueError('Default-binding preservation requires explicit review')
    exports = load('contracts/native-exports.json')['capabilities']
    native = load('contracts/native-operations.json')['operations']
    extra_queries = [{'plane': x['plane'], 'method': x['method'], 'path': x['path'], 'options': x['sdk_extra_query']} for x in load(AUDIT / 'model-security-operations.json') if x['sdk_extra_query']]
    current_inventory = {'schemaVersion': 1, 'policy': 'Current capability inventory must retain the immutable floor referenced below; vendor omissions never authorize removal.',
        'baselineCommit': '4a80dbdb3dc14365e1c25d838ac0bd139485d48c',
        'exports': [{'baselineId': x['baselineId'], 'rustPaths': x['rustPaths']} for x in exports],
        'methods': [{'baselineId': x['baselineId'], 'rustMethods': x['rustMethods']} for x in native],
        'defaultBindings': defaults, 'modelSecurityExtraQueries': extra_queries,
        'postBaselinePortkeyOperations': load('contracts/vendor/portkey-access/a27feb9/operations.json')['operations'],
        'postBaselineRedTeamOperations': [{'method': x['method'], 'path': x['path'], 'nativeBindings': [{'class': x['nativeClass'], 'method': x['nativeMethod'], 'source': x['source']}]} for x in load('contracts/vendor/red-team-data-plane/0.93.0/operations.json')['operations']],
        'postBaselineRedTeamModelAdditions': load('contracts/vendor/red-team-data-plane/0.93.0/model-additions.json'),
        'preservedPolicies': ['G10: omission/null/value and opt-in no-default mode', 'G25: response format tolerance, open enums and unknown keys; requests retain reviewed strictness', 'Legacy target status inputs transmitted unchanged', 'Legacy provider order and first-success resolution for absent multi-turn discriminator', 'Pagination, bounded collections, cancellation and uncertain mutation handling', 'Authentication, destination admission, no authenticated redirect forwarding, secret-safe diagnostics', 'Gateway/Portkey, AgentGuard, realtime, DLP and IAM retain separate provenance outside AIRS denominator'],
        'redTeamExtraMethods': [{'baselineId': x['baselineId'], 'rustMethods': x['rustMethods'], 'source': x['source']} for x in native if x['family'] == 'red-team' and any(t in x['source'] for t in ('/eula.rs', '/instances.rs'))],
        'additionalTargetTemplates': ['RedTeamTargetsClient::get_target_metadata', 'RedTeamTargetsClient::get_target_templates'],
        'evidenceLimit': 'Static preservation inventory; phase 23 must exercise old consumers and round-trips.'}
    retention = retained_floor(current_inventory)
    adopted = source_ids[SOURCE_ID]['usedForGeneration']
    if type(adopted) is not bool:
        raise ValueError('Management generation adoption must be explicit boolean')
    if adopted:
        verify_runtime_adoption()
    return {
        Path('contracts/airs-active-sources.json'): {'schemaVersion': 1, 'snapshotCommit': COMMIT, 'sources': source_rows},
        Path('contracts/airs-operations.json'): {'schemaVersion': 1, 'snapshotCommit': COMMIT, 'bindingProvenance': str(AUDIT), 'operations': rows},
        Path('contracts/airs-retained-capabilities.json'): retention,
        VENDOR / 'operations.json': {'schemaVersion': 1, 'sourceId': SOURCE_ID, 'operations': management_rows,
            'runtimeEvidence': historical_management_runtime_evidence() if adopted else None},
        Path('contracts/typescript-035-management-compatibility.json'): management_compatibility_evidence(),
        # Stable machine identifiers consumed downstream; do not reformat phase tokens.
        VENDOR / 'compatibility-decisions.json': {'schemaVersion': 1, 'sourceId': SOURCE_ID, 'baselineVersion': baseline['info']['version'], 'currentVersion': management_spec['info']['version'], 'deltas': deltas, 'plan': 'docs/red-team-management-0.86-plan.md', 'runtimeAdoption': management_adoption_tokens(adopted)[1], 'targetResponseDeclarations': target_response_declarations(management_spec) if adopted else None},
    }


MANAGEMENT_WIRE_TESTS = (
    'vertex_client_session_and_new_values_preserve_both_response_modes',
    'request_new_variants_and_vertex_patterns_are_strict',
    'missing_defaults_and_explicit_null_are_distinct',
    'new_projected_fields_keep_previous_passthrough_values',
    'legacy_multi_turn_resolution_and_hybrid_provider_precedence_remain',
    'all_five_operations_preserve_declared_sensitive_response_shapes',
    'profiling_filter_propagates_through_every_page_and_collection_surface',
    'invalid_profiling_filter_never_acquires_token_or_dispatches',
    'unknown_discriminators_and_required_vertex_types_stay_validated',
    'untagged_complete_stateful_config_keeps_frozen_branch_selection',
    'explicit_client_session_wins_over_legacy_structural_fields',
    'malformed_target_response_errors_never_echo_credentials',
    'typed_projection_values_override_raw_collisions_and_unset_restores_them',
)


MANAGEMENT_FIXTURE_TESTS = (
    'all_twelve_projection_occurrences_preserve_independent_raw_oracles',
    'vendor_union_required_types_formats_and_legacy_order_are_independent',
    'all_vendor_enum_additions_and_unknown_response_values_roundtrip',
)


TARGET_RESPONSE_DECLARATIONS = (
    ('POST', '/v1/target', 'TargetResponseSchema'),
    ('GET', '/v1/target/{target_uuid}', 'TargetRedactSchema'),
    ('PUT', '/v1/target/{target_uuid}', 'TargetSchema'),
    ('POST', '/v1/target/probe', 'TargetResponseSchema'),
    ('PUT', '/v1/target/{target_uuid}/profile', 'TargetSchema'),
)


def target_response_declarations(spec):
    """Record field coverage without claiming full JSON Schema equivalence."""
    graph = load('crates/scm-sdk/src/validation/schema_graph.json')
    root_name = 'src/models/red-team.ts#TargetResponseSchema'
    sid = graph['roots'][root_name]
    node = graph['nodes'][sid]
    fields = node['properties']
    required = sorted(name for name, field in fields.items() if field['inputPresence'] == 'rejects')
    rows = []
    for method, path, component in TARGET_RESPONSE_DECLARATIONS:
        operation = spec['paths'][path][method.lower()]
        expected_ref = '#/components/schemas/' + component
        response_refs = [response.get('content', {}).get('application/json', {}).get('schema', {}).get('$ref')
                         for status, response in operation['responses'].items() if status.startswith('2')]
        if expected_ref not in response_refs:
            raise ValueError('Target operation response declaration changed')
        vendor = spec['components']['schemas'][component]
        vendor_required = sorted(vendor.get('required', []))
        if required != vendor_required or not set(vendor['properties']) <= fields.keys():
            raise ValueError('Retained target root required fields or vendor field coverage differ')
        comparisons = []
        for name, schema in sorted(vendor['properties'].items()):
            nullable = schema.get('type') == 'null' or any(
                branch.get('type') == 'null' for branch in schema.get('anyOf', []) + schema.get('oneOf', []))
            if nullable and fields[name]['inputNull'] != 'accepts':
                raise ValueError('Retained target root rejects a declared nullable field: ' + name)
            comparisons.append({'field': name, 'vendorSchema': schema, 'vendorRequired': name in vendor_required,
                                'vendorExplicitlyNullable': nullable, 'sdkFieldSchema': fields[name]['schema'],
                                'sdkAcceptsMissing': fields[name]['inputPresence'] == 'accepts',
                                'sdkAcceptsNull': fields[name]['inputNull'] == 'accepts'})
        rows.append({'method': method, 'path': path, 'vendorSchema': component,
                     'retainedSdkRoot': root_name, 'retainedSdkSchema': sid,
                     'requiredFields': required, 'fieldComparisons': comparisons,
                     'sdkAdditionalFields': sorted(fields.keys() - vendor['properties'].keys()),
                     'supersetJustification': 'All declared top-level fields are represented, required fields agree, and declared nullable fields remain accepted. The shared root retains optional credentials and connection fields, including on operations whose vendor response base omits them. Nested provider/auth/multi-turn variants are verified by the bound fixture matrix.',
                     'comparisonLimit': 'Field presence/nullability comparison and reviewed nested fixtures, not a formal proof of complete JSON Schema equivalence. Frozen response tolerance and passthrough semantics remain authoritative.',
                     'fixture': 'crates/scm-sdk/tests/red_team_management_contracts.rs::all_five_operations_preserve_declared_sensitive_response_shapes',
                     'responseModes': ['Response', 'ResponseWithoutDefaults']})
    return rows


def historical_management_runtime_evidence():
    """Read the immutable adoption snapshot; never present its hashes as current."""
    path = VENDOR / 'operations.json'
    records = [row for row in load('contracts/manifest.json')['vendorArtifacts'] if row['path'] == str(path)]
    expected = '1888202d439416ba32ccc529095a40231bca1af6d171b73279aa67ef665007f6'
    if len(records) != 1 or records[0]['sha256'] != expected:
        raise ValueError('Historical management operations snapshot pin differs')
    data = checked_bytes(path, expected)
    if len(data) != records[0]['bytes']:
        raise ValueError('Historical management operations snapshot length differs')
    return json.loads(data)['runtimeEvidence']


def management_compatibility_evidence():
    historical = historical_management_runtime_evidence()
    return {
        'schemaVersion': 1,
        'scope': 'Additive TypeScript 0.35.0 compatibility evidence. The manifest-pinned vendor operations artifact retains historical adoption hashes; current source bindings below are verified independently. Neither snapshot establishes test execution, directional live acceptance, or completion of a historical release phase.',
        'historicalArtifact': str(VENDOR / 'operations.json'),
        'historicalArtifactSha256': sha(safe_path(VENDOR / 'operations.json').read_bytes()),
        'historicalRuntimeEvidence': historical,
        'currentRuntimeEvidence': management_runtime_evidence(),
    }


def management_runtime_evidence():
    paths = [
        'contracts/red-team-management-model-overlay.json',
        'contracts/red-team-management-union-extensions.json',
        'crates/scm-sdk/src/airs/red_team/targets.rs',
        'crates/scm-sdk/examples/red_team_management.rs',
        'crates/scm-sdk/tests/red_team_management_contracts.rs',
        'crates/scm-sdk/tests/red_team_management_models.rs',
        'crates/scm-sdk/tests/red_team_management_fixtures.rs',
        'tools/test_red_team_management_models.py',
    ]
    return {'scope': 'Source-bound implementation and test bindings; execution is established only by separate phase checks, never live acceptance.',
            'sourceBindings': [{'path': path, 'sha256': sha(safe_path(path).read_bytes())} for path in paths],
            'wireTestBindings': list(MANAGEMENT_WIRE_TESTS),
            'fixtureTestBindings': list(MANAGEMENT_FIXTURE_TESTS)}


def management_adoption_tokens(adopted):
    # Stable machine identifiers consumed downstream; intentionally not reformatted.
    return (('runtime_adopted_phase22', 'adopted_phase22') if adopted else
            ('pinned_for_phase22_not_runtime_adopted', 'pending_phase22'))


def verify_runtime_adoption():
    ledger = load('contracts/red-team-management-model-overlay.json')
    if ledger.get('generator') != 'tools/red_team_management_models.py':
        raise ValueError('Management overlay generator identity differs')
    checked_bytes(ledger['generator'], ledger['generatorSha256'])
    checked_bytes(VENDOR / 'mp-openapi.yaml', ledger['sourceSha256'])
    bindings = ledger.get('sourceBindings', [])
    required = {'tools/red_team_management_models.py', 'tools/generate_models.py',
                'crates/scm-sdk/src/models/red_team_management.rs',
                'crates/scm-sdk/src/models/generated.rs', 'NOTICE',
                'tools/test_red_team_management_models.py',
                'crates/scm-sdk/src/validation/schema_graph.json', 'crates/scm-sdk/src/validation.rs',
                'crates/scm-sdk/tests/red_team_management_models.rs',
                'crates/scm-sdk/tests/red_team_management_contracts.rs',
                str(VENDOR / 'mp-openapi.yaml'), str(VENDOR / 'openapi.json')}
    if (not isinstance(bindings, list) or len({row['path'] for row in bindings}) != len(bindings)
            or not required <= {row['path'] for row in bindings}):
        raise ValueError('Management overlay lacks required source bindings')
    for row in bindings:
        checked_bytes(row['path'], row['sha256'])
    extensions = load('contracts/red-team-management-union-extensions.json')
    if extensions.get('extensions') != ledger.get('runtimeUnionExtensions'):
        raise ValueError('Management union extension ledgers differ')
    checked_bytes(extensions['baselinePins'], extensions['baselinePinsSha256'])
    graph = load('crates/scm-sdk/src/validation/schema_graph.json')
    if not ledger.get('publicModels') or any(sid not in graph['nodes'] for sid in ledger['publicModels'].values()):
        raise ValueError('Management public models lack runtime graph nodes')
    tokens = rust_code_tokens(safe_path('crates/scm-sdk/tests/red_team_management_contracts.rs').read_text())
    for name in MANAGEMENT_WIRE_TESTS:
        if not re.search(r'\bfn\s+' + re.escape(name) + r'\s*\(', tokens):
            raise ValueError('Missing management contract test binding: ' + name)


HISTORICAL_LIVE_REFERENCE = 'contracts/historical/0.2.2-live-evidence.json'
HISTORICAL_LIVE_SHA256 = 'a95495f685ec3718dc91e400c5a0f3184b30742e3997148f484222c3ab39e0c5'


def verify_offline_live_boundary():
    """Retain old acceptance without promoting it onto changed runtime source."""
    reference = json.loads(checked_bytes(HISTORICAL_LIVE_REFERENCE, HISTORICAL_LIVE_SHA256))
    checked_bytes(reference['report'], reference['reportSha256'])
    current = ROOT / 'contracts/live-evidence.json'
    if current.exists() or current.is_symlink():
        raise ValueError('Phase 22/23 offline scope cannot claim a current live receipt')
    for name, rows in (('native-operations', 'operations'), ('native-exports', 'capabilities'),
                       ('qualification', 'operations')):
        document = load('contracts/' + name + '.json')
        if document.get('liveReceipt') is not None or document['summary'].get('liveVerified') != 0:
            raise ValueError('Offline qualification must have zero current live acceptance: ' + name)
        for row in document[rows]:
            if row.get('liveAcceptance') != 'unverified' or 'liveEvidence' in row:
                raise ValueError('Historical live acceptance leaked into current ledger: ' + name)


def verify_service_qualification():
    """Require current-source bounded read observations without promoting provider workflows."""
    import live_evidence
    import native_operations
    reference = json.loads(checked_bytes(HISTORICAL_LIVE_REFERENCE, HISTORICAL_LIVE_SHA256))
    checked_bytes(reference['report'], reference['reportSha256'])
    _, envelope = live_evidence.load(ROOT, required=True)
    if envelope['report']['schemaVersion'] != 2:
        raise ValueError('Current-source qualification requires fresh bounded read-only management evidence')
    if envelope['report'].get('limitations') != live_evidence.READ_ONLY_LIMITATIONS:
        raise ValueError('Current-source qualification requires explicit exercised and unexercised endpoint profiles')
    annotations, _, receipt = native_operations.live_projection(ROOT)
    for name, key in (('native-operations', 'operations'), ('qualification', 'operations'),
                      ('native-exports', 'capabilities')):
        document = load('contracts/' + name + '.json')
        if document.get('liveReceipt') != receipt:
            raise ValueError('Current service ledger has a stale live receipt: ' + name)
        rows = document[key]
        if name != 'native-exports':
            native_operations.verify_live_rows(rows, annotations)
        else:
            for row in rows:
                expected = annotations.get(row.get('operationId'), {})
                if (row.get('liveAcceptance') != expected.get('liveAcceptance', 'unverified')
                        or row.get('liveEvidence') != expected.get('liveEvidence')):
                    raise ValueError('Export live evidence exceeds the recorded workflow')
        if document['summary'].get('liveVerified') != sum(row['liveAcceptance'] == 'verified' for row in rows):
            raise ValueError('Current service ledger has an incorrect live count: ' + name)


def verify_cumulative_qualification(*, package_receipts=False, phase=23):
    """Verify owned offline artifacts; final package checks require real receipts."""
    import management_fixtures
    import management_consumers
    if load('contracts/native-fixtures/red-team-management.json') != management_fixtures.expected():
        raise ValueError('Stale derived management fixture corpus')
    tokens = rust_code_tokens(safe_path('crates/scm-sdk/tests/red_team_management_fixtures.rs').read_text())
    for name in MANAGEMENT_FIXTURE_TESTS:
        if not re.search(r'\bfn\s+' + re.escape(name) + r'\s*\(', tokens):
            raise ValueError('Missing management fixture test binding: ' + name)
    management_consumers.verify(root=ROOT)
    if package_receipts:
        import importlib.util
        path = ROOT / 'scripts/check-management-package.py'
        spec = importlib.util.spec_from_file_location('management_package_verifier', path)
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        for compiler in ('1.89.0', '1.99.0'):
            prefix = 'phase-24-release' if phase == 24 else 'phase-23'
            module.verify_receipt(ROOT, ROOT / f'docs/checks/{prefix}-package-{compiler}.json', compiler)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--check', action='store_true')
    parser.add_argument('--current-source-compatibility', action='store_true',
                        help='Check retained runtime contracts against fresh bounded reads; no release or package acceptance')
    parser.add_argument('--verify-package-receipts', action='store_true', help='Phase 23/24 final gate: verify both executed clean package receipts')
    parser.add_argument('--verify-retention-origin', action='store_true', help='Phase 21 only: verify current source ledgers equal audited seed digests')
    parser.add_argument('--require-through-phase', type=int, choices=range(21, 26), default=21)
    args = parser.parse_args()
    try:
        if args.current_source_compatibility and (not args.check or args.verify_package_receipts
                or args.verify_retention_origin or args.require_through_phase != 21):
            raise ValueError('Current-source compatibility requires --check with default phase 21 and no package or retention verification')
        if args.verify_package_receipts and args.require_through_phase not in (23, 24):
            raise ValueError('Package receipt verification requires phase 23 or 24')
        if args.require_through_phase > 24:
            raise ValueError('Phase 25 qualification evidence is not implemented; static plan evidence cannot advance this phase')
        if args.verify_retention_origin:
            verify_retention_origin()
        outputs = build()
        if args.current_source_compatibility or args.require_through_phase in (22, 23, 24):
            source = next(row for row in load('contracts/manifest.json')['sources'] if row['id'] == SOURCE_ID)
            if source['usedForGeneration'] is not True:
                raise ValueError('Management runtime adoption is not declared')
            verify_runtime_adoption()
            if args.current_source_compatibility or args.require_through_phase == 24:
                verify_service_qualification()
            else:
                verify_offline_live_boundary()
        if args.current_source_compatibility:
            verify_cumulative_qualification(package_receipts=False)
        elif args.require_through_phase in (23, 24):
            verify_cumulative_qualification(package_receipts=args.verify_package_receipts, phase=args.require_through_phase)
        for path, value in outputs.items():
            content = encoded(value)
            if args.check:
                if not (ROOT / path).is_file() or (ROOT / path).read_text() != content:
                    raise ValueError(f'Stale contract artifact: {path}')
            else:
                (ROOT / path).parent.mkdir(parents=True, exist_ok=True)
                (ROOT / path).write_text(content)
        print('AIRS contracts: 7 source pins, 160 operation mappings, 42 management operations; '
              + ('current-source compatibility and bounded reads verified; release acceptance not assessed'
                 if args.current_source_compatibility else 'static evidence only'))
    except (ValueError, KeyError, OSError) as exc:
        parser.exit(1, str(exc) + '\n')


if __name__ == '__main__':
    main()
