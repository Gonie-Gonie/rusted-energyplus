"""Proposed paired v13 published-tooling/parent-format contracts; installation is separately authorized.

A fresh private epoch is owned by one authority_from_evidence call, or by a
separate quality-phase read-only preflight before its first recorder launch.
The preflight owner closes/clears/detaches before every command; later calls
use None. The existing release contract still describes Authority normalization;
this ignored preflight prototype requires a future separately reviewed release.
Snapshot Source/archive path observations and pure JSON/Rust masking may repeat
within that epoch; Source/archive bytes and current command boundaries stay fresh.
This is a formal stable-layout observation boundary, not arbitrary-ABA equivalence.
No recipe code is loaded.
"""
from __future__ import annotations
from dataclasses import dataclass
from pathlib import Path
import copy,hashlib,json,math,re,struct,os,tomllib,fnmatch,csv,io
from . import common116 as c

ROOT=Path(__file__).resolve().parents[4]
PROFILE='ideal_loads.oa_topology.v1'
FROZEN_V4_RELEASE={'path': 'tools/porting/paired/v4/release.json', 'sha256': 'a68819407a41647cb506cc9a959242e0b592073e7b043c9c68bfdeb7ce4eb8f5', 'bytes': 8742}
FROZEN_V5_RELEASE={'path': 'tools/porting/paired/v5/release.json', 'sha256': '89aabe3af3993118c4f0f38c073648d746e424ed4d4a27ccdfcde78a8c5953ea', 'bytes': 8924}
FROZEN_V6_RELEASE={'path': 'tools/porting/paired/v6/release.json', 'sha256': 'fad05d195e3969372afbcbff8451aff70b70484e3b12683c1890c51ac5c01754', 'bytes': 9106}
FROZEN_V7_RELEASE={'path': 'tools/porting/paired/v7/release.json', 'sha256': '22b0757b755b71403d2bf5bd65b1a25b647b36c790c8477e96a4fdd622b7de15', 'bytes': 10330}
FROZEN_V8_RELEASE={'path': 'tools/porting/paired/v8/release.json', 'sha256': 'fa2d45b0ab6696e726f97ad1555589d138112ab151f0f753dc0b15b9ab476dc0', 'bytes': 11634}
FROZEN_V10_RELEASE={'path': 'tools/porting/paired/v10/release.json', 'sha256': '4ee07caa27b87a028178c26f5e7d48a5210d20ba5e5664ebdd3358531aa7cb0b', 'bytes': 13469}
FROZEN_V11_RELEASE={'path': 'tools/porting/paired/v11/release.json', 'sha256': 'b0222c884418cbad11e8478ff0942087b4897b9078c7746abbf27e8e50ad24af', 'bytes': 14731}
FROZEN_V12_RELEASE={'path': 'tools/porting/paired/v12/release.json', 'sha256': '25b970c14f338556ee55f95637b88b3401c06b0cca877b8e99d1d7df0ecbc2f4', 'bytes': 16373}
WARNING_PHASE_CONTRACT={'path': '.runtime/object-expansion/paired-warning-phase-contract-v2.json', 'sha256': '9d2abce1cc108cfb473e423500b8ad4ff49eda90169f0d2e71d7b9eabea82a18', 'bytes': 3476}
INHERITED_WARNING_PHASE_CONTRACT={'path': '.runtime/object-expansion/unit128-prep/science-final01/warning-phase-contract.json', 'sha256': 'afa92ae5e216dff8f9b91aefc5a2592d181b18110b376141503ee09a4347357a', 'bytes': 2976}
NEW_CONSTANT_SCIENTIFIC_SHA256='db5faa295c450692ce2552581020292f2e4cf31eafbc82867296ca5b9e0ae245'
SOURCE_RECORDER_INVOCATION_CONTRACT={'schema': 'paired.SourceRecorderInvocation.v1', 'owner': 'source_production.main.record', 'invocation': 'fixed_trusted_runpy_same_process', 'recorder_schema': 'recorded-porting-command.v1', 'recorder': 'tools/porting/record_command.py', 'recorder_bytes': 'unchanged_frozen_v8_recorder', 'recorder_arguments': 'full_real_sys.argv_with_every_ordered_explicit_input', 'recorder_OS_subprocess_created': False, 'command_launch': 'unchanged_recorder_subprocess.run', 'Python_preconditions': ['assertions_enabled', 'utf8_mode_1', 'dont_write_bytecode_true'], 'stdin_precondition': 'producer_outer_OS_stdin_DEVNULL_inherited_by_unchanged_recorder_command_child', 'stdin_precondition_runtime_enforced': False, 'environment': 'temporary_exact_prior_child_env_including_PATH_restored_in_finally', 'arguments_restoration': 'original_sys.argv_object_restored_in_finally', 'diagnostic_print_capture': 'UTF8_text_newline_empty_exclusive_recorder.stdout_and_recorder.stderr', 'command_stdout_stderr': 'unchanged_binary_recorder_owned_files', 'exit': 'integer_SystemExit_code_only_other_SystemExit_failure_no_return_is_error', 'fresh_input_authentication': 'all_existing_ordered_leaf_auth_calls_after_epoch_close_unchanged', 'epoch_cleanup': 'close_clear_setNone_detach_before_input_authentication', 'schema_science_actor_or_gate_change': False}
QUALITY_PREFLIGHT_EPOCH_CONTRACT={'schema': 'paired.QAPreflightObservationEpoch.v1', 'owner': 'source_production.main_quality_preflight', 'lifetime': 'one_invocation_read_only_prefix_until_first_record', 'stable_read_only_path_layout_required': True, 'stable_pinned_evidence_required': True, 'observations': 'existing_path_JSON_Rust_text_memo_only', 'per_stage_and_binding_snapshot_tables': 'independent', 'bytes_authentication': 'fresh_at_every_existing_auth_call', 'current_Source_checks': 'no_epoch_for_source_ref_or_snapshot', 'cleanup': 'close_clear_setNone_detach_before_every_record_and_input_authentication', 'build_and_evidence_epoch': None, 'after_first_record_epoch': None, 'cross_invocation_actor_or_command_reuse': False, 'concurrent_path_replacement_or_ABA': 'unsupported', 'exception_lifetime': 'unreturned_local_or_traceback_only_no_prompt_erasure_guarantee'}
NORMALIZATION_EPOCH_CONTRACT={'schema': 'paired.NormalizationValidationEpoch.v1', 'owner': 'authority_from_evidence', 'lifetime': 'one_invocation_only', 'stable_read_only_path_layout_required': True, 'stable_pinned_evidence_required': True, 'path_resolution': 'first_observation_per_logical_name', 'JSON_parse_key': 'actual_read_SHA256_and_length', 'Rust_mask_key': 'exact_authenticated_source_text', 'bytes_authentication': 'fresh_at_every_existing_auth_call', 'current_Source_membership_and_executable_checks': 'fresh_existing_inspect_snapshot_and_command_boundaries', 'cross_invocation_actor_or_command_reuse': False, 'concurrent_path_replacement_or_ABA': 'unsupported', 'success_cleanup': 'clear_close_detach', 'exception_lifetime': 'unreturned_local_or_traceback_only_no_prompt_erasure_guarantee'}
# Versioned V2 authority contract; frozen V1/preflight semantics remain separately declared.
NORMALIZATION_EPOCH_V2_CONTRACT={
 'schema':'paired.NormalizationValidationEpoch.v2',
 'owner':'authority_from_evidence',
 'lifetime':'one_normalization_invocation_only',
 'stable_read_only_path_layout_required':True,
 'stable_pinned_evidence_and_in_epoch_descriptors_required':True,
 'snapshot_metadata_key':'freshly_authenticated_actual_read_SHA256_and_length',
 'snapshot_metadata_seed':'only_after_complete_existing_Source_reference_boundary_checks',
 'snapshot_cold_miss':'original_schema_member_checks_and_archive_loop_no_early_index_validation',
 'snapshot_metadata_storage':'private_immutable_scalar_tuples_with_detached_owner_tables',
 'per_stage_and_binding_snapshot_tables':'existing_independent_AuthorityPins_compartments',
 'pin_membership':'per_collection_exact_descriptor_set_after_unconditional_auth_preserving_first_seen_and_Ref2_Ref3',
 'descriptor_mutation_or_list_bypass_during_epoch':'unsupported_no_such_current_Authority_route',
 'bytes_authentication':'every_existing_auth_call_and_archive_loop_unchanged',
 'current_Source_checks':'existing_inspect_snapshot_current_membership_and_command_boundaries_unchanged',
 'general_load_receipt_parse_memo':'deferred',
 'whole_Authority_success_or_cross_invocation_actor_command_reuse':False,
 'success_cleanup':'clear_metadata_and_pin_sets_close_detach_before_return',
 'concurrent_path_replacement_or_ABA':'unsupported',
 'exception_lifetime':'unreturned_local_or_traceback_only_no_prompt_erasure_guarantee',
}
NORMALIZATION_EPOCH_V3_CONTRACT={'schema': 'paired.NormalizationValidationEpoch.v3', 'owner': 'authority_from_evidence', 'lifetime': 'one_normalization_invocation_only', 'metadata_inheritance': 'exact_V2_path_JSON_Rust_snapshot_index_and_pin_compartments', 'stable_read_only_path_layout_required': True, 'stable_pinned_evidence_and_descriptors_required': True, 'stable_immutable_archive_contents_required': True, 'archive_eligibility': 'snapshot_member_archive_only_exact_.runtime/porting/source-objects/{sha256[:2]}/{sha256}', 'archive_key': ['canonical_logical_path', 'sha256', 'declared_member_length'], 'archive_seed': 'full_canonical_Ref2_or_Ref3_scalar_path_validation_and_actual_SHA256_length_verified_read', 'archive_warm_hit': 'same_validated_descriptor_and_member_length_only_no_repeat_archive_byte_read', 'archive_storage': 'private_ordered_immutable_scalar_facts_no_archive_payload_or_caller_tables', 'legacy_or_noneligible_archive': 'unchanged_fresh_auth_then_read_and_member_length_check', 'final_sweep': 'mandatory_fresh_unmemoized_canonical_auth_of_each_distinct_archive_before_clean_return', 'current_Source_snapshot_manifest_receipt_executable_input_and_command_boundary_authentication': 'existing_fresh_checks_unchanged_no_archive_memo', 'quality_preflight': 'unchanged_V1_no_V3_archive_memo', 'successful_Authority_or_Context_cache': False, 'cross_invocation_or_actor_or_command_reuse': False, 'concurrent_archive_path_replacement_alias_equivalence_or_ABA': 'unsupported_no_detection_claim', 'success_cleanup': 'final_fresh_distinct_archive_sweep_then_clear_close_detach', 'final_sweep_failure_cleanup': 'clear_close_detach_in_finally_before_propagating_failure', 'earlier_exception_lifetime': 'unreturned_local_or_traceback_only_no_prompt_erasure_guarantee', 'science_gate_tolerance_or_actor_protocol_changes': False}
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
def snapshot(pin,current=False,epoch=None):
 require(epoch is None or type(epoch)in (_NormalizationEpoch,_NormalizationEpochV2,_NormalizationEpochV3)and not epoch.closed,'Invalid normalization path epoch')
 require(not current or epoch is None,'Fresh current snapshot cannot use a path epoch')
 if type(epoch)in (_NormalizationEpochV2,_NormalizationEpochV3):
  raw=auth(pin).read_bytes();metadata=epoch.snapshot_metadata(raw)
  if metadata is not None:by=metadata.members()
  else:
   value=json.loads(raw);require(value['schema']=='rust-command-source-snapshot.v1'and value['bytes_normalized']is False,'Unnormalized Source snapshot required')
   rows=value['files'];require(type(rows)is list,'Source inventory must be a list');by={source_path(r['historical_path'],epoch):r for r in rows};require(len(by)==len(rows),'Duplicate source snapshot member');check_source_member_metadata(by,epoch)
 else:
  value=load(pin);require(value['schema']=='rust-command-source-snapshot.v1'and value['bytes_normalized']is False,'Unnormalized Source snapshot required')
  rows=value['files'];require(type(rows)is list,'Source inventory must be a list');by={source_path(r['historical_path'],epoch):r for r in rows};require(len(by)==len(rows),'Duplicate source snapshot member');check_source_member_metadata(by,epoch)
 for path,row in by.items():
  require(type(row['size_bytes'])is int and row['size_bytes']>=0,'Invalid Source member size')
  if type(epoch)is _NormalizationEpochV3 and not current and epoch.verify_member_archive(row['archive'],row['size_bytes']):continue
  data=auth(row['archive']).read_bytes();require(len(data)==row['size_bytes'],'Source archive size')
  if current:require((ROOT/path).read_bytes()==data,'Current Source differs from build')
 if current:
  paths=set((ROOT/'crates').rglob('*.rs'))|set((ROOT/'crates').rglob('Cargo.toml'));paths|={ROOT/n for n in ['Cargo.toml','Cargo.lock','rust-toolchain.toml','.cargo/config.toml']if(ROOT/n).is_file()}
  require(set(by)=={p.relative_to(ROOT).as_posix()for p in paths},'Current Source membership differs')
 return by
