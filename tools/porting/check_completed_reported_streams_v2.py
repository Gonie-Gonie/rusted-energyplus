"""Generic, manifest-bound engineering comparison of completed physical streams.

No engine is launched. The historical whole-stream parser and arithmetic remain
an authenticated dependency, not a copied implementation or a conformance gate.
"""
from pathlib import Path
from datetime import datetime
import argparse
import csv
import hashlib
import importlib.util
import json
import math
import re
import sys

ROOT = Path(__file__).resolve().parents[2]
KERNEL = dict(path='tools/porting/check_completed_reported_streams.py',
              sha256='f5669cd0217a3f953e08f2c96fd09fcf9067be85f6a9257b3dc1011e9297f833',bytes=24871)
PATHS = dict(path='tools/porting/validation_paths.py',
             sha256='c9f4611707a4082d04688deb1db703f057283932948639eb22fbae6bde4e64fa',bytes=1336)
RECORDER = dict(path='tools/porting/record_command.py',
                sha256='3dbaf8f78af809a6c11e195800c9a04d5ef961ff6ff49d567fc05cd4c4ee0c32',bytes=11614)
MANIFEST_KEYS = {'schema','id','input','weather','scope','clock','policy','Native','Source','artifacts'}
STREAM_KEYS = {'eso','mtr','rdd','mdd'}


def require(value, message):
    if not value:
        raise ValueError(message)


def exact(value, keys, label):
    require(type(value) is dict and set(value) == set(keys), 'Unexpected '+label+' keys')


def typed(value):
    if type(value) is float:
        require(math.isfinite(value),'Nonfinite metadata')
        return ('float',value.hex())
    if type(value) is dict:
        return ('object',tuple((key,typed(value[key])) for key in sorted(value)))
    if type(value) is list:
        return ('array',tuple(typed(item) for item in value))
    return (type(value).__name__,value)


def resolved(name, cwd=ROOT):
    require(type(name) is str and name,'Missing path')
    path = Path(name)
    return (path if path.is_absolute() else cwd/path).resolve()


def identity(path):
    path = Path(path).resolve()
    with path.open('rb') as stream:
        sha = hashlib.file_digest(stream,'sha256').hexdigest()
    name = path.relative_to(ROOT).as_posix() if path.is_relative_to(ROOT) else path.as_posix()
    return dict(path=name,sha256=sha,bytes=path.stat().st_size)


def auth(pin, optional_size=False):
    require(type(pin) is dict and set(pin) in ([{'path','sha256'},{'path','sha256','bytes'}] if optional_size else [{'path','sha256','bytes'}]),'Malformed file reference')
    name = pin['path']
    require(type(name) is str and name and '\\' not in name,'Noncanonical reference path')
    segments = name.split('/')[1:] if name.startswith('/') else name.split('/')
    require(not name.startswith('//') and all(part and part not in ['.','..'] for part in segments),'Noncanonical reference segments')
    require(Path(name).is_absolute() or ':' not in name,'Drive-relative references are unsupported')
    require(type(pin['sha256']) is str and re.fullmatch('[0-9a-f]{64}',pin['sha256']) is not None,'Invalid reference hash')
    if 'bytes' in pin:
        require(type(pin['bytes']) is int and pin['bytes'] >= 0,'Invalid reference byte count')
    path = resolved(name)
    actual = identity(path)
    require(actual['path'] == name and all(actual[key] == value for key,value in pin.items()),'Changed referenced file: '+name)
    return path


def load(pin):
    return json.loads(auth(pin).read_bytes())


def token(value):
    parts = value.split('#')
    require(len(parts) == 3 and re.fullmatch('0|[1-9][0-9]*',parts[2]) is not None,'Expected path#sha256#bytes')
    result = dict(path=parts[0],sha256=parts[1],bytes=int(parts[2])); auth(result)
    return result


