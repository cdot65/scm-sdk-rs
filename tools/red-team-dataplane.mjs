#!/usr/bin/env node
// Development-only, independent YAML and JSON Schema verification.
import fs from 'node:fs';
import path from 'node:path';
import {fileURLToPath} from 'node:url';
import {createRequire} from 'node:module';
const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const require = createRequire(new URL('./package.json', import.meta.url));
const yaml = require('js-yaml');
const Ajv = require('ajv/dist/2020.js').default;
const addFormats = require('ajv-formats');
const dir = path.join(root, 'contracts/vendor/red-team-data-plane/0.93.0');
const pin = JSON.parse(fs.readFileSync(path.join(dir, 'source.json')));
const pkg = JSON.parse(fs.readFileSync(path.join(root, 'tools/package.json')));
const lock = JSON.parse(fs.readFileSync(path.join(root, 'tools/package-lock.json')));
if (pkg.dependencies['js-yaml'] !== pin.yamlParser.version ||
    lock.packages['node_modules/js-yaml'].version !== pin.yamlParser.version ||
    require('js-yaml/package.json').version !== pin.yamlParser.version)
  throw Error('YAML parser pin mismatch');
const spec = yaml.load(fs.readFileSync(path.join(root, pin.path), 'utf8'));
const canonical = JSON.stringify(spec, null, 2) + '\n';
const target = path.join(dir, 'openapi.json');
if (process.argv.includes('--check')) {
  if (fs.readFileSync(target, 'utf8') !== canonical) throw Error('Canonical vendor JSON drift');
} else fs.writeFileSync(target, canonical);
const ajv = new Ajv({strict: false, allErrors: true, validateFormats: true});
addFormats(ajv);
const id = 'urn:scm-sdk:red-team-dp:0.93.0';
ajv.addSchema({...spec, $id: id});
let cases = 0;
const fixtures = path.join(dir, 'model-fixtures.json');
if (fs.existsSync(fixtures)) {
  for (const row of JSON.parse(fs.readFileSync(fixtures)).cases) {
    const validate = ajv.getSchema(id + '#/components/schemas/' + row.schema);
    if (!validate) throw Error('Missing schema ' + row.schema);
    const valid = validate(row.value);
    if (valid !== row.valid) throw Error(row.name + ': ' + JSON.stringify(validate.errors));
    cases++;
  }
}
console.log(JSON.stringify({canonical: true, independentlyValidatedModelCases: cases}));
