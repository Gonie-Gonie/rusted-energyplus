"""Closed data contracts for the frozen paired runner. No recipe code is loaded."""
from __future__ import annotations
from dataclasses import dataclass
from pathlib import Path
import copy,hashlib,json,math,re,struct,os,csv,io
from . import common116 as c

ROOT=Path(__file__).resolve().parents[4]
PROFILE='ideal_loads.oa_topology.v1'
CASE_KEYS={'schema','id','case_kind','profile','source_authority','input','baseline_input','scientific_dom','policy','counts','clock','edit_ledger','inverse_operations','scientific','structure','aliases','documentary'}
def require(ok,message):
 if not ok:raise ValueError(message)
def read(path):return json.loads(Path(path).read_bytes())
def ref(path):
 p=Path(path).resolve();require(p.is_relative_to(ROOT),'Reference outside repository');b=p.read_bytes();return dict(path=p.relative_to(ROOT).as_posix(),sha256=hashlib.sha256(b).hexdigest(),bytes=len(b))
def auth(value):
 require(type(value)is dict and set(value)<={'path','sha256','bytes'}and {'path','sha256'}<=set(value),'Malformed reference')
 p=(ROOT/value['path']).resolve();require(p.is_relative_to(ROOT)and p!=ROOT,'Reference containment');actual=ref(p);require(actual['sha256']==value['sha256']and actual['bytes']==value.get('bytes',actual['bytes']),'Reference bytes differ');return p
def load(value):return read(auth(value))
def output(path,value):
 p=Path(path).resolve();require(p.is_relative_to(ROOT)and not p.exists(),'Output must be a fresh repository path');p.parent.mkdir(parents=True,exist_ok=True);p.write_text(json.dumps(value,indent=2,ensure_ascii=False,allow_nan=False)+'\n',encoding='utf8',newline='\n');return ref(p)
def exact_keys(value,keys,label):require(type(value)is dict and set(value)==set(keys),'Unexpected '+label+' fields')
def typed(value):
 if type(value)is float:require(math.isfinite(value),'Nonfinite JSON scalar');return ('float',value.hex())
 if isinstance(value,dict):return ('object',tuple((k,typed(value[k]))for k in sorted(value)))
 if isinstance(value,list):return ('array',tuple(typed(v)for v in value))
 return(type(value).__name__,value)
def digest(value):return hashlib.sha256(json.dumps(value,ensure_ascii=False,separators=(',',':')).encode()).hexdigest()
def parse_ref(value):
 name,sha=value.rsplit('#',1);return ref(auth(dict(path=name,sha256=sha)))
def snapshot(pin,current=False):
 rows=load(pin)['files'];by={r['historical_path']:r for r in rows};require(len(by)==len(rows),'Duplicate source snapshot member')
 for path,row in by.items():
  data=auth(row['archive']).read_bytes();require(len(data)==row['size_bytes'],'Source archive size')
  if current:require((ROOT/path).read_bytes()==data,'Current Source differs from build')
 return by
# Only these closed adapters may consume old owner schemas. New work emits SourceEvidence.v1.
LEGACY_OWNERS={
 'Root-Unit155-positive-OA-supply-humidity-above-one-execution-review.v1':('155',4,'all4_raw_endpoints_bound_to_preformat_snapshot','all_other4672_Source_files_literal157'),
 'Root-Unit158-positive-OA-exact-empty-dehumidification-execution-review.v1':('158',2,'all2_raw_endpoints_bound_to_preformat_snapshot','all_other4674_Source_files_literal160'),
 'Root-Unit159-positive-OA-exact-empty-humidification-execution-review.v1':('159',2,'all2_raw_endpoints_bound_to_preformat_snapshot','all_other4674_Source_files_literal161'),
}
EVIDENCE_KEYS={'schema','build','quality','adoption','format_review','executions','current_QA_commands','total_actual_QA_commands','failed_QA_attempts','historical_refs','retained_getter_review','format_stages','current_Source_primary_review','qa_reuse'}
_GETTER_TEST='heat_balance::ordinary_equipment::tests::actual_native_get_equipment_pair_and_input_owner_transitions'
_BUILD_COMMAND=['cargo','build','--locked','--offline','-p','ep_cli','--bin','eplus-rs','--message-format=json-render-diagnostics']
def _receipt(pin):
 r=load(pin);require(r['schema']=='recorded-porting-command.v1','Actual recorder receipt required')
 require(type(r['command'])is list and r['command'] and Path(r['cwd']).resolve()==ROOT,'Recorded command/cwd')
 require(type(r['source_bytes_match_before_and_after'])is bool,'Missing source custody observation')
 return r

def _authority_add(pins,pin):
 auth(pin)
 if pin not in pins:pins.append(pin)

class _AuthorityPins(list):
 def __init__(self):super().__init__();self.snapshots={}

def _authority_snapshot(pin,pins):
 # Cache immutable archived snapshots only for this one authority invocation.
 # Fresh current files are checked separately by inspect_snapshot every time.
 key=(pin['path'],pin['sha256'])
 if key in pins.snapshots:return pins.snapshots[key]
 value=load(pin);require(value['schema']=='rust-command-source-snapshot.v1'and value['bytes_normalized']is False,'Unnormalized Source snapshot required');_authority_add(pins,pin)
 rows=snapshot(pin);pins.snapshots[key]=rows;return rows

def _authority_stage(edge,pins):
 exact_keys(edge,{'kind','before_snapshot','after_snapshot','endpoints','receipt','review'},'Source stage')
 require(edge['kind']in ['adopt','format','correction'],'Unknown Source stage')
 before=_authority_snapshot(edge['before_snapshot'],pins);after=_authority_snapshot(edge['after_snapshot'],pins);require(set(before)==set(after),'Source stage inventory differs')
 endpoints=edge['endpoints'];require(type(endpoints)is list and len({v['path']for v in endpoints})==len(endpoints),'Duplicate Source stage endpoint')
 changed={p for p in before if before[p]['archive']['sha256']!=after[p]['archive']['sha256']or before[p]['size_bytes']!=after[p]['size_bytes']}
 require(changed=={v['path']for v in endpoints},'Source stage changed-path set differs')
 for row in endpoints:
  exact_keys(row,{'path','before','after'},'Source stage endpoint');p=row['path']
  for key,table in [('before',before),('after',after)]:
   require(auth(row[key]).read_bytes()==auth(table[p]['archive']).read_bytes(),'Source stage endpoint bytes');_authority_add(pins,row[key])
 review=load(edge['review']);require(review['accepted']is True,'Unaccepted Source stage review');_authority_add(pins,edge['review'])
 if edge['kind']=='format':
  require(edge['receipt']is not None,'Missing actual format receipt');r=_receipt(edge['receipt']);require(r['exit_code']==0 and r['launch_error']is None and r['source_snapshot']==edge['before_snapshot']and r['command'][:2]==['cargo','fmt']and '--check'not in r['command'],'Format receipt/source stage mismatch');_authority_add(pins,edge['receipt'])
 else:require(edge['receipt']is None,'Authored stage cannot invent an execution')
 return before,after

def _authority_reuse(item,receipt,final_snapshot,pins,test_names):
 exact_keys(item,{'receipt','lineage','bodies'},'QA reuse');require(item['receipt']==receipt,'Reuse receipt mismatch');r=_receipt(receipt);cursor=r['source_snapshot'];require(item['lineage'],'Missing reuse lineage')
 for edge in item['lineage']:
  require(edge['before_snapshot']==cursor,'Discontinuous reuse lineage');_authority_stage(edge,pins);cursor=edge['after_snapshot']
 require(cursor==final_snapshot,'Reuse lineage does not reach build')
 # A reused command is never counted as a newly executed body. Every changed
 # snapshot is visible; complete test-file equality and a Root stage review are mandatory.
 names=set()
 for body in item['bodies']:
  exact_keys(body,{'test_name','path','sha256','bytes'},'Reuse test-file proof')
  a=_authority_snapshot(r['source_snapshot'],pins);b=_authority_snapshot(final_snapshot,pins);path=body['path'];old=auth(a[path]['archive']).read_bytes();new=auth(b[path]['archive']).read_bytes()
  x,y=old,new
  require(x==y and len(x)==body['bytes']and hashlib.sha256(x).hexdigest()==body['sha256'],'Reused complete test file changed')
  require(x.count(('fn '+body['test_name'].rsplit('::',1)[-1]+'(').encode())==1,'Reuse body/name mismatch');names.add(body['test_name'])
 require(names==set(test_names),'Reuse body coverage differs')


def _getter_chain(owner,adoption,pins):
 if 'retained_Source160_actual_getter_owner_review'not in owner:return None
 keys=[k for k in adoption if k.startswith('actual_parent')and k.endswith('_publication')];require(len(keys)==1,'Getter parent publication identity')
 _authority_add(pins,adoption[keys[0]]);parent=load(adoption[keys[0]]);_authority_add(pins,parent['owner_execution_review']);require(parent['head']==parent['remote_head']and parent['worktree_clean']is True,'Getter parent not published cleanly');prior=load(parent['owner_execution_review'])
 expected=parent['owner_execution_review']if 'actual_getter_owner_fidelity_current_test'in prior and prior['actual_getter_owner_fidelity_current_test']is True else prior['retained_Source160_actual_getter_owner_review']
 retained=owner['retained_Source160_actual_getter_owner_review'];require(retained==expected,'Wrong historical getter lineage');return retained


def _authority_primary(pin,members,pins):
 p=load(pin);require(p['accepted']is True and p['literal_function_equal']is True,'Unaccepted primary-source review');_authority_add(pins,pin)
 path=p['current_source']['path'];current=auth(members[path]['archive']).read_bytes();old=auth(p['historical_archive']).read_bytes()
 marker=b'fn calculate_return_flows(';require(current.count(marker)==old.count(marker)==1,'Primary method identity')
 def body(raw):
  start=raw.index(marker);return raw[start:raw.index(b'\n}',start)+2]
 a,b=body(old),body(current);require(a==b and len(b)==p['function_bytes']and hashlib.sha256(b).hexdigest()==p['function_sha256'],'Primary function changed')
 historical=load(p['historical_build']);require(historical['source_snapshot']==p['historical_source_snapshot'],'Primary historical build/snapshot')
 rows=_authority_snapshot(p['historical_source_snapshot'],pins);entry=rows[p['original_citation']['path']];require(entry['archive']['sha256']==p['historical_archive']['sha256']==p['original_citation']['sha256']and entry['size_bytes']==p['original_citation']['bytes'],'Primary citation/archive differs')
 for key in ['historical_archive','historical_build']:_authority_add(pins,p[key])


