"""Publish one Root-completed paired domain, with exact Source and Git custody."""
from pathlib import Path
import argparse,json,hashlib,subprocess,os,tomllib,importlib.util
if not __debug__:raise RuntimeError('Assertions are required before metadata tool activity')
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

R=Path.cwd().resolve();p=argparse.ArgumentParser();p.add_argument('--home',required=True);p.add_argument('--release',required=True);p.add_argument('--parent',required=True);p.add_argument('--source-adoption',required=True);p.add_argument('--message',required=True);a=p.parse_args();H=(R/a.home).resolve();assert H.is_relative_to(R/'.runtime/porting');O=H/'publish01';assert not O.exists()
def read(path):return json.loads(path.read_bytes())
def ref(path):
 b=path.read_bytes();return dict(path=path.relative_to(R).as_posix(),sha256=hashlib.sha256(b).hexdigest(),bytes=len(b))
def auth(pin):
 path=(R/pin['path']).resolve();assert path.is_relative_to(R);actual=ref(path);assert actual['sha256']==pin['sha256']and actual['bytes']==pin.get('bytes',actual['bytes']);return path
release_path,release_sha=a.release.rsplit('#',1);release_pin=ref(auth(dict(path=release_path,sha256=release_sha)));package,release,R=load_metadata(release_pin);d=package.contracts
env=os.environ.copy();env['PATH']='C:/Users/GonieGonie/.cargo/bin;'+env['PATH']
def query(argv):
 assert argv[0]=='git';argv=['git','-c','status.renames=false','-c','diff.renames=false',*argv[1:]];result=subprocess.run(argv,cwd=R,env=env,capture_output=True);assert result.returncode==0,(argv,result.stderr.decode('utf8'));return result.stdout
assert query(['git','branch','--show-current']).strip()==b'main'and query(['git','rev-parse','HEAD']).decode().strip()==a.parent and query(['git','ls-remote','origin','refs/heads/main']).decode().split()==[a.parent,'refs/heads/main']and not query(['git','diff','--cached','--name-only']).strip()
adoption=read(H/'docs-adoption01/receipt.json');assert adoption['applied']and adoption['new_qualifying_cases']==1 and adoption['new_qualifying_values']==75120
for pin in adoption['after']:auth(pin)
done=read(auth(adoption['completed']));root=read(auth(adoption['Root_scientific_review']));assert done['schema']=='paired.Completed.v1'and done['mode']=='fresh'and done['scientific_acceptance']and done['qualifying_cases']==1 and done['qualifying_values']==75120 and done['actual_Native_engine_calls']==done['actual_Source_engine_calls']==1 and root['accepted']and root['engine_reruns']==0
assert done['release']==release_pin
authority=d.load_authority(done['source_authority'],'fresh');assert authority['QA_profile']in release['QA_profiles']
build=read(auth(authority['build']));snapshot=read(auth(build['source_snapshot']));assert len(snapshot['files'])==authority['inventory_count']
assert authority['current_test_body_executions']==len(authority['required_current_test_names'])==4 and authority['reused_test_bodies']==0 and authority['current_getter_credit']is False
for row in snapshot['files']:assert(R/row['historical_path']).read_bytes()==auth(row['archive']).read_bytes()
quality=read(auth(authority['quality']));assert len(quality['runs'])==authority['current_QA_commands']and len(quality['all_actual_runs'])==authority['total_actual_QA_commands']
assert all(x['exit_code']==x['recorder_exit']==0 for x in quality['runs'])and sum(x['exit_code']!=0 for x in quality['all_actual_runs'])==authority['failed_QA_attempts']
gates=read(H/'docs01/summary.json');assert[x['name']for x in gates['runs']]==['docs-generate','docs-check','spec-cross-check','project-contract-check']
for row in gates['runs']:
 receipt=read(auth(row['receipt']));assert row['recorder_exit']==row['exit_code']==receipt['exit_code']==0 and receipt['source_bytes_match_before_and_after']and receipt['repository_before']==receipt['repository_after']and receipt['source_snapshot']==build['source_snapshot']
