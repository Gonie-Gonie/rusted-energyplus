//! Consumed selected InitSolar prefix, with real borrowed output.
use super::*;
use crate::heat_balance::ctf_initialization_context::CtfInitializationContext;
use crate::heat_balance::report_scan::{
    ReportScanDiagnostic, ReportScanFailure, try_scan_for_reports,
};
use crate::heat_balance::source_round_format;

pub(super) fn allocate_enclosure(
    model: &TypedModel,
    surfaces: &[SurfaceHeatBalanceState],
    layout: &InsideConvectionLayout,
) -> Result<SolarRadiationEnclosure, OrdinaryRadiationError> {
    let mut enclosure = SolarRadiationEnclosure {
        name: model.spaces[0].name.0.clone(),
        ..Default::default()
    };
    bind_enclosure_geometry(&mut enclosure, model, surfaces, layout)?;
    Ok(enclosure)
}

pub(super) fn bind_enclosure_geometry(
    enclosure: &mut SolarRadiationEnclosure,
    _model: &TypedModel,
    surfaces: &[SurfaceHeatBalanceState],
    layout: &InsideConvectionLayout,
) -> Result<(), OrdinaryRadiationError> {
    let group = &layout.groups()[0];
    let n = group.surfaces.len();
    n.checked_mul(n)
        .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
    let mut indexes = Vec::with_capacity(n);
    for id in &group.surfaces {
        let mut matches = surfaces
            .iter()
            .enumerate()
            .filter(|(_, s)| s.surface_id == *id);
        let (index, surface) = matches
            .next()
            .ok_or(OrdinaryRadiationUnavailable::EnclosureDomain)?;
        if matches.next().is_some()
            || surface.zone_id != group.zone_id
            || !matches!(
                surface.surface_type,
                ep_model::SurfaceType::Wall
                    | ep_model::SurfaceType::Floor
                    | ep_model::SurfaceType::Roof
            )
            || !surface.area_m2.is_finite()
            || surface.area_m2 <= 0.0
            || !surface.azimuth_deg.is_finite()
            || !surface.tilt_deg.is_finite()
            || !surface.inside_solar_absorptance.is_finite()
            || !(0.0..=1.0).contains(&surface.inside_solar_absorptance)
        {
            return Err(OrdinaryRadiationUnavailable::EnclosureDomain.into());
        }
        indexes.push(index);
    }
    if indexes.len() != surfaces.len() {
        return Err(OrdinaryRadiationUnavailable::EnclosureDomain.into());
    }
    // Source SurfaceGeometry1506-1536/1727-1733: on this one-Space domain
    // without subsurfaces, both traversal/report order are Wall,Floor,Roof,
    // retaining actual within-class input order. No SurfaceId-as-index cast.
    enclosure.id = 1;
    enclosure.zone_id = Some(group.zone_id);
    enclosure.space_ids = vec![group.space_id];
    enclosure.surface_indexes = indexes;
    // Geometry setup/binding does not allocate InitSolar/InitInterior arrays.
    Ok(())
}

fn view_factor_scan(
    model: &TypedModel,
    output: &mut CtfInitializationContext<'_>,
) -> Result<(), OrdinaryRadiationError> {
    let mut option = String::new();
    let requested = try_scan_for_reports(
        &model.report_inputs,
        &mut output.reports.scan,
        "ViewFactorInfo",
        None,
        Some(&mut option),
        None,
        |diagnostic| match diagnostic {
            ReportScanDiagnostic::Warning(text) => output.backend.show_warning_error(&text),
            ReportScanDiagnostic::Continue(text) => output.backend.show_continue_error(&text),
        },
    )
    .map_err(|error| match error {
        ReportScanFailure::Unavailable(_) => OrdinaryRadiationUnavailable::ReportScan.into(),
        ReportScanFailure::Diagnostic(error) => {
            OrdinaryRadiationError::Output(WarmupOutputError::Backend(error))
        }
    })?;
    if requested {
        return Err(OrdinaryRadiationUnavailable::ViewFactorReport.into());
    }
    Ok(())
}

fn fill(enclosure: &mut SolarRadiationEnclosure, surfaces: &[SurfaceHeatBalanceState]) {
    let n = enclosure.surface_indexes.len();
    // Literal dimension(...,+0) on every reached initialization call, rather
    // than treating prior allocations/cache contents as initialized inputs.
    enclosure.area = vec![0.0; n];
    enclosure.absorptance = vec![0.0; n];
    enclosure.azimuth = vec![0.0; n];
    enclosure.tilt = vec![0.0; n];
    enclosure.f = vec![0.0; n * n];
    enclosure.surface_ptr = enclosure
        .surface_indexes
        .iter()
        .map(|&index| surfaces[index].surface_id)
        .collect();
    enclosure.surface_report_nums = (0..n).collect();
    enclosure.fix = None;
    for (i, &index) in enclosure.surface_indexes.iter().enumerate() {
        let surface = &surfaces[index];
        enclosure.area[i] = surface.area_m2;
        enclosure.absorptance[i] = surface.inside_solar_absorptance;
        enclosure.azimuth[i] = surface.azimuth_deg;
        enclosure.tilt[i] = surface.tilt_deg;
    }
}

