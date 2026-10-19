#!/usr/bin/env python3
"""Recount pinned response contracts and request UUID checks without changing SDK code."""
import argparse, hashlib, json, pathlib, re
from red_team_dataplane import client_impl
from verify_gateway_bindings import tokens, functions, arguments
ROOT=pathlib.Path(__file__).resolve().parents[1]
DEST=ROOT/'contracts/compatibility/g25/audit.json'
FORMATS={'uuid','rfc3339','datetime','date','duration','email','url','regex'}
READS=set()
def read(path):
    READS.add(str(path));return json.loads((ROOT/path).read_text())
def digest(path):return hashlib.sha256((ROOT/path).read_bytes()).hexdigest()
def fields(nodes,sid,path='$',optional=False,nullable=False,active=()):
    if sid in active:return [{'path':path,'node':sid,'kind':'RecursiveReference','optional':optional,'nullable':nullable}]
    n=nodes[sid];k=n['kind'];active=(*active,sid)
    if k in ('ZodOptional','ZodDefault','ZodNullable','ZodBranded'):
        return fields(nodes,n['inner']['$ref'],path,optional or k in ('ZodOptional','ZodDefault'),nullable or k=='ZodNullable',active)
    if k=='ZodLazy':return fields(nodes,n['target']['$ref'],path,optional,nullable,active)
    if k=='ZodEffects':return fields(nodes,n['inner']['$ref'],path,optional,nullable,active)
    row={'path':path,'node':sid,'kind':k,'optional':optional,'nullable':nullable}
    for key in ('checks','values','value','unknownFields','catchall'):
        if key in n:row[key]=n[key]
    rows=[row]
    if k=='ZodPipeline':
        for key in ('input','output'):rows+=fields(nodes,n[key]['$ref'],path+'|'+key,optional,nullable,active)
    if k=='ZodCatch':rows+=fields(nodes,n['inner']['$ref'],path+'|catch',optional,nullable,active)
    if k=='ZodObject':
        for key,value in sorted(n['properties'].items()):rows+=fields(nodes,value['schema']['$ref'],path+'.'+key,active=active)
    elif k=='ZodArray':rows+=fields(nodes,n['element']['$ref'],path+'[]',active=active)
    elif k=='ZodRecord':rows+=fields(nodes,n['value']['$ref'],path+'{}',active=active)
    elif k in ('ZodUnion','ZodDiscriminatedUnion'):
        for i,v in enumerate(n['variants']):rows+=fields(nodes,v['$ref'],path+f'|{i}',optional,nullable,active)
    elif k=='ZodIntersection':
        for key in ('left','right'):rows+=fields(nodes,n[key]['$ref'],path+'|'+key,optional,nullable,active)
    return rows

