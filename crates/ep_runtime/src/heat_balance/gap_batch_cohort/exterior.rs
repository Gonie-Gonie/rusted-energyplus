//! Selected finite Exterior units on genuine persistent Source owners.
//! All original Native output remains in the external authenticated fixture.
mod ingress;

use crate::heat_balance::inside_convection_dispatcher::source::SourceInsideConvection;
use crate::heat_balance::radiation::ordinary::OrdinarySolarRadiation;
use crate::weather::day::WeatherDayState;
use ingress::GainFactory;
use serde_json::Value;
use std::{io, path::Path};

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

fn invalid(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}
fn array(value: &Value) -> Result<&Vec<Value>> {
    Ok(value
        .as_array()
        .ok_or_else(|| invalid("actual Exterior array"))?)
}
fn object(value: &Value) -> Result<&serde_json::Map<String, Value>> {
    Ok(value
        .as_object()
        .ok_or_else(|| invalid("actual Exterior object"))?)
}
fn member<'a>(value: &'a Value, key: &str) -> Result<&'a Value> {
    Ok(object(value)?.get(key).ok_or_else(|| invalid(key))?)
}
fn text(value: &Value) -> Result<&str> {
    Ok(value
        .as_str()
        .ok_or_else(|| invalid("actual Exterior string"))?)
}
fn same<T: std::fmt::Debug + PartialEq>(source: T, native: T, label: &str) -> Result {
    if source != native {
        return Err(
            format!("{label}: actual Source {source:?} differs from Native {native:?}").into(),
        );
    }
    Ok(())
}
fn bits(source: f64, native: &Value, label: &str) -> Result {
    if !source.is_finite() {
        return Err(invalid("actual Source nonfinite solar return").into());
    }
    same(
        format!("{:016x}", source.to_bits()),
        text(member(native, "value_bits")?)?.to_owned(),
        label,
    )
}
fn pair<'a>(named: &'a Value, observed: &'a Value) -> Result<&'a str> {
    same(
        named,
        member(observed, "input")?,
        "whole named primary lifetime",
    )?;
    same(
        member(named, "name")?,
        member(observed, "name")?,
        "actual lifetime name",
    )?;
    text(member(named, "name")?)
}
fn steps<'a>(named: &'a Value, observed: &'a Value) -> Result<(&'a Vec<Value>, &'a Vec<Value>)> {
    let input = array(member(named, "steps")?)?;
    let actual = array(member(observed, "calls")?)?;
    same(
        input.len(),
        actual.len(),
        "all actual repeated call records",
    )?;
    if input.is_empty() {
        return Err(invalid("positive named persistent call lifetime").into());
    }
    Ok((input, actual))
}
fn primary<'a>(named: &'a Value, observed: &Value) -> Result<&'a Value> {
    same(
        named,
        member(observed, "primary_clock")?,
        "whole actual caller clock primary",
    )?;
    Ok(named)
}
fn selected_ingress<'a>(ingress: &'a Value, named: &Value) -> Result<&'a Value> {
    let row = member(member(ingress, "factories")?, text(member(named, "name")?)?)?;
    same(
        member(row, "actual_primary_IDF")?,
        member(named, "input_authority")?,
        "actual input authority matches Source ingress",
    )?;
    Ok(row)
}
fn record_count(named: &Value, observed: &Value, group: &str) -> Result<usize> {
    let inputs = array(member(named, group)?)?;
    let outputs = array(member(observed, group)?)?;
    same(
        inputs.len(),
        outputs.len(),
        "complete named group lifetimes",
    )?;
    outputs.iter().try_fold(0usize, |sum, row| {
        sum.checked_add(array(member(row, "calls")?)?.len())
            .ok_or_else(|| invalid("actual Exterior call record count overflow").into())
    })
}
fn excluded_geometry_families(row: &Value) -> Result<Vec<(String, usize)>> {
    let mut outside = Vec::new();
    for (family, objects) in object(member(row, "raw")?)? {
        let count = object(objects)?.len();
        if count > 0
            && (family == "FenestrationSurface:Detailed"
                || family.starts_with("Shading:")
                || matches!(
                    family.as_str(),
                    "Window" | "WindowProperty:ShadingControl" | "WindowShadingControl"
                ))
        {
            outside.push((family.clone(), count));
        }
    }
    Ok(outside)
}

