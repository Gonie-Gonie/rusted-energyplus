"""Read actual live CLK-03 owners; original data is comparison-only.

Processed weather physics is not paired. Whole-buffer copies and received
operands are checked within their actual Rust owner, including every slot.
"""
from __future__ import annotations

from collections import Counter
import hashlib
import re

from geo03_provenance import exact, integer, require
from clk03_unit_state import DAY_INTS, DAY_REALS, ENV_DISCRETE, HEADER_KEYS, STREAM, WEATHER_REALS, scalar

TRACE_NAME = 'clk03-weather-day-production-trace.json'
COPIED = {'DryBulb':'dry_bulb_c','DewPoint':'dew_point_c','RelHum':'relative_humidity_percent',
          'AtmPress':'atmospheric_pressure_pa','IRHoriz':'horizontal_infrared_radiation_wh_per_m2',
          'GLBHoriz':'global_horizontal_radiation_wh_per_m2','DirectRad':'direct_normal_radiation_wh_per_m2',
          'DiffuseRad':'diffuse_horizontal_radiation_wh_per_m2','WindDir':'wind_direction_deg','WindSpeed':'wind_speed_m_per_s'}
DATES = {'WYear':'year','WMonth':'month','WDay':'day','WHour':'hour','WMinute':'minute'}
NATIVE_WEATHER = 'Envrn NumOfEnvrn TotRunPers TotRunDesPers NumDataPeriods NumIntervalsPerHour NumSpecialDays LeapYearAdd RptDayType CurDayOfWeek curSimDayForEndOfRunPeriod GetBranchInputOneTimeFlag GetEnvironmentFirstCall FirstCall WaterMainsParameterReport LastHourSet WeatherFileExists DatesShouldBeReset StartDatesCycleShouldBeReset Jan1DatesShouldBeReset RPReadAllWeatherData UseDaylightSaving UseSpecialDays DaylightSavingIsActive ReadEPlusWeatherCurTime TimeStepFraction IsRainThreshold'.split()
ENV_PAIRED = [key for key in ENV_DISCRETE if key != 'RunPeriodStartDayOfWeek']
PHASE = re.compile(r'(Run|Warmup) \{ day: ([1-9][0-9]*) \}\Z')


def selected_policy():
    return {
        'production_cases':['A-24H','A-72H','B-BOTH-24H'], 'trace_levels':['Full','Summary'],
        'whole_copy':'actual Rust Today full11/full17 against actual incoming Tomorrow at observed boundary',
        'consumer_receipt':'every raw field against preserved CLK02 native raw row by external exact input-byte map; literal10 record projections; full17 slot against actual Today',
        'lifecycle_native_selection':{'environment':ENV_PAIRED,'weather':NATIVE_WEATHER,'daily':DAY_INTS,'header':HEADER_KEYS,'stream':STREAM,'context':['current_cycle','set_week_days']},
        'caller_sensitive_pairing':'Initialize FirstCall needs equal incoming FirstCall and BeginSimFlag; EndMonthFlag needs equal BeginSimFlag and incoming EndMonthFlag plus equal FirstCall if BeginSim active. Different declared caller/history is unavailable, not equal-value PASS.',
        'source_literal_initialization':'actual before/after FirstCall branch; EndMonth reset observed only where later BeginDay month-end write does not hide its final value, no invented intermediate write events',
        'clock_selection':'actual caller hour/timestep/phase/civil index and end flags; no inferred native thermal callbacks',
        'source_order':'one GetNext; one first-day search; actual day reads; no last-day prefetch; actual terminal rewind',
        'first_phase_token':'one initial BeginEnvrn/BeginDay transport, no repeated first-loop transport/prefetch',
        'preview_policy':'available preview is independent metadata/evidence; live cursor and Today are thermal payload owners',
        'unpaired':['processed numeric/boolean physics generation','daily solar and shadow coefficients generation','missing/range cache/counters except raw WeathCodes','liquid sentinel normalization','native thermal/warmup callbacks','full native design-day/Interpolation/DST registry'],
        'warmup_policy':'observed only, no production warmup inserted; no warmup observation is unavailable, never PASS',
        'native_internal_record_index_claimed':False, 'unavailable_counted_as_PASS':False,
    }


def weather_shape(value, label):
    require(type(value) is dict and set(value) == set(WEATHER_REALS+['IsRain','IsSnow']), 'Full17 weather fields required: '+label)
    require(type(value['IsRain']) is bool and type(value['IsSnow']) is bool, 'Typed weather bools required')
    for key in WEATHER_REALS: scalar(value[key],label+'/'+key)


