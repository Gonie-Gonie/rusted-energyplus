"""Unit116 read-only helper kernels adapted from Unit112; no engine at import.
The prior kernels are retained literally except CSV total and noncached file hashes.
"""
from pathlib import Path
from collections import Counter, defaultdict
import csv, hashlib, io, json, math, os, re, struct
ROOT = Path(__file__).resolve().parents[4]
R = ROOT
ASCII_UPPER = str.maketrans('abcdefghijklmnopqrstuvwxyz', 'ABCDEFGHIJKLMNOPQRSTUVWXYZ')
AUTHENTICATED = set()
FAILURES = []
ENV_KEYS = [
    'MINREPORTFREQUENCY', 'DEVELOPERFLAG', 'DISPLAYALLWARNINGS',
    'DDONLY', 'REVERSEDD', 'FULLANNUALRUN', 'DISPLAYEXTRAWARNINGS',
    'DISPLAYADVANCEDREPORTVARIABLES', 'DISPLAYUNUSEDOBJECTS', 'DISPLAYUNUSEDSCHEDULES',
    'DISPLAYZONEAIRHEATBALANCEOFFBALANCE', 'SORTIDD', 'REPORTDURINGWARMUP',
    'REPORTDURINGHVACSIZINGSIMULATION', 'IGNORESOLARRADIATION', 'IGNOREBEAMRADIATION',
    'IGNOREDIFFUSERADIATION', 'SUTHERLANDHODGMAN', 'SLATERBARSKY',
    'CREATEMINIMALSURFACEVARIABLES', 'MINIMALSHADOWING', 'TIMINGFLAG',
    'TRACK_AIRLOOP', 'TRACE_AIRLOOP', 'TRACE_HVACCONTROLLER', 'DISPLAYINPUTINAUDIT',
    'CI_FORCE_TIME_STEP', 'BUFFEREDERRFILE', 'RUSTED_ENERGYPLUS_SOURCE_SURFACE_CADENCE',
    'RUSTED_ENERGYPLUS_ORACLE_ROOT',
    'RUSTED_ENERGYPLUS_HEAT_BALANCE_CTF_SEED_POLICY',
    'RUSTED_ENERGYPLUS_HEAT_BALANCE_CTF_INITIAL_HISTORY_POLICY',
    'RUSTED_ENERGYPLUS_HEAT_BALANCE_ZONE_AIR_ALGORITHM',
    'RUSTED_ENERGYPLUS_HEAT_BALANCE_WARMUP_MINIMUM_DAYS',
    'RUSTED_ENERGYPLUS_HEAT_BALANCE_SURFACE_ITERATIONS',
    'RUSTED_ENERGYPLUS_HEAT_BALANCE_INSIDE_HCONV_REEVALUATION_INTERVAL',
    'RUSTED_ENERGYPLUS_HEAT_BALANCE_ZONE_CONDUCTION_REPORT_SOURCE',
    'RUSTED_ENERGYPLUS_HEAT_BALANCE_ZONE_AIR_REPORT_SAMPLING',
    'RUSTED_ENERGYPLUS_HEAT_BALANCE_SURFACE_LOOP_ZONE_AIR_CORRECTION',
]

def read(path):
    return json.loads(path.read_bytes())

def ref(path):
    data = path.read_bytes()
    return dict(path=path.relative_to(ROOT).as_posix(), sha256=hashlib.sha256(data).hexdigest(), bytes=len(data))

def auth(row):
    path = (ROOT / row['path']).resolve()
    assert path.is_relative_to(ROOT) and path.is_file(), row
    actual = ref(path)
    assert actual['sha256'] == row['sha256'] and ('bytes' not in row or actual['bytes'] == row['bytes']), (row, actual)
    return path

def csv_line(line):
    return next(csv.reader([line]))

