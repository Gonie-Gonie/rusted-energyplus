"""Publish one Root-completed paired domain, with exact Source and Git custody."""
from pathlib import Path
import argparse,json,hashlib,subprocess,os,tomllib,importlib.util
if not __debug__:raise RuntimeError('Assertions are required before metadata tool activity')
def load_metadata(release_pin):
 if not __debug__:raise RuntimeError('Assertions are required before release authentication/import')
 root=Path.cwd().resolve()
 def authenticate(pin):
  assert type(pin)is dict and set(pin)=={'path','sha256','bytes'} and type(pin['bytes'])is int and pin['bytes']>=0
  path=(root/pin['path']).resolve();assert path.is_relative_to(root)and path!=root;raw=path.read_bytes();assert hashlib.sha256(raw).hexdigest()==pin['sha256']and len(raw)==pin.get('bytes',len(raw));return path
 release=json.loads(authenticate(release_pin).read_bytes());assert release['schema']=='paired.Release.v6'
 entry=authenticate(release['files']['__main__.py']);assert entry==root/'tools/porting/paired/v6/__main__.py'
 spec=importlib.util.spec_from_file_location('_source_metadata_v6_bootstrap',entry);assert spec and spec.loader;module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
 package,release,root=module.load_package(release_pin);assert authenticate(release['tools'][Path(__file__).name])==Path(__file__).resolve()
 return package,release,root

R=Path.cwd().resolve();p=argparse.ArgumentParser();p.add_argument('--home',required=True);p.add_argument('--release',required=True);p.add_argument('--parent',required=True);p.add_argument('--source-adoption',required=True);p.add_argument('--message',required=True);a=p.parse_args();H=(R/a.home).resolve();assert H.is_relative_to(R/'.runtime/porting');O=H/'publish01';assert not O.exists()
def read(path):return json.loads(path.read_bytes())
def ref(path):
 b=path.read_bytes();return dict(path=path.relative_to(R).as_posix(),sha256=hashlib.sha256(b).hexdigest(),bytes=len(b))
def auth(pin):
 path=(R/pin['path']).resolve();assert path.is_relative_to(R);actual=ref(path);assert actual['sha256']==pin['sha256']and actual['bytes']==pin.get('bytes',actual['bytes']);return path
release_path,release_sha=a.release.rsplit('#',1);release_pin=ref(auth(dict(path=release_path,sha256=release_sha)));package,release,R=load_metadata(release_pin);d=package.contracts
env=os.environ.copy();env['PATH']='C:/Users/GonieGonie/.cargo/bin;'+env['PATH']
queries=[]
def query(argv,input_data=None):
 assert argv[0]=='git';argv=['git','-c','status.renames=false','-c','diff.renames=false',*argv[1:]];result=subprocess.run(argv,cwd=R,env=env,capture_output=True,input=input_data)
 queries.append(dict(command=argv,exit_code=result.returncode,stdout_sha256=hashlib.sha256(result.stdout).hexdigest(),stdout_bytes=len(result.stdout),stderr_sha256=hashlib.sha256(result.stderr).hexdigest(),stderr_bytes=len(result.stderr)))
 assert result.returncode==0,(argv,result.stderr.decode('utf8'));return result.stdout