def grid_shape(value, label):
    require(type(value) is dict and set(value) == {'allocated','time_steps','hours','slots'}
            and value['allocated'] is True and type(value['time_steps']) is int and value['time_steps'] == 4
            and type(value['hours']) is int and value['hours'] == 24 and type(value['slots']) is list
            and len(value['slots']) == 96, 'Actual allocated 4x24 carrier required: '+label)
    for index,row in enumerate(value['slots']): weather_shape(row,label+'/'+str(index))


def owner_shape(value,label):
    require(value['native_internal_record_index_claimed'] is False, 'Native local index may not be fabricated')
    for key in 'BeginSimFlag BeginEnvrnFlag BeginDayFlag BeginHourFlag BeginTimeStepFlag EndDayFlag EndHourFlag EndEnvrnFlag EndDesignDayEnvrnsFlag WarmupFlag DoWeathSim DoDesDaySim DoOutputReporting'.split():
        require(type(value['global'][key]) is bool,'Typed actual caller branch flag required: '+label+'/'+key)
    require(type(value['weather']['FirstCall']) is bool and type(value['environment']['EndMonthFlag']) is bool
            and type(value['environment']['EndYearFlag']) is bool,'Typed source-sensitive owner flags required: '+label)
    for key in ['today_variables','tomorrow_variables']:
        row=value[key]
        require(type(row) is dict and set(row) == set(DAY_INTS+DAY_REALS)
                and all(type(row[field]) is int for field in DAY_INTS),'Full11 daily fields required')
        for field in DAY_REALS: scalar(row[field],label+'/'+key+'/'+field)
    for key in ['today_values','tomorrow_values']: grid_shape(value[key],label+'/'+key)
    for key in ['last_hour','next_hour']: weather_shape(value[key],label+'/'+key)
    weather_shape(value['missing_values']['base'],label+'/missing_values/base')
    require(len(value['missing_values']['extra_reals']) == 4 and type(value['missing_values']['DaysLastSnow']) is int,'Full extended missing storage required')
    for index,row in enumerate(value['missing_values']['extra_reals']): scalar(row,label+'/missing/'+str(index))
    for key in ['missed_counts','out_of_range_counts']:
        rows=value[key]['declaration_order_integer_values']
        require(type(rows) is list and len(rows)==19 and all(type(row) is int for row in rows),'Full19 typed counts required')
    for key in ['actual_cursor_byte','actual_line_read_count','actual_Rust_interpret_count']: integer(value[key],label+'/'+key)
    for key in ['available','errors_found','print_environment_stamp','set_week_days']: require(type(value[key]) is bool,'Typed owner availability required')
    integer(value['current_cycle'],label+'/current_cycle')
    for key in STREAM:
        v=value['stream'][key]
        require((v is None or type(v) is int and v>=0) if key=='position_byte' else type(v) is bool,'Typed stream flag/position required')
    require(value['stream']['position_available'] is (value['stream']['position_byte'] is not None),'Actual position availability differs')


def native_selected(check,actual,native,label,actual_before,native_before,kind):
    env_fields=list(ENV_PAIRED);weather_fields=list(NATIVE_WEATHER)
    if kind=='InitializeWeather':
        require(type(native_before['global']['BeginSimFlag']) is bool and type(native_before['weather']['FirstCall']) is bool
                and type(native_before['environment']['EndMonthFlag']) is bool,'Typed original caller-sensitive flags required')
        sim_equal=exact(actual_before['global']['BeginSimFlag'],native_before['global']['BeginSimFlag'])
        first_equal=exact(actual_before['weather']['FirstCall'],native_before['weather']['FirstCall'])
        eligibility={'weather/FirstCall':sim_equal and first_equal,
            'environment/EndMonthFlag':sim_equal and exact(actual_before['environment']['EndMonthFlag'],native_before['environment']['EndMonthFlag'])
                and (not actual_before['global']['BeginSimFlag'] or first_equal)}
        for path,eligible in eligibility.items():
            if not eligible:
                (weather_fields if path.startswith('weather/') else env_fields).remove(path.split('/')[1])
                check.unavailable_boundaries.append({'location':label+'/'+path,'reason':'different-declared-BeginSim-or-incoming-source-sensitive-state',
                    'actual_before_BeginSimFlag':actual_before['global']['BeginSimFlag'],
                    'native_before_BeginSimFlag':native_before['global']['BeginSimFlag'],
                    'counted_as_PASS':False})
    check.selected(actual['environment'],native['environment'],env_fields,label+'/environment','native_discrete_lifecycle')
    check.selected(actual['weather'],native['weather'],weather_fields,label+'/weather','native_selected_control')
    for key in ['today_variables','tomorrow_variables']:
        check.selected(actual[key],native[key],DAY_INTS,label+'/'+key,'native_daily_discrete')
    check.selected(actual['header'],native['header'],HEADER_KEYS,label+'/header','native_raw_header')
    check.selected(actual['stream'],native['stream'],STREAM,label+'/stream','native_cursor')
    rows=[row for row in native['environments'] if row['KindOfEnvrn']==3]
    require(len(rows)==1,'Actual source selected weather environment required')
    for key,nativekey in [('current_cycle','CurrentCycle'),('set_week_days','SetWeekDays')]:
        check.value(actual[key],rows[0][nativekey],label+'/'+key,'native_selected_environment')


