//! Full genuine Native report entry/exit cohort; no report-formula mirrors.
//! Engine cases and later explicit Native-unit input variants retain separate origins.
use super::*;
use crate::heat_balance::ordinary_system_reports::SourceSystemReports;
use crate::heat_balance::ordinary_thermal_comfort::SourceComfortPredefinedSummary;
use crate::heat_balance::selected_heat_balance_input::zone::ZonePredefined;
use crate::heat_balance::ordinary_equipment::source_purchased_air::owners::SourceZonePurchasedAir;
include!("../fixtures/native_system_report_b.rs");
const ZONE_FIELDS:[&str;20]=["CoolingLoadMetByVent", "CoolingLoadAddedByVent", "OvercoolingByVent", "HeatingLoadMetByVent", "HeatingLoadAddedByVent", "OverheatingByVent", "NoLoadHeatingByVent", "NoLoadCoolingByVent", "OAMassFlow", "OAMass", "OAVolFlowStdRho", "OAVolStdRho", "OAVolFlowCrntRho", "OAVolCrntRho", "MechACH", "TargetVentilationFlowVoz", "TimeBelowVozDyn", "TimeAtVozDyn", "TimeAboveVozDyn", "TimeVentUnocc"];
fn index(n:i32)->Result<usize>{Ok(usize::try_from(n.checked_sub(1).ok_or_else(||invalid("report positive ordinal"))?)?)}
fn row_value(v:&Value)->Result<&Value>{let x=member(v,"value")?;if x.is_null(){return Err(invalid("missing captured actual row").into());}Ok(x)}
fn allocated(v:&Value)->Result<usize>{assert!(boolean(member(v,"allocated")?)?);count(member(v,"size")?)}
fn bind(ids:&mut BTreeMap<(String,usize),usize>,kind:&str,native:&Value,source:usize)->Result {
    let native=count(member(native,"address")?)?;assert!(native>0 && source>0);
    let key=(kind.to_owned(),native);
    if let Some(old)=ids.get(&key){assert_eq!(*old,source,"same {kind} physical row across invocations");}
    else {assert!(!ids.iter().any(|((k,n),s)|k==kind && *n!=native && *s==source),"distinct actual {kind} rows stay distinct");ids.insert(key,source);}
    Ok(())
}
fn identity_name(fixture:&Value,key:&str,ordinal:i32)->Result<String>{
    let rows=array(member(fixture,key)?)?;
    let row=rows.iter().find(|v|member(v,"index").and_then(integer).is_ok_and(|n|n==ordinal)).ok_or_else(||invalid("captured identity ordinal"))?;
    Ok(text(member(row,"name")?)?.to_owned())
}
fn seed_config(equipment:&mut OrdinaryEquipment,fixture:&Value,zone:&Value,vent:bool,ids:&mut BTreeMap<(String,usize),usize>)->Result {
    let number=integer(member(zone,"index")?)?;let outer=member(zone,"config")?;allocated(outer)?;
    assert_eq!(integer(member(outer,"index")?)?,number);let raw=row_value(outer)?;
    let controlled=boolean(member(raw,"IsControlled")?)?;
    let mut inlet_values=None;
    if controlled {
        let n=count(member(raw,"NumInletNodes")?)?;let shape=member(raw,"InletNodeAirLoopNum")?;
        if boolean(member(shape,"allocated")?)? {
            assert_eq!(allocated(shape)?,n);let values=array(member(shape,"values")?)?;assert_eq!(values.len(),n);
            let values=values.iter().map(integer).collect::<Result<Vec<_>>>()?;
            // This is an observed replay-domain check, never a production skip.
            assert!(values.iter().all(|v|*v==0),"positive primary child not represented by Source39");inlet_values=Some(values);
        }else{assert_eq!(n,0);}
    }
    let indices=if vent && controlled {
        let dsoa=integer(member(raw,"ZoneDesignSpecOAIndex")?)?;let distribution=integer(member(raw,"ZoneAirDistributionIndex")?)?;
        assert_eq!(dsoa,0,"positive DSOA child outside this replay");assert_eq!(distribution,0,"positive air distribution outside this replay");
        Some((integer(member(raw,"EquipListIndex")?)?,dsoa,distribution))
    }else{None};
    let address=equipment.restore_native_report_config(number,&identity_name(fixture,"zone_names",number)?,controlled,inlet_values.as_deref(),indices)?;
    bind(ids,"Config",raw,address)
}
fn seed_node(equipment:&mut OrdinaryEquipment,fixture:&Value,raw:&Value,supply:bool,ids:&mut BTreeMap<(String,usize),usize>)->Result {
    let number=integer(member(raw,"index")?)?;allocated(raw)?;let value=row_value(raw)?;
    let (t,w,m)=if supply{(None,None,Some(real(member(value,"MassFlowRate")?)?))}else{(Some(real(member(value,"Temp")?)?),Some(real(member(value,"HumRat")?)?),None)};
    let address=equipment.restore_native_report_node(number,&identity_name(fixture,"node_ids",number)?,t,w,m)?;
    bind(ids,"Node",value,address)
}
fn check_node(equipment:&OrdinaryEquipment,raw:&Value,supply:bool,context:&str,ids:&mut BTreeMap<(String,usize),usize>)->Result {
    let n=equipment.source_zone_air_node(integer(member(raw,"index")?)?)?;let value=row_value(raw)?;
    bind(ids,"Node",value,std::ptr::from_ref(n.temperature) as usize)?;
    if supply{assert_real(*n.mass_flow_rate,member(value,"MassFlowRate")?,context,"physical supply Mdot unchanged")?;}
    else{assert_real(*n.temperature,member(value,"Temp")?,context,"physical recirc T unchanged")?;assert_real(*n.humidity_ratio,member(value,"HumRat")?,context,"physical recirc W unchanged")?;}
    Ok(())
}
fn seed_units(equipment:&mut OrdinaryEquipment,fixture:&Value,zone:&Value,ids:&mut BTreeMap<(String,usize),usize>)->Result {
    let list=member(zone,"equipment_list")?;allocated(list)?;let list_number=integer(member(list,"index")?)?;let list_value=row_value(list)?;
    let entries=array(member(list_value,"equipment")?)?;assert_eq!(entries.len(),count(member(list_value,"NumOfEquipTypes")?)?);
    for entry in entries {
        let kind=integer(member(entry,"EquipType")?)?;let ordinal=integer(member(entry,"index")?)?;let unit_index=integer(member(entry,"EquipIndex")?)?;
        let address=equipment.restore_native_report_equipment_index(list_number,ordinal,kind,unit_index)?;bind(ids,"EquipmentList",list_value,address)?;
        assert_eq!(kind,12,"this cohort observes the actual PurchasedAir normal domain");
        let p=member(entry,"PurchasedAir")?;assert_eq!(integer(member(p,"index")?)?,unit_index);let size=allocated(p)?;let number=integer(member(p,"NumPurchAir")?)?;
        assert_eq!(size,usize::try_from(number)?);
        equipment.purchased_air.number_purchased_air=number;
        equipment.purchased_air.get_purchased_air_input=boolean(member(p,"GetPurchAirInputFlag")?)?;
        assert!(!equipment.purchased_air.get_purchased_air_input,"true lazy input remains Source39 child boundary");
        if let Some(rows)=&equipment.purchased_air.units{assert_eq!(rows.len(),size);}else{equipment.purchased_air.units=Some(vec![SourceZonePurchasedAir::default();size]);}
        let unit=equipment.purchased_air.units.as_mut().and_then(|rows|rows.get_mut(index(unit_index).ok()?)).ok_or_else(||invalid("actual unit ordinal"))?;
        let value=row_value(p)?;
        seed_reals!(unit,value,{"OutdoorAirMassFlowRate"=>outdoor_air_mass_flow_rate,"MixedAirTemp"=>mixed_air_temp,"MixedAirHumRat"=>mixed_air_hum_rat});
        unit.zone_supply_air_node_num=integer(member(value,"ZoneSupplyAirNodeNum")?)?;unit.zone_recirc_air_node_num=integer(member(value,"ZoneRecircAirNodeNum")?)?;
        bind(ids,"PurchasedAir",value,std::ptr::from_ref(unit) as usize)?;
        if let Some(n)=object(p)?.get("supply_node"){seed_node(equipment,fixture,n,true,ids)?;}
        if let Some(n)=object(p)?.get("recirc_node"){seed_node(equipment,fixture,n,false,ids)?;}
    }
    Ok(())
}
fn check_units(equipment:&OrdinaryEquipment,zone:&Value,context:&str,ids:&mut BTreeMap<(String,usize),usize>)->Result {
    let list=member(zone,"equipment_list")?;let value=row_value(list)?;let source=equipment.source_zone_equipment_list(integer(member(list,"index")?)?)?;
    for e in array(member(value,"equipment")?)? {
        let number=integer(member(e,"EquipIndex")?)?;let ordinal=usize::try_from(integer(member(e,"index")?)?)?;
        assert_eq!(*source.equipment_index(ordinal).ok_or_else(||invalid("same EquipIndex"))?,number);
        assert_eq!(*source.equipment_type(ordinal).ok_or_else(||invalid("same EquipType"))?,integer(member(e,"EquipType")?)?);
        let p=member(e,"PurchasedAir")?;assert_eq!(equipment.purchased_air.get_purchased_air_input,boolean(member(p,"GetPurchAirInputFlag")?)?);
        assert_eq!(equipment.purchased_air.number_purchased_air,integer(member(p,"NumPurchAir")?)?);
        let u=equipment.purchased_air.units.as_ref().and_then(|rows|rows.get(index(number).ok()?)).ok_or_else(||invalid("same unit"))?;let v=row_value(p)?;
        bind(ids,"PurchasedAir",v,std::ptr::from_ref(u) as usize)?;
        check_reals!(u,v,context,{"OutdoorAirMassFlowRate"=>outdoor_air_mass_flow_rate,"MixedAirTemp"=>mixed_air_temp,"MixedAirHumRat"=>mixed_air_hum_rat});
        assert_eq!(u.zone_supply_air_node_num,integer(member(v,"ZoneSupplyAirNodeNum")?)?);assert_eq!(u.zone_recirc_air_node_num,integer(member(v,"ZoneRecircAirNodeNum")?)?);
        if let Some(n)=object(p)?.get("supply_node"){check_node(equipment,n,true,context,ids)?;}
        if let Some(n)=object(p)?.get("recirc_node"){check_node(equipment,n,false,context,ids)?;}
    }
    Ok(())
}
fn report_values(raw:&Value)->Result<[f64;20]>{let mut out=[0.0;20];for (i,name) in ZONE_FIELDS.iter().enumerate(){out[i]=real(member(raw,name)?)?;}Ok(out)}
pub(super) fn run()->Result<(usize,usize,usize)> {
    let fixture:Value=serde_json::from_str(NATIVE_SYSTEM_REPORT_COHORT_JSON)?;
    let mut input=input(&fixture)?;
    let inventory=object(member(&fixture,"raw")?)?.iter().map(|(f,o)|Ok((f.as_str(),object(o)?.len()))).collect::<Result<Vec<_>>>()?;
    let admitted=OrdinaryEquipmentInput::from_source_family_inventory(inventory,4)?;let model=identity_model();
    let mut equipment=OrdinaryEquipment::new_for_source(&model,admitted)?;
    let mut globals=SelectedHeatBalanceGlobals::default();globals.num_of_zones=i32::try_from(model.zones.len())?;globals.num_spaces=i32::try_from(model.spaces.len())?;
    assert_eq!(array(member(&fixture,"zone_names")?)?.len(),model.zones.len());
    let mut zones=SelectedZoneInputOwners::default();
    zones.zones=Some(vec![SelectedZoneData {name:"ZONE ONE".into(),typed_zone_id:Some(ep_model::ZoneId(0)),space_indexes:vec![1],num_spaces:1,..Default::default()}]);
    let mut schedules=SelectedScheduleManager::default();schedules.init_constant_state(4)?;
    let mut sizing=SelectedSizingManager::default();let mut branch=SourceBranchNodeConnections::default();let mut gains=SelectedInternalHeatGains::default();
    equipment.get_zone_equipment_source(&mut input,admitted,false,false,&globals,&mut gains,
        SourceEquipmentInputOwners {zones:zones.zones.as_deref_mut().ok_or_else(||invalid("actual Zone"))?,schedules:&mut schedules,sizing:&mut sizing,branch:Some(&mut branch),source_is_epjson:false})?;
    let space_controls=SelectedHeatBalanceSpaceControls::default();let mut controller=SourceZoneAirPredictorCorrector::default();
    controller.demand.allocate_source_demands(&globals,&space_controls)?;controller.reports.allocate_source_reports(&globals,&space_controls)?;
    let mut air=SourceAirHeatBalance::default();air.allocate_source_arrays(&globals)?;
    zones.zone_predefined=Some(vec![ZonePredefined::default();model.zones.len()]);
    let mut reports=SourceSystemReports::default();let mut summary=SourceComfortPredefinedSummary::default();
    let mut hvac=SelectedHvacGlobals::default();let mut environment=WeatherEnvironmentState::default();
    // Separate genuine Native process/state lifetimes never assert cross-process addresses.
    let mut identity_groups:BTreeMap<String,BTreeMap<(String,usize),usize>>=BTreeMap::new();let mut energy=0;let mut vent=0;let mut variants=0;let mut thermal_nonzero=[false;8];
    for case in array(member(&fixture,"cases")?)? {
        let method=text(member(case,"method")?)?;let is_vent=method=="ReportVentilationLoads";assert!(is_vent || method=="ReportSystemEnergyUse");
        let origin=member(case,"origin")?;let kind=text(member(origin,"kind")?)?;
        assert!(kind=="engine" || kind=="native-unit-variant");if kind=="native-unit-variant"{variants+=1;}
        let entry=member(case,"entry")?;let expected=member(case,"expected_exit")?;
        let state=member(entry,"state")?;let flags=member(state,"flags")?;let clock=member(entry,"clock")?;
        // Engine calls share one real state lifetime. Each full Native unit call
        // constructs a fresh EnergyPlusData, even if its allocator reuses an address.
        let identity_scope=if kind=="engine" {
            format!("engine:{}",count(member(clock,"state_address")?)?)
        } else {
            format!("native-unit:{method}:{}",integer(member(entry,"invocation_ordinal")?)?)
        };
        let identities=identity_groups.entry(identity_scope).or_default();
        let context=format!("Native report {method} origin{kind} invocation{}",integer(member(entry,"invocation_ordinal")?)?);
        globals.num_of_zones=integer(member(flags,"NumOfZones")?)?;assert_eq!(usize::try_from(globals.num_of_zones)?,model.zones.len());
        hvac.number_of_primary_air_systems=integer(member(flags,"NumPrimaryAirSys")?)?;assert_eq!(hvac.number_of_primary_air_systems,0);
        hvac.time_step_sys=real(member(clock,"TimeStepSys")?)?;hvac.time_step_sys_seconds=real(member(clock,"TimeStepSysSec")?)?;
        reports.air_loop_loads_report_enabled=boolean(member(flags,"AirLoopLoadsReportEnabled")?)?;
        reports.vent_report_structure_created=boolean(member(flags,"VentReportStructureCreated")?)?;
        reports.vent_loads_report_enabled=boolean(member(flags,"VentLoadsReportEnabled")?)?;
        reports.vent_energy_report_enabled=boolean(member(flags,"VentEnergyReportEnabled")?)?;
        let enabled=if is_vent{reports.vent_report_structure_created && reports.vent_loads_report_enabled}else{reports.air_loop_loads_report_enabled};
        if enabled {
            if is_vent {
                environment.std_rho_air=real(member(state,"StdRhoAir")?)?;environment.current_weather.out_baro_press=real(member(clock,"OutBaroPress")?)?;
                seed_reals!(reports,member(state,"facility_report")?,{"AnyZoneTimeBelowVozDyn"=>any_zone_time_below_voz_dyn,"AllZonesTimeAtVozDyn"=>all_zones_time_at_voz_dyn,"AnyZoneTimeAboveVozDyn"=>any_zone_time_above_voz_dyn,"AnyZoneTimeVentUnocc"=>any_zone_time_vent_unocc,"AnyZoneTimeBelowVozDynOcc"=>any_zone_time_below_voz_dyn_occ,"AllZonesTimeAtVozDynOcc"=>all_zones_time_at_voz_dyn_occ,"AnyZoneTimeAboveVozDynOcc"=>any_zone_time_above_voz_dyn_occ});
                seed_reals!(summary,member(state,"facility_predefined_totals")?,{"TotalAnyZoneBelowVozDynForOA"=>total_any_zone_below_voz_dyn_for_oa,"TotalAllZonesAtVozDynForOA"=>total_all_zones_at_voz_dyn_for_oa,"TotalAnyZoneAboveVozDynForOA"=>total_any_zone_above_voz_dyn_for_oa,"TotalAnyZoneVentUnoccForOA"=>total_any_zone_vent_unocc_for_oa,"TotalAnyZoneBelowVozDynOccForOA"=>total_any_zone_below_voz_dyn_occ_for_oa,"TotalAllZonesAtVozDynOccForOA"=>total_all_zones_at_voz_dyn_occ_for_oa,"TotalAnyZoneAboveVozDynOccForOA"=>total_any_zone_above_voz_dyn_occ_for_oa});
            }
            for z in array(member(state,"zones")?)? {
                let number=integer(member(z,"index")?)?;let i=index(number)?;seed_config(&mut equipment,&fixture,z,is_vent,identities)?;
                if is_vent {
                    let report=member(z,"report")?;reports.prepare_native_report_cohort_rows(allocated(report)?)?;
                    let address=reports.restore_native_report_cohort_row(number,report_values(row_value(report)?)?)?;bind(identities,"ZoneVent",row_value(report)?,address)?;
                }
                let config=row_value(member(z,"config")?)?;
                if !boolean(member(config,"IsControlled")?)?{continue;}
                let d=member(z,"demand")?;let demand=controller.demand.zones.as_mut().and_then(|r|r.get_mut(i)).ok_or_else(||invalid("same demand"))?;
                demand.base.total_output_required=real(member(d,"TotalOutputRequired")?)?;bind(identities,"Demand",d,std::ptr::from_ref(demand) as usize)?;
                // This input array is allocated separately from demand rows in Native2390.
                let deadband=member(z,"DeadBandOrSetback")?;let deadband_size=allocated(deadband)?;
                assert_eq!(deadband_size,model.zones.len());assert_eq!(integer(member(deadband,"index")?)?,number);
                assert_eq!(integer(member(deadband,"lower")?)?,1);assert_eq!(count(member(deadband,"upper")?)?,deadband_size);
                if let Some(rows)=&controller.demand.deadband_or_setback {assert_eq!(rows.len(),deadband_size);}
                else {controller.demand.deadband_or_setback=Some(vec![false;deadband_size]);}
                *controller.demand.deadband_or_setback.as_mut().and_then(|r|r.get_mut(i)).ok_or_else(||invalid("same DeadBand"))?=bool_integer(member(deadband,"value")?)?;
                if !is_vent{continue;}
                let zone=member(z,"zone")?;let actual=zones.zones.as_mut().and_then(|r|r.get_mut(i)).ok_or_else(||invalid("same Zone"))?;
                assert_eq!(actual.name,text(member(zone,"Name")?)?);actual.multiplier=integer(member(zone,"Multiplier")?)?;actual.list_multiplier=integer(member(zone,"ListMultiplier")?)?;actual.volume=real(member(zone,"Volume")?)?;
                bind(identities,"Zone",zone,std::ptr::from_ref(actual) as usize)?;
                let predef=row_value(member(z,"predefined")?)?;let actual=zones.zone_predefined.as_mut().and_then(|r|r.get_mut(i)).ok_or_else(||invalid("same predef"))?;
                actual.is_occupied=boolean(member(predef,"isOccupied")?)?;seed_reals!(actual,predef,{"AFNVentVolStdDen"=>afn_vent_vol_std_den,"TotTimeOcc"=>tot_time_occ,"MechVentVolTotalOcc"=>mech_vent_vol_total_occ,"MechVentVolMin"=>mech_vent_vol_min,"MechVentVolTotalOccStdDen"=>mech_vent_vol_total_occ_std_den,"InfilVolTotalOcc"=>infil_vol_total_occ,"InfilVolMin"=>infil_vol_min,"InfilVolTotalOccStdDen"=>infil_vol_total_occ_std_den,"SimpVentVolTotalOcc"=>simp_vent_vol_total_occ,"SimpVentVolMin"=>simp_vent_vol_min,"SimpVentVolTotalOccStdDen"=>simp_vent_vol_total_occ_std_den,"VozTargetTotalOcc"=>voz_target_total_occ,"VozTargetTimeBelowOcc"=>voz_target_time_below_occ,"VozTargetTimeAtOcc"=>voz_target_time_at_occ,"VozTargetTimeAboveOcc"=>voz_target_time_above_occ,"TotVentTimeNonZeroUnocc"=>tot_vent_time_non_zero_unocc,"MechVentVolTotalStdDen"=>mech_vent_vol_total_std_den,"InfilVolTotalStdDen"=>infil_vol_total_std_den,"SimpVentVolTotalStdDen"=>simp_vent_vol_total_std_den,"VozTargetTotal"=>voz_target_total,"VozTargetTimeBelow"=>voz_target_time_below,"VozTargetTimeAt"=>voz_target_time_at,"VozTargetTimeAbove"=>voz_target_time_above});bind(identities,"Predef",predef,std::ptr::from_ref(actual) as usize)?;
                let a=member(z,"air")?;let actual=air.zones.as_mut().and_then(|r|r.get_mut(i)).ok_or_else(||invalid("same AirHB"))?;
                actual.air.mat=real(member(a,"MAT")?)?;actual.air.air_hum_rat_avg=real(member(a,"airHumRatAvg")?)?;bind(identities,"AirHB",a,std::ptr::from_ref(actual) as usize)?;
                let r=member(z,"ZnAirRpt")?;let actual=controller.reports.zones.as_mut().and_then(|r|r.get_mut(i)).ok_or_else(||invalid("same ZnAirRpt"))?;
                seed_reals!(actual,r,{"InfilVolumeCurDensity"=>infil_volume_cur_density,"InfilVolumeStdDensity"=>infil_volume_std_density,"VentilVolumeCurDensity"=>ventil_volume_cur_density,"VentilVolumeStdDensity"=>ventil_volume_std_density});bind(identities,"ZnAirRpt",r,std::ptr::from_ref(actual) as usize)?;
                seed_units(&mut equipment,&fixture,z,identities)?;
            }
        }
        if is_vent {reports.report_source_ventilation_loads(&globals,&hvac,&equipment,&mut zones,&controller.demand,&controller.reports,&air,&environment,&sizing,&mut summary)?;vent+=1;}
        else {reports.report_source_system_energy_use(&globals,&hvac,&equipment,&controller.demand)?;energy+=1;}
        let out=member(expected,"state")?;let out_flags=member(out,"flags")?;let out_clock=member(expected,"clock")?;
        assert_eq!(globals.num_of_zones,integer(member(out_flags,"NumOfZones")?)?);
        assert_eq!(hvac.number_of_primary_air_systems,integer(member(out_flags,"NumPrimaryAirSys")?)?);
        assert_real(hvac.time_step_sys,member(out_clock,"TimeStepSys")?,&context,"system step unchanged")?;
        assert_real(hvac.time_step_sys_seconds,member(out_clock,"TimeStepSysSec")?,&context,"system seconds unchanged")?;
        assert_eq!(reports.air_loop_loads_report_enabled,boolean(member(out_flags,"AirLoopLoadsReportEnabled")?)?);
        assert_eq!(reports.vent_report_structure_created,boolean(member(out_flags,"VentReportStructureCreated")?)?);
        assert_eq!(reports.vent_loads_report_enabled,boolean(member(out_flags,"VentLoadsReportEnabled")?)?);
        assert_eq!(reports.vent_energy_report_enabled,boolean(member(out_flags,"VentEnergyReportEnabled")?)?);
        if enabled && is_vent {
            assert_real(environment.std_rho_air,member(out,"StdRhoAir")?,&context,"standard rho unchanged")?;
            assert_real(environment.current_weather.out_baro_press,member(out_clock,"OutBaroPress")?,&context,"actual barometric pressure unchanged")?;
        }
        if enabled {
            for z in array(member(out,"zones")?)? {
                let number=integer(member(z,"index")?)?;let i=index(number)?;let config=row_value(member(z,"config")?)?;
                let source=equipment.source_zone_equipment_config(number)?;assert_eq!(*source.is_controlled(),boolean(member(config,"IsControlled")?)?);
                if *source.is_controlled() {
                    assert_eq!(*source.number_inlet_nodes(),count(member(config,"NumInletNodes")?)?);
                    if is_vent {
                        assert_eq!(*source.equipment_list_index(),usize::try_from(integer(member(config,"EquipListIndex")?)?)?);
                        assert_eq!(*source.zone_design_specification_outdoor_air_index(),integer(member(config,"ZoneDesignSpecOAIndex")?)?);
                        assert_eq!(*source.zone_air_distribution_index(),integer(member(config,"ZoneAirDistributionIndex")?)?);
                    }
                }
                if is_vent {
                    let values=reports.native_report_cohort_row(number)?;let expected=row_value(member(z,"report")?)?;
                    bind(identities,"ZoneVent",expected,reports.native_report_cohort_row_address(number)?)?;
                    for (i,(v,name)) in values.into_iter().zip(ZONE_FIELDS).enumerate(){
                        assert_real(v,member(expected,name)?,&context,name)?;
                        if kind=="native-unit-variant" && i<8 && v!=0.0 {thermal_nonzero[i]=true;}
                    }
                }
                if !*source.is_controlled(){continue;}
                let d=controller.demand.zones.as_ref().and_then(|r|r.get(i)).ok_or_else(||invalid("actual same demand after"))?;
                bind(identities,"Demand",member(z,"demand")?,std::ptr::from_ref(d) as usize)?;
                assert_real(d.base.total_output_required,member(member(z,"demand")?,"TotalOutputRequired")?,&context,"demandTotal unchanged")?;
                assert_eq!(*controller.demand.deadband_or_setback.as_ref().and_then(|r|r.get(i)).ok_or_else(||invalid("actual same deadband after"))?,bool_integer(member(member(z,"DeadBandOrSetback")?,"value")?)?);
                for (j,raw) in array(member(member(config,"InletNodeAirLoopNum")?,"values")?)?.iter().enumerate(){assert_eq!(*source.inlet_node_air_loop_number(j+1).ok_or_else(||invalid("same inlet association"))?,integer(raw)?);}
                if !is_vent{continue;}
                let actual_zone=zones.zones.as_ref().and_then(|r|r.get(i)).ok_or_else(||invalid("same Zone after"))?;let raw_zone=member(z,"zone")?;
                bind(identities,"Zone",raw_zone,std::ptr::from_ref(actual_zone) as usize)?;
                assert_eq!(actual_zone.name,text(member(raw_zone,"Name")?)?);
                assert_eq!(actual_zone.multiplier,integer(member(raw_zone,"Multiplier")?)?);assert_eq!(actual_zone.list_multiplier,integer(member(raw_zone,"ListMultiplier")?)?);
                assert_real(actual_zone.volume,member(raw_zone,"Volume")?,&context,"Zone volume unchanged")?;
                let actual_air_report=controller.reports.zones.as_ref().and_then(|r|r.get(i)).ok_or_else(||invalid("same ZnAirRpt after"))?;
                bind(identities,"ZnAirRpt",member(z,"ZnAirRpt")?,std::ptr::from_ref(actual_air_report) as usize)?;
                check_reals!(actual_air_report,member(z,"ZnAirRpt")?,&context,{"InfilVolumeCurDensity"=>infil_volume_cur_density,"InfilVolumeStdDensity"=>infil_volume_std_density,"VentilVolumeCurDensity"=>ventil_volume_cur_density,"VentilVolumeStdDensity"=>ventil_volume_std_density});
                let predef=zones.zone_predefined.as_ref().and_then(|r|r.get(i)).ok_or_else(||invalid("same predef after"))?;let raw=row_value(member(z,"predefined")?)?;
                bind(identities,"Predef",raw,std::ptr::from_ref(predef) as usize)?;
                assert_eq!(predef.is_occupied,boolean(member(raw,"isOccupied")?)?);check_reals!(predef,raw,&context,{"AFNVentVolStdDen"=>afn_vent_vol_std_den,"TotTimeOcc"=>tot_time_occ,"MechVentVolTotalOcc"=>mech_vent_vol_total_occ,"MechVentVolMin"=>mech_vent_vol_min,"MechVentVolTotalOccStdDen"=>mech_vent_vol_total_occ_std_den,"InfilVolTotalOcc"=>infil_vol_total_occ,"InfilVolMin"=>infil_vol_min,"InfilVolTotalOccStdDen"=>infil_vol_total_occ_std_den,"SimpVentVolTotalOcc"=>simp_vent_vol_total_occ,"SimpVentVolMin"=>simp_vent_vol_min,"SimpVentVolTotalOccStdDen"=>simp_vent_vol_total_occ_std_den,"VozTargetTotalOcc"=>voz_target_total_occ,"VozTargetTimeBelowOcc"=>voz_target_time_below_occ,"VozTargetTimeAtOcc"=>voz_target_time_at_occ,"VozTargetTimeAboveOcc"=>voz_target_time_above_occ,"TotVentTimeNonZeroUnocc"=>tot_vent_time_non_zero_unocc,"MechVentVolTotalStdDen"=>mech_vent_vol_total_std_den,"InfilVolTotalStdDen"=>infil_vol_total_std_den,"SimpVentVolTotalStdDen"=>simp_vent_vol_total_std_den,"VozTargetTotal"=>voz_target_total,"VozTargetTimeBelow"=>voz_target_time_below,"VozTargetTimeAt"=>voz_target_time_at,"VozTargetTimeAbove"=>voz_target_time_above});
                let a=air.zones.as_ref().and_then(|r|r.get(i)).ok_or_else(||invalid("same air after"))?;let raw=member(z,"air")?;
                bind(identities,"AirHB",raw,std::ptr::from_ref(a) as usize)?;
                assert_real(a.air.mat,member(raw,"MAT")?,&context,"MAT unchanged")?;assert_real(a.air.air_hum_rat_avg,member(raw,"airHumRatAvg")?,&context,"Wavg unchanged")?;
                check_units(&equipment,z,&context,identities)?;
            }
            if is_vent{check_reals!(reports,member(out,"facility_report")?,&context,{"AnyZoneTimeBelowVozDyn"=>any_zone_time_below_voz_dyn,"AllZonesTimeAtVozDyn"=>all_zones_time_at_voz_dyn,"AnyZoneTimeAboveVozDyn"=>any_zone_time_above_voz_dyn,"AnyZoneTimeVentUnocc"=>any_zone_time_vent_unocc,"AnyZoneTimeBelowVozDynOcc"=>any_zone_time_below_voz_dyn_occ,"AllZonesTimeAtVozDynOcc"=>all_zones_time_at_voz_dyn_occ,"AnyZoneTimeAboveVozDynOcc"=>any_zone_time_above_voz_dyn_occ});check_reals!(summary,member(out,"facility_predefined_totals")?,&context,{"TotalAnyZoneBelowVozDynForOA"=>total_any_zone_below_voz_dyn_for_oa,"TotalAllZonesAtVozDynForOA"=>total_all_zones_at_voz_dyn_for_oa,"TotalAnyZoneAboveVozDynForOA"=>total_any_zone_above_voz_dyn_for_oa,"TotalAnyZoneVentUnoccForOA"=>total_any_zone_vent_unocc_for_oa,"TotalAnyZoneBelowVozDynOccForOA"=>total_any_zone_below_voz_dyn_occ_for_oa,"TotalAllZonesAtVozDynOccForOA"=>total_all_zones_at_voz_dyn_occ_for_oa,"TotalAnyZoneAboveVozDynOccForOA"=>total_any_zone_above_voz_dyn_occ_for_oa});}
        }
    }
    let expected=member(&fixture,"counts")?;assert_eq!(energy,count(member(expected,"ReportSystemEnergyUse")?)?);assert_eq!(vent,count(member(expected,"ReportVentilationLoads")?)?);
    assert_eq!(variants,count(member(&fixture,"native_unit_variants")?)?);assert!(energy>0 && vent>0 && variants>0 && identity_groups.values().all(|ids|ids.len()>1),"genuine nonzero/boundary Native unit variants are required before report-cohort PASS");
    assert!(thermal_nonzero.into_iter().all(|v|v),"all eight actual thermal classifications need nonzero Native-paired Source output");
    Ok((energy,vent,variants))
}
