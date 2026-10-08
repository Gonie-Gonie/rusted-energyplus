"""Independently inspect actual receipts, result structure and declared scope.

No scientific comparer, Rust/native executable, Cargo or Git is invoked.
The existing numerical report is authenticated, not recomputed.
"""
from __future__ import annotations

from collections import Counter
from datetime import datetime, timezone
import hashlib
import importlib.util
import json
import math
from pathlib import Path
import re
import struct
import sys

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[4]
OUT = Path(__file__).resolve().parent
META = ROOT / 'tools/porting/geo03_provenance.py'
META_SHA = '5ca27645245ee9e7206a31c8906904394a5216625723cbfad12a5d5f49da9bf5'
if hashlib.sha256(META.read_bytes()).hexdigest() != META_SHA:
    raise ValueError('Metadata-only verifier bytes changed before import')
spec = importlib.util.spec_from_file_location('clk03_review_metadata', META)
v = importlib.util.module_from_spec(spec)
spec.loader.exec_module(v)
need, exact, read, stamp = v.require, v.exact, v.read, v.timestamp
bindings = v.Bindings()
started = datetime.now(timezone.utc)
PIN = '6f2e40d10250a105b49966baa24d843711e61048'
COMMIT = 'df88eb4c7b03cd8a04ad09d5fce3300a4d5b7b22'
TREE = '47e544dafabcec8e30197597b8832ee1c290c8fc'
SNAPSHOT = 'ca7b7f6bbbc8954f07f5efad2b5a26af93303b9a1ee1a43bf605567386ad3ca4'
COUNT = dict(handoff_sequences=4, weather_sequences=10, handoff_operations=7,
             weather_operations=44, total_sequences=14, total_operations=51)
STATUS = {'source_returned': 46, 'source_fatal': 2,
          'not-invoked-after-prior-source-outcome': 3}
FUNCTIONS = dict(GetNextEnvironment=10, InitializeWeather=27,
                 ReadWeatherForDay=3, UpdateWeatherData=8)
PINS = {
 '.runtime/porting/CLK-03/final-candidate-unit-comparison-01/unit-comparison.json': '80fec36962d0583e7c69bd85e40afddb1cbb645ea0bc1ad53789c7b53a4d95c4',
 '.runtime/porting/CLK-03/final-candidate-unit-comparison-command-01/receipt.json': 'd4f77a8d43a86f76f813345de679bfefb5384c135ef4dfff2ac9e71ae3e0900d',
 '.runtime/porting/CLK-03/final-candidate-probe-build-01/build-receipt.json': 'acbf047718079347630a20e01990597984e1b6e213153a470f1bf7f1db367435',
 '.runtime/porting/CLK-03/final-candidate-probe-command-01/receipt.json': 'f7a23b1ca12ecf67bc2276ce8caf6753034d7c15c7792182425dadd51488cd35',
 '.runtime/porting/CLK-03/unit-dto-freeze-01/projection.json': '4417bed908752814a1a5ea1bd79c1b2af13c0979fb6f2ca11dd3691a31285b6b',
 '.runtime/porting/CLK-03/original-helper-first-03/helper-reference.json': '0513bc94d02703d299cb0a63bc66fa11a0b405e0d0873d3e61a0916a4e123cdc',
 '.runtime/porting/CLK-03/original-command-03/receipt.json': 'cb2f1e545802fcdcdf8015ce5ee2f405fefb02dcd35dd5b196550647dd45fcde',
 '.runtime/porting/CLK-03/independent-original-data-review-02/review.json': 'b8874a962db7c71f6395a5b3ccd811e780fffacc048a76d67bc215634f756166',
 '.runtime/porting/CLK-03/legacy-gap-05/gap.json': '340365c467b4dd606fa9721261e0ffd52f762662943875d1f2aab928bf858097',
 '.runtime/porting/CLK-03/legacy-gap-command-05/receipt.json': '97e3afebd97c101b1ded4101913cf55ce01099bfe8a80061602b1126d79d557a',
 'energyplus_porting_plan/contracts/CLK-03-source.json': 'da6eabfb07236e6303df15ede7e35a4254a2499369ade283a4ebaecfc3cd561d',
 'energyplus_porting_plan/contracts/CLK-03-cases.json': '44d2f61ce68ed13c704e292dc9b25d3e8c6c8f871185027ac88cb3afaa9482c5',
 'energyplus_porting_plan/contracts/CLK-03-tolerances.json': 'fc2b509c35e415f50621cc8f144922ba5b7888ba697e74862298a1a0efb5b98e',
 'energyplus_porting_plan/cases/CLK-03/helper-request.json': 'f12d3d22b5d7d20304cf2934c113555a0c1f7b4023d02cbf497b859d1933b2fb',
 'energyplus_porting_plan/contracts/scope.json': 'b88a28d207400744a108a2aae704f8067fd6b6fd30a5812df53cd8094e94a879',
 'tools/porting/check_clk03_unit.py': '8b77430dd686b4291f756be26d2962d38cc611e6dae31d39b321cb9b244ecfe6',
 'tools/porting/clk03_unit_state.py': '7aff322a5c1278cbd22bd37d00145f8de02023075e51dd0d26aeff99687013e1',
 'tools/porting/geo03_provenance.py': META_SHA,
}