fn no_surface_gains(ingress: &Value, named: &Value, observed: &Value) -> Result<usize> {
    let inputs = array(member(named, "internal_gains")?)?;
    let outputs = array(member(observed, "internal_gains")?)?;
    same(
        inputs.len(),
        outputs.len(),
        "full no-surface gain lifetimes",
    )?;
    let mut records = 0usize;
    for (input, expected) in inputs.iter().zip(outputs) {
        let name = pair(input, expected)?;
        selected_ingress(ingress, input)?;
        let mut owner = GainFactory::new(ingress, name)?;
        owner.assert_gains(member(expected, "before_getter")?)?;
        owner.get_input()?;
        owner.assert_gains(member(expected, "after_getter")?)?;
        // Actual getter allocates device/report rows. Establish retention only
        // after that genuine allocation, then keep every later call on them.
        let mut identity = None;
        owner.retain_gain_identity(member(expected, "after_getter")?, &mut identity)?;
        let (inputs, calls) = steps(input, expected)?;
        for (input, call) in inputs.iter().zip(calls) {
            let input = primary(input, call)?;
            owner.prepare_gain_clock(input)?;
            owner.assert_gains(member(call, "before")?)?;
            owner.retain_gain_identity(member(call, "before")?, &mut identity)?;
            owner.invoke_actual_gains()?;
            owner.assert_gains(member(call, "after")?)?;
            owner.retain_gain_identity(member(call, "after")?, &mut identity)?;
            records += 1;
        }
    }
    Ok(records)
}

fn beam(
    ingress: &Value,
    scope: &Value,
    named: &Value,
    observed: &Value,
    files: &Path,
) -> Result<(usize, usize)> {
    let inputs = array(member(named, "beam_producer")?)?;
    let outputs = array(member(observed, "beam_producer")?)?;
    let classifications = array(member(scope, "beam_classification")?)?;
    same(
        inputs.len(),
        outputs.len(),
        "all original Native beam lifetimes retained",
    )?;
    same(
        inputs.len(),
        classifications.len(),
        "full primary scope classifications",
    )?;
    let mut records = 0usize;
    let mut excluded = 0usize;
    let mut admitted_indices = Vec::new();
    let mut excluded_indices = Vec::new();
    for (at, ((input, expected), classification)) in
        inputs.iter().zip(outputs).zip(classifications).enumerate()
    {
        let name = pair(input, expected)?;
        let row = selected_ingress(ingress, input)?;
        same(
            member(classification, "case_name")?,
            member(input, "name")?,
            "classified actual input name",
        )?;
        same(
            member(classification, "actual_primary_IDF")?,
            member(input, "input_authority")?,
            "classified actual input authority",
        )?;
        same(
            member(classification, "native_index")?.as_u64(),
            Some(at as u64),
            "actual Native beam index",
        )?;
        let outside = excluded_geometry_families(row)?;
        let (inputs, calls) = steps(input, expected)?;
        same(
            member(classification, "actual_call_records")?.as_u64(),
            Some(calls.len() as u64),
            "whole classified actual call count",
        )?;
        let selected = outside.is_empty();
        same(
            member(classification, "Source_unit_selected")?.as_bool(),
            Some(selected),
            "primary-only supported geometry classification",
        )?;
        let actual_counts = Value::Object(
            outside
                .into_iter()
                .map(|(k, v)| (k, Value::from(v)))
                .collect(),
        );
        same(
            member(classification, "actual_window_shading_family_counts")?,
            &actual_counts,
            "whole primary geometry exclusion counts",
        )?;
        if !selected {
            excluded += calls.len();
            excluded_indices.push(at);
            // Keep every actual Native result in the fixture. This declared
            // CON-01 window/shading exclusion is decided only by primary IDF.
            continue;
        }
        admitted_indices.push(at);
        if member(input, "require_positive_casting")?.as_bool() != Some(false) {
            return Err(invalid("no attached shading finite beam unit domain").into());
        }
        let mut owner =
            GainFactory::new_geometry(ingress, name, &files.join(format!("beam-{at}.mtd")), true)?;
        owner.prepare_actual_beam(&inputs[0], &files.join(format!("beam-{at}.shd")))?;
        let mut identity = None;
        for (input, call) in inputs.iter().zip(calls) {
            let input = primary(input, call)?;
            let (daily, direction) = owner.beam_weather_handoff(input)?;
            let returns = member(call, "actual_daily_returns")?;
            for (value, key) in daily.into_iter().zip([
                "A",
                "B",
                "C",
                "AnnVarSolConstant",
                "EquationOfTime",
                "SinDecl",
                "CosDecl",
            ]) {
                bits(value, member(returns, key)?, key)?;
            }
            let expected_direction = array(member(call, "actual_weather_direction")?)?;
            same(
                expected_direction.len(),
                direction.len(),
                "real Weather direction extent",
            )?;
            for (value, expected) in direction.into_iter().zip(expected_direction) {
                bits(value, expected, "actual Weather direction return")?;
            }
            owner.assert_beam(member(call, "before")?, &mut identity)?;
            owner.actual_figure()?;
            owner.assert_beam(member(call, "after")?, &mut identity)?;
            records += 1;
        }
    }
    same(
        serde_json::to_value(&admitted_indices)?,
        member(scope, "source_unit_beam_indices")?.clone(),
        "all selected Source beam indices",
    )?;
    same(
        serde_json::to_value(&excluded_indices)?,
        member(scope, "outside_CON01_beam_indices")?.clone(),
        "all Native-only outside-scope beam indices",
    )?;
    Ok((records, excluded))
}

