"""Additive source ownership and explicit version identity regressions."""
import copy
import importlib.util
import json
from pathlib import Path
import unittest
from unittest.mock import patch

import verify_typescript035_source as source

ROOT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location('contracts035', ROOT / 'tools/verify-contracts.py')
contracts = importlib.util.module_from_spec(spec)
spec.loader.exec_module(contracts)


class TypeScript035SourceTests(unittest.TestCase):
    def test_model_execution_imports_only_reviewed_zod(self):
        text = (source.SOURCE / 'src/models/mgmt-security-profile.ts').read_text()
        source.verify_model_imports(text)
        for changed in (text.replace("from 'zod'", "from 'node:fs'"),
                        text + "\nimport 'node:http';\n",
                        text + "\nimport {readFile} from 'node:fs';\n",
                        text + "\nawait import('node:http');\n",
                        text + "\nrequire('node:http');\n",
                        text + "\nexport {get} from 'node:http';\n",
                        text.replace("import { z } from 'zod';\n", '')):
            with self.assertRaisesRegex(ValueError, 'import only'):
                source.verify_model_imports(changed)

    def test_model_export_delta_is_derived_and_rejects_unreviewed_changes(self):
        inventory = json.loads((ROOT / 'contracts/typescript.json').read_text())
        text = (source.SOURCE / 'src/models/index.ts').read_text()
        changed = json.loads((source.SOURCE / 'manifest.json').read_text())['changedRuntimeSources']
        result = source.verify_model_export_delta(text, inventory, changed)
        self.assertEqual(result, {'baselineExports': 2254, 'currentExports': 2258,
                                 'added': ['ContentTypeConfiguration', 'ContentTypeConfigurationSchema',
                                           'ContentTypeConfigurations', 'ContentTypeConfigurationsSchema'],
                                 'removed': []})
        mutations = [text.replace('  ContentTypeConfigurationSchema,', ''),
                     text.replace('export { AiProfileSchema, type AiProfile }', 'export { type AiProfile }'),
                     text + "\nexport { FutureSchema } from './mgmt-security-profile.js';\n",
                     text + "\nexport { AiProfileSchema } from './ai-profile.js';\n",
                     text.replace('  ContentTypeConfigurationSchema,', '  Bogus as ContentTypeConfigurationSchema,'),
                     text.replace("export * from './ai-gateway.js';", "export * from './unknown.js';"),
                     text + '\nexport const Unsupported = 1;\n',
                     text.replace("export * from './ai-gateway.js';", '')]
        for mutation in mutations:
            self.assertNotEqual(mutation, text)
            with self.subTest(mutation=mutation[-100:]), self.assertRaises(ValueError):
                source.verify_model_export_delta(mutation, inventory, changed)
        with self.assertRaisesRegex(ValueError, 'Changed wildcard'):
            source.verify_model_export_delta(text, inventory, [*changed, 'src/models/ai-gateway.ts'])

    def test_new_declaration_spans_and_property_keys_are_pinned(self):
        text = (source.SOURCE / 'src/models/mgmt-security-profile.ts').read_text()
        expected = {
            'ContentTypeConfigurationSchema': ('data-protection', 'app-protection', 'model-protection', 'agent-protection'),
            'ContentTypeConfigurationsSchema': ('prompt', 'response', 'tool-call', 'tool-response'),
        }
        self.assertEqual(source.verify_new_profile_declarations(text), expected)
        self.assertEqual(source.verify_new_profile_declarations('// shifted\n' + text), expected)
        for name, (length, _) in source.NEW_PROFILE_DECLARATIONS.items():
            start = text.index('export const ' + name)
            expression_start = start + len('export const ')
            declaration = text[start:expression_start + length + 1]
            for changed in (text.replace(declaration, ''), text + '\n' + declaration,
                            text.replace(declaration, declaration.replace('.optional()', '.nullable()', 1)),
                            text.replace(declaration, declaration.replace('.object({', '.object({\n    future: z.string().optional(),', 1))):
                with self.subTest(name=name), self.assertRaisesRegex(ValueError, 'New profile declaration'):
                    source.verify_new_profile_declarations(changed)

    def test_pinned_versions_are_explicitly_consistent(self):
        manifest = source.verify()
        package = json.loads((source.SOURCE / 'package.json').read_text())
        constants = (source.SOURCE / 'src/constants.ts').read_text()
        source.verify_version_identity(manifest, package, constants)
        for changed in (dict(package, version='0.34.0'), {k: v for k, v in package.items() if k != 'version'}):
            with self.subTest(package=changed.get('version')), self.assertRaisesRegex(ValueError, 'versions'):
                source.verify_version_identity(manifest, changed, constants)
        for changed in (constants.replace("SDK_VERSION = '0.35.0'", "SDK_VERSION = '0.34.0'"),
                        '', constants + "\nexport const SDK_VERSION = '0.35.0';\n"):
            with self.subTest(constants=changed[-70:]), self.assertRaisesRegex(ValueError, 'versions'):
                source.verify_version_identity(manifest, package, changed)

    def test_reused_protection_declaration_closure_and_tampering(self):
        inventory = json.loads((ROOT / 'contracts/typescript.json').read_text())
        text = (source.SOURCE / 'src/models/mgmt-security-profile.ts').read_text()
        checked = source.verify_reused_profile_declarations(text, inventory)
        self.assertEqual(set(checked), {
            'DataProtectionSchema', 'DataLeakDetectionMemberSchema', 'DatabaseSecurityItemSchema',
            'SourceCodeDetectionSchema', 'PolicyAppProtectionSchema', 'UrlCategorySchema',
            'MaliciousCodeProtectionSchema', 'ModelProtectionItemSchema', 'SeverityByConfidenceSchema',
            'ToxicCategorySchema', 'TopicArraySchema', 'TopicObjectSchema', 'AgentProtectionItemSchema',
            'PolicyLatencySchema', 'DlpDataProfilePolicySchema', 'DlpRuleSchema',
        })
        for name in checked:
            row = next(row for row in inventory['exports'] if row['name'] == name)
            declaration = row['declarations'][0]['text']
            with self.subTest(name=name):
                for changed in (text.replace(declaration, declaration.replace('z\n', 'z.nullable()\n', 1)),
                                text.replace(declaration + ';', declaration + '.optional();'),
                                text + '\nexport const ' + declaration + ';\n',
                                text.replace('export const ' + declaration + ';', '')):
                    with self.assertRaisesRegex(ValueError, 'Reused profile declaration'):
                        source.verify_reused_profile_declarations(changed, inventory)
        modified = copy.deepcopy(inventory)
        row = next(row for row in modified['exports'] if row['name'] == 'DataProtectionSchema')
        row['declarations'][0]['sha256'] = '0' * 64
        with self.assertRaisesRegex(ValueError, 'declaration hash'):
            source.verify_reused_profile_declarations(text, modified)
        shifted = '\n// shifted source lines\n' + text
        self.assertEqual(source.verify_reused_profile_declarations(shifted, inventory), checked)

    def test_owner_returns_only_verified_inventory(self):
        paths = contracts.supplemental_source_paths()
        actual = {p.relative_to(ROOT).as_posix() for p in source.SOURCE.rglob('*') if p.is_file()}
        self.assertEqual(paths, actual)
        self.assertEqual(len(paths), 10)
        self.assertNotIn('contracts/sources/other/unowned.json', paths)

    def test_dedicated_verifier_failure_is_not_ignored(self):
        with patch.object(contracts.importlib.util, 'module_from_spec') as create:
            create.return_value.verify.side_effect = ValueError('Untrusted supplemental source')
            with patch.object(contracts.importlib.util, 'spec_from_file_location') as spec_factory:
                spec_factory.return_value.loader.exec_module.return_value = None
                with self.assertRaisesRegex(ValueError, 'Untrusted supplemental source'):
                    contracts.supplemental_source_paths()


if __name__ == '__main__':
    unittest.main()