def stream(path, expected_series, steps, expected_environments):
    raw = path.read_bytes()
    lines = raw.decode('utf-8').splitlines()
    assert raw.count(b'\r\n') == raw.count(b'\n') == len(lines)
    assert lines[0].startswith('Program Version,')
    assert lines.count('End of Data Dictionary') == lines.count('End of Data') == 1
    split, finish = lines.index('End of Data Dictionary'), lines.index('End of Data')
    assert finish == len(lines) - 2
    dictionary, ids = lines[1:split], {}
    for line in dictionary:
        fields = csv_line(line)
        rid = int(fields[0])
        assert rid not in ids
        ids[rid] = fields
    assert {1, 2, 3, 4, 5, 6}.issubset(ids)
    data_ids = {rid for rid in ids if rid > 6}
    assert len(data_ids) == expected_series
    values = defaultdict(list)
    clocks, environments, shape, clock_context = [], [], [], []
    local_clocks, clock_rids, clock_open = 0, [], False
    for line in lines[split + 1:finish]:
        fields = csv_line(line)
        rid = int(fields[0])
        assert rid in ids
        if rid == 1:
            if environments:
                assert clock_open and len(clock_rids) == len(set(clock_rids)) == expected_series and set(clock_rids) == data_ids
                assert local_clocks == 24 * steps
                assert all(len(values[r]) == len(environments) * 24 * steps for r in data_ids)
            assert len(environments) < len(expected_environments) and len(fields) == 6
            expected = expected_environments[len(environments)]
            assert fields[1] == expected['title']
            environments.append(fields)
            local_clocks, clock_rids, clock_open = 0, [], False
            shape.append(line)
        elif rid == 2:
            assert environments
            if clock_open:
                assert len(clock_rids) == len(set(clock_rids)) == expected_series and set(clock_rids) == data_ids
            expected = expected_environments[len(environments) - 1]
            assert local_clocks < 24 * steps and len(fields) == 9
            assert [int(fields[k]) for k in range(1, 6)] == [expected['day_of_sim'], expected['month'], expected['day'], expected['DST'], local_clocks // steps + 1]
            assert [float(fields[6]), float(fields[7])] == [(local_clocks % steps) * 60 / steps, (local_clocks % steps + 1) * 60 / steps]
            assert fields[8] == expected['day_type']
            clocks.append(fields)
            clock_context.append(dict(environment_ordinal=len(environments), environment_title=expected['title'], local_clock_index=local_clocks))
            local_clocks += 1
            clock_rids, clock_open = [], True
            shape.append(line)
        else:
            assert clock_open and rid in data_ids and rid not in clock_rids
            assert len(fields) == 2 and math.isfinite(float(fields[1]))
            assert len(values[rid]) == len(clocks) - 1
            values[rid].append(fields[1])
            clock_rids.append(rid)
            shape.append(str(rid))
    assert len(environments) == len(expected_environments) and len(environments) in [5, 6]
    assert clock_open and len(clock_rids) == len(set(clock_rids)) == expected_series and set(clock_rids) == data_ids
    assert local_clocks == 24 * steps and len(clocks) == 24 * len(expected_environments) * steps
    assert set(values) == data_ids and all(len(row) == 24 * len(expected_environments) * steps for row in values.values())
    samples = sum(map(len, values.values()))
    footer = re.fullmatch(r' Number of Records Written= +(\d+)', lines[-1])
    assert footer and int(footer[1]) == samples
    return dict(artifact=ref(path), dictionary=dictionary, ids=ids, values=values, clocks=clocks, clock_context=clock_context, environments=environments, shape=shape, footer=lines[-1], samples=samples)

def rdd(path):
    lines = path.read_text(encoding='utf-8').splitlines()
    assert len(lines) > 2 and lines[0].startswith('Program Version,')
    assert lines[1] == 'Var Type (reported time step),Var Report Type,Variable Name [Units]'
    result = defaultdict(list)
    for line in lines[2:]:
        fields = csv_line(line)
        assert len(fields) == 3 and fields[0] in ['Zone', 'HVAC'] and fields[1] in ['Average', 'Sum'], line
        match = re.fullmatch(r'(.+) \[(.*)\]', fields[2])
        assert match, line
        result[match[1].translate(ASCII_UPPER)].append(dict(stage=fields[0], store=fields[1], units=match[2], raw=line))
    assert result
    return result

def bind(parsed, rows, catalog, meter=False):
    bindings, used = [], set()
    for index, policy in enumerate(rows):
        matches = []
        for rid, fields in parsed['ids'].items():
            if rid <= 6:
                continue
            if meter:
                assert len(fields) == 3 and fields[1] == '1'
                match = re.fullmatch(r'(.+) \[(.*)\] !Hourly', fields[2])
                if match and match[1].translate(ASCII_UPPER) == policy['meter'].translate(ASCII_UPPER):
                    assert match[2] == policy['units'] == 'J' and policy['frequency'] == 'Hourly'
                    matches.append((rid, match[1], None, match[2], dict(stage='Zone', store='Sum', units='J')))
            else:
                assert len(fields) == 4 and fields[1] == '1'
                match = re.fullmatch(r'(.+) \[(.*)\] !TimeStep', fields[3])
                if match and match[1].translate(ASCII_UPPER) == policy['variable'].translate(ASCII_UPPER) and fields[2].translate(ASCII_UPPER) == policy['key'].translate(ASCII_UPPER):
                    assert policy['frequency'] == 'Timestep' and policy['sampling_stage'] in ['zone', 'system']
                    economizer_hours = (policy['variable'], policy['units'], policy['frequency'], policy['sampling_stage']) == ('Zone Ideal Loads Economizer Active Time', 'hr', 'Timestep', 'system')
                    heat_recovery_hours = (policy['variable'], policy['units'], policy['frequency'], policy['sampling_stage']) == ('Zone Ideal Loads Heat Recovery Active Time', 'hr', 'Timestep', 'system')
                    expected_store = 'Sum' if policy['units'] == 'J' or economizer_hours or heat_recovery_hours else 'Average'
                    if index == 1:
                        assert (policy['key'], policy['variable'], policy['units']) == ('ZONE ONE', 'Zone Air Humidity Ratio', 'kgWater/kgDryAir')
                        expected_units = ''
                    else:
                        expected_units = '' if policy['units'] == 'scalar (declared schedule type retained)' else policy['units']
                    # Preserve policy clock selectors; observe the unique Native
                    # RDD producer stage, then require exact Source RDD/catalog.
                    actual = [row for row in catalog[match[1].translate(ASCII_UPPER)] if row['units'] == expected_units and row['store'] == expected_store and row['stage'] in ['Zone', 'HVAC']]
                    assert len(actual) == 1, (index, policy, catalog[match[1].translate(ASCII_UPPER)])
                    actual = actual[0]
                    if economizer_hours or heat_recovery_hours:
                        assert actual['stage'] == 'HVAC' and actual['store'] == 'Sum' and actual['units'] == 'hr'
                    if index == 1:
                        assert actual['stage'] == 'HVAC'
                    assert match[2] == actual['units'] == expected_units
                    matches.append((rid, match[1], fields[2], match[2], actual))
        assert len(matches) == 1 and matches[0][0] not in used, (index, policy, matches)
        rid, name, key, units, producer = matches[0]
        used.add(rid)
        bindings.append(dict(index=index, report_id=rid, name=name, key=key, units=units, policy=policy, producer=producer))
    assert used == set(parsed['values'])
    return bindings

def obj(path):
    return json.loads(path.read_bytes())

def identify(path):
    path = path.resolve()
    data = path.read_bytes()
    return dict(path=path.relative_to(R).as_posix(), sha256=hashlib.sha256(data).hexdigest(), bytes=len(data))

def verify(pin):
    p = (R / pin['path']).resolve()
    assert p.is_relative_to(R) and p.is_file(), pin
    actual = identify(p)
    assert actual['sha256'] == pin['sha256'] and actual['bytes'] == pin.get('bytes', actual['bytes']), (pin, actual)
    AUTHENTICATED.add(p)
    return p

def bits(value):
    return struct.pack('>d', value).hex()

def inspect_snapshot(build, current):
    receipt = obj(verify(build['receipt']))
    verify(build['retained_executable'])
    assert receipt['exit_code'] == 0 and receipt['launch_error'] is None and receipt['source_bytes_match_before_and_after']
    assert receipt['repository_before'] == receipt['repository_after'] and receipt['source_snapshot'] == build['source_snapshot']
    assert receipt['command'] == ['cargo','build','--locked','--offline','-p','ep_cli','--bin','eplus-rs','--message-format=json-render-diagnostics']
    capture = receipt['cargo_compiler_artifacts_capture']
    assert capture['requested'] and capture['status'] == 'pass' and capture['error'] is None
    artifacts = [a for a in capture['rows'] if a['cargo_message']['target']['name'] == 'eplus-rs' and a['cargo_message']['target']['kind'] == ['bin']]
    assert len(artifacts) == 1 and artifacts[0]['cargo_message']['features'] == [] and artifacts[0]['cargo_message']['profile']['test'] is False
    assert artifacts[0]['executable']['sha256'] == build['retained_executable']['sha256']
    snapshot = obj(verify(build['source_snapshot']))
    assert snapshot['schema'] == 'rust-command-source-snapshot.v1' and snapshot['bytes_normalized'] is False
    inventory = {}
    for item in snapshot['files']:
        assert item['historical_path'] not in inventory
        assert verify(item['archive']).stat().st_size == item['size_bytes']
        inventory[item['historical_path']] = item
        if current:
            actual = identify(R / item['historical_path'])
            assert actual['sha256'] == item['archive']['sha256'] and actual['bytes'] == item['size_bytes']
    if current:
        files = set((R/'crates').rglob('*.rs')) | set((R/'crates').rglob('Cargo.toml'))
        files |= {R/n for n in ['Cargo.toml','Cargo.lock','rust-toolchain.toml','.cargo/config.toml'] if (R/n).is_file()}
        assert set(inventory) == {p.relative_to(R).as_posix() for p in files}
    return receipt,inventory

def inspect_json_export(summary, source, data, metadata, stream):
    reported = obj(source/'results/source-reported-results.json')[stream]
    assert reported['path'] == str(data['path']) and reported['reported_record_count'] == data['count'] and reported['deleted_empty'] is False
    assert reported['version_line'] == data['version']
    assert reported['environment_tuples'] == data['environments'] and reported['timestamp_tuples'] == data['stamps']
    assert len(reported['series']) == len(metadata)
    owner = [s for s in summary['rust_runtime']['reported_streams'] if s['stream'] == stream]
    assert len(owner) == 1
    owner = owner[0]
    assert owner['owned_path'] == str(data['path']) and owner['deleted_empty'] is False
    assert owner['reported_numeric_records'] == data['count'] and owner['series'] == len(metadata)
    assert len(owner['series_samples']) == len(metadata)
    checked = 0
    for observed, exported, identity in zip(owner['series_samples'], reported['series'], metadata, strict=True):
        rid = identity['report_id']
        expected = dict(frequency='hourly' if stream == 'mtr' else 'timestep', key=identity['key'],
                        kind='meter' if stream == 'mtr' else 'variable', name=identity['name'], report_id=rid,
                        samples=len(data['values'][rid]), store_type=identity['producer']['store'].lower(), units=identity['units'])
        assert observed == expected
        assert {k:v for k,v in exported.items() if k not in ['samples','sample_count']} == {k:v for k,v in expected.items() if k != 'samples'}
        assert exported['sample_count'] == expected['samples'] == len(exported['samples'])
        for actual, physical in zip(exported['samples'], data['values'][rid], strict=True):
            assert actual['physical_line'] == physical['physical_line'] and actual['stamp_index'] == physical['stamp_index']
            assert actual['stream_data_ordinal'] == physical['stream_data_ordinal']
            assert actual['reported_fields'] == [physical['token']]
            assert actual['parsed_binary64_bits'] == bits(physical['value']) == bits(actual['value'])
            checked += 1
    return checked

def compare_values(a, b, metadata, stream):
    assert a['dictionary'] == b['dictionary'] and a['shape'] == b['shape'] and a['footer'] == b['footer']
    # Physical line positions also agree: neither reader may reorder a clock, environment, or output occurrence.
    assert a['environments'] == b['environments'] and a['stamps'] == b['stamps']
    metrics = []
    for identity in metadata:
        rid = identity['report_id']; rule = identity['policy']
        raw_count = bit_count = point_count = 0
        sse = maximum = 0.0
        for index,(left,right) in enumerate(zip(a['values'][rid],b['values'][rid],strict=True)):
            av,bv = left['value'],right['value']
            delta = abs(av-bv)
            raw_count += left['token'] != right['token']
            bit_count += bits(av) != bits(bv)
            sse += delta*delta
            maximum = max(maximum,delta)
            if not (delta <= rule['atol'] or delta <= rule['rtol']*abs(av)):
                point_count += 1
                FAILURES.append(dict(kind='point', stream=stream, report_id=rid, policy_index=identity['index'], sample_index=index,
                                     Native=left['token'], Source=right['token'], delta=delta, policy=rule))
        rmse = math.sqrt(sse/len(a['values'][rid]))
        if rmse > rule['rmse_max']:
            FAILURES.append(dict(kind='rmse',stream=stream,report_id=rid,rmse=rmse,policy=rule))
        metrics.append(dict(stream=stream,**identity,samples=len(a['values'][rid]),raw_operand_unequal=raw_count,
                            parsed_binary64_unequal=bit_count,sum_squared_delta=sse,max_abs_delta=maximum,
                            rmse=rmse,tolerance_failures=point_count,rmse_limit_met=rmse<=rule['rmse_max']))
    return metrics

def read_stream(path, count, steps, environments):
    raw = path.read_bytes()
    lines = raw.decode('utf8').splitlines()
    assert raw.count(b'\r\n') == raw.count(b'\n') == len(lines)
    assert lines[0].startswith('Program Version,')
    assert lines.count('End of Data Dictionary') == lines.count('End of Data') == 1
    boundary = lines.index('End of Data Dictionary')
    end = lines.index('End of Data')
    assert end == len(lines) - 2
    dictionary = lines[1:boundary]
    catalog = {}
    for line in dictionary:
        fields = row(line)
        rid = int(fields[0])
        assert rid not in catalog
        catalog[rid] = fields
    assert list(catalog)[:6] == [1, 2, 3, 4, 5, 6]
    ordered = [rid for rid in catalog if rid > 6]
    assert len(ordered) == count
    values = {rid: [] for rid in ordered}
    envs, stamps, shape = [], [], []
    seen = []
    local = 0
    numeric_ordinal = 0

    def finish_clock():
        assert len(seen) == len(set(seen)) == count and set(seen) == set(ordered), ('complete physical clock', seen, ordered)

    for line_index in range(boundary + 1, end):
        line = lines[line_index]
        fields = row(line)
        rid = int(fields[0])
        physical_line = line_index + 1
        assert rid in catalog
        if rid == 1:
            if envs:
                finish_clock()
                assert local == steps * 24
            assert len(envs) < len(environments) and len(fields) == 6
            expected = environments[len(envs)]
            assert fields[1] == expected['title']
            envs.append(dict(ordinal=len(envs) + 1, fields=fields[1:], physical_line=physical_line))
            local = 0
            seen = []
            shape.append(line)
        elif rid == 2:
            assert envs and len(fields) == 9
            if local:
                finish_clock()
            expected = environments[len(envs) - 1]
            assert local < steps * 24
            assert [int(x) for x in fields[1:6]] == [expected['day_of_sim'], expected['month'], expected['day'], expected['DST'], local // steps + 1]
            assert [float(x) for x in fields[6:8]] == [(local % steps) * 60 / steps, (local % steps + 1) * 60 / steps]
            assert fields[8] == expected['day_type']
            stamps.append(dict(environment_ordinal=len(envs), fields=fields[1:], kind='interval', physical_line=physical_line, report_id=2))
            local += 1
            seen = []
            shape.append(line)
        else:
            assert local and rid in values and rid not in seen and len(fields) == 2
            value = float(fields[1])
            assert math.isfinite(value) and len(values[rid]) == len(stamps) - 1
            numeric_ordinal += 1
            values[rid].append(dict(token=fields[1], value=value, physical_line=physical_line, stamp_index=len(stamps)-1, stream_data_ordinal=numeric_ordinal))
            seen.append(rid)
            shape.append(str(rid))
    finish_clock()
    assert local == 24 * steps and len(envs) == len(environments)
    complete_clocks = steps * 24 * len(environments)
    assert len(stamps) == complete_clocks
    assert all(len(a) == complete_clocks for a in values.values())
    footer = re.fullmatch(r' Number of Records Written= +(\d+)', lines[-1])
    assert footer and int(footer[1]) == numeric_ordinal == count * complete_clocks
    return dict(pin=identify(path), path=path, version=lines[0], dictionary=dictionary, catalog=catalog,
                ordered=ordered, values=values, environments=envs, stamps=stamps, shape=shape,
                footer=lines[-1], count=numeric_ordinal)

def inspect_csv_export(source, data, metadata, extension):
    path = source / ('results/source-' + extension + '.csv')
    reader = csv.DictReader(io.StringIO(path.read_text(encoding='utf8')))
    assert reader.fieldnames == ['stream','report_id','key','name','units','frequency','store_type','kind','sample_index','stream_data_ordinal','environment_ordinal','environment_fields_json','timestamp_report_id','timestamp_kind','timestamp_fields_json','reported_fields_json','reported_decimal','parsed_binary64_bits']
    actual_rows = list(reader)
    expected = sum(len(data['values'][item['report_id']]) for item in metadata)
    assert len(actual_rows) == expected == (61920 if extension == 'eso' else 240)
    cursor = 0
    for item in metadata:
        rid = item['report_id']
        for i, physical in enumerate(data['values'][rid]):
            actual = actual_rows[cursor]; cursor += 1
            stamp = data['stamps'][physical['stamp_index']]
            environment = data['environments'][stamp['environment_ordinal'] - 1]
            required = dict(stream=extension, report_id=str(rid), key='' if item['key'] is None else item['key'], name=item['name'], units=item['units'], frequency='hourly' if extension == 'mtr' else 'timestep', store_type=item['producer']['store'].lower(), kind='meter' if extension == 'mtr' else 'variable', sample_index=str(i), stream_data_ordinal=str(physical['stream_data_ordinal']), environment_ordinal=str(stamp['environment_ordinal']), timestamp_report_id=str(stamp['report_id']), timestamp_kind=stamp['kind'], reported_decimal=physical['token'], parsed_binary64_bits=bits(physical['value']))
            for key, value in required.items():
                assert actual[key] == value, (extension, cursor, key, actual[key], value)
            assert json.loads(actual['environment_fields_json']) == environment['fields']
            assert json.loads(actual['timestamp_fields_json']) == stamp['fields']
            assert json.loads(actual['reported_fields_json']) == [physical['token']]
    return dict(artifact=ref(path), operand_bits_line_owner_clock_joins=cursor)

def selected_environment(environment):
    values = {key.upper(): value for key, value in environment.items()}
    keys = set(ENV_KEYS) | {key for key in values if 'SURFACE_CADENCE' in key
                          or key.startswith('RUSTED_ENERGYPLUS_HEAT_BALANCE_')}
    return {key: values.get(key) for key in sorted(keys)}

def default_weather_observation():
    path = ROOT / 'in.epw'
    exists, directory = path.exists(), path.is_dir()
    return dict(cwd=str(ROOT), operand='in.epw', resolved_path=str(path.resolve()),
                lexical_entry_exists=os.path.lexists(path), exists=exists, is_directory=directory, is_file=path.is_file(),
                native_exists_and_not_directory=exists and not directory,
                command_weather_option_present=False,
                qualification='OS observation only; actual SourceOpen independently owns WeatherFileExists.')

def row(line):
    return next(csv.reader([line]))
