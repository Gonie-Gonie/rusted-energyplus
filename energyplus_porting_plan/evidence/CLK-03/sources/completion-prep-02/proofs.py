"""Authenticate actual final unit and matched-production reports, without comparing operands."""
from common import *

PINNED={
    'unit':('final-candidate-unit-comparison-01/unit-comparison.json','80fec36962d0583e7c69bd85e40afddb1cbb645ea0bc1ad53789c7b53a4d95c4'),
    'unit_execution':('final-candidate-unit-comparison-command-01/receipt.json','d4f77a8d43a86f76f813345de679bfefb5384c135ef4dfff2ac9e71ae3e0900d'),
    'unit_review':('independent-final-unit-result-review-01/review.json','d2b1475f7f71760856a6f8de84138b8d7bbfdd591643d961140fc4f5a8b7342a'),
    'unit_review_execution':('independent-final-unit-result-review-command-01/receipt.json','a7dc77769546d6164a90a536b21ab703550f4a3984dc6471ab7e393cc95db38e'),
    'plan':('production-matched-plan-01/plan.json','e549cad6a6926f4631b6e9f109931f793dbdf668259f6209625279270ebb3bc9'),
    'source_review':('independent-matched-production-sources-review-01/review.json','250cd20c506ce9c7c5b66614a748f889d486e71f09f21565aae93d676d659c68'),
    'matched_contract':('production-matched-inputs-01/native-contract.json','fa534ac3525b8d1e63e793ef37e46ba082501c1d0e149e111588d2096392d984'),
    'matched_request':('production-matched-inputs-01/helper-request.json','e989b1d9aa052066040536a3320a91f9d512e06b6a33c14c46924aae526eb3d1'),
    'matched_projection':('production-matched-inputs-01/projection.json','b768e6d30c5ac6b9a63602f6aef531acd87703b3ace2a105b7773fcee206cb7a'),
    'original':('original-helper-first-03/helper-reference.json','0513bc94d02703d299cb0a63bc66fa11a0b405e0d0873d3e61a0916a4e123cdc'),
    'original_review':('independent-original-data-review-02/review.json','b8874a962db7c71f6395a5b3ccd811e780fffacc048a76d67bc215634f756166'),
    'gap':('legacy-gap-05/gap.json','340365c467b4dd606fa9721261e0ffd52f762662943875d1f2aab928bf858097'),
    'failed_production':('production-comparison-02/production-comparison.json','085076f535c285794e893e064b210f7536dcbbf7b02d5de06ae60ede5ab253a2'),
    'failed_production_execution':('production-comparison-command-02/receipt.json','0fb86b4517921fb0bf9c9b236c26044d5c32ed13250c54823123d32d13ab5e5a'),
    'failed_production_review':('independent-production-failure-review-01/review.json','77645ff38c2aab3a7b3c9a4b673b7abdfaa9e63ea70148f749c587fce6a7d4d6'),
    'failed_production_review_execution':('independent-production-failure-review-command-01/receipt.json','06906d3ed754ebee7b6d1598f4b13fc281cf57574cc315d6ecfb777becd7da5d'),
    'failed_reader':('production-comparison-command-01/receipt.json','5fdcca184e8c8a29c96f07705abddfa4e7c8dee3d3bda1a8b5b168a0ed56b701'),
}

def build(binding,kind):
    item=read(bound(binding)['path'])
    require(item['schema']=='porting-Rust-build.v1' and item['kind']==kind,'Actual canonical build kind differs')
    require(item['implementation_commit']==item['repository_head']==COMMIT and item['crates_tree']==TREE,'Final committed build identity differs')
    require(item['source_worktree_clean'] is True and item['committed_source_certification'] is True,'Build was not committed and clean')
    require(bound(item['source_snapshot'])['sha256']==SNAPSHOT and item['available_source_inventory_count']==3718,'Final available inventory differs')
    command=same_source(recorded(bound(item['build_execution'])['path']))
    require(command['source_snapshot']==item['source_snapshot'] and command['repository_before']['head']==COMMIT,'Compiler source identity differs')
    require(command['executed_launcher']==item['actual_executed_recorder'],'Actual build recorder differs')
    require(bound(item['binary'])==bound(item['actual_Cargo_executable']['matching_archive']),'Emitted executable archive differs')
    require(item['actual_Cargo_executable']['cargo_message']['reason']=='compiler-artifact','Compiler artifact missing')
    require(item['gates_updated'] is False and item['reference_outputs_supplied'] is False,'Build policy differs')
    for row in item['available_Rust_sources']:bound({'path':row['path'],'sha256':row['sha256']})
    artifacts(item['archived_source_vs_committed_blobs'])
    return item

