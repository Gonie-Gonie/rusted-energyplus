"""Closed data contracts for the frozen paired runner. No recipe code is loaded."""
from __future__ import annotations
from dataclasses import dataclass
from pathlib import Path
import copy,hashlib,json,math,re,struct,os
from . import common116 as c

ROOT=Path(__file__).resolve().parents[4]
PROFILE='ideal_loads.basic_positive_oa.v1'
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
 value=load(pin);require(value['schema']=='rust-command-source-snapshot.v1'and value['bytes_normalized']is False,'Unnormalized Source snapshot required')
 rows=value['files'];require(type(rows)is list,'Source inventory must be a list');by={source_path(r['historical_path']):r for r in rows};require(len(by)==len(rows),'Duplicate source snapshot member');check_source_member_metadata(by)
 for path,row in by.items():
  require(type(row['size_bytes'])is int and row['size_bytes']>=0,'Invalid Source member size');data=auth(row['archive']).read_bytes();require(len(data)==row['size_bytes'],'Source archive size')
  if current:require((ROOT/path).read_bytes()==data,'Current Source differs from build')
 if current:
  paths=set((ROOT/'crates').rglob('*.rs'))|set((ROOT/'crates').rglob('Cargo.toml'));paths|={ROOT/n for n in ['Cargo.toml','Cargo.lock','rust-toolchain.toml','.cargo/config.toml']if(ROOT/n).is_file()}
  require(set(by)=={p.relative_to(ROOT).as_posix()for p in paths},'Current Source membership differs')
 return by
EVIDENCE_KEYS={'QA_profile','schema','build','quality','adoption','format_review','executions','current_QA_commands','total_actual_QA_commands','failed_QA_attempts','historical_refs','retained_getter_review','format_stages','current_Source_primary_review','qa_reuse'}
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

def source_path(path):
 require(type(path)is str and path and '\\'not in path,'Invalid Source path')
 parts=path.split('/');require(all(part and part not in ['.','..']for part in parts),'Noncanonical Source path')
 require(not Path(path).is_absolute()and ':'not in path,'Absolute Source path')
 require((path.startswith('crates/')and(path.endswith('.rs')or path.endswith('/Cargo.toml')))or path in ['Cargo.toml','Cargo.lock','rust-toolchain.toml','.cargo/config.toml'],'Path outside recorded Source domain')
 target=ROOT/path;require(target.resolve().is_relative_to(ROOT)and target.resolve()!=ROOT,'Source path escape')
 return path

def source_delta(before,after):
 return {p for p in set(before)|set(after)if p not in before or p not in after or(before[p]['archive']['sha256'],before[p]['size_bytes'])!=(after[p]['archive']['sha256'],after[p]['size_bytes'])}

def check_source_member_metadata(members):
 require(type(members)is dict,'Invalid Source member table')
 for path,row in members.items():
  source_path(path);exact_keys(row,{'historical_path','size_bytes','archive'},'Source snapshot member')
  require(row['historical_path']==path and type(row['size_bytes'])is int and row['size_bytes']>=0,'Invalid Source member size/path')
  pin=row['archive'];require(type(pin)is dict and {'path','sha256'}<=set(pin)<={'path','sha256','bytes'}and type(pin['path'])is str and type(pin['sha256'])is str and re.fullmatch('[0-9a-f]{64}',pin['sha256'])is not None,'Invalid Source archive metadata')
  if 'bytes'in pin:require(type(pin['bytes'])is int and pin['bytes']==row['size_bytes'],'Invalid Source archive size metadata')

def source_endpoints(before,after):
 return [dict(path=p,before=before[p]['archive']if p in before else None,after=after[p]['archive']if p in after else None)for p in sorted(source_delta(before,after))]

def check_source_endpoints(rows,before,after,pins):
 require(type(rows)is list,'Source endpoints must be a list')
 require(all(type(row)is dict and set(row)=={'path','before','after'}for row in rows),'Malformed Source endpoint')
 paths=[source_path(row['path'])for row in rows];require(paths==sorted(source_delta(before,after)),'Source endpoint order/set differs')
 for row in rows:
  p=row['path'];require(row['before']is not None or row['after']is not None,'Both Source sides absent')
  for key,table in [('before',before),('after',after)]:
   pin=row[key];require((pin is None)==(p not in table),'Source endpoint presence differs')
   if pin is not None:
    expected=table[p];body=auth(pin).read_bytes();require(len(body)==expected['size_bytes']and pin['sha256']==expected['archive']['sha256']and body==auth(expected['archive']).read_bytes(),'Source endpoint/archive differs');_authority_add(pins,pin)
 return paths

