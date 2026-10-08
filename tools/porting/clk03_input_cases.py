"""Prepare input-only CLK03 selection/lifecycle diagnostics; never expected outputs."""
from pathlib import Path
import hashlib
import json
import struct
from datetime import date

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'energyplus_porting_plan/cases/CLK-03'
DAY_INTS = ['DayOfYear', 'DayOfYear_Schedule', 'Year', 'Month', 'DayOfMonth',
            'DayOfWeek', 'DaylightSavingIndex', 'HolidayIndex']
DAY_REALS = ['SinSolarDeclinAngle', 'CosSolarDeclinAngle', 'EquationOfTime']
WEATHER_REALS = ['OutDryBulbTemp', 'OutDewPointTemp', 'OutBaroPress', 'OutRelHum',
                 'WindSpeed', 'WindDir', 'SkyTemp', 'HorizIRSky', 'BeamSolarRad',
                 'DifSolarRad', 'Albedo', 'WaterPrecip', 'LiquidPrecip',
                 'TotalSkyCover', 'OpaqueSkyCover']
COUNT_FIELDS = ['OutDryBulbTemp', 'OutDewPointTemp', 'OutRelHum', 'OutBaroPress',
                'WindDir', 'WindSpeed', 'BeamSolarRad', 'DifSolarRad', 'TotalSkyCover',
                'OpaqueSkyCover', 'Visibility', 'Ceiling', 'LiquidPrecip', 'WaterPrecip',
                'AerOptDepth', 'SnowDepth', 'DaysLastSnow', 'WeathCodes', 'Albedo']


def bits(value):
    return {'bits': struct.pack('>d', value).hex()}


def ref(path):
    return {'path': path.relative_to(ROOT).as_posix(),
            'sha256': hashlib.sha256(path.read_bytes()).hexdigest()}


def put(name, data):
    path = OUT / name
    path.parent.mkdir(parents=True, exist_ok=True)
    if path.exists() and path.read_bytes() != data:
        raise RuntimeError('Refuse to overwrite an existing different diagnostic: ' + str(path))
    if not path.exists():
        path.write_bytes(data)
    return ref(path)


def day(seed, holiday):
    result = {name: seed + i for i, name in enumerate(DAY_INTS)}
    result['HolidayIndex'] = holiday
    result.update({name: bits(-0.0 if i == 0 else seed + i / 4)
                   for i, name in enumerate(DAY_REALS)})
    return result


