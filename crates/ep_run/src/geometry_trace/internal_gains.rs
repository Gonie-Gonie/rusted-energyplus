//! Output-only copies of the retained Source internal-gain owners.
use super::bits;
use ep_runtime::heat_balance::ctf_initialization_context::{
    OrdinaryGainDevice, OrdinaryGainDeviceStorage, OrdinaryGainMiscObject, OrdinaryGainRateTarget,
    OrdinaryGainReport, OrdinaryInternalGains, OrdinaryOtherEquipment,
};
use serde_json::{Value, json};

fn scalar(value: f64) -> Value {
    json!({"value":value,"value_bits":bits(value)})
}

fn collection(values: Option<Vec<Value>>) -> Value {
    json!({"storage_kind":"Rust Option<Vec>","allocated":values.is_some(),
        "size":values.as_ref().map_or(0,Vec::len),
        "operator_parentheses_indexing":"Source one-based rows; Rust zero-based storage",
        "values":values.unwrap_or_default().into_iter().enumerate()
            .map(|(index,value)| json!({"index":index+1,"value":value})).collect::<Vec<_>>()})
}

fn array(values: Option<&[f64]>) -> Value {
    let size = values.map_or(0, <[f64]>::len);
    json!({"storage_kind":"Rust Option<Vec>","allocated":values.is_some(),"size":size,
        "lower":values.map(|_|1),"upper":values.map(|_|size),
        "values":values.unwrap_or_default().iter().enumerate()
            .map(|(index,value)|json!({"index":index+1,"value":scalar(*value)})).collect::<Vec<_>>()})
}

fn fixed(values: &[f64; 26]) -> Value {
    json!({"storage_kind":"Rust fixed [f64;26]","lower":0,"upper":25,"size":values.len(),
        "dynamic_allocation_flag_pairing":"UNPAIRED; fixed Rust storage has no dynamic flag",
        "values":values.iter().enumerate().map(|(index,value)|
            json!({"index":index,"value":scalar(*value)})).collect::<Vec<_>>()})
}

fn equipment(row: OrdinaryOtherEquipment<'_>) -> Value {
    let mut value = json!({"Name":row.name,"spaceIndex":row.space_index,"ZonePtr":row.zone_ptr,
        "sched":{"kind":if row.schedule_index.is_some(){"owned_schedule"}else{"null"},
            "storage_index":row.schedule_index},"OtherEquipFuelType":row.fuel_code,
        "EndUseSubcategory":row.end_use_subcategory,
        "DesignLevel":scalar(row.design_level),"NomMinDesignLevel":scalar(row.nominal_min_design_level),
        "NomMaxDesignLevel":scalar(row.nominal_max_design_level),"FractionLatent":scalar(row.fraction_latent),
        "FractionRadiant":scalar(row.fraction_radiant),"FractionLost":scalar(row.fraction_lost),
        "FractionConvected":scalar(row.fraction_convected),"CO2RateFactor":scalar(row.co2_rate_factor)});
    for (name, stored) in [
        "Power",
        "RadGainRate",
        "ConGainRate",
        "LatGainRate",
        "LostRate",
        "TotGainRate",
        "CO2GainRate",
    ]
    .into_iter()
    .zip(row.rates)
    {
        value[name] = scalar(*stored);
    }
    for (name, stored) in [
        "Consumption",
        "RadGainEnergy",
        "ConGainEnergy",
        "LatGainEnergy",
        "LostEnergy",
        "TotGainEnergy",
    ]
    .into_iter()
    .zip(row.energies)
    {
        value[name] = scalar(*stored);
    }
    value
}

fn report(row: OrdinaryGainReport<'_>) -> Value {
    let mut value = json!({});
    for (index, name) in [
        "TotRadiantGain",
        "TotVisHeatGain",
        "TotConvectiveGain",
        "TotLatentGain",
        "TotTotalHeatGain",
    ]
    .into_iter()
    .enumerate()
    {
        value[name] = scalar(row.total[index * 2]);
        value[format!("{name}Rate")] = scalar(row.total[index * 2 + 1]);
    }
    for (index, name) in [
        "OtherRadGain",
        "OtherConGain",
        "OtherLatGain",
        "OtherLost",
        "OtherTotGain",
    ]
    .into_iter()
    .enumerate()
    {
        value[name] = scalar(row.other[index * 2]);
        value[format!("{name}Rate")] = scalar(row.other[index * 2 + 1]);
    }
    value
}

fn target(value: Option<OrdinaryGainRateTarget>) -> Value {
    match value {
        None => json!({"kind":"null"}),
        Some(OrdinaryGainRateTarget::ZeroPointerValue) => json!({"kind":"zeroPointerVal"}),
        Some(OrdinaryGainRateTarget::OtherEquipment { row_index, field }) => {
            json!({"kind":"ZoneOtherEq","index":row_index+1,"field":field})
        }
    }
}

