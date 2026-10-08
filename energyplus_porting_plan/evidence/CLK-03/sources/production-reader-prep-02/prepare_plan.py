"""Freeze input-only production commands and selected observations; no engines."""
from __future__ import annotations

import argparse
from datetime import datetime, timezone
import json
from pathlib import Path
import shutil
import sys

sys.dont_write_bytecode=True
from clk03_production_provenance import ROOT, Bindings, packet, read, ref, require, timestamp
from check_clk03_production import source_bundle, source_review
from clk03_production_state import selected_policy


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    for key in ['source-review','output-dir']:parser.add_argument('--'+key,type=Path,required=True)
    args=parser.parse_args();bindings=Bindings();refs,_,cases,_=packet(bindings)
    review_ref=ref(args.source_review);reviewed=source_review(review_ref,refs,bindings)
    output=args.output_dir.resolve();require(output.is_relative_to(ROOT/'.runtime/porting/CLK-03') and not output.exists(),'Fresh contained plan directory required')
    scope=read(bindings.check(cases['fixed_CON_scope']));indexed={row['id']:row for row in scope['cases']}
    rows=[]
    for name in cases['production_case_ids']:
        original=indexed[name]
        rows.append({'case_id':name,**{key:original[key] for key in ['scope','duration','input','weather','metadata']},'trace_levels':['Full','Summary']})
    bindings.unchanged();output.mkdir();archives=[]
    for path in source_bundle():
        archived=output/path.name;shutil.copyfile(path,archived)
        require(path.read_bytes()==archived.read_bytes(),'Frozen source archive differs')
        archives.append({'historical_path':path.relative_to(ROOT).as_posix(),'archive':ref(archived)})
    created=datetime.now(timezone.utc).isoformat();require(timestamp(reviewed['review_completed_utc'])<=timestamp(created),'Source review must precede freeze')
    result={'schema':'clk03-production-plan.v1','created_utc':created,'contracts':refs,
        'fixed_scope':cases['fixed_CON_scope'],'input_byte_maps':cases['input_byte_maps'],'cases':rows,
        'actual_command_count':6,'expected_outputs_supplied':False,'physical_execution_performed':False,
        'gates_updated':False,'selected_policy':selected_policy(),'independent_sources_review':review_ref,
        'reader_source_archives':archives,'native_reference_reads_by_plan_writer':False,
        'hour1_production_matrix_added':False,'warmup_options_changed':False,
        'scope':'actual selected live cursor/wholeToday/owned A+B operands; generated processed weather physics unpaired'}
    with (output/'plan.json').open('x',encoding='utf8',newline='\n') as stream:stream.write(json.dumps(result,indent=2,allow_nan=False)+'\n')
    bindings.unchanged();print(json.dumps({'plan':ref(output/'plan.json'),'status':'input-and-observation-policy-declared-before-production','physical_execution_performed':False}))


if __name__=='__main__':main()