def weather(seed):
    result = {'IsRain': bool(seed % 2), 'IsSnow': bool((seed // 2) % 2)}
    result.update({name: bits(-0.0 if (seed + i) % 17 == 0 else seed / 8 + i / 4)
                   for i, name in enumerate(WEATHER_REALS)})
    return result


def handoff_case(identifier, begin, holiday):
    global_state = {'BeginEnvrnFlag': begin, 'BeginDayFlag': True, 'EndDayFlag': False,
                    'EndEnvrnFlag': False, 'WarmupFlag': True, 'DayOfSim': 5,
                    'CalendarYear': 2013, 'PreviousHour': 7, 'HourOfDay': 1,
                    'NumOfDayInEnvrn': 3, 'TimeStepsInHour': 4, 'TimeStep': 2}
    environment = {name: 100 + i for i, name in enumerate(
        ['DayOfYear', 'DayOfYear_Schedule', 'Year', 'Month', 'DayOfMonth', 'DayOfWeek',
         'HolidayIndex', 'DSTIndicator', 'YearTomorrow', 'MonthTomorrow',
         'DayOfMonthTomorrow', 'DayOfWeekTomorrow', 'HolidayIndexTomorrow'])}
    environment.update({name: bits(20 + i / 4) for i, name in enumerate(DAY_REALS)})
    initial = {'global': global_state, 'environment': environment,
               'weather': {'RptDayType': 77, 'LastHourSet': True, 'CurDayOfWeek': 4,
                           'ReadEPlusWeatherCurTime': bits(2.5)},
               'today_variables': day(11, 0), 'tomorrow_variables': day(31, holiday),
               'today_values': [weather(1000 + i) for i in range(96)],
               'tomorrow_values': [weather(2000 + i) for i in range(96)],
               'last_hour': weather(701), 'next_hour': weather(702),
               'missed_counts': {name: 801 + i for i, name in enumerate(COUNT_FIELDS)},
               'out_of_range_counts': {name: 901 + i for i, name in enumerate(COUNT_FIELDS)},
               'missing_values': weather(703) | {'Visibility': bits(1.25), 'Ceiling': bits(2.5),
                  'AerOptDepth': bits(3.75), 'SnowDepth': bits(-0.0), 'DaysLastSnow': 7}}
    return {'id': identifier, 'time_steps_per_hour': 4, 'initial': initial,
            'operations': [
                {'id': 'first', 'kind': 'UpdateWeatherData', 'before': {}},
                {'id': 'repeat-with-new-tomorrow', 'kind': 'UpdateWeatherData',
                 'before': {'global': {'BeginEnvrnFlag': False, 'WarmupFlag': False},
                            'tomorrow_variables': day(51, 9),
                            'tomorrow_values': [weather(3000 + i) for i in range(96)]}}]}


def make_epw(days):
    header = ('LOCATION,CLK03,CO,USA,Test,000000,39.74,-105.18,-7,1600\n'
              'DESIGN CONDITIONS,0\nTYPICAL/EXTREME PERIODS,0\nGROUND TEMPERATURES,0\n'
              'HOLIDAYS/DAYLIGHT SAVINGS,No,0,0,0\nCOMMENTS 1,input-only CLK03\n'
              'COMMENTS 2,no expected outputs\nDATA PERIODS,1,1,DATA,Sunday,6/30,7/2\n')
    rows = []
    for sequence, (year, month, date) in enumerate(days):
        for hour in range(1, 25):
            values = [year, month, date, hour, 60, '?',
                      10 + sequence * 5 + hour / 8, 2 + sequence, 45 + hour / 4,
                      90000 + sequence * 100, 0, 0, 300, 100, 80, 20,
                      0, 0, 0, 0, 180 + sequence * 10, 2 + hour / 16,
                      3, 2, 10, 20000, 9, 999999999, 10, 0.1, 0, 0, 0.2, 0, 1]
            if len(values) != 35:
                raise RuntimeError('Complete 35-field input row required')
            rows.append(','.join(str(v) for v in values))
    return (header + '\n'.join(rows) + '\n').encode('utf-8')


def run_period(begin_month, begin_day, end_month, end_day, hour1=False):
    return {'name': 'CLK03 Unit', 'begin_month': begin_month, 'begin_day_of_month': begin_day,
            'begin_year': 2013, 'end_month': end_month, 'end_day_of_month': end_day,
            'end_year': 2013, 'start_weekday': (date(2013, begin_month, begin_day).weekday() + 1) % 7 + 1,
            'first_hour_interpolation_starting_values': 'Hour1' if hour1 else 'Hour24',
            'use_weather_holidays': False, 'use_weather_dst': False,
            'apply_weekend_holiday_rule': False, 'treat_weather_as_actual': False}


def idf_for(rp):
    first = rp['first_hour_interpolation_starting_values']
    weekday = ['Sunday', 'Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday'][rp['start_weekday'] - 1]
    text = ('Version,26.1;\nTimestep,4;\n'
            'Building,CLK03 Unit,0,Suburbs,0.04,0.4,FullExterior,25,6;\n'
            'GlobalGeometryRules,UpperLeftCorner,Counterclockwise,World;\n'
            f"RunPeriod,CLK03 Unit,{rp['begin_month']},{rp['begin_day_of_month']},2013,"
            f"{rp['end_month']},{rp['end_day_of_month']},2013,{weekday},No,No,No,Yes,Yes,No,{first};\n")
    return text.encode('utf-8')


def lifecycle_operations(total_days, warmup_days=0):
    result = [{'id': 'get-next-environment', 'kind': 'GetNextEnvironment', 'caller': {}}]
    for number in range(warmup_days):
        result.append({'id': f'warmup-{number + 1}', 'kind': 'InitializeWeather',
                       'caller': {'BeginEnvrnFlag': number == 0, 'BeginDayFlag': True,
                                  'WarmupFlag': True, 'DayOfSim': 1}})
    for number in range(1, total_days + 1):
        result.append({'id': f'run-day-{number}', 'kind': 'InitializeWeather',
                       'caller': {'BeginEnvrnFlag': number == 1 and warmup_days == 0,
                                  'BeginDayFlag': True, 'WarmupFlag': False, 'DayOfSim': number}})
    result.append({'id': 'end-environment-rewind', 'kind': 'InitializeWeather',
                   'caller': {'BeginEnvrnFlag': False, 'BeginDayFlag': False,
                              'WarmupFlag': False, 'DayOfSim': total_days,
                              'EndDayFlag': True, 'EndEnvrnFlag': True,
                              'HourOfDay': 24, 'TimeStep': 4}})
    return result


def main():
    scope = json.loads((ROOT / 'energyplus_porting_plan/contracts/scope.json').read_text(encoding='utf-8'))
    calendar = json.loads((ROOT / 'energyplus_porting_plan/contracts/CLK-01-cases.json').read_text(encoding='utf-8'))
    fixed = []
    for case_id in ['A-24H', 'A-72H', 'B-BOTH-24H']:
        source = next(c for c in calendar['calendar_request']['cases'] if c['case_id'] == case_id)
        days = 3 if source['duration'] == '72H' else 1
        fixed.append({'id': case_id, 'input': source['source_metadata']['input'],
                      'weather': source['source_metadata']['weather'], 'run_period': source['run_period'],
                      'time_steps_per_hour': 4, 'prepared_environment_lane': True,
                      'operations': lifecycle_operations(days, 2)})
    epw = put('decoy-three-days.epw', make_epw([(1999, 6, 30), (2004, 7, 1), (2007, 7, 2)]))
    diagnostics = []
    missing = put('missing-target-complete-days.epw', make_epw([(1999, 6, 30), (2007, 7, 2)]))
    for identifier, start, end, first, warmup in [
        ('DECOY-OFFSET-HOUR24', (7, 1), (7, 2), False, 2),
        ('DECOY-OFFSET-HOUR1', (7, 1), (7, 2), True, 0),
        ('FIRST-DAY-THREE-DAYS', (6, 30), (7, 2), False, 0),
        ('MISSING-MATCH-DEFINED-FAILURE', (7, 1), (7, 1), False, 0),
        ('OUTSIDE-DATA-PERIOD', (7, 3), (7, 3), False, 0)]:
        rp = run_period(*start, *end, first)
        input_ref = put(identifier.lower() + '-03.idf', idf_for(rp))
        weather_ref = missing if identifier == 'MISSING-MATCH-DEFINED-FAILURE' else epw
        diagnostics.append({'id': identifier, 'input': input_ref, 'weather': weather_ref, 'run_period': rp,
                            'time_steps_per_hour': 4, 'prepared_environment_lane': True,
                            'operations': lifecycle_operations(3 if start == (6, 30) else 2 if end == (7, 2) else 1, warmup)})
    duplicate = put('duplicate-target-complete-days.epw', make_epw([(1999, 6, 30), (2004, 7, 1), (2010, 7, 1), (2007, 7, 2)]))
    for identifier, source_epw, backspace in [('DIRECT-READDAY-REPLAY', duplicate, False),
                                               ('DIRECT-READDAY-BACKSPACE-ONE-RECORD', epw, True)]:
        rp = run_period(7, 1, 7, 1)
        input_ref = put(identifier.lower() + '-04.idf', idf_for(rp))
        calls = [{'id': 'get-next-environment', 'kind': 'GetNextEnvironment', 'caller': {}},
                 {'id': 'read-first', 'kind': 'ReadWeatherForDay', 'day_to_read': 1,
                  'backspace_after_read': backspace, 'caller': {}}]
        if not backspace:
            calls.append({'id': 'handoff-with-live-input-stream', 'kind': 'UpdateWeatherData', 'caller': {}})
            calls.append({'id': 'replay-search-from-current-position', 'kind': 'ReadWeatherForDay',
                          'day_to_read': 1, 'backspace_after_read': False, 'caller': {}})
        diagnostics.append({'id': identifier, 'input': input_ref, 'weather': source_epw,
                            'run_period': rp, 'time_steps_per_hour': 4,
                            'prepared_environment_lane': True, 'operations': calls})
    request = {'schema': 'clk03-helper-cases.draft.v1', 'expected_values_supplied': False,
               'expected_exits_supplied': False, 'fixed_scope': ref(ROOT / 'energyplus_porting_plan/contracts/scope.json'),
               'handoff_sequences': [{'id': 'CTOR-COPY-DEFAULTS', 'time_steps_per_hour': 4,
                                     'initial': {}, 'operations': [{'id': 'copy-defaults',
                                     'kind': 'UpdateWeatherData', 'before': {}}]},
                                     handoff_case('COPY-BEGIN-ENV-POSITIVE-HOLIDAY', True, 8),
                                     handoff_case('COPY-NORMAL-ZERO-HOLIDAY', False, 0),
                                     handoff_case('COPY-NORMAL-NEGATIVE-HOLIDAY', False, -1)],
               'weather_sequences': fixed + diagnostics,
               'native_preparation': {
                   'lane': 'selected-prepared-environment',
                   'genuine_calls': ['InputProcessor::processInput', 'SetupInterpolationValues',
                       'OpenWeatherFile', 'CloseWeatherFile', 'ReadUserWeatherInput',
                       'AllocateWeatherData', 'ResolveLocationInformation', 'CheckLocationValidity'],
                   'GetEnvironmentFirstCall': False, 'GetBranchInputOneTimeFlag': False,
                   'WaterMainsParameterReport': False, 'BeginSimFlag': False,
                   'DoWeathSim': True, 'DoDesDaySim': False,
                   'Envrn_before_GetNextEnvironment': 'actual-native-TotDesDays',
                   'TimeStepFraction': bits(0.25), 'TimeStepZone': bits(0.25),
                   'registered_whole_simulation_initialization_claimed': False,
                   'eio_sink': 'native-output-stringstream', 'error_sink': 'native-output-stringstream'},
               'production_case_ids': ['A-24H', 'A-72H', 'B-BOTH-24H'],
               'draft_not_frozen': True, 'scientific_execution_performed': False}
    binding = put('helper-request-draft-07.json', (json.dumps(request, indent=2, ensure_ascii=False) + '\n').encode('utf-8'))
    print(json.dumps({'draft_request': binding, 'handoff_sequences': len(request['handoff_sequences']),
                      'weather_sequences': len(request['weather_sequences']),
                      'scope_cases_unchanged': len(scope['cases']), 'expected_values_supplied': False}))


if __name__ == '__main__':
    main()
