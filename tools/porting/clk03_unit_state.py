"""Exact selected CLK-03 observations; reference values are comparison operands only."""
from __future__ import annotations

from collections import Counter
import math
import struct

from geo03_provenance import ROOT, exact, path_of, require

DAY_INTS = 'DayOfYear DayOfYear_Schedule Year Month DayOfMonth DayOfWeek DaylightSavingIndex HolidayIndex'.split()
DAY_REALS = 'SinSolarDeclinAngle CosSolarDeclinAngle EquationOfTime'.split()
WEATHER_REALS = 'OutDryBulbTemp OutDewPointTemp OutBaroPress OutRelHum WindSpeed WindDir SkyTemp HorizIRSky BeamSolarRad DifSolarRad Albedo WaterPrecip LiquidPrecip TotalSkyCover OpaqueSkyCover'.split()
FULL_OWNERS = ['global', 'environment', 'weather', 'today_variables', 'tomorrow_variables',
               'today_values', 'tomorrow_values', 'last_hour', 'next_hour', 'missing_values',
               'missed_counts', 'out_of_range_counts']
HEADER_KEYS = 'EPWHeaderTitle WeatherFileLocationTitle WeatherFileLatitude WeatherFileLongitude WeatherFileTimeZone WeatherFileElevation WFAllowsLeapYears EPWDaylightSaving EPWDST DST NumSpecialDays SpecialDays NumDataPeriods NumIntervalsPerHour DataPeriods SpecialDays_allocated DataPeriods_allocated LeapYearAdd EndDayOfMonth wvarsMissedCounts.WeathCodes'.split()
STREAM = ['is_open', 'good', 'eof', 'fail', 'bad', 'position_available', 'position_byte']
ENV_DISCRETE = 'DayOfYear DayOfYear_Schedule Year Month DayOfMonth DayOfWeek HolidayIndex DSTIndicator YearTomorrow MonthTomorrow DayOfMonthTomorrow DayOfWeekTomorrow HolidayIndexTomorrow TotDesDays CurEnvirNum RunPeriodStartDayOfWeek EnvironmentName EndMonthFlag EndYearFlag'.split()
ENV_LITERAL_REALS = ['Latitude', 'Longitude', 'Elevation']
WEATHER_CONTROL_REALS = ['ReadEPlusWeatherCurTime', 'TimeStepFraction', 'IsRainThreshold']
UNPAIRED = [
    'Native-generated processed weather numeric/boolean values and daily solar coefficients',
    'Physical missing/range count generation and missing-value cache generation',
    'Standard barometric pressure preparation formula and other physical producer formulas',
    'Full native EnvironmentData design-day registry, Interpolation and SolarInterpolation arrays',
    'Native DSTIndex, SpecialDayTypes and WeekDayTypes source-only arrays',
    'Native rdstate integer bits, diagnostic/error text and private source-local variables',
    'Native stored record indices (unobserved); Rust raw provenance is not a native index',
    'Raw-header source_only_context from unowned Typical/Extreme and Ground calculations',
    'BeginEnvrn interior handoff input is not separately snapshotted by the native helper',
]


def scalar(value, label):
    require(type(value) is dict and set(value) == {'value', 'value_bits', 'value_class'}, 'Typed scalar required: '+label)
    bits = value['value_bits']
    require(type(bits) is str and len(bits) == 16 and bits == bits.lower(), 'Binary64 token required: '+label)
    try:
        decoded = struct.unpack('>d', bytes.fromhex(bits))[0]
    except (ValueError, struct.error) as error:
        raise ValueError('Invalid binary64 token: '+label) from error
    require(type(value['value']) is float and math.isfinite(decoded) and math.isfinite(value['value'])
            and struct.pack('>d', value['value']).hex() == bits, 'Finite bit/value identity differs: '+label)
    kind = 'negative_zero' if bits == '8000000000000000' else 'positive_zero' if bits == '0000000000000000' else 'finite'
    require(value['value_class'] == kind, 'Binary64 classification differs: '+label)
    return bits, kind


