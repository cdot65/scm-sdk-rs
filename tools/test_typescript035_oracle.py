"""Exercise the real pinned Zod oracle in disposable copies, never original sources."""
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]
CORPUS = 'contracts/typescript-035-oracle.json'
SCRIPT = 'tools/typescript035-oracle.mts'
SOURCE = 'contracts/sources/typescript-0.35.0'


class TypeScript035OracleTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        for relative in (
            SCRIPT, CORPUS, 'tools/package-lock.json', 'tools/verify_typescript035_source.py',
            'tools/verify-contracts.py', 'contracts/manifest.json', 'contracts/typescript.json',
            'contracts/sources/typescript/git-tree.json',
        ):
            dest = self.root / relative
            dest.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(ROOT / relative, dest)
        shutil.copytree(ROOT / SOURCE, self.root / SOURCE)
        (self.root / 'tools/node_modules').symlink_to(ROOT / 'tools/node_modules', target_is_directory=True)

    def oracle(self, check=True):
        return subprocess.run(
            [str(ROOT / 'tools/node_modules/.bin/tsx'), str(self.root / SCRIPT)] + (['--check'] if check else []),
            cwd=self.root, capture_output=True, text=True, timeout=30,
        )

    def test_actual_zod_is_deterministic_and_isolated(self):
        before = {p.relative_to(self.root / SOURCE): p.read_bytes() for p in (self.root / SOURCE).rglob('*') if p.is_file()}
        original = (self.root / CORPUS).read_bytes()
        for check in (True, False, True):
            result = self.oracle(check)
            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertEqual((self.root / CORPUS).read_bytes(), original)
            self.assertFalse(list((self.root / '.local').glob('typescript035-oracle-*')))
        self.assertEqual(before, {p.relative_to(self.root / SOURCE): p.read_bytes() for p in (self.root / SOURCE).rglob('*') if p.is_file()})
        corpus = json.loads(original)
        self.assertEqual(len(corpus['roots']), 8)
        self.assertEqual(len({row['id'] for row in corpus['cases']}), len(corpus['cases']))
        by_id = {row['id']: row for row in corpus['cases']}
        self.assertEqual(by_id['SecurityProfileListResponseSchema:empty']['zod'], {'success': True, 'output': {'ai_profiles': []}})
        self.assertFalse(by_id['CreateSecurityProfileRequestSchema:policy-omitted']['zod']['success'])
        self.assertTrue(by_id['SecurityProfileSchema:policy-omitted']['zod']['success'])
        nested = [
            ('data-protection.data-leak-detection.mask-data-inline', 'string'),
            ('data-protection.source-code-detection', 'number'),
            ('model-protection.0.severity-by-confidence', 'number'),
            ('model-protection.0.toxic-category-list.0', 'number'),
            ('model-protection.0.toxic-category-list.0.severity-by-confidence.high', 'number'),
        ]
        for schema in ('ContentTypeConfigurationSchema', 'ModelConfigurationSchema'):
            self.assertTrue(by_id[schema + ':nested-valid-base']['zod']['success'])
            for field, wrong_type in nested:
                for label in ('null', wrong_type):
                    self.assertFalse(by_id[f'{schema}:nested-{field}-{label}']['zod']['success'])
        for label in ('null', 'number'):
            self.assertFalse(by_id[f'ModelConfigurationSchema:nested-latency.inline-timeout-action-{label}']['zod']['success'])
        fixture = by_id['SecurityProfileSchema:public-fixture']
        self.assertEqual(fixture['input']['dlp_tenant_id'], '100')
        self.assertEqual(fixture['zod']['output']['dlp_tenant_id'], '100')
        self.assertEqual(corpus['provenance']['oracleSha256'], hashlib.sha256((self.root / SCRIPT).read_bytes()).hexdigest())

    def test_tampered_source_is_rejected_before_execution(self):
        model = self.root / SOURCE / 'src/models/mgmt-security-profile.ts'
        model.write_text(model.read_text() + '\nthrow new Error("UPSTREAM_EXECUTED");\n')
        result = self.oracle()
        self.assertNotEqual(result.returncode, 0)
        self.assertNotIn('UPSTREAM_EXECUTED', result.stderr)
        self.assertFalse((self.root / '.local').exists())

    def test_tampered_manifest_is_rejected_before_execution(self):
        manifest = self.root / SOURCE / 'manifest.json'
        manifest.write_bytes(manifest.read_bytes() + b'\n')
        result = self.oracle()
        self.assertNotEqual(result.returncode, 0)
        self.assertFalse((self.root / '.local').exists())

    def test_stale_success_output_is_rejected_without_rewrite(self):
        corpus = json.loads((self.root / CORPUS).read_bytes())
        row = next(row for row in corpus['cases'] if row['id'] == 'SecurityProfileListResponseSchema:empty')
        row['zod']['output'] = {}
        (self.root / CORPUS).write_text(json.dumps(corpus, indent=2, ensure_ascii=False) + '\n')
        changed = (self.root / CORPUS).read_bytes()
        result = self.oracle()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('Stale TypeScript 0.35.0 oracle corpus', result.stderr)
        self.assertEqual((self.root / CORPUS).read_bytes(), changed)
        self.assertFalse(list((self.root / '.local').glob('typescript035-oracle-*')))

    def test_script_changes_invalidate_corpus_binding(self):
        script = self.root / SCRIPT
        script.write_bytes(script.read_bytes() + b'\n')
        result = self.oracle()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('Stale TypeScript 0.35.0 oracle corpus', result.stderr)


if __name__ == '__main__':
    unittest.main()
