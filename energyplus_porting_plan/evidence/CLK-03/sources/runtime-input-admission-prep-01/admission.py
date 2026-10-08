"""Metadata-only admission of revision3 inputs to unchanged genuine build03."""
import argparse
import copy
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path

ROOT=Path(__file__).resolve().parents[4]
BUILD='.runtime/porting/CLK-03/native-build-03/native-driver-build.json'
ORIGINAL='.runtime/porting/CLK-03/original-helper-first-02/helper-reference.json'
MANIFEST='.runtime/porting/CLK-03/diagnostic-geometry-rules-amendment-01/amendment.json'
FREEZE='.runtime/porting/CLK-03/packet-amendment-prep-03/amendment-result.json'
FREEZE_EXECUTION='.runtime/porting/CLK-03/packet-amendment-command-03/receipt.json'
IDS=['DECOY-OFFSET-HOUR24','DECOY-OFFSET-HOUR1','FIRST-DAY-THREE-DAYS','MISSING-MATCH-DEFINED-FAILURE',
     'OUTSIDE-DATA-PERIOD','DIRECT-READDAY-REPLAY','DIRECT-READDAY-BACKSPACE-ONE-RECORD']
META={'created_utc','packet_revision','scientific_execution_scope','freeze_scope','amendment'}


def require(condition,message):
    if not condition:
        raise RuntimeError(message)


def path(name):
    result=(ROOT/name).resolve()
    require(result.is_relative_to(ROOT) and result!=ROOT,'Contained repository path required')
    return result


def ref(name):
    item=path(name)
    with item.open('rb') as stream:
        digest=hashlib.file_digest(stream,'sha256').hexdigest()
    return {'path':item.relative_to(ROOT).as_posix(),'sha256':digest}


def verify(binding):
    item=path(binding['path'])
    require(set(binding).issubset({'path','sha256','size_bytes','bytes'}) and item.is_file() and not item.is_symlink()
            and ref(binding['path'])['sha256']==binding['sha256'],'Actual bound byte identity differs')
    for key in ['size_bytes','bytes']:
        if key in binding:
            require(type(binding[key]) is int and item.stat().st_size==binding[key],'Actual bound byte size differs')
    return item


def read(name):
    return json.loads(path(name).read_text(encoding='utf-8-sig'))


def instant(value):
    stamp=datetime.fromisoformat(value.replace('Z','+00:00'))
    require(stamp.utcoffset() is not None and stamp.utcoffset().total_seconds()==0,'Aware UTC timestamp required')
    return stamp


def same(binding,other):
    require(exact(binding,other),'Historical/runtime binding identity differs')


def exact(left,right):
    # JSON scalar type is significant: Python False==0 must not authorize an input delta.
    return json.dumps(left,sort_keys=True,separators=(',',':'),allow_nan=False)==json.dumps(right,sort_keys=True,separators=(',',':'),allow_nan=False)


def packet():
    bindings={key:ref(f'energyplus_porting_plan/contracts/CLK-03-{key}.json') for key in ['source','cases','tolerances']}
    records={key:read(value['path']) for key,value in bindings.items()}
    for key,value in records.items():
        require(value['schema']==f'clk03-{key}-contract.v1' and value['packet_revision']==3
                and value['status']=='frozen-before-numerical-execution' and value['frozen_before_numerical_execution'] is True,
                'Actual revision3 frozen runtime packet required')
    cases=records['cases'];request=read(verify(cases['helper_request']).relative_to(ROOT).as_posix())
    require(request['schema']=='clk03-helper-cases.v1' and request['expected_values_supplied'] is False
            and request['expected_exits_supplied'] is False,'Input-only unchanged protocol required')
    counts={lane+'_sequences':len(request[lane+'_sequences']) for lane in ['handoff','weather']}
    counts.update({lane+'_operations':sum(len(row['operations']) for row in request[lane+'_sequences']) for lane in ['handoff','weather']})
    counts['total_sequences']=counts['handoff_sequences']+counts['weather_sequences']
    counts['total_operations']=counts['handoff_operations']+counts['weather_operations']
    require(all(type(value) is int for value in cases['counts'].values()) and exact(cases['counts'],counts),'Actual runtime count arrays differ')
    for item in [cases['fixed_CON_scope'],cases['input_byte_maps'],*cases['input_artifacts']]:
        verify(item)
    return {'contracts':bindings,'helper_request':cases['helper_request'],'counts':counts,
            'fixed_CON_scope':cases['fixed_CON_scope'],'input_byte_maps':cases['input_byte_maps']},records,request


