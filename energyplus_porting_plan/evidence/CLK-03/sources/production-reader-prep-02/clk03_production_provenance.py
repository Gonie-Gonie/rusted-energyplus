"""CLK-03 production metadata only; no engine, compiler or Git calls."""
from __future__ import annotations

from datetime import datetime, timezone
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[4]
sys.dont_write_bytecode = True
sys.path.insert(0,str(ROOT/'tools/porting'))
from geo03_provenance import (Bindings, command_paths, exact, execution_interval, path_of,
    read, ref, require, same_binding, timestamp, verify_contract_bindings, verify_rust_build)
from check_clk03_unit import packet

RAW_REFERENCE = {'path':'.runtime/porting/CLK-02/original-helper-first-01/helper-reference.json',
                 'sha256':'afa446961b4df0651c916a84a24a816814da8becb48b952e511049e7cc4bc3d0'}
RAW_REVIEW = {'path':'.runtime/porting/CLK-02/independent-original-data-review-01/review.json',
              'sha256':'a1acff78956de2f20ac101caef5e90e7a4970b2d4f62ec925fef158d39492b28'}
REQUIRED = ['crates/ep_run/src/pipeline.rs','crates/ep_run/src/clk03_trace.rs',
    'crates/ep_runtime/src/weather_day/production.rs','crates/ep_runtime/src/weather_day/lifecycle.rs',
    'crates/ep_runtime/src/weather_day/production_trace.rs','crates/ep_runtime/src/weather_raw/cursor.rs',
    'crates/ep_runtime/src/weather_raw/day_read.rs','crates/ep_runtime/src/heat_balance/weather_driver.rs',
    'crates/ep_runtime/src/runtime.rs','crates/ep_runtime/src/heat_balance/run_period.rs',
    'crates/ep_runtime/src/heat_balance/warmup.rs','crates/ep_runtime/src/ideal_loads/coupled_runtime.rs']
FULL_ONLY = {'clk03-weather-day-production-trace.json','clk02-weather-production-trace.json',
    'clock-calls.json','psy02-calls.json','psychrometrics-calls.json','zon01-initialization.json',
    'compiled-geometry.json','geometry-consumers.json','geo02-geometry.json','geo02-geometry-operands.json','geo03-zone-volume.json'}


def recorded_json(bindings,value,expected,earliest=None):
    actual=read(bindings.check(value))
    require(actual['schema']=='recorded-porting-command.v1' and actual['launch_error'] is None
            and actual['source_bytes_match_before_and_after'] is True
            and actual['recorder_updates_gates'] is False and actual['recorder_supplies_reference_answers'] is False,
            'Actual guarded recorded command required')
    start,finish=execution_interval(actual,bindings)
    require(exact(read(bindings.check(actual['stdout'])),expected),'Actual recorded typed stdout differs')
    bindings.check(actual['source_snapshot']);bindings.check(actual['executed_launcher']['archive'])
    if earliest is not None:require(earliest<=start,'Required evidence/review preceded recorded command')
    return actual,start,finish


def source_architecture(available,bindings):
    texts={key:bindings.check(available[key]).read_text(encoding='utf8') for key in REQUIRED}
    pipeline=texts['crates/ep_run/src/pipeline.rs']
    require('ProductionWeatherTimestepSeries::from_bytes(' in pipeline
            and 'Preview evidence remains separate from the live owner below.' in pipeline,
            'Archived ordinary pipeline must construct a separate live byte owner')
    production=texts['crates/ep_runtime/src/weather_day/production.rs']
    require('prepared_phase: RefCell<Option<WeatherDayPhase>>' in production
            and '*self.prepared_phase.borrow_mut() = None;' in production
            and 'session.state.today_values.hour(hour_zero + 1)?' in production
            and 'today_produced' in production and 'record_consumer(' in production,
            'Archived live Today/current-day/first-token owner required')
    current=production[production.index('pub fn current_for('):production.index('pub fn initial_hourly_dry_bulb_c(')]
    require(current.index('session.initialize_weather()?;')<current.index('super::production_trace::record_consumer('),
            'Archived terminal initialization must precede actual consumer receipt')
    driver=texts['crates/ep_runtime/src/heat_balance/weather_driver.rs']
    require('records: &[]' in driver and 'sample: None' in driver
            and 'owned: Some(series.current_for(record_index, zone_timestep)?)' in driver,
            'Archived production consumer must receive owned live operands')
    require('driver.begin_day(WeatherDayPhase::Run' in texts['crates/ep_runtime/src/heat_balance/run_period.rs']
            and 'driver.begin_day(WeatherDayPhase::Warmup' in texts['crates/ep_runtime/src/heat_balance/warmup.rs']
            and 'prepare_initial_phase(' in texts['crates/ep_runtime/src/runtime.rs']
            and 'prepare_initial_phase(' in texts['crates/ep_runtime/src/ideal_loads/coupled_runtime.rs'],
            'Archived A/B initial hooks and shared run/warmup hooks required')
    return {'archived_owned_payload_path_present':True,'preview_exists_but_is_not_live_payload_owner':True,
            'role_specific_caller_attribution_available':False,'role_unavailable_counted_as_PASS':False,
            'source_inventory_scope':'available archived bytes, not compiler file-selection proof'}


