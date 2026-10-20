//! Actual selected HeatBalanceManager316 Solar call, before physical/cache binding.
use super::*;
use crate::heat_balance::ctf_initialization_context::CtfInitializationContext;
use crate::heat_balance::report_scan::{
    ReportScanDiagnostic, ReportScanFailure, SourceReportScanInput, try_scan_for_reports_with_input,
};
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError;
use crate::heat_balance::selected_heat_balance_input::surface::{
    SurfaceClass, SurfaceGeometryInputState,
};

fn unavailable(reason: OrdinaryRadiationUnavailable) -> RuntimeError {
    OrdinaryRadiationError::Unavailable(reason).into()
}
fn index(number: i32) -> Result<usize, RuntimeError> {
    usize::try_from(
        number
            .checked_sub(1)
            .ok_or_else(|| unavailable(OrdinaryRadiationUnavailable::EnclosureDomain))?,
    )
    .map_err(|_| unavailable(OrdinaryRadiationUnavailable::EnclosureDomain))
}
fn output_error(error: std::io::Error) -> RuntimeError {
    OrdinaryRadiationError::from(error).into()
}

impl CtfInitializationContext<'_> {
    pub(crate) fn initialize_selected_solar_view_factors(
        &mut self,
        model: &TypedModel,
    ) -> Result<(), RuntimeError> {
        self.initialize_selected_solar_view_factors_with_debug(model, None)
    }

    /// Actual Source caller supplies the same already-open debug stream for IDF.
    pub(crate) fn initialize_selected_solar_view_factors_with_debug(
        &mut self,
        model: &TypedModel,
        debug: Option<&mut dyn std::io::Write>,
    ) -> Result<(), RuntimeError> {
        let mut owner = self
            .reports
            .ordinary_solar_radiation
            .take()
            .ok_or_else(|| unavailable(OrdinaryRadiationUnavailable::EnclosureDomain))?;
        // Restore this same partially written owner before any error escapes.
        let result = owner.initialize_source_solar(model, self, debug);
        self.reports.ordinary_solar_radiation = Some(owner);
        result
    }
}

