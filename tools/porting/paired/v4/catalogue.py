"""Frozen positive-OA science catalogue. Configurations cannot supply code."""
import csv,io,json,math,re,traceback
from pathlib import Path
from . import common116 as c
from . import contracts as p

def typed_dom(value):
    kind=type(value)
    if value is None:return ('null',)
    if kind is bool:return ('bool',value)
    if kind is int:return ('int',value)
    if kind is float:
        assert math.isfinite(value)
        return ('float',c.bits(value))
    if kind is str:return ('string',value)
    if kind is list:return ('array',tuple(typed_dom(v) for v in value))
    assert kind is dict and all(type(k) is str for k in value)
    return ('object',tuple((k,typed_dom(value[k])) for k in sorted(value)))



def table(path,recipe):
    lines=path.read_text(encoding='utf8').splitlines()
    assert lines and lines[0].startswith('Program Version:,')
    rows=list(csv.reader(io.StringIO('\n'.join(lines))))
    contract=recipe['actual_summary_contract']
    reports=[r for r in rows if r and r[0]=='REPORT:']
    assert reports==[['REPORT:',contract['report']]],reports
    assert [r for r in rows if r and r[0]=='FOR:']==[['FOR:',contract['scope']]]
    assert rows.count([contract['table']])==1
    headers=[r for r in rows if r[2:]==contract['columns']]
    assert len(headers)==1 and headers[0][:2]==['','']
    at=rows.index(headers[0]);body=[]
    for row in rows[at+1:]:
        if not row:break
        body.append(row)
    assert len(body)==1 and len(body[0])==7 and body[0][:2]==['','114 ADAPTIVE OCCUPANT'],body
    cells=body[0][2:]
    assert all(math.isfinite(float(v)) and float(v)>=0 for v in cells)
    return dict(artifact=c.ref(path),rows=rows,body=body,nonnegative_all_five_hours=[float(v) for v in cells],lines_except_version=lines[1:],version_line=lines[0])



def statuses(parsed,bindings):
    wanted=sorted([b for b in bindings if 119<=b['index']<123],key=lambda b:b['index']);assert len(wanted)==4
    for b in wanted:assert b['producer']['stage']=='Zone' and b['producer']['store']=='Average' and b['units']==('' if b['index']<121 else 'C')
    occ=[b for b in bindings if b['key'].upper()=='INTERNAL GAIN SCHEDULE' and b['name']=='Schedule Value'];assert len(occ)==1
    arrays=[parsed['values'][b['report_id']] for b in wanted]+[parsed['values'][occ[0]['report_id']]];allowed=[(-1,-1),(0,0),(0,1),(1,1)];counts={pair:0 for pair in allowed};first={};applicable_occupied=unoccupied=out_of_range=0
    for i,values in enumerate(zip(*arrays,strict=True)):
        a,b,mean,comfort,occupancy=map(float,values);assert all(math.isfinite(x) for x in [a,b,mean,comfort,occupancy]);assert a in [-1,0,1] and b in [-1,0,1];pair=(int(a),int(b));assert pair in allowed
        applicable=10.0<=mean<=33.5
        if not applicable or occupancy==0:assert pair==(-1,-1)
        else:assert occupancy>0 and pair!=(-1,-1);applicable_occupied+=1
        if occupancy==0:unoccupied+=1
        if not applicable:assert comfort==-1.0;out_of_range+=1
        else:assert abs(comfort-(0.31*mean+17.8))<=wanted[3]['policy']['atol']==1e-6
        counts[pair]+=1;first.setdefault(str(pair),dict(sample=i,clock=parsed['clocks'][i],context=parsed['clock_context'][i],occupancy=occupancy,running_mean=mean,comfort=comfort,values=list(pair)))
    return dict(status_report_ids=[b['report_id'] for b in wanted[:2]],occupancy_report_id=occ[0]['report_id'],pair_counts={str(k):value for k,value in counts.items()},first_observations=first,unobserved_pairs=[list(k) for k,value in counts.items() if value==0],applicable_occupied_samples=applicable_occupied,unoccupied_samples=unoccupied,out_of_range_samples=out_of_range,counts_are_observations_not_targets=True,qualification='Actual public applicability/occupancy/status and comfort-temperature formula; no old distribution target or private operative-temperature/substep reconstruction.')



def cen_statuses(parsed,bindings):
    wanted=sorted([b for b in bindings if 123<=b['index']<128],key=lambda b:b['index'])
    assert len(wanted)==5
    for i,b in enumerate(wanted):
        assert b['producer']['stage']=='Zone' and b['producer']['store']=='Average'
        assert b['units']==('' if i<3 else 'C')
    occ=[b for b in bindings if b['key'].upper()=='INTERNAL GAIN SCHEDULE' and b['name']=='Schedule Value'];assert len(occ)==1
    arrays=[parsed['values'][b['report_id']] for b in wanted]+[parsed['values'][occ[0]['report_id']]]
    allowed=[(-1,-1,-1),(0,0,0),(0,0,1),(0,1,1),(1,1,1)]
    counts={v:0 for v in allowed};first={};applicable_occupied=unoccupied=out_of_range=low_region=0
    for i,values in enumerate(zip(*arrays,strict=True)):
        a,b,d,mean,comfort,occupancy=map(float,values)
        assert all(math.isfinite(v) for v in [a,b,d,mean,comfort,occupancy])
        assert all(v in [-1,0,1] for v in [a,b,d]);triple=(int(a),int(b),int(d));assert triple in counts
        applicable=10.0<=mean<=30.0
        if not applicable or occupancy==0:assert triple==(-1,-1,-1)
        else:
            assert occupancy>0 and triple!=(-1,-1,-1);applicable_occupied+=1
        if occupancy==0:unoccupied+=1
        if not applicable:assert comfort==-1.0;out_of_range+=1
        else:
            assert abs(comfort-(0.33*mean+18.8))<=1e-6
            if mean<15:low_region+=1
        counts[triple]+=1
        first.setdefault(str(triple),dict(sample=i,clock=parsed['clocks'][i],context=parsed['clock_context'][i],occupancy=occupancy,running_mean=mean,comfort=comfort,values=list(triple)))
    return dict(status_report_ids=[b['report_id'] for b in wanted[:3]],triple_counts={str(k):v for k,v in counts.items()},first_observations=first,unobserved_triples=[list(k) for k,v in counts.items() if v==0],applicable_occupied_samples=applicable_occupied,unoccupied_samples=unoccupied,out_of_range_samples=out_of_range,mean_10_to_15_samples=low_region,counts_are_observations_not_targets=True,coverage_limitation='No requirement or claim that all triples, low region, out-of-range or threshold boundaries were stimulated. Public sample checks do not expose private substep states.')



def scheduled_output(parsed,bindings,recipe):
    observations=[]
    for control in recipe['controls']+recipe['additional_output_controls']:
        found=[b for b in bindings if b['key'].upper()==control['node'].upper() and b['name']==control['variable']]
        schedules=[b for b in bindings if b['key'].upper()==control['schedule'].upper() and b['name']=='Schedule Value']
        assert len(found)==len(schedules)==1;bound=found[0]
        assert bound['units']==control['units'] and bound['producer']['stage']=='HVAC' and bound['producer']['store']=='Average'
        assert all(bound['policy'][k]==control[k] for k in ['atol','rtol','rmse_max'])
        actual=parsed['values'][bound['report_id']];expected=parsed['values'][schedules[0]['report_id']];assert len(actual)==len(expected)==480
        values=list(map(float,actual));scheduled=list(map(float,expected));assert all(math.isfinite(v) for v in values+scheduled)
        assert all(abs(a-b)<=control['atol'] for a,b in zip(values,scheduled,strict=True))
        if control['units']=='C':assert all(v>0 for v in values) and len(set(values))>1
        else:
            assert all(min(abs(v),abs(v-1.0))<=control['atol'] for v in values)
            assert any(abs(v)<=control['atol'] for v in values) and any(abs(v-1.0)<=control['atol'] for v in values)
        observations.append(dict(control=control,report_id=bound['report_id'],schedule_report_id=schedules[0]['report_id'],samples=480,distinct_actual_values=sorted(set(values)),max_same_clock_schedule_delta=max(abs(a-b) for a,b in zip(values,scheduled,strict=True))))
    assert len(observations)==18 and sum(v['control']['advanced_report_required'] for v in observations)==6
    return observations



