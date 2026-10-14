//! Selected GetHeatBalanceInput278-313 parent prefix after GetBuildingData.
//! This stops before InitSolar316 and does not set the ManageHeatBalance guard.
use super::{CtfInitializationContext, SourceErrorBackend, hb_input_error};
use crate::RuntimeError;
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as E, SelectedHeatBalanceInputState,
};
use crate::heat_balance::selected_input_processor::SelectedInputProcessor;

fn unavailable(message: &'static str) -> RuntimeError {
    hb_input_error(E::Unavailable(message))
}

fn count(input: &SelectedInputProcessor, family: &str) -> Result<i32, RuntimeError> {
    i32::try_from(input.get_num_objects_found(family))
        .map_err(|_| unavailable("post-building Source object count int capacity"))
}

fn create_selected_tc_constructions(
    control: &SelectedHeatBalanceInputState,
) -> Result<(), RuntimeError> {
    // Actual5256-5285 traverses real flags. Every false branch continues, so
    // the genuine local NumNewConst remains0 and returns before any redimension.
    for index in 0..control.total_constructions {
        let row = control
            .construction_usage
            .get(index)
            .ok_or_else(|| unavailable("actual TC Construction association"))?;
        if row.is_tc_master {
            return Err(unavailable(
                "positive MaterialGlassTC construction producer",
            ));
        }
    }
    Ok(())
}

fn valid_simulation_objects(input: &SelectedInputProcessor) -> bool {
    // Exact405-445 short-circuit query order; no generator admission is added.
    [
        "SolarCollector:FlatPlate:Water",
        "Generator:Photovoltaic",
        "Generator:InternalCombustionEngine",
        "Generator:CombustionTurbine",
        "Generator:FuelCell",
        "Generator:MicroCHP",
        "Generator:MicroTurbine",
        "Generator:WindTurbine",
    ]
    .iter()
    .any(|name| input.get_num_objects_found(name) > 0)
}

fn check_used_constructions(
    control: &SelectedHeatBalanceInputState,
    input: &SelectedInputProcessor,
    display_extra_warnings: bool,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), RuntimeError> {
    for family in [
        "Pipe:Indoor",
        "Pipe:Outdoor",
        "Pipe:Underground",
        "GroundHeatExchanger:Surface",
        "DaylightingDevice:Tubular",
        "EnergyManagementSystem:ConstructionIndexVariable",
    ] {
        if input.get_num_objects_found(family) > 0 {
            // The next real operation is getObjectItem with the shared arrays.
            // Preserve prior queries/owners; no positive getter is fabricated.
            return Err(unavailable("positive CheckUsedConstructions shared getter"));
        }
    }
    let total = i32::try_from(control.total_constructions)
        .map_err(|_| unavailable("actual TotConstructs int capacity"))?;
    let used = i32::try_from(
        control
            .construction_usage
            .iter()
            .filter(|row| row.is_used)
            .count(),
    )
    .map_err(|_| unavailable("actual used Construction count int capacity"))?;
    let unused = total
        .checked_sub(used)
        .ok_or_else(|| unavailable("unused Construction count int capacity"))?;
    if unused > 0 {
        backend.show_warning_error(&format!("CheckUsedConstructions: There are {unused} nominally unused constructions in input."))
            .map_err(|error| hb_input_error(E::Backend(error)))?;
        if !display_extra_warnings {
            backend.show_continue_error("For explicit details on each unused construction, use Output:Diagnostics,DisplayExtraWarnings;")
                .map_err(|error| hb_input_error(E::Backend(error)))?;
        } else {
            backend
                .show_continue_error("Each Unused construction is shown.")
                .map_err(|error| hb_input_error(E::Backend(error)))?;
            for index in 0..control.total_constructions {
                let row = control
                    .construction_usage
                    .get(index)
                    .ok_or_else(|| unavailable("actual unused Construction Name association"))?;
                if row.is_used {
                    continue;
                }
                backend
                    .show_message(&format!("Construction={}", row.name))
                    .map_err(|error| hb_input_error(E::Backend(error)))?;
            }
        }
    }
    Ok(()) // Native ErrorsFound parameter is deliberately not written here.
}