def check_delta():
    build=read(BUILD);original=read(ORIGINAL);manifest=read(MANIFEST);freeze=read(FREEZE)
    require(ref(BUILD)['sha256']=='cdc90a5e5806c03e98355e7cd1593a0638ea1a5c7f801aa52ac7aad73c1a2700'
            and ref(ORIGINAL)['sha256']=='4eb4b7bf48fdfd24c6853f9e9220c222b9924bffc7bf06976bc5d554e2a1ea2b'
            and ref(MANIFEST)['sha256']=='7cda69fe4c6488a062c3dfee107e3c4c0ad5c40b985fc20ac3fc7900dd97c638',
            'Actual preserved build/original/input amendment identity differs')
    require(ref(FREEZE)['sha256']=='811fb2050030fb6cc31e79f8446882217ae2ea2a8b6f73a1667a1230f8e9bc2e'
            and ref(FREEZE_EXECUTION)['sha256']=='2be6b65f522c7b2e75918add78168110fb442de4aa296546340230e57a2345de',
            'Actual revision3 freeze and execution identity differs')
    freeze_execution=read(FREEZE_EXECUTION)
    require(freeze_execution['schema']=='recorded-porting-command.v1' and type(freeze_execution['exit_code']) is int
            and freeze_execution['exit_code']==0 and freeze_execution['launch_error'] is None,'Actual packet amendment writer failed')
    verify(freeze_execution['stdout']);verify(freeze_execution['stderr'])
    require(build['checks_passed'] is True and build['scientific_execution_performed'] is False
            and original['actual_helper_exit_code']==0 and original['preservation_checks_passed'] is True,
            'Actual compilation and prior original observations required')
    archives={row['historical_path']:row['archive'] for row in original['input_archives']}
    for item in archives.values():
        verify(item)
    old={key:read(archives[value['path']]['path']) for key,value in build['contracts'].items()}
    for key,value in build['contracts'].items():
        require(archives[value['path']]['sha256']==value['sha256'],'Actual historical compiled packet archive differs')
    require(archives[build['helper_request']['path']]['sha256']==build['helper_request']['sha256'],'Actual historical compiled request archive differs')
    old_request=read(archives[build['helper_request']['path']]['path'])
    current,records,request=packet()
    require(freeze['schema']=='clk03-input-packet-amendment-freeze.v1' and freeze['gates_changed'] is False
            and freeze['new_build_claimed'] is False and freeze['Rust_baseline_admitted'] is False,
            'Actual input-only freeze must not claim build or science admission')
    same(freeze['contracts'],current['contracts']);same(freeze['request'],current['helper_request'])
    replacements=manifest['seven_input_replacements']
    require([row['sequence_id'] for row in replacements]==IDS,'Only declared seven diagnostic inputs can change')
    restored=copy.deepcopy(request);changed={}
    for item in replacements:
        old_input,new_input=item['old_input'],item['new_input']
        # Old canonical inputs may be moved after admission. Historical actual bytes remain authoritative.
        verify(new_input);verify(item['old_input_archive']);verify(item['new_input_archive'])
        verify(item['actual_original_input_error'])
        require(archives[old_input['path']]['sha256']==old_input['sha256'],'Actual old input archive differs')
        before=verify(archives[old_input['path']]).read_bytes();after=verify(new_input).read_bytes()
        require(before==verify(item['old_input_archive']).read_bytes()
                and after==verify(item['new_input_archive']).read_bytes(),'Manifest/original actual archive bytes differ')
        added=manifest['required_property_provenance']['added_literal_IDF_object'].encode('utf-8')
        require(after.count(added)==1 and after.replace(added,b'',1)==before,'Exact one required-object input line must be the sole byte delta')
        row=next(row for row in restored['weather_sequences'] if row['id']==item['sequence_id'])
        same(row['input'],new_input);row['input']=old_input;changed[old_input['path']]=new_input
    require(exact(restored,old_request),'Operations/canaries/preparation/weather/caller input changed')
    science=lambda value:{key:item for key,item in value.items() if key not in META}
    require(exact(science(records['tolerances']),science(old['tolerances'])),'Tolerance policy changed')
    cases=copy.deepcopy(records['cases']);cases['helper_request']=build['helper_request']
    for index,item in enumerate(cases['input_artifacts']):
        if item in changed.values():
            name=next(name for name,new in changed.items() if new==item)
            cases['input_artifacts'][index]=next(old_item for old_item in old['cases']['input_artifacts'] if old_item['path']==name)
    require(exact(science(cases),science(old['cases'])),'Case scope/counts/maps/CON or non-IDF artifacts changed')
    source=records['source'];prior=old['source']
    idd=manifest['required_property_provenance']['pinned_IDD'];verify(idd)
    expected_file={'path':'idd/Energy+.idd.in','sha256':idd['sha256']}
    require(source['source_files']==prior['source_files']+[expected_file],'Pinned original scientific source file set changed')
    require(source['selected_ranges'][:-1]==prior['selected_ranges'] and source['selected_ranges'][-1]['file']=='idd/Energy+.idd.in'
            and source['selected_ranges'][-1]['start_line']==9392 and source['selected_ranges'][-1]['end_line']==9432,
            'Only required IDD declaration may extend source range selection')
    require(source['selected_ranges'][-1]['range_sha256']==manifest['required_property_provenance']['range_sha256'],
            'Actual pinned IDD declaration range hash differs')
    require(source['source_audits']==prior['source_audits']+[ref(MANIFEST)],'Source admission history differs')
    restored_source=copy.deepcopy(source)
    for key in ['source_files','selected_ranges','source_audits']:
        restored_source[key]=prior[key]
    require(exact(science(restored_source),science(prior)),'Scientific source/reader/handoff boundary changed')
    for row in build['executed_source_archives']:
        require(path(row['historical_path']).read_bytes()==verify(row['archive']).read_bytes(),'Actual compiled source bytes changed')
    return {'historical_build':ref(BUILD),'prior_original_execution':ref(ORIGINAL),'historical_packet_archives':[
                {'historical_path':name,'archive':archives[name]} for name in [*(item['path'] for item in build['contracts'].values()),build['helper_request']['path']]],
            'historical_input_archives':original['input_archives'],'runtime_packet':current,'input_amendment':ref(MANIFEST),
            'packet_freeze':ref(FREEZE),'packet_freeze_execution':ref(FREEZE_EXECUTION),
            'allowed_input_deltas':replacements,'actual_compiled_source_archives':build['executed_source_archives']}