def public_return_flow(parsed,bindings,recipe):
    contract=c.read(c.auth(recipe['public_return_flow_gate']));wanted=contract['bindings'];assert [v['policy_index'] for v in wanted]==[5,66,69,151,72]
    selected=[];arrays=[]
    for item in wanted:
        matches=[b for b in bindings if b['index']==item['policy_index']];assert len(matches)==1;bound=matches[0]
        assert bound['policy']==item['rule'] and bound['producer']['store']=='Average' and bound['producer']['stage']==('Zone' if bound['index']==5 else 'HVAC')
        values=list(map(float,parsed['values'][bound['report_id']]));assert len(values)==480 and all(math.isfinite(x) for x in values);selected.append(bound);arrays.append(values)
    limits=selected[-1]['policy'];newlimits=selected[3]['policy'];assert limits['atol']==limits['rmse_max']==1e-9 and limits['rtol']==0.0
    assert all(newlimits[k]==limits[k] for k in ['units','frequency','sampling_stage','atol','rtol','rmse_max']) and newlimits['key']==recipe['new_return_node']
    maximum=second_maximum=0.0;off=on=positive=0;first={}
    for index,(fraction,inlet,air,returned,second) in enumerate(zip(*arrays,strict=True)):
        basis=0.0;basis+=inlet;basis+=air;product=basis*fraction;expected=0.0 if product<0.0 else product
        delta=abs(returned-expected);assert delta<=limits['atol'] or delta<=limits['rtol']*abs(expected);maximum=max(maximum,delta)
        assert abs(second)<=limits['atol'];second_maximum=max(second_maximum,abs(second))
        if basis>0.0 and fraction in (0.0,1.0):
            label='off' if fraction==0.0 else 'on'
            first.setdefault(label,dict(sample=index,clock=parsed['clocks'][index],context=parsed['clock_context'][index],fraction=fraction,inlet=inlet,ZoneAir=air,basis=basis,expected_return=expected,actual_return=returned,actual_second_return=second))
            if fraction==0.0:off+=1;assert abs(returned)<=limits['atol']
            else:on+=1;positive+=int(returned>0.0)
    assert off>0 and on>0 and positive>0
    return dict(contract=c.ref(c.auth(recipe['public_return_flow_gate'])),policy_indices=[b['index'] for b in selected],report_ids=[b['report_id'] for b in selected],samples=480,max_same_clock_relation_delta=maximum,positive_basis_fraction_zero_samples=off,positive_basis_fraction_one_samples=on,positive_actual_return_samples=positive,second_return_zero_samples=480,max_abs_old_second_return=second_maximum,first_observations=first,comparison_policy=limits,new_first_policy=newlimits,qualification='Actual public same-clock ordered-basis/fraction/new-first flow relation and complete old-second zero branch. New first must have actual positive flow; zero second is not positive evidence. No private state or forced T/W observations.')



def actual_exhaust_output(parsed,bindings,recipe):
    observations=[]
    for name,request in recipe['physical_output_requests']:
        assert request['key_value']==recipe['new_exhaust_node'] and request['reporting_frequency']=='Timestep'
        found=[b for b in bindings if b['key'].upper()==request['key_value'].upper() and b['name']==request['variable_name']];assert len(found)==1
        bound=found[0];assert bound['producer']['stage']=='HVAC' and bound['producer']['store']=='Average'
        expected_units={'System Node Temperature':'C','System Node Humidity Ratio':'kgWater/kgDryAir','System Node Mass Flow Rate':'kg/s'}
        assert bound['units']==expected_units[request['variable_name']]
        values=list(map(float,parsed['values'][bound['report_id']]));assert len(values)==480 and all(math.isfinite(value) for value in values)
        positive=[i for i,value in enumerate(values) if value>0]
        if request['variable_name']=='System Node Mass Flow Rate':assert positive,'Real new exhaust must have observed positive mass flow'
        first=positive[0] if positive else None
        observations.append(dict(request=name,variable=request['variable_name'],report_id=bound['report_id'],samples=480,minimum_observed=min(values),maximum_observed=max(values),positive_samples=len(positive),first_positive=None if first is None else dict(sample=first,value=values[first],clock=parsed['clocks'][first],context=parsed['clock_context'][first])))
    assert len(observations)==3 and len({row['variable'] for row in observations})==3
    return dict(streams=observations,qualification='Actual public T/W/mass-flow observations only. No forced T/W levels/equality, setpoint-to-flow equality, private state or unreported substeps.')



def warning_diagnostics(files,recipe):
    contract=c.read(c.auth(recipe['warning_phase_contract']));lines=files['err'].read_text(encoding='utf8').splitlines();warnings=[];positions=[]
    for i,line in enumerate(lines):
        match=re.fullmatch(r'\s*\*\* Warning \*\* (.*)',line)
        if match:warnings.append(match[1]);positions.append(i)
    adaptive=contract['inherited_adaptive_warning'];imbalance=contract['unbalanced_flow_warning']
    count=warnings.count(imbalance);assert count in contract['allowed_unbalanced_flow_warning_counts'] and warnings.count(adaptive)==contract['expected_adaptive_warnings']==5 and len(warnings)==5+count and len(warnings) in contract['allowed_total_warning_counts']
    assert [text for text in warnings if text!=imbalance]==[adaptive]*5 and contract['expected_NodeList_blank_warnings']==contract['expected_heat_to_return_warnings']==0
    continuations=[];numerical=[]
    if count:
        position=warnings.index(imbalance);assert position>=1;at=positions[position];continuations=lines[at+1:at+6];assert len(continuations)==contract['required_unbalanced_immediate_continuations']==5
        prefix=r'\s*\*\*   ~~~   \*\* '
        assert re.fullmatch(prefix+r' Environment=.+, at Simulation time=\d{2}/\d{2} .+',continuations[0])
        patterns=[r'  Flows \[m3/s\]: Inlets: (\S+)  Unbalanced exhausts: (\S+)  Returns: (\S+)',r'  Infiltration: (\S+)  Zone Ventilation: (\S+)  Mixing \(incoming\): (\S+)',r'  Imbalance \(excess outflow\): (\S+)  Total system OA flow \(for all airloops serving this zone\): (\S+)']
        for pattern,line in zip(patterns,continuations[1:4],strict=True):
            match=re.fullmatch(prefix+pattern,line);assert match;values=list(map(float,match.groups()));assert all(math.isfinite(value) for value in values);numerical.append(values)
        assert numerical[2][0]>0.0 and continuations[4]=='   **   ~~~   **   This error will only be reported once per zone.'
    counters={}
    for kind in ['Warmup','Sizing']:
        matches=[re.fullmatch(r'\s*\*+ EnergyPlus '+kind+r' Error Summary\. During '+kind+r': (\d+) Warning; (\d+) Severe Errors\.',line) for line in lines];matches=[m for m in matches if m];assert len(matches)==1
        counters[kind]=dict(warnings=int(matches[0][1]),severe=int(matches[0][2]));assert counters[kind]['severe']==0
    assert not any(re.search(r'\*\* (?:Severe|Fatal) \*\*',line) for line in lines)
    completed=[re.fullmatch(r'\s*\*+ EnergyPlus Completed Successfully-- (\d+) Warning; (\d+) Severe Errors; Elapsed Time=.*',line) for line in lines];completed=[m for m in completed if m];assert len(completed)==1 and tuple(map(int,completed[0].groups()))==(len(warnings),0)
    end=files['end'].read_text(encoding='utf8').splitlines();assert len(end)==1
    final=re.fullmatch(r'\s*EnergyPlus Completed Successfully-- (\d+) Warning; (\d+) Severe Errors; Elapsed Time=.*',end[0]);assert final and tuple(map(int,final.groups()))==(len(warnings),0)
    return dict(ERR=c.ref(files['err']),END=c.ref(files['end']),warning_contract=recipe['warning_phase_contract'],ordered_warnings=warnings,unbalanced_flow_continuations=continuations,actual_continuation_numbers=numerical,counters=counters,completed=dict(warnings=len(warnings),severe=0),phase_counts_are_actual_observations=True,unbalanced_warning_count=count,qualification='Five exact adaptive warnings plus optional genuine once-zone unbalanced warning with its complete five immediate continuations and actual operands/counters. No old order-position/phase/count/flow fixture. Whole paired ERR/END remains exact under original version/elapsed boundary.')



def cooling_availability(data,bindings,recipe):
    contract=c.read(c.auth(recipe['cooling_public_gates']));indices=[152,6,63,93,94,81,82,66,99];assert [row['policy_index'] for row in contract['bindings']]==indices
    selected=[];maps=[];ordered=None;reports=[]
    for request in contract['bindings']:
        found=[b for b in bindings if b['index']==request['policy_index']];assert len(found)==1;bound=found[0];assert typed_dom(bound['policy'])==typed_dom(request['rule']) and bound['producer']['stage']==request['producer_stage'] and bound['producer']['store']==request['store'];selected.append(bound)
        samples=data['values'][bound['report_id']];assert len(samples)==480;joined={};order=[]
        for physical in samples:
            stamp=data['stamps'][physical['stamp_index']];environment=data['environments'][stamp['environment_ordinal']-1];key=(stamp['environment_ordinal'],tuple(environment['fields']),stamp['report_id'],stamp['kind'],tuple(stamp['fields']));assert key not in joined and math.isfinite(physical['value']);joined[key]=(physical,stamp,environment);order.append(key)
        if ordered is None:ordered=order
        else:assert order==ordered
        maps.append(joined);reports.append(dict(role=request['role'],policy_index=bound['index'],report_id=bound['report_id'],key=bound['key'],name=bound['name'],units=bound['units'],producer=bound['producer'],policy=bound['policy'],samples=480))
    assert len(ordered)==480;off=on=service=0;max_rate=max_energy=0.0;first={};potential_cap=selected[2]['policy']['atol'];rate_cap=selected[3]['policy']['atol'];energy_cap=selected[4]['policy']['atol'];assert potential_cap==rate_cap==0.001 and energy_cap==1.0
    for index,key in enumerate(ordered):
        rows=[mapping[key] for mapping in maps];values=[row[0]['value'] for row in rows];cooling,overall,potential,rate,energy,zone_rate,zone_energy,inlet_flow,unit_flow=values
        label=None
        if overall>0.0 and cooling<=0.0 and potential < -potential_cap:
            off+=1;max_rate=max(max_rate,abs(rate));max_energy=max(max_energy,abs(energy));assert abs(rate)<=rate_cap and abs(energy)<=energy_cap;label='off'
        elif overall>0.0 and cooling>0.0 and potential < -potential_cap:
            on+=1
            if rate>rate_cap and energy>energy_cap:service+=1;label='on_positive_service'
        if label and label not in first:
            witness=[]
            for request,bound,(physical,stamp,environment) in zip(contract['bindings'],selected,rows,strict=True):witness.append(dict(role=request['role'],policy_index=bound['index'],report_id=bound['report_id'],reported_decimal=physical['token'],parsed_binary64_bits=c.bits(physical['value']),value=physical['value'],physical_line=physical['physical_line'],stream_data_ordinal=physical['stream_data_ordinal'],stamp_index=physical['stamp_index'],timestamp=stamp,environment=environment))
            first[label]=dict(sample=index,full_clock_context=[key[0],list(key[1]),key[2],key[3],list(key[4])],actual_public_reports=witness)
    assert off>0 and on>0 and service>0
    return dict(contract=recipe['cooling_public_gates'],reports=reports,policy_indices=indices,samples=480,full_clock_context_operand_joins=4320,off_potential_samples=off,on_potential_samples=on,positive_on_service_samples=service,max_abs_off_supply_sensible_cooling_rate=max_rate,max_abs_off_supply_sensible_cooling_energy=max_energy,first_observations=first,no_transport_or_zone_total_zero_assertions=True,counts_are_observations_not_targets=True,public_potential_is_not_private_substep_demand=True,qualification='Actual complete public clock/context joins. Only supply sensible93/94 are suppressed under overall-on/cooling-off/public demand; flows66/99 and passive zone81/82 are retained observations. No old profile/count, heating, private CoolOn or timestep-energy reconstruction claim.')



