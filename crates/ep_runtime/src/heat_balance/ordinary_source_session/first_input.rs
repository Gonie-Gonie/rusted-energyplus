//! Real ManageHeatBalance167-187 host on the same retained context/session.
use super::{OrdinarySourceSession, SourceSessionSurfaces};
use crate::RuntimeError;
use crate::heat_balance::ctf_initialization_context::{
    CtfInitializationContext, CtfInitializationReports, hb_input_error,
};
use crate::heat_balance::ctf_thermal_history::CtfThermalHistoryState;
use crate::heat_balance::inside_convection_dispatcher::source::SourceInsideConvection;
use crate::heat_balance::ordinary_equipment::OrdinaryEquipmentInput;
use crate::heat_balance::ordinary_heat_balance_initialization::SourceHeatBalanceManager;
use crate::heat_balance::ordinary_heat_balance_initialization::first_call::{
    SourceManageHeatBalanceFirstCallChildren, prepare_source_manage_heat_balance_first_call,
};
use crate::heat_balance::radiation::ordinary::OrdinaryRadiationInput;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as Error, surface::SurfaceGeometryInputState,
};
use crate::heat_balance::selected_heat_balance_space_controls::SelectedHeatBalanceSpaceControls;
use crate::heat_balance::selected_input_processor::SelectedInputProcessor;
use crate::heat_balance::source_output_registration::SourceMtdFileHost;
use crate::weather::day::{WeatherEnvironmentState, WeatherGlobalState};
use ep_model::TypedModel;

impl OrdinarySourceSession {
    /// Execute only the genuine first-input parent167-187. The later189-238
    /// body must borrow these same retained owners at its actual caller position.
    #[expect(
        clippy::too_many_arguments,
        reason = "Keep actual Source owners and debug stream explicit"
    )]
    pub(crate) fn prepare_first_input(
        &mut self,
        context: &mut CtfInitializationContext<'_>,
        model: &TypedModel,
        environment: &mut WeatherEnvironmentState,
        clock: &WeatherGlobalState,
        minutes: i32,
        output_host: &mut SourceMtdFileHost,
        equipment_input: OrdinaryEquipmentInput,
        radiation_input: OrdinaryRadiationInput,
        debug: &mut dyn std::io::Write,
        dictionary_file: crate::heat_balance::ctf_initialization_context::internal_heat_gains_input::SourceGainDictionaryFile<'_>,
    ) -> Result<(), RuntimeError> {
        let result = prepare_source_manage_heat_balance_first_call(&mut SourceFirstInputHost {
            session: self,
            context,
            model,
            environment,
            clock,
            minutes,
            output_host,
            equipment_input,
            radiation_input,
            debug,
            dictionary_file: Some(dictionary_file),
        });
        result.map_err(hb_input_error)
    }

    fn retain_source_surface_owners(
        &mut self,
        reports: &mut CtfInitializationReports,
    ) -> Result<(), Error> {
        if self.surfaces.is_some() {
            return Err(Error::Unavailable(
                "actual Source surface owners already retained before first-input transfer",
            ));
        }
        let geometry = &mut reports.surface_geometry_input;
        let zones = reports
            .selected_zone_input
            .as_ref()
            .ok_or(Error::Unavailable(
                "actual completed GetHeatBalanceInput Zone/Space owners",
            ))?;
        // This constructor retains Source identities and native control flags;
        // GetUser/static coefficient initialization remains at its actual call.
        let convection = SourceInsideConvection::new(geometry, zones)?;
        let total = usize::try_from(geometry.total_surfaces)
            .map_err(|_| Error::Unavailable("actual Source history surface extent"))?;
        let rows = geometry.final_surfaces.as_ref().ok_or(Error::Unavailable(
            "actual final Surface before Source history identity",
        ))?;
        if rows.len() != total {
            return Err(Error::Unavailable(
                "actual Source history full final Surface identity",
            ));
        }
        let ids = rows
            .iter()
            .map(|row| {
                row.typed_id.ok_or(Error::Unavailable(
                    "actual final Surface typed identity for unallocated Source history",
                ))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let history = CtfThermalHistoryState::new_source_unallocated(ids);
        // The actual move validates full shape/unique identity/no old model
        // permutation before taking. There is no fallible operation after it.
        let outdoor = geometry.take_source_outdoor_in_surface_order()?;
        self.surfaces = Some(SourceSessionSurfaces {
            convection,
            history,
            outdoor,
        });
        Ok(())
    }
}

struct SourceFirstInputHost<'a, 'context, 'dictionary> {
    session: &'a mut OrdinarySourceSession,
    context: &'a mut CtfInitializationContext<'context>,
    model: &'a TypedModel,
    environment: &'a mut WeatherEnvironmentState,
    clock: &'a WeatherGlobalState,
    minutes: i32,
    output_host: &'a mut SourceMtdFileHost,
    equipment_input: OrdinaryEquipmentInput,
    radiation_input: OrdinaryRadiationInput,
    debug: &'a mut dyn std::io::Write,
    dictionary_file: Option<crate::heat_balance::ctf_initialization_context::internal_heat_gains_input::SourceGainDictionaryFile<'dictionary>>,
}
impl SourceManageHeatBalanceFirstCallChildren for SourceFirstInputHost<'_, '_, '_> {
    fn manager(&self) -> &SourceHeatBalanceManager {
        &self.session.heat_balance
    }
    fn manager_mut(&mut self) -> &mut SourceHeatBalanceManager {
        &mut self.session.heat_balance
    }
    fn get_heat_balance_input(&mut self) -> Result<(), Error> {
        if self.session.surfaces.is_some() {
            return Err(Error::Unavailable(
                "retained Source surface owners cannot replay the first-input transfer",
            ));
        }
        // Partial same-owner input effects survive an error. No legacy full
        // thermal factory, re-used CTF result or copied report answer is invoked.
        self.context
            .get_source_heat_balance_input_with_debug(
                self.model,
                self.environment,
                Some(self.clock),
                self.minutes,
                self.output_host,
                self.equipment_input,
                self.radiation_input,
                &mut self.session.hvac.ancillary.branch_node_connections,
                Some(&mut *self.debug),
                self.dictionary_file.take(),
            )
            .map_err(|cause| Error::SourceInput(Box::new(cause)))?;
        self.session
            .retain_source_surface_owners(self.context.reports)
    }
    fn globals(&self) -> Result<&SelectedHeatBalanceGlobals, Error> {
        self.context
            .reports
            .selected_heat_balance_globals
            .as_ref()
            .ok_or(Error::Unavailable(
                "actual same global owner in ManageHeatBalance first call",
            ))
    }
    fn space_controls(&mut self) -> &mut SelectedHeatBalanceSpaceControls {
        &mut self.context.reports.heat_balance_space_controls
    }
    fn geometry(&mut self) -> &mut SurfaceGeometryInputState {
        &mut self.context.reports.surface_geometry_input
    }
    fn input(&self) -> Result<&SelectedInputProcessor, Error> {
        self.context
            .reports
            .selected_input_processor
            .as_ref()
            .ok_or(Error::Unavailable(
                "actual same IP owner in ManageHeatBalance first call",
            ))
    }
}