def copy_boundary(check,incoming,after,label):
    check.value(after['today_variables'],incoming['tomorrow_variables'],label+'/daily11','actual_whole_transport')
    check.value(after['today_values'],incoming['tomorrow_values'],label+'/grid17x96','actual_whole_transport')
    projection={'DayOfYear':'DayOfYear','Year':'Year','Month':'Month','DayOfMonth':'DayOfMonth','DayOfWeek':'DayOfWeek',
                'HolidayIndex':'HolidayIndex','DSTIndicator':'DaylightSavingIndex',**{key:key for key in DAY_REALS}}
    for key,source in projection.items():
        check.value(after['environment'][key],incoming['tomorrow_variables'][source],label+'/environment/'+key,'actual_whole_transport')


def byte_map(declared,weather_bytes,native_records):
    require(declared['header_line_count']==8 and declared['input_byte_length']==len(weather_bytes)
            and len(declared['record_line_map'])==len(native_records)==8760,'Whole frozen byte map and actual native raw inventory required')
    lookup={}
    for index,row in enumerate(declared['record_line_map']):
        require(row['source_record_index']==index and row['start_byte']<row['end_byte']<=len(weather_bytes),'External source ordinal/byte boundary differs')
        source_bytes=weather_bytes[row['start_byte']:row['end_byte']]
        require(hashlib.sha256(source_bytes).hexdigest()==row['input_row_sha256'], 'Frozen whole-range input byte hash differs')
        # std::getline removes one LF delimiter. IOFiles.cc:97-100 removes
        # exactly one trailing CR; all other bytes remain comparison operands.
        delivered=source_bytes[:-1] if source_bytes.endswith(b'\n') else source_bytes
        delivered=delivered[:-1] if delivered.endswith(b'\r') else delivered
        native_bytes=native_records[index]['input_line_utf8'].encode('utf8')
        require(hashlib.sha256(native_bytes).hexdigest()==native_records[index]['input_line_sha256']
                and native_bytes==delivered,'Actual native delivered-line byte identity differs')
        require(native_records[index]['stream_before_read']['position_byte']==row['start_byte']
                and native_records[index]['stream_after_read']['position_byte']==row['end_byte'],
                'Actual native read cursor differs from full source-byte range')
        pair=(row['start_byte'],row['end_byte'])
        require(pair not in lookup,'Duplicate external byte range')
        lookup[pair]=(index,native_records[index]['after'])
    return lookup


