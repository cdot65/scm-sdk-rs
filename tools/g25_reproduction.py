#!/usr/bin/env python3
"""Execute the retained historical G25 reproducer and independent frozen Zod comparison."""
import datetime,hashlib,json,os,pathlib,subprocess
ROOT=pathlib.Path(__file__).resolve().parents[1]
def main():
    env=dict(os.environ);env.setdefault('CARGO_TARGET_DIR','/build/rust/scm-sdk-vendor-target')
    commands=[['python3','tools/verify-contracts.py','--baseline','.local/typescript'],[str(pathlib.Path.home()/'.cargo/bin/cargo'),'+1.99.0','run','--locked','--offline','--manifest-path','tools/g25-repro/Cargo.toml','--target','x86_64-unknown-linux-musl','--quiet'],['.local/typescript/node_modules/.bin/tsx','tools/g25-repro/typescript.mts']]
    runs=[]
    for command in commands:
        result=subprocess.run(command,cwd=ROOT,env=env,capture_output=True,text=True,timeout=300)
        runs.append({'command':command,'exitCode':result.returncode,'stdout':result.stdout,'stderr':result.stderr})
        if result.returncode:raise SystemExit('G25 historical reproduction command failed: '+command[0])
    cases=json.loads((ROOT/'tools/g25-repro/cases.json').read_text())
    sdk=[json.loads(s) for s in runs[1]['stdout'].splitlines()];ts=[json.loads(s) for s in runs[2]['stdout'].splitlines()]
    assert len(sdk)==len(ts)==len(cases)
    for case,a,b in zip(cases,sdk,ts):
        assert (a['schema'],a['field'])==(b['schema'],b['field'])==(case['schema'],case['field'])
        assert a['accepted']==case['expectedSdk'] and b['accepted']==case['expectedTypeScript']
    report={'schemaVersion':1,'executedAt':datetime.datetime.now(datetime.timezone.utc).isoformat(),'sourceCommit':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),'inputs':{str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted((ROOT/'tools/g25-repro').rglob('*')) if p.is_file()},'cases':cases,'sdkOutcomes':sdk,'typeScriptOutcomes':ts,'runs':runs,'confirmedFormatMismatches':sum(not a['accepted'] and b['accepted'] for a,b in zip(sdk,ts)),'limitations':['Historical pre-fix validator reproduction; the expected outcomes deliberately change after implementation.','Not the unavailable airs-cli-rs consumer suite.']}
    (ROOT/'docs/checks/phase-18-reproduction.json').write_text(json.dumps(report,indent=2)+'\n')
    print(json.dumps({'cases':len(cases),'confirmedFormatMismatches':report['confirmedFormatMismatches']}))
if __name__=='__main__':main()