# Only these closed adapters may consume old owner schemas. New work emits SourceEvidence.v1.
LEGACY_OWNERS={
 'Root-Unit155-positive-OA-supply-humidity-above-one-execution-review.v1':('155',4,'all4_raw_endpoints_bound_to_preformat_snapshot','all_other4672_Source_files_literal157'),
 'Root-Unit158-positive-OA-exact-empty-dehumidification-execution-review.v1':('158',2,'all2_raw_endpoints_bound_to_preformat_snapshot','all_other4674_Source_files_literal160'),
 'Root-Unit159-positive-OA-exact-empty-humidification-execution-review.v1':('159',2,'all2_raw_endpoints_bound_to_preformat_snapshot','all_other4674_Source_files_literal161'),
}
EVIDENCE_KEYS={'format_review', 'qa_reuse', 'current_QA_commands', 'adoption', 'schema', 'format_stages', 'QA_run_contexts', 'current_Source_primary_review', 'executions', 'retained_getter_review', 'QA_profile', 'total_actual_QA_commands', 'QA_profile_binding', 'historical_refs', 'build', 'failed_QA_attempts', 'quality'}
_GETTER_TEST='heat_balance::ordinary_equipment::tests::actual_native_get_equipment_pair_and_input_owner_transitions'
_BUILD_COMMAND=['cargo','build','--locked','--offline','-p','ep_cli','--bin','eplus-rs','--message-format=json-render-diagnostics']
def _receipt(pin):
 r=load(pin);require(r['schema']=='recorded-porting-command.v1','Actual recorder receipt required')
 require(type(r['command'])is list and r['command'] and Path(r['cwd']).resolve()==ROOT,'Recorded command/cwd')
 require(type(r['source_bytes_match_before_and_after'])is bool,'Missing source custody observation')
 return r

def _authority_add(pins,pin):
 auth(pin)
 if type(pins)is _AuthorityPins and type(pins.epoch)in (_NormalizationEpochV2,_NormalizationEpochV3):
  require(not pins.epoch.closed,'Closed normalization pin epoch')
  key=tuple(sorted(pin.items()))
  if key not in pins._descriptor_index:pins.append(pin);pins._descriptor_index.add(key)
 elif pin not in pins:pins.append(pin)

class _NormalizationEpoch:
 """One read-only normalization or quality-preflight segment's observations, not a persistent cache.

 Callers must keep the Source/archive path layout stable during this epoch.
 A path is resolved afresh on first use; repeated logical row shape checks remain.
 This formal boundary differs from v6 under concurrent path replacement or ABA.
 All file-byte authentication and current Source/command boundary checks stay fresh.
 """
 def __init__(self):
  self.closed=False;self.paths={};self.json_values={};self.rust_sources={}
 def resolve_target(self,name):
  require(not self.closed,'Closed normalization path epoch')
  if name not in self.paths:self.paths[name]=(ROOT/name).resolve()
  return self.paths[name]
 def json_value(self,raw):
  require(not self.closed,'Closed normalization metadata epoch')
  key=(hashlib.sha256(raw).digest(),len(raw))
  if key not in self.json_values:
   try:value=json.loads(raw)
   except (json.JSONDecodeError,UnicodeDecodeError):value=None
   self.json_values[key]=value
  return self.json_values[key]
 def close(self):
  self.paths.clear();self.json_values.clear();self.rust_sources.clear();self.closed=True

@dataclass(frozen=True)
class _NormalizationSnapshotMetadata:
 """Checked scalar tuples; mutable member tables belong only to their caller."""
 rows:tuple
 def members(self):
  return {path:dict(historical_path=name,size_bytes=size,archive=dict(archive))for path,name,size,archive in self.rows}

class _NormalizationEpochV2(_NormalizationEpoch):
 """Future normalization-only pure metadata extension, never a successful-Authority cache.

 The existing V1 class and quality preflight route remain unchanged. All data
 reaches this cache after each caller's existing auth and actual byte read.
 No in-epoch descriptor/list mutation, path replacement or cross-call reuse.
 """
 def __init__(self):
  super().__init__();self._snapshot_metadata={};self._pin_indexes=[]
 def snapshot_metadata(self,raw):
  require(not self.closed,'Closed normalization snapshot metadata epoch')
  return self._snapshot_metadata.get((hashlib.sha256(raw).digest(),len(raw)))
 def check_source_ref_metadata(self,raw):
  require(not self.closed,'Closed normalization snapshot metadata epoch')
  key=(hashlib.sha256(raw).digest(),len(raw))
  if key in self._snapshot_metadata:return
  value=self.json_value(raw)
  if type(value)is dict and value.get('schema')=='rust-command-source-snapshot.v1':
   require(value['bytes_normalized']is False and type(value['files'])is list,'Invalid Source snapshot boundary metadata')
   rows=value['files'];require(all(type(row)is dict and'historical_path'in row for row in rows),'Malformed Source snapshot member')
   require(all(type(row['historical_path'])is str for row in rows),'Invalid Source snapshot member path type')
   members={row['historical_path']:row for row in rows};require(len(members)==len(rows),'Duplicate Source snapshot member');check_source_member_metadata(members,self)
   for row in rows:
    archive=row['archive'];name=archive['path'];require(name and '\\'not in name and ':'not in name and all(part and part not in ['.','..']for part in name.split('/')),'Noncanonical archive path before snapshot cache')
    target=self.resolve_target(name);require(target.is_relative_to(ROOT)and target!=ROOT and target.relative_to(ROOT).as_posix()==name,'Archive escape before snapshot cache')
   frozen=tuple((path,row['historical_path'],row['size_bytes'],tuple(row['archive'].items()))for path,row in members.items())
   self._snapshot_metadata[key]=_NormalizationSnapshotMetadata(frozen)
 def new_pin_index(self):
  require(not self.closed,'Closed normalization pin epoch')
  index=set();self._pin_indexes.append(index);return index
 def close(self):
  self._snapshot_metadata.clear()
  for index in self._pin_indexes:index.clear()
  self._pin_indexes.clear();super().close()

class _NormalizationEpochV3(_NormalizationEpochV2):
 """One private immutable-archive verification epoch with a fresh final sweep.

 Only exact content-addressed snapshot-member archives are eligible. The
 archive contents and referenced path layout must remain immutable/read-only
 during this normalization. A final fresh sweep rejects persistent changes;
 concurrent replacement and ABA are not claimed to be detected at each hit.
 Current Source, snapshot manifests, receipts, binaries, inputs and command
 boundaries never consume these facts. Other archive paths keep the RAW path.
 """
 def __init__(self):
  super().__init__();self._verified_member_archives={}
 def verify_member_archive(self,pin,size):
  require(not self.closed,'Closed normalization archive epoch')
  require(type(size)is int and size>=0,'Invalid Source member size')
  require(type(pin)is dict and {'path','sha256'}<=set(pin)<={'path','sha256','bytes'},'Malformed Source member archive')
  name=pin['path'];sha=pin['sha256']
  require(type(name)is str and type(sha)is str and re.fullmatch('[0-9a-f]{64}',sha)is not None,'Invalid Source member archive scalar')
  if 'bytes'in pin:require(type(pin['bytes'])is int and pin['bytes']==size,'Invalid Source member archive length')
  if name!='.runtime/porting/source-objects/'+sha[:2]+'/'+sha:return False
  key=(name,sha,size)
  if key in self._verified_member_archives:return True
  # The exact digest-derived lexical path is canonical; validate its actual
  # resolved identity before the one cold read. No hash-only alias key is used.
  path=(ROOT/name).resolve()
  require(path.is_relative_to(ROOT)and path!=ROOT and path.relative_to(ROOT).as_posix()==name,'Archive path escape or alias before verification memo')
  raw=path.read_bytes()
  require(len(raw)==size and hashlib.sha256(raw).hexdigest()==sha,'Source archive bytes differ')
  self._verified_member_archives[key]=None
  return True
 def verify_final_archives(self):
  require(not self.closed,'Closed normalization archive sweep')
  for name,sha,size in tuple(self._verified_member_archives):
   # No epoch, path memo, or archive memo participates in this final boundary.
   _canonical_pin(dict(path=name,sha256=sha,bytes=size))
 def close(self):
  self._verified_member_archives.clear();super().close()

class _AuthorityPins(list):
 def __init__(self,epoch=None):
  super().__init__();self.snapshots={};self.epoch=epoch;self._descriptor_index=epoch.new_pin_index()if type(epoch)in (_NormalizationEpochV2,_NormalizationEpochV3)else None

def _authority_snapshot(pin,pins):
 # Cache immutable archived snapshots only for this one authority invocation.
 # Fresh current files are checked separately by inspect_snapshot every time.
 key=(pin['path'],pin['sha256'])
 if key in pins.snapshots:return pins.snapshots[key]
 value=load(pin);require(value['schema']=='rust-command-source-snapshot.v1'and value['bytes_normalized']is False,'Unnormalized Source snapshot required');_authority_add(pins,pin)
 rows=snapshot(pin,epoch=pins.epoch);pins.snapshots[key]=rows;return rows

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


def _zero_test_stdout(text):
 return type(text)is str and re.fullmatch(r'\s*running 0 tests\r?\n\r?\ntest result: ok\. 0 passed; 0 failed; 0 ignored; 0 measured; [0-9]+ filtered out; finished in [0-9]+(?:\.[0-9]+)?s\s*',text)is not None