def heating_availability(data,bindings,recipe):
    contract=c.read(c.auth(recipe['heating_availability_public_gates']));indices=[152,6,62,87,88,75,76,66,99];assert [row['policy_index'] for row in contract['bindings']]==indices
    selected=[];maps=[];ordered=None;reports=[]
    for request in contract['bindings']:
        found=[b for b in bindings if b['index']==request['policy_index']];assert len(found)==1;bound=found[0];assert typed_dom(bound['policy'])==typed_dom(request['rule']) and bound['producer']['stage']==request['producer_stage'] and bound['producer']['store']==request['store'];selected.append(bound)
        samples=data['values'][bound['report_id']];assert len(samples)==480;joined={};order=[]
        for physical in samples:
            stamp=data['stamps'][physical['stamp_index']];environment=data['environments'][stamp['environment_ordinal']-1];key=(stamp['environment_ordinal'],tuple(environment['fields']),stamp['report_id'],stamp['kind'],tuple(stamp['fields']));assert key not in joined and math.isfinite(physical['value']);joined[key]=(physical,stamp,environment);order.append(key)
        if ordered is None:ordered=order
        else:assert order==ordered
        maps.append(joined);reports.append(dict(role=request['role'],policy_index=bound['index'],report_id=bound['report_id'],key=bound['key'],name=bound['name'],units=bound['units'],producer=bound['producer'],policy=bound['policy'],samples=480))
    assert len(ordered)==480;off=on=service=0;winter_off=winter_on=winter_service=0;max_rate=max_energy=0.0;first={};potential_cap=selected[2]['policy']['atol'];rate_cap=selected[3]['policy']['atol'];energy_cap=selected[4]['policy']['atol'];assert potential_cap==rate_cap==0.001 and energy_cap==1.0
    for index,key in enumerate(ordered):
        rows=[mapping[key] for mapping in maps];values=[row[0]['value'] for row in rows];heating,overall,potential,rate,energy,zone_rate,zone_energy,inlet_flow,unit_flow=values
        label=None
        if overall>0.0 and heating<=0.0 and potential > potential_cap:
            off+=1;winter_off+=int(key[0]==1);max_rate=max(max_rate,abs(rate));max_energy=max(max_energy,abs(energy));assert abs(rate)<=rate_cap and abs(energy)<=energy_cap;label='off'
        elif overall>0.0 and heating>0.0 and potential > potential_cap:
            on+=1;winter_on+=int(key[0]==1)
            if rate>rate_cap and energy>energy_cap:service+=1;winter_service+=int(key[0]==1);label='on_positive_service'
        if key[0]==1 and label and label not in first:
            witness=[]
            for request,bound,(physical,stamp,environment) in zip(contract['bindings'],selected,rows,strict=True):witness.append(dict(role=request['role'],policy_index=bound['index'],report_id=bound['report_id'],reported_decimal=physical['token'],parsed_binary64_bits=c.bits(physical['value']),value=physical['value'],physical_line=physical['physical_line'],stream_data_ordinal=physical['stream_data_ordinal'],stamp_index=physical['stamp_index'],timestamp=stamp,environment=environment))
            first[label]=dict(sample=index,full_clock_context=[key[0],list(key[1]),key[2],key[3],list(key[4])],actual_public_reports=witness)
    assert off>0 and on>0 and service>0 and winter_off>0 and winter_on>0 and winter_service>0
    return dict(contract=recipe['heating_availability_public_gates'],reports=reports,policy_indices=indices,samples=480,full_clock_context_operand_joins=4320,off_potential_samples=off,on_potential_samples=on,positive_on_service_samples=service,winter_off_potential_samples=winter_off,winter_on_potential_samples=winter_on,winter_positive_on_service_samples=winter_service,max_abs_off_supply_sensible_heating_rate=max_rate,max_abs_off_supply_sensible_heating_energy=max_energy,first_observations=first,no_transport_or_zone_total_zero_assertions=True,counts_are_observations_not_targets=True,public_potential_is_not_private_substep_demand=True,qualification='Actual complete public clock/context joins. Only supply sensible87/88 are suppressed under overall-on/heating-off/positive public heating potential62; zone75/76 and flows66/99 remain observations. Meaningful off/on service must occur in actual newWinter environment1. No old profile/count, private HeatOn or substep reconstruction claim.')



def heating_fuel_efficiency(data,bindings,recipe):
    contract=c.read(c.auth(recipe['heating_fuel_public_gates']));indices=[recipe['heating_efficiency_policy_index'],85,86,97];assert [row['policy_index'] for row in contract['bindings']]==indices and contract['case_kind']==recipe['inherited_heating_efficiency_case_kind']=='POSITIVE'
    selected=[];maps=[];ordered=None;reports=[]
    for request in contract['bindings']:
        found=[b for b in bindings if b['index']==request['policy_index']];assert len(found)==1;bound=found[0];assert typed_dom(bound['policy'])==typed_dom(request['rule']) and bound['producer']['stage']==request['producer_stage'] and bound['producer']['store']==request['store'];selected.append(bound)
        samples=data['values'][bound['report_id']];assert len(samples)==480;joined={};order=[]
        for physical in samples:
            stamp=data['stamps'][physical['stamp_index']];environment=data['environments'][stamp['environment_ordinal']-1];key=(stamp['environment_ordinal'],tuple(environment['fields']),stamp['report_id'],stamp['kind'],tuple(stamp['fields']));assert key not in joined and math.isfinite(physical['value']);joined[key]=(physical,stamp,environment);order.append(key)
        if ordered is None:ordered=order
        else:assert order==ordered
        maps.append(joined);reports.append(dict(role=request['role'],policy_index=bound['index'],report_id=bound['report_id'],key=bound['key'],name=bound['name'],units=bound['units'],producer=bound['producer'],policy=bound['policy'],samples=480))
    assert len(ordered)==480;positive=0;branches=dict(raw_positive=0,fallback_nonpositive=0);first=None;maximum=0.0;raw_values=set();effective_values=set();fuel_rule=selected[3]['policy'];assert fuel_rule['atol']==1.0 and fuel_rule['rtol']==0.0 and fuel_rule['rmse_max']==1.0
    for index,key in enumerate(ordered):
        rows=[mapping[key] for mapping in maps];raw,rate,energy,fuel=[row[0]['value'] for row in rows];effective=1.0 if raw<=0.0 else raw;expected=energy/effective;assert math.isfinite(expected);delta=abs(fuel-expected);maximum=max(maximum,delta);assert delta<=fuel_rule['atol'] or delta<=fuel_rule['rtol']*abs(expected)
        branches['fallback_nonpositive' if raw<=0.0 else 'raw_positive']+=1;raw_values.add(raw);effective_values.add(effective)
        selected_branch=(raw>0.0 and raw!=1.0) if recipe['inherited_heating_efficiency_case_kind']=='POSITIVE' else raw<=0.0
        if key[0]==1 and selected_branch and rate>selected[1]['policy']['atol'] and energy>selected[2]['policy']['atol'] and fuel>fuel_rule['atol']:
            positive+=1
            if first is None:
                witness=[]
                for request,bound,(physical,stamp,environment) in zip(contract['bindings'],selected,rows,strict=True):witness.append(dict(role=request['role'],policy_index=bound['index'],report_id=bound['report_id'],reported_decimal=physical['token'],parsed_binary64_bits=c.bits(physical['value']),value=physical['value'],physical_line=physical['physical_line'],stream_data_ordinal=physical['stream_data_ordinal'],stamp_index=physical['stamp_index'],timestamp=stamp,environment=environment))
                first=dict(sample=index,full_clock_context=[key[0],list(key[1]),key[2],key[3],list(key[4])],actual_public_reports=witness,effective_efficiency=effective,expected_fuel_energy=expected,actual_relation_delta=delta)
    assert positive>0 and first is not None and sum(branches.values())==480
    return dict(contract=recipe['heating_fuel_public_gates'],case_kind=recipe['case_kind'],efficiency_case_kind=recipe['inherited_heating_efficiency_case_kind'],reports=reports,policy_indices=indices,samples=480,full_clock_context_operand_joins=1920,branch_counts=branches,positive_service_samples=positive,positive_service_environment_ordinal=1,distinct_actual_selected_efficiencies=sorted(raw_values),distinct_actual_effective_efficiencies=sorted(effective_values),max_abs_fuel_energy_relation_delta=maximum,comparison_policy=fuel_rule,first_positive_service=first,counts_are_observations_not_targets=True,no_zero_load_positive_evidence=True,no_private_substep_reconstruction=True,qualification=contract['reporting_interval_qualification'])