def _authority_stage(edge,pins):
 exact_keys(edge,{'kind','before_snapshot','after_snapshot','endpoints','receipt','review'},'Source stage')
 require(edge['kind']in ['adopt','format','correction'],'Unknown Source stage')
 before=_authority_snapshot(edge['before_snapshot'],pins);after=_authority_snapshot(edge['after_snapshot'],pins)
 check_source_endpoints(edge['endpoints'],before,after,pins)
 review=load(edge['review']);require(review['accepted']is True,'Unaccepted Source stage review');_authority_add(pins,edge['review'])
 if edge['kind']=='format':
  require(set(before)==set(after),'Formatter changed Source membership')
  require(edge['receipt']is not None,'Missing actual format receipt');r=_receipt(edge['receipt']);require(r['exit_code']==0 and r['launch_error']is None and r['source_snapshot']==edge['before_snapshot']and r['command'][:2]==['cargo','fmt']and '--check'not in r['command'],'Format receipt/source stage mismatch');_authority_add(pins,edge['receipt'])
 else:
  require(edge['receipt']is None,'Authored stage cannot invent an execution')
  require(type(review['files'])is list and all(type(row)is dict and {'path','before','after'}<=set(row)for row in review['files']),'Missing reviewed Source endpoints')
  check_source_endpoints([{key:row[key]for key in ['path','before','after']}for row in review['files']],before,after,pins)
 return before,after

def qa_profile(name):
 require(name=='simple_glazing.unused_material.v1','Unknown Source QA profile')
 packages=['-p','ep_runtime','-p','ep_run','-p','ep_compiler']
 tests=[('material-retention-tests','ep_runtime','heat_balance::selected_material_data::air_gap_tests::air_gap_getter_retains_real_group_and_ronly_through_ctf_and_material_report'),('material-rejection-tests','ep_runtime','heat_balance::selected_material_data::air_gap_tests::air_gap_getter_rejects_missing_resistance_and_wrong_scalar_or_typed_custody'),('compiler-tests','ep_compiler','compiler::tests::window_material_simple_glazing_system::simple_glazing_system_rejects_nonpositive_source_derived_conductivity'),('source-caller-tests','ep_run','porting_scope::tests::interior_air_gap_preserves_raw_ip_typed_identity_and_source_support')]
 return [('format',['cargo','fmt',*packages]),('format-check',['cargo','fmt',*packages,'--','--check']),*[(key,['cargo','test','--locked','--offline','-p',package,'--lib',test,'--','--exact'])for key,package,test in tests],('affected-feature-check',['cargo','check','--locked','--offline','-p','ep_runtime','--all-targets','--features','native-controlled-air-cohort']),('clippy',['cargo','clippy','--locked','--offline',*packages,'--all-targets','--','-D','warnings'])]

