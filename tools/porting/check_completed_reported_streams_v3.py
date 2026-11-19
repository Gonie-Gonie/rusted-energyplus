"""Manifest-bound engineering comparison with actual CSV or IDF MDD authority.

The authenticated v2 custody/clock/report owner and v1 numerical kernel remain
unchanged. No engine is launched and no physical dictionary is rewritten.
"""
from pathlib import Path
import argparse
import importlib.util
import json
import re
import sys

V2 = dict(path='tools/porting/check_completed_reported_streams_v2.py',
          sha256='b9e01841cea6dd8e88d2daa7047617174ef2a863312a3c2d1cd19701fe333ef3',bytes=27954)


def require(value, message):
    if not value:
        raise ValueError(message)


def repository_root():
    # The reviewed afterimage can run at its real ignored path before install.
    # A checkout is discovered, never simulated by replacing __file__ or argv.
    roots = [path for path in Path(__file__).resolve().parents
             if (path/'Cargo.toml').is_file() and (path/V2['path']).is_file()]
    require(len(roots) == 1,'Exactly one repository ancestor required')
    return roots[0]


ROOT = repository_root()


def owner():
    import hashlib
    path = ROOT/V2['path']
    raw = path.read_bytes()
    require(len(raw) == V2['bytes'] and hashlib.sha256(raw).hexdigest() == V2['sha256'],
            'Changed frozen v2 comparer')
    spec = importlib.util.spec_from_file_location('_frozen_reported_stream_owner_b9e0',path)
    require(spec is not None and spec.loader is not None,'Cannot load frozen custody owner')
    module = importlib.util.module_from_spec(spec); spec.loader.exec_module(module)
    require(module.ROOT == ROOT,'Frozen custody owner belongs to another repository')
    module.auth(V2)
    return module


def meter_dictionary(text):
    """Parse actual emitted declarations without treating CSV as IDF text."""
    require(type(text) is str,'MDD text required')
    lines = text.splitlines()
    require(len(lines) >= 2,'Missing MDD headers')
    if lines[0].startswith('Program Version,') and lines[1] == 'Var Type (reported time step),Var Report Type,Variable Name [Units]':
        format_name = 'RegularCSV'
    else:
        require(lines[0].startswith('! Program Version,') and lines[1] == '! Output:Meter Objects (applicable to this run)',
                'Unsupported MDD format/header')
        format_name = 'IDF'
    rows = {}
    for number,line in enumerate(lines[2:],3):
        if format_name == 'RegularCSV':
            match = re.fullmatch(r'Zone,Meter,(.+) \[([^\[\]\r\n]*)\]',line)
            require(match is not None,'Malformed Regular CSV MDD row: '+line)
            name,units,cumulative = match[1],match[2],False
        else:
            match = re.fullmatch(r'(Output:Meter|Output:Meter:Cumulative),(.+),hourly; !- \[([^\[\]\r\n]*)\]',line)
            require(match is not None,'Malformed IDF MDD row: '+line)
            name,units,cumulative = match[2],match[3],match[1] == 'Output:Meter:Cumulative'
        identity = name,cumulative
        require(identity not in rows,'Duplicate/ambiguous MDD meter name: '+name)
        rows[identity] = dict(name=name,units=units,cumulative=cumulative,format=format_name,
                              line_number=number,raw_line=line,
                              declared_frequency='Hourly' if format_name == 'IDF' else None)
    for (name,cumulative),row in rows.items():
        other = rows.get((name,not cumulative))
        require(other is None or other['units'] == row['units'],'Cumulative/noncumulative MDD units differ: '+name)
    return dict(format=format_name,lines=lines,rows=rows)


def meter_bindings(source, native, policy_rows, source_pin, native_pin):
    """Both actual declarations must bind before frozen membership is exposed."""
    bindings = {}
    for policy in policy_rows:
        name,units,frequency = policy['meter'],policy['units'],policy['frequency']
        require(frequency == 'Hourly','Frozen meter comparison requires Hourly policy')
        key = name,units,frequency
        require(key not in bindings,'Duplicate selected meter policy')
        declarations = {}
        for actor,dictionary,pin in [('Source',source,source_pin),('Native',native,native_pin)]:
            row = dictionary['rows'].get((name,False))
            require(row is not None,'Selected '+actor+' noncumulative MDD declaration missing: '+name)
            require(row['name'] == name and row['units'] == units and row['cumulative'] is False,
                    'Selected '+actor+' MDD metadata differs: '+name)
            declarations[actor] = dict(dictionary=pin,**row,
                cumulative_companion=dictionary['rows'].get((name,True)))
        bindings[key] = dict(schema='reported-meter-type-authority.v1',store_type='Sum',cumulative=False,
            semantic_binding=dict(name=name,units=units,frequency=frequency,timestep_type='Zone'),
            **declarations,
            authority='Actual noncumulative Meter declarations from Native/Source ProduceRDDMDD; the unchanged kernel separately requires the real Source catalog StoreType::Sum and identical physical MTR dictionary rows.',
            projection='Typed in-memory membership for the frozen Hourly Output:Meter query only. Regular CSV does not declare reporting frequency; actual MTR declarations and named policy bind Hourly. No projected query is an actual raw dictionary line.')
    return bindings


