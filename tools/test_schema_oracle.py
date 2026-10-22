"""Exercise the actual response adapter across distinct Zod module identities."""
import json
from pathlib import Path
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]


class SchemaOracleTests(unittest.TestCase):
    def test_discriminator_dispatch_survives_distinct_zod_constructors(self):
        source = (ROOT / 'tools/schema-oracle.mts').read_text()
        adapter = source[source.index('const responseMemo ='):source.index('function underlying(')]
        zod = ROOT / 'tools/node_modules/zod'
        script = (
            "import assert from 'node:assert/strict';\n"
            "import {createRequire} from 'node:module';\n"
            f"import {{z, type ZodTypeAny}} from {json.dumps((zod / 'index.js').as_uri())};\n"
            f"const other = createRequire(import.meta.url)({json.dumps(str(zod))}).z;\n"
            + adapter + """
const request = other.discriminatedUnion('rule_type', [
  other.object({rule_type: other.literal('a'), state: other.enum(['known'])}),
  other.object({rule_type: other.literal('b'), count: other.number()}),
]);
assert.notEqual(other.ZodLiteral, z.ZodLiteral);
assert.throws(() => z.discriminatedUnion('rule_type', request.options as any),
  /discriminator value/);
const response = responseSchema(request);
const input = {rule_type: 'a', state: 'future', extra: {keep: null}};
assert.deepEqual(response.parse(input), input);
assert.equal(request.safeParse(input).success, false);
assert.equal(response.safeParse({rule_type: 'future', state: 'known'}).success, false);
assert.equal(response.safeParse({state: 'known'}).success, false);
assert.equal(response.safeParse({rule_type: 'a'}).success, false);
assert.equal(response.safeParse({rule_type: 'a', state: 7}).success, false);
assert.equal(response.safeParse({rule_type: 'b', count: 'wrong'}).success, false);
assert.deepEqual(response.parse({rule_type: 'b', count: 1}), {rule_type: 'b', count: 1});
assert.deepEqual(request.parse({rule_type: 'a', state: 'known', extra: true}),
  {rule_type: 'a', state: 'known'});
"""
        )
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / 'adapter.mts'
            path.write_text(script)
            result = subprocess.run([str(ROOT / 'tools/node_modules/.bin/tsx'), str(path)],
                                    capture_output=True, text=True, timeout=30)
        self.assertEqual(result.returncode, 0, result.stderr)


if __name__ == '__main__':
    unittest.main()
