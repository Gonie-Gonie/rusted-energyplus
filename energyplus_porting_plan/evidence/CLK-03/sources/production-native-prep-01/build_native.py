"""Build one supplemental whole-core caller dispatcher; never execute science.

The prior actual656 registration, immutable helpers, original-source guard,
compiler/publisher/derivation foundation and every retained artifact remain live.
Inputs are a separate independently reviewed matched caller contract, not a new
version of the already executed14/51 unit packet or a copied scientific body.
"""
from __future__ import annotations

import argparse
import copy
from datetime import datetime, timezone
import hashlib
import importlib.util
import json
from pathlib import Path
import shutil
import sys
import traceback

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[4]
HERE = Path(__file__).resolve().parent
RAW = ROOT / '.runtime/porting/CLK-03'
TARGET = 'clk03_production_reference_helper'
PRIOR = ROOT / '.runtime/porting/CLK-03/native-build-03/native-driver-build.json'
PRIOR_SHA = 'cdc90a5e5806c03e98355e7cd1593a0638ea1a5c7f801aa52ac7aad73c1a2700'
OLD_REVIEW = {'path':'.runtime/porting/CLK-03/independent-native-build-review-03/review.json',
              'sha256':'771f09567e5d1b3c91fcbd54f9519f0cb415a63db8560818576f62277162b01a'}
OLD_UTILITY = ROOT / 'tools/porting/clk03_reference_native.py'
OLD_UTILITY_SHA = '49ce5ed4dde11a45210556a802fa15bd5e707ef0e34fc821edf5a454a9fc0e4e'
OWN = [HERE/(TARGET+'.cpp'), HERE/'clk03_production_reference.cmake', Path(__file__).resolve()]
INCLUDED = [ROOT/'tools/porting'/name for name in
            ['clk03_reference_helper.cpp','clk03_reference_fields.hh','clk02_reference_fields.hh']]


def require(ok, message):
    if not ok:
        raise ValueError(message)


def path(value):
    item = Path(value)
    item = (item if item.is_absolute() else ROOT/item).resolve()
    require(item.is_relative_to(ROOT) and item != ROOT, 'Artifact escapes repository')
    return item


def sha(value):
    with Path(value).open('rb') as stream:
        return hashlib.file_digest(stream,'sha256').hexdigest()


def read(value):
    def pairs(items):
        result={}
        for key,item in items:
            require(key not in result,'Duplicate JSON key')
            result[key]=item
        return result
    return json.loads(Path(value).read_text(encoding='utf8'), object_pairs_hook=pairs,
                      parse_constant=lambda value: (_ for _ in ()).throw(ValueError('Nonstandard JSON '+value)))


def verify(binding):
    item=path(binding['path'])
    require(item.is_file() and not item.is_symlink() and sha(item)==binding['sha256'], 'Bound bytes differ: '+str(item))
    for key in ['size_bytes','bytes']:
        if key in binding:
            require(type(binding[key]) is int and item.stat().st_size==binding[key], 'Bound size differs')
    return item


def ref(value):
    item=path(value)
    return {'path':item.relative_to(ROOT).as_posix(),'sha256':sha(item)}


def write(value, data):
    with Path(value).open('x',encoding='utf8',newline='\n') as stream:
        stream.write(json.dumps(data,indent=2,allow_nan=False)+'\n')


def utc():
    return datetime.now(timezone.utc).isoformat()


def timestamp(value):
    item=datetime.fromisoformat(value)
    require(item.tzinfo is not None and item.utcoffset().total_seconds()==0,'Aware UTC required')
    return item


def prior_sources(record):
    rows=record['executed_source_archives']
    require(len(rows)==27 and len({x['historical_path'] for x in rows})==27,'Actual prior source package differs')
    for row in rows:
        require(path(row['historical_path']).read_bytes()==verify(row['archive']).read_bytes(), 'Prior source/CMake bytes changed')
    return rows


# Authenticate all importable inherited utility bytes before executing imports.
require(sha(PRIOR)==PRIOR_SHA,'Actual native03 build receipt changed')
prior=read(PRIOR)
prior_sources(prior)
require(sha(OLD_UTILITY)==OLD_UTILITY_SHA,'Inherited genuine-core guard changed')
spec=importlib.util.spec_from_file_location('matched_clk03_guard',OLD_UTILITY)
require(spec is not None and spec.loader is not None,'Pinned inherited utility unavailable')
old=importlib.util.module_from_spec(spec)
spec.loader.exec_module(old)
util=old.util
BUILD,SOURCE,CORE,PIN=util.BUILD,util.SOURCE,util.CORE,util.PIN


