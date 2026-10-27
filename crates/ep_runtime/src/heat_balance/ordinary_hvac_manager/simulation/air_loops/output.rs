//! Literal SimAirLoops2428-2692 zero-loop body and live integer bindings.
use super::*;
use crate::heat_balance::radiation::ordinary::{
    OrdinaryRadiationError, OrdinaryRadiationUnavailable,
};
use crate::heat_balance::source_output_registration::{
    SourceIntegerOutputBinding, SourceIntegerOutputDeclaration, SourceIntegerOutputOwners,
    SourceIntegerOutputTarget as Target, SourceOutputStoreType, SourceOutputUnits,
    SourceReportVariableInput, TimeStepKind,
};
impl SourceIntegerOutputOwners for SourceAirLoopData {
    fn integer_output(&self, target: Target) -> Result<&i32, OrdinaryRadiationError> {
        match target {
            Target::AirLoopMaximumIterations => Ok(&self.simulation_iteration_maximum),
            Target::AirLoopTotalIterations => Ok(&self.simulation_iteration_total),
            Target::AirLoopTotalCalls => Ok(&self.simulation_calls_total),
            _ => Err(OrdinaryRadiationUnavailable::StorageDomain.into()),
        }
    }
}
/// Final whole-State sampler composes actual AirLoop counters and the existing
/// Hvac/Plant resolver; both remain borrowed, with no copied numeric dictionary.
pub(crate) struct SourceAirLoopAndHvacIntegerOwners<'a> {
    pub(crate) air_loop: &'a SourceAirLoopData,
    pub(crate) hvac: &'a dyn SourceIntegerOutputOwners,
}
impl SourceIntegerOutputOwners for SourceAirLoopAndHvacIntegerOwners<'_> {
    fn integer_output(&self, target: Target) -> Result<&i32, OrdinaryRadiationError> {
        match target {
            Target::AirLoopMaximumIterations
            | Target::AirLoopTotalIterations
            | Target::AirLoopTotalCalls => self.air_loop.integer_output(target),
            _ => self.hvac.integer_output(target),
        }
    }
}
impl SourceAirLoopData {
    pub(super) fn simulate_source_air_loops<
        H: SourceOutputFileHost,
        D: SourceOutputDictionaryHost,
    >(
        &mut self,
        first: bool,
        o: &mut SourceAirLoopOwners<'_, '_>,
        output: &mut SourceAirReportRegistrationContext<'_, H, D>,
    ) -> Result<(), Error> {
        if !self.output_setup_flag {
            for (target, name) in [
                (
                    Target::AirLoopMaximumIterations,
                    "Air System Simulation Maximum Iteration Count",
                ),
                (
                    Target::AirLoopTotalIterations,
                    "Air System Simulation Iteration Count",
                ),
                (
                    Target::AirLoopTotalCalls,
                    "Air System Component Model Simulation Calls",
                ),
            ] {
                let binding =
                    SourceIntegerOutputBinding::new(target, &*self).map_err(Error::Output)?;
                let mut report_input = SourceReportVariableInput::new(
                    output.reports,
                    o.input,
                    crate::heat_balance::source_output_registration::SourceReportVariableEnvironment::from_owners(output.minimum_report_frequency, o.globals, &o.project.system.developer_flag, Some(o.clock)),
                    o.backend,
                );
                output
                    .registration
                    .setup_bound_integer(
                        &mut report_input,
                        output.prescan,
                        SourceIntegerOutputDeclaration {
                            name,
                            key: "SimAir",
                            units: SourceOutputUnits::Dimensionless,
                            store_type: SourceOutputStoreType::Sum,
                        },
                        output.minutes_in_time_step,
                        output.host,
                        output.eio,
                        binding,
                        output.dictionary_host,
                        TimeStepKind::System,
                    )
                    .map_err(Error::Output)?;
            }
            self.output_setup_flag = true;
        }
        self.simulation_iteration_maximum = 0;
        if first {
            let previous_time =
                (o.clock.current_time - o.clock.time_step_zone) + o.hvac.system_time_elapsed;
            let rx_time = previous_time * 3600.0;
            if self.saved_previous_hvac_time != rx_time {
                self.saved_previous_hvac_time = rx_time;
                self.simulation_iteration_total = 0;
                self.simulation_calls_total = 0;
            }
        }
        if o.hvac.number_of_primary_air_systems > 0 {
            return Err(Error::Unavailable(
                "positive actual SimAirLoops TurnFansOn2538",
            ));
        }
        // This real outer DOAS-vector predicate is independent of primary count.
        if !self.dedicated_outdoor_air_systems.is_empty() {
            return Err(Error::Unavailable(
                "positive actual DOAS m_AirLoopDOASNum first read2610",
            ));
        }
        // No primary system body touches the supplied SimZoneEquipment cell.
        o.sizing.current_system_number = 0;
        Ok(())
    }
}
