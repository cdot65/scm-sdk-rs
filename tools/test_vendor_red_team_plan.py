"""Verify the planning reference without implementing SDK behavior."""
import hashlib
import json
import pathlib
import subprocess
import unittest

ROOT = pathlib.Path(__file__).resolve().parents[1]
PIN = ROOT / "contracts/vendor/red-team-data-plane/0.93.0/source.json"
EXPECTED_SHA = "77f015de78f2dfe9eee944601b94b541c8f111da35f847e40a36084829451ca1"


def verify_bytes(pin, data):
    if len(data) != pin["bytes"] or hashlib.sha256(data).hexdigest() != pin["sha256"]:
        raise ValueError("Pinned vendor source differs")


class VendorPlanReference(unittest.TestCase):
    def test_pin_matches_preserved_bytes_and_exact_revision(self):
        pin = json.loads(PIN.read_text())
        self.assertEqual(pin["commit"], "afef9d08030a56e18a04ea078f589313f1fa2b0c")
        self.assertEqual(pin["sha256"], EXPECTED_SHA)
        self.assertEqual(pin["license"], "MIT")
        self.assertIn(pin["commit"], pin["sourceUrl"])
        path = ROOT / pin["path"]
        self.assertFalse(path.is_symlink())
        verify_bytes(pin, path.read_bytes())
        with self.assertRaises(ValueError):
            verify_bytes(pin, path.read_bytes() + b"\n")

    def test_pinned_parser_recounts_operations_and_schema_version(self):
        script = """
const fs=require('fs'), yaml=require('./tools/node_modules/js-yaml');
const pin=JSON.parse(fs.readFileSync('contracts/vendor/red-team-data-plane/0.93.0/source.json'));
if(require('./tools/node_modules/js-yaml/package.json').version!==pin.yamlParser.version)
  throw Error('Unreviewed YAML parser');
const s=yaml.load(fs.readFileSync(pin.path,'utf8'));
const old=yaml.load(fs.readFileSync('contracts/sources/pan-dev/openapi-specs/prisma-airs-redteam/data-plane/dp-openapi.yaml','utf8'));
const ops=x=>Object.entries(x.paths).flatMap(([p,ms])=>Object.entries(ms)
  .filter(([m])=>['get','post','put','patch','delete','head','options','trace'].includes(m))
  .map(([m])=>m.toUpperCase()+' '+p));
const all=ops(s),previous=ops(old);
process.stdout.write(JSON.stringify({version:s.info.version,openapi:s.openapi,
  operations:all.length,previous:previous.length,added:all.filter(x=>!previous.includes(x)).length,
  removed:previous.filter(x=>!all.includes(x)),parser:pin.yamlParser.version}));
"""
        result = json.loads(subprocess.check_output(["node", "-e", script], cwd=ROOT, text=True))
        self.assertEqual(result, {"version": "0.93.0", "openapi": "3.1.0", "operations": 47,
                                  "previous": 36, "added": 11, "removed": [], "parser": "4.3.2"})


if __name__ == "__main__":
    unittest.main()