def pinned(path):
    ref = {'path': path, 'sha256': PINS[path]}
    return ref, read(bindings.check(ref))


def same(a, b, message):
    bindings.check(a)
    bindings.check(b)
    need(v.same_binding(a, b), message)


def content(a, b, message):
    need(v.identical_content(a, b, bindings), message)


def interval(receipt):
    a, b = v.execution_interval(receipt, bindings)
    if receipt['schema'] == 'recorded-porting-command.v1':
        need(receipt['launch_error'] is None and receipt['source_bytes_match_before_and_after'] is True,
             'Recorded execution failed or changed source bytes')
        need(receipt['recorder_updates_gates'] is False and receipt['recorder_supplies_reference_answers'] is False,
             'Recorder boundary differs')
        bindings.check(receipt['executed_launcher']['archive'])
    return a, b


def source_clean(receipt):
    need(receipt['source_snapshot']['sha256'] == SNAPSHOT, 'Execution available-source snapshot changed')
    bindings.check(receipt['source_snapshot'])
    for k in ['repository_before', 'repository_after']:
        need(receipt[k]['head'] == COMMIT and receipt[k]['worktree_clean'] is True
             and receipt[k]['porcelain_v1_nul_utf8'] == '', 'Actual execution was not recorded at clean committed source')


report_ref, report = pinned('.runtime/porting/CLK-03/final-candidate-unit-comparison-01/unit-comparison.json')
command_ref, command = pinned('.runtime/porting/CLK-03/final-candidate-unit-comparison-command-01/receipt.json')
cs, cf = interval(command)
source_clean(command)
need(exact(read(bindings.check(command['stdout'])), report), 'Actual reader stdout does not equal the saved report')
need(bindings.check(command['stderr']).stat().st_size == 0, 'Actual passing reader stderr is not empty')
need(exact(command['command'], report['actual_reader_command']), 'Actual comparer argv changed')
need(cs <= stamp(report['started_utc']) <= stamp(report['completed_utc']) <= cf <= started,
     'Actual report/command chronology differs')
need(report['schema'] == 'clk03-unit-comparison.v1'
     and report['status'] == 'pass-exact-selected-carrier-and-lifecycle-state'
     and report['scientific_certification_passed'] is True, 'Report status differs')
for k in ['engines_executed_by_reader', 'Cargo_executed_by_reader', 'Git_executed_by_reader',
          'reference_outputs_supplied_to_Rust', 'original_RHS_recomputed', 'gates_updated', 'unavailable_counted_as_PASS']:
    need(report[k] is False, 'Report boundary differs: ' + k)
need(report['implementation_commit'] == COMMIT and report['crates_tree'] == TREE
     and report['committed_source_certification'] is True, 'Scientific report committed-source identity differs')

frozen = {}
for name in ['source', 'cases', 'tolerances']:
    path = 'energyplus_porting_plan/contracts/CLK-03-' + name + '.json'
    ref, doc = pinned(path)
    same(report['contracts'][name], ref, 'Report input contract differs')
    need(doc['energyplus_commit'] == PIN and doc['packet_revision'] == 3
         and doc['scientific_execution_performed'] is False and doc['gates_changed'] is False,
         'Runtime packet is not literal frozen revision3')
    frozen[name] = doc
