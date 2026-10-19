"""Keep the current validation reference aligned with both vendor response oracles."""
import pathlib
import re
import unittest

ROOT = pathlib.Path(__file__).resolve().parents[1]


class ValidationDocumentationTests(unittest.TestCase):
    def test_removed_vendor_string_keywords_have_documented_response_policy(self):
        document = (ROOT / 'docs/VALIDATION.md').read_text()
        expected = {
            'enum': 'Accept unfamiliar string vocabulary.',
            'format': 'Do not enforce string formats in either response mode.',
            'minLength': 'Do not enforce vendor-only string minimum length in either response mode.',
            'maxLength': 'Do not enforce vendor-only string maximum length in either response mode.',
        }
        for path in ('tools/make-red-team-fixtures.mjs', 'tools/g25-oracle.mts'):
            with self.subTest(oracle=path):
                removed = set(re.findall(r'delete\s+(?:result|copy)\.(\w+)', (ROOT / path).read_text()))
                self.assertEqual(removed, set(expected))
                for keyword in removed:
                    self.assertIn(f'| `{keyword}` | {expected[keyword]} |', document)
        self.assertIn('[G25 compatibility](G25_COMPATIBILITY.md)', document)
        self.assertIn('unpaired operations retain those vendor structural and\nnumeric constraints', document)
        self.assertIn('selects the frozen TypeScript schema exclusively', document)
        self.assertNotIn('formats and bounds. Those specimens use strict component roots', document)


if __name__ == '__main__':
    unittest.main()
