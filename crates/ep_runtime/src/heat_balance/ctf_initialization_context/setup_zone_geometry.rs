//! Selected Setup through801; full transitive Setup/HB closure remains separate.
use super::{CtfInitializationContext, hb_input_error};
use crate::error::RuntimeError;
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::ordinary_equipment::{
    OrdinaryEquipment, OrdinaryEquipmentInput, OrdinaryEquipmentUnavailable,
};
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as E;

fn equipment_error(
    error: OrdinaryEquipmentUnavailable,
    backend: &mut SourceErrorBackend<'_>,
) -> RuntimeError {
    match error {
        OrdinaryEquipmentUnavailable::Fatal(message) => match backend.show_fatal_error(&message) {
            Err(stop) => hb_input_error(E::Fatal(stop)),
            Ok(never) => match never {},
        },
        other => RuntimeError::WarmupUnavailable {
            reason: format!("Source GetZoneEquipment unavailable: {other}"),
        },
    }
}

impl CtfInitializationContext<'_> {
    pub(crate) fn continue_selected_setup_zone_geometry(
        &mut self,
        model: &ep_model::TypedModel,
        clock: Option<OrdinaryEquipmentInput>,
        environment: &crate::weather::day::WeatherEnvironmentState,
        minutes_in_time_step: i32,
        host: &mut dyn crate::heat_balance::source_output_registration::SourceOutputFileHost,
    ) -> Result<Vec<crate::geometry::InitializedZoneGeometry>, RuntimeError> {
        let reports = &mut *self.reports;
        if reports
            .surface_geometry_input
            .release_setup_coordinates_if_errors(reports.heat_balance_input_errors_found)
        {
            // Native292-296 returns here. Its later parent HB fatal is unported.
            return Err(hb_input_error(E::Unavailable(
                "SetupZoneGeometry input-error return before parent HB closure",
            )));
        }
        if !reports
            .surface_geometry_input
            .selected_get_surface_data_returned
        {
            return Err(hb_input_error(E::Unavailable(
                "actual selected GetSurfaceData return before Setup298",
            )));
        }
        // Prove the actual Source allocation operands and associations before
        // borrowing immutable physical IDs; model length alone is not a writer.
        let globals = reports
            .selected_heat_balance_globals
            .as_ref()
            .ok_or_else(|| hb_input_error(E::Unavailable("actual equipment DataGlobal owner")))?;
        let source_zones = reports
            .selected_zone_input
            .as_ref()
            .ok_or_else(|| hb_input_error(E::Unavailable("actual equipment Zone/Space owner")))?;
        let zones = source_zones
            .zones
            .as_ref()
            .ok_or_else(|| hb_input_error(E::Unavailable("actual equipment Zone allocation")))?;
        let spaces = source_zones
            .spaces
            .as_ref()
            .ok_or_else(|| hb_input_error(E::Unavailable("actual equipment Space allocation")))?;
        if model.zones.len() != 1
            || model.spaces.len() != 1
            || zones.len() != 1
            || spaces.len() != 1
            || globals.num_of_zones != 1
            || globals.num_spaces != 1
        {
            return Err(hb_input_error(E::Unavailable(
                "selected actual equipment allocation operands",
            )));
        }
        if zones[0].typed_zone_id != Some(model.zones[0].id)
            || spaces[0].typed_space_id != Some(model.spaces[0].id)
            || spaces[0].zone_num != 1
        {
            return Err(hb_input_error(E::Unavailable(
                "same physical equipment Source associations",
            )));
        }
        let clock = clock.ok_or_else(|| {
            hb_input_error(E::Unavailable(
                "actual effective equipment timestep operand",
            ))
        })?;
        if reports.source_zone_equipment.is_none() {
            reports.source_zone_equipment = Some(
                OrdinaryEquipment::new_for_source(model, clock)
                    .map_err(|error| equipment_error(error, &mut self.backend))?,
            );
        }
        let input = reports
            .selected_input_processor
            .as_mut()
            .ok_or_else(|| hb_input_error(E::Unavailable("same actual IP at Setup298")))?;
        reports
            .source_zone_equipment
            .as_mut()
            .ok_or_else(|| hb_input_error(E::Unavailable("retained actual equipment constructor")))?
            .get_zone_equipment_source(
                input,
                clock,
                reports.heat_balance_space_controls.sizing,
                reports.heat_balance_space_controls.simulation,
            )
            .map_err(|error| equipment_error(error, &mut self.backend))?;
        reports
            .surface_geometry_input
            .finish_zero_window_setup(
                input,
                &mut reports.input_processor_shortcut,
                reports.heat_balance_input_errors_found,
            )
            .map_err(hb_input_error)?;
        reports
            .surface_geometry_input
            .process_opaque_setup_surfaces(
                reports.selected_zone_input.as_mut().ok_or_else(|| {
                    hb_input_error(E::Unavailable("same actual Setup Zone/Space owner"))
                })?,
                &mut reports.heat_balance_input_state,
            )
            .map_err(hb_input_error)?;
        let detailed_wwr = reports
            .surface_geometry_input
            .accumulate_opaque_setup_areas(
                input,
                reports.selected_zone_input.as_mut().ok_or_else(|| {
                    hb_input_error(E::Unavailable("same actual area Zone/Space owner"))
                })?,
                &reports.heat_balance_input_state,
            )
            .map_err(hb_input_error)?;
        reports
            .surface_geometry_input
            .prepare_opaque_setup_heights(
                reports.selected_zone_input.as_mut().ok_or_else(|| {
                    hb_input_error(E::Unavailable("same actual height Zone/Space owner"))
                })?,
                globals.display_extra_warnings,
                detailed_wwr,
                &mut self.backend,
            )
            .map_err(hb_input_error)?;
        let volume_returns = reports
            .surface_geometry_input
            .calculate_opaque_setup_volumes(
                input,
                reports.selected_zone_input.as_mut().ok_or_else(|| {
                    hb_input_error(E::Unavailable("same actual volume Zone/Space owner"))
                })?,
                globals.display_extra_warnings,
                &mut self.backend,
            )
            .map_err(hb_input_error)?;
        reports
            .surface_geometry_input
            .prepare_opaque_setup_zone_bounds(
                reports.selected_zone_input.as_mut().ok_or_else(|| {
                    hb_input_error(E::Unavailable("same actual centroid/bounds Zone owner"))
                })?,
                globals.num_of_zones,
                &mut self.backend,
                &mut reports.heat_balance_input_errors_found,
            )
            .map_err(hb_input_error)?;
        reports.write_selected_setup_geometry_reports(self.eio)?;
        // Actual797 borrows the same earlier radiant/Solar container and the
        // caller's still-live IP/output owners, before any800 temperature write.
        {
            let radiation = reports.ordinary_solar_radiation.as_mut().ok_or_else(|| {
                hb_input_error(E::Unavailable(
                    "same retained Source Solar enclosure container",
                ))
            })?;
            let mut owners =
                crate::heat_balance::selected_heat_balance_input::surface::SurfaceGetterOwners {
                    model,
                    environment,
                    input: reports.selected_input_processor.as_mut().ok_or_else(|| {
                        hb_input_error(E::Unavailable("same Source IP at Solar enclosure setup"))
                    })?,
                    shortcut: &mut reports.input_processor_shortcut,
                    zones: reports.selected_zone_input.as_mut().ok_or_else(|| {
                        hb_input_error(E::Unavailable(
                            "same Source Zone/Space at Solar enclosure setup",
                        ))
                    })?,
                    control: &mut reports.heat_balance_input_state,
                    globals: reports
                        .selected_heat_balance_globals
                        .as_ref()
                        .ok_or_else(|| {
                            hb_input_error(E::Unavailable(
                                "same Source DataGlobal at Solar enclosure setup",
                            ))
                        })?,
                };
            let mut output = crate::heat_balance::radiation::ordinary::EnclosureOutputOwners {
                output: &mut reports.output_registration,
                prescan: &reports.output_prescan,
                minimum_report_frequency: &reports
                    .selected_project_data
                    .as_ref()
                    .ok_or_else(|| {
                        hb_input_error(E::Unavailable("same Solar enclosure process settings"))
                    })?
                    .system
                    .minimum_report_frequency,
                minutes_in_time_step,
                host,
                eio: self.eio,
            };
            radiation
                .setup_solar_enclosures(&reports.surface_geometry_input, &mut owners, &mut output)
                .map_err(hb_input_error)?;
        }
        // Existing returned getters witness the actual Site/Station operands;
        // their absence cannot silently substitute constructor parameters.
        if !environment.source_weather_station_initialized
            || !environment.source_site_atmosphere_initialized
        {
            return Err(hb_input_error(E::Unavailable(
                "actual returned Site/Station before Setup800",
            )));
        }
        let zones = reports
            .selected_zone_input
            .as_mut()
            .and_then(|owner| owner.zones.as_deref_mut())
            .ok_or_else(|| {
                hb_input_error(E::Unavailable(
                    "same Source Zone allocation before Setup800",
                ))
            })?;
        // Actual800 writes every Zone's bulbs;801 then performs the ordered
        // cumulative check. Wind and Surface weather updates are later work.
        crate::heat_balance::convection::surface_environment::ordinary::update_zone_out_bulb_temperatures(
            zones,
            environment.outdoor_environment_parameters,
            environment.current_weather.out_dry_bulb_temp,
            environment.current_weather.out_wet_bulb_temp,
            &mut self.backend,
        )?;
        Ok(volume_returns)
    }
}
