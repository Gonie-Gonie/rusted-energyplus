"""Authenticate a closed Source-metadata v2 release before any paired import."""
from pathlib import Path
import hashlib,importlib.util,json,sys,types

def load_package(release_pin):
 if not __debug__:raise RuntimeError('Assertions are required')
 home=Path(__file__).resolve().parent;root=home.parents[3]
 assert Path.cwd().resolve()==root and home==root/'tools/porting/paired/v2'
 def auth(pin):
  assert type(pin)is dict and {'path','sha256'}<=set(pin)<= {'path','sha256','bytes'}
  path=(root/pin['path']).resolve();assert path.is_relative_to(root)and path!=root;raw=path.read_bytes();assert hashlib.sha256(raw).hexdigest()==pin['sha256']and len(raw)==pin.get('bytes',len(raw));return path
 frozen_pin=dict(path='tools/porting/paired/v1/release.json',sha256='bc74f5e874b22bfb373ff2bcda55c8961ab078c284f774fe85276b64332cff61',bytes=7167)
 frozen=json.loads(auth(frozen_pin).read_bytes());release=json.loads(auth(release_pin).read_bytes())
 assert set(release)==set(frozen)|{'tools','frozen_v1_release','Source_metadata_schema','QA_profiles'}
 assert release['schema']=='paired.Release.v2'and type(release['version'])is int and release['version']==2 and release['frozen_v1_release']==frozen_pin and type(release['Source_metadata_schema'])is int and release['Source_metadata_schema']==2 and release['QA_profiles']==['simple_glazing.unused_material.v1']
 assert release['full_retained_migration_units']==release['partial_shared_kernel_units']==[]
 assert all(release[key]==frozen[key]for key in frozen if key not in ['schema','version','files','full_retained_migration_units','partial_shared_kernel_units'])
 shared={'common116.py','catalogue.py'};local={'__main__.py','contracts.py','runner.py'};assert set(release['files'])==shared|local
 for name,pin in release['files'].items():
  target=root/'tools/porting/paired/v1'/name if name in shared else home/name;assert auth(pin)==target
  if name in shared:assert pin==frozen['files'][name]
 assert all(release['files']['runner.py'][key]==frozen['files']['runner.py'][key]for key in ['sha256','bytes'])
 assert set(release['tools'])=={'source_production.py','coordinate.py','publish.py'}
 for name,pin in release['tools'].items():assert auth(pin)==home/name
 assert auth(release['recorder'])==root/'tools/porting/record_command.py'
 package=types.ModuleType('_frozen_paired_v2');package.__path__=[str(home)];sys.modules[package.__name__]=package
 for name in ['common116','contracts','catalogue','runner']:
  path=auth(release['files'][name+'.py']);fullname=package.__name__+'.'+name;spec=importlib.util.spec_from_file_location(fullname,path);assert spec and spec.loader
  module=importlib.util.module_from_spec(spec);sys.modules[fullname]=module;spec.loader.exec_module(module);setattr(package,name,module)
 return package,release,root

def bootstrap(argv):
 root=Path.cwd().resolve()
 def option(name):
  assert argv.count(name)==1;at=argv.index(name);assert at+1<len(argv);return argv[at+1]
 def pin(value):
  path,sha=value.rsplit('#',1);return dict(path=path,sha256=sha)
 if argv and argv[0]=='authority':release_pin=pin(option('--release'))
 else:
  binding_pin=pin(option('--bindings'));path=(root/binding_pin['path']).resolve();assert path.is_relative_to(root)and path!=root;raw=path.read_bytes();assert hashlib.sha256(raw).hexdigest()==binding_pin['sha256'];binding=json.loads(raw);assert binding['schema']=='paired.RuntimeBinding.v1';release_pin=binding['release']
 package,release,root=load_package(release_pin)
 return package.runner.main(argv)

if __name__=='__main__':raise SystemExit(bootstrap(sys.argv[1:]))
