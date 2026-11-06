"""Offline regressions for phase 25 AIRS upstream drift detection (no network)."""
import contextlib
import copy
import hashlib
import io
import json
import shutil
import socket
import tempfile
import unittest
import uuid
from pathlib import Path
from unittest.mock import patch

import airs_drift as drift
import red_team_management as rtm

NB = 'openapi-specs/prisma-airs-redteam/network-broker/AIRS-Red-Teaming-Network-Broker.yaml'
MP = drift.MANAGEMENT_UPSTREAM
MGMT = 'openapi-specs/prisma-airs/management/mgmt-service_latest.yaml'
SCAN = 'openapi-specs/prisma-airs/scan/scan-service_latest.yaml'
OTHER_COMMIT = 'a' * 40
NULL = {'type': 'null'}


def blob(data):
    return hashlib.sha1(b'blob ' + str(len(data)).encode() + b'\0' + data).hexdigest()


def dumps(spec):
    return (json.dumps(spec, indent=2) + '\n').encode()


def write_candidate(directory, files, commit=OTHER_COMMIT, repository=drift.REPOSITORY, tree_hook=None):
    """Build a self-consistent candidate; tree_hook may corrupt it afterwards."""
    directory = Path(directory)
    products = []
    for product in drift.PRODUCT_DIRECTORIES:
        entries = []
        for upstream, data in sorted(files.items()):
            if upstream.startswith(product + '/'):
                entries.append({'path': upstream[len(product) + 1:], 'mode': '100644', 'gitBlobSha1': blob(data),
                                'sha256': hashlib.sha256(data).hexdigest(), 'bytes': len(data)})
        products.append({'path': product, 'treeSha': drift.git_tree_hash(entries), 'truncated': False,
                         'entries': entries})
    tree = {'schemaVersion': 1, 'repository': repository, 'commit': commit, 'commitTreeSha': 'b' * 40,
            'productTrees': products}
    tree['contentSha256'] = drift.digest(drift.content_identity(tree))
    tree['observation'] = {'method': 'synthetic test candidate'}
    if tree_hook:
        tree_hook(tree)
    for upstream, data in files.items():
        path = directory / 'files' / upstream
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(data)
    (directory / 'files').mkdir(parents=True, exist_ok=True)
    (directory / 'tree.json').write_text(rtm.encoded(tree))
    return directory


def sdk_fingerprint():
    digest = hashlib.sha256()
    for relative in ('crates/scm-sdk', 'Cargo.toml', 'Cargo.lock', 'LICENSE', 'NOTICE', 'contracts/airs-drift.json'):
        path = rtm.ROOT / relative
        paths = sorted(p for p in path.rglob('*') if p.is_file() and 'target' not in p.parts) if path.is_dir() else [path]
        for item in paths:
            digest.update(item.relative_to(rtm.ROOT).as_posix().encode() + b'\0' + item.read_bytes())
    return digest.hexdigest()


