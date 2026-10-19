#!/usr/bin/env python3
"""Render the G25 paired field ledger and reader report from the source audit.

Union positions are retained in descriptors. Grouping paths is navigation only;
structural equality/difference is not proof of equivalent accepted documents.
"""
import argparse
import collections
import hashlib
import json
import pathlib
import re

ROOT = pathlib.Path(__file__).resolve().parents[1]
AUDIT = ROOT / 'contracts/compatibility/g25/audit.json'
LEDGER = ROOT / 'contracts/compatibility/g25/comparison.json'
REPORT = ROOT / 'docs/G25_COMPATIBILITY.md'
CASES = ROOT / 'contracts/compatibility/g25/cases.json'
OUTCOMES = ROOT / 'contracts/compatibility/g25/outcomes.json'
PROJECTIONS = ROOT / 'contracts/compatibility/g25/projections.json'
MANAGEMENT_OVERLAY = ROOT / 'contracts/red-team-management-model-overlay.json'


def grouped(fields):
    result = collections.defaultdict(list)
    for field in fields:
        path = re.sub(r'\|(?:\d+|input|output|catch|left|right)', '', field['path'])
        result[path].append(field)
    return dict(sorted(result.items()))


def signature(fields):
    # Node IDs and diagnostic text differ even when the constraints agree.
    def clean(value):
        if isinstance(value, dict):
            return {k: clean(v) for k, v in value.items()
                    if k not in ('node', 'path', 'message', '$ref')}
        if isinstance(value, list):
            return [clean(v) for v in value]
        return value
    return sorted(json.dumps(clean(f), sort_keys=True) for f in fields)


def residual_strictness(corpus, oracle):
    """Count mutated documents, not pre-mutation vendor-valid witnesses."""
    if len(corpus['cases']) != len(oracle['outcomes']):
        raise ValueError('G25 cases/outcomes length mismatch')
    hits = []
    for index, (case, outcome) in enumerate(zip(corpus['cases'], oracle['outcomes'])):
        if outcome['strict'] is True and outcome['typescript'] is False and outcome['response'] is False:
            hits.append({'caseIndex': index, 'root': corpus['roots'][case['root']]['root'],
                         'path': case['path'], 'mutation': case['kind']})
    return {'caseCount': len(hits), 'operationCount': len({h['root'] for h in hits}),
            'mutationCounts': dict(sorted(collections.Counter(h['mutation'] for h in hits).items())),
            'cases': hits}


def ledger(audit, corpus=None, oracle=None):
    operations = []
    for op in audit['operations']:
        vendor, baseline = grouped(op['vendorFields']), grouped(op['baselineFields'])
        rows = []
        for path in sorted(vendor.keys() | baseline.keys()):
            v, b = vendor.get(path, []), baseline.get(path, [])
            if not op['baselineOperation']:
                classification = 'no-typescript-counterpart'
            elif op['baselineOpaque']:
                classification = 'typescript-opaque-contract'
            elif not b:
                classification = 'vendor-only-path'
            elif not v:
                classification = 'typescript-only-path'
            else:
                classification = ('same-local-description' if signature(v) == signature(b)
                                  else 'different-local-description')
            rows.append({'path': path, 'classification': classification,
                         'typescript': b, 'vendor': v})
        operations.append({k: op[k] for k in ('operationId', 'nativeClass', 'nativeMethod',
            'responseRoot', 'baselineOperation', 'baselineExpression', 'baselineOpaque')}
            | {'fields': rows, 'classificationCounts': dict(sorted(collections.Counter(
                r['classification'] for r in rows).items()))})
    return {'schemaVersion': 1, 'auditSha256': hashlib.sha256(AUDIT.read_bytes()).hexdigest(),
            'semantics': 'Grouped paths are navigation aids. Exact branch paths, requiredness, '
            'nullability, types, checks and unknown-field policies remain in each descriptor. '
            'Local descriptions do not establish whole-document semantic equivalence.',
            'operations': operations, **({'residualStrictness': residual_strictness(corpus, oracle),
             'casesSha256': hashlib.sha256(CASES.read_bytes()).hexdigest(),
             'outcomesSha256': hashlib.sha256(OUTCOMES.read_bytes()).hexdigest()}
             if corpus is not None and oracle is not None else {})}


def projection_usage(rows):
    groups = {key: {'fields': 0, 'models': set()} for key in
              ('request-only', 'request-and-response', 'response-only', 'standalone-schema')}
    for row in rows:
        group = groups[row['usage']]
        group['fields'] += 1
        group['models'].add(row['rustType'])
    return {key: {'fields': group['fields'], 'models': sorted(group['models'])}
            for key, group in groups.items()}