def validate_admission(binding):
    admission=read(verify(binding).relative_to(ROOT).as_posix())
    require(admission['schema']=='clk03-runtime-input-admission.v1'
            and admission['status']=='admitted-input-only-runtime-packet-to-existing-genuine-build',
            'Separate actual post-build runtime input admission required')
    actual=check_delta()
    require(all(exact(admission[key],value) for key,value in actual.items()),'Actual runtime admission changed')
    require(admission['scientific_execution_performed'] is False and admission['new_native_build_claimed'] is False
            and admission['Rust_baseline_admitted'] is False,'Admission is not build or science success')
    verify(admission['reviewed_contract_review']);verify(admission['reviewed_source_review']);verify(admission['executed_metadata_utility'])
    require(path(admission['executed_metadata_utility']['path'])==Path(__file__).resolve(),'Wrong executed admission utility')
    review_bindings(admission['reviewed_contract_review'],admission['reviewed_source_review'],admission['admitted_utc'],actual)
    return admission


def review_bindings(contract_binding,source_binding,admitted_utc,actual):
    contract=read(verify(contract_binding).relative_to(ROOT).as_posix())
    review=read(verify(source_binding).relative_to(ROOT).as_posix())
    require(contract['schema']=='clk03-independent-contract-review.v1' and contract['status']=='pass-before-scientific-execution',
            'Current revision3 independent contract review required')
    expected={**actual['runtime_packet']['contracts'],'helper_request':actual['runtime_packet']['helper_request']}
    require(set(contract['reviewed_contracts'])==set(expected) and contract['scientific_execution_performed'] is False,
            'Exactly current four reviewed packet bindings required')
    for key,row in contract['reviewed_contracts'].items():
        same(ref(row['historical_path']),expected[key])
        require(path(row['historical_path']).read_bytes()==verify(row['archive']).read_bytes(),'Current reviewed packet archive differs')
    require(review['schema']=='clk03-independent-runtime-admission-source-review.v1'
            and review['status']=='pass-source-and-input-only-delta-before-runtime-admission'
            and review['scientific_execution_performed'] is False,'Distinct post-build admission source review required')
    same(review['reviewed_contract_review'],contract_binding)
    wanted={Path(__file__).resolve(),path('.runtime/porting/CLK-03/original-execution-prep-04/run_original.py')}
    require(len(review['reviewed_sources'])==len(wanted)
            and {path(row['historical_path']) for row in review['reviewed_sources']}==wanted,
            'Exactly current admission utility and launcher source review required')
    for row in review['reviewed_sources']:
        require(path(row['historical_path']).read_bytes()==verify(row['archive']).read_bytes(),'Reviewed admission/launcher source changed')
    require(any(path(row['historical_path'])==Path(__file__).resolve() for row in review['reviewed_sources']),'Reviewed metadata utility source required')
    require(instant(read(BUILD)['finished_utc'])<=instant(contract['review_completed_utc'])
            <=instant(review['review_completed_utc'])<=instant(admitted_utc)<=datetime.now(timezone.utc),
            'Actual compile/contract/source review must precede current post-build admission')