def review(report_binding,execution_path,schema,status,expected,compact=False):
    item=read(bound(report_binding)['path']);command=recorded(execution_path)
    require(item['schema']==schema and item['status']==status,'Independent factual review status differs')
    emitted={'review':report_binding,'status':item['status']} if compact else item
    require(exact(read(command['stdout']['path']),emitted),'Actual independent review stdout protocol differs')
    require(timestamp(command['started_utc'])<=timestamp(item['review_started_utc'])
        <=timestamp(item['review_completed_utc'])<=timestamp(command['finished_utc']),'Independent review chronology differs')
    for key,binding in expected.items():require(bound(item[key])==bound(binding),'Independent review binding differs: '+key)
    require(item['unavailable_counted_as_PASS'] is False,'Independent review counts unavailable as PASS')
    require(type(item['authorship_disclosure']) is str and item['authorship_disclosure'].strip(),'Review authorship disclosure required')
    artifacts(item.get('executed_reader_source_archives',[]))
    return item,command

def original_chronology(prod,matrix,refs):
    reference=read(bound(prod['matched_native_reference'])['path']);artifacts(reference)
    outer=recorded(bound(prod['matched_original_execution'])['path'])
    require(reference['schema']=='clk03-matched-production-original-reference.v1'
        and type(reference['actual_helper_exit_code']) is int and type(reference['actual_launcher_exit_code']) is int
        and reference['actual_helper_exit_code']==reference['actual_launcher_exit_code']==0
        and reference['preservation_checks_passed'] is True and reference['source_expected_answers_supplied'] is False,
        'Actual successful genuine matched Original required')
    require(bound(reference['actual_contract'])==refs['matched_contract']
        and bound(reference['actual_request'])==refs['matched_request'] and bound(reference['input_projection'])==refs['matched_projection']
        and bound(reference['production_plan'])==refs['plan'],'Original actual supplemental packet differs')
    require(bound(reference['results'])==bound(prod['matched_original_results']),'Actual original results binding differs')
    require(exact(read(outer['stdout']['path']),{'reference':prod['matched_native_reference'],'status':reference['status'],
        'actual_helper_exit_code':0,'actual_launcher_exit_code':0}),'Original compact actual stdout differs')
    child=read(bound(reference['execution'])['path'])
    require(child['schema']=='original-native-command.v1' and type(child['exit_code']) is int and child['exit_code']==0
        and child['spawn_error'] is None and path_of(child['cwd'])==ROOT,'Native actual child outcome differs')
    require(len(child['command'])==5 and [path_of(value) for value in child['command']]==[
        path_of(reference['executed_helper']['path']),ROOT,path_of(refs['matched_request']['path']),
        path_of(refs['matched_contract']['path']),path_of(reference['results']['path']).parent],
        'Actual native argc5 input-only command differs')
    artifacts(child)
    require(timestamp(read(refs['plan']['path'])['created_utc'])<=timestamp(outer['started_utc'])
        <=timestamp(child['started_utc'])<=timestamp(child['finished_utc'])<=timestamp(outer['finished_utc']),
        'Frozen policy/original child chronology differs')
    build=read(bound(reference['native_driver_build'])['path']);artifacts(build)
    linked=build['linked_original'];linked_path=path_of(linked['historical_path'])
    require(not linked_path.is_symlink() and linked_path.stat().st_size==linked['size_bytes']
        and ref(linked_path)['sha256']==linked['sha256'],'Actual newly linked original bytes differ')
    build_review=read(bound(reference['independent_actual_build_review'])['path']);artifacts(build_review)
    require(build['schema']=='clk03-matched-production-native-build.v1' and build['checks_passed'] is True
        and build['scientific_source_patches'] is False and build['scientific_execution_performed'] is False
        and build['assertions_enabled'] is True and build['fp_contract']=='off'
        and bound(build['helper_binary'])==bound(reference['executed_helper'])
        and build_review['schema']=='clk03-independent-matched-production-native-build-review.v1'
        and build_review['status']=='pass-build-and-derivative-before-original-execution'
        and bound(build_review['reviewed_build'])==bound(reference['native_driver_build'])
        and timestamp(build['finished_utc'])<=timestamp(build_review['review_completed_utc'])<=timestamp(child['started_utc']),
        'Genuine actual build and independent build review missing')
    data=read(bound(prod['matched_original_data_review'])['path']);data_cmd=recorded(bound(prod['matched_original_data_review_execution'])['path'])
    require(data['schema']=='clk03-independent-matched-production-original-data-review.v1'
        and data['status']=='pass-original-matched-history-before-production' and data['scientific_values_compared'] is False
        and data['actual_operation_invocations']==483 and data['actual_operations_skipped']==0,'Actual initial data review incomplete')
    for key,binding in [('reviewed_reference',prod['matched_native_reference']),('reviewed_results',reference['results']),
        ('reviewed_execution',reference['execution']),('reviewed_outer_execution',prod['matched_original_execution']),
        ('reviewed_request',refs['matched_request']),('reviewed_contract',refs['matched_contract']),
        ('reviewed_build',reference['native_driver_build']),('reviewed_build_review',reference['independent_actual_build_review'])]:
        require(bound(data[key])==bound(binding),'Initial matched data-review binding differs: '+key)
    require(exact(read(data_cmd['stdout']['path']),data) and timestamp(outer['finished_utc'])<=timestamp(data_cmd['started_utc'])
        <=timestamp(data['review_started_utc'])<=timestamp(data['review_completed_utc'])<=timestamp(data_cmd['finished_utc']),
        'Original then independent actual data chronology differs')
    for case in matrix['cases']:
        for level in ['Full','Summary']:
            receipt=recorded(bound(case[level]['command_receipt'])['path'])
            require(timestamp(data_cmd['finished_utc'])<=timestamp(receipt['started_utc']),
                'Fresh actual CLI ran before genuine matched Original and independent data')
    return reference