def authority_from_evidence(evidence_ref,mode):
 require(mode in ['fresh','retained'],'Unknown authority mode');e=load(evidence_ref);legacy=e['schema']in LEGACY_OWNERS;pins=_AuthorityPins();_authority_add(pins,evidence_ref)
 if legacy:
  tag,n,raw_flag,other_flag=LEGACY_OWNERS[e['schema']];require(e['accepted']is True,'Unaccepted migration owner');build_ref=e['source_build'];quality_ref=e['quality'];adoption_ref=e['Source_adoption'];format_ref=e['Source_format_review'];executions=[dict(x,role='current',lineage=[])for x in e['executions']];historical=[];stages=None;reuse=[]
 else:
  exact_keys(e,EVIDENCE_KEYS,'SourceEvidence');require(e['schema']=='paired.SourceEvidence.v1','Unsupported source evidence schema');tag=None;build_ref=e['build'];quality_ref=e['quality'];adoption_ref=e['adoption'];format_ref=e['format_review'];executions=e['executions'];historical=e['historical_refs'];stages=e['format_stages'];reuse=e['qa_reuse']
 for pin in [build_ref,quality_ref,adoption_ref,format_ref]:_authority_add(pins,pin)
 build=load(build_ref);br,members=c.inspect_snapshot(build,mode=='fresh');require(br['command']==_BUILD_COMMAND,'Default actual build command');_authority_add(pins,build['receipt']);_authority_add(pins,build['source_snapshot']);_authority_add(pins,build['retained_executable']);pins.snapshots[(build['source_snapshot']['path'],build['source_snapshot']['sha256'])]=members
 quality=load(quality_ref);require(quality['stage']=='quality','Actual quality ledger required');current=quality['runs'];all_runs=(quality['all_actual_runs']if 'all_actual_runs'in quality else current)if legacy else quality['all_actual_runs']
 if not legacy:
  require(quality['source_summaries'],'Complete underlying QA summaries required');observed=[]
  for pin in quality['source_summaries']:
   original=load(pin);require(original['stage']=='quality','Underlying QA summary stage');observed+=original['runs'];_authority_add(pins,pin)
  require(observed==all_runs,'Complete underlying QA attempt order differs')
 require(len({r['receipt']['path']for r in all_runs})==len(all_runs)and len({r['receipt']['sha256']for r in all_runs})==len(all_runs)and len({r['receipt']['path']for r in current})==len(current)and len({r['receipt']['sha256']for r in current})==len(current),'Duplicate QA attempt path/content')
 failed=0;receipts={}
 for row in all_runs:
  pin=row['receipt'];r=_receipt(pin);_authority_add(pins,pin);receipts[pin['path']]=r;bad=r['exit_code']!=0 or r['launch_error']is not None
  require(row['exit_code']==r['exit_code']and (row['recorder_exit']!=0 if bad else row['recorder_exit']==0),'QA actual exit/recorder disagreement');failed+=int(bad)
  for key in ['stdout','stderr']:_authority_add(pins,r[key])
 reuse_by={x['receipt']['path']:x for x in reuse};require(len(reuse_by)==len(reuse),'Duplicate QA reuse')
 execution_by={x['receipt']['path']:x for x in executions};require(len(execution_by)==len(executions)and len({x['receipt']['sha256']for x in executions})==len(executions),'Duplicate test execution receipt path/content')
 for row in current:
  require(row in all_runs,'Current QA missing from actual attempts');r=receipts[row['receipt']['path']];require(r['exit_code']==0 and r['launch_error']is None,'Current QA failed')
  is_format=r['command'][:2]==['cargo','fmt']and '--check'not in r['command']
  if r['command'][:2]==['cargo','test']:require(row['receipt']['path']in execution_by,'Current test receipt lacks explicit body accounting')
  if not is_format:
   require(r['source_bytes_match_before_and_after']is True and r['repository_before']==r['repository_after'],'QA custody changed')
   if r['source_snapshot']!=build['source_snapshot']:
    key=row['receipt']['path'];require(key in reuse_by and key in execution_by,'Missing explicit reused-test lineage');_authority_reuse(reuse_by[key],row['receipt'],build['source_snapshot'],pins,execution_by[key]['test_names'])
 require(e['current_QA_commands']==len(current)and e['total_actual_QA_commands']==len(all_runs)and e['failed_QA_attempts']==failed,'QA count/history differs')
 current_bodies=reused_bodies=historical_bodies=0;current_getter=False;normalized=[]
 for item in executions:
  exact_keys(item,{'kind','role','test_names','receipt','actual_exit_code','lineage'},'test execution');require(item['kind']=='targeted_existing_test'and item['role']in ['current','reused','historical'],'Unknown test role');names=item['test_names'];require(type(names)is list and names and len(set(names))==len(names),'Missing/duplicate test names');r=_receipt(item['receipt']);_authority_add(pins,item['receipt'])
  require(r['command'][:2]==['cargo','test']and '--exact'in r['command']and all(name in r['command']for name in names),'Exact executed-test command required');require(item['actual_exit_code']==r['exit_code']==0 and r['launch_error']is None and r['source_bytes_match_before_and_after']is True,'Failed/unstable test credit')
  text=auth(r['stdout']).read_text(encoding='utf8');observed=re.findall(r'(?m)^test (\S+) \.\.\. ok\s*$',text);require(sorted(observed)==sorted(names),'Actual body execution set differs');_authority_add(pins,r['stdout']);_authority_add(pins,r['source_snapshot'])
  role=item['role'];key=item['receipt']['path']
  if role=='current':
   require(item['receipt']in [v['receipt']for v in current]and r['source_snapshot']==build['source_snapshot']and item['lineage']==[],'Current test source/role differs');current_bodies+=len(names);current_getter|=_GETTER_TEST in names
  elif role=='reused':
   require(item['receipt']in [row['receipt']for row in all_runs]and key in reuse_by and item['lineage']==reuse_by[key]['lineage'],'Reused test absent from complete actual QA or lineage differs');_authority_reuse(reuse_by[key],item['receipt'],build['source_snapshot'],pins,names);reused_bodies+=len(names)
  else:
   require(item['receipt']not in [v['receipt']for v in current]and item['lineage']==[],'Historical test cannot get current credit');_authority_snapshot(r['source_snapshot'],pins);historical_bodies+=len(names)
  normalized.append(item)
 adoption=load(adoption_ref);fmt=load(format_ref);require(adoption['accepted']is True and fmt['accepted']is True and fmt['Source_adoption']==adoption_ref and fmt['current_source_snapshot']==build['source_snapshot'],'Source adoption/format lineage')
 raw={x['path']:x['after']for x in adoption['files']};require(len(raw)==len(adoption['files']),'Duplicate adopted file');baseline=load(adoption['actual_before_Source_build']);_authority_add(pins,adoption['actual_before_Source_build']);old=_authority_snapshot(baseline['source_snapshot'],pins)
 if legacy:
  initial_pin=_receipt(fmt['format_receipt'])['source_snapshot'];initial=_authority_snapshot(initial_pin,pins);require(set(old)==set(initial)==set(members),'Source inventory differs');require({p for p in old if old[p]['archive']!=initial[p]['archive']}==set(raw),'Raw endpoint set differs')
  for path,pin in raw.items():require(auth(initial[path]['archive']).read_bytes()==auth(pin).read_bytes(),'Raw endpoint differs');_authority_add(pins,pin)
  for path in set(members)-set(raw):require(old[path]['archive']==initial[path]['archive']==members[path]['archive'],'Unrelated Source changed')
  require(len(raw)==n and fmt[raw_flag]is fmt[other_flag]is True,'Legacy staged-format flags');retained=_getter_chain(e,adoption,pins)
 else:
  require(type(stages)is list and stages,'Complete Source stage chain required');cursor=baseline['source_snapshot'];seen=set()
  for edge in stages:
   require(edge['before_snapshot']==cursor,'Discontinuous Source stage chain');before,after=_authority_stage(edge,pins);cursor=edge['after_snapshot'];seen.update(v['path']for v in edge['endpoints'])
  require(cursor==build['source_snapshot']and stages[0]['kind']=='adopt'and stages[0]['review']==adoption_ref and set(raw)<=seen,'Source stages do not reach final build');require({x['path']:x['after']for x in stages[0]['endpoints']}==raw,'Adopted raw endpoints differ');retained=e['retained_getter_review']
 final_paths=set(raw)if legacy else {p for p in old if old[p]['archive']['sha256']!=members[p]['archive']['sha256']or old[p]['size_bytes']!=members[p]['size_bytes']}
 require(fmt['source_inventory_count']==len(members),'Source inventory count differs');endpoints=fmt['formatted_source_endpoints'];require({x['path']for x in endpoints}==final_paths,'Formatted endpoint set differs')
 if not legacy:
  formats=[edge for edge in stages if edge['kind']=='format'];require(formats and formats[-1]['receipt']==fmt['format_receipt'],'Final format receipt differs');require(all(edge['receipt']in [row['receipt']for row in all_runs]for edge in formats),'Format stage missing from actual QA')
 for endpoint in endpoints:
  row=members[endpoint['path']];require(endpoint['sha256']==row['archive']['sha256']and endpoint['bytes']==row['size_bytes'],'Formatted endpoint snapshot differs')
  if mode=='fresh':require(auth(endpoint).read_bytes()==auth(row['archive']).read_bytes(),'Current endpoint bytes differ');_authority_add(pins,endpoint)
 for packet in adoption['packets']:
  peer=load(packet['independent_review']);require(peer['accepted_static_only']is True and peer['proposal']==packet['proposal']and peer['findings']==[],'Unaccepted Source proposal review')
  for pin in packet.values():_authority_add(pins,pin)
 if legacy:require([x['independent_review']for x in adoption['packets']]==e['independent_Source_reviews']and e['formatted_source_endpoints']==endpoints and e['source_snapshot']==build['source_snapshot']and e['source_inventory_count']==len(members)and e['current_existing_test_body_executions']==current_bodies,'Migration owner claims differ')
 parent_keys=[k for k in adoption if k.startswith('actual_parent')and k.endswith('_publication')]
 for key in parent_keys:_authority_add(pins,adoption[key])
 if retained is not None:
  owner=load(retained);require(owner['actual_getter_owner_fidelity_current_test']is True,'Missing historical actual getter proof');_authority_add(pins,retained)
  expected=_getter_chain(dict(retained_Source160_actual_getter_owner_review=retained),adoption,pins);require(expected==retained,'Retained getter chain differs')
  getter_build=load(owner['source_build']);c.inspect_snapshot(getter_build,False);require(getter_build['source_snapshot']==owner['source_snapshot'],'Historical getter build source differs')
  for pin in [owner['source_build'],getter_build['receipt'],getter_build['retained_executable']]:_authority_add(pins,pin)
  getter_rows=[x for x in owner['executions']if _GETTER_TEST in x['test_names']];require(len(getter_rows)==1,'Historical getter execution missing');gr=_receipt(getter_rows[0]['receipt']);require(gr['exit_code']==getter_rows[0]['actual_exit_code']==0 and gr['source_snapshot']==owner['source_snapshot']and gr['launch_error']is None and gr['source_bytes_match_before_and_after']is True and gr['command'][:2]==['cargo','test']and '--exact'in gr['command']and _GETTER_TEST in gr['command'],'Historical getter actual source/exit')
  text=auth(gr['stdout']).read_text('utf8');require(re.search(r'(?m)^test '+re.escape(_GETTER_TEST)+r' \.\.\. ok\s*$',text)is not None,'Historical getter body did not execute')
  for pin in [getter_rows[0]['receipt'],gr['stdout'],gr['source_snapshot']]:_authority_add(pins,pin)
 for pin in historical:_authority_add(pins,pin)
 primary=e['current_Source_primary_review'];_authority_primary(primary,members,pins)
 return dict(schema='paired.SourceAuthority.v1',mode=mode,evidence=evidence_ref,legacy_migration=tag,build=build_ref,quality=quality_ref,adoption=adoption_ref,format_review=format_ref,snapshot=build['source_snapshot'],inventory_count=len(members),changed_paths=sorted(final_paths),formatted_endpoints=endpoints,current_Source_primary_review=primary,current_QA_commands=len(current),total_actual_QA_commands=len(all_runs),failed_QA_attempts=failed,current_test_body_executions=current_bodies,reused_test_bodies=reused_bodies,historical_test_bodies=historical_bodies,retained_getter_body_executions=int(retained is not None),executions=normalized,historical_refs=historical,retained_getter_review=retained,current_getter_credit=current_getter,pins=pins)