def winter_weather_activation(data,bindings,recipe):
    contract=c.read(c.auth(recipe['winter_weather_activation_supplement']));request=contract['binding'];found=[b for b in bindings if b['index']==55];assert len(found)==1;bound=found[0];assert typed_dom(bound['policy'])==typed_dom(request['rule']) and bound['producer']['stage']=='Zone' and bound['producer']['store']=='Average' and bound['units']=='C'
    selected=[];first=None;maximum=0.0
    for physical in data['values'][bound['report_id']]:
        stamp=data['stamps'][physical['stamp_index']]
        if stamp['environment_ordinal']!=1:continue
        environment=data['environments'][0];value=physical['value'];assert math.isfinite(value);delta=abs(value-contract['expected_constant_drybulb']);maximum=max(maximum,delta);assert delta<=bound['policy']['atol'] or delta<=bound['policy']['rtol']*abs(contract['expected_constant_drybulb']);selected.append(value)
        if first is None:first=dict(policy_index=55,report_id=bound['report_id'],reported_decimal=physical['token'],parsed_binary64_bits=c.bits(value),value=value,physical_line=physical['physical_line'],stream_data_ordinal=physical['stream_data_ordinal'],stamp_index=physical['stamp_index'],timestamp=stamp,environment=environment)
    assert len(selected)==contract['required_sample_count']==96 and first is not None
    return dict(contract=recipe['winter_weather_activation_supplement'],policy_index=55,report_id=bound['report_id'],key=bound['key'],name=bound['name'],units=bound['units'],producer=bound['producer'],policy=bound['policy'],environment_ordinal=1,samples=96,expected_drybulb=contract['expected_constant_drybulb'],max_abs_deviation=maximum,minimum_observed=min(selected),maximum_observed=max(selected),first_observation=first,site_environment_not_surface_height_temperature=True)



def public_sample(data,bound,physical):
    stamp=data['stamps'][physical['stamp_index']];environment=data['environments'][stamp['environment_ordinal']-1]
    return dict(policy_index=bound['index'],report_id=bound['report_id'],reported_decimal=physical['token'],parsed_binary64_bits=c.bits(physical['value']),value=physical['value'],physical_line=physical['physical_line'],stream_data_ordinal=physical['stream_data_ordinal'],stamp_index=physical['stamp_index'],timestamp=stamp,environment=environment)



def record_gate(name,function,*arguments):
    try:
        value=function(*arguments);passed=not isinstance(value,dict) or value.get('activation_accepted',True) is True
        return dict(name=name,passed=passed,observation=value,error=None if passed else 'Declared positive public witness was not observed')
    except Exception as error:return dict(name=name,passed=False,observation=None,error=repr(error),traceback=traceback.format_exc())



def require(condition,message):
    assert condition,message
    return True



def native_outputs(files,recipe,policy,environments):
    catalog=c.rdd(files['rdd']); answer={};count=0
    for extension,rules,steps in [('eso',policy['rows'],4),('mtr',policy['meter_rows'],1)]:
        old=c.stream(files[extension],len(rules),steps,environments)
        parsed=c.read_stream(files[extension],len(rules),steps,environments)
        assert old['dictionary']==parsed['dictionary'] and old['footer']==parsed['footer']
        binding=c.bind(old,rules,catalog,extension=='mtr')
        by_id={b['report_id']:b for b in binding};ordered=[by_id[rid] for rid in parsed['ordered']]
        assert all(old['values'][rid]==[s['token'] for s in parsed['values'][rid]] for rid in parsed['ordered'])
        answer[extension]=(old,parsed,ordered);count+=parsed['count']
    assert count==recipe['counts']['total_values']
    if 'converted_input' in files:assert typed_dom(c.read(files['converted_input']))==typed_dom(c.read(c.auth(recipe['scientific_dom'])))
    nodes=node_structure(files['bnd'],recipe)
    err=files['err'].read_text(encoding='utf8');end=files['end'].read_text(encoding='utf8');assert 'EnergyPlus Completed Successfully' in err and 'EnergyPlus Completed Successfully' in end and not re.search(r'\*\*\s+(?:Severe|Fatal)\s+\*\*',err)
    eio=[c.csv_line(line) for line in files['eio'].read_text(encoding='utf8').splitlines()];warmup=[int(row[1]) for row in eio if row and row[0]=='Environment:WarmupDays'];assert len(warmup)==5 and all(value>0 for value in warmup)
    ctf=[row for row in eio if row and row[0] in [' Construction CTF',' Material CTF Summary',' Material:Air CTF Summary',' CTF']];assert not ctf
    return answer,dict(total_series=recipe['counts']['total_series'],total_values=count,node_structure=nodes,warmup_days=warmup,CTF_coefficient_rows=0,complete_finite_dictionaries_clocks_footer=True,structural_precheck_accepted=True,qualifying_science_accepted=False,qualifying_pair_credit=False)



def ancillary(nf,sf):
    result={}
    for extension in ['eio','err','end','rdd','mdd','mtd']:
        a,b=nf[extension].read_bytes().splitlines(),sf[extension].read_bytes().splitlines()
        if extension in ['eio','rdd','mdd']:
            assert a[0].startswith((b'Program Version,',b'! Program Version,')) and b[0].startswith((b'Program Version,',b'! Program Version,'))
            assert a[1:]==b[1:],extension
        elif extension=='err':
            assert a[0].startswith(b'Program Version,') and b[0].startswith(b'Program Version,') and a[1:-1]==b[1:-1]
            assert re.sub(rb'Elapsed Time=.*',b'',a[-1])==re.sub(rb'Elapsed Time=.*',b'',b[-1])
        elif extension=='end':
            assert len(a)==len(b)==1 and re.sub(rb'Elapsed Time=.*',b'',a[0])==re.sub(rb'Elapsed Time=.*',b'',b[0])
        else:assert a==b,extension
        result[extension]=dict(Native=c.ref(nf[extension]),Source=c.ref(sf[extension]),complete_body_equal=True)
    assert nf['shd'].read_bytes()==sf['shd'].read_bytes()
    result['shd']=dict(Native=c.ref(nf['shd']),Source=c.ref(sf['shd']),whole_bytes_equal=True)
    return result



def source_scope_checks(source,environments):
    summary=c.read(source/'run-summary.json');runtime=summary['rust_runtime'];support=c.read(source/'support-assessment.json');assert summary['exit_code']==0 and summary['support']['runtime_class']=='ideal-loads-direct-zone-coupled-compatibility'
    assert summary['support']['conformance_claim'] is False and support['claim_boundary']['conformance_claim'] is False and support['claim_boundary']['release_evidence'] is False and support['runtime_class']==summary['support']['runtime_class']
    assert summary['config']['hours'] is None and summary['config']['compare_oracle'] is False and summary['oracle'] is None and summary['comparison'] is None and summary['oracle_status']=='not-requested' and not any(x.is_file() for x in (source/'oracle').rglob('*'))
    assert runtime['native_environment_hour_scope']==120 and runtime['requested_hours'] is None and runtime['native_options']==dict(DoDesDaySim=True,DoWeathSim=False,DDOnly=False,FullAnnualRun=False)
    assert runtime['native_environment_scope']==[dict(ordinal=e['ordinal'],title=e['title'],kind_of_environment=e['kind'],total_days=e['total_days'],selected_by_primary_flags=True) for e in environments]
    scope=c.read(source/'porting_scope.json');assert scope['admissible'] and scope['scope']=='B' and not scope['violations'] and scope['environment']['weather_coverage']['status']=='absent' and scope['environment']['source_calendar_controls']['preview_values_used_by_Source'] is False
    return dict(run_summary=c.ref(source/'run-summary.json'),support_assessment=c.ref(source/'support-assessment.json'),porting_scope=c.ref(source/'porting_scope.json'))