def plan(value,refs,cases,policy,bindings):
    binding=ref(value);declared=read(bindings.check(binding))
    require(declared['schema']=='clk03-production-plan.v1' and declared['actual_command_count']==6
            and declared['expected_outputs_supplied'] is False and declared['physical_execution_performed'] is False
            and declared['gates_updated'] is False and exact(declared['selected_policy'],policy),
            'Pre-execution six-input production observation policy required')
    verify_contract_bindings(declared['contracts'],refs,bindings)
    require(same_binding(declared['fixed_scope'],cases['fixed_CON_scope'])
            and same_binding(declared['input_byte_maps'],cases['input_byte_maps']), 'Frozen input scope/maps differ')
    scope=read(bindings.check(cases['fixed_CON_scope']))
    originals={row['id']:row for row in scope['cases']}
    require([row['case_id'] for row in declared['cases']]==cases['production_case_ids'],'Three frozen case order required')
    for row in declared['cases']:
        require(exact(row['trace_levels'],['Full','Summary']),'Frozen Full/Summary matrix required')
        original=originals[row['case_id']]
        require(row['scope']==original['scope'] and row['duration']==original['duration'],'Fixed CON identity differs')
        for key in ['input','weather','metadata']:
            require(same_binding(row[key],original[key]),'Fixed CON file binding differs');bindings.check(row[key])
    return binding,declared


