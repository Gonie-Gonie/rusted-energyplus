"""Compare already recorded CLK-03 production owners and actual operands.

No engine, compiler, Git, native callback or scientific RHS is executed here.
Native processed weather numbers remain unpaired; observations are not inputs.
"""
from __future__ import annotations

import argparse
from datetime import datetime, timezone
import json
from pathlib import Path
import shutil
import sys

sys.dont_write_bytecode=True
from clk03_production_provenance import (ROOT, Bindings, FULL_ONLY, RAW_REFERENCE, RAW_REVIEW,
    evidence, exact, packet, path_of, plan, read, ref, recorded_run, require,
    same_binding, source_architecture, timestamp, verify_contract_bindings)
from clk03_unit_state import Comparison
from clk03_production_state import TRACE_NAME, observe, selected_policy

PRIOR = ROOT/'.runtime/porting/CLK-03/production-reader-prep-01'

def source_bundle(directory=None):
    directory=Path(__file__).resolve().parent if directory is None else directory
    names=['check_clk03_production.py','clk03_production_state.py','clk03_production_provenance.py','run_production.py','prepare_plan.py']
    if directory!=PRIOR:names.append('reader_amendment.py')
    return [directory/name for name in names]+[
            ROOT/'tools/porting'/name for name in ['check_clk03_unit.py','clk03_unit_state.py','geo03_provenance.py']]


def source_review(binding,refs,bindings,directory=None):
    reviewed=read(bindings.check(binding))
    require(reviewed['schema']=='clk03-independent-production-sources-review.v1'
            and reviewed['status']=='pass-source-and-observation-selection-before-production'
            and reviewed['scientific_execution_performed'] is False,'Independent pre-execution production source review required')
    verify_contract_bindings(reviewed['reviewed_contracts'],refs,bindings)
    rows={row['historical_path']:row['archive'] for row in reviewed['reviewed_sources']}
    sources=source_bundle(directory)
    require(len(rows)==len(reviewed['reviewed_sources'])==len(sources),'Exact reviewed source bundle required')
    for path in sources:
        key=path.relative_to(ROOT).as_posix()
        require(key in rows and rows[key]['sha256']==ref(path)['sha256'],'Reviewed production source differs: '+key)
        bindings.check(rows[key])
    return reviewed


def frozen_sources(declared,bindings,directory=None):
    archived={row['historical_path']:row['archive'] for row in declared['reader_source_archives']}
    require(len(archived)==len(declared['reader_source_archives'])==len(source_bundle(directory)),'Complete frozen source projection required')
    for path in source_bundle(directory):
        key=path.relative_to(ROOT).as_posix()
        require(key in archived and archived[key]['sha256']==ref(path)['sha256'],'Frozen executed reader/source differs')
        bindings.check(archived[key])