request_ref, request = pinned('energyplus_porting_plan/cases/CLK-03/helper-request.json')
same(report['request'], request_ref, 'Scientific report request differs')
need(request['expected_values_supplied'] is False and request['expected_exits_supplied'] is False,
     'Input request contains scientific expected answers')
need(exact(frozen['cases']['counts'], COUNT), 'Declared sequence/operation cardinality differs')
for artifact in frozen['cases']['input_artifacts']:
    bindings.check(artifact)
scope_ref, scope = pinned('energyplus_porting_plan/contracts/scope.json')
same(request['fixed_scope'], scope_ref, 'Fixed CON scope differs')
need(type(scope['cases']) is list and len(scope['cases']) == 15, 'CON scope cardinality differs')
for case in scope['cases']:
    for key in ['input', 'weather', 'metadata']:
        bindings.check(case[key])
need(frozen['tolerances']['tolerance_widening_allowed_for_pass'] is False
     and frozen['tolerances']['policy']['finite_literal_binary64'] == 'exact IEEE754 bits including signed zero; atol=0, rtol=0',
     'Tolerance/signed-zero policy changed')

projection_ref, projection = pinned('.runtime/porting/CLK-03/unit-dto-freeze-01/projection.json')
same(report['declared_pre_execution_DTO_projection'], projection_ref, 'Declared DTO projection differs')
need(projection['schema'] == 'clk03-unit-dto-projection.v1'
     and projection['status'] == 'declared-before-canonical-probe-execution', 'DTO declaration status differs')
for key in ['scientific_comparer_executed', 'prospective_numerical_PASS_claimed', 'gates_updated']:
    need(projection[key] is False, 'Projection certified future science')
need(exact(report['selected_policy'], projection['selected_policy'])
     and exact(report['unpaired_scope'], report['selected_policy']['unpaired_scope']), 'Frozen selected scope changed')
need(exact(report['selected_policy']['selected_environment_context'], ['CurrentCycle', 'SetWeekDays'])
     and report['selected_policy']['unavailable_counted_as_PASS'] is False,
     'Selected environment controls or unavailable policy changed')
reader_sources = {x['historical_path']: x['archive'] for x in report['executed_reader_source_archives']}
need(len(reader_sources) == 3 and set(reader_sources) == {'tools/porting/check_clk03_unit.py',
     'tools/porting/clk03_unit_state.py', 'tools/porting/geo03_provenance.py'}, 'Actual comparer source inventory differs')
for owner, archive in reader_sources.items():
    content(archive, {'path': owner, 'sha256': PINS[owner]}, 'Actually executed comparer/policy source bytes changed')
for k, owner in [('comparer_archive', 'tools/porting/check_clk03_unit.py'),
                 ('state_reader_archive', 'tools/porting/clk03_unit_state.py'),
                 ('metadata_dependency_archive', 'tools/porting/geo03_provenance.py')]:
    content(projection[k], reader_sources[owner], 'Projection/comparer source byte identity differs')

build_ref, _ = pinned('.runtime/porting/CLK-03/final-candidate-probe-build-01/build-receipt.json')
same(report['Rust_build'], build_ref, 'Actual build binding differs')
build, available, cargo = v.verify_rust_build(build_ref, bindings, kind='example', example='clk03_candidate_probe',
        committed=True, required_sources=['crates/ep_runtime/examples/clk03_candidate_probe.rs',
        'crates/ep_runtime/src/weather_day/production.rs', 'crates/ep_runtime/src/weather_raw/cursor.rs'])
need(build['implementation_commit'] == COMMIT and build['crates_tree'] == TREE
     and build['available_source_inventory_count'] == report['available_source_inventory_count'] == 3718,
     'Actual build/report available bytes or source identity differs')
need(build['binary']['sha256'] == '82a192599ef1496de9a170b04371364432d8130d70f0b6c169e4acf6ded39783',
     'Actual emitted executable identity differs')
rust_exec_ref, rust_exec = pinned('.runtime/porting/CLK-03/final-candidate-probe-command-01/receipt.json')
same(report['Rust_execution'], rust_exec_ref, 'Actual candidate execution differs')
rs, rf = interval(rust_exec)
source_clean(rust_exec)
source_clean(cargo)
same(rust_exec['source_snapshot'], build['source_snapshot'], 'Build/run source bytes differ')
need(exact(rust_exec['command'], [build['binary']['path'], request_ref['path']]),
     'Probe was not launched exclusively with its archived Cargo binary and literal request')
