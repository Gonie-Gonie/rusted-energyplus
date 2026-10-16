//! Actual InitInteriorRadExchange449-810 on the admitted copied-Solar route.
use super::{Error, OrdinarySolarRadiation, RadiantExchangeSource, RadiantInitializationOutput};
use super::{Unavailable, backend_error, index, script_f};
use crate::heat_balance::report_scan::{
    ReportScanDiagnostic, ReportScanFailure, SourceReportScanInput,
    try_scan_for_reports_with_input,
};

impl OrdinarySolarRadiation {
    pub(crate) fn initialize_source_radiant(
        &mut self,
        source: &RadiantExchangeSource<'_>,
        output: &mut RadiantInitializationOutput<'_, '_>,
    ) -> Result<(), Error> {
        let mut errors_found = false;
        let mut option = String::new();
        let mut input = SourceReportScanInput::new(
            output.input, output.shortcut, output.report_inputs,
        ).map_err(|_| Unavailable::ReportScan)?;
        self.radiant_exchange.view_factor_report = try_scan_for_reports_with_input(
            output.report_inputs, output.scan, "ViewFactorInfo", None,
            Some(&mut option), None, &mut input,
            &mut |diagnostic| match diagnostic {
                ReportScanDiagnostic::Warning(message) => output.backend.show_warning_error(&message),
                ReportScanDiagnostic::Continue(message) => output.backend.show_continue_error(&message),
            },
        ).map_err(|error| match error {
            ReportScanFailure::Unavailable(_) => Error::Unavailable(Unavailable::ReportScan),
            ReportScanFailure::Diagnostic(error) => backend_error(error),
        })?;
        if self.radiant_exchange.view_factor_report {
            // First unported operation is the actual full report header at475.
            return Err(Unavailable::ViewFactorReport.into());
        }
        self.radiant_exchange.max_num_enclosure_surfaces = 0;
        let count = usize::try_from(self.num_radiant_enclosures)
            .map_err(|_| Unavailable::StorageDomain)?;
        for number in 0..count {
            let enclosure = self.radiant.get_mut(number).ok_or(Unavailable::StorageDomain)?;
            if number == 0 && output.advanced_report_variables {
                writeln!(output.eio, "! <Surface View Factor Check Values>,Zone/Enclosure Name,Original Check Value,Calculated Fixed Check Value,Final Check Value,Number of Iterations,Fixed RowSum Convergence,Used RowSum Convergence")?;
            }
            let spaces = source.zones.spaces.as_ref().ok_or(Unavailable::StorageDomain)?;
            let surfaces = source.geometry.final_surfaces.as_ref().ok_or(Unavailable::StorageDomain)?;
            let mut n = 0_usize;
            for &space_number in &enclosure.space_nums {
                let space = spaces.get(index(space_number)?).ok_or(Unavailable::StorageDomain)?;
                for &surface_number in &space.surfaces {
                    let surface = surfaces.get(index(surface_number)?).ok_or(Unavailable::StorageDomain)?;
                    if surface.row.is_air_boundary_surface { continue; }
                    if surface_number == surface.representative_calc_surface_num {
                        n = n.checked_add(1).ok_or(Unavailable::StorageDomain)?;
                    }
                }
            }
            enclosure.num_surfaces = i32::try_from(n).map_err(|_| Unavailable::StorageDomain)?;
            self.radiant_exchange.max_num_enclosure_surfaces =
                self.radiant_exchange.max_num_enclosure_surfaces.max(enclosure.num_surfaces);
            if n < 1 {
                output.backend.show_severe_error(&format!(
                    "InitInteriorRadExchange: No surfaces in enclosure={}.", enclosure.name,
                )).map_err(backend_error)?;
                errors_found = true;
            }
            let square = n.checked_mul(n).ok_or(Unavailable::StorageDomain)?;
            enclosure.f = Some(vec![0.0; square]);
            enclosure.script_f = Some(vec![0.0; square]);
            enclosure.area = Some(vec![0.0; n]);
            enclosure.emissivity = Some(vec![0.0; n]);
            enclosure.azimuth = Some(vec![0.0; n]);
            enclosure.tilt = Some(vec![0.0; n]);
            if source.carroll_method { return Err(Unavailable::InputBranch.into()); }
            enclosure.source_surface_ptr = Some(vec![0; n]);
            let pointer = enclosure.source_surface_ptr.as_mut().ok_or(Unavailable::StorageDomain)?;
            let mut used = 0_usize;
            for &space_number in &enclosure.space_nums {
                let space = spaces.get(index(space_number)?).ok_or(Unavailable::StorageDomain)?;
                let prior = used;
                for &surface_number in &space.surfaces {
                    let surface = surfaces.get(index(surface_number)?).ok_or(Unavailable::StorageDomain)?;
                    if surface.row.is_air_boundary_surface { continue; }
                    if surface_number == surface.representative_calc_surface_num {
                        *pointer.get_mut(used).ok_or(Unavailable::StorageDomain)? = surface_number;
                        *enclosure.area.as_mut().and_then(|v| v.get_mut(used)).ok_or(Unavailable::StorageDomain)? = surface.row.area;
                        let construction = source.constructions.construction_usage
                            .get(index(surface.row.construction)?).ok_or(Unavailable::StorageDomain)?;
                        *enclosure.emissivity.as_mut().and_then(|v| v.get_mut(used)).ok_or(Unavailable::StorageDomain)? = construction.inside_absorp_thermal;
                        let properties = surface.row.vertex_progress.properties
                            .ok_or(Unavailable::EnclosureDomain)?;
                        *enclosure.azimuth.as_mut().and_then(|v| v.get_mut(used)).ok_or(Unavailable::StorageDomain)? = properties.azimuth_deg;
                        *enclosure.tilt.as_mut().and_then(|v| v.get_mut(used)).ok_or(Unavailable::StorageDomain)? = properties.tilt_deg;
                        used += 1;
                    }
                }
                for &surface_number in &space.surfaces {
                    let surface = surfaces.get(index(surface_number)?).ok_or(Unavailable::StorageDomain)?;
                    if surface.row.is_air_boundary_surface { continue; }
                    if surface_number != surface.representative_calc_surface_num {
                        for (position, &source_pointer) in pointer.iter().enumerate().take(used).skip(prior) {
                            if source_pointer == surface.representative_calc_surface_num {
                                let area = surface.row.area;
                                *enclosure.area.as_mut().and_then(|v| v.get_mut(position))
                                    .ok_or(Unavailable::StorageDomain)? += area;
                            }
                        }
                    }
                    let reported = *source.geometry.all_surface_list_report_order
                        .get(index(surface_number)?).ok_or(Unavailable::StorageDomain)?;
                    for (position, &source_pointer) in pointer.iter().enumerate().take(used).skip(prior) {
                        if source_pointer == reported {
                            enclosure.source_surface_report_nums.push(
                                i32::try_from(position + 1).map_err(|_| Unavailable::StorageDomain)?,
                            );
                            break;
                        }
                    }
                }
            }
            if n == 1 {
                // The dimensioned arrays already hold Source positive zero.
                enclosure.f.as_mut().ok_or(Unavailable::StorageDomain)?.fill(0.0);
                enclosure.script_f.as_mut().ok_or(Unavailable::StorageDomain)?.fill(0.0);
                if output.advanced_report_variables {
                    writeln!(output.eio, "Surface View Factor Check Values,{},0,0,0,-1,0,0", enclosure.name)?;
                }
                continue;
            }
            // Actual613 queries even when it returns zero. The copied-Solar
            // branch uses the same finalized Source F, not reconstructed math.
            let user_count = output.input.get_num_objects_found("ZoneProperty:UserViewFactors:BySurfaceName");
            let solar = self.solar.as_ref().and_then(|rows| rows.get(number))
                .ok_or(Unavailable::StorageDomain)?;
            let use_solar = (!source.geometry.use_representative_surface_calculations || user_count > 0)
                && !source.any_inside_shelf && !solar.f.is_empty();
            if !use_solar {
                // Positive input would next enter its genuine getter, or the
                // unported Source diagnostic-aware Approx/Fix caller.
                return Err(Unavailable::InputBranch.into());
            }
            *enclosure.f.as_mut().ok_or(Unavailable::StorageDomain)? = solar.f.clone();
            let calculated = script_f(
                enclosure.area.as_deref().ok_or(Unavailable::StorageDomain)?,
                enclosure.f.as_deref().ok_or(Unavailable::StorageDomain)?,
                enclosure.emissivity.as_deref_mut().ok_or(Unavailable::StorageDomain)?,
                output.backend,
            )?;
            *enclosure.script_f.as_mut().ok_or(Unavailable::StorageDomain)? = calculated;
            // The useSolarViewFactors branch skips the final check-value row.
        }
        if errors_found {
            return match output.backend.show_fatal_error("InitInteriorRadExchange: Errors found during initialization of radiant exchange.  Program terminated.") {
                Ok(never) => match never {},
                Err(error) => Err(Error::Input(Box::new(crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError::Fatal(error)))),
            };
        }
        Ok(())
    }
}