path='docs/validation/evidence/object-input-domains.json';previous=json.loads(query(['git','show','HEAD:'+path]));current=read(R/path);old=previous['verification'];v=current['verification'];assert v['cases'][:-1]==old['cases']and v['source_builds'][:-1]==old['source_builds']and v['cases'][-1]['id']==done['case_id']and v['source_builds'][-1]==build
assert len(v['cases'])==v['totals']['paired_cases']==old['totals']['paired_cases']+1 and len(v['source_builds'])==old['source_build_inventory_accounting']['actual_recorded_builds']+1
assert v['totals']['reported_samples']==old['totals']['reported_samples']+75120 and v['totals']['raw_operand_unequal']==old['totals']['raw_operand_unequal']+root['raw_operand_unequal']and v['totals']['tolerance_failures']==v['totals']['rmse_failures']==0
source_adoption=(R/a.source_adoption).resolve();assert ref(source_adoption)==authority['adoption'];source=authority['source_endpoints'];canonical=[x['path']for x in adoption['after']];generated=['docs/src/generated/example-object-union.md','docs/src/generated/object-coverage.md']
source_paths=[d.source_path(row['path'])for row in source];files=source_paths+canonical+generated;assert len(files)==len(set(files))
def head_entry(path):
 raw=query(['git','ls-tree','--full-name','-z','HEAD','--',path]);parts=raw.split(b'\0');assert parts[-1]==b'';entries=[row for row in parts if row];assert len(entries)<=1
 if not entries:return None
 meta,name=entries[0].split(b'\t',1);mode,kind,oid=meta.split();assert kind==b'blob'and name.decode('utf8')==path;return oid.decode('ascii')
expected_status={};expected=[];deleted=[]
for row in source:
 path=row['path'];old_blob=head_entry(path);assert(old_blob is None)==(row['before']is None)
 if row['before']is not None:assert query(['git','cat-file','blob',old_blob])==auth(row['before']).read_bytes()
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
 result=subprocess.run(argv,cwd=R,env=env,capture_output=True);out=O/(name+'.stdout');err=O/(name+'.stderr');out.write_bytes(result.stdout);err.write_bytes(result.stderr);runs.append(dict(name=name,command=argv,exit_code=result.returncode,stdout=ref(out),stderr=ref(err)));(O/'commands.json').write_text(json.dumps(runs,indent=2)+'\n',encoding='utf8',newline='\n');print(json.dumps(dict(name=name,exit_code=result.returncode)),flush=True);assert result.returncode==0
run('book-clean',['mdbook','clean','docs']);run('diff-check',['git','diff','--check']);run('stage',['git','add','--all','--',*files])
raw=query(['git','diff','--cached','--name-status','--no-renames','-z']);parts=raw.split(b'\0');assert parts[-1]==b'';parts=parts[:-1];assert len(parts)%2==0;staged={}
for index in range(0,len(parts),2):
 kind,path=parts[index].decode('ascii'),parts[index+1].decode('utf8');assert kind in ['A','M','D']and path not in staged;staged[path]=kind
assert staged==expected_status
for row in deleted:assert query(['git','ls-files','--stage','-z','--',row['path']])==b''and not os.path.lexists(R/row['path'])
for pin in expected:
 blob=query(['git','show',':'+pin['path']]);assert hashlib.sha256(blob).hexdigest()==pin['sha256']and len(blob)==pin['bytes'],pin['path']
run('cached-check',['git','diff','--cached','--check']);run('commit',['git','commit','-m',a.message]);head=query(['git','rev-parse','HEAD']).decode().strip();run('push',['git','push','origin','main'])
assert query(['git','ls-remote','origin','refs/heads/main']).decode().split()==[head,'refs/heads/main']and not query(['git','status','--porcelain']).strip()
result=dict(schema='Root-paired-domain-publication.v2',head=head,remote_head=head,parent=a.parent,worktree_clean=True,files=expected,deleted_files=deleted,Source_endpoints=source,release=release_pin,QA_profile=authority['QA_profile'],required_current_test_names=authority['required_current_test_names'],commands=runs,documentation_adoption=ref(H/'docs-adoption01/receipt.json'),documentation_gates=ref(H/'docs01/summary.json'),Source_adoption=ref(source_adoption),actual_Source_build=authority['build'],Source_authority=done['source_authority'],Root_scientific_review=adoption['Root_scientific_review'],completed=adoption['completed'],accepted_totals=v['totals'],Source_build_accounting=v['source_build_inventory_accounting'],new_qualifying_cases=1,new_qualifying_values=75120,actual_Native_engine_calls=1,actual_Source_engine_calls=1,engine_reruns=0,current_test_body_executions=authority['current_test_body_executions'],historical_test_body_executions=authority['historical_test_bodies'],actual_QA_commands=authority['total_actual_QA_commands'],failed_QA_attempts=authority['failed_QA_attempts'],new_test_functions=0,full_object_support=False)
receipt=O/'receipt.json';receipt.write_text(json.dumps(result,indent=2)+'\n',encoding='utf8',newline='\n');print(json.dumps(dict(publication=ref(receipt),head=head,worktree_clean=True)),flush=True)