fn surface_gains(ingress: &Value, named: &Value, observed: &Value, files: &Path) -> Result<usize> {
    let inputs = array(member(named, "surface_internal_gains")?)?;
    let outputs = array(member(observed, "surface_internal_gains")?)?;
    same(
        inputs.len(),
        outputs.len(),
        "all full positive surface gain lifetimes",
    )?;
    let mut records = 0usize;
    for (at, (input, expected)) in inputs.iter().zip(outputs).enumerate() {
        let name = pair(input, expected)?;
        selected_ingress(ingress, input)?;
        let mut owner = GainFactory::new_geometry(
            ingress,
            name,
            &files.join(format!("surface-gain-{at}.mtd")),
            false,
        )?;
        owner.prepare_actual_surface_gains()?;
        owner.assert_surface_gains(member(expected, "setup")?)?;
        let mut identity = None;
        owner.retain_surface_gain_identity(member(expected, "setup")?, &mut identity)?;
        let (inputs, calls) = steps(input, expected)?;
        for (input, call) in inputs.iter().zip(calls) {
            let input = primary(input, call)?;
            owner.prepare_gain_clock(input)?;
            owner.assert_surface_gains(member(call, "before")?)?;
            owner.retain_surface_gain_identity(member(call, "before")?, &mut identity)?;
            owner.invoke_actual_gains()?;
            owner.assert_surface_gains(member(call, "after")?)?;
            owner.retain_surface_gain_identity(member(call, "after")?, &mut identity)?;
            records += 1;
        }
    }
    Ok(records)
}

