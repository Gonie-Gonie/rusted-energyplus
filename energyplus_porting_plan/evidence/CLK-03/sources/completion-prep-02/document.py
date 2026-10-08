"""Create proposed documents, preserving the literal existing historical record."""
import copy
import subprocess
from common import *

LIMITS=[
    'Only frozen CON-01 A/B nonactual hourly EPW cases and four timesteps/hour are promoted.',
    'The 14-sequence/51-request unit reference and the 3-case/483-call matched production reference have distinct inputs and chronology.',
    'Unavailable owners and interior snapshots, native private record indices and role-specific thermal caller attribution are never counted as PASS.',
    'Raw EPW data, processed WeatherVars carriers and later physical calculations are distinct owners. Generated weather physics, missing/range/sky/solar and interpolation RHS remain unpaired.',
    'Full 11-field daily and 17-field carrier transport is verified where observed; absent native interior boundaries do not acquire synthesized values.',
    'The preserved production comparison with 1,647,956 comparisons and one DatesShouldBeReset mismatch remains a real failure. The new matched caller-history input supplement does not change its status or original selection policy.',
    'Final unit and fresh production execute df88eb4c source; earlier 343647 science and precommit quality commands retain their own source identities.',
    'Available-source inventory certifies retained bytes rather than compiler file selection. Before/after guards cannot detect transient edits.',
    'Writer authored raw cursor/input/day_read and unit/production comparers. Independent result reviews authenticate those results and disclosed provenance; writer does not independently certify own implementation.',
    'This proposal changes no canonical files or gates and grants no native binary retirement. Root must review/apply and record actual canonical closure separately.',
]

def source_projection(source):
    selected=[
        ('GetNextEnvironment signature','GetNextEnvironment',['CloseWeatherFile','OpenEPlusWeatherFile']),
        ('InitializeWeather','InitializeWeather',['ReadWeatherForDay','UpdateWeatherData','SkipEPlusWFHeader']),
        ('UpdateWeatherData','UpdateWeatherData',[]),
        ('ReadWeatherForDay','ReadWeatherForDay',['ReadEPlusWeatherForDay']),
        ('ReadEPlusWeatherForDay','ReadEPlusWeatherForDay',['InterpretWeatherDataLine','SkipEPlusWFHeader','CalculateDailySolarCoeffs']),
        ('OpenEPlusWeatherFile and CloseWeatherFile','OpenEPlusWeatherFile',['ProcessEPWHeader','SkipEPlusWFHeader']),
        ('SkipEPlusWFHeader','SkipEPlusWFHeader',[]),
    ]
    ranges=[]
    pinned=ROOT/'.reference/energyplus-src/26.1.0'
    for original,symbol,helpers in selected:
        rows=[row for row in source['selected_ranges'] if row['symbol']==original and row['file']=='src/EnergyPlus/WeatherManager.cc']
        require(len(rows)==1,'Unique frozen source projection range required')
        row=rows[0];lines=(pinned/row['file']).read_bytes().splitlines(keepends=True)
        ref(pinned/row['file']);raw=b''.join(lines[row['start_line']-1:row['end_line']])
        require(hashlib.sha256(raw).hexdigest()==row['range_sha256'],'Actual frozen source-range bytes differ')
        require(any(symbol in line.decode('utf8') for line in lines[row['start_line']-1:row['end_line']]),'Literal projected source symbol absent')
        ranges.append({'path':row['file'],'symbol':symbol,'start_line':row['start_line'],'end_line':row['end_line'],
            'helpers':helpers,'range_sha256':row['range_sha256'],
            'projection_note':'Selected reviewed caller/helpers; full 109-range source packet and unpaired physical boundaries remain separately bound.'})
    return ranges

def gate(command,artifacts_list,note):
    return {'command':subprocess.list2cmdline(command['command']),'commit':command['repository_before']['head'],
        'exit_code':command['exit_code'],'artifacts':artifacts_list,'note':note}

