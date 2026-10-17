#!/usr/bin/env python3
"""Extract the small model regression corpus from independently executed frozen Zod."""
import argparse
import hashlib
import json
import pathlib

ROOT = pathlib.Path(__file__).resolve().parents[1]
SOURCE = 'contracts/native-fixtures/schemas.json'
OUTPUT = 'crates/scm-sdk/src/fixtures/model-regressions.json'
NAMES = sorted([
    'DefaultTreeDetectionRuleSchema', 'MultiProfileDetectionRuleSchema',
    *['GatewayInferenceResponse' + event + 'EventSchema' for event in ('Completed','Created','Failed','Incomplete')],
    *[prefix + role + 'MessageSchema' for prefix in ('GatewayInferenceChatCompletionRequest','GatewayInferenceInputChatCompletionRequest')
      for role in ('System','Developer','User','Assistant','Tool','Function')],
])


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def generate():
    source = ROOT / SOURCE
    corpus = json.loads(source.read_text())
    inventory = ROOT / 'contracts/typescript.json'
    manifest = ROOT / 'contracts/manifest.json'
    if corpus['baseline'] != json.loads(inventory.read_text())['baseline']:
        raise ValueError('Model corpus frozen baseline differs')
    for key, path in [('inventorySha256', inventory), ('manifestSha256', manifest),
                      ('generatorSha256', ROOT / 'tools/schema-oracle.mts')]:
        if corpus['provenance'][key] != sha(path):
            raise ValueError('Stale independent Zod corpus provenance: ' + key)
    specimens = []
    for name in NAMES:
        matches = [(index, root) for index, root in enumerate(corpus['roots']) if name in root['names']]
        if len(matches) != 1:
            raise ValueError('Missing or duplicate regression schema: ' + name)
        index, root = matches[0]
        accepted = [(i, case) for i, case in enumerate(root['jsonCases']) if case['zod']['success'] is True]
        if not accepted:
            raise ValueError('No independent accepted JSON case: ' + name)
        case_index, case = accepted[0]
        specimens.append({'name': name, 'sid': root['sid'],
                          'sourcePointer': f'/roots/{index}/jsonCases/{case_index}/zod/output',
                          'output': case['zod']['output']})
    return {'schemaVersion': 1, 'baselineCommit': corpus['baseline']['commit'],
            'provenance': {'source': SOURCE, 'sha256': sha(source), 'oracle': 'tools/schema-oracle.mts',
                           'oracleSha256': corpus['provenance']['generatorSha256'],
                           'inventorySha256': sha(inventory), 'generatorSha256': sha(pathlib.Path(__file__)),
                           'attribution': 'Apache-2.0 frozen @cdot65/prisma-airs-sdk; see packaged NOTICE'},
            'specimens': specimens}


def verify_output(expected):
    output = ROOT / OUTPUT
    if output.is_symlink() or output.read_text() != expected:
        raise ValueError('Packaged model regression corpus differs from frozen Zod outputs')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    expected = json.dumps(generate(), indent=2, ensure_ascii=False, allow_nan=False) + '\n'
    output = ROOT / OUTPUT
    if args.check:
        verify_output(expected)
    else:
        output.write_text(expected)
    print(json.dumps({'checked': args.check, 'specimens': len(NAMES), 'bytes': len(expected.encode())}))


if __name__ == '__main__':
    main()
