//! Actual HVAC472 / HeatBalance3354 shared TAB child on the same retained owners.
use super::{
    Error, SourceHvacManager, SourceHvacOutputContext, SourceHvacOwners,
    SourceOrdinaryHvacRemaining,
};
use crate::heat_balance::ordinary_tabular_reports::SourceTabularInputOwners;
use crate::heat_balance::source_output_registration::TimeStepKind;
use std::io::Write;
impl SourceOrdinaryHvacRemaining {
    /// The parent supplies its actual time-step kind; this child owns no numeric state.
    pub(crate) fn update_source_ordinary_tabular<H, D>(
        kind: TimeStepKind,
        manager: &mut SourceHvacManager,
        o: &mut SourceHvacOwners<'_, '_>,
        output: &mut SourceHvacOutputContext<'_, H, D>,
        eio: &mut dyn Write,
    ) -> Result<(), Error> {
        o.tabular.update_source_tabular_reports(
            kind,
            &mut SourceTabularInputOwners {
                files: output.output_control,
                results: output.results,
                input: o.input,
                shortcut: o.shortcut,
                raw: output.tabular_raw,
                sizing_raw: output.sizing_raw,
                globals: o.globals,
                clock: o.clock,
                schedules: o.schedules,
                environment: o.environment,
                zones: o.zones,
                gains: o.gains,
                registry: o.predefined_registry,
                registration: output.registration,
                backend: o.backend,
                eio,
                pollution: &mut manager.ancillary.pollution,
                equipment: o.equipment,
                space_controls: o.space_controls,
                hvac: o.hvac,
                control: o.control,
                gain_modules: o.gain_modules,
                heat_emission: &manager.ancillary.heat_emission,
                surface_heat_emission: o.surface_heat_emission,
                controller: o.controller,
                radiation: o.radiation,
                surface_averages: o.surface_averages,
            },
        )
    }
}