def kernel():
    home = auth(KERNEL).parent; auth(PATHS)
    require('validation_paths' not in sys.modules,'Use a fresh standalone comparer process')
    previous = sys.path[:]
    try:
        sys.path.insert(0,str(home))
        spec = importlib.util.spec_from_file_location('_frozen_reported_stream_kernel_f566',home/'check_completed_reported_streams.py')
        require(spec is not None and spec.loader is not None,'Cannot load frozen parser')
        module = importlib.util.module_from_spec(spec); spec.loader.exec_module(module)
        dependency = sys.modules['validation_paths']
        require(Path(dependency.__file__).resolve() == auth(PATHS),'Unexpected path dependency')
        auth(KERNEL)
        return module
    finally:
        sys.path[:] = previous


def options(command, allowed):
    require(type(command) is list and command and all(type(part) is str for part in command),'Invalid command')
    result, operands, index = {}, [], 1
    while index < len(command):
        part = command[index]
        if part.startswith('-'):
            require(part in allowed and part not in result and index+1 < len(command) and not command[index+1].startswith('--'),'Unsupported/duplicate command option: '+part)
            result[part] = command[index+1]; index += 2
        else:
            operands.append(part); index += 1
    return result, operands


def successful(pin):
    row = load(pin)
    require(row['schema'] == 'recorded-porting-command.v1' and type(row['exit_code']) is int and row['exit_code'] == 0 and row['launch_error'] is None,'Successful actual recorded command required')
    require(row['source_bytes_match_before_and_after'] is True and row['repository_before'] == row['repository_after'],'Unstable command workspace')
    require(resolved(row['cwd']) == ROOT,'Receipt belongs to another checkout')
    auth(row['source_snapshot'],True)
    launcher = row['executed_launcher']
    require(launcher['historical_path'] == 'tools/porting/record_command.py','Unexpected recorded launcher')
    archive = identity(auth(launcher['archive'],True)); auth(RECORDER)
    require(all(archive[key] == RECORDER[key] for key in ['sha256','bytes']),'Recorded launcher bytes differ')
    for key in ['stdout','stderr']:
        auth(row[key],True)
    start,finish = [datetime.fromisoformat(row[key]) for key in ['started_utc','finished_utc']]
    require(start.tzinfo is not None and finish.tzinfo is not None and finish >= start,'Invalid command chronology')
    require(type(row['elapsed_seconds']) in [int,float] and math.isfinite(row['elapsed_seconds']) and row['elapsed_seconds'] >= 0,'Invalid elapsed time')
    return row


def custody(row, required):
    value = row['explicit_input_custody']
    require(value['command_process_started'] is True and value['all_explicit_inputs_unchanged'] is True and value['read_errors_before'] == value['read_errors_after'] == [],'Actual explicit-input custody required')
    require(typed(value['before']) == typed(value['after']),'Explicit inputs changed')
    request = load(value['request'])
    require(request['schema'] == 'recorded-command-explicit-input-custody-request.v1' and request['command'] == row['command'] and resolved(request['cwd']) == ROOT,'Stored request differs')
    require(request['declared_input_arguments'] == value['declared_input_arguments'] and typed(request['inputs_before']) == typed(value['before']) and request['input_read_errors_before'] == [],'Stored input list differs')
    observed = {}
    for pin in value['before']:
        path = auth(pin)
        require(path not in observed,'Duplicate explicit input')
        observed[path] = pin
    require(len(observed) == len(value['declared_input_arguments']) and
            [resolved(name) for name in value['declared_input_arguments']] == [auth(pin) for pin in value['before']],'Explicit argument order differs')
    for pin in required:
        require(auth(pin) in observed and typed(observed[auth(pin)]) == typed(pin),'Missing exact recorded input: '+pin['path'])


