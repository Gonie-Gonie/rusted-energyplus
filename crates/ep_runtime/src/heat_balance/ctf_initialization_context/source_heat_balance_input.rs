//! Genuine GetHeatBalanceInput driver over the same retained input owners.
//! Physical thermal state, CTF/history allocation and later HB initialization
//! belong to their actual later callers.
use super::{CtfInitializationContext, hb_input_error};
use crate::RuntimeError;
use crate::heat_balance::ordinary_equipment::OrdinaryEquipmentInput;
use crate::heat_balance::radiation::ordinary::OrdinaryRadiationInput;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError;
use crate::heat_balance::source_output_registration::SourceMtdFileHost;
use crate::weather::day::WeatherEnvironmentState;
use ep_model::TypedModel;

impl CtfInitializationContext<'_> {
    /// Execute the actual selected GetHeatBalanceInput243-326 input sequence.
    ///
    /// `equipment_input` and `radiation_input` are the same retained admitted
    /// metadata after the real Project timestep/reporting control writes. The
    /// actual InputProcessor, shortcuts, zones, geometry, gains and Solar owners
    /// remain on `reports` on both success and failure.
    /// Legacy boundary callers do not own an actual debug stream. Default EIO
    /// reports remain real; a reached positive IDF debug write stops unavailable.
    #[cfg(all(test, feature = "native-controlled-air-cohort"))]
    pub(crate) fn get_source_heat_balance_input(
        &mut self,
        model: &TypedModel,
        environment: &mut WeatherEnvironmentState,
        minutes: i32,
        output_host: &mut SourceMtdFileHost,
        equipment_input: OrdinaryEquipmentInput,
        radiation_input: OrdinaryRadiationInput,
        branch: &mut crate::heat_balance::startup_integrity::SourceBranchNodeConnections,
    ) -> Result<(), RuntimeError> {
        self.get_source_heat_balance_input_with_debug(
            model,
            environment,
            minutes,
            output_host,
            equipment_input,
            radiation_input,
            branch,
            None,
        )
    }

    #[expect(
        clippy::too_many_arguments,
        reason = "Keep the genuine Source input/IO owners distinct"
    )]
    pub(crate) fn get_source_heat_balance_input_with_debug(
        &mut self,
        model: &TypedModel,
        environment: &mut WeatherEnvironmentState,
        minutes: i32,
        output_host: &mut SourceMtdFileHost,
        equipment_input: OrdinaryEquipmentInput,
        radiation_input: OrdinaryRadiationInput,
        branch: &mut crate::heat_balance::startup_integrity::SourceBranchNodeConnections,
        debug: Option<&mut dyn std::io::Write>,
    ) -> Result<(), RuntimeError> {
        // The existing driver initializes the actual ErrorsFound local, then
        // Project/Site/Material/Frame/Construction/Building Zone input in order.
        self.initialize_selected_heat_balance_input(model, environment, minutes, output_host)?;

        // GetBuildingData's real GetSurfaceData/Setup continuation. A different
        // raw geometry route cannot fall through to the legacy typed factory.
        if !self.prepare_selected_world_surface_geometry(
            model,
            environment,
            Some(radiation_input),
            minutes,
            output_host,
        )? {
            return Err(hb_input_error(HeatBalanceInputError::Unavailable(
                "actual Source GetSurfaceData World route required by GetHeatBalanceInput",
            )));
        }
        // Setup writes the same Zone volume and its native outdoor bulbs. Its
        // extra Rust geometry return is not a thermal-state initialization seed.
        let _geometry_return = self.continue_selected_setup_zone_geometry(
            model,
            Some(equipment_input),
            environment,
            minutes,
            output_host,
            Some(branch),
        )?;

        // Actual278-313 includes variable absorptance, incident/scheduled gains,
        // representative/TC/no-zone checks, used constructions and parent fatal.
        self.continue_selected_heat_balance_input_before_solar()?;
        // The existing Solar child restores its same sole partially written
        // owner before propagating either success or failure.
        self.initialize_selected_solar_view_factors_with_debug(model, debug)?;
        // Native320 InitOnly=true reads gains before later daylighting input.
        self.manage_selected_internal_heat_gains_input(model, minutes, output_host)?;
        // Native325-326 reads DataHeatBal.AnyKiva. The older selected helper
        // reads a geometry compartment member, so this genuine parent uses
        // the actual HeatBalance owner directly, without synchronizing aliases.
        if self.reports.heat_balance_input_state.any_kiva {
            return Err(hb_input_error(HeatBalanceInputError::Unavailable(
                "positive Source Kiva setup",
            )));
        }
        Ok(())
    }
}