def authority_from_evidence(evidence_ref,mode):
 require(mode in ['fresh','retained'],'Unknown authority mode');e=load(evidence_ref);exact_keys(e,EVIDENCE_KEYS,'SourceEvidence');require(e['schema']=='paired.SourceEvidence.v3','Unsupported Source evidence schema');qa_profile(e['QA_profile']);pins=_AuthorityPins(_NormalizationEpochV3());_canonical_pin(evidence_ref);_authority_add(pins,evidence_ref);require(e['qa_reuse']==[],'Required-body reuse is disabled');_canonical_pin(e['QA_profile']);_canonical_pin(e['QA_profile_binding'])
 build_ref=e['build'];quality_ref=e['quality'];adoption_ref=e['adoption'];format_ref=e['format_review'];executions=e['executions'];historical=e['historical_refs'];stages=e['format_stages'];reuse=e['qa_reuse']
 require(e['retained_getter_review']is None,'This Source QA profile has no retained OA getter credit')
 for pin in [build_ref,quality_ref,adoption_ref,format_ref]:_canonical_pin(pin);_authority_add(pins,pin)
 build=load(build_ref);_source_ref(build['source_snapshot'],pins.epoch);br,members=c.inspect_snapshot(build,mode=='fresh');check_source_member_metadata(members,pins.epoch);require(br['command']==_BUILD_COMMAND,'Default actual build command');_authority_add(pins,build['receipt']);_authority_add(pins,build['source_snapshot']);_authority_add(pins,build['retained_executable']);pins.snapshots[(build['source_snapshot']['path'],build['source_snapshot']['sha256'])]=members
 preparation=load(build['quality_preparation']);require(preparation['schema']=='paired.SourceQualityPreparation.v3','Current build requires v3 quality preparation');final_plan=preparation['plan'];_canonical_pin(final_plan);_authority_add(pins,build['quality_preparation']);_authority_add(pins,final_plan)
 quality=load(quality_ref);exact_keys(quality,{'stage','runs','all_actual_runs','source_summaries'},'complete quality ledger');require(quality['stage']=='quality'and type(quality['runs'])is list and quality['runs']and type(quality['all_actual_runs'])is list,'Actual quality ledger required');current=quality['runs'];all_runs=quality['all_actual_runs'];require(quality['source_summaries']and current==load(quality['source_summaries'][-1])['runs'],'Current QA must be the actual final underlying summary')
 adoption=load(adoption_ref);before_build=adoption['actual_before_Source_build'];_canonical_pin(before_build)
 require(type(stages)is list and stages,'Complete Source stage chain required')
 for edge in stages:
  _source_ref(edge['before_snapshot'],pins.epoch);_source_ref(edge['after_snapshot'],pins.epoch)
  for endpoint in edge['endpoints']:
   for side in ['before','after']:
    if endpoint[side]is not None:_source_ref(endpoint[side],pins.epoch)
 fmt=load(format_ref);_source_ref(fmt['current_source_snapshot'],pins.epoch);_source_ref(fmt['actual_raw_snapshot'],pins.epoch)
 for endpoint in fmt['source_endpoints']:
  for side in ['before','after']:
   if endpoint[side]is not None:_source_ref(endpoint[side],pins.epoch)
 require(quality['source_summaries'],'Complete underlying QA summaries required');observed=[]
 for pin in quality['source_summaries']:
  original=load(pin);require(original['stage']=='quality','Underlying QA summary stage');observed+=original['runs'];_authority_add(pins,pin)
 require(observed==all_runs,'Complete underlying QA attempt order differs')
 require(len({r['receipt']['path']for r in all_runs})==len(all_runs)and len({r['receipt']['sha256']for r in all_runs})==len(all_runs)and len({r['receipt']['path']for r in current})==len(current)and len({r['receipt']['sha256']for r in current})==len(current),'Duplicate QA attempt path/content')
 failed=0;receipts={}
 for row in all_runs:
  exact_keys(row,{'name','recorder_exit','receipt','exit_code'},'actual QA run');pin=row['receipt'];_canonical_pin(pin);r=_receipt(pin);_source_ref(r['source_snapshot'],pins.epoch);_authority_add(pins,pin);receipts[pin['path']]=r;_authority_snapshot(r['source_snapshot'],pins);bad=r['exit_code']!=0 or r['launch_error']is not None
  require(row['exit_code']==r['exit_code']and (row['recorder_exit']!=0 if bad else row['recorder_exit']==0),'QA actual exit/recorder disagreement');failed+=int(bad)
  for key in ['stdout','stderr']:_authority_add(pins,r[key])
 reuse_by={x['receipt']['path']:x for x in reuse};require(len(reuse_by)==len(reuse),'Duplicate QA reuse')
 execution_by={x['receipt']['path']:x for x in executions};require(len(execution_by)==len(executions)and len({x['receipt']['sha256']for x in executions})==len(executions),'Duplicate test execution receipt path/content')
 require(set(execution_by)=={row['receipt']['path']for row in all_runs if receipts[row['receipt']['path']]['exit_code']==0 and receipts[row['receipt']['path']]['launch_error']is None and receipts[row['receipt']['path']]['command'][:2]==['cargo','test']and '--exact'in receipts[row['receipt']['path']]['command']},'Successful actual test receipts need complete body accounting')
 contexts=_qa_run_contexts(e,all_runs,stages,before_build,final_plan,pins)
 for row in current:
  require(row in all_runs,'Current QA missing from actual attempts');r=receipts[row['receipt']['path']];require(r['exit_code']==0 and r['launch_error']is None,'Current QA failed')
  is_format=r['command'][:2]==['cargo','fmt']and '--check'not in r['command']
  if r['command'][:2]==['cargo','test']:require(row['receipt']['path']in execution_by,'Current test receipt lacks explicit body accounting')
  if not is_format:
   require(r['source_bytes_match_before_and_after']is True and r['repository_before']==r['repository_after'],'QA custody changed')
   if r['source_snapshot']!=build['source_snapshot']:
    require(False,'Current QA must use the final snapshot; reuse is disabled')
 require(all(type(e[key])is int and e[key]>=0 for key in ['current_QA_commands','total_actual_QA_commands','failed_QA_attempts']),'Invalid QA count type');require(e['current_QA_commands']==len(current)and e['total_actual_QA_commands']==len(all_runs)and e['failed_QA_attempts']==failed,'QA count/history differs')
 current_bodies=reused_bodies=historical_bodies=0;current_getter=False;normalized=[]
 for item in executions:
  exact_keys(item,{'kind','role','test_names','receipt','actual_exit_code','lineage'},'test execution');require(item['kind']=='targeted_existing_test'and item['role']in ['current','historical'],'Unknown test role');names=item['test_names'];require(type(names)is list and (names or item['role']=='historical')and len(set(names))==len(names),'Missing/duplicate test names');r=_receipt(item['receipt']);_authority_add(pins,item['receipt'])
  require(r['command'][:2]==['cargo','test']and '--exact'in r['command']and all(name in r['command']for name in names),'Exact executed-test command required');require(type(item['actual_exit_code'])is int and item['actual_exit_code']==r['exit_code']==0 and r['launch_error']is None and r['source_bytes_match_before_and_after']is True,'Failed/unstable test credit')
  text=auth(r['stdout']).read_text(encoding='utf8');observed=re.findall(r'(?m)^test (\S+) \.\.\. ok\s*$',text);require(sorted(observed)==sorted(names),'Actual body execution set differs');_authority_add(pins,r['stdout']);_authority_add(pins,r['source_snapshot'])
  if not names:require(item['role']=='historical'and r['repository_before']==r['repository_after']and len(r['command'])>=3 and r['command'][-2:]==['--','--exact']and re.fullmatch(r'(?:[A-Za-z_][A-Za-z0-9_]*::)+[A-Za-z_][A-Za-z0-9_]*',r['command'][-3])is not None and _zero_test_stdout(text),'Historical empty execution requires an actual exact zero-test observation')
  role=item['role'];key=item['receipt']['path']
  if role=='current':
   require(item['receipt']in [v['receipt']for v in current]and r['source_snapshot']==build['source_snapshot']and item['lineage']==[],'Current test source/role differs');current_bodies+=len(names);current_getter|=_GETTER_TEST in names
  elif role=='reused':
   require(item['receipt']in [row['receipt']for row in all_runs]and key in reuse_by and item['lineage']==reuse_by[key]['lineage'],'Reused test absent from complete actual QA or lineage differs');_authority_reuse(reuse_by[key],item['receipt'],build['source_snapshot'],pins,names);reused_bodies+=len(names)
  else:
   require(item['receipt']in [v['receipt']for v in all_runs]and item['receipt']not in [v['receipt']for v in current]and item['lineage']==[],'Historical test absent from ledger or gets current credit');_authority_snapshot(r['source_snapshot'],pins);historical_bodies+=len(names)
  normalized.append(item)
 required_names=check_QA_profile(e['QA_profile'],e['QA_profile_binding'],current,receipts,normalized,build['source_snapshot'],before_build,stages,pins)
 final_plan_data=load(final_plan);expected_environment=final_plan_data['launch_environment_selected'];require(br['launch_environment_selected']==expected_environment,'Final build environment differs from final plan')
 frozen_ref=build['frozen_build_receipt'];freeze_ref=build['freeze_execution'];_canonical_pin(frozen_ref);_canonical_pin(freeze_ref);frozen=load(frozen_ref);freeze_receipt=_receipt(freeze_ref)
 require(frozen['schema']=='porting-Rust-build.v1'and ref(_source_ref(frozen['build_execution'],pins.epoch))==build['receipt']and frozen['source_snapshot']==build['source_snapshot']and ref(_source_ref(frozen['binary'],pins.epoch))==build['retained_executable'],'Actual frozen build identity differs')
 freezer_pin=final_plan_data['tools']['freezer'];_canonical_pin(freezer_pin);require(auth(freezer_pin)==ROOT/'tools/porting/freeze_rust_build.py','Unexpected freezer path')
 expected_freeze=[freeze_receipt['python']['executable'],'-X','utf8','-B',str(auth(freezer_pin)),'--command-receipt',str(auth(build['receipt'])),'--output-dir',str(auth(frozen_ref).parent),'--cli','--allow-dirty']
 require(freeze_receipt['command']==expected_freeze and freeze_receipt['exit_code']==0 and freeze_receipt['launch_error']is None and freeze_receipt['source_snapshot']==build['source_snapshot']and freeze_receipt['source_bytes_match_before_and_after']is True and freeze_receipt['repository_before']==freeze_receipt['repository_after'],'Actual freezer invocation/custody differs')
 require(freeze_receipt['launch_environment_selected']==expected_environment,'Final freezer environment differs from final plan')
 executed_freezer=ref(_source_ref(frozen['executed_freezer'],pins.epoch));require(executed_freezer['sha256']==freezer_pin['sha256']and executed_freezer['bytes']==freezer_pin['bytes'],'Executed freezer bytes differ')
 final_context=next(info for info in contexts.values()if info['plan']==final_plan_data)
 for actual in [br,freeze_receipt]:
  custody=actual['explicit_input_custody'];require(custody['all_explicit_inputs_unchanged']is True and custody['command_process_started']is True and custody['read_errors_before']==custody['read_errors_after']==[]and custody['before']==custody['after'],'Final build/freezer explicit-input custody differs')
  observed={}
  for entry in custody['before']:
   _canonical_pin(entry);require(entry['path']not in observed,'Duplicate final build/freezer input');observed[entry['path']]=entry
  for pin in final_context['required']:
   expected=ref(auth(pin));require(observed.get(expected['path'])==expected,'Final build/freezer omitted final plan closure')
 for pin in [frozen_ref,freeze_ref,frozen['executed_freezer']]:_authority_add(pins,pin)
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
 primary=e['current_Source_primary_review'];_source_ref(load(primary)['historical_source_snapshot'],pins.epoch);_authority_primary(primary,members,pins)
 # Do not retain epoch observations in normalized output or later caller activity.
 try:pins.epoch.verify_final_archives()
 finally:pins.epoch.close();pins.epoch=None
 return dict(schema='paired.SourceAuthority.v3',mode=mode,evidence=evidence_ref,build=build_ref,quality=quality_ref,adoption=adoption_ref,format_review=format_ref,snapshot=build['source_snapshot'],inventory_count=len(members),changed_paths=sorted(final_paths),source_endpoints=endpoints,before_inventory_count=len(old),added_paths=added,deleted_paths=deleted,QA_profile=e['QA_profile'],QA_profile_binding=e['QA_profile_binding'],QA_run_contexts=e['QA_run_contexts'],required_test_names=required_names,required_reused_test_names=[],required_current_test_names=required_names,current_Source_primary_review=primary,current_QA_commands=len(current),total_actual_QA_commands=len(all_runs),failed_QA_attempts=failed,current_test_body_executions=current_bodies,reused_test_bodies=reused_bodies,historical_test_bodies=historical_bodies,retained_getter_body_executions=0,executions=normalized,historical_refs=historical,retained_getter_review=None,current_getter_credit=current_getter,pins=pins)

def load_authority(pin,mode):
 a=load(pin);require(a['schema']=='paired.SourceAuthority.v3'and a['mode']==mode,'Authority mode/schema differs');require(a==authority_from_evidence(a['evidence'],mode),'Normalized authority differs from underlying evidence');return a



def source_path(path,epoch=None):
 require(type(path)is str and path and '\\'not in path,'Invalid Source path')
 parts=path.split('/');require(all(part and part not in ['.','..']for part in parts),'Noncanonical Source path')
 require(not Path(path).is_absolute()and ':'not in path,'Absolute Source path')
 require((path.startswith('crates/')and(path.endswith('.rs')or path.endswith('/Cargo.toml')))or path in ['Cargo.toml','Cargo.lock','rust-toolchain.toml','.cargo/config.toml'],'Path outside recorded Source domain')
 require(epoch is None or type(epoch)in (_NormalizationEpoch,_NormalizationEpochV2,_NormalizationEpochV3)and not epoch.closed,'Invalid normalization path epoch')
 target=(ROOT/path).resolve()if epoch is None else epoch.resolve_target(path);require(target.is_relative_to(ROOT)and target!=ROOT,'Source path escape')
 return path


def source_delta(before,after):
 return {p for p in set(before)|set(after)if p not in before or p not in after or(before[p]['archive']['sha256'],before[p]['size_bytes'])!=(after[p]['archive']['sha256'],after[p]['size_bytes'])}


def check_source_member_metadata(members,epoch=None):
 require(type(members)is dict,'Invalid Source member table')
 for path,row in members.items():
  source_path(path,epoch);exact_keys(row,{'historical_path','size_bytes','archive'},'Source snapshot member')
  require(row['historical_path']==path and type(row['size_bytes'])is int and row['size_bytes']>=0,'Invalid Source member size/path')
  pin=row['archive'];require(type(pin)is dict and {'path','sha256'}<=set(pin)<={'path','sha256','bytes'}and type(pin['path'])is str and type(pin['sha256'])is str and re.fullmatch('[0-9a-f]{64}',pin['sha256'])is not None,'Invalid Source archive metadata')
  if 'bytes'in pin:require(type(pin['bytes'])is int and pin['bytes']==row['size_bytes'],'Invalid Source archive size metadata')


