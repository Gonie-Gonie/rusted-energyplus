"""Authenticate supplemental input/native metadata; no engines or physics."""
from __future__ import annotations
from pathlib import Path
from clk03_production_provenance import (ROOT, Bindings, exact, execution_interval, path_of,
    read, ref, require, same_binding, timestamp, verify_contract_bindings, command_paths)

COUNTS={'weather_sequences':3,'GetNextEnvironment':3,'InitializeWeather':480,'total_operations':483}

def matched_contract(binding,refs,cases,bindings):
    value=read(bindings.check(binding))
    require(value['schema']=='clk03-matched-production-native-contract.v1'
        and value['status']=='frozen-input-only-matched-caller-before-supplemental-original-execution'
        and value['source_commit']=='6f2e40d10250a105b49966baa24d843711e61048'
        and value['requested_counts']==COUNTS,'Frozen matched input-only native contract required')
    verify_contract_bindings(value['contracts'],refs,bindings)
    for key in ['expected_values_supplied','expected_exits_supplied','scientific_execution_performed','gates_updated',
        'old_R3_contracts_rewritten','source_RHS_copied_or_native_results_as_inputs','production_matrix_expanded']:
        require(value[key] is False,'Matched input boundary differs: '+key)
    require(value['production_case_ids']==cases['production_case_ids']
        and same_binding(value['fixed_scope'],cases['fixed_CON_scope'])
        and same_binding(value['input_byte_maps'],cases['input_byte_maps']),'Matched CON input scope differs')
    request=read(bindings.check(value['request']));projection=read(bindings.check(value['input_projection']))
    require(request['schema']=='clk03-production-helper-cases.v1' and request['requested_counts']==COUNTS
        and request['expected_values_supplied'] is False and request['expected_exits_supplied'] is False
        and request['scientific_execution_performed'] is False,'Input-only matched request required')
    verify_contract_bindings(request['contracts'],refs,bindings)
    require(same_binding(request['scope'],cases['fixed_CON_scope']) and request['source_commit']==value['source_commit']
        and exact(request['native_preparation'],value['native_preparation']),'Request contract/preparation differs')
    require(projection['schema']=='clk03-matched-production-input-projection.v1'
        and projection['status']=='declared-input-only-caller-history-before-supplemental-original-execution'
        and same_binding(projection['matched_request'],value['request']) and projection['requested_counts']==COUNTS
        and projection['Rust_trace_or_native_result_deserialized_as_input'] is False
        and projection['prior_actual_mismatch_relabelled_unavailable_or_PASS'] is False
        and projection['old_contract_plan_report_or_policy_rewritten'] is False
        and projection['prospective_PASS_claimed'] is False and projection['scientific_execution_performed'] is False,
        'Matched input projection or failure honesty differs')
    for key in ['preserved_failed_comparison','preserved_failed_execution','retained_prior_policy']:
        bindings.check(projection[key])
    scope=read(bindings.check(value['fixed_scope']));literal=[row[key] for row in scope['cases'] for key in ['input','weather','metadata']]
    require(exact(value['literal_CON_input_references'],literal) and len(literal)==45,'All45 literal CON input references required')
    for row in literal:bindings.check(row)
    require(len(request['cases'])==3 and [row['id'] for row in request['cases']]==cases['production_case_ids'],'Three ordered matched CON roots required')
    count=0
    indexed={row['id']:row for row in scope['cases']}
    for row in request['cases']:
        old=indexed[row['id']]
        require(row['time_steps_per_hour']==4 and row['prepared_environment_lane'] is True
            and exact(row['native_preparation'],request['native_preparation']),'Matched prepared lane differs')
        for key in ['input','weather']:
            require(same_binding(row[key],old[key]),'Matched input differs');bindings.check(row[key])
        ops=row['operations'];expected=289 if old['duration']=='72H' else 97;count+=len(ops)
        require(len(ops)==expected and ops[0]['kind']=='GetNextEnvironment'
            and all(op['kind']=='InitializeWeather' for op in ops[1:])
            and len({op['id'] for op in ops})==len(ops),'Complete unique source-call schedule required')
    require(count==483,'Matched call inventory differs')
    for row in value['caller_source_archives']:bindings.check(row['archive'])
    return value,request,projection