fn approximate_and_fix(
    enclosure: &mut SolarRadiationEnclosure,
    surfaces: &[SurfaceHeatBalanceState],
    model: &TypedModel,
) -> Result<(), OrdinaryRadiationError> {
    let zone_name = model.zones[0].name.0.clone();
    let rows = enclosure
        .surface_indexes
        .iter()
        .enumerate()
        .map(|(i, &index)| {
            let s = &surfaces[index];
            super::super::ApproximateViewFactorSurface {
                surface_id: s.surface_id,
                surface_name: s.surface_name.clone(),
                zone_id: s.zone_id,
                zone_name: zone_name.clone(),
                surface_type: s.surface_type,
                area_m2: enclosure.area[i],
                azimuth_deg: enclosure.azimuth[i],
                tilt_deg: enclosure.tilt[i],
            }
        })
        .collect::<Vec<_>>();
    let approximate = super::super::calculate_selected_approximate_view_factors(
        super::super::ApproximateViewFactorContext {
            enclosure_index: enclosure.id,
            zone_id: enclosure
                .zone_id
                .ok_or(OrdinaryRadiationUnavailable::EnclosureDomain)?,
        },
        &rows,
    )
    .map_err(|_| OrdinaryRadiationUnavailable::EnclosureDomain)?;
    // The existing helper buffers warning records after its numeric loop.
    // That cannot reproduce Original's mid-loop backend-stop/F-prefix order.
    // Keep this consumed slice warning-free rather than replay diagnostics.
    if !approximate.warnings.is_empty() {
        return Err(OrdinaryRadiationUnavailable::NumericDomain.into());
    }
    enclosure.f = approximate.view_factors;
    let fixed = super::super::fix_energyplus_approximate_view_factors_with_outcome(
        &enclosure.area,
        &enclosure.f,
    )
    .ok_or(OrdinaryRadiationUnavailable::NumericDomain)?;
    enclosure.f.clone_from(&fixed.view_factors);
    enclosure.fix = Some(fixed);
    Ok(())
}

fn check_report(
    enclosure: &SolarRadiationEnclosure,
    output: &mut CtfInitializationContext<'_>,
) -> Result<(), OrdinaryRadiationError> {
    let label = "Solar";
    if enclosure.surface_ptr.len() == 1 {
        writeln!(
            output.eio,
            "{label} View Factor Check Values,{},0,0,0,-1,0,0",
            enclosure.name
        )?;
        return Ok(());
    }
    let check = enclosure
        .fix
        .as_ref()
        .ok_or(OrdinaryRadiationUnavailable::NumericDomain)?;
    let n = enclosure.surface_ptr.len();
    let mut row_sum = 0.0;
    for &i in &enclosure.surface_report_nums {
        row_sum += (0..n).map(|j| enclosure.f[j * n + i]).sum::<f64>();
    }
    let values = [
        check.original_check,
        check.fixed_check,
        check.final_check,
        (check.row_sum - n as f64).abs(),
        (row_sum - n as f64).abs(),
    ];
    let formatted = values.map(|value| source_round_format::round(value, 6));
    let mut text = Vec::with_capacity(5);
    for value in formatted {
        text.push(value.map_err(|e| {
            OrdinaryRadiationError::Output(WarmupOutputError::Format(format!(
                "radiation .6R unavailable: {e:?}"
            )))
        })?);
    }
    writeln!(
        output.eio,
        "{label} View Factor Check Values,{},{},{},{},{},{},{}",
        enclosure.name, text[0], text[1], text[2], check.iterations, text[3], text[4]
    )?;
    Ok(())
}

impl OrdinarySolarRadiation {
    /// HeatBalanceManager309-320 maps InitSolar before internal-gain input.
    /// Current Rust caller runs this before its present typed convective and
    /// radiant gain consumers; full GetInternalHeatGainsInput is still absent.
    pub(crate) fn initialize_solar_at_input_boundary(
        &mut self,
        model: &TypedModel,
        surfaces: &[SurfaceHeatBalanceState],
        output: &mut CtfInitializationContext<'_>,
    ) -> Result<(), OrdinaryRadiationError> {
        view_factor_scan(model, output)?;
        let solar = self
            .solar
            .as_mut()
            .ok_or(OrdinaryRadiationUnavailable::EnclosureDomain)?;
        for (number, enclosure) in solar.iter_mut().enumerate() {
            if number == 0 && self.input.advanced_report_variables {
                writeln!(
                    output.eio,
                    "! <Solar View Factor Check Values>,Zone/Enclosure Name,Original Check Value,Calculated Fixed Check Value,Final Check Value,Number of Iterations,Fixed RowSum Convergence,Used RowSum Convergence"
                )?;
            }
            fill(enclosure, surfaces);
            for (i, &index) in enclosure.surface_indexes.iter().enumerate() {
                self.back_pointers[index] = (enclosure.id, enclosure.id, i + 1);
            }
            if enclosure.surface_ptr.len() != 1 {
                approximate_and_fix(enclosure, surfaces, model)?;
            }
            if self.input.advanced_report_variables {
                check_report(enclosure, output)?;
            }
        }
        self.solar_initialized = true;
        Ok(())
    }
}