def prior_identity():
    require(prior['schema']=='clk03-native-driver-build.v1' and prior['checks_passed'] is True
            and prior['energyplus_commit']==PIN and prior['scientific_source_patches'] is False,
            'Actual unchanged prior whole-core build required')
    require(verify(prior['core_build'])==CORE,'Prior genuine core identity differs')
    rows=read(verify(prior['compile_commands']))
    require(len(rows)==656,'Exactly prior656 compiled registration rows required')
    _,older_rows=old.prior_identity()
    require([x for x in rows if Path(x['file']).name!='clk03_reference_helper.cpp']==older_rows
            and len(older_rows)==655,'Old655 registrations are not unchanged subset')
    peer=read(verify(OLD_REVIEW))
    require(peer['schema']=='clk03-independent-native-build-review.v1'
            and peer['status']=='pass-build-and-derivative-before-original-execution'
            and peer['reviewed_build']==ref(PRIOR),'Actual independent native03 review required')
    verify(peer['reviewed_failed_original_relocation'])
    for key in ['retained_failed_original02_independently_hashed','retained_failed_derivative02_independently_hashed']:
        verify(peer[key])
    linked=prior['linked_original']
    verify({'path':linked['historical_path'],'sha256':linked['sha256'],'size_bytes':linked['size_bytes']})
    verify(prior['helper_binary'])
    return rows,peer


def packet(value):
    contract_ref=ref(value)
    contract=read(verify(contract_ref))
    require(contract['schema']=='clk03-matched-production-native-contract.v1'
            and contract['status']=='frozen-input-only-matched-caller-before-supplemental-original-execution'
            and contract['source_commit']==PIN,'Separate frozen matched production contract required')
    for key in ['expected_values_supplied','expected_exits_supplied','scientific_execution_performed','gates_updated',
                'old_R3_contracts_rewritten','source_RHS_copied_or_native_results_as_inputs','production_matrix_expanded']:
        require(contract[key] is False,'Input-only matched contract boundary differs: '+key)
    require(contract['production_case_ids']==['A-24H','A-72H','B-BOTH-24H'],'Fixed three-CON case order required')
    request=read(verify(contract['request']))
    require(request['schema']=='clk03-production-helper-cases.v1' and request['source_commit']==PIN
            and request['expected_values_supplied'] is False and request['expected_exits_supplied'] is False
            and request['scientific_execution_performed'] is False,'Separate literal caller request required')
    require(request['contracts']==contract['contracts'] and request['scope']==contract['fixed_scope'], 'Supplemental runtime packet differs')
    require(request['native_preparation']==contract['native_preparation'],'Declared whole preparation differs')
    for binding in contract['contracts'].values(): verify(binding)
    verify(contract['fixed_scope']);verify(contract['input_byte_maps']);verify(contract['independent_input_source_review'])
    inputs=contract['literal_CON_input_references']
    require(len(inputs)==45,'All45 literal CON input bindings required')
    for binding in inputs: verify(binding)
    require(len(request['cases'])==3 and [x['id'] for x in request['cases']]==contract['production_case_ids'],'Matched input case cardinality/order differs')
    actual={'weather_sequences':len(request['cases']),'GetNextEnvironment':0,'InitializeWeather':0,'total_operations':0}
    for case in request['cases']:
        require(case['time_steps_per_hour']==4 and type(case['time_steps_per_hour']) is int
                and case['prepared_environment_lane'] is True and case['native_preparation']==contract['native_preparation'],
                'Matched selected four-step prepared lane differs')
        for key in ['input','weather']:
            require(case[key] in inputs,'Matched input outside literal fixed CON scope');verify(case[key])
        for index,item in enumerate(case['operations']):
            kind=item['kind']
            require(kind==('GetNextEnvironment' if index==0 else 'InitializeWeather') and type(item['caller']) is dict,
                    'Initial selection followed only by whole source InitializeWeather calls required')
            actual[kind]+=1;actual['total_operations']+=1
    require(actual==contract['requested_counts']==request['requested_counts']
            and all(type(x) is int and x>=0 for x in contract['requested_counts'].values()),'Actual declared matched call counts differ')
    projection=read(verify(contract['input_projection']))
    require(projection['matched_request']==contract['request'] and projection['contracts']==contract['contracts']
            and projection['Rust_trace_or_native_result_deserialized_as_input'] is False
            and projection['old_contract_plan_report_or_policy_rewritten'] is False
            and projection['prior_actual_mismatch_relabelled_unavailable_or_PASS'] is False,
            'Supplemental caller inputs changed old actual proofs or reused scientific outputs')
    for row in contract['source_files']:
        source=(SOURCE/row['path']).resolve()
        require(source.is_relative_to(SOURCE) and sha(source)==row['sha256'],'Pinned scientific source differs')
    for row in contract['caller_source_archives']:
        require(path(row['historical_path']).read_bytes()==verify(row['archive']).read_bytes(),'Caller source changed after input freeze')
    return {'matched_contract':contract_ref,'matched_request':contract['request'], 'input_projection':contract['input_projection'],
            'contracts':contract['contracts'],'fixed_scope':contract['fixed_scope'],
            'requested_counts':actual,'caller_source_archives':contract['caller_source_archives']}