pub(super) fn run(fixture: &Value) -> Result<usize> {
    run_category(fixture, None)
}
pub(super) fn run_category(fixture: &Value, category: Option<&str>) -> Result<usize> {
    let named = member(member(fixture, "named_inputs")?, "exterior")?;
    let observed = member(
        member(member(fixture, "native_output")?, "groups")?,
        "exterior",
    )?;
    let ingress = member(fixture, "source_ingress")?;
    let scope = member(fixture, "source_scope")?;
    same(
        member(named, "schema")?,
        &Value::from("src06-exterior-gap-input.v1"),
        "named schema",
    )?;
    same(
        member(observed, "schema")?,
        &Value::from("src06-original-exterior-gap-output.v1"),
        "actual output schema",
    )?;
    same(
        member(ingress, "schema")?,
        &Value::from("src06-Exterior-primary-Source-ingress.v1"),
        "primary Source ingress",
    )?;
    same(
        member(scope, "schema")?,
        &Value::from("src06-Exterior-Source-unit-scope.v1"),
        "explicit finite Source scope",
    )?;
    for key in [
        "Native_full_output_preserved",
        "does_not_expand_production_finite_15_input_admission",
        "no_whole_three_beam_Source_equivalence_claim",
    ] {
        same(
            member(scope, key)?.as_bool(),
            Some(true),
            "explicit scientific coverage limit",
        )?;
    }
    let categories = [
        "solar_clock",
        "exterior_coefficients",
        "absorption",
        "solar_surface",
        "internal_gains",
        "beam_producer",
        "surface_internal_gains",
    ];
    if category.is_some_and(|selected| !categories.contains(&selected)) {
        return Err(invalid("unknown actual Exterior category").into());
    }
    let counts = categories
        .iter()
        .map(|key| record_count(named, observed, key))
        .collect::<Result<Vec<_>>>()?;
    same(
        &counts,
        &vec![47, 57, 18, 15, 9, 27, 6],
        "all actual retained Native call records",
    )?;
    let full_records = counts.iter().sum::<usize>();
    same(full_records, 179, "whole Native retained record count")?;

    // Each selected whole group starts in its own process, matching Native.
    // These are real caller-selected Source MTD/SHD paths, distinct from every
    // retained Native output path. The directory is retained for Root custody.
    let files = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .ok_or("ep_runtime manifest has no workspace ancestor")?
        .join(".runtime/porting/SRC-06")
        .join(format!(
            "Source-Exterior-cohort-output-{}",
            std::process::id()
        ));
    std::fs::create_dir(&files)?;
    let mut admitted = 0usize;
    let mut excluded = 0usize;
    let mut weather_checks = 0usize;
    for (at, &key) in categories.iter().enumerate() {
        if category.is_some_and(|selected| selected != key) {
            continue;
        }
        let (selected_records, selected_excluded, selected_checks) = match key {
            "solar_clock" => {
                let checks = WeatherDayState::run_native_gap_solar_clock(named, observed)?;
                same(checks, array(member(named, "solar_clock")?)?.len() + counts[0] * 2,
                    "Weather phase checks distinguished from records")?;
                (counts[0], 0, checks)
            }
            "exterior_coefficients" => {
                let records = SourceInsideConvection::native_gap_external_convection_cases(named, observed)?;
                same(records, counts[1], "InitExt all repeated records")?;
                (records, 0, 0)
            }
            "absorption" => {
                let records = OrdinarySolarRadiation::native_gap_absorption_cases(named, observed)?;
                same(records, counts[2], "all paired thermal/shortwave records")?;
                (records, 0, 0)
            }
            "solar_surface" => {
                let records = OrdinarySolarRadiation::native_gap_sky_opaque_cases(named, observed)?;
                same(records, counts[3], "all paired aniso/opaque records")?;
                (records, 0, 0)
            }
            "internal_gains" => {
                let records = no_surface_gains(ingress, named, observed)?;
                same(records, counts[4], "full no-surface IHG records")?;
                (records, 0, 0)
            }
            "beam_producer" => {
                let (records, outside) = beam(ingress, scope, named, observed, &files)?;
                same(records + outside, counts[5], "all Native beam records retained with explicit classification")?;
                (records, outside, 0)
            }
            "surface_internal_gains" => {
                let records = surface_gains(ingress, named, observed, &files)?;
                same(records, counts[6], "full positive surface IHG records")?;
                (records, 0, 0)
            }
            _ => return Err(invalid("unreachable admitted Exterior category").into()),
        };
        same(selected_records + selected_excluded, counts[at], "complete selected category record accounting")?;
        admitted = admitted.checked_add(selected_records).ok_or("selected record overflow")?;
        excluded = excluded.checked_add(selected_excluded).ok_or("selected exclusion overflow")?;
        weather_checks = weather_checks.checked_add(selected_checks).ok_or("selected Weather check overflow")?;
        println!("Source Exterior category complete: category={key}; admitted_records={selected_records}; Native_retained_records={}; excluded_records={selected_excluded}; Weather_phase_checks={selected_checks}", counts[at]);
    }
    if let Some(category) = category {
        println!("Source Exterior selected category: {category}; admitted call records {admitted}; Native retained records {}; excluded exploratory records {excluded}; Weather phase checks {weather_checks}; Source output directory {}", admitted + excluded, files.display());
    } else {
        same((admitted, excluded), (170, 9), "declared finite admitted and outside-CON records")?;
        println!("Source Exterior cohort: {admitted} admitted call records; Native full {full_records} retained; {excluded} exploratory shading records outside CON-01; Weather phase checks {weather_checks}; same-process owner assertions only; Source output directory {}", files.display());
    }
    Ok(admitted)
}
