"""Byte authentication only; this writer never runs a comparison or engine."""
from __future__ import annotations
import hashlib
import json
from datetime import datetime, timezone
from pathlib import Path

ROOT=Path(__file__).resolve().parents[4]
BASE='.runtime/porting/CLK-03/'
PLAN='energyplus_porting_plan/'
COMMIT='df88eb4c7b03cd8a04ad09d5fce3300a4d5b7b22'
TREE='47e544dafabcec8e30197597b8832ee1c290c8fc'
SNAPSHOT='ca7b7f6bbbc8954f07f5efad2b5a26af93303b9a1ee1a43bf605567386ad3ca4'
GATES=('scope_review','unit_gate','integration_gate','production_gate')
CONTRACTS={
    'source':'da6eabfb07236e6303df15ede7e35a4254a2499369ade283a4ebaecfc3cd561d',
    'cases':'44d2f61ce68ed13c704e292dc9b25d3e8c6c8f871185027ac88cb3afaa9482c5',
    'tolerances':'fc2b509c35e415f50621cc8f144922ba5b7888ba697e74862298a1a0efb5b98e',
}
SEEN={}

def require(ok,message):
    if not ok:raise ValueError(message)

def path_of(value):
    require(isinstance(value,(str,Path)),'Typed path required')
    path=Path(value);path=(path if path.is_absolute() else ROOT/path).resolve()
    require(path.is_relative_to(ROOT),'Path escapes repository')
    return path

def ref(value):
    path=path_of(value)
    if path not in SEEN:
        with path.open('rb') as stream:SEEN[path]=hashlib.file_digest(stream,'sha256').hexdigest()
    return {'path':path.relative_to(ROOT).as_posix(),'sha256':SEEN[path]}

def bound(value):
    require(type(value) is dict and set(value)=={'path','sha256'},'Exact artifact binding required')
    require(ref(value['path'])==value,'Bound bytes differ: '+str(value['path']))
    return dict(value)

def decode(text):
    def pairs(items):
        out={}
        for key,item in items:
            require(key not in out,'Duplicate JSON key');out[key]=item
        return out
    def invalid(token):raise ValueError('Nonstandard JSON token: '+token)
    return json.loads(text,object_pairs_hook=pairs,parse_constant=invalid)

def read(value,sha=None):
    binding=ref(value)
    require(sha is None or sha==binding['sha256'],'Pinned proof differs: '+str(value))
    return decode(path_of(value).read_text(encoding='utf8'))

def artifacts(value):
    if type(value) is dict:
        if set(value)=={'path','sha256'}:bound(value)
        else:
            for item in value.values():artifacts(item)
    elif type(value) is list:
        for item in value:artifacts(item)

def exact(left,right):
    if type(left) is not type(right):return False
    if type(left) is dict:return set(left)==set(right) and all(exact(left[k],right[k]) for k in left)
    if type(left) is list:return len(left)==len(right) and all(exact(a,b) for a,b in zip(left,right))
    if type(left) is float:return left.hex()==right.hex()
    return left==right

def timestamp(value):
    stamp=datetime.fromisoformat(value)
    require(stamp.tzinfo is not None,'Timezone-aware actual timestamp required')
    return stamp

def now():return datetime.now(timezone.utc).isoformat()

def recorded(value,exit_code=0):
    receipt=read(value)
    require(receipt['schema']=='recorded-porting-command.v1','Actual recorder required')
    require(type(receipt['exit_code']) is int and (exit_code is None or receipt['exit_code']==exit_code)
        and receipt['launch_error'] is None,'Actual command outcome differs')
    require(receipt['source_bytes_match_before_and_after'] is True,'Command source guard failed')
    require(receipt['repository_before']['head']==receipt['repository_after']['head'],'Command HEAD changed')
    require(receipt['recorder_updates_gates'] is False and receipt['recorder_supplies_reference_answers'] is False,'Recorder policy differs')
    for key in ['stdout','stderr','source_snapshot']:bound(receipt[key])
    bound(receipt['executed_launcher']['archive'])
    require(timestamp(receipt['started_utc'])<=timestamp(receipt['finished_utc']),'Actual command chronology differs')
    return receipt

def reported(report_path,receipt_path):
    report=read(report_path);receipt=recorded(receipt_path)
    require(exact(report,read(receipt['stdout']['path'])),'Actual JSON stdout differs from report')
    require(exact(report['actual_reader_command'],receipt['command']),'Actual reader argv differs')
    require(timestamp(receipt['started_utc'])<=timestamp(report['started_utc'])
        <=timestamp(report['completed_utc'])<=timestamp(receipt['finished_utc']),'Actual comparison chronology differs')
    artifacts(report['executed_reader_source_archives'])
    return report,receipt

def same_source(receipt):
    require(receipt['source_snapshot']['sha256']==SNAPSHOT,'Final available source snapshot differs')
    return receipt

def unchanged():
    for path,sha in SEEN.items():
        with path.open('rb') as stream:digest=hashlib.file_digest(stream,'sha256').hexdigest()
        require(digest==sha,'Bound bytes changed before proposal: '+str(path))

def encoded(value):return (json.dumps(value,ensure_ascii=False,indent=2,allow_nan=False)+'\n').encode('utf8')