def observe(check,trace,case,native,raw_native,mapped,weather_bytes):
    label=case['case_id']
    require(trace['schema']=='clk03-weather-day-production-trace.v1'
            and trace['capture_source']=='actual-live-Rust-weather-cursor-daily-owner-and-owned-thermal-operands'
            and trace['observer_supplies_inputs'] is False and trace['observer_recomputes_weather'] is False
            and trace['eager_preview_values_supply_production_weather'] is False
            and trace['consumer_repeated_identities_deduplicated'] is False
            and trace['physical_producer_numerical_parity_claimed'] is False
            and trace['raw_source_year_equals_civil_calendar_year_claimed'] is False
            and trace['thread_coverage']=='collecting-thread-only'
            and trace['complete_on_collecting_thread'] is True and trace['truncation_reason'] is None,'Actual complete passive live trace required')
    ops,consumers=trace['operations_completion_order'],trace['consumers']
    require(trace['total_operation_count']==trace['retained_operation_count']==len(ops)>0
            and trace['total_consumer_count']==trace['retained_consumer_count']==len(consumers)>0
            and trace['observation_limit_per_series']==10000,'Retained actual operation/consumer counts differ')
    day_owner={};first_read=None;day_reads=[];begin_count=Counter();end_ops=[]
    native_ops={row['id']:row for row in native['operations']}
    require(ops[0]['kind']=='GetNextEnvironment' and sum(row['kind']=='GetNextEnvironment' for row in ops)==1,'One genuine initial environment admission required')
    for index,row in enumerate(ops):
        loc=f'{label}/operations/{index}'
        require(row['sequence']==index+1 and row['kind'] in ['GetNextEnvironment','ReadEPlusWeatherForDay','InitializeWeather']
                and row['outcome']=={'returned':True,'source_fatal':False,'error':None},'Actual ordered returned weather calls required')
        for phase in ['before','after']: owner_shape(row[phase],loc+'/'+phase)
        before,after=row['before'],row['after'];caller=before['global']
        if row['kind']=='GetNextEnvironment':
            original=native_ops['get-next-environment']
            native_selected(check,after,original['after'],loc,before,original['before'],row['kind'])
            check.compare(before['stream']['is_open'],False,loc+'/prepared_stream_closed','source_preparation')
            check.compare(after['actual_Rust_interpret_count'],0,loc+'/live_owner_no_eager_record_parse','live_source_order')
            continue
        if row['kind']=='ReadEPlusWeatherForDay':
            day_reads.append(row)
            if first_read is None:first_read=row
            continue
        if caller['BeginDayFlag']:
            key=(caller['WarmupFlag'],caller['DayOfSim']);begin_count[key]+=1
            incoming=first_read['after'] if caller['BeginEnvrnFlag'] else before
            require(incoming is not None,'Initial source read must precede first transport')
            copy_boundary(check,incoming,after,loc)
            day_owner[key]=after
            if not caller['WarmupFlag']:
                original=native_ops[f'run-day-{caller["DayOfSim"]}']
                native_selected(check,after,original['after'],loc,before,original['before'],row['kind'])
        else:
            for key in ['today_variables','today_values','tomorrow_variables','tomorrow_values']:
                check.value(after[key],before[key],loc+'/'+key,'non_begin_day_owner_retention')
        active=caller['BeginSimFlag'] and before['weather']['FirstCall']
        check.compare(after['weather']['FirstCall'],False if active else before['weather']['FirstCall'],loc+'/actual_FirstCall_literal_branch','actual_source_literal_branch')
        if active:
            month_end=caller['BeginDayFlag'] and after['environment']['DayOfMonth']==after['header']['EndDayOfMonth'][after['environment']['Month']-1]
            if not month_end:
                check.compare(after['environment']['EndMonthFlag'],False,loc+'/actual_EndMonthFlag_literal_reset','actual_source_literal_branch')
            else:
                check.unavailable_boundaries.append({'location':loc+'/interior_EndMonthFlag_reset','reason':'later-source-BeginDay-month-end-write-hides-intermediate-reset','counted_as_PASS':False})
        if caller['EndEnvrnFlag']:
            end_ops.append(row)
            check.selected(after['stream'],native_ops['end-environment-rewind']['after']['stream'],STREAM,loc+'/terminal_stream','source_terminal_rewind')
    require(first_read is not None,'Actual first-day source read required')
    env_initial=[row for row in ops if row['kind']=='InitializeWeather' and row['before']['global']['BeginEnvrnFlag']]
    check.compare(len(env_initial),1,label+'/one_initial_environment_transport','first_phase_token')
    for key,count in begin_count.items():check.compare(count,1,label+'/one_day_transport/'+str(key),'first_phase_token')
    hours=72 if case['duration']=='72H' else 24;days=hours//24
    check.compare(len(day_reads),days,label+'/no_extra_or_terminal_prefetch','actual_read_order')
    require(len(end_ops)==1,'One actual terminal rewind before final consumption required')
    native_last=native_ops[f'run-day-{days}']['after']
    check.selected(end_ops[0]['before']['stream'],native_last['stream'],STREAM,label+'/last_day_no_prefetch_cursor','native_cursor')
    lookup=byte_map(mapped,weather_bytes,raw_native['fixed_epw']['records'])
    seen=set();phases=Counter();run_identities=set();ordered_run=[];phase_order=[];phase_rows={};last_consumer=None
    for index,row in enumerate(consumers):
        loc=f'{label}/consumers/{index}';require(row['sequence']==index+1,'Actual consumer sequence differs')
        match=PHASE.fullmatch(row['phase']);require(match is not None,'Actual phase label malformed')
        phase,day=match[1],int(match[2]);warmup=phase=='Warmup';phases[row['phase']]+=1
        if not phase_order or phase_order[-1]!=row['phase']:phase_order.append(row['phase'])
        record=integer(row['record_index'],'civil hour');step=integer(row['timestep'],'zone timestep',1)
        require(step<=4 and record//24+1==(1 if warmup else day),'Actual civil identity outside active owner')
        phase_rows.setdefault(row['phase'],[]).append((record,step))
        owner=day_owner.get((warmup,day));require(owner is not None,'Consumer lacks actual entered Today owner')
        check.value(row['received_Today_slot'],owner['today_values']['slots'][(record%24)*4+step-1],loc+'/full17_Today','actual_consumer_receipt')
        weather_shape(row['received_Today_slot'],loc)
        provenance=row['actual_input_byte_range'];pair=(provenance['start_byte'],provenance['end_byte'])
        require(pair in lookup and type(provenance['line_read_attempt']) is int and provenance['line_read_attempt']>0,'Consumer raw byte range is not an actual supplied row')
        source_index,original_raw=lookup[pair];seen.add(source_index)
        check.value(row['received_raw'],original_raw,loc+'/native_raw_row','native_raw_input_identity')
        projected=row['received_hourly_record']
        check.selected(projected['source_date_fields'],{target:original_raw['date_fields'][source] for source,target in DATES.items()},list(DATES.values()),loc+'/record_dates','literal_hourly_record')
        for source,target in COPIED.items():check.value(projected['legacy_fields'][target],row['received_raw']['mandatory_reals'][source],loc+'/record/'+target,'literal_hourly_record')
        for group in ['legacy_fields']:
            for key,value in projected[group].items():scalar(value,loc+'/'+key)
        sample=row['received_sample'];require(sample['record_index']==record and sample['zone_timestep']==step,'Actual sample identity differs')
        for key,value in sample.items():
            if key not in ['record_index','zone_timestep']:scalar(value,loc+'/sample/'+key)
        for key,value in row['shadowing_metadata'].items():scalar(value,loc+'/shadow/'+key)
        scalar(row['local_hour'],loc+'/local_hour')
        clock=row['caller_state'];check.compare(clock['HourOfDay'],record%24+1,loc+'/HourOfDay','actual_caller_clock')
        check.compare(clock['TimeStep'],step,loc+'/TimeStep','actual_caller_clock')
        check.compare(clock['WarmupFlag'],warmup,loc+'/WarmupFlag','actual_caller_clock')
        check.compare(clock['DayOfSim'],day,loc+'/DayOfSim','actual_caller_clock')
        if not warmup:run_identities.add((record,step));ordered_run.append((record,step))
        last_consumer=row
    expected_run=[(hour,step) for hour in range(hours) for step in range(1,5)]
    # Preserve actual repeated initial context without inventing a thermal role.
    # Only the one hour1/TS1 prefix from initial preparation is admitted.
    initial_warmup=env_initial[0]['before']['global']['WarmupFlag']
    expected_run_order=expected_run
    if not initial_warmup and len(ordered_run)==len(expected_run)+1:expected_run_order=[(0,1)]+expected_run
    check.compare(ordered_run,expected_run_order,label+'/full_actual_run_timestep_order','actual_consumer_order')
    check.compare(sorted(run_identities),expected_run,label+'/full_actual_run_timestep_coverage','actual_consumer_coverage')
    warmup_days=sorted(day for warmup,day in day_owner if warmup)
    check.compare(warmup_days,list(range(1,len(warmup_days)+1)),label+'/warmup_iteration_order','actual_phase_order')
    expected_phases=[f'Warmup {{ day: {day} }}' for day in warmup_days]+[f'Run {{ day: {day} }}' for day in range(1,days+1)]
    check.compare(phase_order,expected_phases,label+'/actual_phase_blocks_order','actual_phase_order')
    for day in warmup_days:
        rows=phase_rows[f'Warmup {{ day: {day} }}'];expected=[(hour,step) for hour in range(24) for step in range(1,5)]
        if day==1 and len(rows)==97:expected=[(0,1)]+expected
        check.compare(rows,expected,label+'/warmup_order/'+str(day),'actual_consumer_order')
    check.compare(last_consumer['caller_state']['EndEnvrnFlag'],True,label+'/final_consumer_after_terminal_rewind','actual_caller_clock')
    check.compare(len(seen),hours,label+'/actual_selected_raw_hours','native_raw_input_identity')
    return {'case_id':label,'actual_operation_count':len(ops),'actual_consumer_count':len(consumers),
            'actual_read_day_count':len(day_reads),'actual_raw_source_ordinals_external':sorted(seen),
            'actual_phases':dict(phases),'warmup_observed':any(key[0] for key in day_owner),
            'warmup_unavailable_counted_as_PASS':False,'native_internal_record_index_claimed':False,
            'processed_physics_paired':False,'preview_numerical_operands_paired':False}