def reviews(args,bound):
    contract_ref,helper_ref=ref(args.contract_review),ref(args.helper_static_review)
    contract,helper=read(verify(contract_ref)),read(verify(helper_ref))
    require(contract['schema']=='clk03-independent-matched-production-contract-review.v1'
            and contract['status']=='pass-input-only-matched-caller-contract-before-original-execution'
            and contract['scientific_execution_performed'] is False,'Independent supplemental contract review required')
    require(contract['reviewed_contract']==bound['matched_contract'] and contract['reviewed_request']==bound['matched_request']
            and contract['reviewed_projection']==bound['input_projection'],'Reviewed matched packet identity differs')
    require(helper['schema']=='clk03-independent-matched-production-helper-static-review.v1'
            and helper['status']=='pass-source-and-contract-before-native-build'
            and helper['scientific_execution_performed'] is False
            and helper['reviewed_contract_review']==contract_ref,'Independent supplemental source/contract review required')
    wanted={x.relative_to(ROOT).as_posix() for x in [*OWN,*INCLUDED,OLD_UTILITY]}
    rows={x['historical_path']:x['archive'] for x in helper['reviewed_sources']}
    allowed={x['historical_path'] for x in prior_sources(prior)}|wanted
    require(len(rows)==len(helper['reviewed_sources']) and wanted<=set(rows)<=allowed,'All new and reused helper/utility dependencies must be reviewed')
    for owner,archive in rows.items():
        require(path(owner).read_bytes()==verify(archive).read_bytes(),'Independently reviewed source bytes differ')
    for value in [contract,helper]: timestamp(value['review_completed_utc'])
    return {'independent_contract_review':contract_ref,'independent_helper_static_review':helper_ref}


def archive_sources(directory):
    destination=directory/'source';destination.mkdir()
    sources={path(x['historical_path']) for x in prior_sources(prior)}|set(OWN)
    rows=[]
    for source in sorted(sources):
        archive=destination/source.name
        require(not archive.exists(),'Source archive basename collision')
        shutil.copyfile(source,archive)
        require(sha(source)==sha(archive),'Source changed while archiving')
        rows.append({'historical_path':source.relative_to(ROOT).as_posix(),'archive':ref(archive)})
    return rows


def preservation(baseline,frozen,core,source_before,sources,registry_path):
    _,registry,_=old.registry(registry_path)
    for key in ['binary','toolchain_publisher_receipt','object_cache_publisher_receipt']: verify(core['compiler'][key])
    current=read(BUILD/'compile_commands.json')
    require([x for x in current if Path(x['file']).name!=TARGET+'.cpp']==baseline,'Historical656 compile rows changed')
    require(len(frozen)==643 and all(x in current for x in frozen),'Frozen643 core compile rows changed')
    guard=util.original_source_guard(core)
    require(guard==source_before,'Original scientific source/publisher archive guard differs')
    for row in sources:
        require(path(row['historical_path']).read_bytes()==verify(row['archive']).read_bytes(),'Archived build source differs')
    _,peer=prior_identity()
    return {'observed_utc':utc(),'protected_legacy_and_library_count':14,
            'retained_CLK02_derivative_and_original_absence_verified':True,
            'prior_successful_CLK03_original_and_derivative_unchanged':True,
            'failed_CLK03_original02_and_derivative02_unchanged':True,
            'historical_compile_rows_checked':len(baseline),'frozen_core_rows_checked':len(frozen),
            'source_guard':guard,'archived_build_sources_unchanged':True,
            'old_CLK03_PE_review':OLD_REVIEW,'retained_registry':ref(registry_path)}