same(report['Rust_results'], rust_exec['stdout'], 'Results were not actual candidate stdout')
need(bindings.check(rust_exec['stderr']).stat().st_size == 0, 'Candidate actual stderr is not empty')
need(stamp(cargo['finished_utc']) <= rs <= rf <= cs and stamp(report['actual_Rust_finished_utc']) == rf,
     'Build/probe/comparer ordering differs')
first = read(ROOT / '.runtime/porting/CLK-03/candidate-probe-command-01/receipt.json')
need(stamp(projection['created_utc']) <= stamp(first['started_utc']) <= rs, 'DTO mapping was not frozen before first candidate')

original_ref, original = pinned('.runtime/porting/CLK-03/original-helper-first-03/helper-reference.json')
original_command_ref, original_command = pinned('.runtime/porting/CLK-03/original-command-03/receipt.json')
data_ref, data = pinned('.runtime/porting/CLK-03/independent-original-data-review-02/review.json')
gap_ref, gap = pinned('.runtime/porting/CLK-03/legacy-gap-05/gap.json')
gap_command_ref, gap_command = pinned('.runtime/porting/CLK-03/legacy-gap-command-05/receipt.json')
for report_key, ref in [('original_helper', original_ref), ('original_command', original_command_ref),
 ('independent_original_data_review', data_ref), ('legacy_gap', gap_ref), ('legacy_gap_command', gap_command_ref)]:
    same(report[report_key], ref, 'Original/data/baseline prerequisite differs')
os_, of = interval(original_command)
gs, gf = interval(gap_command)
need(exact(read(bindings.check(original_command['stdout'])), {'reference': original_ref,
     'status': original['status'], 'actual_helper_exit_code': 0, 'actual_launcher_exit_code': 0}),
     'Actual original wrapper compact stdout differs')
need(exact(read(bindings.check(gap_command['stdout'])), {'gap': gap_ref, 'status': gap['status'],
     'numerical_PASS_claimed': False}), 'Actual gap reader compact stdout differs')
need(original['actual_helper_exit_code'] == original['actual_launcher_exit_code'] == 0
     and original['preservation_checks_passed'] is True and original['energyplus_commit'] == PIN
     and original['source_expected_answers_supplied'] is False and original['new_build_claimed'] is False,
     'Actual original execution/preservation boundary differs')
native_execution = read(bindings.check(original['execution']))
ns, nf = interval(native_execution)
need(native_execution['schema'] == 'original-native-command.v1' and native_execution['spawn_error'] is None,
     'Actual genuine native child differs')
bindings.check(native_execution['binary'])
same(native_execution['request'], request_ref, 'Original child input differs')
same(original['actual_request'], request_ref, 'Original runtime request differs')
for key, ref in report['contracts'].items():
    same(original['contracts'][key], ref, 'Original runtime contract differs')
same(report['original_results'], original['results'], 'Original actual result binding differs')
same(data['reviewed_reference'], original_ref, 'Independent data review actual reference differs')
same(data['reviewed_results'], original['results'], 'Independent data review actual data differs')
same(data['reviewed_execution'], original['execution'], 'Independent data review child differs')
need(data['status'] == 'pass-preserved-original-data-before-Rust-baseline'
     and data['intended_native_branch_coverage_complete'] is True
     and data['prior_preparation_SourceFatal_accepted_as_intended_reader_coverage'] is False,
     'Actual diagnostic source coverage was not independently admitted')
need(gap['numerical_comparison_count'] is None and gap['numerical_mismatch_count'] is None
     and gap['numerical_PASS_claimed'] is False and gap['intended_native_branch_coverage_complete'] is True,
     'Legacy gap falsely claims comparison PASS')
need(os_ <= ns <= nf <= of <= stamp(data['review_completed_utc']) <= gs <= gf <= rs,
     'Original/data/baseline/candidate chronology differs')
