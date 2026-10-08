"""Freeze input-only production caller schedules; never read trace/output values.

Root executes this metadata writer after independent source/input review. All
controls come from the three frozen CON inputs and archived caller source. This
supplement preserves the R3 unit packet and the strict failed production proof.
"""
from __future__ import annotations
import argparse
from datetime import datetime,timezone
import hashlib
import json
from pathlib import Path
import shutil
import sys

ROOT=Path(__file__).resolve().parents[4]
PIN='6f2e40d10250a105b49966baa24d843711e61048'
CONTRACTS={
 'source':'da6eabfb07236e6303df15ede7e35a4254a2499369ade283a4ebaecfc3cd561d',
 'cases':'44d2f61ce68ed13c704e292dc9b25d3e8c6c8f871185027ac88cb3afaa9482c5',
 'tolerances':'fc2b509c35e415f50621cc8f144922ba5b7888ba697e74862298a1a0efb5b98e',
}
CASES=['A-24H','A-72H','B-BOTH-24H']
SOURCE_CLOCK={
 'crates/ep_runtime/src/weather_day/production.rs':[(80,90),(98,132),(208,235),(250,268)],
 'crates/ep_runtime/src/weather_day/lifecycle.rs':[(380,395)],
 '.reference/energyplus-src/26.1.0/src/EnergyPlus/SimulationManager.cc':[(475,554)],
 '.reference/energyplus-src/26.1.0/src/EnergyPlus/WeatherManager.cc':[(1737,1743),(1796,1830),(1954,1976),(1994,2034)],
}
OLD_REQUEST={'path':'energyplus_porting_plan/cases/CLK-03/helper-request.json',
 'sha256':'f12d3d22b5d7d20304cf2934c113555a0c1f7b4023d02cbf497b859d1933b2fb'}
OLD_POLICY={'path':'.runtime/porting/CLK-03/production-plan-01/plan.json',
 'sha256':'3148fbbdcb5215a437eb03f0778039adf73deaa75b67a9f22c84161e84585a7b'}
FAILED_EXECUTION={'path':'.runtime/porting/CLK-03/production-comparison-command-02/receipt.json',
 'sha256':'0fb86b4517921fb0bf9c9b236c26044d5c32ed13250c54823123d32d13ab5e5a'}
SEEN={}

def require(ok,message):
 if not ok:raise ValueError(message)

def path_of(value):
 path=Path(value);path=(path if path.is_absolute() else ROOT/path).resolve()
 require(path.is_relative_to(ROOT),'Contained repository path required');return path

def ref(value):
 path=path_of(value);sha=hashlib.sha256(path.read_bytes()).hexdigest()
 require(path not in SEEN or SEEN[path]==sha,'Previously bound input changed')
 SEEN[path]=sha;return {'path':path.relative_to(ROOT).as_posix(),'sha256':sha}

def bound(value):
 require(set(value)=={'path','sha256'} and ref(value['path'])==value,'Input binding differs');return dict(value)

def read(value):
 return json.loads(path_of(value).read_text(encoding='utf8'))

def phase(day,initial):
 # Literal set_phase assignments. Other flags retain their actual previous
 # caller/source values; assigning them here would change the admitted lane.
 return {'BeginSimFlag':initial,'BeginEnvrnFlag':initial,'BeginDayFlag':True,
  'WarmupFlag':False,'DayOfSim':day,'DayOfSimChr':str(day),'EndDayFlag':False,
  'EndEnvrnFlag':False,'HourOfDay':1,'TimeStep':1}

def operations(days):
 # No native result, Rust trace, selected weather value or expected branch
 # outcome is consulted. These are the independently declared caller controls.
 out=[{'id':'get-next-environment','kind':'GetNextEnvironment','caller':phase(1,True)}]
 for day in range(1,days+1):
  for hour in range(1,25):
   for step in range(1,5):
    if hour==1 and step==1:
     # The initial phase was assigned before GetNext. The first loop/seed 1/1
     # consumers use its token and issue no second InitializeWeather call.
     caller={} if day==1 else {'PreviousHour':24,**phase(day,False)}
    else:
     caller={'HourOfDay':hour,'TimeStep':step,'BeginSimFlag':False,
      'BeginEnvrnFlag':False,'BeginDayFlag':False,'BeginHourFlag':step==1,
      'BeginTimeStepFlag':True,'EndHourFlag':step==4,
      'EndDayFlag':hour==24 and step==4,
      'EndEnvrnFlag':day==days and hour==24 and step==4}
     if step==1:caller['PreviousHour']=hour-1
    out.append({'id':f'zone-D{day:03d}-H{hour:02d}-T{step:02d}',
     'kind':'InitializeWeather','caller':caller})
 return out