def actual_science(args):
    records={};refs={}
    for key,(name,sha) in PINNED.items():
        records[key]=read(BASE+name,sha);refs[key]=ref(BASE+name)
    contracts={key:ref(PLAN+'contracts/CLK-03-'+key+'.json') for key in CONTRACTS}
    for key,sha in CONTRACTS.items():require(contracts[key]['sha256']==sha,'R3 packet changed')
    source=read(contracts['source']['path']);cases=read(contracts['cases']['path'])
    require(source['packet_revision']==cases['packet_revision']==3 and len(source['source_files'])==15
        and len(source['selected_ranges'])==109,'R3 source coverage differs')
    request=bound(cases['helper_request']);artifacts(cases);artifacts(source['source_audits'])
    require(cases['counts']=={'handoff_sequences':4,'weather_sequences':10,'handoff_operations':7,'weather_operations':44,
        'total_sequences':14,'total_operations':51},'Original unit requests changed')
    unit,unit_cmd=reported(refs['unit']['path'],refs['unit_execution']['path']);same_source(unit_cmd)
    require(unit['schema']=='clk03-unit-comparison.v1' and unit['status']=='pass-exact-selected-carrier-and-lifecycle-state'
        and unit['scientific_certification_passed'] is True,'Actual scoped unit PASS required')
    require(unit['comparison']['comparison_count']==239178 and unit['comparison']['mismatch_count']==0
        and unit['comparison']['mismatches']==[] and unit['comparison']['unavailable_operands_counted_as_PASS'] is False,'Unit count/policy differs')
    require(exact(unit['contracts'],contracts) and exact(unit['request'],request),'Unit frozen input differs')
    for key in ['engines_executed_by_reader','Cargo_executed_by_reader','Git_executed_by_reader',
        'reference_outputs_supplied_to_Rust','original_RHS_recomputed','gates_updated','unavailable_counted_as_PASS']:
        require(unit[key] is False,'Unit scope differs: '+key)
    probe=build(unit['Rust_build'],'example');artifacts(unit)
    require(unit_cmd['executed_launcher']==probe['actual_executed_recorder'],'Unit recorder identity differs')
    qa_unit,_=review(refs['unit_review'],refs['unit_review_execution']['path'],'clk03-independent-unit-result-review.v1',
        'pass-authenticated-selected-unit-result-and-honest-unavailable-scope',
        {'reviewed_report':refs['unit'],'reviewed_comparison_execution':refs['unit_execution'],'reviewed_Rust_build':unit['Rust_build']},compact=True)
    require(qa_unit['actual_reported_comparisons']==239178 and qa_unit['actual_reported_mismatches']==0
        and qa_unit['unit_claimed_to_follow_future_supplemental_matched_production_original'] is False,'Final unit review scope differs')
    prod,prod_cmd=reported(args.production_comparison,args.production_comparison_execution);same_source(prod_cmd)
    require(prod['schema']=='clk03-matched-production-comparison.v1' and prod['status']=='pass-selected-live-cursor-Today-and-owned-consumer-handoff'
        and prod['scientific_certification_passed'] is True,'Real matched production PASS required')
    require(type(prod['comparison_count']) is int and prod['comparison_count']>0 and type(prod['mismatch_count']) is int
        and prod['mismatch_count']==0 and prod['mismatches']==[] and prod['actual_command_count']==6,'Actual matched production counts differ')
    for key in ['unavailable_counted_as_PASS','engines_executed_by_reader','Cargo_executed_by_reader','Git_executed_by_reader',
        'reference_outputs_supplied_to_Rust','original_RHS_reconstructed','physical_producer_numerical_parity_claimed',
        'native_thermal_callback_parity_claimed','gates_updated','prior_mismatch_relabelled_unavailable_or_PASS']:
        require(prod[key] is False,'Production limit differs: '+key)
    require(prod['prior_failed_scientific_report_preserved'] is True and prod['implementation_commit']==COMMIT
        and prod['crates_tree']==TREE and exact(prod['contracts'],contracts),'Final production identity/history differs')
    for key,pin in [('declared_plan','plan'),('independent_sources_review','source_review'),('matched_native_contract','matched_contract'),
        ('unit_comparison','unit'),('unit_comparison_execution','unit_execution')]:
        require(bound(prod[key])==refs[pin],'Production proof differs: '+key)
    artifacts(prod);cli=build(prod['Rust_build'],'cli')
    require(prod_cmd['executed_launcher']==cli['actual_executed_recorder'],'Production comparer recorder differs')
    matrix=read(bound(prod['Rust_matrix'])['path']);matrix_cmd=same_source(recorded(args.matrix_execution))
    require(matrix['schema']=='clk03-matched-production-matrix.v1' and matrix['complete'] is True
        and matrix['actual_command_count']==6 and matrix['original_outputs_supplied_to_Rust'] is False
        and matrix['gates_updated'] is False and exact(matrix['build'],prod['Rust_build']),'Actual matched matrix differs')
    emitted=path_of(matrix_cmd['stdout']['path']).read_text(encoding='utf8').splitlines()
    position=0
    for case,level in ((case,level) for case in matrix['cases'] for level in ['Full','Summary']):
        child_ref=bound(case[level]['command_receipt']);child=recorded(child_ref['path'])
        # Authenticated record_command.py forwards each bound log's final8000
        # bytes as UTF-8 with replacement before emitting its compact receipt.
        forwarded=''.join(path_of(child[key]['path']).read_bytes()[-8000:].decode('utf8',errors='replace')
            for key in ['stdout','stderr'])
        forwarded_lines=forwarded.splitlines()
        require(emitted[position:position+len(forwarded_lines)]==forwarded_lines,'Forwarded actual child log tails differ')
        position+=len(forwarded_lines)
        require(position+1<len(emitted)
            and exact(decode(emitted[position]),{'receipt':child_ref,'exit_code':0,'source_bytes_match_before_and_after':True})
            and emitted[position+1]==case['case_id']+' '+level+' actual_exit=0' and type(child['exit_code']) is int
            and child['exit_code']==0 and child['source_bytes_match_before_and_after'] is True,
            'Actual ordered compact child recorder receipt/progress protocol differs')
        position+=2
    require(position==len(emitted)-1 and exact(decode(emitted[position]),bound(prod['Rust_matrix'])),
        'Actual exact six child log/receipt/progress groups and final matrix binding required')
    require(matrix_cmd['executed_launcher']==cli['actual_executed_recorder'],'Matrix recorder differs')
    original_chronology(prod,matrix,refs)
    refs.update({'production':ref(args.production_comparison),'production_execution':ref(args.production_comparison_execution),
        'matrix':bound(prod['Rust_matrix']),'matrix_execution':ref(args.matrix_execution),
        'production_review':ref(args.production_result_review),'production_review_execution':ref(args.production_result_review_execution)})
    qa_prod,qa_prod_cmd=review(refs['production_review'],args.production_result_review_execution,'clk03-independent-production-result-review.v1',
        'pass-authenticated-selected-production-result-and-honest-unavailable-scope',
        {'reviewed_report':refs['production'],'comparison_execution':refs['production_execution'],'Rust_build':prod['Rust_build'],
        'matrix':refs['matrix'],'plan':refs['plan'],'matched_native_contract':refs['matched_contract'],
        'matched_native_reference':prod['matched_native_reference'],'matched_original_data_review':prod['matched_original_data_review']})
    require(timestamp(prod_cmd['finished_utc'])<=timestamp(qa_prod['review_started_utc']),'Production QA preceded result')
    require(qa_prod['actual_reported_comparisons']==prod['comparison_count'] and qa_prod['actual_reported_mismatches']==0,
        'Independent production count transcription differs')
    require(records['gap']['numerical_comparison_count'] is None and records['gap']['numerical_mismatch_count'] is None
        and records['gap']['numerical_PASS_claimed'] is False,'Legacy unavailable observations relabelled PASS')
    failure=records['failed_production'];failed_cmd=recorded(refs['failed_production_execution']['path'],1)
    require(failure['comparison_count']==1647956 and failure['mismatch_count']==1
        and failure['scientific_certification_passed'] is False
        and exact(read(failed_cmd['stdout']['path']),failure),'Historical genuine scientific failure differs')
    failed_review=records['failed_production_review'];failed_review_cmd=recorded(refs['failed_production_review_execution']['path'])
    require(failed_review['schema']=='clk03-independent-production-failure-review.v1'
        and failed_review['status']=='authenticated-failed-production-result-with-unmatched-incoming-history'
        and failed_review['authenticated_actual_comparison_count']==1647956 and failed_review['authenticated_actual_mismatch_count']==1
        and failed_review['scientific_certification_passed'] is False
        and bound(failed_review['reviewed_report'])==refs['failed_production']
        and bound(failed_review['comparison_execution'])==refs['failed_production_execution']
        and exact(read(failed_review_cmd['stdout']['path']),refs['failed_production_review']),
        'Actual independently authenticated historical scientific failure differs')
    recorded(refs['failed_reader']['path'],1)
    return {'records':records,'refs':refs,'contracts':contracts,'source':source,'cases':cases,'request':request,
        'unit':unit,'unit_cmd':unit_cmd,'prod':prod,'prod_cmd':prod_cmd,'matrix':matrix,'matrix_cmd':matrix_cmd,
        'probe':probe,'cli':cli,'qa_unit':qa_unit,'qa_prod':qa_prod,'qa_prod_cmd':qa_prod_cmd}