def audit():
    READS.clear()
    graph=read('crates/scm-sdk/src/validation/schema_graph.json')
    vendor=read('contracts/vendor/red-team-data-plane/0.93.0/operations.json')['operations']
    baseline={r['id']:r for r in read('contracts/operations.json')['operations']}
    mappings=[]
    for p in sorted((ROOT/'contracts/native-operation-mappings').glob('red-team*.json')):mappings+=read(str(p.relative_to(ROOT)))['operations']
    rows=[];counts={};affected=0
    for op in vendor:
        root=op['schemaBinding']['responseSchema']
        candidates=[baseline[m['baselineId']] for m in mappings if m['namespace'].split('::')[-1]==op['nativeClass'] and op['nativeMethod'] in m['methods']]
        if len(candidates)>1:raise ValueError('Ambiguous baseline operation')
        old=candidates[0] if candidates else None
        expr=old['directRequest']['responseSchema']['expression'] if old else None
        sid=None;array=False;opaque=False
        if expr:
            if expr in graph['roots']:sid=graph['roots'][expr]
            elif re.fullmatch(r'z.array\(\w+\)',expr):sid=graph['roots'][expr[8:-1]];array=True
            elif expr in ('z.object({}).passthrough()','z.unknown()'):opaque=True
            else:raise ValueError('Unclassified baseline response: '+expr)
        current=fields(graph['nodes'],graph['roots'][root.removesuffix(':response')+':strict-response']) if root else []
        old_fields=fields(graph['nodes'],sid,'$[]' if array else '$') if sid else []
        if array:old_fields.insert(0,{'path':'$','node':None,'kind':'ZodArray','optional':False,'nullable':False})
        formats=[]
        for f in current:
            for c in f.get('checks',[]):
                if c['kind'] in ('uuid','rfc3339','datetime','date','duration','email','url','regex'):
                    formats.append({'path':f['path'],'kind':c['kind']});counts[c['kind']]=counts.get(c['kind'],0)+1
        if formats:affected+=1
        source_path=op['source'];READS.add(source_path);source=(ROOT/source_path).read_text()
        if digest(source_path)!=op['sourceSha256']:raise ValueError('Stale vendor source hash: '+source_path)
        methods=functions(client_impl(source,op['nativeClass']))
        if op['nativeMethod'] not in methods:raise ValueError('Missing audited method: '+op['nativeClass']+'.'+op['nativeMethod'])
        uuid_checks=uuid_arguments(methods[op['nativeMethod']])
        rows.append({'operationId':op['operationId'],'nativeClass':op['nativeClass'],'nativeMethod':op['nativeMethod'],'responseRoot':root,'responseHandling':op['schemaBinding']['responseHandling'],'baselineOperation':old['id'] if old else None,'baselineExpression':expr,'baselineRoot':sid,'baselineArray':array,'baselineOpaque':opaque,'vendorFields':current,'baselineFields':old_fields,'responseFormats':formats,'requestUuidArguments':uuid_checks,'rejectedQueryOptions':op['rejectedLegacyParameters']})
    request_sources=[]
    for p in sorted((ROOT/'crates/scm-sdk/src/airs/red_team').glob('*.rs')):
        name=str(p.relative_to(ROOT));READS.add(name);source=p.read_text();body=tokens(source)
        total=len(uuid_arguments(body));per_method={name:uuid_arguments(body) for name,body in functions(source).items() if uuid_arguments(body)}
        if total!=sum(map(len,per_method.values())):raise ValueError('UUID inventory does not reconcile: '+name)
        if total:request_sources.append({'source':name,'occurrences':total,'methods':per_method})
    global_roots=[]
    for name,sid in sorted(graph['roots'].items()):
        if name.startswith('vendor:'):continue
        pending=[sid];seen=set();sites=[]
        while pending:
            current=pending.pop()
            if current in seen:continue
            seen.add(current);node=graph['nodes'][current]
            sites += [{'node':current,'kind':c['kind']} for c in node.get('checks',[]) if c['kind'] in FORMATS]
            def refs(value):
                if isinstance(value,dict):
                    if '$ref' in value:pending.append(value['$ref'])
                    for child in value.values():refs(child)
                elif isinstance(value,list):
                    for child in value:refs(child)
            refs(node)
        if sites:global_roots.append({'root':name,'formatSites':sorted(sites,key=lambda r:(r['node'],r['kind']))})
    commit=read('contracts/manifest.json')['baseline']['commit']
    return {'schemaVersion':1,'baselineCommit':commit,'inputs':{p:digest(p) for p in sorted(READS)},'operations':rows,'requestUuidInventory':request_sources,'allFrozenRootsWithFormats':global_roots,'globalScope':'All frozen roots callable through parse(..., response mode), including non-Red-Team and request-named roots. This is a conservative superset of typed response call sites. Each root lists unique reachable format nodes; request mode is unchanged.','summary':{'operations':len(rows),'jsonResponseRoots':sum(r['responseRoot'] is not None for r in rows),'matchedTypeScriptOperations':sum(r['baselineOperation'] is not None for r in rows),'formatAffectedOperations':affected,'formatOccurrences':counts,'allFrozenRootsWithFormats':len(global_roots),'requestUuidOccurrences':sum(r['occurrences'] for r in request_sources)},'policy':'Inventory only. Presence/type differences are explicit paired field records, not an assertion of semantic equivalence. New operations without a TypeScript counterpart are classified separately.'}

def uuid_arguments(body):
    found=[]
    for i in range(len(body)-3):
        if body[i:i+4]==['common','::','uuid','(']:
            args,_=arguments(body,i+3)
            if len(args)!=1:raise ValueError('Unexpected UUID argument count')
            found.append(''.join(args[0]))
    return found


def main():
    parser=argparse.ArgumentParser();parser.add_argument('--check',action='store_true');args=parser.parse_args()
    result=audit();content=json.dumps(result,indent=2)+'\n'
    if args.check:
        if not DEST.exists() or DEST.read_text()!=content:raise SystemExit('G25 response audit is stale')
    else:DEST.write_text(content)
    print(json.dumps(result['summary']))
if __name__=='__main__':main()
