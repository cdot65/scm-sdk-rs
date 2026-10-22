"""Focused checks for the separately authorized directional-profile maintenance."""
import copy
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest
from unittest.mock import patch
import review_phase
from check_phase import commands_for, checkpoint_bindings, verify_checkpoint_bindings
from verify_typescript035_source import SOURCE, baseline_ledger, verify, verify_zod_versions, verify_license

ROOT = Path(__file__).resolve().parents[1]

class DirectionalGovernanceTests(unittest.TestCase):
    def setUp(self):
        self.status = json.loads((ROOT / 'docs/phase-status.json').read_text())

    def test_phase26_driver_freezes_sdk_independently_of_status(self):
        commands = commands_for(26)
        self.assertIn(['git', 'diff', '--exit-code', '578f2e6', '--', 'crates/scm-sdk'], commands)
        self.assertIn(['git', 'diff', '--check', '578f2e6..HEAD', '--', '.', ':(exclude)contracts/sources/typescript-0.35.0/*'], commands)
        guards = [c for c in commands if c[:2] == ['python', '-c']]
        self.assertTrue(any("'--others', '--exclude-standard', '--', 'crates/scm-sdk'" in c[2] for c in guards))

    def test_phase27_retains_governance_regressions(self):
        self.assertIn(['python', '-m', 'unittest', 'discover', '-s', 'tools',
                       '-p', 'test_typescript035_governance.py', '-v'], commands_for(27))

    def test_private_receipt_binding_detects_post_check_tamper(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            relative = 'docs/checks/phase-26-private-material.json'
            path = root / relative
            path.parent.mkdir(parents=True)
            path.write_text('{"passed":true}')
            command = ['python', 'scripts/check-private-material.py', '--check', relative]
            rows = [{'command': command, 'exit_code': 0, 'checkpoint_bindings': checkpoint_bindings(command, root)}]
            self.assertTrue(rows[0]['checkpoint_bindings'])
            verify_checkpoint_bindings(rows, root)
            report = root / 'docs/checks/phase-26.json'
            report.write_text(json.dumps({'phase': 26, 'passed': True,
                'source_sha256': 'current', 'source_unchanged': True, 'checks': rows}))
            with patch('check_phase.commands_for', return_value=[command]), patch('review_phase.fingerprint', return_value='current'):
                review_phase.check_evidence(root, 26, 'current', report)
            path.write_text('{"passed":false}')
            with self.assertRaisesRegex(ValueError, 'Checkpoint evidence changed'):
                verify_checkpoint_bindings(rows, root)
            with patch('check_phase.commands_for', return_value=[command]), patch('review_phase.fingerprint', return_value='current'):
                with self.assertRaisesRegex(ValueError, 'Checkpoint evidence changed'):
                    review_phase.check_evidence(root, 26, 'current', report)

    def test_license_identity_and_digest_are_enforced(self):
        manifest = verify()
        baseline = json.loads((ROOT / 'contracts/manifest.json').read_text())
        verify_license(manifest, baseline)
        for mutation in ('identity', 'digest', 'baseline'):
            changed, old = copy.deepcopy(manifest), copy.deepcopy(baseline)
            if mutation == 'identity':
                changed['license']['spdx'] = 'MIT'
            elif mutation == 'digest':
                next(row for row in changed['files'] if row['upstreamPath'] == 'LICENSE')['sha256'] = '0' * 64
            else:
                next(row for row in old['licenses'] if row['id'] == 'typescript')['spdx'] = 'MIT'
            with self.assertRaisesRegex(ValueError, 'license differs'):
                verify_license(changed, old)

    def test_baseline_ledger_tamper_and_oracle_dependency(self):
        baseline_ledger()
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            for name in ('contracts/manifest.json', 'contracts/sources/typescript/git-tree.json'):
                path = root / name
                path.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(ROOT / name, path)
            path = root / 'contracts/sources/typescript/git-tree.json'
            path.write_bytes(path.read_bytes() + b' ')
            with self.assertRaisesRegex(ValueError, 'baseline tree ledger'):
                baseline_ledger(root)
        package = json.loads((SOURCE / 'package.json').read_text())
        lock = json.loads((ROOT / 'tools/package-lock.json').read_text())
        verify_zod_versions(package, lock)
        for version in ('3.23.0', '4.0.0'):
            lock['packages']['node_modules/zod']['version'] = version
            with self.assertRaises(ValueError):
                verify_zod_versions(package, lock)

    def test_source_and_manifest_tampering_fail(self):
        verify()
        for target in ['manifest.json', 'src/models/mgmt-security-profile.ts', 'commit.txt', 'git-tree.json']:
            with tempfile.TemporaryDirectory() as tmp:
                source = Path(tmp) / 'source'
                shutil.copytree(SOURCE, source)
                path = source / target
                path.write_bytes(path.read_bytes() + b' ')
                with self.assertRaises(ValueError):
                    verify(source)

    def test_self_consistent_manifest_cannot_forge_git_origin(self):
        for target in ('commit.txt', 'git-tree.json', 'blob', 'delta'):
            with tempfile.TemporaryDirectory() as tmp:
                source = Path(tmp) / 'source'
                shutil.copytree(SOURCE, source)
                path = source / 'manifest.json'
                manifest = json.loads(path.read_text())
                if target in ('commit.txt', 'git-tree.json'):
                    proof = source / target
                    if target == 'commit.txt':
                        proof.write_bytes(proof.read_bytes() + b'changed')
                    else:
                        tree = json.loads(proof.read_text())
                        tree['files'].pop()
                        proof.write_text(json.dumps(tree))
                    next(r for r in manifest['proofFiles'] if r['path'] == target)['sha256'] = hashlib.sha256(proof.read_bytes()).hexdigest()
                elif target == 'blob':
                    manifest['files'][0]['gitBlobSha1'] = '0' * 40
                else:
                    manifest['changedRuntimeSources'] = []
                path.write_text(json.dumps(manifest))
                with patch('verify_typescript035_source.MANIFEST_SHA256', hashlib.sha256(path.read_bytes()).hexdigest()):
                    with self.assertRaises(ValueError):
                        verify(source)

    def test_added_file_and_symlink_fail(self):
        with tempfile.TemporaryDirectory() as tmp:
            source = Path(tmp) / 'source'
            shutil.copytree(SOURCE, source)
            extra = source / 'extra'
            extra.write_text('unexpected')
            with self.assertRaises(ValueError):
                verify(source)
            extra.unlink()
            target = source / 'LICENSE'
            saved = Path(tmp) / 'license'
            target.rename(saved)
            target.symlink_to(saved)
            with self.assertRaises(ValueError):
                verify(source)

    def test_independent_dependency_checks_and_original_gate_remain(self):
        # Recreate the planning boundary even after phase 26 has been accepted.
        planning = copy.deepcopy(self.status)
        phase26 = next(p for p in planning['phases'] if p['id'] == 26)
        phase26.update(status='in_progress', reviews=[])
        with patch.object(review_phase, 'verify_record', return_value=True):
            review_phase.validate_predecessors(ROOT, self.status, 26)
            with self.assertRaisesRegex(ValueError, 'Earlier phases'):
                review_phase.validate_predecessors(ROOT, self.status, 25)
            with self.assertRaisesRegex(ValueError, 'Maintenance predecessor'):
                review_phase.validate_predecessors(ROOT, planning, 27)
        with patch.object(review_phase, 'verify_record', return_value=False):
            with self.assertRaises(ValueError):
                review_phase.validate_predecessors(ROOT, self.status, 26)
        changed = copy.deepcopy(self.status)
        next(p for p in changed['phases'] if p['id'] == 26)['depends_on'] = []
        with self.assertRaisesRegex(ValueError, 'dependency declaration'):
            review_phase.validate_predecessors(ROOT, changed, 26)

    def test_real_predecessor_evidence_and_publication_tamper(self):
        review_phase.validate_predecessors(ROOT, self.status, 26)
        original = json.loads((ROOT / 'docs/checks/release-registry-0.2.3.json').read_text())
        for field in ('version', 'sourceCommit', 'archiveSha256'):
            with tempfile.TemporaryDirectory() as tmp:
                root = Path(tmp)
                path = root / 'docs/checks/release-registry-0.2.3.json'
                path.parent.mkdir(parents=True)
                changed = dict(original, **{field: 'invalid'})
                path.write_text(json.dumps(changed))
                with self.assertRaisesRegex(ValueError, 'Maintenance baseline publication evidence changed'):
                    review_phase.validate_predecessors(root, self.status, 26)

    def test_pinned_directory_symlink_rejected(self):
        with tempfile.TemporaryDirectory() as tmp:
            source = Path(tmp) / 'source'
            shutil.copytree(SOURCE, source)
            saved = Path(tmp) / 'src'
            (source / 'src').rename(saved)
            (source / 'src').symlink_to(saved, target_is_directory=True)
            with self.assertRaises(ValueError):
                verify(source)

    def test_frozen_contract_bytes(self):
        paths = ['contracts/typescript.json', 'contracts/operations.json',
                 'contracts/export-classification.json', 'contracts/manifest.json',
                 'contracts/native-union-variants.json',
                 'contracts/sources/typescript/git-tree.json', 'contracts/sources/typescript/commit.txt']
        for path in paths:
            expected = subprocess.check_output(['git', 'show', '578f2e6:' + path], cwd=ROOT)
            self.assertEqual((ROOT / path).read_bytes(), expected, path)
    def test_planning_sdk_bytes(self):
        # Implementation intentionally adds SDK files and refreshes precisely
        # validated fixture provenance; its separate preservation tests own that.
        if self.status['active_phase'] != 26:
            self.skipTest('Phase 26 planning boundary only')
        paths = ['contracts/native-fixtures/schemas.json',
                 'crates/scm-sdk/src/fixtures/model-regressions.json']
        sdk = subprocess.check_output(['git', 'ls-tree', '-r', '--name-only', '578f2e6', '--', 'crates/scm-sdk'], cwd=ROOT).decode().splitlines()
        actual = subprocess.check_output(['git', 'ls-files', '--cached', '--others', '--exclude-standard', '--', 'crates/scm-sdk'], cwd=ROOT).decode().splitlines()
        self.assertEqual(set(actual), set(sdk))
        paths += sdk
        self.assertEqual(subprocess.check_output(['git', 'ls-files', '--others', '--exclude-standard', '--', 'crates/scm-sdk'], cwd=ROOT), b'')
        for path in paths:
            expected = subprocess.check_output(['git', 'show', '578f2e6:' + path], cwd=ROOT)
            self.assertEqual((ROOT / path).read_bytes(), expected, path)

    def test_old_phase_records_preserved_and_new_chunks_bound(self):
        old = json.loads(subprocess.check_output(['git', 'show', '578f2e6:docs/phase-status.json'], cwd=ROOT))
        self.assertEqual(old['phases'], [p for p in self.status['phases'] if p['id'] not in {26, 27, 28}])
        for phase in [26, 27, 28]:
            row = next(p for p in self.status['phases'] if p['id'] == phase)
            self.assertTrue(row['scope_evidence'])
            self.assertIn(phase, review_phase.SCOPE_BOUND_PHASES)
            self.assertEqual(row['plan'], 'docs/typescript-035-parity-plan.md')
            for chunk in row['chunks']:
                self.assertTrue(review_phase.declared_chunk_scope(ROOT, phase, chunk['id'], 1))
        rows = {p['id']: copy.deepcopy(p) for p in self.status['phases']}
        # Exercise progression independently of this checkout's completed phases.
        rows[27]['status'] = rows[28]['status'] = 'pending'
        self.assertEqual(review_phase.next_phase(rows, 26), 27)
        self.assertEqual(review_phase.next_phase(rows, 27), 28)
        rows[27]['status'] = 'complete'
        self.assertEqual(review_phase.next_phase(rows, 26), 28)
        self.assertIsNone(review_phase.next_phase(rows, 25))
        self.assertIsNone(review_phase.next_phase(rows, 28))

if __name__ == '__main__':
    unittest.main()
