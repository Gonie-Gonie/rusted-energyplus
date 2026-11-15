"""Root-owned generic Source QA/build/evidence producer; no import-time activity.

Root reviews and copies this exact text before invocation. Run at repository root.
Three phases use fresh directories; failures preserve their whole actual ledger.
No model engine, new test function, guessed unit/epoch or copied per-unit owner.
"""
from pathlib import Path
import argparse,hashlib,json,os,re,subprocess,sys,importlib.util
if not __debug__:raise RuntimeError('Assertions are required before metadata tool activity')

BUILD=['cargo','build','--locked','--offline','-p','ep_cli','--bin','eplus-rs','--message-format=json-render-diagnostics']

def load_metadata(release_pin):
 if not __debug__:raise RuntimeError('Assertions are required before release authentication/import')
 root=Path.cwd().resolve()
 def authenticate(pin):
  assert type(pin)is dict and {'path','sha256'}<=set(pin)<={'path','sha256','bytes'}
  path=(root/pin['path']).resolve();assert path.is_relative_to(root)and path!=root;raw=path.read_bytes();assert hashlib.sha256(raw).hexdigest()==pin['sha256']and len(raw)==pin.get('bytes',len(raw));return path
 release=json.loads(authenticate(release_pin).read_bytes());assert release['schema']=='paired.Release.v2'
 entry=authenticate(release['files']['__main__.py']);assert entry==root/'tools/porting/paired/v2/__main__.py'
 spec=importlib.util.spec_from_file_location('_source_metadata_v2_bootstrap',entry);assert spec and spec.loader;module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
 package,release,root=module.load_package(release_pin);assert authenticate(release['tools'][Path(__file__).name])==Path(__file__).resolve()
 return package,release,root