def source_endpoints(before,after):
 return [dict(path=p,before=before[p]['archive']if p in before else None,after=after[p]['archive']if p in after else None)for p in sorted(source_delta(before,after))]


def check_source_endpoints(rows,before,after,pins):
 require(type(rows)is list,'Source endpoints must be a list')
 require(all(type(row)is dict and set(row)=={'path','before','after'}for row in rows),'Malformed Source endpoint')
 paths=[source_path(row['path'],pins.epoch)for row in rows];require(paths==sorted(source_delta(before,after)),'Source endpoint order/set differs')
 for row in rows:
  p=row['path'];require(row['before']is not None or row['after']is not None,'Both Source sides absent')
  for key,table in [('before',before),('after',after)]:
   pin=row[key];require((pin is None)==(p not in table),'Source endpoint presence differs')
   if pin is not None:
    expected=table[p];body=auth(pin).read_bytes();require(len(body)==expected['size_bytes']and pin['sha256']==expected['archive']['sha256']and body==auth(expected['archive']).read_bytes(),'Source endpoint/archive differs');_authority_add(pins,pin)
 return paths


def _canonical_pin(pin):
 exact_keys(pin,{'path','sha256','bytes'},'new metadata pin')
 require(type(pin['path'])is str and pin['path']and '\\'not in pin['path']and ':'not in pin['path'],'Invalid metadata path')
 require(all(part and part not in ['.','..']for part in pin['path'].split('/')),'Noncanonical metadata path')
 require(type(pin['sha256'])is str and re.fullmatch('[0-9a-f]{64}',pin['sha256'])is not None,'Invalid metadata hash')
 require(type(pin['bytes'])is int and pin['bytes']>=0,'Invalid metadata byte size')
 path=auth(pin);require(pin['path']==path.relative_to(ROOT).as_posix(),'Noncanonical metadata reference')
 return path


def _source_ref(pin,epoch=None):
 require(epoch is None or type(epoch)in (_NormalizationEpoch,_NormalizationEpochV2,_NormalizationEpochV3)and not epoch.closed,'Invalid normalization path epoch')
 require(type(pin)is dict and set(pin)in [{'path','sha256'},{'path','sha256','bytes'}],'Malformed inherited Source reference')
 require(type(pin['path'])is str and pin['path']and '\\'not in pin['path']and ':'not in pin['path'],'Invalid Source reference path')
 require(all(part and part not in ['.','..']for part in pin['path'].split('/')),'Noncanonical Source reference')
 require(type(pin['sha256'])is str and re.fullmatch('[0-9a-f]{64}',pin['sha256'])is not None,'Invalid Source reference hash')
 require('bytes'not in pin or type(pin['bytes'])is int and pin['bytes']>=0,'Invalid optional Source byte size')
 path=auth(pin);require(pin['path']==path.relative_to(ROOT).as_posix(),'Noncanonical inherited Source reference')
 raw=path.read_bytes()
 if type(epoch)in (_NormalizationEpochV2,_NormalizationEpochV3):
  epoch.check_source_ref_metadata(raw);return path
 if epoch is None:
  try:value=json.loads(raw)
  except (json.JSONDecodeError,UnicodeDecodeError):value=None
 else:value=epoch.json_value(raw)
 if type(value)is dict and value.get('schema')=='rust-command-source-snapshot.v1':
  require(value['bytes_normalized']is False and type(value['files'])is list,'Invalid Source snapshot boundary metadata')
  rows=value['files'];require(all(type(row)is dict and'historical_path'in row for row in rows),'Malformed Source snapshot member')
  require(all(type(row['historical_path'])is str for row in rows),'Invalid Source snapshot member path type')
  members={row['historical_path']:row for row in rows};require(len(members)==len(rows),'Duplicate Source snapshot member');check_source_member_metadata(members,epoch)
  for row in rows:
   archive=row['archive'];name=archive['path'];require(name and '\\'not in name and ':'not in name and all(part and part not in ['.','..']for part in name.split('/')),'Noncanonical archive path before snapshot cache')
   target=(ROOT/name).resolve()if epoch is None else epoch.resolve_target(name);require(target.is_relative_to(ROOT)and target!=ROOT and target.relative_to(ROOT).as_posix()==name,'Archive escape before snapshot cache')
  # RAW snapshot/_authority_snapshot still authenticate every archive byte.
  # Only complete boundary metadata is checked here, BEFORE any cache lookup.
 return path


def qa_profile(profile_ref,epoch=None):
 _canonical_pin(profile_ref);profile=load(profile_ref)
 exact_keys(profile,{'schema','id','affected_packages','checks','required_bodies','declared_Source_changes','reuse_policy'},'Source QA profile')
 require(profile['schema']=='paired.SourceQAProfile.v1'and type(profile['id'])is str and re.fullmatch('[A-Za-z0-9][A-Za-z0-9_.-]*',profile['id'])is not None,'Unsupported Source QA profile')
 require(profile['reuse_policy']=='disabled','Required-body reuse is disabled')
 packages=profile['affected_packages'];require(type(packages)is list and packages and all(type(p)is str and re.fullmatch('[A-Za-z0-9][A-Za-z0-9_-]*',p)is not None for p in packages)and len(set(packages))==len(packages),'Invalid affected Cargo packages')
 paths=profile['declared_Source_changes'];require(type(paths)is list and paths and paths==sorted({source_path(p,epoch)for p in paths}),'Invalid declared Source change union')
 bodies=profile['required_bodies'];require(type(bodies)is list and bodies,'Missing required existing bodies')
 ids=[];names=[];coverage=set()
 for body in bodies:
  exact_keys(body,{'id','package','test_name','test_file','required_role','coverage_paths'},'Source QA body')
  require(type(body['id'])is str and re.fullmatch('[A-Za-z0-9][A-Za-z0-9_.-]*',body['id'])is not None,'Invalid body identifier')
  require(body['package']in packages and body['required_role']=='current','Unsupported body package/role')
  require(type(body['test_name'])is str and re.fullmatch('(?:[A-Za-z_][A-Za-z0-9_]*::)+[A-Za-z_][A-Za-z0-9_]*',body['test_name'])is not None,'Full existing test name required')
  require(source_path(body['test_file'],epoch).endswith('.rs'),'Body is not in a Rust source file')
  covered=body['coverage_paths'];require(type(covered)is list and covered and covered==sorted({source_path(p,epoch)for p in covered})and set(covered)<=set(paths),'Invalid reviewed body coverage')
  ids.append(body['id']);names.append(body['test_name']);coverage.update(covered)
 require(len(set(ids))==len(ids)and len(set(names))==len(names)and coverage==set(paths),'Incomplete/duplicate existing-body coverage')
 checks=profile['checks'];require(type(checks)is list and len(checks)>=len(bodies)+3,'Incomplete Source QA checks')
 options=[value for package in packages for value in ['-p',package]]
 kinds=[];check_names=[];body_checks=[];commands=[]
 for check in checks:
  exact_keys(check,{'name','kind','command','body_id','feature'},'Source QA check')
  require(type(check['name'])is str and re.fullmatch('[A-Za-z0-9][A-Za-z0-9_.-]*',check['name'])is not None,'Invalid check name')
  kind=check['kind'];require(kind in ['format','format_check','test','feature_compile','clippy'],'Unknown Source QA command kind')
  if kind=='test':
   require(check['body_id']in ids and check['feature']is None,'Unbound test check')
   body=bodies[ids.index(check['body_id'])];expected=['cargo','test','--locked','--offline','-p',body['package'],'--lib',body['test_name'],'--','--exact'];body_checks.append(check['body_id'])
  else:
   require(check['body_id']is None,'Non-test command cannot execute a body')
   if kind=='feature_compile':
    require(check['feature']=='native-controlled-air-cohort'and'ep_runtime'in packages,'Unsupported affected feature')
    expected=['cargo','check','--locked','--offline','-p','ep_runtime','--all-targets','--features','native-controlled-air-cohort']
   else:
    require(check['feature']is None,'Unexpected feature argument')
    expected=['cargo','fmt',*options]+(['--','--check']if kind=='format_check'else[])if kind in ['format','format_check']else['cargo','clippy','--locked','--offline',*options,'--all-targets','--','-D','warnings']
  require(type(check['command'])is list and all(type(x)is str for x in check['command'])and check['command']==expected,'Source QA command differs from closed form')
  kinds.append(kind);check_names.append(check['name']);commands.append((check['name'],expected))
 require(len(set(check_names))==len(check_names)and body_checks==ids,'Duplicate/reordered QA check or body')
 require(kinds in [['format','format_check',*['test']*len(bodies),'clippy'],['format','format_check',*['test']*len(bodies),'feature_compile','clippy']],'Source QA check order/cardinality differs')
 require(check_names[0:2]==['format','format-check']and check_names[-1]=='clippy','Mechanical QA names differ')
 if'feature_compile'in kinds:require(check_names[-2]=='affected-feature-check','Feature check name differs')
 return commands


def _qa_profile_binding(profile_ref,binding_ref,before_build,authored_stage_reviews,pins):
 qa_profile(profile_ref,pins.epoch);_canonical_pin(binding_ref);_canonical_pin(before_build);profile=load(profile_ref);binding=load(binding_ref)
 exact_keys(binding,{'schema','profile','before_build','authored_stages','Root_review','independent_review'},'Source QA profile binding')
 require(binding['schema']=='paired.SourceQAProfileBinding.v1'and binding['profile']==profile_ref and binding['before_build']==before_build,'Source QA binding identity differs')
 declared=binding['authored_stages'];require(type(declared)is list and declared and type(authored_stage_reviews)is list,'Missing authored Source stage authorization')
 require(binding['Root_review']!=binding['independent_review'],'Distinct profile reviewers required')
 for role,key in [('Root','Root_review'),('independent','independent_review')]:
  _canonical_pin(binding[key]);review=load(binding[key]);exact_keys(review,{'schema','role','profile','before_build','authored_stages','accepted','accepted_static_only','findings'},'external Source QA review')
  require(review['schema']=='paired.SourceQAProfileReview.v1'and review['role']==role and review['profile']==profile_ref and review['before_build']==before_build and review['authored_stages']==declared,'Source QA review binding differs')
  require(review['accepted']is True and review['accepted_static_only']is True and review['findings']==[],'Unaccepted Source QA profile review');_authority_add(pins,binding[key])
 base=load(before_build);_source_ref(base['source_snapshot'],pins.epoch);baseline=_authority_snapshot(base['source_snapshot'],pins)
 cargo_refs={path:row['archive']for path,row in baseline.items()if path=='Cargo.toml'or path.endswith('/Cargo.toml')}
 stage_refs=[];paths=set()
 for stage in declared:
  exact_keys(stage,{'stage_review','prepared_manifest','proposal','Source_review'},'QA authored-stage declaration')
  for pin in stage.values():_canonical_pin(pin);_authority_add(pins,pin)
  review=load(stage['stage_review']);manifest=load(stage['prepared_manifest']);peer=load(stage['Source_review'])
  require(review['accepted']is True and manifest['proposal']==stage['proposal'],'Authored stage/packet identity differs')
  require(peer['accepted']is True and peer['accepted_static_only']is True and peer['findings']==[]and peer['prepared_manifest']==stage['prepared_manifest']and peer['proposal']==stage['proposal'],'Unaccepted Source packet review')
  projected=[]
  for rows in [review['files'],manifest['files']]:
   require(type(rows)is list and rows,'Missing authored nullable endpoints');one=[]
   for row in rows:
    require(type(row)is dict and {'path','before','after'}<=set(row),'Malformed authored endpoint');path=source_path(row['path'],pins.epoch);require(row['before']is not None or row['after']is not None,'Both Source endpoint sides absent')
    for side in ['before','after']:
     if row[side]is not None:
      _source_ref(row[side],pins.epoch);require(not row[side]['path'].startswith('crates/')and row[side]['path']not in ['Cargo.toml','Cargo.lock','rust-toolchain.toml','.cargo/config.toml'],'Authored endpoint must retain immutable bytes');_authority_add(pins,row[side])
    one.append({key:row[key]for key in ['path','before','after']})
   require([r['path']for r in one]==sorted({r['path']for r in one}),'Authored endpoints not sorted unique');projected.append(one)
  require(projected[0]==projected[1],'Authored packet/stage endpoint projection differs')
  for row in projected[0]:
   if row['path']=='Cargo.toml'or row['path'].endswith('/Cargo.toml'):
    if row['after']is None:cargo_refs.pop(row['path'],None)
    else:cargo_refs[row['path']]=row['after']
  paths.update(r['path']for r in projected[0]);stage_refs.append(stage['stage_review'])
 require(stage_refs==authored_stage_reviews and len({p['path']for p in stage_refs})==len(stage_refs),'Complete ordered authored-stage authorization differs')
 require(sorted(paths)==profile['declared_Source_changes'],'Complete authored Source union differs from profile')
 require('Cargo.toml'in cargo_refs,'Staged workspace Cargo manifest absent')
 workspace=tomllib.loads(_source_ref(cargo_refs['Cargo.toml'],pins.epoch).read_text('utf8'))['workspace'];patterns=workspace['members'];excluded=workspace.get('exclude',[])
 require(type(patterns)is list and all(type(p)is str for p in patterns)and type(excluded)is list and all(type(p)is str for p in excluded),'Invalid authenticated workspace members')
 package_names=[]
 for path,pin in cargo_refs.items():
  if path=='Cargo.toml':continue
  directory=path.rsplit('/',1)[0]
  if any(fnmatch.fnmatchcase(directory,p)for p in patterns)and not any(fnmatch.fnmatchcase(directory,p)for p in excluded):
   manifest=tomllib.loads(_source_ref(pin,pins.epoch).read_text('utf8'));require(type(manifest.get('package',{}).get('name'))is str,'Workspace member has no package name');package_names.append(manifest['package']['name']);_authority_add(pins,pin)
 require(len(package_names)==len(set(package_names))and set(profile['affected_packages'])<=set(package_names),'Affected package absent/ambiguous in authenticated staged workspace')
 _authority_add(pins,cargo_refs['Cargo.toml'])
 for pin in [profile_ref,binding_ref,before_build]:_authority_add(pins,pin)
 return profile