fn device(row: OrdinaryGainDevice<'_>) -> Value {
    let mut value = json!({"CompObjectName":row.component_name,"CompType":row.component_type,
        "spaceGainFrac":scalar(row.space_gain_fraction),"ReturnAirNodeNum":row.return_air_node});
    let fields = [
        ("ConvectGainRate", "PtrConvectGainRate"),
        ("ReturnAirConvGainRate", "PtrReturnAirConvGainRate"),
        ("RadiantGainRate", "PtrRadiantGainRate"),
        ("LatentGainRate", "PtrLatentGainRate"),
        ("ReturnAirLatentGainRate", "PtrReturnAirLatentGainRate"),
        ("CarbonDioxideGainRate", "PtrCarbonDioxideGainRate"),
        ("GenericContamGainRate", "PtrGenericContamGainRate"),
    ];
    for (index, (rate, pointer)) in fields.into_iter().enumerate() {
        value[rate] = scalar(row.cached_rates[index]);
        value[pointer] = target(row.targets[index]);
    }
    value
}

fn devices(row: OrdinaryGainDeviceStorage<'_>) -> Value {
    json!({"storage_kind":"Rust Option<Vec>","allocated":row.storage_size.is_some(),
        "size":row.storage_size.unwrap_or(0),"lower":row.storage_size.map(|_|1),"upper":row.storage_size,
        "numberOfDevices":row.number_of_devices,"maxNumberOfDevices":row.max_number_of_devices,
        "unused_capacity_values_copied":false,
        "copy_reason":if row.active_values.is_none(){Some("actual_device_count_outside_actual_allocated_storage")}else{None},
        "active_values":row.active_values.map(|v|v.into_iter().enumerate().map(|(index,row)|
            json!({"index":index+1,"value":device(row)})).collect::<Vec<_>>())})
}

fn misc(row: OrdinaryGainMiscObject<'_>) -> Value {
    json!({"Name":row.name,"spaceStartPtr":row.space_start_ptr,"numOfSpaces":row.num_spaces,
        "spaceNums":row.space_nums,"names":row.names,
        "nested_storage_kind":"Rust Vec; Native nested EPVector allocation flags UNPAIRED"})
}

fn selected_dictionary_name(name: &str) -> bool {
    let total = [
        "Radiant Heating Energy",
        "Radiant Heating Rate",
        "Visible Radiation Heating Energy",
        "Visible Radiation Heating Rate",
        "Convective Heating Energy",
        "Convective Heating Rate",
        "Latent Gain Energy",
        "Latent Gain Rate",
        "Total Heating Energy",
        "Total Heating Rate",
    ];
    let other = [
        "Radiant Heating Energy",
        "Radiant Heating Rate",
        "Convective Heating Energy",
        "Convective Heating Rate",
        "Latent Gain Energy",
        "Latent Gain Rate",
        "Lost Heat Energy",
        "Lost Heat Rate",
        "Total Heating Energy",
        "Total Heating Rate",
    ];
    total.iter().any(|suffix| {
        ["Zone Total Internal ", "Space Total Internal "]
            .iter()
            .any(|prefix| name == format!("{prefix}{suffix}"))
    }) || other.iter().any(|suffix| {
        [
            "Other Equipment ",
            "Zone Other Equipment ",
            "Space Other Equipment ",
        ]
        .iter()
        .any(|prefix| name == format!("{prefix}{suffix}"))
    })
}