def load_authority(pin,mode):
 a=load(pin);require(a['schema']=='paired.SourceAuthority.v1'and a['mode']==mode,'Authority mode/schema differs');require(a==authority_from_evidence(a['evidence'],mode),'Normalized authority differs from underlying evidence');return a

class Number:
 def __init__(self,token,kind):self.token=token;self.kind=kind;self.value=int(token)if kind=='int'else float(token)
def numeric_records(raw):
 tree=json.loads(raw,parse_int=lambda s:Number(s,'int'),parse_float=lambda s:Number(s,'float'));out={}
 def visit(v,path):
  if isinstance(v,Number):
   x=float(v.value);require(math.isfinite(x),'Nonfinite numeric input');out[path]=dict(JSON_path=path,JSON_kind=v.kind,token=v.token,parsed_binary64_bits=struct.pack('>d',x).hex(),parsed_binary64_hex=x.hex())
  elif isinstance(v,dict):
   for k,x in v.items():visit(x,path+'/'+k.replace('~','~0').replace('/','~1'))
  elif isinstance(v,list):
   for i,x in enumerate(v):visit(x,path+'/'+str(i))
 visit(tree,'');return out
def verify_ledger(config):
 ledger=load(config['edit_ledger']);exact_keys(ledger,{'schema','baseline_input','input','forward','inverse','forward_patch','inverse_patch','numeric_baseline','held_count','changed_numeric_records','added_numeric_records','removed_numeric_records','changed_alpha_records'},'InputEditLedger');require(ledger['schema']=='paired.InputEditLedger.v1'and ledger['baseline_input']==config['baseline_input']and ledger['input']==config['input'],'Ledger identity')
 before=auth(config['baseline_input']).read_bytes();after=auth(config['input']).read_bytes();require(verify_byte_edits(before,after,ledger['forward'])and verify_byte_edits(after,before,ledger['inverse']),'Raw edit inverse')
 require(apply_unified(before,auth(ledger['forward_patch']).read_bytes())==after and apply_unified(after,auth(ledger['inverse_patch']).read_bytes())==before,'Unified endpoint mismatch')
 a,b=numeric_records(before),numeric_records(after);shared=load(ledger['numeric_baseline']);require(shared['schema']in ['paired.SharedNumericBaselineReference.v1','paired.NumericBaselineReference.v1'],'Shared numeric baseline schema');container=load(shared['source_container']);require(shared['records_member']=='held_numeric_records'and container['baseline_input']==config['baseline_input'],'Unsupported numeric baseline member')
 for name,rows in [('held',container['held_numeric_records'])]+[(k,ledger[k])for k in ['changed_numeric_records','added_numeric_records','removed_numeric_records','changed_alpha_records']]:
  require(type(rows)is list and all(type(x)is dict and type(x.get('JSON_path'))is str for x in rows)and len({x['JSON_path']for x in rows})==len(rows),'Duplicate/malformed '+name+' record')
 records={x['JSON_path']:x for x in container['held_numeric_records']}
 if shared['schema']=='paired.NumericBaselineReference.v1':
  require(shared['before_records_member']=='changed_numeric_records','Unsupported baseline delta member')
  for row in container['changed_numeric_records']:
   require(row['JSON_path']not in records,'Duplicate baseline numeric path');records[row['JSON_path']]=row['before']
 require(a==records,'Baseline numeric map differs')
 changed={p:dict(JSON_path=p,before=a[p],after=b[p])for p in a.keys()&b.keys()if a[p]!=b[p]};added={p:b[p]for p in b.keys()-a.keys()};removed={p:a[p]for p in a.keys()-b.keys()};require({x['JSON_path']:x for x in ledger['changed_numeric_records']}==changed and {x['JSON_path']:x for x in ledger['added_numeric_records']}==added and {x['JSON_path']:x for x in ledger['removed_numeric_records']}==removed,'Changed/added/removed numeric records differ');require(ledger['held_count']==len(a.keys()&b.keys())-len(changed),'Held numeric count differs')
 def strings(v,path=''):
  if type(v)is str:return {path:v}
  if type(v)is dict:return {p:x for k,item in v.items()for p,x in strings(item,path+'/'+k.replace('~','~0').replace('/','~1')).items()}
  if type(v)is list:return {p:x for i,item in enumerate(v)for p,x in strings(item,path+'/'+str(i)).items()}
  return {}
 sa,sb=strings(json.loads(before)),strings(json.loads(after));alpha={p:dict(JSON_path=p,JSON_kind='string',before_present=p in sa,before=sa.get(p),after_present=p in sb,after=sb.get(p))for p in sa.keys()|sb.keys()if p not in sa or p not in sb or sa[p]!=sb[p]}
 require(not ((sa.keys()-sb.keys())&b.keys())and not ((sb.keys()-sa.keys())&a.keys()),'String/numeric type transitions require a new reviewed ledger profile')
 require({x['JSON_path']:x for x in ledger['changed_alpha_records']}==alpha,'Alpha kind/presence/value delta differs')
 require(shared['baseline_input']==config['baseline_input']and shared['record_count']==len(records)and shared['records_sha256']==digest([records[p]for p in sorted(records)]),'Shared numeric descriptor differs')
 document=json.loads(after);require(typed(document)==typed(load(config['scientific_dom'])),'Scientific DOM differs');restored=copy.deepcopy(document)
 for op in config['inverse_operations']:
  exact_keys(op,{'op','path','before_present','before','after_present','after'},'inverse operation');require(op['op']in ['set','remove']and type(op['path'])is list and op['path']and all(type(x)is str for x in op['path'])and type(op['before_present'])is type(op['after_present'])is bool,'Unsupported inverse operation');parent=restored
  for part in op['path'][:-1]:
   require(type(parent)is dict and part in parent,'Inverse path must traverse present object members');parent=parent[part]
  require(type(parent)is dict,'Inverse set/remove supports object members only; array operations are held')
  key=op['path'][-1];present=key in parent;require(present==op['before_present']and(not present or typed(parent[key])==typed(op['before'])),'Inverse precondition')
  if op['op']=='remove':require(present and op['after_present']is False,'Remove presence');del parent[key]
  else:require(op['after_present']is True,'Set presence');parent[key]=copy.deepcopy(op['after'])
 require(typed(restored)==typed(json.loads(before)),'Whole inverse does not restore baseline');return document
def structure_baseline(spec):
    bundle = _topology_contract(spec)
    _validate_topology_rows(bundle['prediction'], bundle['rows'], bundle['parent_rows'])
    return bundle['prediction'], bundle['expected']
def reference_closure(value):
 """Authenticate refs in closed case data; never recursively follow arbitrary file contents."""
 result={}
 def walk(v):
  if type(v)is dict:
   if {'path','sha256'}<=v.keys()and set(v)<={'path','sha256','bytes'}:result[v['path']]=ref(auth(v))
   else:
    for x in v.values():walk(x)
  elif type(v)is list:
   for x in v:walk(x)
 walk(value);return list(result.values())
@dataclass(frozen=True)
class Context:
 binding_ref:dict
 binding:dict
 release_ref:dict
 release:dict
 case_ref:dict
 case:dict
 authority_ref:dict
 authority:dict
 mode:str
 recipe:dict
 policy:dict
 document:dict
 environments:list
 native_bundle:dict
 pins:tuple