def build_driver(args):
    directory=path(args.output_dir)
    require(directory.is_relative_to(RAW) and directory!=RAW and not directory.exists(),'Fresh ignored matched-build output required')
    directory.mkdir(parents=True)
    receipt={'schema':'clk03-matched-production-native-build.v1','started_utc':utc(),'actual_argv':list(sys.orig_argv),
             'actual_cwd':str(Path.cwd().resolve()),'source_commit':PIN,'checks_passed':False,'preservation_observations':{},
             'scientific_execution_performed':False,'scientific_source_patches':False,'Cargo_or_Git_executed':False,
             'gates_updated':False,'original_removed':False,'full_linked_original_copied':False,
             'existing_unit_packet_or_actual_receipts_rewritten':False,'target':TARGET,
             'independent_actual_build_PE_review_required_before_original_execution':True}
    sources=baseline=frozen=core=source_before=registry_path=None
    bound=reviewed=foundation=None
    try:
        bound=packet(args.contract);receipt.update(bound)
        reviewed=reviews(args,bound);receipt.update(reviewed)
        registry_path,registry,legacy_path=old.registry(args.registry)
        baseline,_=prior_identity()
        foundation_args=copy.copy(args);foundation_args.registry=str(legacy_path)
        foundation=old.old.utility_identity(foundation_args)
        for binding in [*reviewed.values(),*(foundation[k] for k in ['independent_review','independent_actual_review','independent_runtime_review'])]:
            require(timestamp(read(verify(binding))['review_completed_utc'])<=timestamp(receipt['started_utc']),'Review must precede new native build')
        core,frozen=util.verified_core(),read(util.FROZEN_CORE_ROWS)
        source_before=util.original_source_guard(core)
        require(not (BUILD/'Products'/(TARGET+'.exe')).exists(),'New supplemental target must not overwrite any original')
        sources=archive_sources(directory)
        receipt.update(core_build=ref(CORE),prior_native_build=ref(PRIOR),prior_actual_build_review=OLD_REVIEW,
                       retained_registry=ref(registry_path),derivative_utility=foundation,
                       executed_source_archives=sources,previous_compile_row_count=len(baseline),
                       frozen_core_compile_row_count=len(frozen),frozen_original_build_commands=ref(util.FROZEN_CORE_ROWS))
        observe=lambda:preservation(baseline,frozen,core,source_before,sources,registry_path)
        receipt['preservation_observations']['before_configure']=observe()
        write(directory/'previous-compile-commands.json',baseline)
        receipt['previous_compile_commands']=ref(directory/'previous-compile-commands.json')
        configured_before=read(verify(prior['configure']))
        require(type(configured_before['exit_code']) is int and configured_before['exit_code']==0
                and sum(x.startswith('-DCMAKE_PROJECT_INCLUDE=') for x in configured_before['command'])==1,
                'Actual prior configure argument/source identity differs')
        command=['-DCMAKE_PROJECT_INCLUDE='+str(HERE/'clk03_production_reference.cmake')
                 if x.startswith('-DCMAKE_PROJECT_INCLUDE=') else x for x in configured_before['command']]
        configured=util.process(command,directory,'configure');receipt['configure']=ref(directory/'configure-command.json')
        require(configured['exit_code']==0,'Actual configure failed; logs preserved')
        receipt['preservation_observations']['after_configure']=observe()
        rows=read(BUILD/'compile_commands.json');own=[x for x in rows if Path(x['file']).name==TARGET+'.cpp']
        require(len(rows)==len(baseline)+1 and len(own)==1 and path(own[0]['file'])==HERE/(TARGET+'.cpp'),
                'Only one fresh supplemental compile unit may be added')
        for flag in ['-DEP_psych_errors','-UNDEBUG','-Werror','-O0','-ffp-contract=off','-Wa,-mbig-obj','-std=c++20']:
            require(flag in own[0]['command'].split(),'Missing unchanged original compiler flag '+flag)
        for flag in ['_GLIBCXX_DEBUG','-ffast-math','EP_psych_stats']: require(flag not in own[0]['command'],'Incorrect native ABI/mode')
        require(own[0]['command'].startswith(str(verify(core['compiler']['binary']))),'Wrong compiler executable')
        write(directory/'actual-compile-commands.json',own);receipt['actual_compile_commands']=ref(directory/'actual-compile-commands.json')
        compiled=util.process([command[0],'--build',str(BUILD),'--target',TARGET,'--parallel','1','--verbose'],directory,'build')
        receipt['compile_link']=ref(directory/'build-command.json')
        require(compiled['exit_code']==0,'Targeted compile/link failed; actual logs preserved')
        receipt['preservation_observations']['after_compile_link']=observe()
        live=BUILD/'Products'/(TARGET+'.exe')
        linked={'historical_path':live.relative_to(ROOT).as_posix(),'sha256':sha(live),'size_bytes':live.stat().st_size}
        receipt['linked_original']=linked
        for name in ['CMakeCache.txt','compile_commands.json']: shutil.copyfile(BUILD/name,directory/name)
        receipt.update(cache=ref(directory/'CMakeCache.txt'),compile_commands=ref(directory/'compile_commands.json'))
        derive=[sys.executable,'-X','utf8','-B',str(ROOT/'tools/porting/derive_native_helper.py'),'--original',str(live),
                '--original-sha256',linked['sha256'],'--strip-tool',str(path(args.strip_tool)),'--tool-audit',str(path(args.tool_audit)),
                '--help-receipt',str(path(args.help_receipt)),'--output-dir',str(directory/'derivation')]
        derived=util.process(derive,directory,'derive');receipt['derivation_command']=ref(directory/'derive-command.json')
        if (directory/'derivation/receipt.json').exists(): receipt['validated_derivative']=ref(directory/'derivation/receipt.json')
        require(derived['exit_code']==0,'Actual authenticated derivation failed; receipt/log/maps preserved')
        receipt['helper_binary']=old.old.validate_derivative(read(directory/'derivation/receipt.json'),linked)
        receipt['preservation_observations']['after_derivation']=observe()
        receipt.update(checks_passed=True,existing_target_compile_commands_unchanged=True,
                       original_core_and_API_bytes_unchanged=True,same_compiler_owned_state=True,
                       original_directory_and_target_definitions_inherited=True,assertions_enabled=True,fp_contract='off')
    except Exception as error:
        receipt['failure']={'type':type(error).__name__,'message':str(error)}
        (directory/'failure.log').write_text(traceback.format_exc(),encoding='utf8',newline='\n')
        receipt['failure_log']=ref(directory/'failure.log')
    finally:
        if sources is not None:
            try:
                receipt['preservation_observations']['final']=preservation(baseline,frozen,core,source_before,sources,registry_path)
                packet(args.contract)
                for binding in [*reviewed.values(),*foundation.values()]: verify(binding)
                if 'linked_original' in receipt:
                    linked=receipt['linked_original'];live=path(linked['historical_path'])
                    require(sha(live)==linked['sha256'] and live.stat().st_size==linked['size_bytes'],'Supplemental new original changed')
                if 'helper_binary' in receipt: verify(receipt['helper_binary'])
            except Exception as error:
                receipt['checks_passed']=False
                receipt['final_preservation_failure']={'type':type(error).__name__,'message':str(error)}
        receipt['finished_utc']=utc();write(directory/'native-driver-build.json',receipt)
    return {'driver_build':ref(directory/'native-driver-build.json'),'checks_passed':receipt['checks_passed'],
            'helper_binary':receipt.get('helper_binary')}


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    mode=parser.add_mutually_exclusive_group(required=True)
    mode.add_argument('--check',action='store_true');mode.add_argument('--build-driver',action='store_true')
    names=['contract','contract-review','helper-static-review','registry','output-dir','derivation-review','derivation-validation',
           'derivation-actual-review','derivation-runtime-validation','derivation-runtime-review','strip-tool','tool-audit','help-receipt']
    for name in names: parser.add_argument('--'+name)
    args=parser.parse_args()
    require(Path.cwd().resolve()==ROOT,'Repository cwd required')
    for name in names:
        if args.build_driver or name in ['contract','contract-review','helper-static-review','registry']:
            require(getattr(args,name.replace('-','_')),'Requires --'+name)
    if args.check:
        bound=packet(args.contract);reviewed=reviews(args,bound);old.registry(args.registry);prior_identity()
        print(json.dumps({**bound,**reviewed,'native_actions_performed':False}));return 0
    result=build_driver(args);print(json.dumps(result));return 0 if result['checks_passed'] else 1


if __name__=='__main__':
    raise SystemExit(main())