def policy(pin):
    value = load(pin); exact(value,{'schema','variables','meters'},'named policy')
    require(value['schema'] == 'reported-stream-policy.v1','Unsupported explicit policy')
    for group,meter in [('variables',False),('meters',True)]:
        rows = value[group]; require(type(rows) is list,'Policy rows must be a list')
        keys = {'meter','units','frequency','atol','rtol','rmse_max'} if meter else {'key','variable','units','frequency','atol','rtol','rmse_max'}
        for row in rows:
            exact(row,keys,'policy row')
            require(all(type(row[key]) is str for key in keys-{'atol','rtol','rmse_max'}),'Invalid named policy identity')
            require(row['meter' if meter else 'variable'],'Empty output name')
            require(row['frequency'] in ['Timestep','TimeStep','Hourly'] and (not meter or row['frequency'] == 'Hourly'),'Unsupported frequency: frozen MDD binding is Hourly only')
            for key in ['atol','rtol','rmse_max']:
                require(type(row[key]) in [int,float] and math.isfinite(row[key]) and row[key] >= 0,'Invalid explicit tolerance')
    require(len({row['frequency'].upper() for row in value['variables']}) <= 1,'Mixed ESO clocks require a separately reviewed kernel version')
    return value


def compare(manifest, core, output, raw, failures, streams):
    exact(manifest,MANIFEST_KEYS,'manifest')
    require(manifest['schema'] == 'reported-stream-comparison-manifest.v2' and type(manifest['id']) is str and manifest['id'],'Unsupported manifest')
    require(manifest['scope'] in ['A','B'],'Explicit supported Source scope required')
    input_path = auth(manifest['input']); weather = auth(manifest['weather']) if manifest['weather'] is not None else None
    require(input_path.suffix.lower() == '.idf' and (weather is None or weather.suffix.lower() == '.epw'),'Original IDF and optional EPW files required')
    exact(manifest['Native'],{'receipt','executable'},'Native binding')
    exact(manifest['Source'],{'receipt','build_receipt','executable','summary'},'Source binding')
    native,source,build = [successful(pin) for pin in [manifest['Native']['receipt'],manifest['Source']['receipt'],manifest['Source']['build_receipt']]]
    require(manifest['Native']['receipt'] != manifest['Source']['receipt'],'Cannot compare one receipt with itself')
    np,nargs = options(native['command'],{'--weather','--output-directory','--output-prefix','--output-suffix'})
    command = source['command']; require(len(command) >= 3 and command[1] == 'run','Source must launch the captured CLI directly')
    sp,sargs = options([command[0],*command[2:]],{'--weather','--output-dir','--mode','--partial','--porting-scope','--oracle-root','--trace-level'})
    require(len(nargs) == len(sargs) == 1 and resolved(nargs[0]) == resolved(sargs[0]) == input_path,'Both actors must consume the same original input')
    require(resolved(native['command'][0]) == auth(manifest['Native']['executable']) and resolved(command[0]) == auth(manifest['Source']['executable']),'Actor executable differs')
    require(sp.get('--mode') == 'compatibility' and sp.get('--partial') == 'deny' and sp.get('--porting-scope') == manifest['scope'],'Unsupported Source route')
    for opts in [np,sp]:
        require(('--weather' in opts) == (weather is not None) and (weather is None or resolved(opts['--weather']) == weather),'Actor weather differs')
    base = [manifest['input']] + ([] if weather is None else [manifest['weather']])
    custody(native,[*base,manifest['Native']['executable']]); custody(source,[*base,manifest['Source']['executable']])
    require(build['command'] == ['cargo','build','--locked','--offline','-p','ep_cli','--bin','eplus-rs','--message-format=json-render-diagnostics'],'Unsupported Source build command')
    capture = build['cargo_compiler_artifacts_capture']
    require(capture['requested'] is True and capture['status'] == 'pass' and capture['error'] is None,'Missing actual emitted CLI capture')
    candidates = [row for row in capture['rows'] if row['cargo_message']['target']['name'] == 'eplus-rs' and row['cargo_message']['target']['kind'] == ['bin'] and resolved(row['cargo_message']['manifest_path']) == ROOT/'crates/ep_cli/Cargo.toml' and resolved(row['cargo_message']['target']['src_path']) == ROOT/'crates/ep_cli/src/main.rs']
    require(len(candidates) == 1,'Exactly one captured CLI required')
    artifact = candidates[0]
    require(artifact['cargo_message']['features'] == [] and artifact['cargo_message']['profile']['test'] is False and artifact['executable']['sha256'] == manifest['Source']['executable']['sha256'],'Wrong compiled executable/features')
    require(resolved(artifact['cargo_message']['executable']) == resolved(artifact['executable']['historical_path']),'Captured Cargo executable path differs')
    require(datetime.fromisoformat(build['finished_utc']) <= datetime.fromisoformat(source['started_utc']),'Build follows Source run')
    # Workspace snapshots and compiled-code snapshot are separately retained;
    # a later workspace observation is not falsely equated with compiled bytes.
    summary = load(manifest['Source']['summary']); runtime = summary['rust_runtime']; config = summary['config']
    require(summary['status'] == 'success' and type(summary['exit_code']) is int and summary['exit_code'] == 0 and summary['oracle'] is None,'Source did not complete independently')
    require(config['mode'] == 'compatibility' and config['partial_policy'] == 'deny' and config['hours'] is None and all(config[key] is False for key in ['dry_run','oracle_baseline','compare_oracle']),'Partial/fallback Source output is unsupported')
    require(runtime['schema'] == 'rusted-energyplus.source-runtime.v1' and runtime['execution'] == 'actual Source ManageSimulation successful through620' and runtime['requested_hours'] is None,'Not a whole completed Source outcome')
    require(not {'result_store_json','selected_outputs_csv','meters_csv'} & set(summary['artifacts']),'Legacy result artifacts are not Source streams')
    consumed_input = identity(resolved(summary['input']['original']))
    require(all(consumed_input[key] == manifest['input'][key] for key in ['sha256','bytes']),'Consumed Source original input differs')
    consumed_weather = summary['input']['weather']
    require((consumed_weather is None) == (weather is None) and (weather is None or all(identity(resolved(consumed_weather))[key] == manifest['weather'][key] for key in ['sha256','bytes'])),'Consumed Source weather differs')
    clock = load(manifest['clock']); exact(clock,{'schema','hours','steps_per_hour','zone_steps','selected_Source_environments','streams'},'clock')
    require(clock['schema'] == 'reported-stream-clock.v1' and all(type(clock[key]) is int and clock[key] > 0 for key in ['hours','steps_per_hour','zone_steps']) and 60 % clock['steps_per_hour'] == 0 and clock['zone_steps'] == clock['hours']*clock['steps_per_hour'],'Invalid whole-run clock')
    selected = [row for row in runtime['native_environment_scope'] if row['selected_by_primary_flags'] is True]
    require(type(clock['selected_Source_environments']) is list and clock['selected_Source_environments'] and typed(selected) == typed(clock['selected_Source_environments']),'Selected Source environments differ')
    require(all(type(row['total_days']) is int and row['total_days'] > 0 and type(row['kind_of_environment']) is int for row in selected),'Invalid selected environment metadata')
    require(type(runtime['native_environment_hour_scope']) is int and runtime['native_environment_hour_scope'] == clock['hours'] and sum(row['total_days'] for row in selected)*24 == clock['hours'],'Whole environment hour scope differs')
    require(weather is not None or all(row['kind_of_environment'] in [1,2] for row in selected),'Weather-less mode is DesignDay only')
    rules = policy(manifest['policy'])
    exact(manifest['artifacts'],{'Native','Source'},'artifact actors'); exact(clock['streams'],{'eso','mtr'},'clock streams')
    for actor in ['Native','Source']:
        exact(manifest['artifacts'][actor],STREAM_KEYS,'artifact set')
    prefix = np['--output-prefix']; suffix = np.get('--output-suffix','L')
    require(re.fullmatch('[A-Za-z0-9_-]+',prefix) is not None and suffix in ['L','D','C'],'Unsupported Native filename mode')
    native_dir,source_dir = resolved(np['--output-directory']),resolved(sp['--output-dir'])
    require(native_dir.is_relative_to(ROOT) and source_dir.is_relative_to(ROOT) and native_dir != source_dir and resolved(manifest['Source']['summary']['path']).parent == source_dir,'Distinct workspace output directories required by the frozen kernel')
    native_stem = native_dir/(prefix+('out' if suffix == 'L' else ''))
    owners = {row['stream']:row for row in runtime['reported_streams']}
    require(len(runtime['reported_streams']) == len(owners) == 2 and set(owners) == {'eso','mtr'},'Wrong Source stream owners')
    for owner in owners.values():
        require(type(owner['deleted_empty']) is bool and type(owner['series']) is int and owner['series'] >= 0 and type(owner['reported_numeric_records']) is int and owner['reported_numeric_records'] >= 0 and type(owner['series_samples']) is list,'Invalid Source stream counters')
        require(all(type(row['samples']) is int and row['samples'] >= 0 and type(row['report_id']) is int for row in owner['series_samples']),'Invalid Source series counters')
    source_stem = resolved(owners['eso']['owned_path']).with_suffix('')
    require(source_stem.parent == source_dir and resolved(owners['eso']['owned_path']) == source_stem.with_suffix('.eso') and resolved(owners['mtr']['owned_path']) == source_stem.with_suffix('.mtr'),'Source owner paths differ')
    rdds,mdds = [],[]
    for actor,stem in [('Source',source_stem),('Native',native_stem)]:
        for ext in ['rdd','mdd']:
            require(auth(manifest['artifacts'][actor][ext]) == stem.with_suffix('.'+ext),'Dictionary artifact ownership differs')
        rdds.append(core.available_variables(stem.with_suffix('.rdd')))
        mdds.append(set(stem.with_suffix('.mdd').read_text('utf8').splitlines()))
    opened = []; original_lines,original_pair = core.PhysicalLines,core.paired
    state = dict(stream=None,environments=[],in_data=False)
    class ObservedLines(original_lines):
        def __init__(self,path):
            try:
                super().__init__(path)
            finally:
                handle = getattr(self,'file',None)
                if handle is not None:
                    opened.append(handle)
    def observed_pair(left,right,exact=True):
        x,y = original_pair(left,right,exact)
        if x == 'End of Data Dictionary':
            state['in_data'] = True
        elif state['in_data'] and x.startswith('1,'):
            state['environments'].append(x)
        if x != y:
            classification = 'allowed_program_version' if left.line == 1 and x.startswith('Program Version,') and y.startswith('Program Version,') else 'unvalidated_header_difference' if left.line == 1 else 'raw_reported_row_difference'
            raw.write(json.dumps(dict(stream=state['stream'],Source_line=left.line,Native_line=right.line,
                classification=classification,
                Source_raw=x,Native_raw=y),ensure_ascii=False)+'\n')
        return x,y
    core.PhysicalLines,core.paired = ObservedLines,observed_pair
    try:
        for kind,rows in [('eso',rules['variables']),('mtr',rules['meters'])]:
            state.update(stream=kind,environments=[],in_data=False)
            for actor,stem in [('Source',source_stem),('Native',native_stem)]:
                pin = manifest['artifacts'][actor][kind]; physical = stem.with_suffix('.'+kind)
                require((pin is None and not physical.exists()) or (pin is not None and auth(pin) == physical),'Missing/changed/extra physical stream')
            result = core.completed_stream(kind,source_stem.with_suffix('.'+kind),native_stem.with_suffix('.'+kind),owners[kind],rows,
                clock['zone_steps'],clock['steps_per_hour'],*rdds,*mdds,failures)
            expected = clock['streams'][kind]
            exact(expected,{'deleted_empty','environment_rows','timestamp_rows','ordered_clock_sha256'},'stream clock')
            require(type(expected['deleted_empty']) is bool and result['deleted_empty'] is expected['deleted_empty'],'Deleted-empty declaration differs')
            require(type(expected['environment_rows']) is list and all(type(row) is str for row in expected['environment_rows']) and type(expected['timestamp_rows']) is int and expected['timestamp_rows'] >= 0,'Invalid expected clock rows')
            if result['deleted_empty']:
                require(expected['environment_rows'] == [] and expected['timestamp_rows'] == 0 and expected['ordered_clock_sha256'] is None,'Deleted stream has declared samples')
            else:
                require(type(expected['ordered_clock_sha256']) is str and re.fullmatch('[0-9a-f]{64}',expected['ordered_clock_sha256']) is not None,'Invalid expected clock digest')
                frequency = rows[0]['frequency'].upper() if rows else None
                stamps = clock['zone_steps'] if frequency == 'TIMESTEP' else clock['hours'] if frequency == 'HOURLY' else 0
                require(type(expected['timestamp_rows']) is int and expected['timestamp_rows'] == result['timestamp_rows'] == stamps,'Missing/extra whole-run clock rows')
                require(expected['environment_rows'] == state['environments'] and len(state['environments']) == (len(selected) if rows else 0),'Ordered physical environments differ')
                require(result['exact_ordered_environment_and_timestamp_rows_sha256'] == expected['ordered_clock_sha256'],'Ordered whole clock digest differs')
                for actor in ['Source','Native']:
                    pin = manifest['artifacts'][actor][kind]
                    require(result[actor]['sha256'] == pin['sha256'] and result[actor]['bytes'] == pin['bytes'],'Scanned stream bytes differ from manifest')
            streams.append(result)
    finally:
        core.PhysicalLines,core.paired = original_lines,original_pair
        close_errors = []
        for handle in opened:
            try:
                handle.close()
            except (OSError,ValueError) as error:
                close_errors.append(str(error))
        if close_errors:
            raise OSError('Read-handle cleanup failed: '+'; '.join(close_errors))
    require(any(stream['series'] for stream in streams),'No numeric series compared')
    return dict(Source_compiled_snapshot=build['source_snapshot'],Source_run_workspace_snapshot=source['source_snapshot'],
                Native_run_workspace_snapshot=native['source_snapshot'],selected_Source_environments=selected)