def HVAC_humidistat_warning_diagnostics(files,recipe):
    # Only final PurchasedAir saturation is newly reachable here. Exact no-OA
    # fields exclude both requested-OA-flow constraint warning branches.
    contract=c.read(c.auth(recipe['warning_phase_contract']));lines=files['err'].read_text(encoding='utf8').splitlines()
    unit_name=recipe['IdealLoads_public_name'];owner='ZoneHVAC:IdealLoadsAirSystem "'+unit_name+'"'
    number=r'(-?\d+\.\d{5}(?:E[+-]\d{3})?)'
    first_pattern=re.escape(owner)+r' Supply humidity ratio = '+number+r' exceeds saturation limit '+number+r' \[kgWater/kgDryAir\]'
    recurring_message=owner+' Supply humidity ratio exceeds saturation limit warning continues, delta max/min [kgWater/kgDryAir]...'
    warnings=[];positions=[];first=[]
    for at,line in enumerate(lines):
        match=re.fullmatch(r'\s*\*\* Warning \*\* (.*)',line)
        if match:
            warnings.append(match[1]);positions.append(at)
            saturation=re.fullmatch(first_pattern,match[1])
            if saturation:first.append((at,match[1],saturation.groups()))
    summary=[at for at,line in enumerate(lines) if re.fullmatch(r'\s*\*{13}  \*\* Warning \*\* '+re.escape(recurring_message),line)]
    if not first and not summary:
        answer=warning_diagnostics(files,recipe)
        answer['PurchasedAir_saturation']=dict(first_warning_count=0,recurring_summary_present=False,recurring_observed_counts=None)
        return answer
    all_recurring_warning_positions=[at for at,line in enumerate(lines) if re.fullmatch(r'\s*\*{13}  \*\* Warning \*\* .*',line)]
    assert all_recurring_warning_positions==summary
    # Constructor latch0 is incremented once and never reset per environment.
    assert len(first)==1 and len(summary)<=1
    at,message,tokens=first[0];operands=list(map(float,tokens));assert all(math.isfinite(value) for value in operands) and operands[0]>=operands[1]
    prefix=r'\s*\*\*   ~~~   \*\* '
    assert re.fullmatch(prefix+r' Simulation continuing \. \. \. ',lines[at+1])
    assert re.fullmatch(prefix+r'(?: Environment=| During Warmup, Environment=| During Warmup & Sizing, Environment=| During Sizing, Environment=).+, at Simulation time=\d{2}/\d{2} .+',lines[at+2])
    first_continuations=lines[at+1:at+3]
    adaptive=contract['inherited_adaptive_warning'];imbalance=contract['unbalanced_flow_warning'];count=warnings.count(imbalance)
    assert count in contract['allowed_unbalanced_flow_warning_counts'] and warnings.count(adaptive)==contract['expected_adaptive_warnings']==5 and len(warnings)==6+count
    assert [text for text in warnings if text not in [message,imbalance]]==[adaptive]*5
    continuations=[];numerical=[]
    if count:
        inherited=[text for text in warnings if text!=message];assert inherited.index(imbalance)>=1
        pos=positions[warnings.index(imbalance)];continuations=lines[pos+1:pos+6];assert len(continuations)==contract['required_unbalanced_immediate_continuations']==5
        assert re.fullmatch(prefix+r' Environment=.+, at Simulation time=\d{2}/\d{2} .+',continuations[0])
        patterns=[r'  Flows \[m3/s\]: Inlets: (\S+)  Unbalanced exhausts: (\S+)  Returns: (\S+)',r'  Infiltration: (\S+)  Zone Ventilation: (\S+)  Mixing \(incoming\): (\S+)',r'  Imbalance \(excess outflow\): (\S+)  Total system OA flow \(for all airloops serving this zone\): (\S+)']
        for pattern,line in zip(patterns,continuations[1:4],strict=True):
            match=re.fullmatch(prefix+pattern,line);assert match;values=list(map(float,match.groups()));assert all(math.isfinite(value) for value in values);numerical.append(values)
        assert numerical[2][0]>0.0 and continuations[4]=='   **   ~~~   **   This error will only be reported once per zone.'
    recurring=None;summary_lines=[]
    if summary:
        pos=summary[0];assert pos>at;headings=[i for i,line in enumerate(lines) if re.fullmatch(r'\s*\*{13} ===== Recurring Error Summary =====',line)];assert len(headings)==1 and at<headings[0]<pos
        stat=r'\s*\*{13}  \*\*   ~~~   \*\*   ';counts=[]
        for offset,pattern in enumerate([r'This error occurred (\d+) total times;',r'during Warmup (\d+) times;',r'during Sizing (\d+) times\.'],1):
            match=re.fullmatch(stat+pattern,lines[pos+offset]);assert match;counts.append(int(match[1]))
        assert counts[0]>0 and 0<=counts[1]<=counts[0] and 0<=counts[2]<=counts[0]
        extrema=re.fullmatch(stat+r'Max=(-?\d+\.\d{6})  Min=(-?\d+\.\d{6})',lines[pos+4]);assert extrema
        maximum,minimum=map(float,extrema.groups());assert math.isfinite(maximum) and math.isfinite(minimum) and 0.0<minimum<=maximum
        recurring=dict(total=counts[0],warmup=counts[1],sizing=counts[2],reported_delta_maximum=maximum,reported_delta_minimum=minimum,units='kgWater/kgDryAir (message); empty per-field units',first_event_excluded_from_recurring_count=True)
        summary_lines=lines[pos:pos+5]
    counters={}
    for phase in ['Warmup','Sizing']:
        matches=[re.fullmatch(r'\s*\*+ EnergyPlus '+phase+r' Error Summary\. During '+phase+r': (\d+) Warning; (\d+) Severe Errors\.',line) for line in lines];matches=[match for match in matches if match];assert len(matches)==1
        counters[phase]=dict(warnings=int(matches[0][1]),severe=int(matches[0][2]));assert counters[phase]['severe']==0
    assert not any(re.search(r'\*\*\s+(?:Severe|Fatal)\s+\*\*',line) for line in lines)
    completed=[re.fullmatch(r'\s*\*+ EnergyPlus Completed Successfully-- (\d+) Warning; (\d+) Severe Errors; Elapsed Time=.*',line) for line in lines];completed=[match for match in completed if match];assert len(completed)==1
    actual_completed=dict(warnings=int(completed[0][1]),severe=int(completed[0][2]));assert actual_completed['severe']==0
    end=files['end'].read_text(encoding='utf8').splitlines();assert len(end)==1;final=re.fullmatch(r'\s*EnergyPlus Completed Successfully-- (\d+) Warning; (\d+) Severe Errors; Elapsed Time=.*',end[0]);assert final and tuple(map(int,final.groups()))==(actual_completed['warnings'],0)
    saturation=dict(first_warning_count=1,first_warning=message,first_reported_tokens=list(tokens),first_reported_operands=operands,first_continuations=first_continuations,recurring_summary_present=bool(summary),recurring_observed_counts=recurring,recurring_summary_lines=summary_lines,first_ShowWarningError_counted=True,recurring_only_after_first_latch=True,no_fixed_occurrence_or_phase_count=True,no_delta_reconstruction_from_truncated_operands=True)
    return dict(ERR=c.ref(files['err']),END=c.ref(files['end']),warning_contract=recipe['warning_phase_contract'],ordered_warnings=warnings,unbalanced_flow_continuations=continuations,actual_continuation_numbers=numerical,counters=counters,completed=actual_completed,phase_counts_are_actual_observations=True,unbalanced_warning_count=count,PurchasedAir_saturation=saturation,qualification='Exact optional final PurchasedAir supersaturation family after availability clamps; first .5T warning/continuation/empty timestamp and ELSE-only recurring raw delta max/min. Count and phase quantities are observed independently. All inherited Adaptive/unbalanced messages and complete paired ERR/END remain mandatory. No OA constraint, arbitrary psychrometric warning, fitted count or warning stripping is admitted.')



def inspect_csv_export131(source, data, metadata, extension):
    path = source / ('results/source-' + extension + '.csv')
    reader = csv.DictReader(io.StringIO(path.read_text(encoding='utf8')))
    assert reader.fieldnames == ['stream','report_id','key','name','units','frequency','store_type','kind','sample_index','stream_data_ordinal','environment_ordinal','environment_fields_json','timestamp_report_id','timestamp_kind','timestamp_fields_json','reported_fields_json','reported_decimal','parsed_binary64_bits']
    actual_rows = list(reader)
    expected = sum(len(data['values'][item['report_id']]) for item in metadata)
    assert len(actual_rows) == expected == (74880 if extension == 'eso' else 120*len(metadata))
    cursor = 0
    for item in metadata:
        rid = item['report_id']
        for i, physical in enumerate(data['values'][rid]):
            actual = actual_rows[cursor]; cursor += 1
            stamp = data['stamps'][physical['stamp_index']]
            environment = data['environments'][stamp['environment_ordinal'] - 1]
            required = dict(stream=extension, report_id=str(rid), key='' if item['key'] is None else item['key'], name=item['name'], units=item['units'], frequency='hourly' if extension == 'mtr' else 'timestep', store_type=item['producer']['store'].lower(), kind='meter' if extension == 'mtr' else 'variable', sample_index=str(i), stream_data_ordinal=str(physical['stream_data_ordinal']), environment_ordinal=str(stamp['environment_ordinal']), timestamp_report_id=str(stamp['report_id']), timestamp_kind=stamp['kind'], reported_decimal=physical['token'], parsed_binary64_bits=c.bits(physical['value']))
            for key, value in required.items():
                assert actual[key] == value, (extension, cursor, key, actual[key], value)
            assert json.loads(actual['environment_fields_json']) == environment['fields']
            assert json.loads(actual['timestamp_fields_json']) == stamp['fields']
            assert json.loads(actual['reported_fields_json']) == [physical['token']]
    return dict(artifact=c.ref(path), operand_bits_line_owner_clock_joins=cursor)



