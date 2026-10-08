#!/usr/bin/env python3
"""Compare recorded CLK-03 carriers and source-order lifecycle observations.

Both engines have already run. This reader uses reference data only to compare
observations; it never launches an engine, changes inputs/tolerances or gates,
or reconstructs original scientific right-hand sides.
"""
from __future__ import annotations

import argparse
from datetime import datetime, timezone
import json
from pathlib import Path
import shutil
import sys

sys.dont_write_bytecode = True
from geo03_provenance import (
    ROOT, Bindings, exact, execution_interval, path_of, read, ref, require,
    same_binding, timestamp, verify_contract_bindings, verify_rust_build,
)
from clk03_unit_state import (
    Comparison, DAY_INTS, ENV_DISCRETE, ENV_LITERAL_REALS, FULL_OWNERS,
    HEADER_KEYS, STREAM, UNPAIRED, WEATHER_CONTROL_REALS, compare,
)

FROZEN = {
    'source':'da6eabfb07236e6303df15ede7e35a4254a2499369ade283a4ebaecfc3cd561d',
    'cases':'44d2f61ce68ed13c704e292dc9b25d3e8c6c8f871185027ac88cb3afaa9482c5',
    'tolerances':'fc2b509c35e415f50621cc8f144922ba5b7888ba697e74862298a1a0efb5b98e',
}
REQUEST_SHA = 'f12d3d22b5d7d20304cf2934c113555a0c1f7b4023d02cbf497b859d1933b2fb'
PROVENANCE_SHA = '5ca27645245ee9e7206a31c8906904394a5216625723cbfad12a5d5f49da9bf5'
PIN = '6f2e40d10250a105b49966baa24d843711e61048'
EXAMPLE = 'clk03_candidate_probe'


def selected_policy():
    """Literal protocol paths, frozen before the canonical Rust observation."""
    return {
        'pure_handoff_full_owners':FULL_OWNERS,
        'raw_header_owner_keys':HEADER_KEYS,
        'semantic_stream_keys':STREAM+['file_path'],
        'lifecycle_global':'all emitted typed caller/global clock fields',
        'lifecycle_environment_discrete':ENV_DISCRETE,
        'lifecycle_environment_literal_reals':ENV_LITERAL_REALS,
        'lifecycle_weather_control':'all emitted typed owner controls',
        'lifecycle_weather_control_reals':WEATHER_CONTROL_REALS,
        'lifecycle_daily_discrete':DAY_INTS,
        'lifecycle_grid_structure':['allocated','time_steps','hours','ordering'],
        'selected_environment_context':['CurrentCycle','SetWeekDays'],
        'shared_raw_counter':'header.wvarsMissedCounts.WeathCodes and missed_counts.WeathCodes',
        'within_lane_handoff':'full11 daily plus full17 every slot, environment projection and RptDayType/PreviousHour',
        'unpaired_scope':UNPAIRED,
        'unavailable_counted_as_PASS':False,
    }


def write(path, value):
    with path.open('xb') as stream:
        stream.write((json.dumps(value,indent=2,allow_nan=False)+'\n').encode('utf-8'))


def packet(bindings):
    refs = {key:{'path':f'energyplus_porting_plan/contracts/CLK-03-{key}.json','sha256':digest} for key,digest in FROZEN.items()}
    source,cases,tolerances = [read(bindings.check(refs[key])) for key in FROZEN]
    require(all(row['card'] == 'CLK-03' and row['energyplus_commit'] == PIN and row['packet_revision'] == 3
                and row['frozen_before_numerical_execution'] is True and row['status'] == 'frozen-before-numerical-execution'
                for row in [source,cases,tolerances]), 'Frozen revision3 input packet required')
    require(cases['helper_request']['sha256'] == REQUEST_SHA, 'Frozen input-only request differs')
    request = read(bindings.check(cases['helper_request']))
    require(request['schema'] == 'clk03-helper-cases.v1' and request['expected_values_supplied'] is False
            and request['expected_exits_supplied'] is False and request['scientific_execution_performed'] is False, 'Input-only request required')
    require(exact(tolerances['policy'], {
        'finite_literal_binary64':'exact IEEE754 bits including signed zero; atol=0, rtol=0',
        'discrete':'Exact type, value, order, cardinality, caller flags and available byte positions.',
        'unavailable':'Not counted as comparison PASS; source-undefined/unwritten locals never fabricated.',
        'native_processed_physics':'Retained diagnostically but unpaired unless literally supplied carrier copies.'}), 'Frozen exact/unpaired policy differs')
    scope = read(bindings.check(cases['fixed_CON_scope']))
    require(scope['schema_version'] == 'con01-scope.v2' and scope['energyplus']['commit'] == PIN and len(scope['cases']) == 15, 'Fixed CON scope required')
    for row in scope['cases']:
        for key in ['input','weather','metadata']: bindings.check(row[key])
    for row in cases['input_artifacts']: bindings.check(row)
    bindings.check(cases['input_byte_maps'])
    for row in source['source_files']:
        bindings.check({'path':'.reference/energyplus-src/26.1.0/'+row['path'],'sha256':row['sha256']})
    require(same_binding(request['fixed_scope'],cases['fixed_CON_scope']), 'Request CON scope differs')
    return refs,source,cases,request