def references(value):
    if type(value) is dict:
        if set(value) == {'path','sha256','bytes'}:
            yield value
        else:
            for item in value.values():
                yield from references(item)
    elif type(value) is list:
        for item in value:
            yield from references(item)


def write_reports(output, report):
    series = [dict(stream=stream['stream'],**row) for stream in report['streams'] for row in stream['series']]
    with (output/'series.csv').open('x',encoding='utf8',newline='') as stream:
        fields = ['stream','report_id','key','name','units','frequency','store_type','samples','policy_index','atol','rtol','rmse_max','raw_operand_unequal','parsed_binary64_unequal','max_abs_delta','max_relative_delta','rmse_delta','tolerance_failures','rmse_limit_met','passed']
        writer = csv.DictWriter(stream,fieldnames=fields,extrasaction='ignore'); writer.writeheader()
        for row in series:
            writer.writerow(dict(row,**{key:row['policy'][key] for key in ['atol','rtol','rmse_max']}))
    lines = ['# Reported-stream engineering comparison','',f"Status: {report['status']}",f"Complete: {report['complete']}",
             f"Numerical policy passed: {report['accepted']}",'',report['qualification'],'']
    if report['error'] is not None:
        lines += ['Rejection: '+report['error'],'']
    lines += ['| Stream | ID | Name | Samples | Raw differences | Max absolute | RMSE | Passed |','| --- | --- | --- | --- | --- | --- | --- | --- |']
    for row in series:
        name = row['name'].replace('|','\\|').replace('\n',' ')
        lines.append(f"| {row['stream']} | {row['report_id']} | {name} | {row['samples']} | {row['raw_operand_unequal']} | {row['max_abs_delta']} | {row['rmse_delta']} | {row['passed']} |")
    (output/'report.md').write_text('\n'.join(lines)+'\n',encoding='utf8',newline='\n')
    report['retained_files'] = {name:identity(output/name) for name in ['report.md','series.csv','all-tolerance-failures.jsonl','all-raw-differences.jsonl']}
    (output/'comparison.json').write_text(json.dumps(report,ensure_ascii=False,indent=2,allow_nan=False)+'\n',encoding='utf8',newline='\n')


