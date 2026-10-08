"""Run six fixed ordinary CON inputs with an archived committed CLI binary."""
from __future__ import annotations

import argparse
import json
from pathlib import Path
import shutil
import subprocess
import sys

sys.dont_write_bytecode=True
from clk03_production_provenance import (ROOT, Bindings, evidence, exact, packet, plan, read, ref,
    recorded_json, require, same_binding, timestamp, verify_rust_build)
from check_clk03_production import frozen_sources, source_review
from clk03_production_state import selected_policy


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    for key in ['build','unit-comparison','unit-comparison-execution','plan','source-review','output-dir']:
        parser.add_argument('--'+key,type=Path,required=True)
    args=parser.parse_args();bindings=Bindings();refs,_,cases,_=packet(bindings)
    plan_ref,declared=plan(args.plan,refs,cases,selected_policy(),bindings)
    review_ref=ref(args.source_review);reviewed=source_review(review_ref,refs,bindings)
    require(same_binding(review_ref,declared['independent_sources_review']),'Declared source review differs')
    require(timestamp(reviewed['review_completed_utc'])<=timestamp(declared['created_utc']), 'Independent source review must precede plan freeze')
    frozen_sources(declared,bindings)
    build_ref,unit_ref,unit_execution_ref=[ref(path) for path in [args.build,args.unit_comparison,args.unit_comparison_execution]]
    unit,_,_,build,_,prerequisites_finished=evidence({'build':build_ref,'unit_comparison':unit_ref,
        'unit_comparison_execution':unit_execution_ref},refs,cases,bindings)
    output=args.output_dir.resolve();require(output.is_relative_to(ROOT/'.runtime/porting/CLK-03') and not output.exists(),'Fresh contained output required')
    output.mkdir();archive=output/'launcher.py';shutil.copyfile(Path(__file__),archive)
    require(archive.read_bytes()==Path(__file__).read_bytes(),'Executed launcher archive differs')
    recorder=ROOT/'tools/porting/record_command.py';recorder_ref=ref(recorder);bindings.check(recorder_ref)
    matrix={'schema':'clk03-production-matrix.v1','build':build_ref,'unit_comparison':unit_ref,
        'unit_comparison_execution':unit_execution_ref,'declared_plan':plan_ref,'contracts':refs,
        'independent_sources_review':review_ref,'executed_launcher':ref(archive),
        'implementation_commit':build['implementation_commit'],'crates_tree':build['crates_tree'],
        'cases':[],'actual_command_count':0,'complete':False,'original_outputs_supplied_to_Rust':False,'gates_updated':False}
    matrix_path=output/'matrix.json'
    def update():matrix_path.write_text(json.dumps(matrix,indent=2,allow_nan=False)+'\n',encoding='utf8',newline='\n')
    update()
    for case in declared['cases']:
        row={key:case[key] for key in ['case_id','scope','duration','input','weather','metadata']};matrix['cases'].append(row)
        directory=output/case['case_id'];directory.mkdir()
        input_path,weather_path=[bindings.check(case[key]) for key in ['input','weather']]
        for level in case['trace_levels']:
            destination,receipt_dir=directory/level.lower(),directory/(level.lower()+'-command')
            command=[str(bindings.check(build['binary'])),'run',str(input_path),'--weather',str(weather_path),
                '--output-dir',str(destination),'--mode','compatibility','--partial','deny','--trace-level',level.lower(),
                '--porting-scope',case['scope'].lower()]
            bindings.unchanged()
            completed=subprocess.run([sys.executable,'-X','utf8','-B',str(recorder),'--output-dir',str(receipt_dir),'--',*command],cwd=ROOT,check=False)
            receipt_ref=ref(receipt_dir/'receipt.json');receipt=read(bindings.check(receipt_ref))
            row[level]={'command_receipt':receipt_ref,'output_directory':destination.relative_to(ROOT).as_posix(),
                        'artifacts':[ref(path) for path in sorted(destination.rglob('*')) if path.is_file()]}
            matrix['actual_command_count']+=1;update()
            require(completed.returncode==receipt['exit_code']==0 and receipt['launch_error'] is None
                    and exact(receipt['command'],command),'Actual CLI failed; partial matrix/receipts retained')
            require(receipt['source_bytes_match_before_and_after'] is True and same_binding(receipt['source_snapshot'],build['source_snapshot'])
                    and receipt['repository_before']['head']==receipt['repository_after']['head']==build['implementation_commit']
                    and receipt['repository_before']['worktree_clean'] is True and receipt['repository_after']['worktree_clean'] is True,
                    'Actual CLI committed build/source guard differs')
            require(receipt['executed_launcher']['archive']['sha256']==recorder_ref['sha256']==build['actual_executed_recorder']['archive']['sha256'],
                    'Actual CLI recorder identity differs')
            require(max(prerequisites_finished,timestamp(declared['created_utc']),timestamp(reviewed['review_completed_utc']))<=timestamp(receipt['started_utc']),
                    'Actual CLI preceded unit/build/frozen plan/independent source review')
            summary=read(destination/'run-summary.json')
            require(summary['status']=='success' and summary['exit_code']==0 and summary['config']['dry_run'] is False
                    and summary['config']['oracle_baseline'] is False and summary['config']['compare_oracle'] is False
                    and summary['oracle'] is None,'Actual ordinary input-only run required')
            bindings.unchanged();print(case['case_id']+' '+level+' actual_exit=0',flush=True)
    matrix['complete']=matrix['actual_command_count']==6;update();bindings.unchanged();print(json.dumps(ref(matrix_path)))


if __name__=='__main__':main()