assert query(['git','branch','--show-current']).strip()==b'main'and query(['git','rev-parse','HEAD']).decode().strip()==a.parent and query(['git','ls-remote','origin','refs/heads/main']).decode().split()==[a.parent,'refs/heads/main']and not query(['git','diff','--cached','--name-only']).strip()
adoption=read(H/'docs-adoption01/receipt.json');assert adoption['applied']and adoption['new_qualifying_cases']==1 and adoption['new_qualifying_values']==75120
for pin in adoption['after']:auth(pin)
done=read(auth(adoption['completed']));root=read(auth(adoption['Root_scientific_review']));assert done['schema']=='paired.Completed.v1'and done['mode']=='fresh'and done['scientific_acceptance']and done['qualifying_cases']==1 and done['qualifying_values']==75120 and done['actual_Native_engine_calls']==done['actual_Source_engine_calls']==1 and root['accepted']and root['engine_reruns']==0
assert done['release']==release_pin
assert root['completion']==adoption['completed']and root['readback']==done['paired']and root['binding']==done['bindings']
case=read(auth(done['case_config']));assert type(case['case_kind'])is str and case['case_kind']and case['profile']==release['profile']and case['id']==done['case_id']and case['source_authority']==done['source_authority']
assert case['structure']['schema']==release['structure_schema']=='paired.OATopologyStructure.v1'
d.structure_baseline(case['structure'])
docs_proposal=read(auth(adoption['manifest']));fixed_manifest=read(auth(docs_proposal['conditional_application']['fixed_input_manifest']));assert fixed_manifest['input']==case['input']and docs_proposal['conditional_application']['case_kind']in [None,case['case_kind']]
authority=d.load_authority(done['source_authority'],'fresh');assert authority['schema']=='paired.SourceAuthority.v3'and authority['mode']=='fresh'
profile=read(auth(authority['QA_profile']));required_tests=authority['required_current_test_names'];assert required_tests==authority['required_test_names']==[body['test_name']for body in profile['required_bodies']]and authority['required_reused_test_names']==[]
build=read(auth(authority['build']));snapshot=read(auth(build['source_snapshot']));assert len(snapshot['files'])==authority['inventory_count']
current_executions=[item for item in authority['executions']if item['role']=='current']
assert authority['current_test_body_executions']==len(required_tests)>0 and authority['reused_test_bodies']==0
assert authority['current_getter_credit']==('heat_balance::ordinary_equipment::tests::actual_native_get_equipment_pair_and_input_owner_transitions'in required_tests)
assert [name for item in current_executions for name in item['test_names']]==required_tests
for row in snapshot['files']:assert(R/row['historical_path']).read_bytes()==auth(row['archive']).read_bytes()
quality=read(auth(authority['quality']));assert len(quality['runs'])==authority['current_QA_commands']and len(quality['all_actual_runs'])==authority['total_actual_QA_commands']
assert all(x['exit_code']==x['recorder_exit']==0 for x in quality['runs'])and sum(x['exit_code']!=0 for x in quality['all_actual_runs'])==authority['failed_QA_attempts']
commands=d.qa_profile(authority['QA_profile'])
assert authority['current_QA_commands']==len(commands)and [row['name']for row in quality['runs']]==[name for name,command in commands]
for row,(name,command)in zip(quality['runs'],commands,strict=True):
 receipt=read(auth(row['receipt']));assert receipt['command']==command and Path(receipt['cwd']).resolve()==R and receipt['exit_code']==0 and receipt['launch_error']is None
successful=[]
for row in quality['all_actual_runs']:
 receipt=read(auth(row['receipt']))
 if receipt['command'][:2]==['cargo','test']and receipt['exit_code']==0 and receipt['launch_error']is None:successful.append(row['receipt'])
assert sorted((pin['path'],pin['sha256'])for pin in successful)==sorted((item['receipt']['path'],item['receipt']['sha256'])for item in authority['executions'])
gates=read(H/'docs01/summary.json');assert[x['name']for x in gates['runs']]==['docs-generate','docs-check','spec-cross-check','project-contract-check']
for row in gates['runs']:
 receipt=read(auth(row['receipt']));assert row['recorder_exit']==row['exit_code']==receipt['exit_code']==0 and receipt['source_bytes_match_before_and_after']and receipt['repository_before']==receipt['repository_after']and receipt['source_snapshot']==build['source_snapshot']