impl OrdinarySolarRadiation {
    fn initialize_source_solar(
        &mut self,
        model: &TypedModel,
        output: &mut CtfInitializationContext<'_>,
        mut debug: Option<&mut dyn std::io::Write>,
    ) -> Result<(), RuntimeError> {
        let mut errors_found = false; // Original820 local, distinct from parent HB ErrorsFound.
        let mut option = String::new();
        let report = {
            let reports = &mut *output.reports;
            let mut source = SourceReportScanInput::new(
                reports
                    .selected_input_processor
                    .as_mut()
                    .ok_or_else(|| unavailable(OrdinaryRadiationUnavailable::ReportScan))?,
                &mut reports.input_processor_shortcut,
                &model.report_inputs,
            )
            .map_err(|_| unavailable(OrdinaryRadiationUnavailable::ReportScan))?;
            try_scan_for_reports_with_input(
                &model.report_inputs,
                &mut reports.scan,
                "ViewFactorInfo",
                None,
                Some(&mut option),
                None,
                &mut source,
                &mut |diagnostic| match diagnostic {
                    ReportScanDiagnostic::Warning(text) => output.backend.show_warning_error(&text),
                    ReportScanDiagnostic::Continue(text) => {
                        output.backend.show_continue_error(&text)
                    }
                },
            )
            .map_err(|error| match error {
                ReportScanFailure::Unavailable(_) => {
                    unavailable(OrdinaryRadiationUnavailable::ReportScan)
                }
                ReportScanFailure::Diagnostic(error) => {
                    OrdinaryRadiationError::Output(WarmupOutputError::Backend(error)).into()
                }
            })?
        };
        // Original InitSolar821 report is a LOCAL, not the persistent radiant
        // ViewFactorReport member. The later radiant scan owns that write.
        if report {
            super::view_factor_report::header(true, output.eio)?;
        }
        // Original834 is a local const: do not rewrite shared CurrentModuleObject.
        let user_f_count = output
            .reports
            .selected_input_processor
            .as_ref()
            .ok_or_else(|| unavailable(OrdinaryRadiationUnavailable::EnclosureDomain))?
            .get_num_objects_found("ZoneProperty:UserViewFactors:BySurfaceName");
        if user_f_count > 0 {
            return Err(unavailable(OrdinaryRadiationUnavailable::InputBranch));
        } // Next genuine call would be AlignInputViewFactors836.
        let count = usize::try_from(self.num_solar_enclosures)
            .map_err(|_| unavailable(OrdinaryRadiationUnavailable::StorageDomain))?;
        // NumOfSolarEnclosures0 skips access to an unallocated EnclSolInfo.
        // This empty borrowed view creates no Solar row or numeric allocation.
        let rows = self.solar.as_deref_mut().unwrap_or_default();
        for number in 0..count {
            let enclosure = rows
                .get_mut(number)
                .ok_or_else(|| unavailable(OrdinaryRadiationUnavailable::EnclosureDomain))?;
            let advanced = output
                .reports
                .selected_heat_balance_globals
                .as_ref()
                .ok_or_else(|| unavailable(OrdinaryRadiationUnavailable::EnclosureDomain))?
                .display_advanced_report_variables;
            if number == 0 && advanced {
                writeln!(output.eio, "! <Solar View Factor Check Values>,Zone/Enclosure Name,Original Check Value,Calculated Fixed Check Value,Final Check Value,Number of Iterations,Fixed RowSum Convergence,Used RowSum Convergence")
                    .map_err(output_error)?;
            }
            let zones = output
                .reports
                .selected_zone_input
                .as_ref()
                .ok_or_else(|| unavailable(OrdinaryRadiationUnavailable::EnclosureDomain))?;
            let spaces = zones
                .spaces
                .as_ref()
                .ok_or_else(|| unavailable(OrdinaryRadiationUnavailable::EnclosureDomain))?;
            let surfaces = output
                .reports
                .surface_geometry_input
                .final_surfaces
                .as_mut()
                .ok_or_else(|| unavailable(OrdinaryRadiationUnavailable::EnclosureDomain))?;
            let mut n = 0_usize;
            for &space_number in &enclosure.space_nums {
                let space = spaces
                    .get(index(space_number)?)
                    .ok_or_else(|| unavailable(OrdinaryRadiationUnavailable::EnclosureDomain))?;
                for &surface_number in &space.surfaces {
                    let row = &surfaces
                        .get(index(surface_number)?)
                        .ok_or_else(|| unavailable(OrdinaryRadiationUnavailable::EnclosureDomain))?
                        .row;
                    if !row.is_air_boundary_surface {
                        n = n.checked_add(1).ok_or_else(|| {
                            unavailable(OrdinaryRadiationUnavailable::StorageDomain)
                        })?;
                    }
                }
            }
            enclosure.num_surfaces = i32::try_from(n)
                .map_err(|_| unavailable(OrdinaryRadiationUnavailable::StorageDomain))?;
            if n < 1 {
                output
                    .backend
                    .show_severe_error(&format!(
                        "InitSolarViewFactors: No surfaces in enclosure={}.",
                        enclosure.name,
                    ))
                    .map_err(|error| {
                        RuntimeError::from(OrdinaryRadiationError::Output(
                            WarmupOutputError::Backend(error),
                        ))
                    })?;
                errors_found = true; // AFTER successful severe delivery, Original865.
            }
            if errors_found {
                // Zero-dimensional arrays and subsequent empty Fix are unported.
                // Preserve count/severe/local-error prefix; do not invent a fatal.
                return Err(unavailable(OrdinaryRadiationUnavailable::StorageDomain));
            }
            let square = n
                .checked_mul(n)
                .ok_or_else(|| unavailable(OrdinaryRadiationUnavailable::StorageDomain))?;
            enclosure.f = vec![0.0; square];
            enclosure.fix = None; // Rust diagnostic metadata for this newly dimensioned call.
            enclosure.source_array_allocated[0] = true;
            enclosure.area = vec![0.0; n];
            enclosure.source_array_allocated[1] = true;
            enclosure.absorptance = vec![0.0; n];
            enclosure.source_array_allocated[2] = true;
            enclosure.azimuth = vec![0.0; n];
            enclosure.source_array_allocated[3] = true;
            enclosure.tilt = vec![0.0; n];
            enclosure.source_array_allocated[4] = true;
            enclosure.source_surface_ptr = Some(vec![0; n]);
            let pointer = enclosure
                .source_surface_ptr
                .as_mut()
                .ok_or_else(|| unavailable(OrdinaryRadiationUnavailable::StorageDomain))?;
            let constructions = &output.reports.heat_balance_input_state.construction_usage;
            let report_order = &output
                .reports
                .surface_geometry_input
                .all_surface_list_report_order;
            let enclosure_number = i32::try_from(number + 1)
                .map_err(|_| unavailable(OrdinaryRadiationUnavailable::StorageDomain))?;
            let mut used = 0_usize;
            for &space_number in &enclosure.space_nums {
                let space = spaces
                    .get(index(space_number)?)
                    .ok_or_else(|| unavailable(OrdinaryRadiationUnavailable::EnclosureDomain))?;
                let prior = used;
                for &surface_number in &space.surfaces {
                    let row = &mut surfaces
                        .get_mut(index(surface_number)?)
                        .ok_or_else(|| unavailable(OrdinaryRadiationUnavailable::EnclosureDomain))?
                        .row;
                    if row.is_air_boundary_surface {
                        continue;
                    }
                    pointer[used] = surface_number;
                    used += 1;
                    row.solar_encl_surface_index = i32::try_from(used)
                        .map_err(|_| unavailable(OrdinaryRadiationUnavailable::StorageDomain))?;
                    row.solar_encl_index = enclosure_number;
                    row.rad_encl_index = enclosure_number;
                    let construction =
                        constructions.get(index(row.construction)?).ok_or_else(|| {
                            unavailable(OrdinaryRadiationUnavailable::EnclosureDomain)
                        })?;
                    if construction.type_is_window
                        && row.exterior_boundary > 0
                        && row.base_surface != surface_number
                    {
                        enclosure.has_interzone_window = true;
                    }
                }
                for &surface_number in &space.surfaces {
                    let reported = *report_order.get(index(surface_number)?).ok_or_else(|| {
                        unavailable(OrdinaryRadiationUnavailable::EnclosureDomain)
                    })?;
                    for (position, &source_number) in
                        pointer.iter().enumerate().take(used).skip(prior)
                    {
                        if source_number == reported {
                            enclosure.source_surface_report_nums.push(
                                i32::try_from(position + 1).map_err(|_| {
                                    unavailable(OrdinaryRadiationUnavailable::StorageDomain)
                                })?,
                            );
                            break;
                        }
                    }
                }
            }
            for (position, &surface_number) in pointer.iter().enumerate() {
                let row = &surfaces
                    .get(index(surface_number)?)
                    .ok_or_else(|| unavailable(OrdinaryRadiationUnavailable::EnclosureDomain))?
                    .row;
                enclosure.area[position] = row.area;
                enclosure.absorptance[position] = constructions
                    .get(index(row.construction)?)
                    .ok_or_else(|| unavailable(OrdinaryRadiationUnavailable::EnclosureDomain))?
                    .inside_absorp_solar;
                let properties = row
                    .vertex_progress
                    .properties
                    .ok_or_else(|| unavailable(OrdinaryRadiationUnavailable::EnclosureDomain))?;
                enclosure.azimuth[position] = properties.azimuth_deg;
                enclosure.tilt[position] = properties.tilt_deg;
            }
            if n == 1 {
                if advanced {
                    writeln!(
                        output.eio,
                        "Solar View Factor Check Values,{},0,0,0,-1,0,0",
                        enclosure.name
                    )
                    .map_err(output_error)?;
                }
                continue;
            }
            // Pure helpers borrow the genuine Source-produced ordered operands.
            let actual_zones = zones
                .zones
                .as_ref()
                .ok_or_else(|| unavailable(OrdinaryRadiationUnavailable::EnclosureDomain))?;
            let mut geometry = Vec::with_capacity(n);
            for (position, &surface_number) in pointer.iter().enumerate() {
                let surface = &surfaces[index(surface_number)?];
                let zone = actual_zones
                    .get(index(surface.row.zone)?)
                    .ok_or_else(|| unavailable(OrdinaryRadiationUnavailable::EnclosureDomain))?;
                geometry.push(super::super::ApproximateViewFactorSurface {
                    surface_id: surface.typed_id.ok_or_else(|| {
                        unavailable(OrdinaryRadiationUnavailable::EnclosureDomain)
                    })?,
                    surface_name: surface.row.name.clone(),
                    zone_id: zone.typed_zone_id.ok_or_else(|| {
                        unavailable(OrdinaryRadiationUnavailable::EnclosureDomain)
                    })?,
                    zone_name: zone.name.clone(),
                    surface_type: match surface.row.class {
                        SurfaceClass::Wall => ep_model::SurfaceType::Wall,
                        SurfaceClass::Floor => ep_model::SurfaceType::Floor,
                        SurfaceClass::Roof => ep_model::SurfaceType::Roof,
                        _ => return Err(unavailable(OrdinaryRadiationUnavailable::InputBranch)),
                    },
                    area_m2: enclosure.area[position],
                    azimuth_deg: enclosure.azimuth[position],
                    tilt_deg: enclosure.tilt[position],
                });
            }
            let approximate = super::super::calculate_selected_approximate_view_factors(
                super::super::ApproximateViewFactorContext {
                    enclosure_index: number + 1,
                    zone_id: geometry[0].zone_id,
                },
                &geometry,
            )
            .map_err(|_| unavailable(OrdinaryRadiationUnavailable::EnclosureDomain))?;
            // Buffered diagnostics cannot deliver Native mid-loop failure order.
            if !approximate.warnings.is_empty() {
                return Err(unavailable(OrdinaryRadiationUnavailable::NumericDomain));
            }
            enclosure.f.clone_from(&approximate.view_factors);
            // Native951-953 saves this genuine pre-Fix matrix only for reporting.
            let saved_approximate = report.then(|| enclosure.f.clone());
            let fixed = super::super::fix_energyplus_approximate_view_factors_with_outcome(
                &enclosure.area,
                &enclosure.f,
            )
            .ok_or_else(|| unavailable(OrdinaryRadiationUnavailable::NumericDomain))?;
            enclosure.f.clone_from(&fixed.view_factors);
            if let Some(saved) = saved_approximate.as_deref() {
                super::view_factor_report::solar(
                    enclosure,
                    &output.reports.surface_geometry_input,
                    saved,
                    &option,
                    output.eio,
                    &mut debug,
                )?;
            }
            drop(saved_approximate); // Native1077 before check-value summation.
            let mut sum = 0.0;
            for &source_position in &enclosure.source_surface_report_nums {
                let position = index(source_position)?;
                if position >= n {
                    return Err(unavailable(OrdinaryRadiationUnavailable::EnclosureDomain));
                }
                sum += (0..n).map(|j| enclosure.f[j * n + position]).sum::<f64>();
            }
            if advanced {
                let values = [
                    fixed.original_check,
                    fixed.fixed_check,
                    fixed.final_check,
                    (fixed.row_sum - n as f64).abs(),
                    (sum - n as f64).abs(),
                ];
                let text = values
                    .map(|value| crate::heat_balance::source_round_format::round(value, 6))
                    .into_iter()
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|error| {
                        RuntimeError::from(OrdinaryRadiationError::Output(
                            WarmupOutputError::Format(format!(
                                "radiation .6R unavailable: {error:?}"
                            )),
                        ))
                    })?;
                writeln!(
                    output.eio,
                    "Solar View Factor Check Values,{},{},{},{},{},{},{}",
                    enclosure.name, text[0], text[1], text[2], fixed.iterations, text[3], text[4]
                )
                .map_err(output_error)?;
            }
            enclosure.fix = Some(fixed);
        }
        if errors_found {
            return match output.backend.show_fatal_error("InitSolarViewFactors: Errors found during initialization of diffuse solar distribution.  Program terminated.") {
                Err(error) => Err(RuntimeError::HeatBalanceInput(Box::new(HeatBalanceInputError::Fatal(error)))),
                Ok(never) => match never {},
            };
        }
        self.solar_initialized = true;
        Ok(())
    }

    pub(crate) fn bind_source_initialized_solar_geometry(
        &mut self,
        model: &TypedModel,
        physical: &[SurfaceHeatBalanceState],
        input: OrdinaryRadiationInput,
        source: &SurfaceGeometryInputState,
    ) -> Result<(), OrdinaryRadiationError> {
        if !self.solar_initialized {
            return Err(OrdinaryRadiationUnavailable::EnclosureDomain.into());
        }
        // This existing binder enriches only physical associations; numeric and
        // Source pointer/report/backpointer owners above must not be filled again.
        self.bind_initialized_solar_geometry(model, physical, input)?;
        let surfaces = source
            .final_surfaces
            .as_ref()
            .ok_or(OrdinaryRadiationUnavailable::EnclosureDomain)?;
        for enclosure in self
            .solar
            .as_mut()
            .ok_or(OrdinaryRadiationUnavailable::EnclosureDomain)?
        {
            let pointer = enclosure
                .source_surface_ptr
                .as_ref()
                .ok_or(OrdinaryRadiationUnavailable::EnclosureDomain)?;
            if usize::try_from(enclosure.num_surfaces).ok() != Some(pointer.len()) {
                return Err(OrdinaryRadiationUnavailable::EnclosureDomain.into());
            }
            let mut indexes = Vec::with_capacity(pointer.len());
            let mut ids = Vec::with_capacity(pointer.len());
            for &source_number in pointer {
                let position = usize::try_from(
                    source_number
                        .checked_sub(1)
                        .ok_or(OrdinaryRadiationUnavailable::EnclosureDomain)?,
                )
                .map_err(|_| OrdinaryRadiationUnavailable::EnclosureDomain)?;
                let row = surfaces
                    .get(position)
                    .ok_or(OrdinaryRadiationUnavailable::EnclosureDomain)?;
                let id = row
                    .typed_id
                    .ok_or(OrdinaryRadiationUnavailable::EnclosureDomain)?;
                let mut matching = physical
                    .iter()
                    .enumerate()
                    .filter(|(_, surface)| surface.surface_id == id);
                let (physical_index, _) = matching
                    .next()
                    .ok_or(OrdinaryRadiationUnavailable::EnclosureDomain)?;
                if matching.next().is_some() || indexes.contains(&physical_index) {
                    return Err(OrdinaryRadiationUnavailable::EnclosureDomain.into());
                }
                self.back_pointers[physical_index] = (
                    usize::try_from(row.row.solar_encl_index)
                        .map_err(|_| OrdinaryRadiationUnavailable::EnclosureDomain)?,
                    usize::try_from(row.row.rad_encl_index)
                        .map_err(|_| OrdinaryRadiationUnavailable::EnclosureDomain)?,
                    usize::try_from(row.row.solar_encl_surface_index)
                        .map_err(|_| OrdinaryRadiationUnavailable::EnclosureDomain)?,
                );
                indexes.push(physical_index);
                ids.push(id);
            }
            enclosure.surface_indexes = indexes;
            enclosure.surface_ptr = ids;
            enclosure.surface_report_nums = enclosure
                .source_surface_report_nums
                .iter()
                .map(|&value| {
                    usize::try_from(
                        value
                            .checked_sub(1)
                            .ok_or(OrdinaryRadiationUnavailable::EnclosureDomain)?,
                    )
                    .map_err(|_| OrdinaryRadiationUnavailable::EnclosureDomain)
                })
                .collect::<Result<Vec<_>, _>>()?;
        }
        Ok(())
    }
}
