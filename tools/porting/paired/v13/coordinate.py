"""Root orchestration of the unchanged frozen CLI; no scientific reader code."""
from pathlib import Path
import argparse,sys,json,hashlib,subprocess,types,importlib.util,runpy,contextlib
if not __debug__:raise RuntimeError('Assertions are required before metadata tool activity')
def load_metadata(release_pin):
 if not __debug__:raise RuntimeError('Assertions are required before release authentication/import')
 root=Path.cwd().resolve()
 def authenticate(pin):
  assert type(pin)is dict and set(pin)=={'path','sha256','bytes'}and type(pin['path'])is str and pin['path']and '\\'not in pin['path']and ':'not in pin['path']and all(p and p not in ['.','..']for p in pin['path'].split('/'))and type(pin['sha256'])is str and len(pin['sha256'])==64 and all(c in '0123456789abcdef'for c in pin['sha256'])and type(pin['bytes'])is int and pin['bytes']>=0
  path=(root/pin['path']).resolve();assert path.is_relative_to(root)and path!=root;raw=path.read_bytes();assert hashlib.sha256(raw).hexdigest()==pin['sha256']and len(raw)==pin.get('bytes',len(raw));return path
 release=json.loads(authenticate(release_pin).read_bytes());assert release['schema']=='paired.Release.v13'
 entry=authenticate(release['files']['__main__.py']);assert entry==root/'tools/porting/paired/v13/__main__.py'
 spec=importlib.util.spec_from_file_location('_source_metadata_v13_bootstrap',entry);assert spec and spec.loader;module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
 package,release,root=module.load_package(release_pin);assert authenticate(release['tools'][Path(__file__).name])==Path(__file__).resolve()
 return package,release,root

R=Path.cwd().resolve();p=argparse.ArgumentParser();p.add_argument('phase',choices=['bind','validate','native','source','precheck','verify','complete']);p.add_argument('--home',required=True);p.add_argument('--release',required=True);p.add_argument('--Root-review');p.add_argument('--peer-review');args=p.parse_args();H=(R/args.home).resolve();assert H.is_relative_to(R/'.runtime/porting')and H.is_dir()
def ref(path):
 b=path.read_bytes();return dict(path=path.relative_to(R).as_posix(),sha256=hashlib.sha256(b).hexdigest(),bytes=len(b))
def auth(pin):
 path=(R/pin['path']).resolve();assert path.is_relative_to(R);actual=ref(path);assert actual['sha256']==pin['sha256']and actual['bytes']==pin.get('bytes',actual['bytes']);return path
def token(pin):return pin['path']+'#'+pin['sha256']
def read(path):return json.loads(path.read_bytes())
def save(path,value):
 assert not path.exists();path.write_text(json.dumps(value,indent=2)+'\n',encoding='utf8',newline='\n');return ref(path)
release_path,release_sha=args.release.rsplit('#',1);release_pin=ref(auth(dict(path=release_path,sha256=release_sha)));package,release,R=load_metadata(release_pin)
casepin=ref(H/'case-config.json');case=read(auth(casepin));authority=case['source_authority'];assert authority is not None
if args.phase=='bind':
 assert args.Root_review and args.peer_review;reviews=[ref((R/value).resolve())for value in [args.Root_review,args.peer_review]]
 for pin in reviews:
  review=read(auth(pin));assert review['schema']=='paired.RuntimeReview.v1'and review['accepted']is True and review['mode']=='fresh'and review['release']==release_pin and review['case_config']==casepin and review['source_authority']==authority
 result=save(H/'binding.json',dict(schema='paired.RuntimeBinding.v1',mode='fresh',release=release_pin,case_config=casepin,source_authority=authority,Root_review=reviews[0],independent_review=reviews[1]));print(json.dumps(dict(binding=result)));raise SystemExit(0)
assert not args.Root_review and not args.peer_review;binding=ref(H/'binding.json');assert read(auth(binding))['release']==release_pin;base=[sys.executable,'-X','utf8','-B',str(auth(release['files']['__main__.py']))];common=['--bindings',token(binding),'--case',case['id'],'--mode','fresh']
if args.phase in ['precheck','verify']:
 d,runner=package.contracts,package.runner;ctx=d.load_context(binding,case['id'],'fresh');actors=[ref(H/'native/launch.json')];output=H/('native-precheck.json'if args.phase=='precheck'else'readback.json');assert not output.exists()
 if args.phase=='verify':actors.append(ref(H/'source/launch.json'));command=runner.verify_command(ctx,*actors,output)
 else:command=base+['precheck']+common+['--native-launch',token(actors[0]),'--output',str(output)]
 recorder=auth(release['recorder']);argv=[str(recorder),'--output-dir',str(H/(args.phase+'-command'))]
 for pin in list(ctx.pins)+actors:argv+=['--input',str(auth(pin))]
 argv+=['--',*command];previous=sys.argv[:]
 with(H/(args.phase+'-recorder.stdout')).open('x',encoding='utf8')as out,(H/(args.phase+'-recorder.stderr')).open('x',encoding='utf8')as err,contextlib.redirect_stdout(out),contextlib.redirect_stderr(err):
  try:
   sys.argv=argv
   try:runpy.run_path(str(recorder),run_name='__main__')
   except SystemExit as exc:rc=exc.code
  finally:sys.argv=previous
 print(json.dumps(dict(phase=args.phase,exit_code=rc,output=ref(output)if output.exists()else None,receipt=ref(H/(args.phase+'-command/receipt.json')))));raise SystemExit(rc)
if args.phase in ['native','source']:
 command=base+['act']+common+['--engine',args.phase,'--run-dir',str(H/args.phase)]
 if args.phase=='source':command+=['--native-proof',token(ref(H/'native-precheck.json')),'--native-precheck-receipt',token(ref(H/'precheck-command/receipt.json'))]
elif args.phase=='complete':command=base+['complete']+common+['--paired',token(ref(H/'readback.json')),'--verification-receipt',token(ref(H/'verify-command/receipt.json')),'--output',str(H/'completed.json')]
else:assert args.phase=='validate';command=base+['validate']+common
with(H/(args.phase+'.stdout')).open('xb')as out,(H/(args.phase+'.stderr')).open('xb')as err:result=subprocess.run(command,cwd=R,stdin=subprocess.DEVNULL,stdout=out,stderr=err)
print(json.dumps(dict(phase=args.phase,exit_code=result.returncode)));raise SystemExit(result.returncode)