path='docs/validation/evidence/object-input-domains.json';previous=json.loads(query(['git','show','HEAD:'+path]));current=read(R/path);old=previous['verification'];v=current['verification'];failed_refs=adoption.get('retained_failed_Source_builds',[]);assert type(failed_refs)is list;failed_builds=[read(auth(pin))for pin in failed_refs];new_builds=adoption.get('new_recorded_Source_builds',1+len(failed_builds));assert type(new_builds)is int and new_builds==1+len(failed_builds)
assert v['cases'][:-1]==old['cases']and v['source_builds'][:-new_builds]==old['source_builds']and v['source_builds'][-new_builds:-1]==failed_builds and v['cases'][-1]['id']==done['case_id']and v['source_builds'][-1]==build
assert v['retained_failed_Source_attempts'][:len(old['retained_failed_Source_attempts'])]==old['retained_failed_Source_attempts']
assert v['retained_recording_failures']==old['retained_recording_failures']
for key in ['retained_unaccepted_Source_observer_attempts','retained_unaccepted_Source_comparison_attempts']:
 assert v[key]==old[key]
provenance=v['cases'][-1]['comparison_provenance']
assert provenance['release']==release_pin and provenance['case_config']==done['case_config']and provenance['Source_authority']==done['source_authority']
assert not({'prior_failed_recording','recovery_plan','retained_recording_failure','unit_total_Native_engine_starts','unit_total_Source_engine_starts'}&set(provenance))
failed_attempts=adoption.get('retained_failed_Source_attempts',[]);assert type(failed_attempts)is list
added_failures=v['retained_failed_Source_attempts'][len(old['retained_failed_Source_attempts']):];assert len(added_failures)==len(failed_builds)==len(failed_attempts)
for row,pin,failed_build in zip(added_failures,failed_attempts,failed_builds,strict=True):
 failure=read(auth(pin));expected=dict(failure['canonical_failed_attempt']);expected['retained_failure']=pin;assert row==expected and failure['failed_Source_build']==failed_build and row['admitted_cases']==row['admitted_samples']==0 and row['accepted_pair']is False
assert len(v['cases'])==v['totals']['paired_cases']==old['totals']['paired_cases']+1 and len(v['source_builds'])==old['source_build_inventory_accounting']['actual_recorded_builds']+new_builds
assert v['totals']['reported_samples']==old['totals']['reported_samples']+75120 and v['totals']['raw_operand_unequal']==old['totals']['raw_operand_unequal']+root['raw_operand_unequal']and v['totals']['tolerance_failures']==v['totals']['rmse_failures']==0
account=v['source_build_inventory_accounting'];prior_account=old['source_build_inventory_accounting']
deltas={'actual_recorded_builds':new_builds,'builds_with_accepted_paired_cases':1,'builds_without_accepted_paired_cases':len(failed_builds),'builds_with_only_retained_failed_Source_attempts':len(failed_builds),'builds_with_only_unaccepted_Source_attempts':len(failed_builds)}
assert set(account)==set(prior_account)
for key,value in prior_account.items():
 if key!='qualification':assert account[key]==value+deltas.get(key,0)
source_adoption=(R/a.source_adoption).resolve();assert ref(source_adoption)==authority['adoption'];initial=read(source_adoption);assert initial['accepted']is True and initial['actual_parent_HEAD']==a.parent
baseline=read(auth(initial['actual_before_Source_build']));assert baseline['source_snapshot']==initial['actual_before_Source_snapshot']
source_publication=read(auth(initial['actual_parent_Source_publication']))
assert initial['actual_parent_Source_publication']==docs_proposal['actual_parent_publication']
assert source_publication['head']==source_publication['remote_head']and source_publication['worktree_clean']is True
assert initial['actual_before_Source_build']==source_publication['actual_Source_build']
if source_publication['head']==a.parent:
 assert 'intervening_Python_only_publication'not in initial
else:
 tooling_pin=initial['intervening_Python_only_publication'];tooling=read(auth(tooling_pin))
 assert tooling['schema']=='Root-paired-v6-tooling-publication.v1'and tooling['parent_HEAD']==source_publication['head']and tooling['head']==tooling['remote_head']==a.parent and tooling['worktree_clean']is True
 assert tooling['Source_held_build']==initial['actual_before_Source_build']and tooling['all_Source_current_archive_bytes_held']is True and tooling['Source_inventory_count']==len(d.snapshot(baseline['source_snapshot']))
 assert tooling['release']==release_pin and tooling['new_Source_builds']==tooling['new_qualifying_cases']==tooling['new_qualifying_values']==0
