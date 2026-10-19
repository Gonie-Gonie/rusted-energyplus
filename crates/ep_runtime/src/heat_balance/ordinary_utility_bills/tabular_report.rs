//! Actual WriteTabularTariffReports4082-4514 on the same economic/TAB owners.
use super::{Error, SourceEconomicTariff};
use crate::heat_balance::error_backend::{SourceDiagnosticFlags, SourceErrorBackend};
use crate::heat_balance::ordinary_tabular_reports::SourceTabularReports;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneInputOwners;

impl SourceEconomicTariff {
    pub(crate) fn write_source_tabular_tariff_reports(
        &self,
        tabular: &mut SourceTabularReports,
        globals: &SelectedHeatBalanceGlobals,
        zones: &SelectedZoneInputOwners,
        backend: &mut SourceErrorBackend<'_>,
        flags: &SourceDiagnosticFlags<'_>,
    ) -> Result<(), Error> {
        // Native4089-4093 construct local empty arrays, not retained owners.
        // Its local perAreaUnitConv4099 is initialized before the area check.
        let _per_area_unit_conversion = 0.0_f64;
        // This check is BEFORE numTariff4108, even for the zero-tariff case.
        tabular.prepare_source_tariff_report_floor_area(globals, zones)?;
        if self.number_tariffs > 0 {
            if tabular.display_economic_result_summary {
                backend.display_source_string("Writing Tariff Reports", flags)?;
                return Err(Error::Unavailable(
                    "actual WriteTabularTariffReports showWarningsBasedOnTotal4111 child",
                ));
            }
            // Native's displayTariffReport4273 is inside the economic-summary
            // block. If that actual block is false, no tariff report is written.
        }
        Ok(())
    }
}