for key in ['runtime_input_admission', 'runtime_input_admission_execution']:
    same(original[key], gap[key], 'Original/gap runtime admission differs')
    same(original[key], data['reviewed_' + key] if 'reviewed_' + key in data else original[key],
         'Initial data runtime admission differs')
bindings.check(original['native_driver_build'])

comparison = report['comparison']
need(type(comparison['comparison_count']) is int and comparison['comparison_count'] == 239178
     and type(comparison['mismatch_count']) is int and comparison['mismatch_count'] == 0
     and exact(comparison['mismatches'], []) and comparison['mismatch_payloads_truncated'] is False,
     'Actual numerical report count/payload differs')
need(all(type(x) is int and x >= 0 for x in comparison['categories'].values())
     and sum(comparison['categories'].values()) == comparison['comparison_count'], 'Reported category accounting differs')
coverage = report['coverage']
need(exact(coverage['requested_counts'], COUNT) and exact(coverage['actual_native_status_counts'], STATUS),
     'Actual requested/outcome coverage differs')
need(exact(coverage['within_lane_literal_transport_boundaries'], {'Rust': 22, 'original': 22}),
     'Declared observed full-carrier transport coverage differs')
missing = coverage['unavailable_value_paths']
need(type(missing) is list and len(missing) == comparison['unavailable_operand_count'] == 114,
     'Unavailable value cardinality differs')
need(all(x['actual_available'] is False and x['original_available'] is False
     and x['counted_as_PASS'] is False for x in missing), 'Unavailable values counted as passing comparisons')
suffixes = Counter(x['location'].rsplit('/', 1)[-1] for x in missing)
need(exact(dict(suffixes), {'position_byte': 72, 'returned_bool': 39, 'source_fatal': 3}),
     'Unavailable operand reasons differ')
interior = coverage['unavailable_interior_handoff_boundaries']
need(len(interior) == 12 and all(x['counted_as_PASS'] is False for x in interior)
     and exact(dict(Counter(x['lane'] for x in interior)), {'Rust': 6, 'original': 6})
     and coverage['unpaired_fields_counted_as_PASS'] is False
     and comparison['unavailable_operands_counted_as_PASS'] is False,
     'Unobserved boundary/unpaired fields counted as passing')

DAILY = {'CosSolarDeclinAngle', 'DayOfMonth', 'DayOfWeek', 'DayOfYear', 'DayOfYear_Schedule',
         'DaylightSavingIndex', 'EquationOfTime', 'HolidayIndex', 'Month', 'SinSolarDeclinAngle', 'Year'}
SLOT = {'Albedo', 'BeamSolarRad', 'DifSolarRad', 'HorizIRSky', 'IsRain', 'IsSnow', 'LiquidPrecip',
        'OpaqueSkyCover', 'OutBaroPress', 'OutDewPointTemp', 'OutDryBulbTemp', 'OutRelHum', 'SkyTemp',
        'TotalSkyCover', 'WaterPrecip', 'WindDir', 'WindSpeed'}
scalar_classes = Counter()


def dto_shapes(value):
    if type(value) is list:
        for child in value:
            dto_shapes(child)
    elif type(value) is dict:
        if set(value) == {'value', 'value_bits', 'value_class'}:
            real = value['value']
            need(type(real) in (int, float) and type(real) is not bool and math.isfinite(real)
                 and type(value['value_bits']) is str and re.fullmatch('[0-9a-f]{16}', value['value_bits']),
                 'Malformed/nonfinite observed binary64 DTO')
            bits = struct.pack('>d', float(real)).hex()
            cls = 'negative_zero' if bits == '8000000000000000' else 'positive_zero' if bits == '0000000000000000' else 'finite'
            need(value['value_bits'] == bits and value['value_class'] == cls,
                 'Actual DTO decimal/bits/class representation is inconsistent')
            scalar_classes[cls] += 1
        for key in ['today_variables', 'tomorrow_variables']:
            if key in value:
                need(set(value[key]) == DAILY, 'Daily full11 carrier storage incomplete')
        for key in ['last_hour', 'next_hour']:
            if key in value:
                need(set(value[key]) == SLOT, 'Hourly full17 carrier storage incomplete')
        for key in ['today_values', 'tomorrow_values']:
            if key in value:
                grid = value[key]
                need(type(grid['allocated']) is bool and type(grid['time_steps']) is int
                     and type(grid['hours']) is int and type(grid['slots']) is list,
                     'Malformed actual grid storage')
                if grid['allocated']:
                    need(grid['hours'] == 24 and grid['time_steps'] > 0
                         and grid['ordering'] == 'hour-major,timestep-minor'
                         and len(grid['slots']) == grid['hours'] * grid['time_steps'], 'Incomplete actual allocated grid')
                    for i, slot in enumerate(grid['slots']):
                        need(type(slot['hour']) is int and type(slot['time_step']) is int
                             and slot['hour'] == i // grid['time_steps'] + 1
                             and slot['time_step'] == i % grid['time_steps'] + 1
                             and set(slot['value']) == SLOT, 'Grid ordered full17 slot missing')
        for key, child in value.items():
            # requested_operation is literal input JSON, not an observed storage DTO.
            if key != 'requested_operation':
                dto_shapes(child)