def outer(bindings, receipt_path, expected_stdout, completed=None):
    binding = ref(receipt_path)
    record = read(bindings.check(binding))
    require(record['schema'] == 'recorded-porting-command.v1' and record['launch_error'] is None
            and record['source_bytes_match_before_and_after'] is True and record['recorder_updates_gates'] is False
            and record['recorder_supplies_reference_answers'] is False, 'Actual successful guarded command required')
    start,finish = execution_interval(record,bindings)
    require(exact(read(bindings.check(record['stdout'])),expected_stdout), 'Actual typed command stdout differs from report')
    bindings.check(record['source_snapshot'])
    bindings.check(record['executed_launcher']['archive'])
    if completed is not None: require(completed <= start, 'Recorded reader preceded required review')
    return binding,record,start,finish


def metadata(args,refs,cases,bindings):
    original_ref,review_ref,gap_ref = [ref(value) for value in [args.original_reference,args.original_data_review,args.baseline_review]]
    original,review,gap = [read(bindings.check(value)) for value in [original_ref,review_ref,gap_ref]]
    require(original['schema'] == 'clk03-original-helper-execution.v1' and original['actual_helper_exit_code'] == 0
            and original['actual_launcher_exit_code'] == 0 and original['preservation_checks_passed'] is True
            and original['Rust_compared'] is False and original['gates_updated'] is False
            and original['source_expected_answers_supplied'] is False, 'Actual preserved original zero required')
    verify_contract_bindings(original['contracts'],refs,bindings)
    require(same_binding(original['actual_request'],cases['helper_request']), 'Actual original request differs')
    original_stdout = {'reference':original_ref,'status':original['status'],
                       'actual_helper_exit_code':original['actual_helper_exit_code'],
                       'actual_launcher_exit_code':original['actual_launcher_exit_code']}
    original_outer,_,_,original_finish = outer(bindings,args.original_command,original_stdout)
    native_command = read(bindings.check(original['execution']))
    require(native_command['schema'] == 'original-native-command.v1' and native_command['spawn_error'] is None, 'Actual native child command required')
    _,native_finish = execution_interval(native_command,bindings)
    require(native_finish <= original_finish and len(native_command['command']) == 4
            and path_of(native_command['command'][0]) == bindings.check(native_command['binary'])
            and path_of(native_command['command'][1]) == ROOT
            and path_of(native_command['command'][2]) == bindings.check(cases['helper_request']), 'Actual native input-only argv differs')
    require(review['schema'] == 'clk03-independent-original-data-review.v1'
            and review['status'] == 'pass-preserved-original-data-before-Rust-baseline'
            and review['intended_native_branch_coverage_complete'] is True
            and same_binding(review['reviewed_reference'],original_ref)
            and same_binding(review['reviewed_results'],original['results'])
            and same_binding(review['reviewed_reference_command'],original_outer), 'Accepted original data and intended coverage review required')
    require(original_finish <= timestamp(review['review_completed_utc']), 'Original data review chronology differs')
    require(gap['schema'] == 'clk03-legacy-gap-observations.v1'
            and gap['status'] == 'recorded-preserved-disjoint-observations-and-unavailable-owners'
            and gap['intended_native_branch_coverage_complete'] is True and gap['numerical_PASS_claimed'] is False
            and gap['numerical_comparison_count'] is None and gap['numerical_mismatch_count'] is None
            and gap['engine_Cargo_Git_executed_by_reader'] is False
            and same_binding(gap['reviewed_receipts']['original_reference'],original_ref)
            and same_binding(gap['reviewed_receipts']['original_data_review'],review_ref), 'Actual honest legacy gap evidence required')
    gap_stdout = {'gap':gap_ref,'status':gap['status'],'numerical_PASS_claimed':False}
    gap_outer,_,_,gap_finish = outer(bindings,args.baseline_command,gap_stdout,timestamp(review['review_completed_utc']))
    for key in ['source','cases','tolerances']:
        require(same_binding(gap['frozen_input_packet'][key],refs[key]), 'Actual gap packet differs')
    require(same_binding(gap['frozen_input_packet']['helper_request'],cases['helper_request']), 'Actual gap input request differs')
    for key in ['legacy_receipt','legacy_command']:
        bindings.check(gap['reviewed_receipts'][key])
    for key in ['legacy_results','legacy_Rust_build','original_native_build']:
        bindings.check(gap[key])
    bindings.check(review['executed_reader']['archive'])
    bindings.check(gap['executed_reader']['archive'])
    native = read(bindings.check(original['results']))
    require(native['source_commit'] == PIN and same_binding(native['actual_binary'],native_command['binary'])
            and same_binding(native['actual_request'],cases['helper_request']), 'Observed native binary/request differs')
    verify_contract_bindings(native['contracts'],refs,bindings)
    build_ref = ref(args.rust_build)
    required = ['crates/ep_runtime/examples/'+EXAMPLE+'.rs','crates/ep_runtime/src/weather_day/lifecycle.rs',
                'crates/ep_runtime/src/weather_raw/cursor.rs','crates/ep_runtime/src/weather_raw/day_read.rs']
    build,available,cargo = verify_rust_build(build_ref,bindings,kind='example',example=EXAMPLE,committed=True,required_sources=required)
    execution_ref = ref(args.rust_execution)
    execution = read(bindings.check(execution_ref))
    require(execution['schema'] == 'recorded-porting-command.v1' and execution['launch_error'] is None
            and execution['source_bytes_match_before_and_after'] is True and execution['recorder_updates_gates'] is False
            and execution['recorder_supplies_reference_answers'] is False, 'Actual guarded canonical Rust execution required')
    start,finish = execution_interval(execution,bindings)
    require(gap_finish <= start and timestamp(cargo['finished_utc']) <= start, 'Canonical Rust observation preceded original/data/legacy evidence or its build')
    require(len(execution['command']) == 2 and path_of(execution['command'][0]) == bindings.check(build['binary'])
            and path_of(execution['command'][1]) == bindings.check(cases['helper_request']), 'Rust receives exclusively its archived binary and input-only request')
    require(same_binding(execution['source_snapshot'],build['source_snapshot'])
            and exact(execution['executed_launcher'],build['actual_executed_recorder'])
            and execution['repository_before']['head'] == execution['repository_after']['head'] == build['implementation_commit']
            and execution['repository_before']['worktree_clean'] is True and execution['repository_after']['worktree_clean'] is True,
            'Actual committed compiler/source/recorder identity differs')
    rust = read(bindings.check(execution['stdout']))
    verify_contract_bindings(rust['contracts'],refs,bindings)
    require(same_binding(rust['actual_request'],cases['helper_request']) and same_binding(rust['fixed_scope'],cases['fixed_CON_scope']), 'Actual Rust request/scope binding differs')
    return native,rust,execution,{
        'original_helper':original_ref,'original_command':original_outer,'original_results':original['results'],
        'independent_original_data_review':review_ref,'legacy_gap':gap_ref,'legacy_gap_command':gap_outer,
        'Rust_build':build_ref,'Rust_execution':execution_ref,'Rust_results':execution['stdout'],
        'implementation_commit':build['implementation_commit'],'crates_tree':build['crates_tree'],
        'committed_source_certification':build['committed_source_certification'],
        'available_source_inventory_count':len(available),'actual_Rust_finished_utc':finish.isoformat(),
    }