def load_context(binding_ref,case_id,mode):
 require(mode=='fresh','OA topology profile is fresh-only');binding=load(binding_ref);require(binding['schema']=='paired.RuntimeBinding.v1'and binding['mode']==mode,'Binding mode');release=load(binding['release']);require(release['schema']=='paired.Release.v3'and release['profile']==PROFILE and release['structure_schema']=='paired.OATopologyStructure.v1'and type(release['Source_metadata_schema'])is int and release['Source_metadata_schema']==1,'Release schema')
 for key in ['Root_review','independent_review']:
  review=load(binding[key]);require(review['schema']=='paired.RuntimeReview.v1'and review['accepted']is True and review['release']==binding['release']and review['case_config']==binding['case_config']and review['source_authority']==binding['source_authority']and review['mode']==mode,'External review binding differs')
 case=load(binding['case_config']);exact_keys(case,CASE_KEYS,'CaseConfig');require(case['schema']=='paired.CaseConfig.v1'and case['id']==case_id and case['profile']==PROFILE and case['source_authority']==binding['source_authority'],'Case profile/identity');authority=load_authority(binding['source_authority'],mode)
 require(set(case['scientific'])==set(release['scientific_fields']),'Scientific input field set');document=verify_ledger(case);policy=load(case['policy']);counts=case['counts'];require(counts['total_series']==len(policy['rows'])+len(policy['meter_rows'])and counts['variable_values']==len(policy['rows'])*counts['zone_steps_per_variable']and counts['meter_values']==len(policy['meter_rows'])*counts['meter_hour_samples']and counts['total_values']==counts['variable_values']+counts['meter_values'],'Exact numeric dimensions')
 require(digest(typed({k:case['scientific'][k]for k in release['constant_scientific_fields']}))==release['constant_scientific_sha256'],'Frozen scientific contract changed')
 require(case['policy']==release['policy']and counts==release['counts']and counts['ESO_series']==len(policy['rows'])and counts['MTR_series']==len(policy['meter_rows']),'Frozen profile policies/dimensions differ')
 require(case['baseline_input']==release['baseline_input'],'Initial profile baseline differs')
 require(case['structure']['prediction']==case['scientific']['structural_prediction'],'Two structural authorities differ')
 bundle=_topology_contract(case['structure']);structure_baseline(case['structure']);require(bundle['prediction']['input']==case['input']and bundle['exports']['input']==case['input'],'Topology input identity')
 if bundle['parent_case']['profile']==PROFILE:require(case['structure']['parent']['release']==binding['release'],'Topology parent release differs')
 adoption=load(authority['adoption']);require(adoption['actual_before_Source_build']==bundle['parent_authority']['build'],'Actual Source predecessor differs from published topology parent')
 exact_keys(case['aliases'],{'fuel_case','gate_case','original_exhaust_recipe','science_contract'},'profile aliases')
 require(len(policy['rows'])>155 and counts['DesignDays']==5 and counts['zone_steps_per_variable']==480 and counts['meter_hour_samples']==120,'Unsupported initial profile dimensions')
 recipe=copy.deepcopy(case['scientific']);recipe.update(input=case['input'],baseline_input=case['baseline_input'],scientific_dom=case['scientific_dom'],derived_policy=case['policy'],counts=counts,case_kind=case['case_kind'],inherited_heating_efficiency_case_kind=case['aliases']['fuel_case'],inherited_exhaust_baseline_recipe=case['aliases']['original_exhaust_recipe'],structure_config=case['structure'],science_contract=case['aliases']['science_contract'])
 recipe['exact_empty_selector_science_contract']=recipe['science_contract'];recipe.update(scope='B',format='epJSON',weather=None,weather_cli_argument=None,hours_cli_argument=None,DDOnly_cli_argument=False,Native_convert_argument=False)
 require(recipe['inherited_heating_efficiency_case_kind']=='POSITIVE'and case['aliases']['gate_case']=='SUM','Unexpected profile ancestry');envs=load(case['clock'])['environments'];require(len(envs)==5 and [x['title']for x in envs]==sorted(document['SizingPeriod:DesignDay']),'Clock basis')
 require(case['clock']==recipe['ordered_environment_clock_basis']and case['aliases']['original_exhaust_recipe']==release['original_exhaust_recipe'],'Clock/exhaust ancestry differs')
 require(authority['current_Source_primary_review']==recipe['current_Source_primary_review'],'Current return-flow authority differs')
 source_rows=snapshot(authority['snapshot'],mode=='fresh');documentary=case['documentary'];require(type(documentary)is list,'Documentary source list')
 for row in documentary:
  exact_keys(row,{'historical_path','archive'},'documentary source');auth(row['archive'])
 # Every current citation belongs to the actual build. Retained mode accepts only
 # the matching explicitly remapped archive for that same historical path.
 source_pins={}
 def sources(v):
  if type(v)is dict:
   if {'path','sha256'}<=v.keys()and set(v)<={'path','sha256','bytes'}:
    path=v['path'];matches=[k for k,row in source_rows.items()if path==k or path==row['archive']['path']]
    require(not path.startswith('crates/')or bool(matches),'Source citation not in actual build')
    if matches:
     require(len(matches)==1,'Ambiguous source citation');row=source_rows[matches[0]];expected=dict(path=matches[0],sha256=row['archive']['sha256'],bytes=row['size_bytes'])if mode=='fresh'else row['archive'];require(v==expected,'Stale/current source citation differs');source_pins[path]=v
   else:
    for x in v.values():sources(x)
  elif type(v)is list:
   for x in v:sources(x)
 sources(recipe);sources(bundle['derivation']['primary']);sources(bundle['derivation']['report_cutoff'])
 pp=load(recipe['primary_pins']);pins=bundle['pins']+[binding_ref,binding['release'],binding['case_config'],binding['source_authority'],binding['Root_review'],binding['independent_review']]+reference_closure(case)+list(release['files'].values())+authority['pins']
 for key in ['edit_ledger','clock']:
  pins+=reference_closure(load(case[key]))
 ledger=load(case['edit_ledger']);pins+=reference_closure(load(ledger['numeric_baseline']))
 pins+=reference_closure(load(case['aliases']['science_contract']))+[release['recorder']]
 for p in pp['Native_bundle'].values():auth(p);pins.append(p)
 unique={x['path']:ref(auth(x))for x in pins};require(len(source_rows)==authority['inventory_count']==4678,'Actual Source inventory count')
 return Context(binding_ref,binding,binding['release'],release,binding['case_config'],case,binding['source_authority'],authority,mode,recipe,policy,document,envs,pp['Native_bundle'],tuple(unique.values()))

# BEGIN FROZEN PROTOCOL KERNELS
R=ROOT

def typed_scientific(value):
    kind=type(value)
    if kind is float:return ('float',c.bits(value))
    if kind is dict:return ('object',tuple((k,typed_scientific(value[k])) for k in sorted(value)))
    if kind is list:return ('array',tuple(typed_scientific(v) for v in value))
    return (kind.__name__,value)



def dump(path,value):
    with path.open('x',encoding='utf8',newline='\n') as f:
        json.dump(value,f,indent=2,ensure_ascii=False,allow_nan=False);f.write('\n')



def inside(value,parent=R):
    p=Path(value);p=(R/p).resolve() if not p.is_absolute() else p.resolve()
    assert p.is_relative_to(parent) and p!=parent,(p,parent)
    return p



def clean_environment():
    values=c.selected_environment(dict(os.environ))
    assert len(values)==39 and all(v is None for v in values.values()),values
    assert not [k for k in os.environ if k.upper().startswith(('EP_','ENERGYPLUS_','RUSTED_ENERGYPLUS_'))]
    weather=c.default_weather_observation();assert not weather['lexical_entry_exists'] and not weather['exists']
    return values,weather



def command(engine,build,bundle,input_path,output):
    if engine=='native':return [str(c.auth(bundle['exe'])),'--output-directory',str(output),'--output-prefix','native',str(input_path)]
    return [str(c.auth(build['retained_executable'])),'run',str(input_path),'--output-dir',str(output),'--mode','compatibility','--partial','deny','--porting-scope','B','--trace-level','debug']



def files(output,engine,q):
    result={}
    for ext in ['eso','mtr','eio','err','end','rdd','mdd','mtd','shd','bnd']:
        matches=list(output.glob('*.'+ext));assert len(matches)==1,(ext,matches);result[ext]=matches[0]
    tables=list(output.glob('*tbl.csv'));assert len(tables)==1,tables;result['table']=tables[0]
    for pattern in ['*.dxf','*tbl.htm','*tbl.html','*tbl.txt','*tbl.tab','*.sql','*.sqlite','*.sqlite3','*.msgpack','*.cbor']:
        assert not list(output.glob(pattern)),('unexpected unrequested artifact',pattern)
    if engine=='source':
        result['converted_input']=output/'input/converted.epJSON';assert result['converted_input'].is_file()
    else:assert not (output/(c.auth(q['input']).stem+'.idf')).exists(), 'Native --convert must not create an IDF'
    return result



def verify_byte_edits(old,new,edits):
    previous=0;shift=0;chunks=[]
    for edit in sorted(edits,key=lambda row:row['offset']):
        at=edit['offset'];removed=bytes.fromhex(edit['removed_hex'])
        assert len(removed)==edit['removed_bytes'] and at>=previous and old[at:at+len(removed)]==removed
        after=at+shift;inserted=new[after:after+edit['inserted_bytes']]
        assert len(inserted)==edit['inserted_bytes'] and c.hashlib.sha256(inserted).hexdigest()==edit['inserted_sha256']
        chunks.append((at,after,removed,inserted));previous=at+len(removed);shift+=len(inserted)-len(removed)
    forward=old
    for at,after,removed,inserted in reversed(chunks):forward=forward[:at]+inserted+forward[at+len(removed):]
    assert forward==new
    inverse=new
    for at,after,removed,inserted in reversed(chunks):
        assert inverse[after:after+len(inserted)]==inserted
        inverse=inverse[:after]+removed+inverse[after+len(inserted):]
    assert inverse==old
    return True



def apply_unified(raw,patch):
    source=raw.splitlines(keepends=True);diff=patch.splitlines(keepends=True);output=[];cursor=0
    for i,line in enumerate(diff):
        if not line.startswith(b'@@ '):continue
        match=c.re.match(rb'@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@',line);assert match
        start=int(match[1])-1;assert start>=cursor;output.extend(source[cursor:start]);cursor=start
        for change in diff[i+1:]:
            if change.startswith((b'@@ ',b'--- ',b'+++ ')):break
            if change.startswith(b' '):assert source[cursor]==change[1:];output.append(source[cursor]);cursor+=1
            elif change.startswith(b'-'):assert source[cursor]==change[1:];cursor+=1
            elif change.startswith(b'+'):output.append(change[1:])
            else:raise AssertionError(('Unexpected patch record',change))
    output.extend(source[cursor:]);return b''.join(output)


