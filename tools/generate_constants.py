#!/usr/bin/env python3
"""Generate typed public constants from the frozen owned export inventory."""
import argparse
import json
from pathlib import Path
import re
import subprocess

ROOT=Path(__file__).resolve().parents[1]
SOURCE=ROOT/"contracts/typescript.json"
OUTPUT=ROOT/"crates/scm-sdk/src/constants.rs"
COMMIT="4a80dbdb3dc14365e1c25d838ac0bd139485d48c"

def quoted(value):return json.dumps(value,ensure_ascii=False)
def variant(value):return "".join(part[0].upper()+part[1:] for part in re.split(r"[^a-zA-Z0-9]+",value) if part)

def generate():
    inventory=json.loads(SOURCE.read_text())
    if inventory["baseline"]["commit"]!=COMMIT:raise ValueError("Unexpected constant baseline")
    rows=[row for row in inventory["exports"] if row["kind"]=="constant"]
    if len(rows)!=163:raise ValueError("Frozen public constant count changed")
    lines=['//! Typed constants from the frozen TypeScript baseline. Regenerate with tools/generate_constants.py.',
           'use serde::{Serialize,Deserialize};',
           'pub const BASELINE_SDK_VERSION: &str = "0.34.0";']
    secret=next(row["value"] for row in rows if row["name"]=="AI_GATEWAY_SECRET_FIELDS")
    lines+=['#[derive(Clone,Copy,PartialEq,Eq,Serialize,Deserialize)]','#[non_exhaustive]','pub enum AIGatewaySecretOperation {']
    for name in secret:lines+=[f'#[serde(rename={quoted(name)})] {variant(name)},']
    lines+=['}','impl AIGatewaySecretOperation { pub fn as_str(self)-> &\'static str { match self {']
    for name in secret:lines+=[f'Self::{variant(name)} => {quoted(name)},']
    lines+=['}} pub fn parse(value:&str)->crate::Result<Self>{match value {']
    for name in secret:lines+=[f'{quoted(name)} => Ok(Self::{variant(name)}),']
    lines+=['_ => Err(crate::Error::new(crate::ErrorKind::Validation)),}}}',
            '#[derive(Clone,Copy,Default,PartialEq,Eq,Serialize,Deserialize)]',
            '#[serde(rename_all="lowercase")] pub enum GatewaySecretDirection { #[default] Request, Response }',
            'pub type GatewaySecretPathSegment = String;',
            '#[derive(Clone,Copy,PartialEq,Eq,Serialize)] #[serde(rename_all="lowercase")] pub enum GatewaySecretRedaction {Value,Subtree}',
            '#[derive(Clone,Copy,Serialize)] pub struct GatewaySecretRule {pub path: &\'static [&\'static str], pub redact: GatewaySecretRedaction, #[serde(rename="oneTime",skip_serializing_if="Option::is_none")] pub one_time: Option<bool>}',
            '#[derive(Clone,Copy,Serialize)] pub struct GatewaySecretRules {pub request: &\'static [GatewaySecretRule],pub response: &\'static [GatewaySecretRule]}',
            'impl GatewaySecretRules {pub fn direction(&self,direction:GatewaySecretDirection)-> &\'static [GatewaySecretRule] {match direction{GatewaySecretDirection::Request=>self.request,GatewaySecretDirection::Response=>self.response}}}',
            'crate::redaction::redacted_debug!(AIGatewaySecretOperation,GatewaySecretDirection,GatewaySecretRedaction,GatewaySecretRule,GatewaySecretRules);']
    for row in rows:
        name=row["name"]
        if name=="globalConfiguration":continue
        if name=="SDK_VERSION":lines+=['pub const SDK_VERSION: &str=env!("CARGO_PKG_VERSION");'];continue
        if name=="USER_AGENT":lines+=['pub const USER_AGENT: &str=concat!("PAN-AIRS/",env!("CARGO_PKG_VERSION"),"-rust-sdk");'];continue
        value=row["value"]
        if name=="AI_GATEWAY_SECRET_FIELDS":
            lines+=['pub const AI_GATEWAY_SECRET_FIELDS: &[(AIGatewaySecretOperation,GatewaySecretRules)] = &[']
            for operation,rules in value.items():
                lines+=[f'(AIGatewaySecretOperation::{variant(operation)}, GatewaySecretRules {{']
                for direction in ['request','response']:
                    lines+=[f'{direction}: &[']
                    for rule in rules[direction]:
                        if rule['redact'] not in {'value','subtree'}:raise ValueError('Unknown secret redaction kind')
                        path=','.join(quoted(part) for part in rule['path']);one='Some(true)' if rule.get('oneTime') else 'None'
                        lines+=[f'GatewaySecretRule {{path:&[{path}],redact:GatewaySecretRedaction::{variant(rule["redact"])},one_time:{one}}},']
                    lines+=['],']
                lines+=['}),']
            lines+= ['];']
        elif isinstance(value,str):lines+=[f'pub const {name}: &str = {quoted(value)};']
        elif isinstance(value,int):lines+=[f'pub const {name}: usize = {value};']
        elif isinstance(value,list):
            typ='u16' if all(isinstance(v,int) for v in value) else '&str'
            if not all(isinstance(v,str if typ=='&str' else int) for v in value):raise ValueError(name)
            lines+=[f'pub const {name}: &[{typ}] = &['+','.join(quoted(v) for v in value)+'];']
        elif name=='AIRS_ENDPOINTS':lines+=[f'pub const {name}: &[(&str,&str)] = &['+','.join(f'({quoted(k)},{quoted(v)})' for k,v in value.items())+'];']
        else:raise ValueError(f"Unsupported constant {name}")
    result=subprocess.run([str(Path.home()/'.cargo/bin/rustfmt'),'+1.99.0','--edition','2024'],input='\n'.join(lines)+'\n',text=True,capture_output=True,check=True)
    return result.stdout

def main():
    parser=argparse.ArgumentParser();parser.add_argument('--check',action='store_true');args=parser.parse_args()
    output=generate()
    if args.check:
        if not OUTPUT.is_file() or OUTPUT.read_text()!=output:raise ValueError('Generated constants differ')
    else:OUTPUT.write_text(output)
    print(json.dumps({'checked':args.check,'staticConstants':162,'globalConfiguration':'isolated ScanConfiguration adaptation'}))
if __name__=='__main__':main()