def node_structure(path, recipe):
    raw = path.read_bytes()
    version, body = raw.split(b'\n', 1)
    assert version.startswith(b'Program Version,')
    actual = body.replace(b'\r\n', b'\n')
    bundle = p._topology_contract(recipe['structure_config'])
    prediction, expected = p.structure_baseline(recipe['structure_config'])
    assert actual == expected.replace(b'\r\n', b'\n')
    result = p._validate_topology_rows(prediction, p._topology_csv(actual), bundle['parent_rows'])
    witnesses = {name: dict(node_id=node_id, UTF8_hex=name.encode('utf8').hex()) for name, node_id in result['ids'].items()}
    return dict(artifact=c.ref(path), body=actual.decode('utf8').splitlines(), actual_nodes=result['nodes'], actual_node_ids_by_name=result['ids'], exact_references_by_name=result['references'], total_nonparent_connections=len(result['connections']), outdoor_air_registry=result['registry'], complete_connection_roles=result['roles'], suspicious_nodes=result['suspicious'], nonconnected_nodes=result['nonconnected'], whole_prospective_topology_exact=True, structural_overlay_inverse_exact=True, inherited_core_connections_and_headers_exact=True, source_based_derivation=recipe['structure_config']['derivation'], identity_fields_stripped=False, EOL_normalization='CRLF-to-LF only', UTF8_name_and_ID_witnesses=witnesses, private_getter_telemetry_observed=False)



def source_input_projection(source,recipe):
    summary=c.read(source/'run-summary.json');assert summary['input']['kind']=='epjson'
    staged=source/'input/original.epJSON';converted=source/'input/converted.epJSON'
    expected=c.auth(recipe['input']).read_bytes()
    assert staged.read_bytes()==converted.read_bytes()==expected
    scientific=c.read(c.auth(recipe['scientific_dom']));document=c.read(converted)
    assert typed_dom(document)==typed_dom(scientific)
    assert summary['input']['original']==str(staged) and summary['input']['converted_epjson']==str(converted)
    projection=c.read(source/'compiled-geometry.json')
    assert projection['physics_executed'] is False and projection['observer_supplies_inputs'] is False
    assert projection['inputs']['source_assisted_lexical_conversion'] is False
    actual=projection['parsed_input']
    assert actual['raw_source_is_epjson'] is True
    assert actual['scheduled_idf_declaration_overlay'] is actual['nodelist_idf_declaration_overlay'] is False
    assert len(actual['objects']['NodeList'])==5 and len(actual['objects']['SetpointManager:Scheduled'])==9
    for family in ['NodeList','SetpointManager:Scheduled']:
        assert typed_dom(actual['objects'][family])==typed_dom(document[family])
    assert actual['nodelist_source_order_unavailable'] is None and actual['nodelist_source_order']==recipe['nodelist_order']
    contract=c.read(c.auth(recipe['source_input_contract']))
    assert contract['case_inputs'][p.CASE]==recipe['input'] and contract['whole_staged_bytes_and_typed_DOM_mandatory'] is True
    for family,count in contract['declared_family_counts'].items():assert len(document.get(family,{}))==count
    unit=document['ZoneHVAC:IdealLoadsAirSystem'][contract['unit_raw_name']]
    dsoa=document['DesignSpecification:OutdoorAir'];assert list(dsoa)==[contract['DSOA_name']]
    row=dsoa[contract['DSOA_name']]
    assert row['outdoor_air_method']==recipe['outdoor_air_method']
    for field,value in contract['common_DSOA_numeric_tokens'].items():assert typed_dom(row[field])==typed_dom(json.loads(value))
    for field in contract['optional_DSOA_schedule_fields_omitted']:assert field not in row
    assert unit['design_specification_outdoor_air_object_name']==contract['DSOA_name']
    assert unit['outdoor_air_inlet_node_name']==contract['OA_node_name']
    baseline=c.read(c.auth(recipe['baseline_input']))['ZoneHVAC:IdealLoadsAirSystem'][contract['unit_raw_name']]
    held={key:value for key,value in unit.items() if key not in ['design_specification_outdoor_air_object_name','outdoor_air_inlet_node_name']}
    assert typed_dom(held)==typed_dom(baseline)
    return dict(artifact=c.ref(source/'compiled-geometry.json'),staged_original=c.ref(staged),staged_complete_input=c.ref(converted),actual_Scheduled_inputs=actual['objects']['SetpointManager:Scheduled'],actual_NodeList_inputs=actual['objects']['NodeList'],actual_NodeList_source_order=actual['nodelist_source_order'],actual_parent_EquipmentConnections=document['ZoneHVAC:EquipmentConnections'],actual_IdealLoads_objects=document['ZoneHVAC:IdealLoadsAirSystem'],actual_DSOA_objects_from_staged_input=dsoa,actual_DesignDay_inputs_from_staged_input=document['SizingPeriod:DesignDay'],raw_source_is_epjson=True,scheduled_idf_declaration_overlay=False,nodelist_idf_declaration_overlay=False,qualification='Whole staged bytes and typed DOM plus existing actual NodeList/SPM export only. DSOA fields are authored raw-input observations, not private pointer/getter/default/sizing/cache state.')



def public_outdoor_air_mixing(data,bindings,recipe):
    contract=c.read(c.auth(recipe['public_outdoor_air_mixing_gate']))
    indices=[6,99,101,146,147,152,153,154,155]
    assert contract['public_OA_gates']['indices']==indices
    selected=[];maps=[];ordered=None;reports=[]
    for index in indices:
        found=[bound for bound in bindings if bound['index']==index];assert len(found)==1
        bound=found[0];assert bound['producer']['stage']==('Zone' if index in [6,152] else 'HVAC') and bound['producer']['store']=='Average'
        assert bound['policy']['atol']==(1e-6 if index in [146,154] else 1e-9) and bound['policy']['rtol']==0.0
        samples=data['values'][bound['report_id']];assert len(samples)==480
        joined={};order=[]
        for physical in samples:
            stamp=data['stamps'][physical['stamp_index']];environment=data['environments'][stamp['environment_ordinal']-1]
            key=(stamp['environment_ordinal'],tuple(environment['fields']),stamp['report_id'],stamp['kind'],tuple(stamp['fields']))
            assert key not in joined and math.isfinite(physical['value'])
            joined[key]=physical;order.append(key)
        if ordered is None:ordered=order
        else:assert order==ordered
        selected.append(bound);maps.append(joined)
        reports.append(dict(policy_index=index,report_id=bound['report_id'],key=bound['key'],name=bound['name'],units=bound['units'],producer=bound['producer'],policy=bound['policy'],samples=480))
    assert len(ordered)==480
    candidate=temperature=humidity=0;first={};max_temperature=max_humidity=0.0
    for sample,key in enumerate(ordered):
        physical=[mapping[key] for mapping in maps];values=[item['value'] for item in physical]
        overall,supply,oa,exhaust_t,exhaust_w,mode,volume,mixed_t,mixed_w=values
        if not (overall>1e-9 and mode>1e-9 and oa>1e-9 and supply>oa+1e-9 and volume>1e-9):continue
        candidate+=1;delta_t=abs(mixed_t-exhaust_t);delta_w=abs(mixed_w-exhaust_w)
        max_temperature=max(max_temperature,delta_t);max_humidity=max(max_humidity,delta_w)
        labels=['common_candidate']
        if delta_t>1e-6:temperature+=1;labels.append('mixed_temperature_response')
        if delta_w>1e-9:humidity+=1;labels.append('mixed_humidity_response')
        for label in labels:
            if label not in first:first[label]=dict(sample=sample,full_clock_context=[key[0],list(key[1]),key[2],key[3],list(key[4])],actual_public_reports=[public_sample(data,bound,item) for bound,item in zip(selected,physical,strict=True)])
    return dict(contract=recipe['public_outdoor_air_mixing_gate'],policy_indices=indices,report_ids=[bound['report_id'] for bound in selected],reports=reports,samples=480,full_clock_context_operand_joins=4320,common_candidate_samples=candidate,mixed_temperature_response_samples=temperature,mixed_humidity_response_samples=humidity,max_abs_candidate_temperature_difference=max_temperature,max_abs_candidate_humidity_difference=max_humidity,first_observations=first,activation_accepted=candidate>0 and temperature>0 and humidity>0,counts_are_observations_not_targets=True,qualification='Each engine independently shows public interval positive OA/supply and mixed-air response at complete common clocks. Temperature and humidity witnesses may differ. No averaged mixing ratio/enthalpy, exact design-volume/capacity/availability law, private mode or substep reconstruction.')



def paired_science(n,s,nobs,sobs,ngates,sgates,nf,sf):
    checks={};indices=[119,120,123,124,125]
    nb={bound['index']:bound for bound in n['eso'][2]};sb={bound['index']:bound for bound in s['eso'][2]}
    checks['status_arrays_exact']=record_gate('status_arrays_exact',require,all([float(value) for value in n['eso'][0]['values'][nb[index]['report_id']]]==[float(value) for value in s['eso'][0]['values'][sb[index]['report_id']]] for index in indices),'Status arrays differ')
    checks['warmup_days_exact']=record_gate('warmup_days_exact',require,nobs['warmup_days']==sobs['warmup_days'],'Warmup days differ')
    checks['whole_BND_body_exact']=record_gate('whole_BND_body_exact',require,nobs['node_structure']['body']==sobs['node_structure']['body'],'Whole BND body differs')
    checks['whole_table_exact']=record_gate('whole_table_exact',require,nf['table'].read_text(encoding='utf8').splitlines()[1:]==sf['table'].read_text(encoding='utf8').splitlines()[1:],'Complete table body differs')
    def warnings():
        left=ngates['error_diagnostics'];right=sgates['error_diagnostics'];assert left['passed'] and right['passed']
        for key in ['ordered_warnings','unbalanced_flow_continuations','actual_continuation_numbers','counters','completed','PurchasedAir_saturation']:assert left['observation'][key]==right['observation'][key]
        return True
    checks['warning_semantics_exact']=record_gate('warning_semantics_exact',warnings)
    checks['ancillary']=record_gate('ancillary',ancillary,nf,sf)
    return checks