def projection(args,refs,cases,execution,dependencies,bindings):
    projection_ref = ref(args.dto_projection)
    declared = read(bindings.check(projection_ref))
    require(declared['schema'] == 'clk03-unit-dto-projection.v1' and declared['status'] == 'declared-before-canonical-probe-execution'
            and declared['scientific_comparer_executed'] is False and declared['prospective_numerical_PASS_claimed'] is False
            and declared['gates_updated'] is False, 'Declared pre-execution DTO projection required')
    verify_contract_bindings(declared['contracts'],refs,bindings)
    require(same_binding(declared['input_request'],cases['helper_request']) and exact(declared['selected_policy'],selected_policy()), 'Frozen exact selected observation policy differs')
    require(same_binding(declared['comparer'],ref(Path(__file__))), 'Comparer differs from frozen projection')
    for key in ['comparer_archive','state_reader_archive','metadata_dependency_archive']: bindings.check(declared[key])
    require(declared['comparer_archive']['sha256'] == declared['comparer']['sha256']
            and declared['state_reader_archive']['sha256'] == dependencies[1]['sha256']
            and declared['metadata_dependency_archive']['sha256'] == PROVENANCE_SHA, 'Frozen reader/dependency archive differs')
    require(timestamp(declared['created_utc']) <= timestamp(execution['started_utc']), 'DTO projection was frozen after Rust numerical observation')
    return projection_ref


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--original-reference','--native-reference',dest='original_reference',type=Path,required=True)
    parser.add_argument('--rust-execution','--rust-receipt',dest='rust_execution',type=Path,required=True)
    for key in ['original-command','original-data-review','baseline-review','baseline-command',
                'rust-build','dto-projection','output-dir']:
        parser.add_argument('--'+key,type=Path,required=True)
    args = parser.parse_args()
    output = args.output_dir.resolve()
    require(output.is_relative_to(ROOT/'.runtime') and not output.exists(), 'Fresh ignored output directory required')
    started = datetime.now(timezone.utc).isoformat()
    bindings = Bindings()
    dependencies = [{'path':'tools/porting/geo03_provenance.py','sha256':PROVENANCE_SHA},
                    ref(ROOT/'tools/porting/clk03_unit_state.py')]
    for row in dependencies: bindings.check(row)
    refs,source,cases,request = packet(bindings)
    native,rust,execution,identities = metadata(args,refs,cases,bindings)
    projected = projection(args,refs,cases,execution,dependencies,bindings)
    check = Comparison()
    coverage = compare(check,native,rust,request)
    require(exact(coverage['requested_counts'],cases['counts']), 'Actual frozen operation cardinalities differ')
    bindings.unchanged()
    output.mkdir()
    source_dir = output/'source'; source_dir.mkdir()
    archives = []
    for path in [Path(__file__).resolve(),*[bindings.check(row) for row in dependencies]]:
        archived = source_dir/path.name
        shutil.copyfile(path,archived)
        require(path.read_bytes() == archived.read_bytes(), 'Executed comparer archive differs')
        archives.append({'historical_path':path.relative_to(ROOT).as_posix(),'archive':ref(archived)})
    report = {
        'schema':'clk03-unit-comparison.v1',
        'status':'pass-exact-selected-carrier-and-lifecycle-state' if check.mismatch_count == 0 else 'fail-selected-carrier-or-lifecycle-state',
        'scientific_certification_passed':check.mismatch_count == 0,
        'started_utc':started,'completed_utc':datetime.now(timezone.utc).isoformat(),
        'actual_reader_command':list(sys.orig_argv),'actual_reader_cwd':str(Path.cwd()),
        'contracts':refs,'request':cases['helper_request'],**identities,
        'declared_pre_execution_DTO_projection':projected,'selected_policy':selected_policy(),
        'comparison':{'comparison_count':check.count,'mismatch_count':check.mismatch_count,'categories':dict(check.categories),
                      'mismatches':check.mismatches,'mismatch_payload_limit':1000,
                      'unavailable_operand_count':len(check.unavailable_paths),
                      'unavailable_operands_counted_as_PASS':False,
                      'mismatch_payloads_truncated':check.mismatch_count > len(check.mismatches)},
        'coverage':coverage,'executed_reader_source_archives':archives,
        'engines_executed_by_reader':False,'Cargo_executed_by_reader':False,'Git_executed_by_reader':False,
        'reference_outputs_supplied_to_Rust':False,'original_RHS_recomputed':False,'gates_updated':False,
        'unpaired_scope':UNPAIRED,'unavailable_counted_as_PASS':False,
        'limits':['Full supplied-carrier copies pair literal typed values, including signed zero; no generated processed physics parity is claimed.',
                  'Lifecycle selected dates, caller flags, source clock controls, actual counters and available byte positions compare exactly.',
                  'Full original constructor/preparation/registry arrays are not claimed by a selected Rust session.',
                  'Generated full-carrier transport is checked within each actual lane when its before/after boundary is observed.',
                  'The BeginEnvrn interior native handoff boundary remains unavailable; it is never counted as a passing comparison.',
                  'Source-error text/private-local parity and production invocation require separate evidence.'],
    }
    write(output/'unit-comparison.json',report)
    print(json.dumps(report,allow_nan=False))
    if check.mismatch_count: raise SystemExit(1)


if __name__ == '__main__':
    main()