def _qa_test_declaration(body,members,epoch=None):
 path=source_path(body['test_file'],epoch);require(path in members and path.endswith('.rs'),'Required existing test file absent')
 parts=path.split('/');require(len(parts)>3 and parts[0]=='crates','Test file outside a recorded crate')
 manifest='/'.join(parts[:2])+'/Cargo.toml';require(manifest in members,'Test crate Cargo manifest absent')
 cargo=tomllib.loads(_source_ref(members[manifest]['archive'],epoch).read_text('utf8'));require(cargo.get('package',{}).get('name')==body['package'],'Test file belongs to a different Cargo package')
 text=_source_ref(members[path]['archive'],epoch).read_text('utf8')
 source=None if epoch is None else epoch.rust_sources.get(text)
 if source is None:
  masked=list(text);i=0
  raw_pattern=re.compile(r'(?:br|r)(#*)"');char_pattern=re.compile(r"(?:b)?'(?:\\(?:u\{[0-9A-Fa-f_]+\}|x[0-9A-Fa-f]{2}|.)|[^'\\\n])'")
  while i<len(text):
   start=i
   if text.startswith('//',i):
    end=text.find('\n',i);i=len(text)if end<0 else end
   elif text.startswith('/*',i):
    i+=2;depth=1
    while i<len(text)and depth:
     if text.startswith('/*',i):depth+=1;i+=2
     elif text.startswith('*/',i):depth-=1;i+=2
     else:i+=1
    require(depth==0,'Unterminated Rust block comment')
   else:
    raw=raw_pattern.match(text,i);char=char_pattern.match(text,i)
    if raw:
     stop='"'+raw[1];end=text.find(stop,i+len(raw[0]));require(end>=0,'Unterminated Rust raw string');i=end+len(stop)
    elif text[i]=='"'or text.startswith('b"',i):
     i+=2 if text.startswith('b"',i)else 1;closed=False
     while i<len(text):
      if text[i]=='\\':i+=2
      elif text[i]=='"':i+=1;closed=True;break
      else:i+=1
     require(closed,'Unterminated Rust string')
    elif char:i+=len(char[0])
    else:i+=1;continue
   for at in range(start,i):
    if text[at]not in '\r\n':masked[at]=' '
  source=''.join(masked)
  if epoch is not None:epoch.rust_sources[text]=source
 name=body['test_name'].rsplit('::',1)[-1]
 require(len(re.findall(r'\bfn\s+'+re.escape(name)+r'\s*\(',source))==1,'Existing test function is missing/ambiguous')
 require(len(re.findall(r'#\s*\[\s*test\s*\]\s*(?:#\s*\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+'+re.escape(name)+r'\s*\(',source))==1,'Required existing function is not a unique test declaration')


def check_QA_profile(profile_ref,binding_ref,current,receipts,executions,final_snapshot,before_build,stages,pins):
 profile=_qa_profile_binding(profile_ref,binding_ref,before_build,[edge['review']for edge in stages if edge['kind']!='format'],pins);commands=qa_profile(profile_ref)
 require(type(current)is list and [row['name']for row in current]==[key for key,command in commands],'Source QA profile order differs')
 for row,(key,command)in zip(current,commands,strict=True):
  require(receipts[row['receipt']['path']]['command']==command,'Current Source QA command differs')
 active=[item for item in executions if item['role']in ['current','reused']];names=[body['test_name']for body in profile['required_bodies']]
 require([item['test_names']for item in active]==[[name]for name in names],'Required current Source body set/order differs')
 require(all(item['role']=='current'and item['lineage']==[]and receipts[item['receipt']['path']]['source_snapshot']==final_snapshot for item in active),'Required Source bodies must be actual current executions')
 _source_ref(final_snapshot,pins.epoch);before_snapshot=load(before_build)['source_snapshot'];_source_ref(before_snapshot,pins.epoch)
 members=_authority_snapshot(final_snapshot,pins);before_members=_authority_snapshot(before_snapshot,pins)
 workspace=tomllib.loads(_source_ref(members['Cargo.toml']['archive'],pins.epoch).read_text('utf8'))['workspace'];package_names=[]
 for path,row in members.items():
  if path=='Cargo.toml'or not path.endswith('/Cargo.toml'):continue
  directory=path.rsplit('/',1)[0]
  if any(fnmatch.fnmatchcase(directory,p)for p in workspace['members'])and not any(fnmatch.fnmatchcase(directory,p)for p in workspace.get('exclude',[])):
   manifest=tomllib.loads(_source_ref(row['archive'],pins.epoch).read_text('utf8'));package_names.append(manifest['package']['name'])
 require(len(package_names)==len(set(package_names))and set(profile['affected_packages'])<=set(package_names),'Affected package absent/ambiguous in actual final workspace')
 for body in profile['required_bodies']:_qa_test_declaration(body,before_members,pins.epoch);_qa_test_declaration(body,members,pins.epoch)
 return names


def _current_source_citations(value,members,current):
 require(type(current)is bool,'Citation mode must be boolean');found={};todo=[value]
 while todo:
  item=todo.pop()
  if type(item)is list:todo.extend(item);continue
  if type(item)is not dict:continue
  if {'path','sha256'}<=set(item)and set(item)<={'path','sha256','bytes'}:
   require(type(item['path'])is str,'Invalid citation path');path=item['path'];matches=[p for p,row in members.items()if path==p or path==row['archive']['path']]
   source_domain=path.startswith('crates/')or path in ['Cargo.toml','Cargo.lock','rust-toolchain.toml','.cargo/config.toml']
   require(not source_domain or bool(matches),'Source citation not in the matching snapshot')
   if not matches:continue
   require(len(matches)==1,'Ambiguous archived Source citation');p=matches[0];row=members[p]
   require(type(item['sha256'])is str and item['sha256']==row['archive']['sha256']and('bytes'not in item or type(item['bytes'])is int and item['bytes']==row['size_bytes']),'Stale Source citation metadata')
   archive=_source_ref(row['archive']);require(len(archive.read_bytes())==row['size_bytes'],'Source citation archive size differs')
   if current:
    expected=dict(path=p,sha256=row['archive']['sha256'],bytes=row['size_bytes']);require(item==expected,'Current Source citation must use exact current identity');_canonical_pin(expected);pin=expected
   else:
    # Old live-path citations are authenticated against their immutable archive,
    # never by reading the now-current Source path or normalizing old authority.
    require(path==p or path==row['archive']['path'],'Historical Source citation identity');pin=ref(archive)
   require(p not in found or found[p]==pin,'Conflicting Source citations');found[p]=pin
  else:todo.extend(item.values())
 return found