class Comparison:
    def __init__(self):
        self.count = 0
        self.mismatch_count = 0
        self.categories = Counter()
        self.mismatches = []
        self.transport_boundaries = Counter()
        self.unavailable_boundaries = []
        self.unavailable_paths = []

    def compare(self, actual, original, label, category='typed_state'):
        if actual is None or original is None:
            self.unavailable_paths.append({'location':label,'actual_available':actual is not None,
                                           'original_available':original is not None,'counted_as_PASS':False})
            if actual is None and original is None:
                return
            # The availability/type mismatch is observable. The unavailable
            # numerical/value operand itself is never treated as a comparison.
            self.compare(actual is not None, original is not None, label+'/availability', category+'/availability')
            self.compare(type(actual).__name__,type(original).__name__,label+'/type',category+'/type')
            return
        self.count += 1
        self.categories[category] += 1
        if not exact(actual, original):
            self.mismatch_count += 1
            if len(self.mismatches) < 1000:
                self.mismatches.append({'location': label, 'actual': actual, 'original': original})

    def value(self, actual, original, label, category='typed_state'):
        if actual is None or original is None:
            self.compare(actual,original,label,category)
            return
        if type(original) is dict and set(original) == {'value', 'value_bits', 'value_class'}:
            self.compare(scalar(actual, label), scalar(original, label), label, category+'/binary64')
        elif type(original) is dict:
            require(type(actual) is dict and set(actual) == set(original), 'Selected field set differs: '+label)
            for key in original:
                self.value(actual[key], original[key], label+'/'+key, category)
        elif type(original) is list:
            require(type(actual) is list and len(actual) == len(original), 'Selected cardinality differs: '+label)
            self.compare(len(actual), len(original), label+'/length', category+'/cardinality')
            for index, (left, right) in enumerate(zip(actual, original, strict=True)):
                self.value(left, right, label+'/'+str(index), category)
        else:
            self.compare(actual, original, label, category)

    def selected(self, actual, original, keys, label, category='typed_state'):
        require(type(actual) is dict and type(original) is dict and set(keys) <= set(actual)
                and set(keys) <= set(original), 'All selected fields required: '+label)
        for key in keys:
            self.value(actual[key], original[key], label+'/'+key, category)


def stream(check, actual, original, label):
    for row in [actual, original]:
        require(type(row) is dict and set(STREAM+['file_path']) <= set(row), 'Semantic stream fields required')
        require(all(type(row[key]) is bool for key in STREAM if key != 'position_byte'), 'Typed stream flags required')
        require(type(row['file_path']) is str and row['position_available'] is (row['position_byte'] is not None), 'Stream availability differs')
        require(row['position_byte'] is None or type(row['position_byte']) is int and row['position_byte'] >= 0, 'Typed actual position required')
    check.selected(actual, original, STREAM, label, 'source_stream')
    paths = []
    for row in [actual, original]:
        paths.append(path_of(row['file_path']).relative_to(ROOT).as_posix() if row['file_path'] else '')
    check.compare(*paths, label+'/file_path', 'input_identity')


def validate_slots(value, label):
    require(type(value) is dict and set(value) == {'allocated','time_steps','hours','ordering','slots'}, 'Typed weather grid required: '+label)
    require(type(value['allocated']) is bool and type(value['time_steps']) is int and type(value['hours']) is int
            and value['time_steps'] >= 0 and value['hours'] >= 0 and value['ordering'] == 'hour-major,timestep-minor', 'Weather grid dimensions/order differ')
    require((value['time_steps'] > 0 and value['hours'] == 24) if value['allocated']
            else (value['time_steps'] == 0 and value['hours'] == 0), 'Allocated/unallocated weather grid shape differs')
    require(type(value['slots']) is list and len(value['slots']) == value['time_steps']*value['hours'], 'Actual weather grid cardinality differs')
    for index, row in enumerate(value['slots']):
        require(type(row) is dict and set(row) == {'hour','time_step','value'} and type(row['hour']) is int
                and type(row['time_step']) is int and row['hour'] == index//value['time_steps']+1
                and row['time_step'] == index%value['time_steps']+1, 'Actual weather grid slot identity differs')
        require(type(row['value']) is dict and set(row['value']) == set(WEATHER_REALS+['IsRain','IsSnow']), 'Full17 weather slot required')
        require(type(row['value']['IsRain']) is bool and type(row['value']['IsSnow']) is bool, 'Typed weather slot booleans required')
        for key in WEATHER_REALS: scalar(row['value'][key], label+'/'+str(index)+'/'+key)