binding=read(auth(done['bindings']));runtime_review=read(auth(binding['Root_review']))
assert adoption['Root_runtime_review']==binding['Root_review']
assert runtime_review['schema']=='paired.RuntimeReview.v1'and runtime_review['accepted']is True and runtime_review['mode']=='fresh'
assert runtime_review['release']==release_pin and runtime_review['case_config']==done['case_config']and runtime_review['source_authority']==done['source_authority']
assert runtime_review['fixed_input_manifest']==docs_proposal['conditional_application']['fixed_input_manifest']
input_review_pin=runtime_review['independent_input_review'];input_review=read(auth(input_review_pin))
assert adoption['independent_input_review']==input_review_pin and docs_proposal['conditional_application']['independent_input_review']in [None,input_review_pin]
assert input_review['accepted_static_only']is True and input_review['findings']==[]and runtime_review['fixed_input_manifest']in input_review.values()
evidence=read(auth(authority['evidence']));stages=evidence['format_stages']
assert stages[0]['kind']=='adopt'and stages[0]['review']==authority['adoption']and stages[0]['before_snapshot']==baseline['source_snapshot']
corrections=[edge['review']for edge in stages if edge['kind']=='correction']
assert corrections==adoption['correction_reviews']
old_members=d.snapshot(baseline['source_snapshot']);members={row['historical_path']:row for row in snapshot['files']}
assert len(members)==len(snapshot['files'])==authority['inventory_count']and len(old_members)==authority['before_inventory_count']
changed=sorted(d.source_delta(old_members,members))
assert authority['changed_paths']==changed
assert authority['added_paths']==sorted(set(members)-set(old_members))and authority['deleted_paths']==sorted(set(old_members)-set(members))
assert profile['declared_Source_changes']==sorted({row['path']for edge in stages if edge['kind']!='format'for row in edge['endpoints']})
assert set(changed)<=set(profile['declared_Source_changes'])and stages[-1]['after_snapshot']==build['source_snapshot']
source=authority['source_endpoints'];assert source==d.source_endpoints(old_members,members)
canonical=[x['path']for x in adoption['after']];assert len(canonical)==4 and set(canonical)=={'docs/validation/evidence/object-input-domains.json','docs/src/current/current-status.md','docs/src/porting-map/heat-balance-source-map.md','specs/object_coverage.toml'}
before_spec=tomllib.loads(query(['git','show','HEAD:specs/object_coverage.toml']).decode('utf8'));after_spec=tomllib.loads((R/'specs/object_coverage.toml').read_text('utf8'))
registry_before=len(before_spec['object']);registry_after=len(after_spec['object'])
assert [row['name']for row in before_spec['object']]==[row['name']for row in after_spec['object']if row['name']in {r['name']for r in before_spec['object']}]
old_registry={row['name']:row for row in before_spec['object']};new_registry={row['name']:row for row in after_spec['object']}
assert len(old_registry)==registry_before and len(new_registry)==registry_after and set(old_registry)<=set(new_registry)
registry_changes=docs_proposal['registry_changes'];assert type(registry_changes)is list and registry_changes
assert [row['name']for row in registry_changes]==sorted({row['name']for row in registry_changes})
assert set(row['name']for row in registry_changes)=={name for name in new_registry if old_registry.get(name)!=new_registry[name]}
for change in registry_changes:
 assert set(change)=={'name','before','after'}and change['before']==old_registry.get(change['name'])and change['after']==new_registry[change['name']]
 if change['before']is not None:
  allowed={'source_admission','source_boundary','source_refs','support_boundary'}
  assert {k:v for k,v in change['before'].items()if k not in allowed}=={k:v for k,v in change['after'].items()if k not in allowed}
