//! Shared real-source surface group; no independent test target or fixture seed.
use crate::heat_balance::ctf_thermal_history::{
    CtfThermalHistoryAlgorithm, CtfThermalHistoryArrays, CtfThermalHistoryContext,
    CtfThermalHistoryFluxReports, CtfThermalHistoryMasters, CtfThermalHistoryState,
    CtfThermalHistorySurface, CtfThermalHistoryUpdateRoute, update_source_ctf_thermal_histories,
};
use crate::heat_balance::surface_manager::ctf_public_storage::CtfPublicCoefficientStorage;
use serde_json::Value;
type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;
fn array(v: &Value) -> Result<&Vec<Value>> {
    v.as_array()
        .ok_or_else(|| "Native surface array absent".into())
}
fn real(v: &Value) -> Result<f64> {
    let s = v["value_bits"].as_str().ok_or("raw input bits absent")?;
    if s.len() != 16
        || !s
            .bytes()
            .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
    {
        return Err("raw input encoding invalid".into());
    }
    let value = f64::from_bits(u64::from_str_radix(s, 16)?);
    if !value.is_finite() {
        return Err("nonfinite input outside cohort".into());
    }
    Ok(value)
}
fn bits(v: f64, e: &Value, name: &str) -> Result<()> {
    if Some(format!("{:016x}", v.to_bits()).as_str()) != e["value_bits"].as_str() {
        return Err(format!("{name}: actual Source/Native bits differ").into());
    }
    Ok(())
}
fn vector(v: &[f64], e: &Value, name: &str) -> Result<()> {
    let expected = array(&e["values"])?;
    if e["allocated"].as_bool() != Some(true)
        || expected.len() != v.len()
        || e["size"].as_u64() != Some(v.len() as u64)
        || e["lower"].as_i64() != Some(1)
        || e["upper"].as_u64() != Some(v.len() as u64)
    {
        return Err(format!("{name}: shape differs").into());
    }
    for (a, b) in v.iter().zip(expected) {
        bits(*a, b, name)?;
    }
    Ok(())
}
fn input_history(v: &Value) -> Result<[Vec<f64>; 19]> {
    let rows = array(v)?;
    if rows.len() != 19 {
        return Err("fixed19 primary history required".into());
    }
    let mut result: [Vec<f64>; 19] = std::array::from_fn(|_| Vec::new());
    for (slot, row) in result.iter_mut().zip(rows) {
        let values = array(row)?;
        if values.len() != 1 {
            return Err("one physical surface history required".into());
        }
        slot.push(real(&values[0])?);
    }
    Ok(result)
}
fn input_histories(v: &Value, prefix: &str) -> Result<CtfThermalHistoryArrays> {
    Ok(CtfThermalHistoryArrays {
        inside_temperature: input_history(&v[format!("{prefix}_inside_T")])?,
        outside_temperature: input_history(&v[format!("{prefix}_outside_T")])?,
        inside_flux: input_history(&v[format!("{prefix}_inside_q")])?,
        outside_flux: input_history(&v[format!("{prefix}_outside_q")])?,
    })
}
fn history(v: &[Vec<f64>; 19], e: &Value, name: &str) -> Result<()> {
    let terms = array(&e["terms"])?;
    if e["allocated"].as_bool() != Some(true)
        || terms.len() != 19
        || e["lower"].as_i64() != Some(1)
        || e["upper"].as_i64() != Some(19)
    {
        return Err(format!("{name}: history allocation differs").into());
    }
    for (row, e) in v.iter().zip(terms) {
        vector(row, e, name)?;
    }
    Ok(())
}
fn history_state(v: &CtfThermalHistoryState, e: &Value) -> Result<()> {
    if e["UpdateFirst"].as_bool() != Some(v.update_first_time) {
        return Err("Update first latch differs".into());
    }
    let master = v
        .masters
        .as_ref()
        .ok_or("Source master allocation absent")?;
    for (rows, key) in [
        (&v.normal.inside_temperature, "normal_inside_T"),
        (&v.normal.outside_temperature, "normal_outside_T"),
        (&v.normal.inside_flux, "normal_inside_q"),
        (&v.normal.outside_flux, "normal_outside_q"),
        (&master.arrays.inside_temperature, "master_inside_T"),
        (&master.arrays.outside_temperature, "master_outside_T"),
        (&master.arrays.inside_flux, "master_inside_q"),
        (&master.arrays.outside_flux, "master_outside_q"),
    ] {
        history(rows, &e[key], key)?;
    }
    let counters = array(&e["counter"]["values"])?;
    if e["counter"]["allocated"].as_bool() != Some(true)
        || counters.len() != master.current_num_histories.len()
        || master
            .current_num_histories
            .iter()
            .zip(counters)
            .any(|(a, b)| b.as_i64() != Some(i64::from(*a)))
    {
        return Err("actual master counters differ".into());
    }
    for (values, key) in [
        (&v.flux_reports.inside_conduction, "inside_conduction"),
        (&v.flux_reports.inside_flux, "inside_flux"),
        (&v.flux_reports.outside_conduction, "outside_conduction"),
        (&v.flux_reports.outside_flux, "outside_flux"),
    ] {
        vector(values, &e[key], key)?;
    }
    if let Some(s) = &v.scratch {
        for (values, key) in [
            (&s.outside_flux_first, "QExt1"),
            (&s.inside_flux_first, "QInt1"),
            (&s.outside_temperature_first, "TempExt1"),
            (&s.inside_temperature_first, "TempInt1"),
            (&s.sum_time, "SumTime"),
        ] {
            vector(values, &e[key], key)?;
        }
    } else {
        for key in ["QExt1", "QInt1", "TempExt1", "TempInt1", "SumTime"] {
            if e[key]["allocated"].as_bool() != Some(false)
                || e[key]["size"].as_u64() != Some(0)
                || !e[key]["values"].is_null()
            {
                return Err("cold scratch observation differs".into());
            }
        }
    }
    Ok(())
}
fn master_history(input: &Value, native: &Value) -> Result<usize> {
    let cases = array(&input["master_history"])?;
    let actual = array(&native["master_history"])?;
    if cases.len() != actual.len() {
        return Err("master case count differs".into());
    }
    let mut count = 0;
    for (case, oracle) in cases.iter().zip(actual) {
        if case != &oracle["input"] || case["name"] != oracle["name"] {
            return Err("master primary input differs".into());
        }
        let reports = array(&case["initial_flux_reports"])?;
        if reports.len() != 4 {
            return Err("four primary report values required".into());
        }
        let histories = &case["initial_histories"];
        let mut state = CtfThermalHistoryState {
            surface_ids: vec![ep_model::SurfaceId(0)],
            normal: input_histories(histories, "normal")?,
            masters: Some(CtfThermalHistoryMasters {
                arrays: input_histories(histories, "master")?,
                current_num_histories: vec![i32::try_from(
                    case["initial_counter"].as_i64().ok_or("counter absent")?,
                )?],
            }),
            scratch: None,
            update_first_time: true,
            flux_reports: CtfThermalHistoryFluxReports {
                inside_conduction: vec![real(&reports[0])?],
                inside_flux: vec![real(&reports[1])?],
                outside_conduction: vec![real(&reports[2])?],
                outside_flux: vec![real(&reports[3])?],
            },
        };
        // Only these primary construction members are consumed by Update. Other
        // coefficient tails are reset storage, not a full CalcCTF parent claim.
        let mut coefficients = CtfPublicCoefficientStorage::reset();
        coefficients.outside[0] = real(&case["outside0"])?;
        coefficients.cross[0] = real(&case["cross0"])?;
        coefficients.inside[0] = real(&case["inside0"])?;
        coefficients.ctf_time_step = real(&case["CTFTimeStep"])?;
        coefficients.num_histories =
            i32::try_from(case["NumHistories"].as_i64().ok_or("NumHistories absent")?)?;
        coefficients.num_ctf_terms =
            i32::try_from(case["NumCTFTerms"].as_i64().ok_or("NumCTFTerms absent")?)?;
        history_state(&state, &oracle["initial"])?;
        let steps = array(&case["steps"])?;
        let expected = array(&oracle["calls"])?;
        if steps.len() != expected.len() {
            return Err("master retained calls differ".into());
        }
        let groups = vec![vec![ep_model::SurfaceId(0)]];
        let ordered = [ep_model::SurfaceId(0)];
        for (step, e) in steps.iter().zip(expected) {
            if step != &e["primary_step"] {
                return Err("master step input differs".into());
            }
            let inside = real(&step["inside_T"])?;
            state.normal.inside_temperature[0][0] = inside;
            state.normal.outside_temperature[0][0] = real(&step["outside_T"])?;
            history_state(&state, &e["before"])?;
            let surfaces = [CtfThermalHistorySurface {
                surface_id: ordered[0],
                construction_id: ep_model::ConstructionId(0),
                public_owner_construction_id: ep_model::ConstructionId(0),
                coefficients: &coefficients,
                algorithm: CtfThermalHistoryAlgorithm::Ctf,
                ordinary_opaque: true,
                source_sink_present: false,
                // Unconsumed by actual Update, unlike initialization.
                outside_boundary: ep_model::OutsideBoundaryCondition::Adiabatic,
                outside_dry_bulb: 0.0,
                area: real(&case["area"])?,
                inside_surface_temperature: inside,
                inside_constant: real(&step["inside_constant"])?,
                outside_constant: real(&step["outside_constant"])?,
            }];
            let route = update_source_ctf_thermal_histories(
                &mut state,
                &surfaces,
                CtfThermalHistoryContext {
                    simple_ctf_only: case["simple_ctf_only"]
                        .as_bool()
                        .ok_or("SimpleCTFOnly absent")?,
                    any_ems_in_model: false,
                    any_construction_overrides: false,
                    any_internal_heat_source: false,
                    global_max_ctf_terms: coefficients.num_ctf_terms,
                    time_step_zone_hours: real(&case["TimeStepZone"])?,
                    heat_transfer_groups: &groups,
                    opaque_groups: &groups,
                    all_heat_transfer_surfaces: &ordered,
                },
            )?;
            if route != CtfThermalHistoryUpdateRoute::MasterHistories {
                return Err("actual master route differed".into());
            }
            history_state(&state, &e["after"])?;
            count += 1;
        }
    }
    Ok(count)
}
pub(super) fn run(fixture: &Value) -> Result<usize> {
    let input = &fixture["named_inputs"]["surface"];
    let native = &fixture["native_output"]["groups"]["surface"];
    // Preserve true original group order, without selectively skipping cases.
    let radiation_calls =
        crate::heat_balance::radiation::ordinary::OrdinarySolarRadiation::run_native_gap_radiation(
            input, native,
        )?;
    let inverse_calls =
        crate::heat_balance::radiation::ordinary::OrdinarySolarRadiation::run_native_gap_inverse(
            input, native,
        )?;
    let history_calls = master_history(input, native)?;
    let minute_calls=crate::heat_balance::selected_schedule_manager::SelectedScheduleManager::run_native_gap_schedule_minute(input,native)?;
    Ok(radiation_calls + inverse_calls + history_calls + minute_calls)
}