def caller_sources(trace,available,bindings):
    owner='crates/ep_runtime/src/heat_balance/weather_driver.rs'
    lines=bindings.check(available[owner]).read_text(encoding='utf8').splitlines()
    for row in trace['consumers']:
        call=row['caller'];require(set(call)=={'file','line','column'} and call['file'].replace('\\','/')==owner
            and type(call['line']) is int and type(call['column']) is int and 1<=call['line']<=len(lines)
            and 1<=call['column']<=len(lines[call['line']-1])+1
            and 'series.current_for(' in lines[call['line']-1],'Actual consumer caller does not point to archived owned payload hook')


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    for key in ['matrix','reader-amendment','reader-amendment-execution','output-dir']:parser.add_argument('--'+key,type=Path,required=True)
    args=parser.parse_args();require(Path.cwd().resolve()==ROOT,'Repository cwd required')
    output=args.output_dir.resolve();require(output.is_relative_to(ROOT/'.runtime/porting/CLK-03') and not output.exists(),'Fresh contained output required')
    started=datetime.now(timezone.utc).isoformat();bindings=Bindings();check=Comparison()
    refs,source,cases,request=packet(bindings)
    matrix_ref=ref(args.matrix);matrix=read(bindings.check(matrix_ref))
    require(matrix['schema']=='clk03-production-matrix.v1' and matrix['complete'] is True
            and matrix['actual_command_count']==6 and matrix['original_outputs_supplied_to_Rust'] is False
            and matrix['gates_updated'] is False,'Complete actual six CLI matrix required')
    verify_contract_bindings(matrix['contracts'],refs,bindings)
    plan_ref,declared=plan(bindings.check(matrix['declared_plan']),refs,cases,selected_policy(),bindings)
    reviewed=source_review(matrix['independent_sources_review'],refs,bindings,PRIOR)
    require(same_binding(matrix['independent_sources_review'],declared['independent_sources_review']), 'Matrix and frozen plan source review differ')
    require(matrix['executed_launcher']['sha256']==ref(PRIOR/'run_production.py')['sha256'], 'Actually executed historical launcher source differs')
    bindings.check(matrix['executed_launcher'])
    frozen_sources(declared,bindings,PRIOR)
    amendment_ref=ref(args.reader_amendment)
    from reader_amendment import authenticate_amendment
    amendment_execution_ref=ref(args.reader_amendment_execution)
    amendment=authenticate_amendment(amendment_ref,amendment_execution_ref,matrix_ref,plan_ref,refs,selected_policy(),source_bundle(),bindings)
    amendment_command=read(bindings.check(amendment_execution_ref))
    unit,native,raw_native,build,available,finished=evidence(matrix,refs,cases,bindings)
    architecture=source_architecture(available,bindings)
    planned=max(timestamp(declared['created_utc']),timestamp(reviewed['review_completed_utc']))
    require(timestamp(reviewed['review_completed_utc'])<=timestamp(declared['created_utc']), 'Source review must precede plan freeze')
    expected=declared['cases'];require(len(matrix['cases'])==len(expected)==3,'Exactly three CON cases required')
    maps=read(bindings.check(cases['input_byte_maps']))
    native_cases={row['id']:row for row in native['weather_sequences']}
    seen=set();observations=[];commands=0
    for actual,declared_case in zip(matrix['cases'],expected,strict=True):
        require(all(exact(actual[key],declared_case[key]) for key in ['case_id','scope','duration','input','weather','metadata']), 'Executed case identity differs')
        outputs={};summaries={}
        for level in ['Full','Summary']:
            directory,summary,artifacts=recorded_run(actual,actual[level],level,build,finished,planned,bindings,seen)
            commands+=1;outputs[level]=directory;summaries[level]=summary
            if level=='Summary':
                require(not any((directory/name).exists() for name in FULL_ONLY),'Summary enabled Full-only observation')
            else:
                trace_ref=artifacts.get(directory/TRACE_NAME);require(trace_ref is not None,'Full missing actual live weather-day observation')
                trace=read(bindings.check(trace_ref));caller_sources(trace,available,bindings)
                entries=[row for row in maps['files'] if same_binding(row['input'],actual['weather'])]
                require(len(entries)==1,'Exact frozen EPW byte map required')
                observation=observe(check,trace,actual,native_cases[actual['case_id']],raw_native,entries[0],bindings.check(actual['weather']).read_bytes())
                observation['actual_trace']=trace_ref
                observation['actual_runtime_class']=summary['rust_runtime']['runtime_class']
                observation['role_specific_caller_attribution_available']=False
                observation['role_unavailable_counted_as_PASS']=False
                observations.append(observation)
        full,summary=summaries['Full'],summaries['Summary'];label=actual['case_id']
        check.compare(sorted(full),sorted(summary),label+'/ordinary_summary_keys','Full_Summary_ordinary_equality')
        for key in set(full)-{'artifacts','timing','input','config'}:
            check.value(full[key],summary[key],label+'/ordinary/'+key,'Full_Summary_ordinary_equality')
        check.value({key:v for key,v in full['config'].items() if key!='trace_level'},
                    {key:v for key,v in summary['config'].items() if key!='trace_level'},label+'/config','Full_Summary_ordinary_equality')
        for name in ['results/selected-outputs.csv','results/meters.csv']:
            a,b=[bindings.check(ref(outputs[level]/name)).read_bytes() for level in ['Full','Summary']]
            check.compare(a==b,True,label+'/'+name,'Full_Summary_output_bytes')
        for name in ['results/result-store.json','porting_scope.json']:
            check.value(read(bindings.check(ref(outputs['Full']/name))),read(bindings.check(ref(outputs['Summary']/name))),label+'/'+name,'Full_Summary_output_values')
    require(commands==6,'Actual six distinct CLI commands required');bindings.unchanged()
    output.mkdir();source_dir=output/'source';source_dir.mkdir();archives=[]
    for path in source_bundle():
        archive=source_dir/path.name;shutil.copyfile(path,archive)
        require(path.read_bytes()==archive.read_bytes(),'Executed source archive differs')
        archives.append({'historical_path':path.relative_to(ROOT).as_posix(),'archive':ref(archive)})
    report={'schema':'clk03-production-comparison.v1',
        'status':'pass-selected-live-cursor-Today-and-owned-consumer-handoff' if check.mismatch_count==0 else 'fail-production-owner-or-handoff',
        'scientific_certification_passed':check.mismatch_count==0,'started_utc':started,'completed_utc':datetime.now(timezone.utc).isoformat(),
        'actual_reader_command':list(sys.orig_argv),'actual_reader_cwd':str(ROOT),'contracts':refs,
        'Rust_matrix':matrix_ref,'declared_plan':plan_ref,'independent_sources_review':matrix['independent_sources_review'],
        'reader_transport_amendment':amendment_ref,'amended_reader_independent_source_review':amendment['independent_source_review'],
        'reader_transport_amendment_execution':amendment_execution_ref,
        'metadata_amendment_source_identity':{'source_snapshot':amendment_command['source_snapshot'],
            'repository_head':amendment_command['repository_after']['head'],'counted_as_historical_scientific_build_identity':False},
        'preserved_failed_reader_execution':amendment['preserved_failed_reader_execution'],
        'unit_comparison':matrix['unit_comparison'],'unit_comparison_execution':matrix['unit_comparison_execution'],
        'executed_production_launcher':matrix['executed_launcher'],'Rust_build':matrix['build'],
        'implementation_commit':build['implementation_commit'],'crates_tree':build['crates_tree'],
        'original_results':unit['original_results'],'preserved_raw_reference':RAW_REFERENCE,'preserved_raw_data_review':RAW_REVIEW,
        'actual_command_count':commands,'comparison_count':check.count,'mismatch_count':check.mismatch_count,
        'comparison_categories':dict(check.categories),'mismatches':check.mismatches,
        'unavailable_operands':check.unavailable_paths,'unavailable_boundaries':check.unavailable_boundaries,'unavailable_counted_as_PASS':False,
        'observations':observations,'source_architecture':architecture,'selected_policy':selected_policy(),
        'executed_reader_source_archives':archives,'engines_executed_by_reader':False,'Cargo_executed_by_reader':False,
        'Git_executed_by_reader':False,'reference_outputs_supplied_to_Rust':False,'original_RHS_reconstructed':False,
        'physical_producer_numerical_parity_claimed':False,'native_thermal_callback_parity_claimed':False,'gates_updated':False}
    with (output/'production-comparison.json').open('x',encoding='utf8',newline='\n') as stream:
        stream.write(json.dumps(report,indent=2,allow_nan=False)+'\n')
    print(json.dumps(report,allow_nan=False))
    if check.mismatch_count:raise SystemExit(1)


if __name__=='__main__':main()