def exact_empty_selector_warning_parser(files,recipe):
    lines=files['err'].read_text(encoding='utf8').splitlines();owner='ZoneHVAC:IdealLoadsAirSystem "'+recipe['IdealLoads_public_name']+'"'
    number=r'(-?\d+\.\d{5}(?:E[+-]\d{3})?)';prefix=r'\s*\*\*   ~~~   \*\* '
    oa_pattern=re.escape(owner)+r' Requested outdoor air flow rate = '+number+r' \[m3/s\] exceeds limit\.'
    recurring_messages={kind:owner+' Requested outdoor air flow rate [m3/s] reduced to Maximum '+kind+' Air Flow Rate warning continues...' for kind in ['Cooling','Heating']}
    def number_at(precision):return r'(-?\d+\.\d{'+str(precision)+r'}(?:E[+-]\d{3})?)'
    n2,n3,n4,n5=[number_at(i)for i in [2,3,4,5]]
    families={
        'Calculated Humidity Ratio invalid (PsyWFnTdbH)':dict(kind='W_FROM_ENTHALPY',recurring='Calculated Humidity Ratio invalid (PsyWFnTdbH)',units='[]',comma=False,patterns=[r' Dry-Bulb= '+n2+r' Enthalpy= '+n3,r'Calculated Humidity Ratio= '+n4+r' \.\.\. Humidity Ratio set to \.00001']),
        'Enthalpy out of range (PsyTsatFnHPb)':dict(kind='TSAT_ENTHALPY_RANGE',recurring='Enthalpy out of range (PsyTsatFnHPb)',units='J/kg',comma=True,patterns=[r' Enthalpy='+n5+r' Pressure= '+n2,r' Initial Resultant Temperature= '+n2]),
        'Temperature out of range [-100. to 200.] (PsyPsatFnTemp)':dict(kind='PSAT_RANGE',recurring='Temperature out of range [-100. to 200.] (PsyPsatFnTemp)',units='C',comma=True,patterns=[r' Input Temperature='+n2]),
        'Calculated Humidity Ratio Invalid (PsyWFnTdbTwbPb)':dict(kind='WET_BULB_HUMIDITY',recurring='Calculated Humidity Ratio Invalid (PsyWFnTdbTwbPb)',units='[]',comma=False,patterns=[r' Dry-Bulb= '+n2+r' Wet-Bulb= '+n2+r' Pressure= '+n2,r'Calculated Humidity Ratio= '+n4+r', will recalculate Humidity Ratio using Relative Humidity \.01% \(and Dry-Bulb and Pressure as shown\)']),
        'Calculated Humidity Ratio is invalid (PsyWFnTdbRhPb)':dict(kind='RH_FALLBACK_HUMIDITY',recurring='Calculated Humidity Ratio Invalid (PsyWFnTdbTwbPb)',units='[]',comma=False,patterns=[r' Dry-Bulb= '+n2+r' Relative Humidity \[%\]= '+n2+r' Pressure= '+n2,r'Calculated Humidity Ratio= '+n4+r' \.\.\. Humidity Ratio set to \.00001']),
    }
    psych_messages={row['recurring']for row in families.values()}
    psych_first_rows=[];psych_summary_rows=[]
    for at,line in enumerate(lines):
        match=re.fullmatch(r'\s*\*\* Warning \*\* (.*)',line)
        if match and match[1]in families:psych_first_rows.append(at)
        match=re.fullmatch(r'\s*\*{13}  \*\* Warning \*\* (.*)',line)
        if match and match[1]in psych_messages:psych_summary_rows.append(at)
    first_rows=[];summary_rows=[]
    for at,line in enumerate(lines):
        match=re.fullmatch(r'\s*\*\* Warning \*\* (.*)',line)
        if match and re.fullmatch(oa_pattern,match[1]):first_rows.append(at)
        match=re.fullmatch(r'\s*\*{13}  \*\* Warning \*\* (.*)',line)
        if match and match[1] in recurring_messages.values():summary_rows.append(at)
    if not first_rows and not summary_rows and not psych_first_rows and not psych_summary_rows:
        result=HVAC_humidistat_warning_diagnostics(files,recipe)
        recurrence=result['PurchasedAir_saturation']['recurring_observed_counts']
        recurring_total=0 if recurrence is None else recurrence['total']
        assert result['completed']['warnings']==len(result['ordered_warnings'])+recurring_total
        assert all(0<=phase['warnings']<=result['completed']['warnings'] and phase['severe']==0 for phase in result['counters'].values())
        result.update(OA_flow_limit_warnings={},recurring_warning_order=[],psychrometric_first=[],psychrometric_recurring={},Warning_total_accounting=dict(immediate_warning_heads=len(result['ordered_warnings']),uncounted_psychrometric_first_heads=0,counted_immediate_warning_heads=len(result['ordered_warnings']),recurring_total=recurring_total,completed=result['completed']['warnings']))
        return result
    contract=c.read(c.auth(recipe['warning_phase_contract']));adaptive=contract['inherited_adaptive_warning'];imbalance=contract['unbalanced_flow_warning']
    warnings=[];positions=[]
    for at,line in enumerate(lines):
        match=re.fullmatch(r'\s*\*\* Warning \*\* (.*)',line)
        if match:warnings.append(match[1]);positions.append(at)
    assert warnings.count(adaptive)==contract['expected_adaptive_warnings']==5
    count=warnings.count(imbalance);assert count in contract['allowed_unbalanced_flow_warning_counts']
    timestamp=prefix+r'(?: Environment=| During Warmup, Environment=| During Warmup & Sizing, Environment=| During Sizing, Environment=).+, at Simulation time=\d{2}/\d{2} .+'
    oa={};saturation=[];psych_first=[];seen_psych_heads=set();sat_pattern=re.escape(owner)+r' Supply humidity ratio = '+number+r' exceeds saturation limit '+number+r' \[kgWater/kgDryAir\]'
    sat_message=owner+' Supply humidity ratio exceeds saturation limit warning continues, delta max/min [kgWater/kgDryAir]...'
    for at,message in zip(positions,warnings,strict=True):
        match=re.fullmatch(oa_pattern,message)
        if match:
            following=re.fullmatch(prefix+r' Will be reduced to the Maximum (Cooling|Heating) Air Flow Rate = '+number+r' \[m3/s\]',lines[at+1]);assert following
            kind=following[1];assert kind not in oa and re.fullmatch(timestamp,lines[at+2])
            requested=float(match[1]);maximum=float(following[2]);assert math.isfinite(requested) and math.isfinite(maximum) and requested>=0.0 and maximum>=0.0
            oa[kind]=dict(first_warning=message,reported_requested_token=match[1],reported_limit_token=following[2],reported_requested=requested,reported_limit=maximum,first_continuations=lines[at+1:at+3],position=at)
        elif re.fullmatch(sat_pattern,message):
            match=re.fullmatch(sat_pattern,message);operands=list(map(float,match.groups()));assert all(math.isfinite(v) for v in operands) and operands[0]>=operands[1]
            assert re.fullmatch(prefix+r' Simulation continuing \. \. \. ',lines[at+1]) and re.fullmatch(timestamp,lines[at+2])
            saturation.append(dict(first_warning=message,first_reported_tokens=list(match.groups()),first_reported_operands=operands,first_continuations=lines[at+1:at+3],position=at))
        elif message in families:
            family=families[message];assert message not in seen_psych_heads;seen_psych_heads.add(message)
            routine=re.fullmatch(prefix+r' Routine=(CalcPurchAirMixedAir|CalcPurchAirLoads)'+(','if family['comma']else'')+r'(?: Environment=| During Sizing, Environment=).+, at Simulation time=\d{2}/\d{2} .+',lines[at+1]);assert routine
            tokens=[];operands=[]
            for offset,pattern in enumerate(family['patterns'],2):
                match=re.fullmatch(prefix+pattern,lines[at+offset]);assert match
                tokens.extend(match.groups());values=list(map(float,match.groups()));assert all(math.isfinite(v)for v in values);operands.extend(values)
            psych_first.append(dict(family=family['kind'],headline=message,recurring_message=family['recurring'],caller=routine[1],position=at,first_continuations=lines[at+1:at+2+len(family['patterns'])],reported_tokens=tokens,reported_operands=operands,counted_immediate_warning=False,first_event_included_in_recurring_count=True))
        else:assert message in [adaptive,imbalance]
    assert len(oa)<=2 and len(saturation)<=1 and len(warnings)==5+count+len(oa)+len(saturation)+len(psych_first)
    continuations=[];numerical=[]
    if count:
        assert warnings.index(imbalance)>=1;at=positions[warnings.index(imbalance)];continuations=lines[at+1:at+6];assert len(continuations)==5
        assert re.fullmatch(prefix+r' Environment=.+, at Simulation time=\d{2}/\d{2} .+',continuations[0])
        patterns=[r'  Flows \[m3/s\]: Inlets: (\S+)  Unbalanced exhausts: (\S+)  Returns: (\S+)',r'  Infiltration: (\S+)  Zone Ventilation: (\S+)  Mixing \(incoming\): (\S+)',r'  Imbalance \(excess outflow\): (\S+)  Total system OA flow \(for all airloops serving this zone\): (\S+)']
        for pattern,line in zip(patterns,continuations[1:4],strict=True):
            match=re.fullmatch(prefix+pattern,line);assert match;values=list(map(float,match.groups()));assert all(math.isfinite(v) for v in values);numerical.append(values)
        assert numerical[2][0]>0.0 and continuations[4]=='   **   ~~~   **   This error will only be reported once per zone.'
    summaries=[];headings=[at for at,line in enumerate(lines) if re.fullmatch(r'\s*\*{13} ===== Recurring Error Summary =====',line)];assert len(headings)<=1
    stat=r'\s*\*{13}  \*\*   ~~~   \*\*   ';seen=set();sat_recurring=None;psych_recurring={}
    for at,line in enumerate(lines):
        match=re.fullmatch(r'\s*\*{13}  \*\* Warning \*\* (.*)',line)
        if not match:continue
        message=match[1];assert message in list(recurring_messages.values())+([sat_message] if saturation else [])+list(psych_messages) and message not in seen;seen.add(message)
        assert len(headings)==1 and at>headings[0];counts=[]
        for offset,pattern in enumerate([r'This error occurred (\d+) total times;',r'during Warmup (\d+) times;',r'during Sizing (\d+) times\.'],1):
            value=re.fullmatch(stat+pattern,lines[at+offset]);assert value;counts.append(int(value[1]))
        assert counts[0]>0 and 0<=counts[1]<=counts[0] and 0<=counts[2]<=counts[0]
        details=dict(total=counts[0],warmup=counts[1],sizing=counts[2],first_event_excluded_from_recurring_count=True)
        if message==sat_message:
            assert saturation[0]['position']<headings[0];match=re.fullmatch(stat+r'Max=(-?\d+\.\d{6})  Min=(-?\d+\.\d{6})',lines[at+4]);assert match
            maximum,minimum=map(float,match.groups());assert math.isfinite(maximum) and math.isfinite(minimum) and 0.0<minimum<=maximum
            details.update(reported_delta_maximum=maximum,reported_delta_minimum=minimum);sat_recurring=details
        elif message in psych_messages:
            firsts=[row for row in psych_first if row['recurring_message']==message];assert firsts and all(row['position']<headings[0]for row in firsts)
            units={row['units']for row in families.values()if row['recurring']==message};assert len(units)==1;units=next(iter(units))
            match=re.fullmatch(stat+r'Max=(-?\d+\.\d{6}) '+re.escape(units)+r'  Min=(-?\d+\.\d{6}) '+re.escape(units),lines[at+4]);assert match
            maximum,minimum=map(float,match.groups());assert math.isfinite(maximum)and math.isfinite(minimum)and minimum<=maximum
            assert counts[1]==0 and counts[0]>=len(firsts)
            details.update(first_event_excluded_from_recurring_count=False,first_event_included_in_recurring_count=True,reported_delta_maximum=maximum,reported_delta_minimum=minimum,units=units,observed_first_families=[row['family']for row in firsts],shared_by_message_text=True,private_Psy_index_attribution=False,summary_lines=lines[at:at+5])
            psych_recurring[message]=details
        else:
            kind=next(kind for kind,text in recurring_messages.items() if text==message);assert oa[kind]['position']<headings[0]
            match=re.fullmatch(stat+r'Max=(-?\d+\.\d{6})',lines[at+4]);assert match;maximum=float(match[1]);assert math.isfinite(maximum) and maximum>=0.0
            details.update(reported_maximum=maximum,Max_only=True,units='');oa[kind]['recurring']=details;oa[kind]['recurring_summary_lines']=lines[at:at+5]
        summaries.append(message)
    assert all(row['recurring_message']in psych_recurring for row in psych_first)
    counters={}
    for phase in ['Warmup','Sizing']:
        matches=[re.fullmatch(r'\s*\*+ EnergyPlus '+phase+r' Error Summary\. During '+phase+r': (\d+) Warning; (\d+) Severe Errors\.',line) for line in lines];matches=[v for v in matches if v];assert len(matches)==1
        counters[phase]=dict(warnings=int(matches[0][1]),severe=int(matches[0][2]));assert counters[phase]['severe']==0
    assert not any(re.search(r'\*\*\s+(?:Severe|Fatal)\s+\*\*',line) for line in lines)
    completion=[re.fullmatch(r'\s*\*+ EnergyPlus Completed Successfully-- (\d+) Warning; (\d+) Severe Errors; Elapsed Time=.*',line) for line in lines];completion=[v for v in completion if v];assert len(completion)==1
    actual_completed=dict(warnings=int(completion[0][1]),severe=int(completion[0][2]));assert actual_completed['severe']==0
    recurring_total=sum(row.get('recurring',{}).get('total',0)for row in oa.values())+(0 if sat_recurring is None else sat_recurring['total'])+sum(row['total']for row in psych_recurring.values())
    counted_immediate=len(warnings)-len(psych_first)
    assert actual_completed['warnings']==counted_immediate+recurring_total
    assert all(0<=phase['warnings']<=actual_completed['warnings'] and phase['severe']==0 for phase in counters.values())
    end=files['end'].read_text(encoding='utf8').splitlines();assert len(end)==1;last=re.fullmatch(r'\s*EnergyPlus Completed Successfully-- (\d+) Warning; (\d+) Severe Errors; Elapsed Time=.*',end[0]);assert last and tuple(map(int,last.groups()))==(actual_completed['warnings'],0)
    sat=dict(first_warning_count=len(saturation),first=saturation,recurring_summary_present=sat_recurring is not None,recurring_observed_counts=sat_recurring)
    return dict(ERR=c.ref(files['err']),END=c.ref(files['end']),warning_contract=recipe['warning_phase_contract'],selector_contract=recipe['exact_empty_selector_science_contract'],ordered_warnings=warnings,unbalanced_flow_continuations=continuations,actual_continuation_numbers=numerical,counters=counters,completed=actual_completed,unbalanced_warning_count=count,PurchasedAir_saturation=sat,OA_flow_limit_warnings=oa,recurring_warning_order=summaries,psychrometric_first=psych_first,psychrometric_recurring=psych_recurring,Warning_total_accounting=dict(immediate_warning_heads=len(warnings),uncounted_psychrometric_first_heads=len(psych_first),counted_immediate_warning_heads=counted_immediate,recurring_total=recurring_total,completed=actual_completed['warnings']),no_fixed_occurrence_or_phase_counts=True,qualification='Exact old adaptive/unbalanced/SAT/OA families plus five owned PurchasedAir Psy first families, four message-deduplicated recurring rows and unit-specific max/min. Psy first ShowWarningMessage is uncounted and recurrence includes that event; global phase counters remain observed. Full original ERR/END remains paired, unknown families/callers/Severe fail; no raw inequality, private index or call-count inference.')