def _qa_run_contexts(evidence,all_runs,stages,before_build,final_plan,pins):
 plan_keys={'schema','id','before_build','source_prepared_manifest','adoption','current_adoption','prior_format_stages','prior_quality_summaries','current_Source_primary_review','historical_refs','tools','release','QA_profile','QA_profile_binding','prior_QA_run_contexts','launch_environment_selected'}
 environment_keys={'RUSTFLAGS','CARGO_ENCODED_RUSTFLAGS','CARGO_BUILD_TARGET','CARGO_TARGET_DIR','RUSTUP_TOOLCHAIN','RUSTC','RUSTC_WRAPPER','RUSTC_WORKSPACE_WRAPPER','CARGO_HOME'}
 contexts=evidence['QA_run_contexts'];require(type(contexts)is list and len(contexts)==len(all_runs),'Incomplete QA run context history')
 _canonical_pin(final_plan);plans={};result={};authored=[edge['review']for edge in stages if edge['kind']!='format']
 for context,row in zip(contexts,all_runs,strict=True):
  exact_keys(context,{'receipt','plan'},'QA run context');require(context['receipt']==row['receipt'],'QA run context order/identity differs')
  _canonical_pin(context['receipt']);_canonical_pin(context['plan']);key=(context['plan']['path'],context['plan']['sha256'])
  if key not in plans:
   plan=load(context['plan']);exact_keys(plan,plan_keys,'Source production plan')
   require(plan['schema']=='paired.SourceProductionPlan.v3'and type(plan['id'])is str and plan['id']and plan['before_build']==before_build,'Historical QA plan/baseline differs')
   environment=plan['launch_environment_selected'];exact_keys(environment,environment_keys,'selected launch environment');require(all(value is None or type(value)is str for value in environment.values()),'Invalid selected launch environment value')
   prefix=plan['prior_format_stages'];require(type(prefix)is list and prefix==stages[:len(prefix)]and len(prefix)<len(stages),'QA plan stage prefix differs')
   prior_rows=[]
   for prior_ref in plan['prior_quality_summaries']:
    _canonical_pin(prior_ref);summary=load(prior_ref);require(summary['stage']=='quality','Prior QA summary stage differs');prior_rows+=summary['runs']
   require(prior_rows==all_runs[:len(prior_rows)]and len(prior_rows)<=len(result),'Historical plan prior summaries are not its actual past prefix')
   prior_contexts=plan['prior_QA_run_contexts'];require(type(prior_contexts)is list and prior_contexts==contexts[:len(prior_rows)]and [item['receipt']for item in prior_contexts]==[item['receipt']for item in prior_rows],'Historical plan omitted/changed prior receipt-plan context mapping')
   prior_authored=[edge['review']for edge in prefix if edge['kind']!='format']
   if prior_authored and prior_authored[-1]==plan['current_adoption']:
    format_index=len(prefix);require(stages[format_index]['kind']=='format','Unchanged-source retry must append only a genuine format stage');own_authored=prior_authored
   else:
    active_index=len(prefix);active=stages[active_index];require(active['kind']in ['adopt','correction']and active['review']==plan['current_adoption'],'QA plan current adoption/stage differs');own_authored=prior_authored+[plan['current_adoption']];format_index=active_index+1
    require(format_index<len(stages)and stages[format_index]['kind']=='format','QA lacks its actual post-adoption format stage')
   require(plan['adoption']==stages[0]['review'],'QA plan initial adoption differs')
   require(own_authored==authored[:len(own_authored)],'Historical QA authored-stage prefix differs')
   closure=_AuthorityPins(pins.epoch);profile=_qa_profile_binding(plan['QA_profile'],plan['QA_profile_binding'],before_build,own_authored,closure)
   require(load(plan['QA_profile_binding'])['authored_stages'][0]['prepared_manifest']==plan['source_prepared_manifest'],'QA plan initial Source manifest differs')
   _canonical_pin(plan['release']);installed=ref(Path(__file__).with_name('release.json'));require(plan['release']in [FROZEN_V5_RELEASE,FROZEN_V6_RELEASE,FROZEN_V7_RELEASE,FROZEN_V8_RELEASE,FROZEN_V10_RELEASE,FROZEN_V11_RELEASE,FROZEN_V12_RELEASE,installed],'QA plan must use an exact frozen v5/v6/v7/v8/v10 v11/v12 or installed v13 release')
   release=load(plan['release']);expected_schema='paired.Release.v5'if plan['release']==FROZEN_V5_RELEASE else 'paired.Release.v6'if plan['release']==FROZEN_V6_RELEASE else 'paired.Release.v7'if plan['release']==FROZEN_V7_RELEASE else 'paired.Release.v8'if plan['release']==FROZEN_V8_RELEASE else 'paired.Release.v10'if plan['release']==FROZEN_V10_RELEASE else 'paired.Release.v11'if plan['release']==FROZEN_V11_RELEASE else 'paired.Release.v12'if plan['release']==FROZEN_V12_RELEASE else 'paired.Release.v13';require(release['schema']==expected_schema and release['Source_metadata_schema']==2 and release['Source_evidence_schema']==3,'QA plan release differs')
   require(set(plan['tools'])=={'recorder','freezer'},'QA plan tools differ')
   required=[context['plan'],plan['release'],before_build,plan['source_prepared_manifest'],plan['adoption'],plan['current_adoption'],plan['current_Source_primary_review'],*plan['prior_quality_summaries'],*plan['historical_refs'],*plan['tools'].values(),*release['files'].values(),*release['tools'].values(),release['frozen_v1_release'],release['frozen_v2_release'],release['frozen_v3_release'],*closure]
   require(release['frozen_v4_release']==FROZEN_V4_RELEASE,'Frozen v4 release differs');required.append(release['frozen_v4_release'])
   if plan['release']in [FROZEN_V6_RELEASE,FROZEN_V7_RELEASE,FROZEN_V8_RELEASE,FROZEN_V10_RELEASE,FROZEN_V11_RELEASE,FROZEN_V12_RELEASE,installed]:require(release['frozen_v5_release']==FROZEN_V5_RELEASE,'Frozen v5 release differs');required.append(release['frozen_v5_release'])
   if plan['release']in [FROZEN_V7_RELEASE,FROZEN_V8_RELEASE,FROZEN_V10_RELEASE]:
    require(release['frozen_v6_release']==FROZEN_V6_RELEASE and release['normalization_validation_epoch']==NORMALIZATION_EPOCH_CONTRACT,'Frozen v6/normalization epoch contract differs');required.append(release['frozen_v6_release'])
   if plan['release']in [FROZEN_V11_RELEASE,FROZEN_V12_RELEASE]:
    require(release['frozen_v6_release']==FROZEN_V6_RELEASE and release['normalization_validation_epoch']==NORMALIZATION_EPOCH_V2_CONTRACT,'Frozen v11/v12 V2 normalization contract differs');required.append(release['frozen_v6_release'])
   if plan['release']==installed:
    require(release['frozen_v6_release']==FROZEN_V6_RELEASE and release['normalization_validation_epoch']==NORMALIZATION_EPOCH_V3_CONTRACT,'Installed v13/V3 normalization contract differs');required.append(release['frozen_v6_release'])
   if plan['release']in [FROZEN_V8_RELEASE,FROZEN_V10_RELEASE,FROZEN_V11_RELEASE,FROZEN_V12_RELEASE,installed]:
    require(release['frozen_v7_release']==FROZEN_V7_RELEASE and release['quality_preflight_observation_epoch']==QUALITY_PREFLIGHT_EPOCH_CONTRACT,'Frozen v7/QA preflight epoch contract differs');required.append(release['frozen_v7_release'])
   if plan['release']in [FROZEN_V10_RELEASE,FROZEN_V11_RELEASE,FROZEN_V12_RELEASE,installed]:
    require(release['frozen_v8_release']==FROZEN_V8_RELEASE and release['Source_recorder_invocation']==SOURCE_RECORDER_INVOCATION_CONTRACT,'Frozen v8/Source recorder invocation contract differs');required.append(release['frozen_v8_release'])
   if plan['release']in [FROZEN_V11_RELEASE,FROZEN_V12_RELEASE,installed]:
    require(release['frozen_v10_release']==FROZEN_V10_RELEASE,'Frozen v10 release differs');required.append(release['frozen_v10_release'])
   if plan['release']==installed:
    require(release['frozen_v11_release']==FROZEN_V11_RELEASE and release['frozen_v12_release']==FROZEN_V12_RELEASE,'Frozen v11/v12 release differs');required.extend([release['frozen_v11_release'],release['frozen_v12_release']])
   for pin in required:_authority_add(pins,pin)
   plans[key]=dict(plan=plan,profile=profile,commands=qa_profile(plan['QA_profile']),required=required,format_index=format_index)
  info=plans[key];plan=info['plan'];r=_receipt(row['receipt']);_authority_add(pins,row['receipt']);_source_ref(r['source_snapshot'],pins.epoch)
  require(type(r['exit_code'])is int and type(row['exit_code'])is int and type(row['recorder_exit'])is int,'Invalid recorded QA exit type')
  require(type(row['name'])is str and row['name']in [name for name,command in info['commands']],'QA invocation name not in its contemporaneous profile')
  expected=dict(info['commands'])[row['name']];require(r['command']==expected,'Historical QA command differs from its own profile')
  exact_keys(r['launch_environment_selected'],environment_keys,'actual selected launch environment')
  require(all(value is None or type(value)is str for value in r['launch_environment_selected'].values())and r['launch_environment_selected']==plan['launch_environment_selected'],'QA launch environment differs from contemporaneous plan')
  format_edge=stages[info['format_index']]
  is_format=expected[:2]==['cargo','fmt']and '--check'not in expected
  expected_snapshot=format_edge['before_snapshot']if is_format else format_edge['after_snapshot']
  require(r['source_snapshot']==expected_snapshot,'QA raw/formatted source stage position differs')
  custody=r.get('explicit_input_custody');require(type(custody)is dict and type(custody.get('before'))is list and type(custody.get('after'))is list,'Missing contemporaneous QA explicit-input observations')
  observed={}
  for entry in custody['before']:
   if type(entry)is dict and set(entry)=={'path','sha256','bytes'}:
    _canonical_pin(entry);require(entry['path']not in observed,'Duplicate QA explicit input');observed[entry['path']]=entry
  own_plan=context['plan'];require(observed.get(own_plan['path'])==own_plan,'QA receipt did not observe its stated plan')
  success=r['exit_code']==0 and r['launch_error']is None
  if success:
   if is_format:require(format_edge['receipt']==row['receipt'],'Successful format invocation/stage receipt differs')
   require(custody['all_explicit_inputs_unchanged']is True and custody['command_process_started']is True and custody['read_errors_before']==custody['read_errors_after']==[]and custody['before']==custody['after'],'Successful QA lacks unchanged actual explicit-input closure')
   for pin in info['required']:
    exact=ref(auth(pin));require(observed.get(exact['path'])==exact,'Successful QA omitted its contemporaneous authenticated input')
   if expected[:2]==['cargo','test']:
    body=next(body for body in info['profile']['required_bodies']if body['test_name']==expected[-3]);_qa_test_declaration(body,_authority_snapshot(r['source_snapshot'],pins),pins.epoch)
  if row in load(evidence['quality'])['runs']:require(context['plan']==final_plan,'Prior plan receipt cannot be relabeled current')
  require(row['receipt']['path']not in result,'Duplicate QA context receipt');result[row['receipt']['path']]=info
 final=load(final_plan);require(final['QA_profile']==evidence['QA_profile']and final['QA_profile_binding']==evidence['QA_profile_binding'],'Final evidence/profile binding differs')
 for summary_ref in load(evidence['quality'])['source_summaries']:
  summary=load(summary_ref);rows=summary['runs']
  if not rows:continue
  own=[result[row['receipt']['path']]for row in rows];require(all(item is own[0]for item in own),'A QA summary mixes contemporaneous plans')
  expected=own[0]['commands'];require([row['name']for row in rows]==[name for name,command in expected[:len(rows)]]and len(rows)<=len(expected),'Historical QA summary is not its actual profile prefix')
  require(all(row['exit_code']==row['recorder_exit']==0 for row in rows[:-1]),'QA summary continued after a failed invocation')
 return result


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
def original_idf_bundle(pin):
 from . import original_idf
 return original_idf.load_contract(pin)

def original_idf_tooling_dependency(pin,release_pin,release,authority):
 from . import original_idf
 return original_idf.tooling_dependency(pin,release_pin,release,authority)

def case_document(case):
 if case['schema']=='paired.CaseConfig.v1':return verify_ledger(case)
 from . import original_idf
 return original_idf.verify_case(case)['document']