class MeterMembership:
    """A query view, not a replacement dictionary or physical IDF document."""
    def __init__(self, bindings):
        self.bindings = bindings

    def __contains__(self, query):
        return any(query == f'Output:Meter,{name},hourly; !- [{units}]'
                   for name,units,frequency in self.bindings if frequency == 'Hourly')


def compare(manifest, custody_owner, core, output, raw, failures, streams):
    rules = custody_owner.policy(manifest['policy'])
    dictionaries = {}
    for actor in ['Source','Native']:
        pin = manifest['artifacts'][actor]['mdd']
        dictionaries[actor] = meter_dictionary(custody_owner.auth(pin).read_text(encoding='utf8'))
    bindings = meter_bindings(dictionaries['Source'],dictionaries['Native'],rules['meters'],
                             manifest['artifacts']['Source']['mdd'],manifest['artifacts']['Native']['mdd'])
    membership = MeterMembership(bindings)
    original = core.completed_stream

    def completed_stream(kind, source_path, native_path, stream_owner, policy_rows, expected_steps,
                         steps_per_hour, source_rdd, native_rdd, source_mdd, native_mdd, failure_file):
        if kind == 'mtr':
            require(source_mdd == set(dictionaries['Source']['lines']) and native_mdd == set(dictionaries['Native']['lines']),
                    'MDD changed between validation and comparison')
            source_mdd = native_mdd = membership
        result = original(kind,source_path,native_path,stream_owner,policy_rows,expected_steps,
                          steps_per_hour,source_rdd,native_rdd,source_mdd,native_mdd,failure_file)
        if kind == 'mtr':
            for row in result['series']:
                key = row['name'],row['units'],row['frequency']
                require(key in bindings and row['store_type'] == 'sum','Unbound/non-Sum compared meter')
                row['type_authority'] = bindings[key]
        return result

    core.completed_stream = completed_stream
    try:
        return custody_owner.compare(manifest,core,output,raw,failures,streams)
    finally:
        core.completed_stream = original


def main():
    require(Path.cwd().resolve() == ROOT,'Run the comparer from the repository root')
    custody_owner = owner()
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--manifest',required=True,help='canonical/path#sha256#bytes')
    parser.add_argument('--output-dir',required=True)
    args = parser.parse_args(); manifest_pin = custody_owner.token(args.manifest)
    output = custody_owner.resolved(args.output_dir)
    require(output.is_relative_to(ROOT/'.runtime/porting') and output != ROOT/'.runtime/porting' and not output.exists(),'Fresh ignored output directory required')
    output.mkdir(parents=True)
    report = dict(schema='reported-stream-comparison.v3',manifest=manifest_pin,comparison_code=custody_owner.identity(__file__),
                  frozen_custody_owner=V2,frozen_kernel=custody_owner.KERNEL,path_dependency=custody_owner.PATHS,recorded_launcher=custody_owner.RECORDER,
                  status='rejected',complete=False,accepted=False,error=None,streams=[],custody=None,
                  qualification='Whole ESO/MTR engineering comparison for this explicit manifest only; no registered conformance case, ancillary dictionary format parity, internal-bit or general Source/EnergyPlus equivalence claim.',
                  provenance_limit='Pins authenticate supplied current files and recorded build/run joins. A receipt alone is not independent OS execution or proof that outputs were never replaced between execution and comparison.',
                  fixed_conformance_case=False,scientific_acceptance=False,engines_launched=0)
    try:
        manifest = custody_owner.load(manifest_pin); core = custody_owner.kernel()
        with (output/'all-tolerance-failures.jsonl').open('x',encoding='utf8',newline='\n') as failures, (output/'all-raw-differences.jsonl').open('x',encoding='utf8',newline='\n') as raw:
            report['custody'] = compare(manifest,custody_owner,core,output,raw,failures,report['streams'])
        for pin in [manifest_pin,*custody_owner.references(manifest),V2,custody_owner.KERNEL,custody_owner.PATHS,custody_owner.RECORDER]:
            custody_owner.auth(pin)
        series = [row for stream in report['streams'] for row in stream['series']]
        report['totals'] = dict(series=len(series),samples=sum(row['samples'] for row in series),raw_operand_unequal=sum(row['raw_operand_unequal'] for row in series),
            parsed_binary64_unequal=sum(row['parsed_binary64_unequal'] for row in series),tolerance_failures=sum(row['tolerance_failures'] for row in series),rmse_failures=sum(not row['rmse_limit_met'] for row in series))
        report.update(complete=True,accepted=all(row['passed'] for row in series),status='passed' if all(row['passed'] for row in series) else 'numeric_mismatch')
    except (ValueError,KeyError,TypeError,IndexError,OSError,UnicodeError,ImportError,OverflowError) as error:
        report['error'] = type(error).__name__+': '+str(error)
        for name in ['all-tolerance-failures.jsonl','all-raw-differences.jsonl']:
            if not (output/name).exists():
                (output/name).write_bytes(b'')
    custody_owner.write_reports(output,report)
    print(json.dumps(dict(comparison=custody_owner.identity(output/'comparison.json'),status=report['status'],complete=report['complete'],accepted=report['accepted'])))
    return 0 if report['complete'] and report['accepted'] else 1 if report['complete'] else 2


if __name__ == '__main__':
    try:
        raise SystemExit(main())
    except (ValueError,OSError,KeyError,TypeError) as error:
        print('Comparer rejected: '+str(error),file=sys.stderr)
        raise SystemExit(2)