def check_QA_profile(name,current,receipts,executions,final_snapshot):
 commands=qa_profile(name);require([row['name']for row in current]==[key for key,command in commands],'Source QA profile order differs')
 expected=[]
 for row,(key,command)in zip(current,commands,strict=True):
  r=receipts[row['receipt']['path']];require(r['command']==command,'Source QA profile command differs')
  if command[:2]==['cargo','test']:expected.append(command[-3])
 active=[item for item in executions if item['role']in ['current','reused']]
 require([item['test_names']for item in active]==[[name]for name in expected],'Source QA body profile differs')
 require(all(item['role']=='current'and item['lineage']==[]and receipts[item['receipt']['path']]['source_snapshot']==final_snapshot for item in active),'Required Source QA bodies must be current')
 return expected
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
  a=_authority_snapshot(r['source_snapshot'],pins);b=_authority_snapshot(final_snapshot,pins);path=source_path(body['path']);require(path in a and path in b,'Reused test file absent');old=auth(a[path]['archive']).read_bytes();new=auth(b[path]['archive']).read_bytes()
  x,y=old,new
  require(x==y and len(x)==body['bytes']and hashlib.sha256(x).hexdigest()==body['sha256'],'Reused complete test file changed')
  require(x.count(('fn '+body['test_name'].rsplit('::',1)[-1]+'(').encode())==1,'Reuse body/name mismatch');names.add(body['test_name'])
 require(names==set(test_names),'Reuse body coverage differs')



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
 require(mode in ['fresh','retained'],'Unknown authority mode');e=load(evidence_ref);exact_keys(e,EVIDENCE_KEYS,'SourceEvidence');require(e['schema']=='paired.SourceEvidence.v2','Unsupported Source evidence schema');qa_profile(e['QA_profile']);pins=_AuthorityPins();_authority_add(pins,evidence_ref)
 build_ref=e['build'];quality_ref=e['quality'];adoption_ref=e['adoption'];format_ref=e['format_review'];executions=e['executions'];historical=e['historical_refs'];stages=e['format_stages'];reuse=e['qa_reuse']
 require(e['retained_getter_review']is None,'This Source QA profile has no retained OA getter credit')
 for pin in [build_ref,quality_ref,adoption_ref,format_ref]:_authority_add(pins,pin)
 build=load(build_ref);br,members=c.inspect_snapshot(build,mode=='fresh');check_source_member_metadata(members);require(br['command']==_BUILD_COMMAND,'Default actual build command');_authority_add(pins,build['receipt']);_authority_add(pins,build['source_snapshot']);_authority_add(pins,build['retained_executable']);pins.snapshots[(build['source_snapshot']['path'],build['source_snapshot']['sha256'])]=members
 quality=load(quality_ref);require(quality['stage']=='quality','Actual quality ledger required');current=quality['runs'];all_runs=quality['all_actual_runs']
 require(quality['source_summaries'],'Complete underlying QA summaries required');observed=[]
 for pin in quality['source_summaries']:
  original=load(pin);require(original['stage']=='quality','Underlying QA summary stage');observed+=original['runs'];_authority_add(pins,pin)
 require(observed==all_runs,'Complete underlying QA attempt order differs')
 require(len({r['receipt']['path']for r in all_runs})==len(all_runs)and len({r['receipt']['sha256']for r in all_runs})==len(all_runs)and len({r['receipt']['path']for r in current})==len(current)and len({r['receipt']['sha256']for r in current})==len(current),'Duplicate QA attempt path/content')
 failed=0;receipts={}
 for row in all_runs:
  pin=row['receipt'];r=_receipt(pin);_authority_add(pins,pin);receipts[pin['path']]=r;_authority_snapshot(r['source_snapshot'],pins);bad=r['exit_code']!=0 or r['launch_error']is not None
  require(row['exit_code']==r['exit_code']and (row['recorder_exit']!=0 if bad else row['recorder_exit']==0),'QA actual exit/recorder disagreement');failed+=int(bad)
  for key in ['stdout','stderr']:_authority_add(pins,r[key])
 reuse_by={x['receipt']['path']:x for x in reuse};require(len(reuse_by)==len(reuse),'Duplicate QA reuse')
 execution_by={x['receipt']['path']:x for x in executions};require(len(execution_by)==len(executions)and len({x['receipt']['sha256']for x in executions})==len(executions),'Duplicate test execution receipt path/content')
 require(set(execution_by)=={row['receipt']['path']for row in all_runs if receipts[row['receipt']['path']]['exit_code']==0 and receipts[row['receipt']['path']]['launch_error']is None and receipts[row['receipt']['path']]['command'][:2]==['cargo','test']and '--exact'in receipts[row['receipt']['path']]['command']},'Successful actual test receipts need complete body accounting')
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
   require(item['receipt']in [v['receipt']for v in all_runs]and item['receipt']not in [v['receipt']for v in current]and item['lineage']==[],'Historical test absent from ledger or gets current credit');_authority_snapshot(r['source_snapshot'],pins);historical_bodies+=len(names)
  normalized.append(item)
 required_names=check_QA_profile(e['QA_profile'],current,receipts,normalized,build['source_snapshot'])
 adoption=load(adoption_ref);fmt=load(format_ref);require(adoption['accepted']is True and fmt['accepted']is True and fmt['schema']=='paired.SourceFormatReview.v2'and fmt['Source_adoption']==adoption_ref and fmt['current_source_snapshot']==build['source_snapshot'],'Source adoption/format lineage')
 require(type(adoption['files'])is list and all(type(row)is dict and {'path','before','after'}<=set(row)for row in adoption['files']),'Missing adopted Source endpoints');raw=[{key:row[key]for key in ['path','before','after']}for row in adoption['files']]
 baseline=load(adoption['actual_before_Source_build']);_authority_add(pins,adoption['actual_before_Source_build']);old=_authority_snapshot(baseline['source_snapshot'],pins)
 require(type(stages)is list and stages,'Complete Source stage chain required');cursor=baseline['source_snapshot'];declared=set()
 for edge in stages:
  require(edge['before_snapshot']==cursor,'Discontinuous Source stage chain');before,after=_authority_stage(edge,pins);cursor=edge['after_snapshot'];paths={row['path']for row in edge['endpoints']}
  if edge['kind']=='format':require(paths<=declared,'Formatter changed undeclared Source paths')
  else:declared.update(paths)
 require(cursor==build['source_snapshot']and stages[0]['kind']=='adopt'and stages[0]['review']==adoption_ref and stages[0]['endpoints']==raw,'Source stages/adoption do not reach final build')
 final_paths=source_delta(old,members);endpoints=fmt['source_endpoints'];check_source_endpoints(endpoints,old,members,pins)
 added=sorted(set(members)-set(old));deleted=sorted(set(old)-set(members));require(fmt['before_inventory_count']==len(old)and fmt['source_inventory_count']==len(members)and fmt['added_paths']==added and fmt['deleted_paths']==deleted,'Source membership accounting differs')
 formats=[edge for edge in stages if edge['kind']=='format'];require(formats and formats[-1]['receipt']==fmt['format_receipt'],'Final format receipt differs');require(all(edge['receipt']in [row['receipt']for row in all_runs]for edge in formats),'Format stage absent from actual QA')
 if mode=='fresh':
  for path in deleted:require(not os.path.lexists(ROOT/path),'Deleted Source lexical path still exists')
  for path in sorted(final_paths&set(members)):
   endpoint=dict(path=path,sha256=members[path]['archive']['sha256'],bytes=members[path]['size_bytes']);require(auth(endpoint).read_bytes()==auth(members[path]['archive']).read_bytes(),'Current Source endpoint differs');_authority_add(pins,endpoint)
 for packet in adoption['packets']:
  peer=load(packet['independent_review']);require(peer['accepted_static_only']is True and peer['proposal']==packet['proposal']and peer['findings']==[],'Unaccepted Source proposal review')
  for pin in packet.values():_authority_add(pins,pin)
 parent_keys=[k for k in adoption if k.startswith('actual_parent')and k.endswith('_publication')]
 for key in parent_keys:_authority_add(pins,adoption[key])
 for pin in historical:_authority_add(pins,pin)
 primary=e['current_Source_primary_review'];_authority_primary(primary,members,pins)
 return dict(schema='paired.SourceAuthority.v2',mode=mode,evidence=evidence_ref,build=build_ref,quality=quality_ref,adoption=adoption_ref,format_review=format_ref,snapshot=build['source_snapshot'],inventory_count=len(members),changed_paths=sorted(final_paths),source_endpoints=endpoints,before_inventory_count=len(old),added_paths=added,deleted_paths=deleted,QA_profile=e['QA_profile'],required_current_test_names=required_names,current_Source_primary_review=primary,current_QA_commands=len(current),total_actual_QA_commands=len(all_runs),failed_QA_attempts=failed,current_test_body_executions=current_bodies,reused_test_bodies=reused_bodies,historical_test_bodies=historical_bodies,retained_getter_body_executions=0,executions=normalized,historical_refs=historical,retained_getter_review=None,current_getter_credit=current_getter,pins=pins)