def selected_environment(snapshot):
    require(snapshot['Environment_allocated'] is True and type(snapshot['environments']) is list, 'Actual allocated source Environment required')
    rows = [row for row in snapshot['environments'] if type(row.get('KindOfEnvrn')) is int and row['KindOfEnvrn'] == 3]
    require(len(rows) == 1, 'Sole actual RunPeriodWeather environment required')
    return {key: rows[0][key] for key in ['CurrentCycle','SetWeekDays']}


def snapshot(check, actual, original, label, *, full=False, selected_context=False):
    require(actual['native_internal_record_index_observed'] is False and actual['native_source_local_variables_observed'] is False
            and original['native_internal_record_index_observed'] is False and original['native_source_local_variables_observed'] is False,
            'No invented native index or local-variable observation allowed')
    for row in [actual, original]:
        for owner in ['today_values', 'tomorrow_values']: validate_slots(row[owner], label+'/'+owner)
    if full:
        for key in FULL_OWNERS: check.value(actual[key], original[key], label+'/'+key, 'full_literal_owner')
    else:
        check.value(actual['global'], original['global'], label+'/global', 'caller_clock')
        check.selected(actual['environment'], original['environment'], ENV_DISCRETE+ENV_LITERAL_REALS, label+'/environment', 'day_environment')
        check.value(actual['weather'], original['weather'], label+'/weather', 'weather_control')
        for key in ['today_variables', 'tomorrow_variables']:
            check.selected(actual[key], original[key], DAY_INTS, label+'/'+key, 'daily_discrete')
        for key in ['today_values','tomorrow_values']:
            check.selected(actual[key], original[key], ['allocated','time_steps','hours','ordering'], label+'/'+key, 'daily_grid_structure')
        check.compare(actual['missed_counts']['WeathCodes'], original['missed_counts']['WeathCodes'], label+'/actual_weather_code_missed_count', 'raw_parser_counter')
        if selected_context:
            check.value(actual['selected_environment_context'], selected_environment(original), label+'/selected_environment_context', 'selected_environment_control')
    check.selected(actual['header'], original['header'], HEADER_KEYS, label+'/header', 'raw_header_owner')
    stream(check, actual['stream'], original['stream'], label+'/stream')


def transport(check, before, after, caller, label):
    """Compare each actual lane's literal copies; no generated cross-lane numbers."""
    check.value(after['today_variables'], before['tomorrow_variables'], label+'/daily_whole_copy', 'within_lane_transport')
    check.value(after['today_values'], before['tomorrow_values'], label+'/grid_whole_copy', 'within_lane_transport')
    projection = {'DayOfYear':'DayOfYear','Year':'Year','Month':'Month','DayOfMonth':'DayOfMonth','DayOfWeek':'DayOfWeek',
                  'HolidayIndex':'HolidayIndex','DSTIndicator':'DaylightSavingIndex', **{key:key for key in DAY_REALS}}
    for field, source in projection.items():
        check.value(after['environment'][field], before['tomorrow_variables'][source], label+'/environment/'+field, 'within_lane_transport')
    holiday = before['tomorrow_variables']['HolidayIndex']
    expected_type = holiday if holiday > 0 else before['tomorrow_variables']['DayOfWeek']
    check.compare(after['weather']['RptDayType'], expected_type, label+'/RptDayType', 'within_lane_transport')
    check.compare(after['global']['PreviousHour'], 24 if caller['BeginEnvrnFlag'] else before['global']['PreviousHour'], label+'/PreviousHour', 'within_lane_transport')
    check.transport_boundaries[label.split('/')[0]] += 1