pub(super) fn projection(owners: OrdinaryInternalGains<'_>) -> Value {
    let output = owners.output_registration;
    let schedules=owners.schedules.map(|manager|json!({
        "ScheduleInputProcessed":manager.input_processed,
        "scheduleMap":manager.schedule_map.into_iter().collect::<std::collections::BTreeMap<_,_>>(),
        "storage_size":manager.rows.len(),"storage_kind":"Rust owned Vec; Native pointer presence UNPAIRED",
        "values":manager.rows.into_iter().enumerate().map(|(index,row)|json!({"storage_index":index,"owner":{
            "Name":row.name,"Num":row.number,"schedTypeNum":row.schedule_type_number,
            "isUsed":row.is_used,"isMinMaxSet":row.is_min_max_set,"minVal":scalar(row.min_value),
            "maxVal":scalar(row.max_value),"currentVal":scalar(row.current_value),
            "EMSActuatedOn":row.ems_actuated_on,"EMSVal":scalar(row.ems_value)}})).collect::<Vec<_>>(),
        "copy_scope":"same actual schedule manager; no accessor calls; type enum and pointer presence UNPAIRED"}));
    let mut value = json!({"schema":"ordinary-source-internal-gains.v1",
        "capture_boundary":"successful retained Rust initialization owners; not an internal HB320 checkpoint or full parent return",
        "GetInternalHeatGainsInputFlag":owners.get_input_flag,"ErrorsFound":owners.errors_found,
        "zeroPointerVal":scalar(owners.zero_pointer_value),
        "ZoneCO2Gen_allocation":{"storage_kind":"Rust Option<Vec>","allocated":owners.co2_generation_size.is_some(),
            "size":owners.co2_generation_size.unwrap_or(0)},
        "ZoneIntGain":collection(owners.zone_int_gain.map(|v|v.iter().map(|r|json!({"NOFOCC":scalar(r[0]),"QLTSW":scalar(r[1])})).collect())),
        "spaceIntGain":collection(owners.space_int_gain.map(|v|v.iter().map(|r|json!({"NOFOCC":scalar(r[0]),"QLTSW":scalar(r[1])})).collect())),
        "spaceIntGainDevices":collection(owners.space_devices.map(|v|v.into_iter().map(devices).collect())),
        "ZoneOtherEq":collection(owners.other_equipment.map(|v|v.into_iter().map(equipment).collect())),
        "ZoneRpt":collection(owners.zone_report.map(|v|v.into_iter().map(report).collect())),
        "spaceRpt":collection(owners.space_report.map(|v|v.into_iter().map(report).collect())),
        "lightsObjects":collection(owners.lights_objects.map(|v|v.into_iter().map(misc).collect())),
        "zoneElectricObjects":collection(owners.electric_objects.map(|v|v.into_iter().map(misc).collect())),
        "spacePowerReductionFactor":array(owners.space_power_reduction_factor),
        "ZoneIntEEuse":collection(owners.zone_electric_end_use.map(|v|v.iter().map(|r|json!({
            "EEConvected":fixed(&r[0]),"EERadiated":fixed(&r[1]),"EELost":fixed(&r[2]),"EELatent":fixed(&r[3])})).collect())),
        "RefrigCaseCredit":collection(owners.refrigeration_credit.map(|v|v.iter().map(|r|json!({
            "SenCaseCreditToZone":scalar(r[0]),"LatCaseCreditToZone":scalar(r[1]),
            "SenCaseCreditToHVAC":scalar(r[2]),"LatCaseCreditToHVAC":scalar(r[3])})).collect())),
        "zones":collection(owners.zones.map(|v|v.into_iter().map(|r|json!({"Name":r.name,
            "TotOccupants":scalar(r.total_occupants),"InternalHeatGains":scalar(r.internal_heat_gains)})).collect())),
        "spaces":collection(owners.spaces.map(|v|v.into_iter().map(|r|json!({"Name":r.name,"zoneNum":r.zone_number})).collect())),
        "schedules":schedules,
        "output_registration":{
            "global_counters":{"NumTotalRVariable":output.num_total_real_variables,
                "NumOfRVariable":output.num_real_variables,"NumOfRVariable_Setup":output.num_real_variables_setup,
                "NumOfRVariable_Sum":output.num_real_variables_sum,"NumOfRVariable_Meter":output.num_real_variables_meter},
            "report_number_counter":output.report_number_counter,
            "global_counters_IHG_only_pairing":"UNPAIRED; shared earlier/later producers",
            "IHG_per_call_key_order_history":null,"IHG_per_call_key_order_history_scope":"UNPAIRED; no retained call ledger",
            "ddOutVars_storage_size":output.dictionary_storage_size,
            "selected_DD_entries":output.dictionary.into_iter().filter(|row|selected_dictionary_name(row.name)).map(|row|json!({
                "storage_index":row.storage_index,"name":row.name,"storeType":row.store_type,"units":row.units,
                "integer":row.integer,"Next":row.next,"ReportedOnDDFile":row.reported_on_dictionary_file,
                "keyOutVarNums":row.key_out_var_nums})).collect::<Vec<_>>(),
            "selected_keyed_OutVars":null,
            "dictionary_copy_scope":"actual retained subset/order; symbolic stored enums/Option Next, not60-call ledger"},
        "unpaired_fields":["ZoneOtherEq.EMSZoneEquipOverrideOn","ZoneOtherEq.ManageDemand","ZoneOtherEq.EMSEquipPower",
            "ZoneOtherEq.CO2DesignRate","ZoneOtherEq.DemandLimit","ZoneOtherEq.otherEquipFuelTypeString",
            "misc.ZoneListActive","misc.spaceListActive","misc.spaceOrSpaceListPtr","misc.nested EPVector allocation flags",
            "spaces.TotOccupants","schedules.type","schedules.pointer_present","fixed EE dynamic allocation flags",
            "output_registration.NumVariablesForOutput","output_registration.ddOutVarMap",
            "output_registration.timeStepType","output_registration.selected_keyed_OutVars","output_registration.per-call history"],
        "unavailable_locals":["six getter argument arrays","deallocated RepVarSet","per-query/call history"],
        "values_reconstructed_or_expected_values_injected":false});
    for (name, count) in [
        "TotPeople",
        "TotLights",
        "TotElecEquip",
        "TotGasEquip",
        "TotHWEquip",
        "TotStmEquip",
        "TotOthEquip",
        "TotITEquip",
        "TotBBHeat",
        "TotCO2Gen",
    ]
    .into_iter()
    .zip(owners.totals)
    {
        value[name] = json!(*count);
    }
    value
}