def main():
 parser=argparse.ArgumentParser(description=__doc__)
 parser.add_argument('--source-review',type=Path,required=True)
 parser.add_argument('--output-dir',type=Path,required=True)
 args=parser.parse_args();require(Path.cwd().resolve()==ROOT,'Repository cwd required')
 output=path_of(args.output_dir)
 require(output.is_relative_to(ROOT/'.runtime/porting/CLK-03') and not output.exists(),'Fresh ignored output required')
 started=datetime.now(timezone.utc).isoformat()
 contracts={name:ref('energyplus_porting_plan/contracts/CLK-03-'+name+'.json') for name in CONTRACTS}
 for name,sha in CONTRACTS.items():require(contracts[name]['sha256']==sha,'Frozen R3 contract differs')
 old=read(bound(OLD_REQUEST)['path']);policy=read(bound(OLD_POLICY)['path'])
 reviewed=read(args.source_review)
 require(reviewed['schema']=='clk03-independent-production-matched-input-source-review.v1'
  and reviewed['status']=='pass-input-only-matched-production-caller-source-before-freeze'
  and reviewed['scientific_execution_performed'] is False,'Independent input/source review required')
 require(reviewed['reviewed_contracts']==contracts,'Reviewed R3 contracts differ')
 sources={row['historical_path']:row['archive'] for row in reviewed['reviewed_sources']}
 require(len(sources)==len(reviewed['reviewed_sources']) and all(type(name) is str for name in sources),'Unique typed reviewed source paths required')
 require(str(Path(__file__).relative_to(ROOT).as_posix()) in sources,'Input writer was not reviewed')
 for name in [Path(__file__).relative_to(ROOT).as_posix(),*SOURCE_CLOCK]:
  require(name in sources and bound(sources[name])['sha256']==ref(name)['sha256'],'Reviewed caller source differs')
 require(datetime.fromisoformat(reviewed['review_completed_utc'])<=datetime.fromisoformat(started),'Source review must precede freeze')
 case_contract=read(contracts['cases']['path']);scope_ref=bound(case_contract['fixed_CON_scope']);scope=read(scope_ref['path'])
 require(case_contract['production_case_ids']==CASES and case_contract['production_trace_levels']==['Full','Summary'],'Frozen production scope differs')
 con={row['id']:row for row in scope['cases']};inputs={row['id']:row for row in old['weather_sequences']}
 cases=[]
 for name in CASES:
  original=inputs[name];literal=con[name]
  for key in ['input','weather']:require(bound(original[key])==bound(literal[key]),'CON input differs')
  bound(literal['metadata']);period=original['run_period']
  require(period['begin_year']==period['end_year']==2013 and period['treat_weather_as_actual'] is False
   and period['first_hour_interpolation_starting_values']=='Hour24','Fixed hourly nonactual2013 production required')
  days=3 if literal['duration']=='72H' else 1
  cases.append({**{key:original[key] for key in ['id','input','weather','run_period','time_steps_per_hour','prepared_environment_lane']},
   'native_preparation':old['native_preparation'],'operations':operations(days)})
 counts={'weather_sequences':3,'GetNextEnvironment':3,'InitializeWeather':480,'total_operations':483}
 require(sum(len(case['operations']) for case in cases)==483,'Declared matched caller count differs')
 request={'schema':'clk03-production-helper-cases.v1','source_commit':PIN,'contracts':contracts,'scope':scope_ref,
  'native_preparation':old['native_preparation'],'cases':cases,'requested_counts':counts,
  'expected_values_supplied':False,'expected_exits_supplied':False,'scientific_execution_performed':False}
 output.mkdir();archive_dir=output/'source';archive_dir.mkdir();archives=[]
 for index,name in enumerate([Path(__file__).relative_to(ROOT).as_posix(),*SOURCE_CLOCK]):
  target=archive_dir/f'{index:02d}-{Path(name).name}';shutil.copyfile(path_of(name),target)
  row={'historical_path':name,'archive':ref(target)}
  if name in SOURCE_CLOCK:
   lines=path_of(name).read_bytes().splitlines(keepends=True)
   row['caller_ranges']=[{'start_line':a,'end_line':b,'sha256':hashlib.sha256(b''.join(lines[a-1:b])).hexdigest()} for a,b in SOURCE_CLOCK[name]]
  archives.append(row)
 request_path=output/'helper-request.json';request_path.write_text(json.dumps(request,indent=2,allow_nan=False)+'\n',encoding='utf8',newline='\n')
 # Preserved failed evidence is bound as bytes only. It is never deserialized
 # into a caller assignment, an expected value or a subsequent runtime input.
 bound(FAILED_EXECUTION)
 failed_report=ref('.runtime/porting/CLK-03/production-comparison-02/production-comparison.json')
 projection={'schema':'clk03-matched-production-input-projection.v1',
  'status':'declared-input-only-caller-history-before-supplemental-original-execution',
  'created_utc':datetime.now(timezone.utc).isoformat(),'actual_metadata_writer_command':list(sys.orig_argv),
  'contracts':contracts,'old_unit_request':OLD_REQUEST,'matched_request':ref(request_path),'fixed_scope':scope_ref,
  'input_byte_maps':bound(case_contract['input_byte_maps']),'input_artifacts':[bound(row) for row in case_contract['input_artifacts']],
  'requested_counts':counts,'independent_input_source_review':ref(args.source_review),'caller_source_archives':archives,
  'retained_prior_policy':OLD_POLICY,'prior_selected_field_arrays':policy['selected_policy']['lifecycle_native_selection'],
  'preserved_failed_comparison':failed_report,'preserved_failed_execution':FAILED_EXECUTION,
  'prior_actual_mismatch_relabelled_unavailable_or_PASS':False,'old_contract_plan_report_or_policy_rewritten':False,
  'clock_model':'literal selected production set_phase/current_for assignments; PreviousHour only on actual external hour/day transition; whole native InitializeWeather owns retained writes',
  'initial_token':'GetNext after initial phase, one initial Initialize; duplicate seed/first-loop consumer identity invokes no additional Initialize',
  'terminal':'final zone timestep EndEnvrn before consumer; no duplicate finish initialization or day prefetch',
  'warmup_inserted':False,'native_thermal_callback_parity_claimed':False,
  'Rust_trace_or_native_result_deserialized_as_input':False,'scientific_execution_performed':False,
  'expected_values_supplied':False,'prospective_PASS_claimed':False,'gates_updated':False}
 (output/'projection.json').write_text(json.dumps(projection,indent=2,allow_nan=False)+'\n',encoding='utf8',newline='\n')
 literal_scope_refs=[bound(row[key]) for row in scope['cases'] for key in ['input','weather','metadata']]
 require(len(literal_scope_refs)==45,'Whole frozen CON literal input inventory required')
 source_contract=read(contracts['source']['path'])
 for row in source_contract['source_files']:
  require(ref('.reference/energyplus-src/26.1.0/'+row['path'])['sha256']==row['sha256'],'Pinned original source bytes differ')
 native_contract={'schema':'clk03-matched-production-native-contract.v1',
  'status':'frozen-input-only-matched-caller-before-supplemental-original-execution',
  'created_utc':projection['created_utc'],'source_commit':PIN,'contracts':contracts,
  'request':ref(request_path),'input_projection':ref(output/'projection.json'),'fixed_scope':scope_ref,
  'input_byte_maps':projection['input_byte_maps'],'literal_CON_input_references':literal_scope_refs,
  'source_files':source_contract['source_files'],'production_case_ids':CASES,'requested_counts':counts,
  'native_preparation':old['native_preparation'],'independent_input_source_review':ref(args.source_review),
  'caller_source_archives':archives,'expected_values_supplied':False,'expected_exits_supplied':False,
  'scientific_execution_performed':False,'gates_updated':False,'old_R3_contracts_rewritten':False,
  'source_RHS_copied_or_native_results_as_inputs':False,'production_matrix_expanded':False}
 (output/'native-contract.json').write_text(json.dumps(native_contract,indent=2,allow_nan=False)+'\n',encoding='utf8',newline='\n')
 for path,sha in SEEN.items():
  require(hashlib.sha256(path.read_bytes()).hexdigest()==sha,'Input/source/proof changed during metadata freeze: '+str(path))
 print(json.dumps({'request':ref(request_path),'projection':ref(output/'projection.json'),
  'contract':ref(output/'native-contract.json'),'requested_counts':counts,
  'status':projection['status'],'scientific_execution_performed':False},allow_nan=False))

if __name__=='__main__':main()