def proposed_documents(science,quality,historical,writer_review):
    refs=science['refs'];prod=science['prod'];unit=science['unit'];matrix=science['matrix']
    cli_commands=[]
    require([case['case_id'] for case in matrix['cases']]==['A-24H','A-72H','B-BOTH-24H'],'Ordered production cases differ')
    for case in matrix['cases']:
        for level in ['Full','Summary']:
            binding=bound(case[level]['command_receipt']);actual=same_source(recorded(binding['path']))
            require(actual['repository_before']['head']==COMMIT and actual['repository_before']['worktree_clean'] is True
                and actual['repository_after']['worktree_clean'] is True,'Normal CLI source provenance differs')
            require(path_of(actual['command'][0])==path_of(science['cli']['binary']['path']) and actual['command'][1]=='run',
                'Actual ordinary CLI entry required')
            require(actual['executed_launcher']==science['cli']['actual_executed_recorder'],'Actual CLI recorder differs')
            cli_commands.append({'case_id':case['case_id'],'trace_level':level,'receipt':binding,'actual_argv':actual['command'],'exit_code':0})
    evidence={'schema':'clk03-bounded-closure-evidence.v1','card':'CLK-03','implementation_commit':COMMIT,'crates_tree':TREE,
        'source_snapshot':bound(science['cli']['source_snapshot']),'source_inventory_scope':science['cli']['source_inventory_scope'],
        'archived_source_vs_committed_blobs':science['cli']['archived_source_vs_committed_blobs'],
        'contracts':science['contracts'],'request':science['request'],'source_audits':science['source']['source_audits'],
        'original_reference':refs['original'],'original_data_review':refs['original_review'],'legacy_gap':refs['gap'],
        'unit_comparison':refs['unit'],'unit_comparison_execution':refs['unit_execution'],
        'unit_independent_result_review':refs['unit_review'],'unit_independent_result_review_execution':refs['unit_review_execution'],
        'Rust_probe_build':unit['Rust_build'],'Rust_probe_execution':unit['Rust_execution'],'Rust_probe_results':unit['Rust_results'],
        'production_comparison':refs['production'],'production_comparison_execution':refs['production_execution'],
        'production_matrix':refs['matrix'],'production_matrix_execution':refs['matrix_execution'],'Rust_cli_build':prod['Rust_build'],
        'matched_native_contract':refs['matched_contract'],'matched_request':refs['matched_request'],'matched_projection':refs['matched_projection'],
        'matched_native_reference':prod['matched_native_reference'],'matched_original_execution':prod['matched_original_execution'],
        'matched_original_results':prod['matched_original_results'],'matched_original_data_review':prod['matched_original_data_review'],
        'matched_original_data_review_execution':prod['matched_original_data_review_execution'],
        'declared_plan':refs['plan'],'independent_sources_review':refs['source_review'],
        'production_independent_result_review':refs['production_review'],'production_independent_result_review_execution':refs['production_review_execution'],
        'actual_production_commands':cli_commands,'unit_counts':copy.deepcopy(unit['comparison']),
        'production_counts':{key:prod[key] for key in ['comparison_count','mismatch_count','actual_command_count','comparison_categories']},
        'actual_production_observations':copy.deepcopy(prod['observations']),
        'quality_receipts':quality['refs'],'workspace_actual_counts':quality['counts'],
        'quality_actual_command_heads':quality['actual_command_heads'],'quality_actual_repository_states':quality['actual_repository_states'],
        'quality_checks_claimed_to_have_run_at_final_commit':False,'quality_source_scope':quality['scope'],
        'preserved_prior_scientific_failure':{'report':refs['failed_production'],'execution':refs['failed_production_execution'],
            'independent_failure_review':refs['failed_production_review'],'independent_failure_review_execution':refs['failed_production_review_execution'],
            'comparison_count':1647956,'mismatch_count':1,'scientific_certification_passed':False,'relabelled_unavailable_or_PASS':False},
        'preserved_reader_transport_failure':refs['failed_reader'],'preserved_historical_evidence':historical,
        'completion_writer_source_review':writer_review,'unit_reference_predates_final_unit':True,
        'matched_original_and_data_review_predate_fresh_six_CLI':True,'all_required_actual_commands_exit_zero_source_unchanged':True,
        'reference_answers_supplied':False,'original_RHS_reconstructed':False,'assignment_tolerance_used':False,
        'numeric_error_metrics':'Exact selected typed/finite binary64 comparisons; actual mismatch counts zero. No RMSE recomputed by closure writer.',
        'unavailable_counted_as_PASS':False,'physical_weather_RHS_certified':False,'limits':LIMITS,
        'engines_Cargo_Git_or_comparers_executed_by_writer':False,'canonical_gates_updated_by_writer':False}
    evidence_bytes=encoded(evidence)
    evidence_ref={'path':PLAN+'evidence/CLK-03.json','sha256':hashlib.sha256(evidence_bytes).hexdigest()}
    old=read(PLAN+'plan.json');plan=copy.deepcopy(old);task=next(row for row in plan['tasks'] if row['id']=='CLK-03')
    require(all(task[key]=='미확인' for key in GATES),'Only unclosed CLK-03 can be proposed')
    for predecessor in ['CON-01','CLK-01','CLK-02']:
        require(all(next(row for row in plan['tasks'] if row['id']==predecessor)[key]=='통과' for key in GATES),'Prerequisite gates incomplete')
    task.update({'symbols':'GetNextEnvironment; ReadEPlusWeatherForDay; UpdateWeatherData; InitializeWeather',
        'source_boundary_note':'R3 15파일/109구간과 14/51 단위 입력은 보존한다. 생산 호출 이력 3사례/483호출을 실행 전에 별도 동결하고 실제 원본·소비 대조를 수행했다. 처리된 물리 계산과 관측 불가 상태는 인증하지 않는다.',
        'unit_tests':'14 sequence/51 요청: 48 호출, 46 정상 반환, 의도한 fatal2/후속skip3; exact239178/0 및 독립 결과 검토',
        'integration_test':'라이브 byte cursor·Today/Tomorrow 전체 인계와 실제 A/B owned 소비; matched 원본3환경/480 Initialize의 시간·상태 이력 대조',
        'numeric_policy':'동결한 selected typed/copy/order/finite binary64 bit exact, atol=0/rtol=0. 물리 RHS와 관측 불가 경계는 PASS에서 제외.',
        **{key:'통과' for key in GATES}})
    common=[*science['contracts'].values(),science['request'],refs['original'],refs['original_review'],refs['matched_contract'],refs['plan'],refs['source_review']]
    task['evidence']={'scopes':['A','B'],'implementation_commit':COMMIT,'source_ranges':source_projection(science['source']),
        'complete_source_packet':science['contracts']['source'],
        'input_hashes':[science['request'],refs['matched_request']]+[bound(case[key]) for case in matrix['cases'] for key in ['input','weather','metadata']],
        'reference':{'kind':'cpp-wrapper','artifacts':[refs['original'],prod['matched_native_reference'],prod['matched_original_results'],prod['matched_original_data_review']]},
        'checks':{'scope_review':gate(science['unit_cmd'],common+[refs['unit_review']], 'Frozen source, input-only caller schedule, independent source/results and explicit unpaired boundaries.'),
            'unit_gate':gate(science['unit_cmd'],[refs['unit'],refs['unit_execution'],refs['unit_review'],refs['unit_review_execution']], 'Actual final239178 exact comparisons, zero mismatches; unavailable never PASS.'),
            'integration_gate':gate(science['prod_cmd'],[refs['production'],refs['production_execution'],refs['matrix'],refs['production_review']], 'Actual live mutable owner and source-matched full caller history; no downstream physics claim.'),
            'production_gate':gate(science['prod_cmd'],[refs['production'],refs['production_execution'],refs['matrix'],refs['production_review'],*quality['refs'].values()], 'Six ordinary Full/Summary runs after genuine matched original/data review, exact owned handoff and ordinary equality, all actual QA passed.')},
        'production':{'rust_entrypoint':science['cli']['binary']['path']+' run <fixed-IDF> --weather <pinned-EPW> --trace-level full|summary',
            'oracle_inputs_used':False,'fixture_inputs_used':False,'completed_actual_cases':['A-24H','A-72H','B-BOTH-24H'],'actual_successful_normal_CLI_commands':6},
        'comparison':{'contract':science['contracts']['tolerances'],'report':refs['production'],'unit_report':refs['unit']},
        'scope_metrics':{'unit':unit['comparison'],'production':evidence['production_counts']},
        'provenance':{'implementation_commit':COMMIT,'crates_tree':TREE,'source_snapshot':science['cli']['source_snapshot'],'compact_evidence':evidence_ref},
        'implementation_validation':{'required_checks':quality['refs'],'workspace_actual_counts':quality['counts'],'all_actual_exit_zero_source_unchanged':True},
        'limitations':LIMITS}
    require(all(exact(a,b) for a,b in zip(old['tasks'],plan['tasks'],strict=True) if a['id']!='CLK-03'),'Other task changed')
    require(all(exact(old[key],plan[key]) for key in old if key!='tasks'),'Plan root changed')
    card_path=PLAN+'cards/CLK-03.md';card=path_of(card_path).read_text(encoding='utf8')
    require(card.count('- [ ] ')==4 and card.count('구현 커밋:  \n')==1,'Unclosed card shape differs')
    card=card.replace('- [ ] ','- [x] ').replace('구현 커밋:  \n',f'구현 커밋: `{COMMIT}`; crates tree `{TREE}`\n')
    replacements={'시험 명령:  \n':'시험 명령: 실제 unit·production·workspace·Clippy·구조·형식 명령 및 기록은 아래 종료 증거에 결합한다.\n',
        '증거 경로:  \n':'증거 경로: [종료 증거](../evidence/CLK-03.json).\n',
        '최대오차/RMSE/상태 불일치:  \n':f'최대오차/RMSE/상태 불일치: exact 단위 239,178건/0, 생산 {prod["comparison_count"]:,}건/0. RMSE를 재계산하지 않는다.\n',
        '추가 검토할 helper:\n':'추가 검토할 helper: CLK-04/06 및 기존 물리 계산·native 내부 상태는 계속 별도 검증 대상이다.\n'}
    for before,after in replacements.items():require(card.count(before)==1,'Card placeholder missing');card=card.replace(before,after)
    card+='\n## 최종 생산 호출 이력 대조와 종료\n\n'+(
        '앞 절의 준비·실패·초기 결과는 당시 기록으로 보존한다. 이전 생산 대조는 1,647,956건 중 DatesShouldBeReset 한 건이 불일치했다. 생산은 각 timestep에 Initialize를 호출했지만 최초 원본 대조 입력은 일 경계만 호출했다. 실패 보고서의 값·정책·상태를 바꾸지 않았다.\n\n'
        '현재 Rust caller 소스의 부분 대입과 실제 호출 시점을 입력만으로 재구성한 3사례/483호출 계약과 관측 선택을 새 원본 실행 전에 동결했다. 새 genuine 원본의 3개 GetNextEnvironment·480개 Initialize 호출과 독립 결과 검토가 끝난 뒤, 같은 최종 커밋 EXE로 Full/Summary 6개를 실행했다. Rust trace나 원본 결과값은 실행 입력으로 공급하지 않았다.\n\n'
        f'최종 커밋의 단위 대조는 239,178건/불일치0, 새 생산 대조는 {prod["comparison_count"]:,}건/불일치0이다. 두 결과는 각각 별도의 독립 메타데이터 검토를 받았다. workspace 4,580개, Clippy·source-quality·structure·scoped rustfmt도 실제 종료 코드0이며 최종 source snapshot과 연결한다. 단위 원본14/51과 별도 생산 원본3/483의 시간 순서는 구분한다.\n\n'
        '완료 범위는 CON-01의 비실측 hourly EPW와 4 timestep 환경에서 라이브 커서·선택 상태·전체 carrier 복사·owned A/B 소비 인계이다. 결측·보간·sky·solar·처리된 기상 물리 RHS, native private index와 관측 불가 경계에는 PASS를 부여하지 않는다. 자세한 실제 명령·입력·원본·소스 해시·실패 보존·독립 검토는 [종료 증거](../evidence/CLK-03.json)에 기록한다.\n')
    return {evidence_ref['path']:evidence_bytes,PLAN+'plan.json':encoded(plan),card_path:card.encode('utf8')},evidence_ref