def _topology_csv(raw):
    require(type(raw) is bytes and raw.endswith(b'\n'), 'Complete topology body required')
    text = raw.decode('utf8').replace('\r\n', '\n')
    require('\r' not in text and '\x00' not in text and not text.startswith('Program Version,'), 'Invalid topology body framing')
    return list(csv.reader(io.StringIO(text), strict=True))


def _topology_contract(spec):
    exact_keys(spec, {'schema', 'prediction', 'parent', 'parent_edit_ledger', 'parent_inverse_operations', 'body_overlay', 'source_exports', 'derivation', 'prediction_reviews'}, 'OA topology structure')
    require(spec['schema'] == 'paired.OATopologyStructure.v1', 'Unknown topology schema')
    pins = {}

    def take(pin):
        exact_keys(pin, {'path', 'sha256', 'bytes'}, 'topology pin')
        require(type(pin['path']) is str and pin['path'] and type(pin['sha256']) is str and re.fullmatch('[0-9a-f]{64}', pin['sha256']) is not None and type(pin['bytes']) is int and pin['bytes'] >= 0, 'Malformed topology pin')
        path = auth(pin)
        require(pin['path'] == path.relative_to(ROOT).as_posix(), 'Noncanonical topology path')
        if pin['path'] in pins:
            require(pins[pin['path']] == pin, 'Conflicting topology pin')
        pins[pin['path']] = pin
        return path.read_bytes()

    def data(pin):
        return json.loads(take(pin))

    parent = spec['parent']
    exact_keys(parent, {'case', 'release', 'publication', 'input', 'prediction'}, 'topology parent')
    parent_case, parent_release, publication = data(parent['case']), data(parent['release']), data(parent['publication'])
    require(parent_case['schema'] == 'paired.CaseConfig.v1' and parent_case['profile'] in ['ideal_loads.basic_positive_oa.v1', PROFILE] and parent_case['input'] == parent['input'] and parent_case['structure']['prediction'] == parent['prediction'], 'Parent case identity')
    if parent_case['profile'] == 'ideal_loads.basic_positive_oa.v1':
        require(parent['release'] == dict(path='tools/porting/paired/v1/release.json', sha256='bc74f5e874b22bfb373ff2bcda55c8961ab078c284f774fe85276b64332cff61', bytes=7167), 'Legacy parent release')
    require(parent_release['profile'] == parent_case['profile'], 'Parent profile/release')
    require(publication['schema'] == 'Root-paired-domain-publication.v2' and publication['worktree_clean'] is True and publication['release'] == parent['release'] and publication['new_qualifying_cases'] == 1 and publication['new_qualifying_values'] == 75120, 'Accepted parent publication required')
    parent_completed = data(publication['completed'])
    require(parent_completed['schema'] == 'paired.Completed.v1' and parent_completed['mode'] == 'fresh' and parent_completed['scientific_acceptance'] is True and parent_completed['qualifying_cases'] == 1 and parent_completed['qualifying_values'] == 75120 and parent_completed['case_config'] == parent['case'] and parent_completed['source_authority'] == parent_case['source_authority'] and parent_completed['release'] == parent['release'], 'Parent is not the accepted published case')
    parent_authority = data(parent_case['source_authority'])
    require(publication['Source_authority'] == parent_case['source_authority'] and publication['actual_Source_build'] == parent_authority['build'], 'Parent Source publication identity')
    data(parent_authority['build'])
    parent_prediction = data(parent['prediction'])
    parent_body = take(parent_prediction['expected_body'])
    parent_rows = _topology_csv(parent_body)
    parent_document = data(parent['input'])
    prediction = data(spec['prediction'])
    exact_keys(prediction, {'schema', 'input', 'parent', 'expected_body', 'rows', 'counts', 'nodes', 'connections', 'outdoor_registry', 'roles', 'derivation', 'source_exports'}, 'OA topology prediction')
    require(prediction['schema'] == 'paired.OATopologyPrediction.v1' and prediction['parent'] == parent and prediction['derivation'] == spec['derivation'] and prediction['source_exports'] == spec['source_exports'], 'Prediction identity')
    document = data(prediction['input'])
    expected = take(prediction['expected_body'])
    rows = _topology_csv(expected)
    require(type(prediction['rows']) is list and prediction['rows'] == rows, 'Complete predicted CSV rows differ')
    ledger = data(spec['parent_edit_ledger'])
    require(ledger['baseline_input'] == parent['input'] and ledger['input'] == prediction['input'], 'Parent input ledger identity')
    # Reuse the unchanged lexical/numeric/string/object-inverse kernel.
    verify_ledger(dict(baseline_input=parent['input'], input=prediction['input'], scientific_dom=prediction['input'], edit_ledger=spec['parent_edit_ledger'], inverse_operations=spec['parent_inverse_operations']))
    for pin in reference_closure(ledger):
        take(pin)
    shared = data(ledger['numeric_baseline'])
    data(shared['source_container'])
    # The shared container's unrelated historical fields are not new case
    # authority. Authenticate its whole bytes, not arbitrary nested references.
    for pin in reference_closure(shared):
        take(pin)
    overlay = spec['body_overlay']
    exact_keys(overlay, {'before', 'after', 'forward', 'inverse', 'byte_edits'}, 'topology body overlay')
    require(overlay['before'] == parent_prediction['expected_body'] and overlay['after'] == prediction['expected_body'], 'Topology overlay endpoints')
    require(verify_byte_edits(parent_body, expected, overlay['byte_edits']) and apply_unified(parent_body, take(overlay['forward'])) == expected and apply_unified(expected, take(overlay['inverse'])) == parent_body, 'Whole topology overlay inverse')
    exports = data(spec['source_exports'])
    exact_keys(exports, {'schema', 'input', 'parent_input', 'objects', 'object_order', 'core_nodelist_order'}, 'topology Source exports')
    require(exports['schema'] == 'paired.OATopologySourceExports.v1' and exports['input'] == prediction['input'] and exports['parent_input'] == parent['input'], 'Export contract identity')
    exact_keys(exports['objects'], {'NodeList', 'SetpointManager:Scheduled'}, 'exported families')
    exact_keys(exports['object_order'], {'NodeList', 'SetpointManager:Scheduled'}, 'exported orders')
    core_order = parent_case['scientific']['nodelist_order']
    require(type(core_order) is list and len(core_order) == len(set(core_order)) == 5 and exports['core_nodelist_order'] == core_order, 'Inherited five core NodeLists')
    for family in ['NodeList', 'SetpointManager:Scheduled']:
        require(type(document.get(family)) is dict and type(exports['objects'][family]) is dict and typed(exports['objects'][family]) == typed(document[family]), 'Complete exported object contract differs')
        order = exports['object_order'][family]
        require(type(order) is list and all(type(name) is str for name in order) and len(order) == len(set(order)) == len(document[family]) and order == sorted(document[family]), 'Complete direct-epJSON object order differs')
    require([name for name in exports['object_order']['NodeList'] if name in core_order] == core_order, 'Inherited NodeList relative order')
    require(all(name in document['NodeList'] and typed(document['NodeList'][name]) == typed(parent_document['NodeList'][name]) for name in core_order), 'Inherited core NodeList definition changed')
    require(len(document['SetpointManager:Scheduled']) == 9 and typed(document['SetpointManager:Scheduled']) == typed(parent_document['SetpointManager:Scheduled']), 'Existing nine SPM objects changed')
    derivation = data(spec['derivation'])
    exact_keys(derivation, {'schema', 'input', 'parent', 'primary', 'seed', 'report_cutoff', 'input_orders', 'events'}, 'topology derivation')
    require(derivation['schema'] == 'paired.OATopologyDerivation.v1' and derivation['input'] == prediction['input'] and derivation['parent'] == parent, 'Derivation identity')
    require(derivation['seed'] == dict(kind='empty_before_first_node_input', nodes=[], connections=[], registry=[], outdoor_next_stream=1) and type(derivation['seed']['outdoor_next_stream']) is int, 'Empty structural derivation seed required')
    exact_keys(derivation['primary'], {'Native', 'Source'}, 'topology primary owners')

    def primary(row, owner):
        exact_keys(row, {'ref', 'function', 'first_line', 'last_line'}, 'topology primary range')
        path = row['ref']['path']
        require(type(path) is str and path.endswith(('.rs', '.cc', '.hh')) and (path.startswith('crates/') if owner == 'Source' else path.startswith('.reference/')), 'Actual primary code path required')
        code = take(row['ref']).decode('utf8').splitlines()
        require(type(row['function']) is str and row['function'] and type(row['first_line']) is int and type(row['last_line']) is int and 1 <= row['first_line'] <= row['last_line'] <= len(code), 'Primary function/range')
        require(row['function'] in '\n'.join(code[row['first_line'] - 1:row['last_line']]), 'Primary range omits named function')

    for owner in ['Native', 'Source']:
        require(type(derivation['primary'][owner]) is list and derivation['primary'][owner], 'Missing derivation primary owners')
        for row in derivation['primary'][owner]:
            primary(row, owner)
    cutoff = derivation['report_cutoff']
    exact_keys(cutoff, {'label', 'Native', 'Source'}, 'topology report cutoff')
    require(type(cutoff['label']) is str and cutoff['label'], 'Report cutoff explanation required')
    for owner in ['Native', 'Source']:
        primary(cutoff[owner], owner)
    exact_keys(derivation['input_orders'], {'general_NodeLists', 'outdoor_lists', 'outdoor_singles'}, 'topology input orders')
    for label, family in [('general_NodeLists', 'NodeList'), ('outdoor_lists', 'OutdoorAir:NodeList'), ('outdoor_singles', 'OutdoorAir:Node')]:
        require(derivation['input_orders'][label] == sorted(document.get(family, {})), 'Genuine complete epJSON order')
    exact_keys(spec['prediction_reviews'], {'Root', 'independent'}, 'prediction reviews')
    require(spec['prediction_reviews']['Root'] != spec['prediction_reviews']['independent'], 'Two distinct prediction reviews required')
    for pin in spec['prediction_reviews'].values():
        review = data(pin)
        exact_keys(review, {'schema', 'accepted', 'accepted_static_only', 'findings', 'prediction', 'input', 'parent', 'source_exports', 'derivation', 'whole_body_predicted_before_actors', 'physical_outputs_consulted'}, 'prediction review')
        require(review['schema'] == 'paired.OATopologyPredictionReview.v1' and review['accepted'] is review['accepted_static_only'] is review['whole_body_predicted_before_actors'] is True and review['physical_outputs_consulted'] is False and review['findings'] == [], 'Unaccepted prospective prediction review')
        require(review['prediction'] == spec['prediction'] and review['input'] == prediction['input'] and review['parent'] == parent and review['source_exports'] == spec['source_exports'] and review['derivation'] == spec['derivation'], 'Prediction review binding differs')
    return dict(prediction=prediction, expected=expected, rows=rows, parent_case=parent_case, parent_release=parent_release, parent_authority=parent_authority, parent_rows=parent_rows, document=document, exports=exports, derivation=derivation, pins=list(pins.values()))