def evidence(matrix,refs,cases,bindings):
    unit=read(bindings.check(matrix['unit_comparison']))
    require(unit['schema']=='clk03-unit-comparison.v1' and unit['status']=='pass-exact-selected-carrier-and-lifecycle-state'
            and unit['scientific_certification_passed'] is True and unit['comparison']['mismatch_count']==0
            and unit['comparison']['comparison_count']>0 and unit['gates_updated'] is False
            and unit['unavailable_counted_as_PASS'] is False,'Actual selected unit pass required first')
    verify_contract_bindings(unit['contracts'],refs,bindings)
    require(same_binding(unit['request'],cases['helper_request']),'Unit frozen request differs')
    execution,_,finished=recorded_json(bindings,matrix['unit_comparison_execution'],unit)
    require(exact(execution['command'],unit['actual_reader_command']),'Actual unit reader command differs')
    unit_build,_,_=verify_rust_build(unit['Rust_build'],bindings,kind='example',example='clk03_candidate_probe',committed=True,
        required_sources=['crates/ep_runtime/examples/clk03_candidate_probe.rs'])
    require(same_binding(execution['source_snapshot'],unit_build['source_snapshot'])
            and execution['repository_before']['head']==execution['repository_after']['head']==unit_build['implementation_commit']
            and execution['executed_launcher']['historical_path']=='tools/porting/record_command.py'
            and execution['executed_launcher']['archive']['sha256']==unit_build['actual_executed_recorder']['archive']['sha256'],
            'Actually executed unit comparer recorder/build/source identity differs')
    probe=read(bindings.check(unit['Rust_execution']));_,probe_finished=execution_interval(probe,bindings)
    require(probe['schema']=='recorded-porting-command.v1' and probe['launch_error'] is None
            and probe['recorder_updates_gates'] is False and probe['recorder_supplies_reference_answers'] is False
            and probe['source_bytes_match_before_and_after'] is True and same_binding(probe['stdout'],unit['Rust_results'])
            and same_binding(probe['source_snapshot'],unit_build['source_snapshot'])
            and probe['repository_before']['head']==probe['repository_after']['head']==unit_build['implementation_commit']
            and probe['executed_launcher']['historical_path']=='tools/porting/record_command.py'
            and probe['executed_launcher']['archive']['sha256']==unit_build['actual_executed_recorder']['archive']['sha256'],
            'Actual canonical probe build/results identity differs')
    command_paths(probe['command'],[unit_build['binary']['path'],cases['helper_request']['path']])
    require(probe_finished<=timestamp(unit['started_utc'])<=timestamp(unit['completed_utc'])<=finished,'Actual unit proof chronology differs')
    projected=read(bindings.check(unit['declared_pre_execution_DTO_projection']))
    require(projected['schema']=='clk03-unit-dto-projection.v1' and projected['status']=='declared-before-canonical-probe-execution'
            and projected['scientific_comparer_executed'] is False and projected['prospective_numerical_PASS_claimed'] is False
            and projected['gates_updated'] is False and exact(projected['selected_policy'],unit['selected_policy'])
            and timestamp(projected['created_utc'])<=timestamp(probe['started_utc']), 'Actual unit pre-execution projection differs')
    verify_contract_bindings(projected['contracts'],refs,bindings)
    require(same_binding(projected['input_request'],cases['helper_request']),'Unit projected request differs')
    archives={row['historical_path']:row['archive'] for row in unit['executed_reader_source_archives']}
    require(len(archives)==len(unit['executed_reader_source_archives'])==3,'Unit executed archive inventory differs')
    for name,key in [('check_clk03_unit.py','comparer_archive'),('clk03_unit_state.py','state_reader_archive'),('geo03_provenance.py','metadata_dependency_archive')]:
        original=projected[key];bindings.check(original);archive=archives['tools/porting/'+name];bindings.check(archive)
        require(archive['sha256']==original['sha256'],'Unit executed source differs from pre-execution archive')
    require(projected['comparer']['sha256']==projected['comparer_archive']['sha256']
            and len(execution['command'])>=6 and execution['command'][1:4]==['-X','utf8','-B']
            and path_of(execution['command'][4])==path_of(projected['comparer']['path']), 'Actual unit comparer owner differs')
    flags=execution['command'][5:];require(len(flags)%2==0 and len(set(flags[::2]))==len(flags[::2]),'Actual unit comparer flags malformed')
    values=dict(zip(flags[::2],flags[1::2],strict=True))
    for flag,binding in [('--original-reference',unit['original_helper']),('--original-command',unit['original_command']),
            ('--original-data-review',unit['independent_original_data_review']),('--baseline-review',unit['legacy_gap']),
            ('--baseline-command',unit['legacy_gap_command']),('--rust-build',unit['Rust_build']),('--rust-execution',unit['Rust_execution']),
            ('--dto-projection',unit['declared_pre_execution_DTO_projection'])]:
        require(flag in values and path_of(values[flag])==bindings.check(binding),'Actual unit comparer evidence argument differs: '+flag)
    require('--output-dir' in values and path_of(values['--output-dir'])==bindings.check(matrix['unit_comparison']).parent,'Unit output argument differs')
    original=read(bindings.check(unit['original_helper']));native=read(bindings.check(unit['original_results']))
    require(original['actual_helper_exit_code']==original['actual_launcher_exit_code']==0
            and original['preservation_checks_passed'] is True and same_binding(original['results'],unit['original_results'])
            and same_binding(native['actual_request'],cases['helper_request']),'Actual preserved source helper data required')
    peer=read(bindings.check(unit['independent_original_data_review']))
    require(peer['intended_native_branch_coverage_complete'] is True and same_binding(peer['reviewed_reference'],unit['original_helper']),
            'Initial independent actual intended coverage review required')
    raw_reference=read(bindings.check(RAW_REFERENCE));raw_peer=read(bindings.check(RAW_REVIEW))
    require(raw_reference['actual_helper_exit_code']==0 and raw_reference['preservation_checks_passed'] is True
            and raw_peer['status']=='pass-preserved-original-data-before-Rust-baseline'
            and same_binding(raw_peer['reviewed_reference'],RAW_REFERENCE)
            and same_binding(raw_peer['reviewed_results'],raw_reference['results']),'Preserved independently reviewed CLK02 raw original required')
    raw_native=read(bindings.check(raw_reference['results']))
    require(raw_native['schema']=='clk02-helper-results.v1' and raw_native['complete'] is True
            and raw_native['expected_answers_supplied'] is False and raw_native['fixed_epw']['record_count']==8760
            and raw_native['fixed_epw']['all_records_returned_normally'] is True,'Actual original full raw inventory required')
    build,available,compilation=verify_rust_build(matrix['build'],bindings,kind='cli',committed=True,required_sources=REQUIRED)
    require(build['crates_tree']==unit_build['crates_tree']==unit['crates_tree']
            and build['implementation_commit']==unit_build['implementation_commit']==unit['implementation_commit']
            and same_binding(build['source_snapshot'],unit_build['source_snapshot']), 'Unit and actual CLI committed Rust tree differs')
    return unit,native,raw_native,build,available,max(finished,timestamp(compilation['finished_utc']))