def main():
 if not __debug__:raise RuntimeError('Assertions are required')
 R=Path.cwd().resolve();assert(R/'Cargo.toml').is_file()and(R/'tools/porting/record_command.py').is_file()
 ap=argparse.ArgumentParser(description=__doc__);ap.add_argument('phase',choices=['quality','build','evidence']);ap.add_argument('--plan',required=True);ap.add_argument('--output-dir',required=True);ap.add_argument('--quality-preparation');ap.add_argument('--build-row');args=ap.parse_args()
 def inside(value):
  p=(R/value).resolve();assert p.is_relative_to(R)and p!=R;return p
 def ref(p):
  p=p.resolve();assert p.is_relative_to(R);b=p.read_bytes();return dict(path=p.relative_to(R).as_posix(),sha256=hashlib.sha256(b).hexdigest(),bytes=len(b))
 def auth(pin):
  assert type(pin)is dict and {'path','sha256'}<=pin.keys()and set(pin)<={'path','sha256','bytes'};p=inside(pin['path']);a=ref(p);assert a['sha256']==pin['sha256']and a['bytes']==pin.get('bytes',a['bytes']);return p
 def load(pin):return json.loads(auth(pin).read_bytes())
 def parse(value):
  path,sha=value.rsplit('#',1);return ref(auth(dict(path=path,sha256=sha)))
 def save(p,value):
  assert not p.exists();p.write_text(json.dumps(value,indent=2,ensure_ascii=False,allow_nan=False)+'\n',encoding='utf8',newline='\n');return ref(p)
 def snapshot(pin,current=False):return d.snapshot(pin,current)
 def changed(a,b):return d.source_delta(a,b)
 def endpoints(a,b):return d.source_endpoints(a,b)
 plan_pin=parse(args.plan);plan=load(plan_pin)
 assert set(plan)=={'schema','id','before_build','source_prepared_manifest','adoption','current_adoption','prior_format_stages','prior_quality_summaries','current_Source_primary_review','historical_refs','tools','release','QA_profile'}and plan['schema']=='paired.SourceProductionPlan.v2'and type(plan['id'])is str and plan['id']
 package,release,_=load_metadata(plan['release']);d=package.contracts;commands=d.qa_profile(plan['QA_profile'])
 assert set(plan['tools'])=={'recorder','freezer'};recorder=auth(plan['tools']['recorder']);freezer=auth(plan['tools']['freezer']);assert recorder==R/'tools/porting/record_command.py'and freezer==R/'tools/porting/freeze_rust_build.py'
 initial=load(plan['adoption']);active=load(plan['current_adoption']);assert initial['accepted']is active['accepted']is True and initial['actual_before_Source_build']==plan['before_build'];base=load(plan['before_build']);baseline=snapshot(base['source_snapshot'])
 assert any(p['manifest']==plan['source_prepared_manifest']for p in initial['packets'])
 root=inside(args.output_dir);assert root.is_relative_to(R/'.runtime/porting')and not root.exists();root.mkdir(parents=True)
 pins=[plan['release'],release['frozen_v1_release'],*release['files'].values(),*release['tools'].values(),ref(Path(__file__).resolve()),plan_pin,plan['before_build'],plan['source_prepared_manifest'],plan['adoption'],plan['current_adoption'],*plan['tools'].values()]
 unique={}
 for pin in pins:
  if pin['path']in unique:assert unique[pin['path']]['sha256']==pin['sha256']
  unique[pin['path']]=ref(auth(pin))
 pins=list(unique.values())
 env=os.environ.copy();env['PATH']=str(Path.home()/'.cargo/bin')+';'+str(R/'.runtime/reference-tools/winlibs-gcc13.2.0-ucrt-r3/mingw64/bin')+';'+env['PATH']
 def record(name,command):
  argv=[sys.executable,'-X','utf8','-B',str(recorder),'--output-dir',str(root/name)]
  for pin in pins:argv+=['--input',str(auth(pin))]
  child=subprocess.run([*argv,'--',*command],cwd=R,env=env,stdin=subprocess.DEVNULL,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
  (root/(name+'-recorder.stdout')).write_bytes(child.stdout);(root/(name+'-recorder.stderr')).write_bytes(child.stderr);rp=root/name/'receipt.json'
  if not rp.exists():raise RuntimeError('Recorder produced no receipt: '+name)
  receipt=load(ref(rp));assert receipt['schema']=='recorded-porting-command.v1'and receipt['command']==command and Path(receipt['cwd']).resolve()==R
  return dict(name=name,recorder_exit=child.returncode,receipt=ref(rp),exit_code=receipt['exit_code']),receipt
 def successful(row,r,mutable=False):
  assert row['recorder_exit']==row['exit_code']==r['exit_code']==0 and r['launch_error']is None
  if not mutable:assert r['source_bytes_match_before_and_after']is True and r['repository_before']==r['repository_after']
 def stage_check(edge):return d._authority_stage(edge,d._AuthorityPins())
 def preparation():
  p=load(parse(args.quality_preparation));assert p['schema']=='paired.SourceQualityPreparation.v2'and p['plan']==plan_pin;summary=load(p['quality']);assert[x['name']for x in summary['runs']]==[x[0]for x in commands]
  for row in summary['runs']:successful(row,load(row['receipt']),row['name']=='format')
  snapshot(p['formatted_snapshot'],True);return p,summary
 if args.phase=='quality':
  assert not args.quality_preparation and not args.build_row
  prefix=plan['prior_format_stages'];cursor=base['source_snapshot']
  for edge in prefix:assert edge['before_snapshot']==cursor;stage_check(edge);cursor=edge['after_snapshot']
  if prefix:assert prefix[0]['kind']=='adopt'and prefix[0]['review']==plan['adoption']
  else:assert plan['current_adoption']==plan['adoption']
  previous=snapshot(cursor);expected={p:row['archive']for p,row in previous.items()}
  actual_files={row['path']:row for row in active['files']};assert len(actual_files)==len(active['files'])
  for path,row in actual_files.items():
   d.source_path(path);assert(row['before']is None)==(path not in previous)
   if row['before']is not None:assert auth(row['before']).read_bytes()==auth(previous[path]['archive']).read_bytes()
   if row['after']is None:
    assert path in expected;del expected[path];assert not os.path.lexists(inside(path))
   else:expected[path]=row['after']
  paths=set((R/'crates').rglob('*.rs'))|set((R/'crates').rglob('Cargo.toml'));paths|={R/n for n in ['Cargo.toml','Cargo.lock','rust-toolchain.toml','.cargo/config.toml']if(R/n).is_file()};assert set(expected)=={path.relative_to(R).as_posix()for path in paths}
  for path,pin in expected.items():assert inside(path).read_bytes()==auth(pin).read_bytes()
  runs=[];receipts={};summary=root/'summary.json';raw=None;formatted=None;format_review=None;stages=None
  for name,command in commands:
   row,r=record(name,command);runs.append(row);receipts[name]=r;summary.write_text(json.dumps(dict(stage='quality',runs=runs),indent=2)+'\n',encoding='utf8',newline='\n');successful(row,r,name=='format')
   if name=='format':raw=r['source_snapshot'];rawrows=snapshot(raw);assert set(expected)==set(rawrows)and all(auth(expected[p]).read_bytes()==auth(rawrows[p]['archive']).read_bytes()for p in expected)
   elif name=='format-check':
    formatted=r['source_snapshot'];final=snapshot(formatted,True);assert set(rawrows)==set(final)and changed(rawrows,final)<={v['path']for v in active['files']}
    actual_files={v['path']:v for v in active['files']};incoming_paths=changed(previous,rawrows);assert incoming_paths==set(actual_files)
    incoming=dict(kind='correction'if prefix else'adopt',before_snapshot=cursor,after_snapshot=raw,endpoints=[dict(path=p,before=actual_files[p]['before'],after=actual_files[p]['after'])for p in sorted(incoming_paths)],receipt=None,review=plan['current_adoption']);stage_check(incoming)
    initial_raw={v['path']:v['after']for v in initial['files']}
    if not prefix:assert{v['path']:v['after']for v in incoming['endpoints']}==initial_raw
    fv=dict(schema='paired.SourceFormatReview.v2',accepted=True,Source_adoption=plan['adoption'],format_receipt=runs[0]['receipt'],current_source_snapshot=formatted,source_inventory_count=len(final),source_endpoints=endpoints(baseline,final),before_inventory_count=len(baseline),added_paths=sorted(set(final)-set(baseline)),deleted_paths=sorted(set(baseline)-set(final)),actual_raw_snapshot=raw,staged_prefix_explicit=True)
    format_review=save(root/'format-review.json',fv);fmtedge=dict(kind='format',before_snapshot=raw,after_snapshot=formatted,endpoints=endpoints(rawrows,final),receipt=runs[0]['receipt'],review=format_review);stage_check(fmtedge);stages=[*prefix,incoming,fmtedge]
   else:assert formatted is not None and r['source_snapshot']==formatted
  result=dict(schema='paired.SourceQualityPreparation.v2',plan=plan_pin,quality=ref(summary),formatted_snapshot=formatted,format_review=format_review,format_stages=stages)
  print(json.dumps(dict(preparation=save(root/'quality-preparation.json',result),passed=True)));return 0
 prep,quality=preparation()
 if args.phase=='build':
  assert not args.build_row;row,r=record('build',BUILD);save(root/'build-attempt.json',dict(stage='build',runs=[row]));successful(row,r);assert r['source_snapshot']==prep['formatted_snapshot'];cap=r['cargo_compiler_artifacts_capture'];items=[v for v in cap['rows']if v['cargo_message']['target']['name']=='eplus-rs'and v['cargo_message']['target']['kind']==['bin']];assert cap['requested']and cap['status']=='pass'and cap['error']is None and len(items)==1 and items[0]['cargo_message']['features']==[]and items[0]['cargo_message']['profile']['test']is False
  command=[sys.executable,'-X','utf8','-B',str(freezer),'--command-receipt',str(auth(row['receipt'])),'--output-dir',str(root/'frozen'),'--cli','--allow-dirty'];freeze_row,fr=record('freeze',command);save(root/'freeze-attempt.json',dict(stage='freeze',runs=[freeze_row]));successful(freeze_row,fr);assert fr['source_snapshot']==prep['formatted_snapshot']
  frozen=ref(root/'frozen/build-receipt.json');f=load(frozen);assert f['schema']=='porting-Rust-build.v1'and ref(auth(f['build_execution']))==row['receipt']and f['source_snapshot']==prep['formatted_snapshot'];build=dict(id=plan['id'],receipt=row['receipt'],source_snapshot=r['source_snapshot'],retained_executable=ref(auth(f['binary'])),frozen_build_receipt=frozen,quality_preparation=parse(args.quality_preparation))
  print(json.dumps(dict(build=save(root/'build-row.json',build),default_builds=1,engine_calls=0)));return 0
 assert args.phase=='evidence'and args.build_row;build_pin=parse(args.build_row);build=load(build_pin);assert build['quality_preparation']==parse(args.quality_preparation)and build['source_snapshot']==prep['formatted_snapshot'];br=load(build['receipt']);assert br['command']==BUILD and br['exit_code']==0 and br['launch_error']is None and br['source_bytes_match_before_and_after']is True and br['repository_before']==br['repository_after'];snapshot(build['source_snapshot'],True)
 summaries=[*plan['prior_quality_summaries'],prep['quality']];actual=[]
 for pin in summaries:
  q=load(pin);assert q['stage']=='quality';actual+=q['runs']
 assert len({v['receipt']['path']for v in actual})==len({v['receipt']['sha256']for v in actual})==len(actual);failed=0
 for row in actual:
  r=load(row['receipt']);bad=r['exit_code']!=0 or r['launch_error']is not None;assert row['exit_code']==r['exit_code']and(row['recorder_exit']!=0 if bad else row['recorder_exit']==0);failed+=int(bad)
 ledger=save(root/'quality-ledger.json',dict(stage='quality',runs=quality['runs'],all_actual_runs=actual,source_summaries=summaries));executions=[]
 for key,command in commands:
  if command[:2]!=['cargo','test']:continue
  assert command[-2:]==['--','--exact'];name=command[-3]
  row=next(v for v in quality['runs']if v['name']==key);r=load(row['receipt']);assert r['source_snapshot']==build['source_snapshot'];observed=re.findall(r'(?m)^test (\S+) \.\.\. ok\s*$',auth(r['stdout']).read_text('utf8'));assert observed==[name];executions.append(dict(kind='targeted_existing_test',role='current',test_names=[name],receipt=row['receipt'],actual_exit_code=0,lineage=[]))
 for row in actual:
  if row in quality['runs']:continue
  r=load(row['receipt']);command=r['command']
  if r['exit_code']!=0 or r['launch_error']is not None or command[:2]!=['cargo','test']:continue
  assert command[-2:]==['--','--exact']and r['source_bytes_match_before_and_after']is True
  name=command[-3];observed=re.findall(r'(?m)^test (\S+) \.\.\. ok\s*$',auth(r['stdout']).read_text('utf8'));assert observed==[name]
  executions.append(dict(kind='targeted_existing_test',role='historical',test_names=[name],receipt=row['receipt'],actual_exit_code=0,lineage=[]))
 for pin in plan['historical_refs']:auth(pin)
 evidence=dict(schema='paired.SourceEvidence.v2',QA_profile=plan['QA_profile'],build=build_pin,quality=ledger,adoption=plan['adoption'],format_review=prep['format_review'],executions=executions,current_QA_commands=len(quality['runs']),total_actual_QA_commands=len(actual),failed_QA_attempts=failed,historical_refs=plan['historical_refs'],retained_getter_review=None,format_stages=prep['format_stages'],current_Source_primary_review=plan['current_Source_primary_review'],qa_reuse=[])
 print(json.dumps(dict(evidence=save(root/'source-evidence.json',evidence),current_test_body_executions=sum(len(x['test_names'])for x in executions if x['role']=='current'),current_getter_credit=any(d._GETTER_TEST in x['test_names']for x in executions if x['role']=='current'),default_builds=1,engine_calls=0)));return 0

if __name__=='__main__':raise SystemExit(main())