impl CtfInitializationContext<'_> {
    pub(crate) fn continue_selected_heat_balance_input_before_solar(
        &mut self,
    ) -> Result<(), RuntimeError> {
        let reports = &mut *self.reports;
        // Actual DataSurfaces749 returns on the genuine retained Material flag,
        // without accessing or resetting AllVaryAbsOpaqSurfaceList.
        if reports.selected_material_data.any_variable_absorptance {
            return Err(unavailable(
                "positive variable absorptance Surface list producer",
            ));
        }
        let input = reports
            .selected_input_processor
            .as_mut()
            .ok_or_else(|| unavailable("same post-building InputProcessor owner"))?;
        let shortcut = &mut reports.input_processor_shortcut;
        shortcut.current_module_object = "SurfaceProperty:IncidentSolarMultiplier".into();
        reports
            .surface_geometry_input
            .total_incident_solar_multipliers = count(input, &shortcut.current_module_object)?;
        if reports
            .surface_geometry_input
            .total_incident_solar_multipliers
            > 0
        {
            return Err(unavailable(
                "positive incident solar multiplier allocation/getter",
            ));
        } // Actual2096 returns before allocation; no empty array is manufactured.

        shortcut.current_module_object = "SurfaceProperty:SolarIncidentInside".into();
        let arguments = input
            .get_object_def_max_args(&shortcut.current_module_object)
            .map_err(|_| unavailable("actual SolarIncidentInside schema definition/counts"))?;
        if arguments.alphas != 4 {
            self.backend.show_severe_error(&format!(
                "GetScheduledSurfaceGains: {}: Object Definition indicates not = 4 Alpha Objects, Number Indicated={}",
                shortcut.current_module_object, arguments.alphas))
                .map_err(|error| hb_input_error(E::Backend(error)))?;
            reports.heat_balance_input_errors_found = true; // AFTER successful severe IO.
        }
        reports
            .surface_geometry_input
            .total_incident_solar_scheduled_gains = count(input, &shortcut.current_module_object)?;
        if reports
            .surface_geometry_input
            .total_incident_solar_scheduled_gains
            > 0
        {
            return Err(unavailable(
                "positive incident scheduled surface gains allocation/getter",
            ));
        }
        shortcut.current_module_object = "ComplexFenestrationProperty:SolarAbsorbedLayers".into();
        reports
            .surface_geometry_input
            .total_fenestration_absorbed_solar_scheduled_gains =
            count(input, &shortcut.current_module_object)?;
        if reports
            .surface_geometry_input
            .total_fenestration_absorbed_solar_scheduled_gains
            > 0
        {
            return Err(unavailable(
                "positive fenestration absorbed scheduled gains allocation/getter",
            ));
        } // Both zero: actual5178 short-circuits every CheckScheduledSurfaceGains call.

        if reports
            .surface_geometry_input
            .use_representative_surface_calculations
        {
            return Err(unavailable(
                "positive representative Surface assignment report",
            ));
        }
        create_selected_tc_constructions(&reports.heat_balance_input_state)?;
        let globals = reports
            .selected_heat_balance_globals
            .as_ref()
            .ok_or_else(|| unavailable("same post-building DataGlobal owner"))?;
        if reports.surface_geometry_input.total_surfaces > 0
            && globals.num_of_zones == 0
            && !valid_simulation_objects(input)
        {
            self.backend.show_severe_error("GetHeatBalanceInput: There are surfaces in input but no zones found.  Invalid simulation.")
                .map_err(|error| hb_input_error(E::Backend(error)))?;
            reports.heat_balance_input_errors_found = true;
        }
        check_used_constructions(
            &reports.heat_balance_input_state,
            input,
            globals.display_extra_warnings,
            &mut self.backend,
        )?;
        if reports.heat_balance_input_errors_found {
            return match self
                .backend
                .show_fatal_error("Errors found in Building Input, Program Stopped")
            {
                Err(error) => Err(hb_input_error(E::Fatal(error))),
                Ok(never) => match never {},
            };
        }
        Ok(()) // This is the selected pre-InitSolar316 boundary, not full HB readiness.
    }
}