def operation(check, actual, original, declared, label, full):
    for row in [actual, original]:
        require(row['id'] == declared['id'] and row['kind'] == declared['kind'] and exact(row['requested_operation'],declared), 'Actual operation routing differs')
    original_invoked, rust_invoked = original['actual_source_invoked'], actual['actual_rust_invoked']
    require(type(original_invoked) is bool and type(rust_invoked) is bool, 'Typed actual invocation availability required')
    check.compare(rust_invoked, original_invoked, label+'/actual_invocation', 'outcome_availability')
    for row in [actual, original]:
        outcome = row['call_outcome']
        require(type(outcome) is dict and {'status','source_fatal','exception_message'} <= set(outcome), 'Actual outcome fields required')
        allowed = ['source_returned','source_fatal','not-invoked-after-prior-source-outcome']
        if row is actual: allowed.append('rust_admission_error')
        require(type(outcome['status']) is str and outcome['status'] in allowed, 'Actual outcome status required')
        if outcome['status'] == 'not-invoked-after-prior-source-outcome':
            require(outcome['source_fatal'] is None and outcome['exception_message'] is None, 'Skipped source outcome must stay unavailable')
        else:
            require(type(outcome['source_fatal']) is bool and outcome['source_fatal'] is (outcome['status'] == 'source_fatal'), 'Actual source-fatal availability differs')
            require(outcome['exception_message'] is None if outcome['status'] == 'source_returned'
                    else type(outcome['exception_message']) is str and bool(outcome['exception_message']), 'Actual returned/fatal message availability differs')
    check.selected(actual['call_outcome'], original['call_outcome'], ['status','source_fatal'], label+'/call_outcome', 'outcome_availability')
    for phase in ['before_caller','before','after']:
        snapshot(check, actual[phase], original[phase], label+'/'+phase, full=full, selected_context=not full)
    for key in ['returned_bool','Available','ErrorsFound','print_environment_stamp_before','print_environment_stamp_after']:
        if key in original:
            require(key in actual, 'Actual selected call result missing: '+key)
            check.compare(actual[key], original[key], label+'/'+key, 'selected_call_result')
    if not original_invoked or not rust_invoked: return
    if original['call_outcome']['status'] != 'source_returned' or actual['call_outcome']['status'] != 'source_returned': return
    for lane,row in [('Rust',actual),('original',original)]:
        caller = row['before']['global']
        if declared['kind'] == 'UpdateWeatherData' or declared['kind'] == 'InitializeWeather' and caller['BeginDayFlag'] and not caller['BeginEnvrnFlag']:
            transport(check,row['before'],row['after'],caller,lane+'/'+label)
        elif declared['kind'] == 'InitializeWeather' and caller['BeginDayFlag'] and caller['BeginEnvrnFlag']:
            check.unavailable_boundaries.append({'lane':lane,'operation':label,'boundary':'native helper does not snapshot between first read and interior handoff','counted_as_PASS':False})


def compare(check, original, candidate, request):
    require(original['schema'] == 'clk03-helper-results.v1' and candidate['schema'] == 'clk03-rust-probe-results.v1', 'Actual CLK-03 observation schemas differ')
    counts, statuses = Counter(), Counter()
    for lane in ['handoff','weather']:
        native_rows, rust_rows, inputs = original[lane+'_sequences'],candidate[lane+'_sequences'],request[lane+'_sequences']
        require(type(native_rows) is list and type(rust_rows) is list and len(native_rows) == len(rust_rows) == len(inputs), 'Actual sequence cardinality differs')
        counts[lane+'_sequences'] = len(inputs)
        for native,rust,supplied in zip(native_rows,rust_rows,inputs,strict=True):
            require(native['id'] == rust['id'] == supplied['id'], 'Actual sequence identity differs')
            label = lane+'/'+supplied['id']
            full = lane == 'handoff'
            for phase in ['constructor','allocated_defaults','prepared','final_state']:
                # Weather constructor/allocation precedes genuine original input
                # preparation; source-only preparation state is not a Rust ctor.
                if not full and phase in ['constructor','allocated_defaults']: continue
                snapshot(check,rust[phase],native[phase],label+'/'+phase,full=full,selected_context=not full)
            native_ops,rust_ops,declared_ops = native['operations'],rust['operations'],supplied['operations']
            require(len(native_ops) == len(rust_ops) == len(declared_ops), 'Actual operation cardinality differs')
            counts[lane+'_operations'] += len(declared_ops)
            for a,b,c in zip(rust_ops,native_ops,declared_ops,strict=True):
                statuses[b['call_outcome']['status']] += 1
                operation(check,a,b,c,label+'/'+c['id'],full)
    counts['total_sequences'] = counts['handoff_sequences']+counts['weather_sequences']
    counts['total_operations'] = counts['handoff_operations']+counts['weather_operations']
    check.value(candidate['requested_counts'],original['requested_counts'],'requested_counts','cardinality')
    return {'requested_counts':dict(counts),'actual_native_status_counts':dict(statuses),
            'within_lane_literal_transport_boundaries':dict(check.transport_boundaries),
            'unavailable_interior_handoff_boundaries':check.unavailable_boundaries,
            'unavailable_value_paths':check.unavailable_paths,
            'unpaired_fields_counted_as_PASS':False}