def main():
    parser=argparse.ArgumentParser()
    parser.add_argument('--contract-review',required=True)
    parser.add_argument('--source-review',required=True)
    parser.add_argument('--output-dir',required=True)
    args=parser.parse_args();out=path(args.output_dir)
    require(out.is_relative_to(ROOT/'.runtime/porting/CLK-03') and not out.exists(),'Fresh contained metadata admission output required')
    actual=check_delta();contract_binding=ref(args.contract_review);source_binding=ref(args.source_review)
    admitted_utc=datetime.now(timezone.utc).isoformat()
    review_bindings(contract_binding,source_binding,admitted_utc,actual)
    out.mkdir(parents=True)
    receipt=actual|{'schema':'clk03-runtime-input-admission.v1','status':'admitted-input-only-runtime-packet-to-existing-genuine-build',
        'admitted_utc':admitted_utc,'reviewed_contract_review':contract_binding,'reviewed_source_review':source_binding,
        'executed_metadata_utility':ref(Path(__file__).relative_to(ROOT).as_posix()),'scientific_execution_performed':False,
        'new_native_build_claimed':False,'Rust_baseline_admitted':False,'gates_updated':False,'engines_Cargo_Git_executed':False}
    (out/'admission.json').write_text(json.dumps(receipt,indent=2,allow_nan=False)+'\n',encoding='utf-8')
    print(json.dumps({'admission':ref((out/'admission.json').relative_to(ROOT).as_posix()),'scientific_execution_performed':False}))


if __name__=='__main__':
    main()