lane_metadata = []
for lane, results_ref, invoked_key in [('Rust', report['Rust_results'], 'actual_rust_invoked'),
                                      ('original', report['original_results'], 'actual_source_invoked')]:
    results = read(bindings.check(results_ref))
    need(results['complete'] is True and exact(results['requested_counts'], COUNT)
         and results['actual_operation_invocations'] == 48 and results['actual_operations_skipped'] == 3
         and exact(results['actual_operation_counts'], FUNCTIONS), 'Actual lane operation accounting differs')
    statuses, functions = Counter(), Counter()
    case_rows = []
    for group in ['handoff_sequences', 'weather_sequences']:
        requested, observed = request[group], results[group]
        need(type(observed) is list and len(observed) == len(requested), 'Actual sequence cardinality differs')
        for desired, actual in zip(requested, observed):
            need(actual['id'] == desired['id'] and len(actual['operations']) == len(desired['operations']),
                 'Actual input sequence identity/order differs')
            rows = []
            for want, got in zip(desired['operations'], actual['operations']):
                need(got['id'] == want['id'] and got['kind'] == want['kind']
                     and exact(got['requested_operation'], want) and type(got[invoked_key]) is bool,
                     'Actual requested call identity/type differs')
                status = got['call_outcome']['status']
                need(status in STATUS and got[invoked_key] is (status != 'not-invoked-after-prior-source-outcome'),
                     'Actual invocation availability is inconsistent')
                need(got['call_outcome']['source_fatal'] is (True if status == 'source_fatal' else False if status == 'source_returned' else None),
                     'Actual fatal status/availability differs')
                if got[invoked_key]:
                    functions[got['kind']] += 1
                statuses[status] += 1
                rows.append({'id': got['id'], 'kind': got['kind'], 'status': status, 'invoked': got[invoked_key]})
            if desired['id'] == 'MISSING-MATCH-DEFINED-FAILURE':
                need(rows[1]['status'] == 'source_fatal' and rows[1]['kind'] == 'InitializeWeather', 'Missing-date failed before intended reader')
            elif desired['id'] == 'OUTSIDE-DATA-PERIOD':
                need(rows[0]['status'] == 'source_fatal' and rows[0]['kind'] == 'GetNextEnvironment', 'Outside-period failed before intended selection')
            else:
                need(all(x['status'] == 'source_returned' for x in rows), 'Normal/direct diagnostic stopped before intended call')
            case_rows.append({'id': actual['id'], 'operations': rows})
    need(exact(dict(statuses), STATUS) and exact(dict(functions), FUNCTIONS), 'Actual observed statuses/counts differ')
    dto_shapes(results)
    lane_metadata.append({'lane': lane, 'results': results_ref, 'actual_status_counts': dict(statuses),
                          'actual_function_counts': dict(functions), 'sequence_observations': case_rows})
    if lane == 'Rust':
        same(results['actual_request'], request_ref, 'Candidate observer request differs')
        for key in ['expected_answers_supplied', 'legacy_reference_results_read', 'native_reference_results_read',
                    'original_error_text_peer_parity_claimed', 'probe_changes_production_sources',
                    'processed_weather_physics_compared', 'whole_native_constructor_or_private_state_claimed', 'gates_updated']:
            need(results[key] is False, 'Candidate observer unowned certification boundary differs: ' + key)