def load_authority(pin,mode):
 a=load(pin);require(a['schema']=='paired.SourceAuthority.v2'and a['mode']==mode,'Authority mode/schema differs');require(a==authority_from_evidence(a['evidence'],mode),'Normalized authority differs from underlying evidence');return a

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
 exact_keys(spec,{'prediction','overlays','signed_prediction','outdoor_role_positions'},'structure')
 prediction=load(spec['prediction']);expected=auth(prediction['expected_body']).read_bytes();restored=expected
 for stage in spec['overlays']:
  exact_keys(stage,{'before','after','forward','inverse','byte_edits'},'structural inverse stage');before=auth(stage['before']).read_bytes();after=auth(stage['after']).read_bytes()
  require(restored==after and verify_byte_edits(before,after,stage['byte_edits']),'Structural stage endpoint/byte edits')
  require(apply_unified(before,auth(stage['forward']).read_bytes())==after and apply_unified(after,auth(stage['inverse']).read_bytes())==before,'Structural forward/inverse patch');restored=before
 old=load(spec['signed_prediction']);require(restored==auth(old['expected_body']).read_bytes(),'Structural chain terminal differs')
 inverse=restored.replace(b'\r\n',b'\n').splitlines()
 for change in old['rename_fields']:
  at=change['body_line']-1;fields=inverse[at].split(b',');require(fields[change['field_index']]==bytes.fromhex(change['new_hex']),'Identifier inverse precondition');fields[change['field_index']]=change['old_ascii'].encode();inverse[at]=b','.join(fields)
 original=b'\n'.join(inverse)+b'\n'
 for engine in ['Native','Source']:
  raw=auth(old['baseline'][engine]).read_bytes();version,body=raw.split(b'\n',1);require(version.startswith(b'Program Version,')and original==body.replace(b'\r\n',b'\n'),'Whole accepted BND inverse')
 positions=spec['outdoor_role_positions'];require(len(positions)==2 and len(set(positions))==2 and all(type(i)is int and 0<=i<29 for i in positions),'Outdoor role positions')
 return prediction,expected
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
 binding=load(binding_ref);require(binding['schema']=='paired.RuntimeBinding.v1'and binding['mode']==mode,'Binding mode');release=load(binding['release']);require(release['schema']=='paired.Release.v2','Release schema')
 for key in ['Root_review','independent_review']:
  review=load(binding[key]);require(review['schema']=='paired.RuntimeReview.v1'and review['accepted']is True and review['release']==binding['release']and review['case_config']==binding['case_config']and review['source_authority']==binding['source_authority']and review['mode']==mode,'External review binding differs')
 case=load(binding['case_config']);exact_keys(case,CASE_KEYS,'CaseConfig');require(case['schema']=='paired.CaseConfig.v1'and case['id']==case_id and case['profile']==PROFILE and case['source_authority']==binding['source_authority'],'Case profile/identity');authority=load_authority(binding['source_authority'],mode)
 require(set(case['scientific'])==set(release['scientific_fields']),'Scientific input field set');document=verify_ledger(case);policy=load(case['policy']);counts=case['counts'];require(counts['total_series']==len(policy['rows'])+len(policy['meter_rows'])and counts['variable_values']==len(policy['rows'])*counts['zone_steps_per_variable']and counts['meter_values']==len(policy['meter_rows'])*counts['meter_hour_samples']and counts['total_values']==counts['variable_values']+counts['meter_values'],'Exact numeric dimensions')
 require(digest(typed({k:case['scientific'][k]for k in release['constant_scientific_fields']}))==release['constant_scientific_sha256'],'Frozen scientific contract changed')
 require(case['policy']==release['policy']and counts==release['counts']and counts['ESO_series']==len(policy['rows'])and counts['MTR_series']==len(policy['meter_rows']),'Frozen profile policies/dimensions differ')
 require(case['baseline_input']==release['baseline_input'],'Initial profile baseline differs')
 require(case['structure']['prediction']==case['scientific']['structural_prediction'],'Two structural authorities differ')
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
 sources(recipe)
 pp=load(recipe['primary_pins']);pins=[binding_ref,binding['release'],binding['case_config'],binding['source_authority'],binding['Root_review'],binding['independent_review']]+reference_closure(case)+list(release['files'].values())+authority['pins']
 for key in ['edit_ledger','clock']:
  pins+=reference_closure(load(case[key]))
 ledger=load(case['edit_ledger']);pins+=reference_closure(load(ledger['numeric_baseline']))
 pins+=reference_closure(load(case['aliases']['science_contract']))+[release['recorder']]
 for p in pp['Native_bundle'].values():auth(p);pins.append(p)
 unique={x['path']:ref(auth(x))for x in pins};require(len(source_rows)==authority['inventory_count'],'Actual Source inventory count')
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
