"""Authenticate the complete local release before importing any paired module.

Invoke this file directly with Python -X utf8 -B. No parent __init__.py runs.
"""
from pathlib import Path
import hashlib,importlib.util,json,sys,types

def bootstrap(argv):
 if not __debug__:raise RuntimeError('Optimized Python disables required assertions')
 assert __debug__;home=Path(__file__).resolve().parent;root=home.parents[3]
 assert Path.cwd().resolve()==root and home==root/'tools/porting/paired/v1'
 def option(name):
  assert argv.count(name)==1,'Exactly one '+name+' required';i=argv.index(name);assert i+1<len(argv);return argv[i+1]
 def authenticate(pin):
  assert type(pin)is dict and {'path','sha256'}<=pin.keys()and set(pin)<={'path','sha256','bytes'}
  p=(root/pin['path']).resolve();assert p.is_relative_to(root)and p!=root;b=p.read_bytes();assert hashlib.sha256(b).hexdigest()==pin['sha256']and len(b)==pin.get('bytes',len(b));return p
 def parse(value):
  path,sha=value.rsplit('#',1);return dict(path=path,sha256=sha)
 if argv and argv[0]=='authority':release_pin=parse(option('--release'))
 else:
  binding=json.loads(authenticate(parse(option('--bindings'))).read_bytes());assert binding['schema']=='paired.RuntimeBinding.v1';release_pin=binding['release']
 release=json.loads(authenticate(release_pin).read_bytes());assert release['schema']=='paired.Release.v1'and release['profile']=='ideal_loads.basic_positive_oa.v1'
 names={'common116.py','contracts.py','catalogue.py','runner.py','__main__.py'};assert set(release['files'])==names
 for name,pin in release['files'].items():assert authenticate(pin)==home/name
 assert authenticate(release['recorder'])==root/'tools/porting/record_command.py'
 # Only these authenticated modules enter the private package namespace.
 package=types.ModuleType('_frozen_paired_v1');package.__path__=[str(home)];sys.modules[package.__name__]=package
 for name in ['common116','contracts','catalogue','runner']:
  fullname=package.__name__+'.'+name;spec=importlib.util.spec_from_file_location(fullname,home/(name+'.py'));assert spec and spec.loader
  module=importlib.util.module_from_spec(spec);sys.modules[fullname]=module;spec.loader.exec_module(module);setattr(package,name,module)
 return package.runner.main(argv)

if __name__=='__main__':raise SystemExit(bootstrap(sys.argv[1:]))