def case_gate_ids(case,release,category):
 return release['original_IDF_gate_ids'][category]if case['schema']=='paired.CaseConfig.v2'else release['gate_ids'][category]

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
def _warning_contract_closure(release):
 require(release['warning_phase_contract']==WARNING_PHASE_CONTRACT,'Exact typed warning contract differs')
 warning=load(release['warning_phase_contract'])
 exact_keys(warning,{'schema','inherited_warning_phase_contract','owned_family','primary'},'Typed warning contract')
 require(warning['schema']=='paired.WarningPhaseContract.v2'and warning['inherited_warning_phase_contract']==INHERITED_WARNING_PHASE_CONTRACT and warning['owned_family']['schema']=='EcoRoof.AdvancedStability.v1','Warning contract lineage/family differs')
 require(type(warning['primary'])is list and len(warning['primary'])==7,'Closed warning primary reference set required')
 pins=[release['warning_phase_contract'],warning['inherited_warning_phase_contract']]
 for row in warning['primary']:
  exact_keys(row,{'ref','scope'},'Warning primary citation');require(type(row['scope'])is str,'Warning primary scope');pins.append(row['ref'])
 for pin in pins:_canonical_pin(pin);auth(pin)
 return pins

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
 require(mode=='fresh','OA topology profile is fresh-only');binding=load(binding_ref);require(binding['schema']=='paired.RuntimeBinding.v1'and binding['mode']==mode,'Binding mode');release=load(binding['release']);require(release['schema']=='paired.Release.v13'and release['frozen_v12_release']==FROZEN_V12_RELEASE and release['tooling_dependency_schemas']==['paired.VersionedToolingDependency.v1','paired.PublishedToolingDependency.v1']and release['frozen_v11_release']==FROZEN_V11_RELEASE and release['frozen_v10_release']==FROZEN_V10_RELEASE and release['warning_phase_contract']==WARNING_PHASE_CONTRACT and release['constant_scientific_sha256']==NEW_CONSTANT_SCIENTIFIC_SHA256 and release['frozen_v8_release']==FROZEN_V8_RELEASE and release['Source_recorder_invocation']==SOURCE_RECORDER_INVOCATION_CONTRACT and release['frozen_v7_release']==FROZEN_V7_RELEASE and release['quality_preflight_observation_epoch']==QUALITY_PREFLIGHT_EPOCH_CONTRACT and release['normalization_validation_epoch']==NORMALIZATION_EPOCH_V3_CONTRACT and release['frozen_v6_release']==FROZEN_V6_RELEASE and release['profile']==PROFILE and release['structure_schema']=='paired.OATopologyStructure.v1'and type(release['Source_metadata_schema'])is int and release['Source_metadata_schema']==2 and type(release['Source_evidence_schema'])is int and release['Source_evidence_schema']==3,'Release schema')
 for key in ['Root_review','independent_review']:
  review=load(binding[key]);require(review['schema']=='paired.RuntimeReview.v1'and review['accepted']is True and review['release']==binding['release']and review['case_config']==binding['case_config']and review['source_authority']==binding['source_authority']and review['mode']==mode,'External review binding differs')
 case=load(binding['case_config']);exact_keys(case,CASE_KEYS|({'input_contract'}if case['schema']=='paired.CaseConfig.v2'else set()),'CaseConfig');require(case['schema']in ['paired.CaseConfig.v1','paired.CaseConfig.v2']and case['id']==case_id and case['profile']==PROFILE and case['source_authority']==binding['source_authority'],'Case profile/identity');authority=load_authority(binding['source_authority'],mode)
 require((case['schema']=='paired.CaseConfig.v2')==(case['structure']['schema']=='paired.OATopologyStructure.v2')and(case['schema']=='paired.CaseConfig.v1'or case['input_contract']==case['structure']['input_contract']),'Case/structure input format contract');tooling=None
 # Both new v13 Case schemas bind the same genuinely reviewed tooling dependency.
 reviews=[load(binding[key])for key in ['Root_review','independent_review']]
 for role,review in zip(['Root','independent'],reviews,strict=True):require(review['review_role']==role and review['accepted_static_only']is True and review['findings']==[],'Genuine v13 runtime review')
 require(reviews[0]['tooling_dependency']==reviews[1]['tooling_dependency'],'Runtime tooling dependency identity')
 tooling=original_idf_tooling_dependency(reviews[0]['tooling_dependency'],binding['release'],release,authority)
 require(set(case['scientific'])==set(release['scientific_fields']),'Scientific input field set');document=case_document(case);policy=load(case['policy']);counts=case['counts'];require(counts['total_series']==len(policy['rows'])+len(policy['meter_rows'])and counts['variable_values']==len(policy['rows'])*counts['zone_steps_per_variable']and counts['meter_values']==len(policy['meter_rows'])*counts['meter_hour_samples']and counts['total_values']==counts['variable_values']+counts['meter_values'],'Exact numeric dimensions')
 require(digest(typed({k:case['scientific'][k]for k in release['constant_scientific_fields']}))==release['constant_scientific_sha256'],'Frozen scientific contract changed')
 require(case['scientific']['warning_phase_contract']==release['warning_phase_contract'],'Case warning contract differs')
 inherited_constants={k:case['scientific'][k]for k in release['constant_scientific_fields']};inherited_constants['warning_phase_contract']=INHERITED_WARNING_PHASE_CONTRACT
 require(digest(typed(inherited_constants))==load(FROZEN_V10_RELEASE)['constant_scientific_sha256'],'Other37 frozen scientific constants changed')
 require(case['policy']==release['policy']and counts==release['counts']and counts['ESO_series']==len(policy['rows'])and counts['MTR_series']==len(policy['meter_rows']),'Frozen profile policies/dimensions differ')
 require(case['baseline_input']==release['baseline_input'],'Initial profile baseline differs')
 require(case['structure']['prediction']==case['scientific']['structural_prediction'],'Two structural authorities differ')
 bundle=_topology_contract(case['structure']);structure_baseline(case['structure']);require(bundle['prediction']['input']==case['input']and bundle['exports']['input']==case['input'],'Topology input identity')
 require(case['structure']['parent']['release']in [binding['release'],*release['compatible_parent_releases']],'Topology parent release differs')
 adoption=load(authority['adoption']);require(adoption['actual_before_Source_build']==bundle['parent_authority']['build'],'Actual Source predecessor differs from published topology parent')
 exact_keys(case['aliases'],{'fuel_case','gate_case','original_exhaust_recipe','science_contract'},'profile aliases')
 require(len(policy['rows'])>155 and counts['DesignDays']==5 and counts['zone_steps_per_variable']==480 and counts['meter_hour_samples']==120,'Unsupported initial profile dimensions')
 recipe=copy.deepcopy(case['scientific']);recipe.update(input=case['input'],baseline_input=case['baseline_input'],scientific_dom=case['scientific_dom'],derived_policy=case['policy'],counts=counts,case_kind=case['case_kind'],inherited_heating_efficiency_case_kind=case['aliases']['fuel_case'],inherited_exhaust_baseline_recipe=case['aliases']['original_exhaust_recipe'],structure_config=case['structure'],science_contract=case['aliases']['science_contract'])
 recipe['exact_empty_selector_science_contract']=recipe['science_contract'];recipe.update(scope='B',format='epJSON',weather=None,weather_cli_argument=None,hours_cli_argument=None,DDOnly_cli_argument=False,Native_convert_argument=False)
 if case['schema']=='paired.CaseConfig.v2':recipe.update(format='original_IDF',input_contract=case['input_contract'])
 require(recipe['inherited_heating_efficiency_case_kind']=='POSITIVE'and case['aliases']['gate_case']=='SUM','Unexpected profile ancestry');envs=load(case['clock'])['environments'];require(len(envs)==5 and [x['title']for x in envs]==(original_idf_bundle(case['input_contract'])['orders']['SizingPeriod:DesignDay']if case['schema']=='paired.CaseConfig.v2'else sorted(document['SizingPeriod:DesignDay'])),'Clock basis')
 require(case['clock']==recipe['ordered_environment_clock_basis']and case['aliases']['original_exhaust_recipe']==release['original_exhaust_recipe'],'Clock/exhaust ancestry differs')
 require(authority['current_Source_primary_review']==recipe['current_Source_primary_review'],'Current return-flow authority differs')
 source_rows=snapshot(authority['snapshot'],mode=='fresh');documentary=case['documentary'];require(type(documentary)is list,'Documentary source list')
 for row in documentary:
  exact_keys(row,{'historical_path','archive'},'documentary source');auth(row['archive'])
 # Every current citation belongs to the actual build. Retained mode accepts only
 # the matching explicitly remapped archive for that same historical path.
 source_pins=_current_source_citations([recipe,bundle['derivation']['primary'],bundle['derivation']['report_cutoff']],source_rows,True)
 parent_rows=snapshot(bundle['parent_authority']['snapshot']);parent_derivation=load(bundle['parent_case']['structure']['derivation'])
 parent_pins=_current_source_citations([bundle['parent_case']['scientific'],parent_derivation['primary'],parent_derivation['report_cutoff']],parent_rows,False)
 expected_citations=(set(parent_pins)-set(authority['deleted_paths']))|{path for path in authority['added_paths']if path.endswith('.rs')}
 require(set(source_pins)==expected_citations,'Complete current Source citation set differs from parent/additions/deletions')
 pp=load(recipe['primary_pins']);pins=bundle['pins']+[binding_ref,binding['release'],binding['case_config'],binding['source_authority'],binding['Root_review'],binding['independent_review']]+reference_closure(case)+_warning_contract_closure(release)+list(release['files'].values())+authority['pins']
 for key in ['edit_ledger','clock']:
  pins+=reference_closure(load(case[key]))
 ledger=load(original_idf_bundle(case['input_contract'])['contract']['semantic_edit_ledger']if case['schema']=='paired.CaseConfig.v2'else case['edit_ledger']);pins+=reference_closure(load(ledger['numeric_baseline']))
 if case['schema']=='paired.CaseConfig.v2':pins+=original_idf_bundle(case['input_contract'])['pins']
 pins+=reference_closure(load(case['aliases']['science_contract']))+[release['recorder']]
 for p in pp['Native_bundle'].values():auth(p);pins.append(p)
 if case['schema']=='paired.CaseConfig.v2':
  conversion=original_idf_bundle(case['input_contract'])['contract']['conversion'];require(conversion['api']in pp['Native_bundle'].values()and auth(conversion['executable']).parent==auth(pp['Native_bundle']['exe']).parent,'Actual converter must belong to pinned Native bundle')
 pins+=[bundle['parent_authority']['snapshot'],*parent_pins.values()]
 if tooling is not None:pins+=tooling['pins']
 unique={x['path']:ref(auth(x))for x in pins};require(type(authority['inventory_count'])is int and len(source_rows)==authority['inventory_count'],'Actual Source inventory count')
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



def command(engine,build,bundle,input_path,output,input_contract=None):
    if engine=='native':return [str(c.auth(bundle['exe'])),'--output-directory',str(output),'--output-prefix','native',str(input_path)]
    command=[str(c.auth(build['retained_executable'])),'run',str(input_path),'--output-dir',str(output),'--mode','compatibility','--partial','deny','--porting-scope','B','--trace-level','debug']
    if input_contract is not None:
        conversion=original_idf_bundle(input_contract)['contract']['conversion'];command+=['--oracle-root',str(auth(conversion['executable']).parent)]
    return command



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
    """Apply one exact unified patch, including standard incomplete-line markers."""
    assert type(raw)is bytes and type(patch)is bytes
    source=raw.splitlines(keepends=True);diff=patch.splitlines(keepends=True)
    output=[];cursor=0;i=0;seen=set();source_eof=False;target_eof=False
    marker=b'\\ No newline at end of file'
    if diff and diff[0].startswith(b'--- '):
        assert len(diff)>=2 and diff[1].startswith(b'+++ ')
        i=2
    while i<len(diff):
        match=c.re.fullmatch(rb'@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@[^\r\n]*(?:\r?\n)?',diff[i]);assert match
        old_count=int(match[2]or b'1');new_count=int(match[4]or b'1')
        old_at=int(match[1])-(1 if old_count else 0)
        new_at=int(match[3])-(1 if new_count else 0)
        assert old_count or new_count
        assert 0<=cursor<=old_at<=len(source) and new_at>=0
        key=(old_at,old_count,new_at,new_count);assert key not in seen;seen.add(key)
        untouched=source[cursor:old_at]
        if untouched:
            assert not source_eof and not target_eof
            output.extend(untouched)
            if not untouched[-1].endswith((b'\n',b'\r')):
                assert old_at==len(source);source_eof=target_eof=True
        cursor=old_at;assert new_at==len(output)
        old_used=new_used=0;i+=1
        while old_used<old_count or new_used<new_count:
            assert i<len(diff)
            change=diff[i];kind=change[:1];assert kind in [b' ',b'-',b'+']
            payload=change[1:];i+=1;marked=False
            if i<len(diff) and diff[i].startswith(b'\\'):
                assert diff[i]in [marker,marker+b'\n',marker+b'\r\n']
                assert payload.endswith(b'\n')
                payload=payload[:-1];i+=1;marked=True
            assert payload
            incomplete=marked or not payload.endswith((b'\n',b'\r'))
            if kind in [b' ',b'-']:
                assert not source_eof and old_used<old_count and cursor<len(source)
                assert source[cursor]==payload
                cursor+=1;old_used+=1
                if incomplete:
                    assert cursor==len(source);source_eof=True
            if kind in [b' ',b'+']:
                assert not target_eof and new_used<new_count
                output.append(payload);new_used+=1
                if incomplete:target_eof=True
        assert old_used==old_count and new_used==new_count
        assert cursor==old_at+old_count and len(output)==new_at+new_count
    tail=source[cursor:]
    if tail:assert not source_eof and not target_eof
    output.extend(tail);return b''.join(output)


def _topology_csv(raw):
    require(type(raw) is bytes and raw.endswith(b'\n'), 'Complete topology body required')
    text = raw.decode('utf8').replace('\r\n', '\n')
    require('\r' not in text and '\x00' not in text and not text.startswith('Program Version,'), 'Invalid topology body framing')
    return list(csv.reader(io.StringIO(text), strict=True))