def management_projection_section(projections):
    """Document the separately pinned management projections without merging corpora."""
    groups = collections.defaultdict(list)
    seen = set()
    for row in projections:
        identity = (row['legacySchema'], row['field'])
        if identity in seen:
            raise ValueError('Duplicate management projection documentation binding')
        seen.add(identity)
        groups[(row['rustType'], row['legacySchema'])].append(row['field'])
    return [
        '### Management 0.86.0 projection addendum', '',
        'The separate management overlay adds the following optional typed projections. '
        'They use the same typed-over-raw precedence: a supplied typed value, including '
        'explicit null, supersedes a same-key extension. Leaving the typed field unset '
        'preserves the raw extension. Wrong-type values remain in extension storage; '
        'these conveniences do not narrow the frozen request or response validators.', '',
        'These pairings and generated Rust names are recorded in '
        '[the management overlay ledger](../contracts/red-team-management-model-overlay.json), '
        'whose generator checks baseline identities and completeness. They are additional '
        'to the 55 data-plane projection field occurrences above; the historical '
        '`projection-bindings.json` owns only that data-plane corpus.', '',
        '| Management projection model | Frozen schema | Added fields |',
        '| --- | --- | --- |',
        *[f'| `{model}` | `{schema}` | ' + ', '.join(f'`{field}`' for field in sorted(fields)) + ' |'
          for (model, schema), fields in sorted(groups.items())], '',
    ]