def actual_matched_native(matrix,refs,cases,declared,bindings):
    contract,request,projection=matched_contract(matrix['matched_native_contract'],refs,cases,bindings)
    require(same_binding(matrix['matched_native_contract'],declared['matched_native_contract']),'Matrix plan matched contract differs')
    reference=read(bindings.check(matrix['matched_native_reference']))
    require(reference['schema']=='clk03-matched-production-original-reference.v1'
        and reference['actual_helper_exit_code']==reference['actual_launcher_exit_code']==0
        and reference['preservation_checks_passed'] is True and reference['source_expected_answers_supplied'] is False,
        'Actual successful preserved genuine matched original required')
    require(same_binding(reference['actual_request'],contract['request'])
        and same_binding(reference['actual_contract'],matrix['matched_native_contract']),'Actual original input identity differs')
    require(same_binding(reference['production_plan'],matrix['declared_plan'])
        and same_binding(reference['production_plan_source_review'],matrix['independent_sources_review']),
        'Actual original must use the exact prior frozen matched observation plan')
    original=read(bindings.check(matrix['matched_original_execution']))
    require(original['schema']=='recorded-porting-command.v1' and original['launch_error'] is None
        and original['source_bytes_match_before_and_after'] is True
        and original['recorder_updates_gates'] is False and original['recorder_supplies_reference_answers'] is False,'Actual guarded original recorder required')
    begin,end=execution_interval(original,bindings)
    expected={'reference':matrix['matched_native_reference'],'status':reference['status'],
        'actual_helper_exit_code':0,'actual_launcher_exit_code':0}
    require(exact(read(bindings.check(original['stdout'])),expected),'Actual original compact stdout binding differs')
    require(timestamp(declared['created_utc'])<=begin and timestamp(projection['created_utc'])<=begin,
        'Fresh observation/input policy must be frozen before matched original scientific invocation')
    child=read(bindings.check(reference['execution']))
    require(child['schema']=='original-native-command.v1' and type(child['exit_code']) is int and child['exit_code']==0
        and child['spawn_error'] is None and path_of(child['cwd'])==ROOT,'Actual successfully launched native child required')
    child_start,child_finish=timestamp(child['started_utc']),timestamp(child['finished_utc'])
    require(begin<=child_start<=child_finish<=end,'Native child timestamps must be inside actual outer command')
    for key,binding in [('binary',reference['executed_helper']),('request',contract['request']),
        ('contract',matrix['matched_native_contract']),('production_plan',matrix['declared_plan']),
        ('native_driver_build',reference['native_driver_build'])]:
        require(same_binding(child[key],binding),'Actual child binding differs: '+key);bindings.check(binding)
    result_path=bindings.check(reference['results'])
    command_paths(child['command'],[reference['executed_helper']['path'],str(ROOT),contract['request']['path'],
        matrix['matched_native_contract']['path'],str(result_path.parent)])
    bindings.check(child['stdout']);bindings.check(child['stderr'])
    build=read(bindings.check(reference['native_driver_build']))
    require(build['schema']=='clk03-matched-production-native-build.v1' and build['checks_passed'] is True
        and build['scientific_source_patches'] is False and build['scientific_execution_performed'] is False
        and build['assertions_enabled'] is True and build['fp_contract']=='off'
        and same_binding(build['helper_binary'],reference['executed_helper'])
        and same_binding(build['matched_contract'],matrix['matched_native_contract'])
        and timestamp(build['finished_utc'])<=child_start,'Actual genuine built-and-derived helper must precede execution')
    build_review=read(bindings.check(reference['independent_actual_build_review']))
    require(same_binding(build_review['reviewed_build'],reference['native_driver_build'])
        and timestamp(build['finished_utc'])<=timestamp(build_review['review_completed_utc'])<=child_start,
        'Actual independent native build review must follow build and precede original execution')
    native=read(bindings.check(reference['results']))
    require(native['schema']=='clk03-production-helper-results.v1' and native['complete'] is True
        and native['source_commit']==contract['source_commit'] and native['requested_counts']==COUNTS
        and native['actual_operation_invocations']==483 and native['actual_operations_skipped']==0
        and native['actual_operation_counts']=={'GetNextEnvironment':3,'InitializeWeather':480}
        and native['expected_answers_supplied'] is False and native['gates_updated'] is False
        and native['native_raw_record_index_claimed'] is False and native['processed_weather_physics_retained_unpaired'] is True
        and native['pure_body_fallback_used'] is False,'Actual whole matched native observations required')
    verify_contract_bindings(native['contracts'],refs,bindings)
    require(same_binding(native['actual_request'],contract['request'])
        and same_binding(native['actual_contract'],matrix['matched_native_contract']),'Native runtime packet differs')
    require(same_binding(native['actual_binary'],reference['executed_helper']),'Actually executed native binary differs from reference')
    bindings.check(reference['executed_helper'])
    require([row['id'] for row in native['cases']]==cases['production_case_ids'],'Actual ordered three native roots required')
    for actual,literal in zip(native['cases'],request['cases'],strict=True):
        require(same_binding(actual['input'],literal['input']) and same_binding(actual['weather_input'],literal['weather'])
            and exact(actual['declared_run_period_input'],literal['run_period'])
            and len(actual['operations'])==len(literal['operations']),'Actual native case input/call inventory differs')
        require(len(actual['preparation_calls'])==8 and all(row['call_outcome']['status']=='source_returned'
            and row['call_outcome']['source_fatal'] is False for row in actual['preparation_calls']),
            'Every genuine preparation call must actually return')
        for actual_op,literal_op in zip(actual['operations'],literal['operations'],strict=True):
            require(actual_op['id']==literal_op['id'] and actual_op['kind']==literal_op['kind']
                and exact(actual_op['requested_operation'],literal_op) and actual_op['actual_source_invoked'] is True
                and actual_op['call_outcome']['status']=='source_returned' and actual_op['call_outcome']['source_fatal'] is False
                and actual_op['Available'] is True and actual_op['ErrorsFound'] is False,
                'Each intended matched native operation must actually invoke and return in order before CLI')
    review=read(bindings.check(matrix['matched_original_data_review']))
    require(review['schema']=='clk03-independent-matched-production-original-data-review.v1'
        and review['status']=='pass-original-matched-history-before-production'
        and review['scientific_values_compared'] is False,'Independent original data review required before CLI')
    for key,binding in [('reviewed_reference',matrix['matched_native_reference']),('reviewed_results',reference['results']),
            ('reviewed_request',contract['request']),('reviewed_contract',matrix['matched_native_contract']),
            ('reviewed_execution',reference['execution']),
            ('reviewed_build',reference['native_driver_build']),('reviewed_build_review',reference['independent_actual_build_review'])]:
        require(same_binding(review[key],binding),'Independent matched original evidence binding differs');bindings.check(binding)
    require(review['actual_operation_invocations']==483 and review['actual_operations_skipped']==0,'Initial actual matched call coverage incomplete')
    review_command=read(bindings.check(matrix['matched_original_data_review_execution']))
    require(review_command['schema']=='recorded-porting-command.v1' and review_command['launch_error'] is None
        and review_command['source_bytes_match_before_and_after'] is True,'Actual guarded initial review recorder required')
    start,finish=execution_interval(review_command,bindings)
    require(end<=start<=timestamp(review['review_started_utc'])<=timestamp(review['review_completed_utc'])<=finish
        and exact(read(bindings.check(review_command['stdout'])),review),'Actual independent data review chronology/stdout differs')
    return native,request,max(end,finish)