def main():
    require(Path.cwd().resolve() == ROOT,'Run the installed comparer from the repository root')
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--manifest',required=True,help='canonical/path#sha256#bytes')
    parser.add_argument('--output-dir',required=True)
    args = parser.parse_args(); manifest_pin = token(args.manifest)
    output = resolved(args.output_dir)
    require(output.is_relative_to(ROOT/'.runtime/porting') and output != ROOT/'.runtime/porting' and not output.exists(),'Fresh ignored output directory required')
    output.mkdir(parents=True)
    report = dict(schema='reported-stream-comparison.v2',manifest=manifest_pin,comparison_code=identity(__file__),
                  frozen_kernel=KERNEL,path_dependency=PATHS,recorded_launcher=RECORDER,status='rejected',complete=False,accepted=False,error=None,streams=[],custody=None,
                  qualification='Whole ESO/MTR engineering comparison for this explicit manifest only; no registered conformance case, ancillary parity, internal-bit or general Source/EnergyPlus equivalence claim.',
                  provenance_limit='Pins authenticate supplied current files and recorded build/run joins. A receipt alone is not independent OS execution or proof that outputs were never replaced between execution and comparison.',
                  fixed_conformance_case=False,scientific_acceptance=False,engines_launched=0)
    try:
        manifest = load(manifest_pin); core = kernel()
        with (output/'all-tolerance-failures.jsonl').open('x',encoding='utf8',newline='\n') as failures, (output/'all-raw-differences.jsonl').open('x',encoding='utf8',newline='\n') as raw:
            report['custody'] = compare(manifest,core,output,raw,failures,report['streams'])
        for pin in [manifest_pin,*references(manifest),KERNEL,PATHS,RECORDER]:
            auth(pin)
        series = [row for stream in report['streams'] for row in stream['series']]
        report['totals'] = dict(series=len(series),samples=sum(row['samples'] for row in series),raw_operand_unequal=sum(row['raw_operand_unequal'] for row in series),
            parsed_binary64_unequal=sum(row['parsed_binary64_unequal'] for row in series),tolerance_failures=sum(row['tolerance_failures'] for row in series),rmse_failures=sum(not row['rmse_limit_met'] for row in series))
        report.update(complete=True,accepted=all(row['passed'] for row in series),status='passed' if all(row['passed'] for row in series) else 'numeric_mismatch')
    except (ValueError,KeyError,TypeError,IndexError,OSError,UnicodeError,ImportError,OverflowError) as error:
        report['error'] = type(error).__name__+': '+str(error)
        for name in ['all-tolerance-failures.jsonl','all-raw-differences.jsonl']:
            if not (output/name).exists():
                (output/name).write_bytes(b'')
    write_reports(output,report)
    print(json.dumps(dict(comparison=identity(output/'comparison.json'),status=report['status'],complete=report['complete'],accepted=report['accepted'])))
    return 0 if report['complete'] and report['accepted'] else 1 if report['complete'] else 2


if __name__ == '__main__':
    try:
        raise SystemExit(main())
    except (ValueError,OSError,KeyError,TypeError) as error:
        print('Comparer rejected: '+str(error),file=sys.stderr)
        raise SystemExit(2)