need(scalar_classes['negative_zero'] > 0, 'Actual literal signed-zero canaries absent')

source_dir = OUT / 'source'
need(not (OUT / 'review.json').exists() and not source_dir.exists(), 'Fresh immutable review output required')
source_dir.mkdir()
archives = []
for index, path in enumerate([__file__, str(META), str(ROOT / 'tools/porting/check_clk03_unit.py'),
                            str(ROOT / 'tools/porting/clk03_unit_state.py')]):
    original_path = Path(path).resolve()
    archive = source_dir / (str(index) + '-' + original_path.name)
    archive.write_bytes(original_path.read_bytes())
    archives.append({'historical_path': original_path.relative_to(ROOT).as_posix(), 'archive': v.ref(archive)})
bindings.unchanged()
output = {
 'schema': 'clk03-independent-unit-result-review.v1',
 'status': 'pass-authenticated-selected-unit-result-and-honest-unavailable-scope',
 'review_started_utc': started.isoformat(), 'review_completed_utc': datetime.now(timezone.utc).isoformat(),
 'preserved_prior_metadata_attempts': [v.ref(ROOT / '.runtime/porting/CLK-03/independent-unit-result-review-command-01/receipt.json'), v.ref(ROOT / '.runtime/porting/CLK-03/independent-unit-result-review-command-02/receipt.json')],
 'reviewed_report': report_ref, 'reviewed_comparison_execution': command_ref,
 'reviewed_Rust_build': build_ref, 'reviewed_Rust_execution': rust_exec_ref,
 'reviewed_original_reference': original_ref, 'reviewed_original_execution': original_command_ref,
 'reviewed_independent_original_data': data_ref, 'reviewed_legacy_gap': gap_ref,
 'reviewed_legacy_gap_execution': gap_command_ref, 'reviewed_pre_execution_projection': projection_ref,
 'reviewed_frozen_contracts': report['contracts'], 'reviewed_request': request_ref, 'reviewed_fixed_CON_scope': scope_ref,
 'actual_reported_comparisons': comparison['comparison_count'], 'actual_reported_mismatches': comparison['mismatch_count'],
 'actual_reported_scoped_unit_PASS_authenticated': True,
 'selected_policy_unchanged_from_pre_execution_projection': True,
 'actual_unavailable_values': len(missing), 'unavailable_value_suffixes': dict(suffixes),
 'actual_unavailable_interior_boundaries': len(interior), 'unavailable_counted_as_PASS': False,
 'actual_lane_observations': lane_metadata, 'actual_binary64_DTO_representation_counts': dict(scalar_classes),
 'available_source_inventory_count': len(available),
 'available_source_inventory_is_compiler_selection': False,
 'actual_build_commit': COMMIT, 'actual_crates_tree': TREE, 'actual_source_snapshot': build['source_snapshot'],
 'numerical_comparisons_recomputed_by_reviewer': False,
 'scientific_comparer_imported_or_executed_by_reviewer': False,
 'native_or_Rust_engine_executed_by_reviewer': False, 'Cargo_or_Git_executed_by_reviewer': False,
 'tracked_files_or_gates_changed': False, 'whole_weather_physics_certified_by_reviewer': False,
 'production_invocation_certified_by_this_review': False,
 'unit_original_is_preserved_14_sequence_51_operation_reference': True,
 'unit_claimed_to_follow_future_supplemental_matched_production_original': False,
 'authorship_disclosure': 'Reviewer authored raw header and thermal consumer bridges/tests, and native build guards. This independent review authenticates other-authored comparison/observer outputs and actual evidence; it does not independently certify those self-authored implementations.',
 'executed_reader_source_archives': archives,
 'limits': report['limits'] + ['No scientific value comparison was rerun. Numerical 239178/0 is the authenticated actual Root execution report.',
                            'Native-generated processed physics is unpaired across lanes; full literal supplied carriers and observed within-lane transport are the selected unit scope.',
                            'No production invocation or physical-stage closure is inferred from this unit report.',
                            'This final unit execution follows the preserved 14/51 original. It does not follow or certify the distinct future three-case matched-production original request.'],
}
(OUT / 'review.json').write_text(json.dumps(output, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
print(json.dumps({'review': v.ref(OUT / 'review.json'), 'status': output['status']}, ensure_ascii=False))