docs_peer=read(auth(adoption['independent_review']));assert docs_peer['accepted']is docs_peer['accepted_static_only']is True and docs_peer['findings']==[]and docs_peer['proposal']==adoption['manifest']
assert current['current_object_union']==previous['current_object_union']and current['current_object_union']['combined_canonical_types']==116
assert adoption['registry_before']==registry_before and adoption['registry_after']==registry_after and adoption['original_example_union']==116

generated=['docs/src/generated/example-object-union.md','docs/src/generated/object-coverage.md']
source_paths=[row['path']for row in source]
for path in source_paths:
 assert type(path)is str and Path(path).as_posix()==path and not Path(path).is_absolute()and '..'not in Path(path).parts and (R/path).resolve().is_relative_to(R)
files=source_paths+canonical+generated;assert len(files)==len(set(files))
def head_entry(path):
 raw=query(['git','ls-tree','--full-name','-z','HEAD','--',path]);parts=raw.split(b'\0');assert parts[-1]==b'';entries=[row for row in parts if row];assert len(entries)<=1
 if not entries:return None
 meta,name=entries[0].split(b'\t',1);mode,kind,oid=meta.split();assert kind==b'blob'and name.decode('utf8')==path;return oid.decode('ascii')
def git_canonical(raw,path):
 attrs_raw=query(['git','check-attr','-z','text','eol','filter','working-tree-encoding','--',path]).split(b'\0');assert attrs_raw[-1]==b'';attrs_raw=attrs_raw[:-1];assert len(attrs_raw)==12
 attrs={}
 for at in range(0,len(attrs_raw),3):
  owner,key,value=attrs_raw[at:at+3];assert owner.decode()==path;attrs[key.decode()]=value.decode()
 assert attrs['text']in ['auto','set','unset']and attrs['eol']=='lf'and attrs['filter']=='unspecified'and attrs['working-tree-encoding']=='unspecified',attrs
 raw.decode('utf8');assert b'\0'not in raw
 canonical=raw if attrs['text']=='unset'else raw.replace(b'\r\n',b'\n');oid=query(['git','hash-object','--path='+path,'--stdin'],raw).decode().strip();assert query(['git','hash-object','--stdin'],canonical).decode().strip()==oid
 return canonical,oid,attrs
git_custody=[]
expected_status={};expected=[];deleted=[]
for row in source:
 path=row['path'];old_blob=head_entry(path);assert(old_blob is None)==(row['before']is None)
 if row['before']is not None:
  before_bytes,before_oid,before_attrs=git_canonical(auth(row['before']).read_bytes(),path);assert old_blob==before_oid and query(['git','cat-file','blob',old_blob])==before_bytes
 if row['after']is None:
  assert not os.path.lexists(R/path);deleted.append(dict(path=path,before=row['before']));expected_status[path]='D'
 else:
  pin=ref(R/path);assert auth(row['after']).read_bytes()==auth(pin).read_bytes();expected.append(pin);expected_status[path]='A'if row['before']is None else'M'
for path in canonical+generated:
 pin=ref(R/path);expected.append(pin);blob=head_entry(path)
 if blob is None:expected_status[path]='A'
 elif query(['git','cat-file','blob',blob])!=(R/path).read_bytes():expected_status[path]='M'
status=query(['git','status','--porcelain=v1','-z','--untracked-files=all']).decode('utf8').split('\0');actual={}
for row in status:
 if not row:continue
 assert len(row)>=4 and row[2]==' 'and 'R'not in row[:2]and'C'not in row[:2];path=row[3:];assert path not in actual;actual[path]=row[:2]
assert set(actual)==set(expected_status)
for path,state in actual.items():
 required=expected_status[path];assert(state=='??'if required=='A'else state==' '+required),(path,state,required)