def _validate_topology_rows(pred, rows, parent_rows):
    def integer(value, minimum=0):
        require(type(value) is int and minimum <= value <= 2147483647, 'Topology integer domain')
        return value

    def token(value, minimum=0):
        require(type(value) is str and re.fullmatch('0|[1-9][0-9]*', value) is not None, 'Canonical topology integer token')
        return integer(int(value), minimum)

    def upper(value):
        return value.translate(str.maketrans('abcdefghijklmnopqrstuvwxyz', 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'))

    def selected(table, tag):
        return [[row[0].strip(), *row[1:]] for row in table if row and row[0].strip() == tag]

    exact_keys(pred['counts'], {'nodes', 'nonparent_connections', 'outdoor_registry', 'suspicious_nodes', 'nonconnected_nodes'}, 'topology counts')
    for key, value in pred['counts'].items():
        integer(value, 1 if key in ['nodes', 'outdoor_registry'] else 0)
    raw_nodes = selected(rows, 'Node')
    require(len(raw_nodes) == pred['counts']['nodes'], 'Complete Node count')
    nodes = []
    for ordinal, row in enumerate(raw_nodes, 1):
        require(len(row) == 5 and token(row[1], 1) == ordinal and type(row[2]) is str and row[2] and row[3] in ['blank', 'Air'], 'Exact ordered Node record')
        nodes.append(dict(id=ordinal, name=row[2], fluid=row[3], references=token(row[4])))
    require(type(pred['nodes']) is list and all(type(x) is dict for x in pred['nodes']) and typed(nodes) == typed(pred['nodes']), 'Predicted Node records differ')
    names = {row['name']: row['id'] for row in nodes}
    require(len(names) == len({upper(name) for name in names}) == len(nodes), 'Duplicate physical Node identity')
    parent_names = [row[2] for row in selected(parent_rows, 'Node')]
    require(set(parent_names) <= set(names), 'Inherited physical Node name removed')
    raw_connections = selected(rows, 'Non-Parent Node Connection')
    connections = []
    for row in raw_connections:
        require(len(row) == 6 and row[1] in names and all(type(x) is str and x for x in row[2:5]), 'Complete Non-Parent connection')
        connections.append(dict(node_id=names[row[1]], node=row[1], object_type=row[2], object_name=row[3], connection_type=row[4], fluid_stream=token(row[5], 1)))
    require(type(pred['connections']) is list and typed(connections) == typed(pred['connections']) and len(connections) == pred['counts']['nonparent_connections'], 'Predicted connection sequence differs')
    registry_rows = selected(rows, 'Outdoor Air Node')
    registry = []
    for row in registry_rows:
        require(len(row) == 3 and row[2] in names and token(row[1], 1) == names[row[2]], 'Outdoor registry identity')
        registry.append(names[row[2]])
    require(type(pred['outdoor_registry']) is list and all(type(x) is int for x in pred['outdoor_registry']) and registry == pred['outdoor_registry'] and len(registry) == len(set(registry)) == pred['counts']['outdoor_registry'], 'Complete unique ordered registry')
    for tag, key in [('#Nodes', 'nodes'), ('#Non-Parent Node Connections', 'nonparent_connections'), ('#Outdoor Air Nodes', 'outdoor_registry')]:
        require(selected(rows, tag) == [[tag, str(pred['counts'][key])]], 'Exact BND count/header row')
    suspicious = [['Suspicious Node', str(row['id']), row['name'], row['fluid'], '0'] for row in nodes if row['references'] == 0]
    require(selected(rows, 'Suspicious Node') == suspicious and len(suspicious) == pred['counts']['suspicious_nodes'], 'Suspicious rows/ref count')
    connected = {row['node_id'] for row in connections}
    nonconnected = [['NonConnected Node', str(row['id']), row['name']] for row in nodes if row['id'] not in connected]
    require(selected(rows, 'NonConnected Node') == nonconnected and len(nonconnected) == pred['counts']['nonconnected_nodes'], 'NonConnected rows')
    require(selected(rows, '#NonConnected Nodes') == ([['#NonConnected Nodes', str(len(nonconnected))]] if nonconnected else []), 'Conditional nonconnected count')
    suspicious_headers = [
        ['! Suspicious nodes have 0 references.  It is normal for some nodes', ' however.'],
        ['! Listing nodes with 0 references (culled from previous list):'],
        ['! <Suspicious Node>', '<NodeNumber>', '<Node Name>', '<Node Fluid Type>', '<# Times Node Referenced After Definition>'],
    ]
    nonconnected_headers = [['! <#NonConnected Nodes>', '<Number of NonConnected Nodes>'], ['! <NonConnected Node>', '<NonConnected Node Number>', '<NonConnected Node Name>']]
    def skeleton(table):
        # Remove only whole, correctly located variable blocks. Every other
        # separator/header/blank is retained in the parent comparison.
        table = list(table)
        separator = ['! ===============================================================']
        headers = {
            'Node': ['! <Node>', '<NodeNumber>', '<Node Name>', '<Node Fluid Type>', '<# Times Node Referenced After Definition>'],
            'Non-Parent Node Connection': ['! <Non-Parent Node Connection>', '<Node Name>', '<Node ObjectType>', '<Node ObjectName>', '<Node ConnectionType>', '<Node FluidStream>'],
            'Outdoor Air Node': ['! <Outdoor Air Node>', '<NodeNumber>', '<Node Name>'],
        }
        remove = set()
        ends = {}
        for tag, header in headers.items():
            require(table.count(header) == 1, 'Unique exact topology record header')
            start = table.index(header) + 1
            positions = [i for i, row in enumerate(table) if row and row[0].strip() == tag]
            require(positions == list(range(start, start + len(positions))), 'Complete contiguous record run at genuine header')
            require(all(table[i][0] == ' ' + tag for i in positions), 'Exact Native dynamic record tag spelling')
            remove.update(positions)
            ends[tag] = start + len(positions)
        local_nodes = selected(table, 'Node')
        zero_rows = [[' Suspicious Node', *row[1:]] for row in local_nodes if token(row[4]) == 0]
        local_connections = {row[1] for row in selected(table, 'Non-Parent Node Connection')}
        loose_rows = [[' NonConnected Node', row[1], row[2]] for row in local_nodes if row[2] not in local_connections]
        blocks = []
        if zero_rows:
            blocks.append((ends['Node'], [separator, *suspicious_headers, *zero_rows]))
        if loose_rows:
            blocks.append((ends['Non-Parent Node Connection'], [separator, nonconnected_headers[0], [' #NonConnected Nodes', str(len(loose_rows))], nonconnected_headers[1], *loose_rows]))
        for start, block in blocks:
            require(table[start:start + len(block)] == block, 'Exact conditional report block/location')
            remove.update(range(start, start + len(block)))
        conditional_tags = {row[0] for row in suspicious_headers + nonconnected_headers} | {'Suspicious Node', 'NonConnected Node', '#NonConnected Nodes'}
        require(all(i in remove for i, row in enumerate(table) if row and row[0].strip() in conditional_tags), 'Extra or misplaced conditional report row')
        answer = []
        for i, row in enumerate(table):
            if i in remove:
                continue
            tag = row[0].strip() if row else ''
            if tag in ['#Nodes', '#Non-Parent Node Connections', '#Outdoor Air Nodes']:
                require(len(row) == 2, 'Malformed topology count')
                answer.append([row[0], '<reviewed-topology-count>'])
            else:
                answer.append(row)
        return answer

    require(skeleton(rows) == skeleton(parent_rows), 'Unaccounted BND header/fixed-row remainder')
    core = lambda table: [row for row in selected(table, 'Non-Parent Node Connection') if row[2] not in ['OUTDOORAIR:NODE', 'OUTDOORAIR:NODELIST']]
    require(core(rows) == core(parent_rows), 'Inherited non-OA connection tuples/order changed')
    parent_case = load(pred['parent']['case'])
    q = parent_case['scientific']
    unit, zone = q['IdealLoads_public_name'], 'ZONE ONE'
    scheduled = [['Non-Parent Node Connection', item['node'], 'SETPOINTMANAGER:SCHEDULED', item['manager'], 'Setpoint', '1'] for item in q['controls'] + q['additional_output_controls']]
    exhaust = [['Non-Parent Node Connection', q['new_exhaust_node'], 'ZONEHVAC:EQUIPMENTCONNECTIONS', zone, 'ZoneExhaust', '1'], ['Non-Parent Node Connection', q['new_exhaust_node'], 'ZONEHVAC:IDEALLOADSAIRSYSTEM', unit, 'Inlet', '1']]
    sensors = [['Non-Parent Node Connection', name, 'ZONEHVAC:EQUIPMENTCONNECTIONS', zone, 'Sensor', '1'] for name in q['new_basis_members']]
    returns = [['Non-Parent Node Connection', name, 'ZONEHVAC:EQUIPMENTCONNECTIONS', zone, 'ZoneReturn', '1'] for name in q['return_nodes_original_order']]
    roles = {}
    for name, tuples in [('scheduled', scheduled), ('zone_exhaust', exhaust), ('basis_sensor', sensors), ('zone_return', returns)]:
        positions = []
        for row in tuples:
            found = [i for i, actual in enumerate(raw_connections) if actual == row]
            require(len(found) == 1, 'Missing/duplicate inherited connection role')
            positions.append(found[0])
        # Role positions are report order. The complete parent core comparison
        # above, not the order of a metadata tuple-construction loop, preserves
        # the original registration order.
        roles[name] = sorted(positions)
    roles['zone_core'] = [i for i, row in enumerate(connections) if row['object_type'] == 'ZONEHVAC:EQUIPMENTCONNECTIONS' and row['object_name'] == zone]
    roles['selected_ideal_loads'] = [i for i, row in enumerate(connections) if row['object_type'] == 'ZONEHVAC:IDEALLOADSAIRSYSTEM' and row['object_name'] == unit]
    require(len(roles['selected_ideal_loads']) == 3, 'Selected IdealLoads complete roles')
    for role, kind, object_name in [('outdoor_single', 'OUTDOORAIR:NODE', 'OutdoorAir:Node'), ('outdoor_list', 'OUTDOORAIR:NODELIST', 'OutdoorAir:NodeList')]:
        roles[role] = [i for i, row in enumerate(connections) if row['object_type'] == kind]
        # The Native enum is OutsideAir; the public BND token is OutdoorAir.
        require(all(connections[i]['object_name'] == object_name and connections[i]['connection_type'] == 'OutdoorAir' for i in roles[role]), 'Exact outdoor registration role')
    exact_keys(pred['roles'], set(roles), 'topology role positions')
    require(all(type(x) is list and all(type(i) is int and 0 <= i < len(connections) for i in x) and x == sorted(set(x)) for x in pred['roles'].values()) and pred['roles'] == roles, 'Complete computed role positions differ')
    require(set(i for values in roles.values() for i in values) == set(range(len(connections))), 'Unaccounted connection role')
    document = load(pred['input'])
    derivation = load(pred['derivation'])
    events = derivation['events']
    require(type(events) is list and events, 'Missing complete derivation events')
    common = {'ordinal', 'call_id', 'phase', 'operand', 'operation'}
    fields = common | {'effective_name', 'node_id', 'references_before', 'references_after', 'fluid_before', 'fluid_after', 'returned_nodes', 'registration', 'stream_before', 'stream_after', 'connection_ordinal', 'registry_ordinal'}
    required = {
        'input_fill': set(),
        'allocate': {'effective_name', 'node_id', 'references_after', 'fluid_after'},
        'reference': {'effective_name', 'node_id', 'references_before', 'references_after'},
        'fluid_type': {'effective_name', 'node_id', 'fluid_before', 'fluid_after'},
        'expand': {'effective_name', 'returned_nodes'},
        'advance_stream': {'returned_nodes', 'stream_before', 'stream_after'},
        'register_new': {'effective_name', 'node_id', 'registration', 'connection_ordinal'},
        'register_existing': {'effective_name', 'node_id', 'registration', 'connection_ordinal'},
        'registry_append': {'effective_name', 'node_id', 'registry_ordinal'},
        'registry_existing': {'effective_name', 'node_id', 'registry_ordinal'},
        'registry_commit': {'returned_nodes'},
        'zero_expand': {'effective_name', 'returned_nodes'},
        'zero_definition': {'effective_name', 'returned_nodes'},
    }
    phases = {'general_nodelist', 'zone_equipment', 'scheduled_manager', 'purchased_air', 'outdoor_list', 'outdoor_single', 'outdoor_input', 'selected_consumer'}

    def escape(value):
        return value.replace('~', '~0').replace('/', '~1')

    def resolve(path):
        require(type(path) is str and path.startswith('/'), 'Input JSON pointer required')
        value = document
        for part in path[1:].split('/'):
            require(re.search('~(?![01])', part) is None, 'Invalid JSON pointer escape')
            part = part.replace('~1', '/').replace('~0', '~')
            if type(value) is list:
                index = token(part)
                require(index < len(value), 'Input array pointer outside declared length')
                value = value[index]
            else:
                require(type(value) is dict and part in value, 'Missing input pointer')
                value = value[part]
        return value

    def operand_value(operand):
        exact_keys(operand, {'kind', 'pointer', 'key'}, 'closed input operand')
        kind, path, key = operand['kind'], operand['pointer'], operand['key']
        require(type(kind) is str and type(path) is str, 'Operand kind/pointer types')
        if kind == 'control':
            require(path == '' and key is None, 'Control operand shape')
            return None
        if kind == 'value':
            require(key is None, 'String value operand cannot select a key')
            value = resolve(path)
            require(type(value) is str, 'Actual String value operand required')
            return value
        require(kind == 'object_key' and path in ['/OutdoorAir:Node', '/OutdoorAir:NodeList'] and type(key) is str, 'Closed actual OA object-key operand')
        objects = resolve(path)
        require(type(objects) is dict and key in objects and type(objects[key]) is dict, 'Missing actual OA object key/body')
        return key

    def value_operand(path):
        return dict(kind='value', pointer=path, key=None)

    def key_operand(family, key):
        return dict(kind='object_key', pointer='/' + family, key=key)

    nodelists = {upper(name): value for name, value in document['NodeList'].items()}
    require(len(nodelists) == len(document['NodeList']), 'Ambiguous general NodeList identity')
    expected_definitions = []
    for name in derivation['input_orders']['general_NodeLists']:
        members = document['NodeList'][name].get('nodes')
        require(type(members) is list and members and all(type(row) is dict and type(row.get('node_name')) is str for row in members), 'Complete general NodeList String operands')
        expected_definitions += [value_operand('/NodeList/' + escape(name) + '/nodes/' + str(i) + '/node_name') for i in range(len(members))]
    # This sequence includes every GetObjectItem, even rows with zero alphas,
    # and each actual GetNodeNums operand. Nameless map keys are not A1 values.
    expected_outdoor = []
    for name in derivation['input_orders']['outdoor_lists']:
        body = document['OutdoorAir:NodeList'][name]
        require(type(body) is dict, 'Outdoor list object body')
        members = body.get('nodes', [])
        require(type(members) is list and all(type(row) is dict and type(row.get('node_or_nodelist_name')) is str for row in members), 'Outdoor list String operands')
        expected_outdoor.append(('fill', 'outdoor_list', key_operand('OutdoorAir:NodeList', name)))
        expected_outdoor += [('call', 'outdoor_list', value_operand('/OutdoorAir:NodeList/' + escape(name) + '/nodes/' + str(i) + '/node_or_nodelist_name')) for i in range(len(members))]
    for name in derivation['input_orders']['outdoor_singles']:
        operand = key_operand('OutdoorAir:Node', name)
        expected_outdoor += [('fill', 'outdoor_single', operand), ('call', 'outdoor_single', operand)]
    ideal = document.get('ZoneHVAC:IdealLoadsAirSystem')
    require(type(ideal) is dict and len(ideal) == 1, 'One selected genuine IdealLoads owner')
    ideal_name = next(iter(ideal))
    require(upper(ideal_name) == unit and type(ideal[ideal_name].get('outdoor_air_inlet_node_name')) is str and ideal[ideal_name]['outdoor_air_inlet_node_name'], 'Actual selected outdoor inlet String')
    selected_operand = value_operand('/ZoneHVAC:IdealLoadsAirSystem/' + escape(ideal_name) + '/outdoor_air_inlet_node_name')
    selected_calls = []
    live, live_names, persisted, keys, pending = [], {}, [], {}, []
    final_registry = None
    next_stream = 1
    calls, call_order, definition_operands, actual_outdoor = {}, [], [], []
    previous_call = None

    def finish_call(info):
        if info['requested_fluid'] == 'Air':
            ids = info['expansion'] if info['expansion'] is not None else info['registry']
            require(all(live[i - 1]['fluid'] == 'Air' for i in ids), 'Air-requested call must finish its own fluid writes')

    for at, event in enumerate(events, 1):
        exact_keys(event, fields, 'topology event')
        require(type(event['ordinal']) is int and event['ordinal'] == at and type(event['call_id']) is str and event['call_id'] and event['phase'] in phases and event['operation'] in required, 'Event ordinal/phase/operation')
        op, phase, call = event['operation'], event['phase'], event['call_id']
        nonnull = common | required[op]
        require(all((event[key] is not None) if key in nonnull else (event[key] is None) for key in fields), 'Operation-specific required/null map')
        operand = operand_value(event['operand'])
        require(operand is not None or op == 'registry_commit', 'Unsupported implicit operand')
        if call != previous_call:
            require(call not in calls, 'Derivation calls cannot interleave or reopen')
            if previous_call is not None:
                finish_call(calls[previous_call])
            previous_call = call
            call_order.append(call)
            calls[call] = dict(phase=phase, operand=event['operand'], fill=op == 'input_fill', requested_fluid='blank' if phase in ['general_nodelist', 'scheduled_manager', 'outdoor_input'] else 'Air', expansion=None, kind=None, allocated=[], references=[], typed_nodes=[], registered=[], registry=[], registry_ops=[], advanced=False)
            if phase in ['outdoor_list', 'outdoor_single']:
                actual_outdoor.append(('fill' if op == 'input_fill' else 'call', phase, event['operand']))
            if phase == 'selected_consumer':
                require(final_registry is not None and event['operand'] == selected_operand and not selected_calls, 'Exactly one genuine selected CheckAndAdd operand after getter')
                predecessors = [calls[label] for label in call_order[:-1] if calls[label]['phase'] not in ['outdoor_list', 'outdoor_single', 'outdoor_input']]
                require(predecessors and predecessors[-1]['phase'] == 'purchased_air' and predecessors[-1]['operand'] == selected_operand, 'CheckAndAdd must follow its real PurchasedAir outdoor inlet call')
                registered = predecessors[-1]['registered']
                require(len(registered) == 1 and registered[0]['object_type'] == 'ZONEHVAC:IDEALLOADSAIRSYSTEM' and registered[0]['object_name'] == unit and registered[0]['connection_type'] == 'Outlet' and registered[0]['fluid_stream'] == 1, 'Actual PurchasedAir OA outlet registration precedes membership')
                selected_calls.append(call)
        info = calls[call]
        require(info['phase'] == phase and info['operand'] == event['operand'], 'Call owner/operand changed')
        require(not info['fill'] or op == 'input_fill', 'GetObjectItem fill cannot contain node operations')
        if actual_outdoor and final_registry is None:
            require(phase in ['outdoor_list', 'outdoor_single', 'outdoor_input'], 'Core work interleaved inside synchronous outdoor getter')
        if phase in ['outdoor_list', 'outdoor_single']:
            require(final_registry is None, 'Outdoor input after registry commit')
            if op != 'input_fill':
                require(event['operand']['kind'] == ('value' if phase == 'outdoor_list' else 'object_key'), 'Outdoor GetNodeNums actual operand kind')
        if phase == 'outdoor_input':
            require(op == 'registry_commit', 'Only successful collection commit belongs to outdoor_input')
        if phase == 'general_nodelist':
            require(op in ['allocate', 'reference', 'zero_definition'] and event['operand']['kind'] == 'value' and event['effective_name'] == upper(operand), 'General NodeList definition operation')
            definition_operands.append(event['operand'])
        for key in ['node_id', 'references_before', 'references_after', 'stream_before', 'stream_after', 'connection_ordinal', 'registry_ordinal']:
            if event[key] is not None:
                integer(event[key], 1 if key in ['node_id', 'stream_before', 'stream_after'] else 0)
        if event['effective_name'] is not None:
            require(type(event['effective_name']) is str, 'Effective name type')
        if event['returned_nodes'] is not None:
            require(type(event['returned_nodes']) is list and all(type(i) is int and 1 <= i <= len(live) for i in event['returned_nodes']), 'Positive returned IDs; Node0 is undefined/unestablished')
        node = None
        if event['node_id'] is not None and op != 'allocate':
            require(event['node_id'] <= len(live), 'Node event before allocation')
            node = live[event['node_id'] - 1]
            require(event['effective_name'] == node['name'], 'Node name/ID association')
        if op == 'input_fill':
            require(phase in ['outdoor_list', 'outdoor_single'] and event['operand']['kind'] == 'object_key' and event['operand']['pointer'] == ('/OutdoorAir:NodeList' if phase == 'outdoor_list' else '/OutdoorAir:Node') and call_order.count(call) == 1, 'Actual outdoor object input fill')
            require(sum(row['call_id'] == call for row in events) == 1, 'Exactly one input fill event per object')
        elif op == 'allocate':
            require(event['node_id'] == len(live) + 1 and event['effective_name'] and upper(event['effective_name']) not in live_names and event['references_after'] == 0 and event['fluid_after'] in ['blank', 'Air'], 'Genuine first Node allocation')
            require(event['effective_name'] == upper(operand) and event['fluid_after'] == info['requested_fluid'], 'Actual allocation projection/requested fluid')
            live.append(dict(id=event['node_id'], name=event['effective_name'], fluid=event['fluid_after'], references=0))
            live_names[upper(event['effective_name'])] = event['node_id']
            info['allocated'].append(event['node_id'])
        elif op == 'reference':
            require(event['references_before'] == node['references'] and event['references_after'] == node['references'] + 1, 'Live reference progression')
            node['references'] += 1
            info['references'].append(event['node_id'])
        elif op == 'fluid_type':
            require(info['requested_fluid'] == 'Air' and event['fluid_before'] == node['fluid'] == 'blank' and event['fluid_after'] == 'Air', 'Native requested blank-to-Air write')
            node['fluid'] = 'Air'
            info['typed_nodes'].append(event['node_id'])
        elif op in ['expand', 'zero_expand']:
            require(phase not in ['general_nodelist', 'outdoor_input'] and info['expansion'] is None and event['effective_name'] == upper(operand), 'Exact call expansion operand')
            name = event['effective_name']
            if op == 'zero_expand':
                require(name == '' and event['returned_nodes'] == [], 'Only direct empty String is a defined zero expansion')
                wanted, kind = [], 'zero'
            elif name in nodelists:
                members = nodelists[name]['nodes']
                nonblank = sum(bool(row['node_name']) for row in members)
                prefix = members[:nonblank]
                require(prefix and all(row['node_name'] for row in prefix), 'Referenced interior-blank prefix is Native undefined/unestablished')
                require(all(upper(row['node_name']) in live_names for row in prefix), 'General NodeList member not allocated')
                wanted, kind = [live_names[upper(row['node_name'])] for row in prefix], 'list'
            else:
                require(name and name in live_names, 'Direct name lacks allocation')
                wanted, kind = [live_names[name]], 'direct'
            require(event['returned_nodes'] == wanted, 'Expansion IDs/order differ')
            info['expansion'], info['kind'] = wanted, kind
        elif op == 'zero_definition':
            require(phase == 'general_nodelist' and operand == event['effective_name'] == '' and event['returned_nodes'] == [], 'Blank definition has no Node0 access')
        elif op == 'advance_stream':
            require(phase in ['outdoor_list', 'outdoor_single'] and not info['advanced'] and event['returned_nodes'] == info['expansion'], 'Outdoor stream call identity')
            require(event['stream_before'] == next_stream and event['stream_after'] == next_stream + len(event['returned_nodes']), 'Returned-length stream increment')
            require([row['fluid_stream'] for row in info['registered']] == list(range(next_stream, next_stream + len(event['returned_nodes']))), 'Per-member streams')
            next_stream = integer(event['stream_after'], 1)
            info['advanced'] = True
        elif op in ['register_new', 'register_existing']:
            row = event['registration']
            exact_keys(row, {'node_id', 'node', 'object_type', 'object_name', 'connection_type', 'fluid_stream'}, 'registration event')
            require(type(row['node_id']) is int and row['node_id'] == event['node_id'] and row['node'] == event['effective_name'] and all(type(row[k]) is str and row[k] for k in ['object_type', 'object_name', 'connection_type']), 'Registration Node/key')
            integer(row['fluid_stream'], 1)
            require(info['expansion'] is not None, 'Registration before expansion')
            if phase in ['outdoor_list', 'outdoor_single', 'selected_consumer']:
                family = 'OutdoorAir:NodeList' if phase == 'outdoor_list' else 'OutdoorAir:Node'
                require(row['object_type'] == upper(family) and row['object_name'] == family and row['connection_type'] == 'OutdoorAir', 'Registration bound to actual outdoor owner')
                if phase == 'selected_consumer':
                    require(final_registry is not None and row['fluid_stream'] == len(final_registry), 'CheckAndAdd uses current registry length as stream')
            key = (row['node_id'], row['object_type'], upper(row['object_name']), row['connection_type'], row['fluid_stream'])
            if op == 'register_new':
                require(key not in keys and event['connection_ordinal'] == len(persisted), 'New registration ordinal/key')
                keys[key] = len(persisted)
                persisted.append(row)
            else:
                require(key in keys and event['connection_ordinal'] == keys[key] and persisted[keys[key]] == row, 'Registration reuse differs')
            info['registered'].append(row)
        elif op in ['registry_append', 'registry_existing']:
            require(phase in ['outdoor_list', 'outdoor_single', 'selected_consumer'], 'Registry owner phase')
            active = pending if phase != 'selected_consumer' else final_registry
            require(active is not None, 'Consumer before getter result')
            if phase == 'selected_consumer':
                require(event['operand']['kind'] == 'value' and upper(operand) == event['effective_name'], 'CheckAndAdd actual already-resolved node operand')
            if op == 'registry_append':
                require(event['node_id'] not in active and event['registry_ordinal'] == len(active), 'First-seen registry append')
                active.append(event['node_id'])
            else:
                require(phase != 'outdoor_single' and event['node_id'] in active and event['registry_ordinal'] == active.index(event['node_id']), 'Registry reuse; duplicate single is fatal')
            info['registry'].append(event['node_id'])
            info['registry_ops'].append(op)
        else:
            require(op == 'registry_commit' and phase == 'outdoor_input' and event['operand']['kind'] == 'control' and final_registry is None and event['returned_nodes'] == pending and actual_outdoor == expected_outdoor, 'Complete successful getter registry commit')
            final_registry = list(pending)
    finish_call(calls[previous_call])
    require(definition_operands == expected_definitions, 'Every general definition slot exactly once in IP order')
    require(actual_outdoor == expected_outdoor, 'Every outdoor object and operand call exactly once in IP order')
    require(len(selected_calls) == 1, 'Missing selected CheckAndAdd membership operation')
    for call, info in calls.items():
        if info['fill']:
            continue
        ids = info['expansion']
        if ids is None:
            if info['phase'] == 'selected_consumer':
                require(info['registry_ops'] == ['registry_existing'] and not any(info[k] for k in ['allocated', 'references', 'typed_nodes', 'registered']) and not info['advanced'], 'Existing CheckAndAdd membership has no GetNodeNums writes')
            else:
                require(info['phase'] in ['general_nodelist', 'outdoor_input'] and not any(info[k] for k in ['typed_nodes', 'registered', 'registry']) and not info['advanced'], 'Unaccounted core call/result')
            continue
        require([row['node_id'] for row in info['registered']] == ids, 'Returned nodes/registration coverage')
        require(all(i in ids for i in info['allocated'] + info['typed_nodes']) and len(info['allocated']) == len(set(info['allocated'])) and len(info['typed_nodes']) == len(set(info['typed_nodes'])), 'No allocation/type writes outside actual result')
        expected_refs = ids if info['kind'] == 'list' else [i for i in ids if i not in info['allocated']]
        require(info['references'] == expected_refs, 'Complete reference call coverage')
        if info['phase'] in ['outdoor_list', 'outdoor_single']:
            require(info['advanced'] and info['registry'] == ids, 'Outdoor stream/registry coverage')
            require(info['phase'] != 'outdoor_single' or len(ids) == 1, 'Successful OutdoorAir:Node must return exactly one physical node')
        elif info['phase'] == 'selected_consumer':
            require(info['registry_ops'] == ['registry_append'] and info['registry'] == ids and not info['allocated'] and not info['advanced'], 'Auto-append CheckAndAdd exact node/result')
    require(typed(live) == typed(nodes) and typed(persisted) == typed(connections) and final_registry == registry, 'Complete seeded progression differs from prediction')
    return dict(nodes=raw_nodes, connections=raw_connections, ids=names, references={row['name']: row['references'] for row in nodes}, registry=registry_rows, roles=roles, suspicious=suspicious, nonconnected=nonconnected)