class DriftTestCase(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.base = drift.baseline()

    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.work = Path(self.temporary.name)
        self.output_root = rtm.ROOT / '.local/airs-drift' / ('test-' + uuid.uuid4().hex)
        self.addCleanup(shutil.rmtree, self.output_root, ignore_errors=True)

    def spec(self, upstream):
        return copy.deepcopy(self.base[upstream]['spec'])

    def files(self):
        return {upstream: item['bytes'] for upstream, item in self.base.items()}

    def candidate(self, mutations=None, *, commit=OTHER_COMMIT, extra=None, drop=(), name='candidate', **options):
        files = {upstream: item['bytes'] for upstream, item in self.base.items() if upstream not in drop}
        for upstream, mutate in (mutations or {}).items():
            value = mutate(self.spec(upstream))
            files[upstream] = value if isinstance(value, bytes) else dumps(value)
        files.update(extra or {})
        return write_candidate(self.work / name, files, commit=commit, **options)

    def compare(self, mutations=None, **options):
        return drift.compare_candidate(self.candidate(mutations, **options))

    def only(self, report, cls):
        return [change for change in report['changes'] if change['class'] == cls]

    def assertClass(self, report, cls, disposition=None, pointer=None):
        matches = [c for c in self.only(report, cls)
                   if (disposition is None or c['disposition'] == disposition)
                   and (pointer is None or c['pointer'] == pointer)]
        self.assertTrue(matches, f'{cls} {disposition} {pointer} not in {[(c["class"], c["disposition"], c["pointer"]) for c in report["changes"]]}')
        return matches[0]


class IdentityAndDeterminismTests(DriftTestCase):
    def test_ledger_is_current_and_bound_to_the_pinned_sources(self):
        document = drift.check()
        self.assertEqual(document['snapshotCommit'], rtm.COMMIT)
        self.assertEqual(document['productDirectories'], list(drift.PRODUCT_DIRECTORIES))
        self.assertEqual(document['changeClasses'], drift.CHANGE_CLASSES)
        self.assertEqual(document['reviewedCandidateChanges'], [])
        self.assertEqual(document['policy'], drift.POLICY)
        self.assertNotIn('baselineTree', document)
        self.assertEqual({row['upstreamPath'] for row in document['sources']}, set(rtm.COUNTS))
        for row in document['sources']:
            data = self.base[row['upstreamPath']]['bytes']
            self.assertEqual(row['gitBlobSha1'], blob(data))
            self.assertEqual(row['operationCount'], rtm.COUNTS[row['upstreamPath']])
        management = next(row for row in document['sources'] if row['upstreamPath'] == MP)
        self.assertEqual(management['gitBlobSha1'], rtm.load(rtm.VENDOR / 'source.json')['gitBlobSha1'])
        self.assertEqual([row['path'] for row in document['bindings']], [str(p) for p in drift.BINDINGS])
        self.assertNotRegex(rtm.encoded(document), r'\d{4}-\d{2}-\d{2}T\d{2}:')

    def test_baseline_self_compare_is_unchanged_and_deterministic(self):
        first = drift.verify(rtm.ROOT)
        self.assertEqual(first['status'], 'unchanged')
        self.assertEqual(first['changes'], [])
        candidate = self.candidate()
        one, two = drift.compare_candidate(candidate), drift.compare_candidate(candidate)
        self.assertEqual(rtm.encoded(one), rtm.encoded(two))
        self.assertEqual(one['status'], 'unchanged')
        self.assertEqual(one['adoption'], 'not_adopted')
        with self.assertRaisesRegex(ValueError, 'repository root'):
            drift.verify(self.work)

    def test_change_identity_is_the_reviewed_delta_digest(self):
        def mutate(spec):
            spec['components']['schemas']['Channel']['properties']['region'] = {'type': 'string'}
            return spec
        report = self.compare({NB: mutate})
        change = self.assertClass(report, 'property_added', 'additive')
        identity = {'path': [NB, 'components', 'schemas', 'Channel', 'properties', 'region'], 'kind': 'added',
                    'new': {'type': 'string'}}
        self.assertEqual(change['id'], hashlib.sha256(rtm.encoded(identity).encode()).hexdigest())
        self.assertEqual(change['pointer'], '/components/schemas/Channel/properties/region')
        self.assertIn('GET /v1/channels', change['affectedOperations'])
        self.assertIn('src/red-team/network-broker-client.ts:RedTeamNetworkBrokerClient.listChannels',
                      change['retainedCapability'])
        self.assertEqual(report['status'], 'review_required')
        self.assertNotRegex(rtm.encoded(report), r'\d{4}-\d{2}-\d{2}T\d{2}:')


class AdditionTests(DriftTestCase):
    def test_each_addition_class(self):
        def mutate(spec):
            channels = spec['paths']['/v1/channels']
            channels['get']['parameters'].append({'in': 'query', 'name': 'region', 'required': False, 'schema': {'type': 'string'}})
            channels['post']['parameters'].append({'in': 'header', 'name': 'X-Tenant', 'required': True, 'schema': {'type': 'string'}})
            channels['get']['responses']['404'] = {'$ref': '#/components/responses/NotFound'}
            channels['get']['responses']['200']['content']['application/x-ndjson'] = {'schema': {'type': 'string'}}
            channels['get']['requestBody'] = {'required': False, 'content': {'application/json': {'schema': {'type': 'object'}}}}
            spec['paths']['/v1/channels/{id}/restart'] = {'post': {'operationId': 'restartChannel', 'responses': {'204': {'description': 'ok'}}}}
            spec['components']['schemas']['ChannelStatus']['enum'].append('PAUSED')
            spec['components']['schemas']['Region'] = {'type': 'string'}
            spec['components']['schemas']['Channel']['properties']['region'] = {'$ref': '#/components/schemas/Region'}
            return spec
        report = self.compare({NB: mutate}, extra={'openapi-specs/prisma-airs/new/new-service.yaml': dumps(
            {'openapi': '3.1.0', 'info': {'title': 'New', 'version': '1.0.0'}, 'paths': {}})})
        self.assertClass(report, 'parameter_added', 'additive', '/paths/~1v1~1channels/get/parameters/query/region')
        self.assertClass(report, 'parameter_added', 'breaking', '/paths/~1v1~1channels/post/parameters/header/X-Tenant')
        self.assertClass(report, 'response_status_added', 'additive', '/paths/~1v1~1channels/get/responses/404')
        self.assertClass(report, 'media_type_added', 'additive')
        self.assertClass(report, 'request_body_added', 'additive')
        added = self.assertClass(report, 'operation_added', 'additive', '/paths/~1v1~1channels~1{id}~1restart/post')
        self.assertEqual(added['nativeBindings'], [])
        self.assertEqual(added['retainedCapability'], [])
        self.assertEqual(self.assertClass(report, 'enum_value_added', 'additive')['new'], 'PAUSED')
        self.assertClass(report, 'component_added', 'additive', '/components/schemas/Region')
        self.assertClass(report, 'property_added', 'additive', '/components/schemas/Channel/properties/region')
        self.assertClass(report, 'source_added', 'additive')
        self.assertEqual(next(s for s in report['sources'] if s['upstreamPath'].endswith('new-service.yaml'))['status'], 'added')


class DeletionTests(DriftTestCase):
    def test_deletions_retain_native_capability(self):
        def mutate(spec):
            del spec['paths']['/v1/channels']['get']
            post = spec['paths']['/v1/channels']['post']
            del post['responses']['403']
            del post['requestBody']
            spec['components']['schemas']['ChannelStatus']['enum'].remove('DRAFT')
            del spec['components']['schemas']['Channel']['properties']['added_by']
            return spec
        report = self.compare({NB: mutate}, drop=(SCAN,))
        removed = self.assertClass(report, 'operation_removed', 'breaking', '/paths/~1v1~1channels/get')
        self.assertEqual(removed['nativeBindings'][0]['method'], 'list_channels')
        self.assertEqual(removed['sdkRetention'], 'retained')
        self.assertEqual(removed['retainedCapability'],
                         ['src/red-team/network-broker-client.ts:RedTeamNetworkBrokerClient.listChannels'])
        self.assertClass(report, 'response_status_removed', 'breaking')
        self.assertClass(report, 'request_body_removed', 'breaking')
        self.assertEqual(self.assertClass(report, 'enum_value_removed', 'breaking')['old'], 'DRAFT')
        self.assertClass(report, 'property_removed', 'breaking')
        source = self.assertClass(report, 'source_removed', 'breaking')
        self.assertEqual(source['sdkRetention'], 'retained')
        self.assertEqual(len(source['affectedOperations']), rtm.COUNTS[SCAN])
        self.assertTrue(source['retainedCapability'])

    def test_parameter_media_and_component_removals(self):
        def mutate(spec):
            get = spec['paths']['/v1/channels']['get']
            get['parameters'] = [p for p in get['parameters'] if p['name'] != 'search']
            del get['responses']['200']['content']['application/json']
            del spec['components']['schemas']['ChannelStats']
            spec['paths']['/v1/channels/stats']['get']['responses']['200']['content']['application/json']['schema'] = {'type': 'object'}
            return spec
        report = self.compare({NB: mutate})
        self.assertClass(report, 'parameter_removed', 'breaking', '/paths/~1v1~1channels/get/parameters/query/search')
        self.assertClass(report, 'media_type_removed', 'breaking')
        self.assertClass(report, 'component_removed', 'breaking', '/components/schemas/ChannelStats')


def rename(spec, before, after, *, edit=None, keep_reference=False):
    schemas = spec['components']['schemas']
    schemas[after] = schemas.pop(before)
    if edit:
        edit(schemas[after])
    if not keep_reference:
        spec = json.loads(json.dumps(spec).replace(f'"#/components/schemas/{before}"', f'"#/components/schemas/{after}"'))
    return spec


class RenameTests(DriftTestCase):
    def test_exact_rename_rewrites_all_references(self):
        report = self.compare({NB: lambda spec: rename(spec, 'ChannelStats', 'ChannelStatistics')})
        changes = report['changes']
        self.assertEqual([c['class'] for c in changes], ['component_renamed'])
        self.assertEqual(changes[0]['renamedFrom'], 'ChannelStats')
        self.assertEqual(changes[0]['new'], 'ChannelStatistics')
        self.assertEqual(changes[0]['disposition'], 'breaking')
        self.assertEqual(changes[0]['affectedOperations'], ['GET /v1/channels/stats'])

    def test_edited_rename_is_removed_plus_added(self):
        def edit(schema):
            schema['properties']['paused_channels'] = {'type': 'integer'}
        report = self.compare({NB: lambda spec: rename(spec, 'ChannelStats', 'ChannelStatistics', edit=edit)})
        classes = {c['class'] for c in report['changes']}
        self.assertIn('component_removed', classes)
        self.assertIn('component_added', classes)
        self.assertIn('type_changed', classes)
        self.assertNotIn('component_renamed', classes)

    def test_ambiguous_rename_fails_classification(self):
        def mutate(spec):
            spec = rename(spec, 'ChannelStats', 'StatsA')
            spec['components']['schemas']['StatsB'] = copy.deepcopy(spec['components']['schemas']['StatsA'])
            return spec
        with self.assertRaisesRegex(ValueError, '(?s)Ambiguous component rename.*digest=[0-9a-f]{64}'):
            self.compare({NB: mutate})

    def test_dangling_reference_fails_classification(self):
        with self.assertRaisesRegex(ValueError, 'Dangling local reference'):
            self.compare({NB: lambda spec: rename(spec, 'ChannelStats', 'ChannelStatistics', keep_reference=True)})


class TypeAndRequirednessTests(DriftTestCase):
    def test_type_and_requiredness_both_directions(self):
        def mutate(spec):
            schemas = spec['components']['schemas']
            schemas['Channel']['properties']['name']['type'] = 'integer'
            schemas['CreateChannelRequest']['required'] = ['description']
            get = spec['paths']['/v1/channels']['get']
            get['parameters'][1]['required'] = True
            schemas['Channel']['properties']['name']['maxLength'] = 32
            return spec
        report = self.compare({NB: mutate})
        self.assertClass(report, 'type_changed', 'breaking', '/components/schemas/Channel/properties/name/type')
        self.assertClass(report, 'requiredness_changed', 'breaking', '/components/schemas/CreateChannelRequest/required/description')
        self.assertClass(report, 'requiredness_changed', 'additive', '/components/schemas/CreateChannelRequest/required/name')
        self.assertClass(report, 'requiredness_changed', 'breaking', '/paths/~1v1~1channels/get/parameters/query/search/required')
        self.assertClass(report, 'constraint_changed', 'breaking', '/components/schemas/Channel/properties/name/maxLength')

    def test_required_reorder_is_byte_only(self):
        def mutate(spec):
            spec['components']['schemas']['ErrorResponse']['required'] = ['message', 'code']
            return spec
        report = self.compare({NB: mutate})
        self.assertEqual(report['status'], 'byte_only')


def audio_format(spec):
    return spec['components']['schemas']['TargetMetadata']['properties']['audio_format']


class UnionTests(DriftTestCase):
    REF = {'$ref': '#/components/schemas/AudioFormat'}

    def union(self, branches):
        def mutate(spec):
            audio_format(spec)['anyOf'] = branches
            return spec
        return self.compare({MP: mutate})

    def test_append_before_trailing_null_is_additive(self):
        report = self.union([self.REF, {'type': 'string'}, NULL])
        change = self.assertClass(report, 'union_variant_appended', 'additive',
                                  '/components/schemas/TargetMetadata/properties/audio_format/anyOf')
        self.assertEqual(change['new'], {'type': 'string'})
        self.assertEqual(change['priorDecision']['category'], 'already_supported')
        self.assertTrue(change['affectedOperations'])

    def test_order_insert_and_removal_are_breaking(self):
        self.assertClass(self.union([NULL, self.REF]), 'union_order_changed', 'breaking')
        self.assertClass(self.union([{'type': 'string'}, self.REF, NULL]), 'union_variant_inserted', 'breaking')
        self.assertClass(self.union([self.REF]), 'union_variant_removed', 'breaking')


class SecurityAndServerTests(DriftTestCase):
    def test_security_requirement_and_scheme(self):
        def mutate(spec):
            spec['paths']['/v1/channels']['get']['security'] = [{'apiKey': []}]
            spec['components']['securitySchemes']['apiKey'] = {'type': 'apiKey', 'in': 'header', 'name': 'x-key'}
            spec['components']['securitySchemes']['bearerAuth']['bearerFormat'] = 'opaque'
            return spec
        report = self.compare({NB: mutate})
        self.assertClass(report, 'security_changed', 'breaking', '/paths/~1v1~1channels/get/security')
        scheme = self.assertClass(report, 'security_scheme_changed', 'breaking',
                                  '/components/securitySchemes/bearerAuth/bearerFormat')
        self.assertIn('POST /v1/channels', scheme['affectedOperations'])

    def test_root_security_change_is_reported_per_operation(self):
        def mutate(spec):
            spec['security'] = [{'bearerAuth': ['read']}]
            return spec
        report = self.compare({MP: mutate})
        self.assertTrue(self.only(report, 'security_changed'))
        self.assertTrue(all(c['affectedOperations'] for c in self.only(report, 'security_changed')))

    def test_server_url_and_description(self):
        def url(spec):
            spec['servers'][0]['url'] = 'https://api.example.invalid/network-broker'
            return spec
        def description(spec):
            spec['servers'][0]['description'] = 'Now documented'
            return spec
        report = self.compare({NB: url})
        self.assertEqual({c['class'] for c in report['changes']}, {'server_changed'})
        self.assertEqual(len(report['changes']), rtm.COUNTS[NB])
        self.assertEqual({c['class'] for c in self.compare({NB: description}, name='b')['changes']}, {'annotation_changed'})


class SameVersionTests(DriftTestCase):
    def test_same_version_content_change_is_flagged_and_classified(self):
        def mutate(spec):
            spec['components']['schemas']['Channel']['properties']['name']['type'] = 'integer'
            return spec
        report = self.compare({NB: mutate})
        source = next(s for s in report['sources'] if s['upstreamPath'] == NB)
        self.assertTrue(source['sameVersionContentChange'])
        self.assertEqual(source['oldVersion'], source['newVersion'])
        self.assertEqual(source['status'], 'changed')
        self.assertEqual({c['class'] for c in report['changes']}, {'type_changed'})

    def test_whitespace_only_is_byte_only_with_exit_zero(self):
        candidate = self.candidate({NB: lambda spec: b'\n\n' + self.base[NB]['bytes']})
        report = drift.compare_candidate(candidate)
        self.assertEqual(report['status'], 'byte_only')
        self.assertEqual([c['class'] for c in report['changes']], ['byte_only'])
        self.assertTrue(next(s for s in report['sources'] if s['upstreamPath'] == NB)['sameVersionContentChange'])
        with contextlib.redirect_stdout(io.StringIO()), contextlib.redirect_stderr(io.StringIO()):
            self.assertEqual(drift.main(['compare', '--candidate', str(candidate)]), 0)

    def test_version_bump_only_is_a_label_change(self):
        def mutate(spec):
            spec['info']['version'] = '1.0.1'
            return spec
        candidate = self.candidate({NB: mutate})
        report = drift.compare_candidate(candidate)
        self.assertEqual([c['class'] for c in report['changes']], ['version_label_changed'])
        self.assertFalse(next(s for s in report['sources'] if s['upstreamPath'] == NB)['sameVersionContentChange'])
        self.assertEqual(report['status'], 'review_required')
        with contextlib.redirect_stdout(io.StringIO()), contextlib.redirect_stderr(io.StringIO()):
            self.assertEqual(drift.main(['compare', '--candidate', str(candidate)]), 2)


class UnknownChangeTests(DriftTestCase):
    CASES = {
        'webhooks': lambda spec: {**spec, 'webhooks': {'channelOnline': {'post': {'responses': {'200': {'description': 'ok'}}}}}},
        'not': lambda spec: (spec['components']['schemas']['Channel'].update({'not': {'required': ['uuid']}}), spec)[1],
        'extension': lambda spec: (spec['components']['schemas']['Channel'].update({'x-internal': True}), spec)[1],
        'discriminator': lambda spec: (spec['components']['schemas']['Channel'].update({'discriminator': {'propertyName': 'status'}}), spec)[1],
        'external': lambda spec: (spec['components']['schemas']['Channel']['properties'].update({'meta': {'$ref': 'https://example.invalid/meta.json'}}), spec)[1],
        'operation-extension': lambda spec: (spec['paths']['/v1/channels']['get'].update({'deprecated': True}), spec)[1],
    }

    def test_unknown_changes_raise_without_receipt(self):
        for name, mutate in self.CASES.items():
            with self.subTest(name=name):
                candidate = self.candidate({NB: mutate}, name=name)
                with self.assertRaisesRegex(ValueError, r'(?s)no receipt written.*identity=.*digest=[0-9a-f]{64}'):
                    drift.compare_candidate(candidate)
                receipt = self.output_root / f'{name}.json'
                stderr = io.StringIO()
                with contextlib.redirect_stdout(io.StringIO()), contextlib.redirect_stderr(stderr):
                    code = drift.main(['compare', '--candidate', str(candidate), '--output', str(receipt)])
                self.assertEqual(code, 1)
                self.assertIn('digest=', stderr.getvalue())
                self.assertFalse(receipt.exists())


class AddedContentFailsClosedTests(DriftTestCase):
    """Added structures are scanned with the same keyword sets the compare methods classify."""
    NEW_OPERATION = {'operationId': 'restartChannel', 'responses': {'204': {'description': 'ok'}}}
    CASES = {
        'component-extension': lambda spec: (spec['components']['schemas'].update(
            {'Region': {'type': 'string', 'x-internal': True}}), spec)[1],
        'property-discriminator': lambda spec: (spec['components']['schemas']['Channel']['properties'].update(
            {'region': {'type': 'object', 'discriminator': {'propertyName': 'kind'}}}), spec)[1],
        'nested-not-in-added-property': lambda spec: (spec['components']['schemas']['Channel']['properties'].update(
            {'region': {'type': 'object', 'properties': {'code': {'not': {'type': 'null'}}}}}), spec)[1],
        'operation-callbacks': lambda spec: (spec['paths'].update({'/v1/channels/{id}/restart': {'post': {
            **AddedContentFailsClosedTests.NEW_OPERATION, 'callbacks': {}}}}), spec)[1],
        'operation-deprecated': lambda spec: (spec['paths'].update({'/v1/channels/{id}/restart': {'post': {
            **AddedContentFailsClosedTests.NEW_OPERATION, 'deprecated': True}}}), spec)[1],
        'added-parameter-extension': lambda spec: (spec['paths']['/v1/channels']['get']['parameters'].append(
            {'in': 'query', 'name': 'region', 'schema': {'type': 'string'}, 'x-flag': 1}), spec)[1],
        'added-media-schema-not': lambda spec: (spec['paths']['/v1/channels']['get']['responses']['200']['content'].update(
            {'application/x-ndjson': {'schema': {'not': {'type': 'string'}}}}), spec)[1],
    }

    def test_unclassified_keywords_in_added_content_raise_without_receipt(self):
        for name, mutate in self.CASES.items():
            with self.subTest(name=name):
                with self.assertRaisesRegex(ValueError, r'(?s)no receipt written.*digest=[0-9a-f]{64}'):
                    drift.compare_candidate(self.candidate({NB: mutate}, name=name))

    def test_added_source_is_scanned_too(self):
        spec = {'openapi': '3.1.0', 'info': {'title': 'New', 'version': '1.0.0'},
                'paths': {'/v1/x': {'get': {'operationId': 'getX', 'responses': {'200': {'description': 'ok'}}}}},
                'components': {'schemas': {'X': {'type': 'object', 'not': {'required': ['a']}}}}}
        candidate = self.candidate(extra={'openapi-specs/prisma-airs/new/new-service.yaml': dumps(spec)})
        with self.assertRaisesRegex(ValueError, 'no receipt written'):
            drift.compare_candidate(candidate)

    def test_operation_added_identity_binds_the_whole_operation(self):
        def add(responses):
            def mutate(spec):
                spec['paths']['/v1/channels/{id}/restart'] = {'post': {'operationId': 'restartChannel', 'responses': responses}}
                return spec
            return mutate
        first = self.assertClass(self.compare({NB: add({'204': {'description': 'ok'}})}, name='a'), 'operation_added')
        second = self.assertClass(self.compare({NB: add({'202': {'description': 'accepted'}})}, name='b'), 'operation_added')
        self.assertNotEqual(first['id'], second['id'])
        self.assertIn('responses', first['new'])

    def test_removed_source_carries_native_bindings(self):
        source = self.assertClass(self.compare(drop=(SCAN,)), 'source_removed', 'breaking')
        self.assertTrue(source['nativeBindings'])
        self.assertTrue(all(isinstance(binding, dict) for binding in source['nativeBindings']))

    def test_single_in_place_union_branch_change_is_scoped(self):
        def mutate(spec):
            branches = audio_format(spec)['anyOf']
            index = next(i for i, branch in enumerate(branches) if branch != NULL)
            branches[index] = {**branches[index], 'maxLength': 64}
            return spec
        report = self.compare({MP: mutate})
        self.assertFalse(self.only(report, 'union_variant_removed'))
        self.assertFalse(self.only(report, 'union_variant_inserted'))
        change = self.assertClass(report, 'constraint_changed', 'breaking')
        self.assertTrue(change['pointer'].endswith('/maxLength'))


class CandidateTamperingTests(DriftTestCase):
    def expect(self, message, **options):
        with self.assertRaisesRegex(ValueError, message):
            drift.compare_candidate(self.candidate(**options))

    def test_tree_document_tampering(self):
        def first(tree):
            return next(t for t in tree['productTrees'] if t['entries'])
        def entry_hash(tree):
            first(tree)['entries'][0]['gitBlobSha1'] = '0' * 40
        def content(tree):
            tree['commitTreeSha'] = 'c' * 40
        def truncated(tree):
            first(tree)['truncated'] = True
        def symlink_mode(tree):
            first(tree)['entries'][0]['mode'] = '120000'
        def traversal(tree):
            first(tree)['entries'][0]['path'] = '../escape.yaml'
        def absolute(tree):
            first(tree)['entries'][0]['path'] = '/etc/passwd'
        def missing_tree(tree):
            tree['productTrees'].pop()
        for message, hook in (('tree hash does not match', entry_hash), ('contentSha256', content),
                              ('truncated', truncated), ('file mode', symlink_mode), ('Unsafe upstream path', traversal),
                              ('Unsafe upstream path', absolute), ('three AIRS product trees', missing_tree)):
            with self.subTest(message=message, hook=hook.__name__):
                self.expect(message, tree_hook=hook, name=hook.__name__)

    def test_file_set_and_byte_tampering(self):
        candidate = self.candidate(name='extra')
        (candidate / 'files' / 'openapi-specs/prisma-airs/extra.yaml').write_bytes(b'{}')
        with self.assertRaisesRegex(ValueError, 'extra='):
            drift.compare_candidate(candidate)
        candidate = self.candidate(name='missing')
        (candidate / 'files' / NB).unlink()
        with self.assertRaisesRegex(ValueError, 'missing='):
            drift.compare_candidate(candidate)
        candidate = self.candidate(name='bytes')
        (candidate / 'files' / NB).write_bytes(self.base[NB]['bytes'] + b'\n')
        with self.assertRaisesRegex(ValueError, 'bytes do not match'):
            drift.compare_candidate(candidate)
        candidate = self.candidate(name='symlink')
        target = candidate / 'files' / NB
        target.unlink()
        target.symlink_to(rtm.ROOT / self.base[NB]['row']['activeSpecificationPath'])
        with self.assertRaisesRegex(ValueError, 'Symlink'):
            drift.compare_candidate(candidate)
        candidate = self.candidate(name='top')
        (candidate / 'notes.txt').write_text('x')
        with self.assertRaisesRegex(ValueError, 'exactly tree.json'):
            drift.compare_candidate(candidate)

    def test_foreign_repository_and_pinned_commit_tampering(self):
        self.expect('Foreign candidate repository', repository='https://github.com/example/pan.dev', name='foreign')
        def mutate(spec):
            spec['info']['version'] = '9.9.9'
            return spec
        self.expect('Source tampering', mutations={NB: mutate}, commit=rtm.COMMIT, name='pinned')
        self.expect('Source tampering', drop=(SCAN,), commit=rtm.COMMIT, name='pinned-drop')
        self.assertEqual(drift.compare_candidate(self.candidate(commit=rtm.COMMIT, name='pinned-ok'))['status'], 'unchanged')
        self.expect('40 lowercase hex', commit='ABC', name='short')

    def test_non_specification_file_fails_classification(self):
        self.expect('non-specification', extra={'openapi-specs/prisma-airs/README.md': b'# readme\n'}, name='readme')
        self.expect('non-OpenAPI', extra={'openapi-specs/prisma-airs/data.json': b'{"a": 1}\n'}, name='data')


BASELINE_FILES = (
    'contracts/airs-active-sources.json', 'contracts/airs-drift.json', 'contracts/airs-operations.json',
    'contracts/airs-retained-capabilities.json', 'contracts/manifest.json',
    'contracts/compatibility/airs-0.2.2-retention.json', 'contracts/licenses/pan-dev-582e30543bf2.LICENSE',
    'contracts/vendor/red-team-management/0.86.0/source.json',
    'contracts/vendor/red-team-management/0.86.0/compatibility-decisions.json',
    'contracts/vendor/red-team-data-plane/0.93.0/source.json',
    'docs/research/2026-10-19-prisma-airs-openapi/inventory.json',
)


class BaselineTamperingTests(DriftTestCase):
    def copy_root(self):
        root = self.work / 'root'
        paths = list(BASELINE_FILES) + [row['activeSpecificationPath'] for row in
                                        rtm.load(drift.ACTIVE_SOURCES)['sources']]
        for relative in paths:
            (root / relative).parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(rtm.ROOT / relative, root / relative)
        return root

    def check_in(self, root):
        with patch.object(rtm, 'ROOT', root):
            return drift.check()

    def test_copied_baseline_verifies_then_each_tamper_fails(self):
        root = self.copy_root()
        self.check_in(root)
        cases = (
            (self.base[NB]['row']['activeSpecificationPath'], lambda b: b.replace(b'Channels', b'Channelz', 1), 'Pinned bytes changed'),
            ('contracts/vendor/red-team-management/0.86.0/source.json', lambda b: b.replace(b'"usedForGeneration": true', b'"usedForGeneration": false'), 'Pinned bytes changed|metadata mismatch'),
            ('contracts/vendor/red-team-management/0.86.0/compatibility-decisions.json', lambda b: b + b' ', 'Pinned bytes changed'),
            ('contracts/airs-drift.json', lambda b: b.replace(b'"classifierVersion": 2', b'"classifierVersion": 3'), 'Stale or tampered'),
            ('contracts/airs-active-sources.json', lambda b: b.replace(rtm.COMMIT.encode(), b'f' * 40, 1), 'Pinned bytes changed|snapshot commit'),
        )
        for relative, tamper, message in cases:
            with self.subTest(relative=relative):
                root = self.copy_root()
                path = root / relative
                path.write_bytes(tamper(path.read_bytes()))
                with self.assertRaisesRegex(ValueError, message):
                    self.check_in(root)
                shutil.rmtree(root)

    def test_missing_ledger_and_symlinked_baseline_fail(self):
        root = self.copy_root()
        (root / 'contracts/airs-drift.json').unlink()
        with self.assertRaisesRegex(ValueError, 'Missing drift ledger'):
            self.check_in(root)
        shutil.rmtree(root)
        root = self.copy_root()
        relative = self.base[NB]['row']['activeSpecificationPath']
        (root / relative).unlink()
        (root / relative).symlink_to(rtm.ROOT / relative)
        with self.assertRaisesRegex(ValueError, 'Unsafe pin path'):
            self.check_in(root)

    def test_cli_check_fails_closed(self):
        with patch.object(drift, 'current_ledger_text', return_value=None), \
                contextlib.redirect_stderr(io.StringIO()) as stderr:
            self.assertEqual(drift.main(['--check']), 1)
        self.assertIn('Missing drift ledger', stderr.getvalue())


class FakeGitHub:
    """Minimal GitHub git-data API and raw blob host, built from baseline bytes."""

    def __init__(self, files, commit, *, truncate=False, redirect=None, oversize=None, corrupt=None):
        self.files, self.commit = files, commit
        self.truncate, self.redirect, self.oversize, self.corrupt = truncate, redirect, oversize, corrupt
        self.requests = []
        self.routes = {}
        specs_entries = [{'path': 'other-product', 'mode': '040000', 'type': 'tree', 'sha': '1' * 40}]
        for product in drift.PRODUCT_DIRECTORIES:
            name = product.split('/', 1)[1]
            listing, entries = [], []
            directories = set()
            for upstream, data in sorted(files.items()):
                if upstream.startswith(product + '/'):
                    relative = upstream[len(product) + 1:]
                    parts = relative.split('/')
                    for i in range(1, len(parts)):
                        directories.add('/'.join(parts[:i]))
                    listing.append({'path': relative, 'mode': '100644', 'type': 'blob', 'sha': blob(data), 'size': len(data)})
                    entries.append({'path': relative, 'mode': '100644', 'gitBlobSha1': blob(data)})
                    self.routes[f'{drift.RAW_BASE}/{commit}/{upstream}'] = data
            for directory in sorted(directories):
                listing.append({'path': directory, 'mode': '040000', 'type': 'tree', 'sha': '2' * 40})
            sha = drift.git_tree_hash(entries)
            self.routes[f'{drift.GITHUB_API}/git/trees/{sha}?recursive=1'] = {'sha': sha, 'truncated': truncate, 'tree': listing}
            specs_entries.append({'path': name, 'mode': '040000', 'type': 'tree', 'sha': sha})
        specs_sha = drift.flat_tree_hash(specs_entries)
        self.routes[f'{drift.GITHUB_API}/git/trees/{specs_sha}'] = {'sha': specs_sha, 'truncated': False, 'tree': specs_entries}
        root_entries = [{'path': 'openapi-specs', 'mode': '040000', 'type': 'tree', 'sha': specs_sha},
                        {'path': 'README.md', 'mode': '100644', 'type': 'blob', 'sha': '3' * 40}]
        self.root_sha = drift.flat_tree_hash(root_entries)
        self.routes[f'{drift.GITHUB_API}/git/trees/{self.root_sha}'] = {'sha': self.root_sha, 'truncated': False, 'tree': root_entries}
        self.routes[f'{drift.GITHUB_API}/git/commits/{commit}'] = {'sha': commit, 'tree': {'sha': self.root_sha}}

    def __call__(self, url, headers, limit):
        self.requests.append((url, dict(headers)))
        if url not in self.routes:
            return 404, url, b''
        body = self.routes[url]
        body = body if isinstance(body, bytes) else json.dumps(body).encode()
        if self.redirect and self.redirect in url:
            return 200, url.replace('https://', 'https://evil.') , body
        if self.oversize and self.oversize in url:
            body = body + b' ' * (limit + 1 - len(body))
        if self.corrupt and self.corrupt in url:
            body = body + b'#'
        return 200, url, body[:limit + 1]


class RefreshTests(DriftTestCase):
    COMMIT = 'c0ffee' + '0' * 34
    TOKEN = 'ghp_' + 'S3cr3tT0k3n' * 3

    def refresh(self, fake=None, **options):
        fake = fake or FakeGitHub(self.files(), self.COMMIT)
        output = options.pop('output', self.output_root / self.COMMIT)
        return drift.refresh(options.pop('commit', self.COMMIT), output, fetch=fake, token=self.TOKEN, **options), fake

    def test_successful_refresh_is_verified_comparable_and_token_free(self):
        target, fake = self.refresh(clock=lambda: '2026-11-06T00:00:00+00:00')
        tree = json.loads((target / 'tree.json').read_text())
        self.assertEqual(tree['commit'], self.COMMIT)
        self.assertEqual(tree['commitTreeSha'], fake.root_sha)
        self.assertEqual(tree['observation']['observedAt'], '2026-11-06T00:00:00+00:00')
        report = drift.compare_candidate(target)
        self.assertEqual(report['status'], 'unchanged')
        self.assertEqual(report['candidate']['commit'], self.COMMIT)
        for path in target.rglob('*'):
            if path.is_file():
                self.assertNotIn(self.TOKEN.encode(), path.read_bytes())
        self.assertNotIn(self.TOKEN, rtm.encoded(report))
        for url, headers in fake.requests:
            self.assertEqual('Authorization' in headers, url.startswith('https://api.github.com/'))
        content_before = tree['contentSha256']
        tree['observation'] = {'method': 'edited observation'}
        (target / 'tree.json').write_text(rtm.encoded(tree))
        self.assertEqual(drift.verify_candidate(target)[1]['contentSha256'], content_before)

    def test_refreshed_drift_is_fetched_but_unknown_files_fail_comparison(self):
        files = self.files()
        files['openapi-specs/prisma-airs/README.md'] = b'# new upstream readme\n'
        target, _ = self.refresh(FakeGitHub(files, self.COMMIT))
        self.assertTrue((target / 'files/openapi-specs/prisma-airs/README.md').is_file())
        with self.assertRaisesRegex(ValueError, 'non-specification'):
            drift.compare_candidate(target)
        spec = self.spec(NB)
        spec['paths']['/v1/channels']['get']['parameters'][0]['required'] = True
        files = self.files()
        files[NB] = dumps(spec)
        target, _ = self.refresh(FakeGitHub(files, self.COMMIT), output=self.output_root / 'drifted')
        report = drift.compare_candidate(target)
        self.assertEqual(report['status'], 'review_required')
        self.assertEqual([c['class'] for c in report['changes']], ['requiredness_changed'])

    def test_refresh_failures_write_nothing(self):
        cases = (
            ('truncated', dict(truncate=True)),
            ('redirected', dict(redirect='raw.githubusercontent.com')),
            ('size limit', dict(oversize='raw.githubusercontent.com')),
            ('blob hash mismatch', dict(corrupt=NB)),
        )
        for message, options in cases:
            with self.subTest(message=message):
                output = self.output_root / message.replace(' ', '-')
                with self.assertRaisesRegex(ValueError, message):
                    self.refresh(FakeGitHub(self.files(), self.COMMIT, **options), output=output)
                self.assertFalse(output.exists())

    def test_refresh_rejects_bad_commit_existing_output_and_unguarded_paths(self):
        for commit in ('main', 'C0FFEE' + '0' * 34, 'a' * 39):
            with self.assertRaisesRegex(ValueError, '40-character'):
                drift.refresh(commit, self.output_root / 'x', fetch=lambda *a: self.fail('network used'))
        existing = self.output_root / 'existing'
        existing.mkdir(parents=True)
        with self.assertRaisesRegex(ValueError, 'existing refresh output'):
            drift.refresh(self.COMMIT, existing, fetch=lambda *a: self.fail('network used'))
        with self.assertRaisesRegex(ValueError, 'outside drift output roots'):
            drift.refresh(self.COMMIT, self.work / 'elsewhere', fetch=lambda *a: self.fail('network used'))
        fake = FakeGitHub(self.files(), self.COMMIT)
        fake.routes[f'{drift.GITHUB_API}/git/commits/{self.COMMIT}']['sha'] = 'd' * 40
        with self.assertRaisesRegex(ValueError, 'commit identity'):
            self.refresh(fake, output=self.output_root / 'identity')

    def test_refresh_rejects_unsupported_entries_and_incomplete_listings(self):
        fake = FakeGitHub(self.files(), self.COMMIT)
        key = next(k for k in fake.routes if k.endswith('?recursive=1') and fake.routes[k]['tree'])
        fake.routes[key]['tree'][0]['mode'] = '120000'
        with self.assertRaisesRegex(ValueError, 'Unsupported upstream entry'):
            self.refresh(fake, output=self.output_root / 'mode')
        fake = FakeGitHub(self.files(), self.COMMIT)
        key = next(k for k in fake.routes if k.endswith('?recursive=1') and fake.routes[k]['tree'])
        fake.routes[key]['tree'] = [e for e in fake.routes[key]['tree'] if e['type'] == 'tree'] or [{'path': 'x', 'mode': '040000', 'type': 'tree', 'sha': '2' * 40}]
        with self.assertRaisesRegex(ValueError, 'incomplete'):
            self.refresh(fake, output=self.output_root / 'incomplete')
        fake = FakeGitHub(self.files(), self.COMMIT)
        root = fake.routes[f'{drift.GITHUB_API}/git/trees/{fake.root_sha}']
        root['tree'].append({'path': 'injected', 'mode': '100644', 'type': 'blob', 'sha': '4' * 40})
        with self.assertRaisesRegex(ValueError, 'does not hash'):
            self.refresh(fake, output=self.output_root / 'listing')


class OfflineAndGuardTests(DriftTestCase):
    def test_check_and_compare_never_use_the_network_or_mutate_the_sdk(self):
        before = sdk_fingerprint()
        candidate = self.candidate({NB: lambda spec: (spec['info'].update(version='1.0.1'), spec)[1]})
        def refuse(*args, **kwargs):
            raise AssertionError('network access attempted')
        receipt = self.output_root / 'receipt.json'
        with patch.object(socket, 'socket', side_effect=refuse), \
                patch.object(socket, 'create_connection', side_effect=refuse), \
                patch.object(socket, 'getaddrinfo', side_effect=refuse), \
                contextlib.redirect_stdout(io.StringIO()), contextlib.redirect_stderr(io.StringIO()):
            self.assertEqual(drift.main(['--check']), 0)
            self.assertEqual(drift.main(['compare', '--candidate', str(candidate), '--output', str(receipt)]), 2)
        self.assertEqual(json.loads(receipt.read_text())['status'], 'review_required')
        self.assertEqual(sdk_fingerprint(), before)
        with contextlib.redirect_stdout(io.StringIO()), contextlib.redirect_stderr(io.StringIO()) as stderr:
            self.assertEqual(drift.main(['compare', '--candidate', str(candidate), '--output', str(receipt)]), 1)
        self.assertIn('Refusing to overwrite', stderr.getvalue())

    def test_output_guard(self):
        allowed = drift.guarded_output('docs/checks/airs-drift/receipt.json')
        self.assertEqual(allowed, rtm.ROOT / 'docs/checks/airs-drift/receipt.json')
        self.assertEqual(drift.guarded_output(self.output_root / 'r.json'), self.output_root / 'r.json')
        for path in ('contracts/airs-drift.json', 'docs/checks/airs-drift', '.local/airs-drift',
                     '.local/airs-drift/../escape.json', 'crates/scm-sdk/src/lib.rs', self.work / 'r.json',
                     'docs/checks/phase-25.json'):
            with self.subTest(path=str(path)), self.assertRaisesRegex(ValueError, 'Refusing write'):
                drift.guarded_output(path)
        self.output_root.mkdir(parents=True)
        (self.output_root / 'link').symlink_to(self.work)
        with self.assertRaisesRegex(ValueError, 'symlink'):
            drift.guarded_output(self.output_root / 'link' / 'r.json')
        (self.output_root / 'once.json').write_text('{}')
        with self.assertRaises(FileExistsError):
            drift.write_new(self.output_root / 'once.json', '{}')

    def test_urllib_is_only_imported_by_the_refresh_path(self):
        source = (rtm.ROOT / 'tools/airs_drift.py').read_text()
        header = source[:source.index('def urllib_fetcher')]
        self.assertNotIn('import urllib', header)
        self.assertNotIn('import socket', source)


class ChangeClassCoverageTests(DriftTestCase):
    def test_change_class_table_is_covered_by_classified_drift(self):
        def nb(spec):
            spec['info']['version'] = '1.0.1'
            spec['info']['description'] = 'Edited'
            spec['info']['license'] = {'name': 'Apache-2.0'}
            channels = spec['paths']['/v1/channels']
            get, post = channels['get'], channels['post']
            get['operationId'] = 'listAllChannels'
            get['parameters'] = [p for p in get['parameters'] if p['name'] != 'skip']
            get['parameters'][0]['required'] = True
            get['parameters'].append({'in': 'query', 'name': 'region', 'required': False, 'schema': {'type': 'string'}})
            get['requestBody'] = {'content': {'application/json': {'schema': {'type': 'object'}}}}
            get['responses']['404'] = {'$ref': '#/components/responses/NotFound'}
            get['responses']['200']['content']['text/plain'] = {'schema': {'type': 'string'}}
            get['security'] = []
            get['servers'] = [{'url': 'https://example.invalid'}]
            del post['requestBody']
            del post['responses']['403']
            del channels['post']['responses']['200']['content']['application/json']
            spec['paths']['/v1/channels/{id}/restart'] = {'post': {'operationId': 'restart', 'responses': {'204': {'description': 'ok'}}}}
            del spec['paths']['/v1/channels/stats']
            schemas = spec['components']['schemas']
            schemas['ChannelStatus']['enum'] = ['OFFLINE', 'ONLINE', 'PAUSED']
            schemas['Channel']['properties']['name']['type'] = 'integer'
            schemas['Channel']['properties']['name']['maxLength'] = 10
            schemas['Channel']['properties']['region'] = {'type': 'string'}
            del schemas['Channel']['properties']['added_by']
            schemas['CreateChannelRequest']['required'] = ['description']
            schemas['Region'] = {'type': 'string'}
            schemas['ChannelStatistics'] = schemas.pop('ChannelStats')
            del schemas['UpdateChannelRequest']
            spec['paths']['/v1/channels/{channelId}']['patch']['requestBody']['content']['application/json']['schema'] = {'type': 'object'}
            spec['components']['securitySchemes']['bearerAuth']['bearerFormat'] = 'opaque'
            return spec
        def mp(spec):
            props = spec['components']['schemas']['TargetMetadata']['properties']
            props['audio_format']['anyOf'] = [{'$ref': '#/components/schemas/AudioFormat'}, {'type': 'string'}, NULL]
            other = spec['components']['schemas']['TargetSchema']['properties']['target_type']
            other['anyOf'] = [NULL, other['anyOf'][0]]
            third = spec['components']['schemas']['TargetResponseSchema']['properties']['target_type']
            third['anyOf'] = [{'type': 'integer'}, *third['anyOf']]
            fourth = spec['components']['schemas']['TargetListItemSchema']['properties']['target_type']
            fourth['anyOf'] = fourth['anyOf'][:1]
            spec['components']['schemas']['ResponseMode']['enum'].reverse()
            return spec
        report = self.compare({NB: nb, MP: mp,
                               MGMT: lambda spec: b'\n\n' + self.base[MGMT]['bytes']},
                              drop=(SCAN,), extra={'openapi-specs/prisma-airs/new/new.yaml': dumps(
                                  {'openapi': '3.1.0', 'info': {'title': 'n', 'version': '1'}, 'paths': {}})})
        seen = {change['class'] for change in report['changes']}
        self.assertEqual(seen, set(drift.CHANGE_CLASSES) - {'unknown'})
        for change in report['changes']:
            self.assertIn(change['disposition'], drift.CHANGE_CLASSES[change['class']]['dispositions'])
        self.assertEqual(json.loads(drift.current_ledger_text())['changeClasses'], drift.CHANGE_CLASSES)
        self.assertEqual(drift.CHANGE_CLASSES['unknown']['dispositions'], ['fail_closed'])


if __name__ == '__main__':
    unittest.main()