def render(audit, comparison, projections, management_projections=()):
    s = audit['summary']
    usage = projection_usage(projections)
    requests = sorted(set(usage['request-only']['models']) | set(usage['request-and-response']['models']))
    request_models = ', '.join(f'`{name}`' for name in requests) or 'none'
    standalone_models = ', '.join(f'`{name}`' for name in usage['standalone-schema']['models']) or 'none'
    residual = comparison['residualStrictness']
    examples = list(dict.fromkeys((h['root'], tuple(h['path'])) for h in residual['cases']))[:3]
    residual_examples = '; '.join(
        f"`{root.removeprefix('vendor:red-team-dp:0.93.0:').removesuffix(':response')}` at `"
        + '$' + ''.join(f'[{part}]' if isinstance(part, int) else '.' + part for part in path) + '`'
        for root, path in examples)
    lines = ['# G25 response compatibility — 0.2.2 policy and current projections', '',
        '<!-- Generated by python3 tools/g25_report.py; edit that source to update this report. -->', '',
        '## Scope and decision', '',
        f"The audit covers **{s['operations']} operations, {s['jsonResponseRoots']} JSON response roots, "
        f"and {s['matchedTypeScriptOperations']} TypeScript counterparts**. The baseline is TypeScript "
        f"0.34.0 at `{audit['baselineCommit']}`; the data-plane vendor contract is Red Team 0.93.0. "
        "The management projection addendum covers Red Team management 0.86.0.", '',
        'For existing operations, response validation uses the actual TypeScript operation schema, '
        'with the shared response policy below. Eleven operations have no TypeScript counterpart; '
        'ten have JSON responses and one returns CSV bytes. Their vendor structure remains authoritative. '
        'This is a compatibility correction, not a claim of live service acceptance.', '',
        '### Loosened response rules', '',
        '- Both `Mode::Response` and `Mode::ResponseWithoutDefaults` ignore string checks named '
        '`uuid`, `rfc3339`, `datetime`, `date`, `duration`, `email`, `url`, and `regex` across the SDK. '
        'Strings remain strings and their contents are preserved.',
        '- Vendor-only string length bounds are request-only. Existing TypeScript response bounds '
        'remain enforced; using the exact baseline schema also retains its requiredness, nullability '
        'and types instead of imposing stricter vendor fields.',
        '- Optional vendor-added typed projections preserve unsupported JSON in extension storage '
        'when the typed field cannot represent it. An explicitly set typed field supersedes that '
        'fallback. Baseline-field extension collisions remain errors.', '',
        'This typed-over-raw precedence also applies intentionally to every request-reachable '
        'projection model in the data-plane corpus: ' + request_models + '. If a caller supplies both a typed projection and '
        'an extension with the same wire key, the typed value is serialized and the raw extension '
        'is omitted. Callers must leave the typed projection unset to serialize a preserved raw value. '
        'Projection classifications are recorded in '
        '[projections.json](../contracts/compatibility/g25/projections.json). The reviewed legacy/vendor '
        'schema pairings are pinned separately in '
        '[projection-bindings.json](../contracts/compatibility/g25/projection-bindings.json) '
        'and generation fails if they change.', '',
        'Standalone schema projections are classified separately: ' + standalone_models + '. '
        'They are not labeled as public-operation request or response projections.', '',
        '| Projection reachability | Added fields | Models |',
        '| --- | ---: | --- |',
        *[f"| `{kind}` | {group['fields']} | " + ', '.join(f'`{name}`' for name in group['models']) + ' |'
          for kind, group in usage.items()], '',
        'Baseline decoding errors remain generic and redacted to avoid exposing service values '
        'through serde diagnostics. Projection decoding retains a value copy for lossless fallback; '
        'public-client responses remain subject to the existing body and validation resource limits.', '',
        *(management_projection_section(management_projections) if management_projections else []),
        '### Rules deliberately retained', '',
        '- Unknown response enum strings and unknown object keys were already supported. They remain supported.',
        '- String type checks, baseline presence/type/bounds checks, resource limits and finite JSON '
        'rules remain. Defaults still follow the selected response mode; this fix does not disable defaults.',
        '- For operations without a TypeScript counterpart, vendor numeric `minimum`/`maximum` and '
        'integer constraints, and array `minItems`/`maxItems`, remain enforced in both response modes. '
        'For example, `AsrResponseSchema.asr` must remain within 0–100. These retained constraints '
        'can still reject an unexpected service value. No TypeScript counterpart establishes '
        'broader numeric domains or structural cardinality for these operations; the accepted '
        'plan retains those constraints while relaxing only the scoped string compatibility policy.',
        '- Frozen refinements and transforms remain active. In particular, the `time_order` '
        'effect still parses `start_time` and `end_time` timestamp content in response modes '
        'and can reject unparseable timestamps or invalid ordering. Bypassing a string-format '
        'assertion does not bypass a separate semantic refinement.',
        '- Request schemas, caller-supplied UUID checks, safe path encoding and authentication/destination '
        'policies remain strict. See the full UUID inventory below.', '',
        'The independent frozen-Zod reproduction rejects all three disputed structural cases: '
        '**missing `category_display_name`, numeric `tsg_id`, and null `sub_category`**. '
        'They remain rejected. In contrast, opaque IDs such as `job-1`, `p-1`, and `map-1`, '
        'and timestamp strings containing a space or lacking an offset are accepted in responses.', '',
        '## Reading the comparison evidence', '',
        '[The generated field ledger](../contracts/compatibility/g25/comparison.json) contains every '
        'paired field for every operation. Each descriptor includes presence (`optional`), '
        'nullability, node kind, checks (including string min/max), and object unknown-field policy '
        'where present. Exact original branch paths and node IDs are retained. '
        'The [source audit](../contracts/compatibility/g25/audit.json) binds the underlying source hashes.', '',
        'Union branch positions are removed only from the grouping key. A nullable wrapper and '
        'a union containing `ZodNull` can describe the same accepted nulls despite different local '
        'descriptions. Likewise, branch alternatives can accept a document rejected by another '
        'branch. **The local comparison categories are not semantic equivalence claims.** '
        'The independent Zod/AJV outcome corpus and Rust regressions decide acceptance.', '',
        'Ledger categories distinguish matching local descriptions, differing local descriptions, '
        'vendor-only paths, TypeScript-only paths, opaque baseline contracts, and operations with '
        'no TypeScript counterpart. An absent baseline field in a passthrough object does not '
        'mean its value is rejected.', '',
        '### Deliberately retained strictness relative to the vendor', '',
        'The frozen TypeScript schema is selected exclusively for paired response operations; '
        'it is not unioned with the vendor schema. Consequently, some documents allowed by '
        'the vendor are rejected because TypeScript also rejects them. This is a deliberate '
        'compatibility boundary and a residual read-failure risk.', '',
        f"The generated mutation corpus contains **{residual['caseCount']} cases across "
        f"{residual['operationCount']} operations** for which the mutated document passes strict "
        'vendor validation but fails both frozen TypeScript and the selected response policy. '
        'These are corpus cases, including repeated branch witnesses, not unique service documents '
        'or observed service failures. The ledger lists their case indices, operation roots, paths '
        'and mutation kinds. Counts use the oracle outcome after mutation, not the valid starting witness.', '',
        *(['Examples: ' + residual_examples + '.', '',
           'Mutation counts: ' + ', '.join(f'`{kind}` {count}' for kind, count in residual['mutationCounts'].items()) + '.', '']
          if residual['caseCount'] else
          ['No residual strictness cases were found in the generated corpus; this is not proof that none can occur outside the corpus.', '']),
        'Format occurrences across the strict vendor response trees are '
        + ', '.join(f'`{kind}` {count}' for kind, count in sorted(s['formatOccurrences'].items())) + '; '

        f"{s['formatAffectedOperations']} operations are affected. Counts include repeated union "
        'branches, not only distinct fields. Strict request tests use the independent oracle: '
        'an alternative union branch can legitimately accept a mutation.', '',
        f"The shared relaxation reaches {s['allFrozenRootsWithFormats']} frozen roots with format "
        'checks. The source audit lists each root and format node, including management, IAM, '
        'Model Security, Gateway and request-named roots callable with response mode. This is '
        'an intentional response-policy difference from frozen TypeScript format refinements; '
        'it does not mean TypeScript ignores every format in those products.', '',
        '## Operation coverage', '',
        'Each row links the operation to its actual frozen response expression. Full vendor operation '
        'IDs, root names and field descriptors are in the ledger. Format counts include branch occurrences.', '',
        '| Rust operation | TypeScript response contract | Format occurrences | Compared paths |',
        '| --- | --- | ---: | ---: |']
    for op, comp in zip(audit['operations'], comparison['operations']):
        expression = op['baselineExpression'] or ('CSV bytes; no counterpart' if not op['responseRoot'] else 'No counterpart; vendor structure')
        lines.append(f"| `{op['nativeClass']}.{op['nativeMethod']}` | `{expression}` | {len(op['responseFormats'])} | {len(comp['fields'])} |")
    lines += ['', '## Query options', '',
        '`get_target_profile_error_logs` accepts only `limit` under the pinned vendor contract. '
        'Supplied `skip` and `search` remain errors; they are not silently dropped. There is no '
        'evidence from populated tenant error logs that establishes those options are applied by '
        'the service. CLI consumers adopting this release must make `--offset` and `--search` '
        'usage errors for this operation.', '',
        'All operation-level rejected legacy query options in the audit are listed below; '
        'this patch does not change them.', '', '| Rust operation | Rejected options |', '| --- | --- |']
    for op in audit['operations']:
        if op['rejectedQueryOptions']:
            lines.append(f"| `{op['nativeClass']}.{op['nativeMethod']}` | `{json.dumps(op['rejectedQueryOptions'], sort_keys=True)}` |")
    lines += ['', '## Retained caller UUID checks', '',
        f"All **{s['requestUuidOccurrences']}** `common::uuid` call sites are retained. "
        'The table includes methods beyond the 47 vendor operations and pagination helpers. '
        'Each listed argument has its own check. These are deliberately retained Rust request '
        'constraints; response IDs are no longer format-checked. TypeScript operation IDs use '
        'string parameters, so callers should not infer that every string accepted by TypeScript '
        'will pass Rust request validation.', '',
        '| Source module | Method | Checked arguments |', '| --- | --- | --- |']
    for source in audit['requestUuidInventory']:
        for method, args in source['methods'].items():
            lines.append(f"| `{pathlib.Path(source['source']).name}` | `{method}` | " + ', '.join(f'`{a}`' for a in args) + ' |')
    lines += ['', '## Reproduction and delivery boundaries', '',
        '- `tools/g25-repro` and `docs/checks/phase-18-reproduction.json` retain the pre-fix '
        'Rust/frozen-Zod reproduction, including the three intentionally rejected cases.',
        '- `tools/g25_fixtures.py` derives root, format, enum, structural and typed-projection '
        'cases; `tools/g25-oracle.mts` derives independent expected outcomes. '
        '`crates/scm-sdk/tests/g25_response_compatibility.rs` executes them in Rust.',
        '- The SDK transport regressions cover typed attacks and error logs. The reported '
        '17 airs-cli-rs loopback tests on worf have not been executed here; no consumer adoption is claimed.',
        '- Live qualification and publication have separate receipts in `docs/checks/` and '
        '`docs/phase-status.json`. This generated contract report alone establishes neither.', '',
        'Regenerate with `python3 tools/g25_audit.py` followed by `python3 tools/g25_report.py`; '
        'verify freshness with each command’s `--check` option.', '']
    return '\n'.join(lines)


def validate_corpus_binding(cases_bytes, oracle):
    if oracle.get('casesSha256') != hashlib.sha256(cases_bytes).hexdigest():
        raise ValueError('G25 oracle corpus digest mismatch')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    audit = json.loads(AUDIT.read_text())
    corpus = json.loads(CASES.read_text())
    oracle = json.loads(OUTCOMES.read_text())
    validate_corpus_binding(CASES.read_bytes(), oracle)
    comparison = ledger(audit, corpus, oracle)
    outputs = {LEDGER: json.dumps(comparison, indent=2, ensure_ascii=False) + '\n',
               REPORT: render(audit, comparison, json.loads(PROJECTIONS.read_text()),
                              json.loads(MANAGEMENT_OVERLAY.read_text())['modelOnlyProjections'])}
    stale = []
    for path, text in outputs.items():
        if args.check:
            if not path.exists() or path.read_text() != text:
                stale.append(str(path.relative_to(ROOT)))
        else:
            path.write_text(text)
    if stale:
        raise SystemExit('Stale G25 report output: ' + ', '.join(stale))
    print('G25 comparison report is current' if args.check else 'Generated G25 comparison report')


if __name__ == '__main__':
    main()