assert tomllib.loads((R/'docs/book.toml').read_text('utf8'))['build']['build-dir']=='book';book=(R/'docs/book').resolve();assert book==(R/'docs').resolve()/'book'and book.is_relative_to(R/'docs')
O.mkdir();runs=[]
def run(name,argv):
 result=subprocess.run(argv,cwd=R,env=env,capture_output=True);runs.append(dict(name=name,command=argv,exit_code=result.returncode,stdout_sha256=hashlib.sha256(result.stdout).hexdigest(),stdout_bytes=len(result.stdout),stderr_sha256=hashlib.sha256(result.stderr).hexdigest(),stderr_bytes=len(result.stderr)));(O/'commands.json').write_text(json.dumps(runs,indent=2)+'\n',encoding='utf8',newline='\n');print(json.dumps(dict(name=name,exit_code=result.returncode)),flush=True);assert result.returncode==0
run('book-clean',['mdbook','clean','docs']);run('diff-check',['git','diff','--check']);run('stage',['git','add','--all','--',*files])
raw=query(['git','diff','--cached','--name-status','--no-renames','-z']);parts=raw.split(b'\0');assert parts[-1]==b'';parts=parts[:-1];assert len(parts)%2==0;staged={}
for index in range(0,len(parts),2):
 kind,path=parts[index].decode('ascii'),parts[index+1].decode('utf8');assert kind in ['A','M','D']and path not in staged;staged[path]=kind
assert staged==expected_status
for row in deleted:assert query(['git','ls-files','--stage','-z','--',row['path']])==b''and not os.path.lexists(R/row['path'])
for pin in expected:
 raw_source=auth(pin).read_bytes();canonical,oid,attrs=git_canonical(raw_source,pin['path']);blob=query(['git','show',':'+pin['path']]);assert blob==canonical and query(['git','rev-parse',':'+pin['path']]).decode().strip()==oid,pin['path'];git_custody.append(dict(path=pin['path'],actual_Source_bytes=pin,Git_blob=oid,Git_canonical_sha256=hashlib.sha256(blob).hexdigest(),Git_canonical_bytes=len(blob),attributes=attrs,CRLF_only_builtin_normalization=raw_source!=blob))
run('cached-check',['git','diff','--cached','--check']);run('commit',['git','commit','-m',a.message]);head=query(['git','rev-parse','HEAD']).decode().strip();run('push',['git','push','origin','main'])
assert query(['git','ls-remote','origin','refs/heads/main']).decode().split()==[head,'refs/heads/main']and not query(['git','status','--porcelain']).strip()
result=dict(schema='Root-paired-domain-publication.v2',head=head,remote_head=head,parent=a.parent,worktree_clean=True,files=expected,deleted_files=deleted,Git_canonical_custody=git_custody,retained_failed_Source_builds=failed_refs,new_recorded_Source_builds=new_builds,Source_endpoints=source,baseline_Source_build=initial['actual_before_Source_build'],release=release_pin,required_current_tests=required_tests,metadata_loader=ref(R/'tools/porting/paired/v6/__main__.py'),commands=runs,Git_query_hashes=queries,initial_Source_publication=initial['actual_parent_Source_publication'],Root_runtime_review=binding['Root_review'],independent_input_review=input_review_pin,correction_reviews=corrections,registry_before=registry_before,registry_after=registry_after,original_example_union=116,documentation_adoption=ref(H/'docs-adoption01/receipt.json'),documentation_gates=ref(H/'docs01/summary.json'),Source_adoption=ref(source_adoption),actual_Source_build=authority['build'],Source_authority=done['source_authority'],Root_scientific_review=adoption['Root_scientific_review'],completed=adoption['completed'],accepted_totals=v['totals'],Source_build_accounting=v['source_build_inventory_accounting'],new_qualifying_cases=1,new_qualifying_values=75120,actual_Native_engine_calls=1,actual_Source_engine_calls=1,engine_reruns=0,current_test_body_executions=authority['current_test_body_executions'],historical_test_body_executions=authority['historical_test_bodies'],actual_QA_commands=authority['total_actual_QA_commands'],failed_QA_attempts=authority['failed_QA_attempts'],new_test_functions=0,full_object_support=False)
receipt=O/'receipt.json';receipt.write_text(json.dumps(result,indent=2)+'\n',encoding='utf8',newline='\n');print(json.dumps(dict(publication=ref(receipt),head=head,worktree_clean=True)),flush=True)
