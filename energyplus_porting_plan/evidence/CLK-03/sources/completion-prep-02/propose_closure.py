"""Write three ignored proposed documents only after real selected proofs pass.

Canonical plan/card/evidence are never modified here. The proposed packet is
not an actual card-closure receipt and is not a native retirement admission.
"""
import argparse
import sys
from pathlib import Path
from common import *
from proofs import actual_science
from quality import actual_quality
from document import proposed_documents

NAMES=['common.py','proofs.py','quality.py','document.py','propose_closure.py']

def writer_review(args,started):
    binding=ref(args.writer_source_review);review=read(binding['path']);receipt=recorded(args.writer_source_review_execution)
    require(review['schema']=='clk03-independent-completion-writer-source-review.v1'
        and review['status']=='pass-source-before-proposed-closure-metadata'
        and review['scientific_execution_performed'] is False,'Independent completion writer source review required')
    require(exact(read(receipt['stdout']['path']),review),'Actual writer source review stdout differs')
    require(timestamp(receipt['started_utc'])<=timestamp(review['review_started_utc'])
        <=timestamp(review['review_completed_utc'])<=timestamp(receipt['finished_utc'])<=timestamp(started),
        'Actual independent writer source review must precede proposal')
    rows=review['reviewed_sources'];require(type(rows) is list and len(rows)==len(NAMES),'Exact five writer sources required')
    keyed={row['historical_path']:row['archive'] for row in rows};require(len(keyed)==len(rows),'Duplicate writer source bindings')
    for name in NAMES:
        source=Path(__file__).parent/name;key=source.relative_to(ROOT).as_posix()
        require(key in keyed and bound(keyed[key])['sha256']==ref(source)['sha256'],'Reviewed writer source differs: '+name)
    return binding

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    for key in ['production-comparison','production-comparison-execution','production-result-review',
        'production-result-review-execution','matrix-execution','writer-source-review','writer-source-review-execution','output-dir']:
        parser.add_argument('--'+key,type=Path,required=True)
    args=parser.parse_args();require(Path.cwd().resolve()==ROOT,'Repository cwd required')
    output=path_of(args.output_dir)
    require(output.is_relative_to(ROOT/BASE) and not output.exists(),'Fresh contained ignored output required')
    require(not path_of(PLAN+'evidence/CLK-03.json').exists(),'Canonical closure evidence already exists')
    started=now();review_binding=writer_review(args,started)
    before=[ref(PLAN+'plan.json'),ref(PLAN+'cards/CLK-03.md')]
    historical=[ref(path) for path in sorted(path_of(PLAN+'evidence').rglob('*')) if path.is_file()]
    science=actual_science(args);quality=actual_quality()
    proposed,evidence_ref=proposed_documents(science,quality,historical,review_binding)
    require(set(proposed)=={PLAN+'evidence/CLK-03.json',PLAN+'plan.json',PLAN+'cards/CLK-03.md'},'Exact three proposed files required')
    unchanged();output.mkdir();after=[]
    for canonical,data in proposed.items():
        destination=output/Path(canonical).name
        with destination.open('xb') as stream:stream.write(data)
        after.append({'historical_path':canonical,'archive':ref(destination)})
    source_dir=output/'source';source_dir.mkdir();source_archives=[]
    for name in NAMES:
        source=Path(__file__).parent/name;destination=source_dir/name
        with destination.open('xb') as stream:stream.write(source.read_bytes())
        require(ref(source)['sha256']==ref(destination)['sha256'],'Writer executed source archive differs')
        source_archives.append({'historical_path':source.relative_to(ROOT).as_posix(),'archive':ref(destination)})
    unchanged()
    result={'schema':'clk03-proposed-card-closure.v1','status':'ready-for-reviewed-root-apply',
        'started_utc':started,'completed_utc':now(),'actual_writer_command':list(sys.orig_argv),'actual_writer_cwd':str(ROOT),
        'before':before,'previously_absent':[PLAN+'evidence/CLK-03.json'],'after':after,
        'proposed_evidence':evidence_ref,'only_CLK03_plan_task_changed':True,'proposed_four_CLK03_gates_pass':True,
        'canonical_files_written':False,'canonical_gates_updated':False,'actual_card_closure_claimed':False,
        'native_retirement_admitted':False,'unit_comparison':science['refs']['unit'],'production_comparison':science['refs']['production'],
        'independent_unit_result_review':science['refs']['unit_review'],'independent_production_result_review':science['refs']['production_review'],
        'final_quality':quality['refs'],'writer_source_review':review_binding,'writer_source_review_execution':ref(args.writer_source_review_execution),
        'preserved_historical_evidence':historical,'executed_writer_sources':source_archives,
        'prior_actual_failure_relabelled_as_PASS':False,'engines_Cargo_Git_or_scientific_comparers_executed':False}
    with (output/'proposal.json').open('xb') as stream:stream.write(encoded(result))
    print(json.dumps(result,ensure_ascii=False,allow_nan=False))

if __name__=='__main__':main()