def _published_parent_member(parent, parent_case, parent_release, publication, data):
    """Select a real accepted parent; data freshly authenticates and records each read."""
    if publication['schema'] == 'Root-paired-domain-publication.v2':
        require(publication['schema'] == 'Root-paired-domain-publication.v2' and publication['worktree_clean'] is True and publication['release'] == parent['release'] and publication['new_qualifying_cases'] == 1 and publication['new_qualifying_values'] == 75120, 'Accepted parent publication required')
        parent_completed = data(publication['completed'])
        require(parent_completed['schema'] == 'paired.Completed.v1' and parent_completed['mode'] == 'fresh' and parent_completed['scientific_acceptance'] is True and parent_completed['qualifying_cases'] == 1 and parent_completed['qualifying_values'] == 75120 and parent_completed['case_config'] == parent['case'] and parent_completed['source_authority'] == parent_case['source_authority'] and parent_completed['release'] == parent['release'], 'Parent is not the accepted published case')
        return dict(publication_schema=publication['schema'], member_index=None, member=None, completed=publication['completed'], pins=[copy.deepcopy(publication['completed'])])
    require(publication['schema'] == 'Root-paired-domain-batch-publication.v1', 'Unknown parent publication schema')
    require(publication['worktree_clean'] is True and publication['release'] == parent['release'], 'Accepted batch parent publication required')
    require(type(publication['head']) is str and publication['head'] and publication['head'] == publication['remote_head'], 'Batch parent publication HEAD differs')
    require('completed' not in publication and 'Root_scientific_review' not in publication, 'Batch parent must retain real member references')
    require(publication['Source_authority'] == parent_case['source_authority'], 'Batch parent Source authority differs')
    members = publication['members']
    require(type(members) is list and len(members) >= 1, 'Batch parent requires real members')
    require(type(publication['new_qualifying_cases']) is int and publication['new_qualifying_cases'] == len(members), 'Batch parent case count differs')
    require(type(parent_release['counts']['total_values']) is int and parent_release['counts']['total_values'] == 75120, 'Unsupported parent member dimensions')
    require(type(publication['new_qualifying_values']) is int and publication['new_qualifying_values'] == len(members) * parent_release['counts']['total_values'], 'Batch parent value count differs')
    pins = {}
    completed_refs, case_refs, case_ids, matches = set(), set(), set(), []
    for index, member in enumerate(members):
        exact_keys(member, {'completed', 'Root_scientific_review', 'fixed_input_manifest', 'independent_input_review'}, 'published parent member')
        for pin in member.values():
            exact_keys(pin, {'path', 'sha256', 'bytes'}, 'published parent member pin')
            require(type(pin['path']) is str and pin['path'] and '\\' not in pin['path'] and ':' not in pin['path'] and all(part and part not in ['.', '..'] for part in pin['path'].split('/')), 'Invalid published member path')
            require(type(pin['sha256']) is str and re.fullmatch('[0-9a-f]{64}', pin['sha256']) is not None and type(pin['bytes']) is int and pin['bytes'] >= 0, 'Invalid published member pin')
        complete = data(member['completed'])
        require(complete['schema'] == 'paired.Completed.v1' and complete['mode'] == 'fresh' and complete['scientific_acceptance'] is True, 'Unaccepted batch parent member')
        require(type(complete['qualifying_cases']) is int and complete['qualifying_cases'] == 1 and type(complete['qualifying_values']) is int and complete['qualifying_values'] == 75120, 'Batch member qualifying dimensions differ')
        require(complete['release'] == parent['release'] and complete['source_authority'] == publication['Source_authority'], 'Batch member release/Source authority differs')
        require(type(complete['actual_Native_engine_calls']) is int and complete['actual_Native_engine_calls'] == 1 and type(complete['actual_Source_engine_calls']) is int and complete['actual_Source_engine_calls'] == 1, 'Batch member actual actor counts differ')
        exact_keys(complete['case_config'], {'path', 'sha256', 'bytes'}, 'published member case pin')
        case_pin = complete['case_config']
        require(type(case_pin['path']) is str and case_pin['path'] and '\\' not in case_pin['path'] and ':' not in case_pin['path'] and all(part and part not in ['.', '..'] for part in case_pin['path'].split('/')), 'Invalid published member case path')
        require(type(case_pin['sha256']) is str and re.fullmatch('[0-9a-f]{64}', case_pin['sha256']) is not None and type(case_pin['bytes']) is int and case_pin['bytes'] >= 0, 'Invalid published member case pin')
        completion_key = tuple(member['completed'][key] for key in ['path', 'sha256', 'bytes'])
        case_key = tuple(complete['case_config'][key] for key in ['path', 'sha256', 'bytes'])
        require(type(complete['case_id']) is str and complete['case_id'], 'Batch member case id required')
        require(completion_key not in completed_refs and case_key not in case_refs and complete['case_id'] not in case_ids, 'Duplicate published parent member')
        completed_refs.add(completion_key)
        case_refs.add(case_key)
        case_ids.add(complete['case_id'])
        # Fresh authentication occurs through data even for repeated references;
        # pins is only a detached ordered closure descriptor, never a read cache.
        for key in ['completed']:
            pin = member[key]
            if pin['path'] in pins:
                require(pins[pin['path']] == pin, 'Conflicting published parent pin')
            pins[pin['path']] = copy.deepcopy(pin)
        if complete['case_config'] == parent['case']:
            require(complete['case_id'] == parent_case['id'], 'Selected parent case id differs')
            matches.append((index, member, complete))
    require(len(matches) == 1, 'Exactly one real published parent member must match case Ref3')
    index, member, complete = matches[0]
    selected = {}
    for key in ['Root_scientific_review', 'fixed_input_manifest', 'independent_input_review']:
        pin = member[key]
        selected[key] = data(pin)
        if pin['path'] in pins:
            require(pins[pin['path']] == pin, 'Conflicting published parent pin')
        pins[pin['path']] = copy.deepcopy(pin)
    review, fixed, peer = [selected[key] for key in ['Root_scientific_review', 'fixed_input_manifest', 'independent_input_review']]
    require(review['schema'] == 'Root.CompletedPairedScientificReview.v1' and review['accepted'] is True and review['scientific_acceptance'] is True, 'Selected parent scientific review is unaccepted')
    require(review['completion'] == member['completed'] and review['case_config'] == parent['case'] and review['Source_authority'] == parent_case['source_authority'] and review['release'] == parent['release'], 'Selected parent scientific review identity differs')
    require(review['readback'] == complete['paired'] and review['binding'] == complete['bindings'], 'Selected parent scientific readback/binding differs')
    require(fixed['input'] == parent['input'] == parent_case['input'], 'Selected parent fixed input differs')
    require(peer['accepted'] is True and peer['accepted_static_only'] is True and peer['findings'] == [] and member['fixed_input_manifest'] in peer.values(), 'Selected parent input review differs')
    return dict(publication_schema=publication['schema'], member_index=index, member=copy.deepcopy(member), completed=copy.deepcopy(member['completed']), pins=list(pins.values()))


def _topology_contract(spec):
    idf = spec['schema'] == 'paired.OATopologyStructure.v2'
    extra = {'input_contract'} if idf else set()
    exact_keys(spec, {'schema', 'prediction', 'parent', 'parent_edit_ledger', 'parent_inverse_operations', 'body_overlay', 'source_exports', 'derivation', 'prediction_reviews'} | extra, 'OA topology structure')
    require(spec['schema'] in ['paired.OATopologyStructure.v1', 'paired.OATopologyStructure.v2'], 'Unknown topology schema')
    input_bundle = original_idf_bundle(spec['input_contract']) if idf else None
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
    require(parent_case['schema'] in ['paired.CaseConfig.v1', 'paired.CaseConfig.v2'] and parent_case['profile'] == PROFILE and parent_case['input'] == parent['input'] and parent_case['structure']['prediction'] == parent['prediction'], 'Parent case identity')
    require(parent['release'] in [FROZEN_V5_RELEASE, FROZEN_V6_RELEASE, FROZEN_V7_RELEASE, FROZEN_V8_RELEASE, FROZEN_V10_RELEASE, FROZEN_V11_RELEASE, FROZEN_V12_RELEASE, ref(Path(__file__).with_name('release.json'))], 'Exact compatible topology parent release required')
    require(parent_release['profile'] == parent_case['profile'], 'Parent profile/release')
    parent_selection = _published_parent_member(parent, parent_case, parent_release, publication, data)
    parent_authority = data(parent_case['source_authority'])
    require(publication['Source_authority'] == parent_case['source_authority'] and publication['actual_Source_build'] == parent_authority['build'], 'Parent Source publication identity')
    data(parent_authority['build'])
    parent_prediction = data(parent['prediction'])
    parent_body = take(parent_prediction['expected_body'])
    parent_rows = _topology_csv(parent_body)
    # Preserve both existing epJSON-parent routes; an IDF parent always owns its whole converted DOM.
    parent_semantic = parent_case['scientific_dom'] if idf or parent_case['schema'] == 'paired.CaseConfig.v2' else parent['input']
    parent_document = data(parent_semantic)
    prediction = data(spec['prediction'])
    exact_keys(prediction, {'schema', 'input', 'parent', 'expected_body', 'rows', 'counts', 'nodes', 'connections', 'outdoor_registry', 'roles', 'derivation', 'source_exports'} | extra, 'OA topology prediction')
    require(prediction['schema'] == ('paired.OATopologyPrediction.v2' if idf else 'paired.OATopologyPrediction.v1') and prediction['parent'] == parent and prediction['derivation'] == spec['derivation'] and prediction['source_exports'] == spec['source_exports'], 'Prediction identity')
    if idf:
        require(prediction['input_contract'] == spec['input_contract'] and prediction['input'] == input_bundle['contract']['input'], 'Original-IDF topology contract identity')
        for value in input_bundle['pins']: take(value)
    document = input_bundle['document'] if idf else data(prediction['input'])
    semantic_input = input_bundle['contract']['scientific_dom'] if idf else prediction['input']
    semantic_parent = parent_semantic
    expected = take(prediction['expected_body'])
    rows = _topology_csv(expected)
    require(type(prediction['rows']) is list and prediction['rows'] == rows, 'Complete predicted CSV rows differ')
    ledger = data(spec['parent_edit_ledger'])
    require(ledger['baseline_input'] == semantic_parent and ledger['input'] == semantic_input, 'Parent input ledger identity')
    # Reuse the unchanged lexical/numeric/string/object-inverse kernel.
    verify_ledger(dict(baseline_input=semantic_parent, input=semantic_input, scientific_dom=semantic_input, edit_ledger=spec['parent_edit_ledger'], inverse_operations=spec['parent_inverse_operations']))
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
    exact_keys(exports, {'schema', 'input', 'parent_input', 'objects', 'object_order', 'core_nodelist_order'} | extra, 'topology Source exports')
    require(exports['schema'] == ('paired.OATopologySourceExports.v2' if idf else 'paired.OATopologySourceExports.v1') and exports['input'] == prediction['input'] and exports['parent_input'] == parent['input'], 'Export contract identity')
    if idf: require(exports['input_contract'] == spec['input_contract'], 'Source exports original-IDF contract')
    exact_keys(exports['objects'], {'NodeList', 'SetpointManager:Scheduled'}, 'exported families')
    exact_keys(exports['object_order'], {'NodeList', 'SetpointManager:Scheduled'}, 'exported orders')
    core_order = parent_case['scientific']['nodelist_order']
    require(type(core_order) is list and len(core_order) == len(set(core_order)) == 5 and exports['core_nodelist_order'] == core_order, 'Inherited five core NodeLists')
    for family in ['NodeList', 'SetpointManager:Scheduled']:
        require(type(document.get(family)) is dict and type(exports['objects'][family]) is dict and typed(exports['objects'][family]) == typed(input_bundle['source_exports'][family] if idf else document[family]), 'Complete exported object contract differs')
        order = exports['object_order'][family]
        require(type(order) is list and all(type(name) is str for name in order) and len(order) == len(set(order)) == len(document[family]) and order == (input_bundle['orders'][family] if idf else sorted(document[family])), 'Complete format-aware object order differs')
    require([name for name in exports['object_order']['NodeList'] if name in core_order] == core_order, 'Inherited NodeList relative order')
    require(all(name in document['NodeList'] and typed(document['NodeList'][name]) == typed(parent_document['NodeList'][name]) for name in core_order), 'Inherited core NodeList definition changed')
    require(len(document['SetpointManager:Scheduled']) == 9 and typed(document['SetpointManager:Scheduled']) == typed(parent_document['SetpointManager:Scheduled']), 'Existing nine SPM objects changed')
    derivation = data(spec['derivation'])
    exact_keys(derivation, {'schema', 'input', 'parent', 'primary', 'seed', 'report_cutoff', 'input_orders', 'events'} | extra, 'topology derivation')
    require(derivation['schema'] == ('paired.OATopologyDerivation.v2' if idf else 'paired.OATopologyDerivation.v1') and derivation['input'] == prediction['input'] and derivation['parent'] == parent, 'Derivation identity')
    if idf: require(derivation['input_contract'] == spec['input_contract'], 'Derivation original-IDF contract')
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
        require(derivation['input_orders'][label] == (input_bundle['orders'][family] if idf else sorted(document.get(family, {}))), 'Genuine complete format-aware input order')
    exact_keys(spec['prediction_reviews'], {'Root', 'independent'}, 'prediction reviews')
    require(spec['prediction_reviews']['Root'] != spec['prediction_reviews']['independent'], 'Two distinct prediction reviews required')
    for pin in spec['prediction_reviews'].values():
        review = data(pin)
        exact_keys(review, {'schema', 'accepted', 'accepted_static_only', 'findings', 'prediction', 'input', 'parent', 'source_exports', 'derivation', 'whole_body_predicted_before_actors', 'physical_outputs_consulted'}, 'prediction review')
        require(review['schema'] == ('paired.OATopologyPredictionReview.v2' if idf else 'paired.OATopologyPredictionReview.v1') and review['accepted'] is review['accepted_static_only'] is review['whole_body_predicted_before_actors'] is True and review['physical_outputs_consulted'] is False and review['findings'] == [], 'Unaccepted prospective prediction review')
        require(review['prediction'] == spec['prediction'] and review['input'] == prediction['input'] and review['parent'] == parent and review['source_exports'] == spec['source_exports'] and review['derivation'] == spec['derivation'], 'Prediction review binding differs')
    return dict(prediction=prediction, expected=expected, rows=rows, parent_case=parent_case, parent_release=parent_release, parent_authority=parent_authority, parent_rows=parent_rows, document=document, exports=exports, derivation=derivation, parent_selection=parent_selection, pins=list(pins.values()))


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
    if pred['schema'] == 'paired.OATopologyPrediction.v2':
        replay_input = original_idf_bundle(pred['input_contract'])
        require(pred['input'] == replay_input['contract']['input'], 'Topology replay physical input identity')
        document = replay_input['document']
    else:
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