def selector_warning_diagnostics(files,recipe):
    lines=files['err'].read_text(encoding='utf8').splitlines()
    count=sum('Resetting Demand Controlled Ventilation Type to NoDCV'in line for line in lines)
    assert count==0,'Exact-empty DCV must not enter the named CO2 reset warning path'
    value=exact_empty_selector_warning_parser(files,recipe)
    value.update(CO2_reset_warning_forbidden=True,observed_CO2_reset_warnings=count)
    return value



def science_gates153(files,answer,recipe):
    old,data,bindings=answer['eso'];exhaust_recipe=dict(recipe)
    baseline=c.read(c.auth(recipe['inherited_exhaust_baseline_recipe']))
    exhaust_requests=[row for row in recipe['physical_output_requests']if row[1]['key_value']==recipe['new_exhaust_node']]
    assert exhaust_requests==baseline['physical_output_requests'] and len(exhaust_requests)==3
    exhaust_recipe['physical_output_requests']=exhaust_requests
    calls=[('actual_status',statuses,(old,bindings)),('actual_cen',cen_statuses,(old,bindings)),('scheduled_setpoint',scheduled_output,(old,bindings,recipe)),('actual_exhaust',actual_exhaust_output,(old,bindings,exhaust_recipe)),('public_return_flow',public_return_flow,(old,bindings,recipe)),('public_cooling_availability',cooling_availability,(data,bindings,recipe)),('public_heating_availability',heating_availability,(data,bindings,recipe)),('public_heating_fuel_efficiency',heating_fuel_efficiency,(data,bindings,recipe)),('winter_weather_activation',winter_weather_activation,(data,bindings,recipe)),('adaptive_summary',table,(files['table'],recipe)),('error_diagnostics',selector_warning_diagnostics,(files,recipe)),('public_outdoor_air_mixing',public_outdoor_air_mixing,(data,bindings,recipe))]
    return {name:record_gate(name,function,*arguments)for name,function,arguments in calls}



def science_gates158(files,answer,recipe):
    warning_recipe=dict(recipe)
    warning_recipe['exact_empty_selector_science_contract']=recipe['science_contract']
    return science_gates153(files,answer,warning_recipe)