def recorded_run(case,item,level,build,earliest,planned,bindings,seen):
    output=path_of(item['output_directory']);require(output.is_dir() and output not in seen,'Fresh distinct CLI output directory required');seen.add(output)
    execution=read(bindings.check(item['command_receipt']))
    require(execution['schema']=='recorded-porting-command.v1' and execution['launch_error'] is None
            and execution['source_bytes_match_before_and_after'] is True
            and execution['recorder_updates_gates'] is False and execution['recorder_supplies_reference_answers'] is False,'Actual guarded input-only CLI required')
    start,_=execution_interval(execution,bindings);require(earliest<=start and planned<=start,'CLI preceded unit/build/plan/source review')
    require(execution['repository_before']['head']==execution['repository_after']['head']==build['implementation_commit']
            and execution['repository_before']['worktree_clean'] is True and execution['repository_after']['worktree_clean'] is True
            and same_binding(execution['source_snapshot'],build['source_snapshot']),'Actual CLI source/head differs')
    expected=[build['binary']['path'],'run',case['input']['path'],'--weather',case['weather']['path'],'--output-dir',str(output),
              '--mode','compatibility','--partial','deny','--trace-level',level.lower(),'--porting-scope',case['scope'].lower()]
    command_paths(execution['command'],expected,{0,2,4,6})
    require(execution['executed_launcher']['historical_path']=='tools/porting/record_command.py'
            and execution['executed_launcher']['archive']['sha256']==build['actual_executed_recorder']['archive']['sha256'],'Actual CLI recorder differs from build')
    bindings.check(execution['executed_launcher']['archive'])
    artifacts={}
    for row in item['artifacts']:
        path=bindings.check(row);require(path.is_relative_to(output) and path not in artifacts,'Foreign/duplicate CLI artifact');artifacts[path]=row
    require(set(artifacts)=={path.resolve() for path in output.rglob('*') if path.is_file()},'Actual retained artifact inventory differs')
    summary=read(bindings.check(artifacts[output/'run-summary.json']));config=summary['config']
    require(summary['status']=='success' and summary['exit_code']==execution['exit_code']==0
            and summary['source_order_gate']['matches'] is True and summary['selected_algorithm_lane']['diagnostic_probe_used'] is False,
            'Actual successful source-order physical runtime required')
    require(config['mode']=='compatibility' and config['partial_policy']=='deny' and config['trace_level']==level.lower()
            and config['dry_run'] is False and config['hours'] is None and config['oracle_baseline'] is False
            and config['compare_oracle'] is False and summary['oracle'] is None and summary['comparison'] is None,
            'Ordinary input-only CLI boundary differs')
    require(summary['rust_runtime']['samples']==(72 if case['duration']=='72H' else 24)
            and summary['support']['conformance_claim'] is False,'Actual duration/scope differs')
    if case['scope']=='B':
        require(summary['rust_runtime']['fixture_demand_injection_used'] is False
                and type(summary['rust_runtime']['purchased_air_coupling_call_count']) is int
                and summary['rust_runtime']['purchased_air_coupling_call_count']>0,'Actual B physical demand coupling missing')
    else:
        require(summary['rust_runtime']['runtime_class']=='one-zone-heat-balance-compatibility'
                and summary['rust_runtime']['fixture_demand_injection_used'] is None
                and summary['rust_runtime']['purchased_air_coupling_call_count'] is None,'A unavailable coupling registrations must stay unavailable')
    scope=read(bindings.check(artifacts[output/'porting_scope.json']))
    require(scope['schema']=='porting-scope.v1' and scope['scope']==case['scope'] and scope['admissible'] is True
            and scope['violations']==[],'Actual frozen scope must be admitted')
    return output,summary,artifacts
