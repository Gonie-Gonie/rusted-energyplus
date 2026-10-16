//! Literal Summary1404-1885 selection, titles, actual registry and BEPS.
use super::*;
fn count(value: i32) -> Result<usize, Error> {
    usize::try_from(value).map_err(|_| Error::Unavailable("actual TAB predefined report count"))
}
fn report_rows(
    registry: &mut PredefinedRegistry,
) -> Result<&mut [crate::heat_balance::predefined_tables::PredefinedReportName], Error> {
    let n = count(registry.report_count)?;
    if n == 0 {
        return Ok(&mut []);
    }
    registry
        .reports
        .as_mut()
        .and_then(|v| v.get_mut(..n))
        .ok_or(Error::Unavailable(
            "same actual predefined reportName allocated occupied prefix",
        ))
}
impl SourceTabularReports {
    pub(super) fn get_source_summary_reports(
        &mut self,
        o: &mut SourceTabularInputOwners<'_, '_>,
    ) -> Result<(), Error> {
        const FAMILY: &str = "Output:Table:SummaryReports";
        let mut errors_found = false;
        if !o.files.source_write_tabular(o.results) {
            self.input_state.write_tabular_files = false;
            return Ok(());
        }
        let objects = o.input.get_num_objects_found(FAMILY);
        if objects == 1 {
            let counts = o
                .input
                .get_object_def_max_args(FAMILY)
                .map_err(|_| Error::Unavailable("actual SummaryReports definition1455"))?;
            let _alphas = vec![String::new(); counts.alphas];
            let _numbers = vec![0.0; counts.numerics];
            let alphas = o.raw.summary_values(o.sizing_raw, o.input, counts.alphas)?;
            self.display_tabular_beps = false;
            self.initialize_source_monthly_titles(o)?;
            for (at, name) in alphas.iter().enumerate() {
                let mut name_found = false;
                match name.to_ascii_uppercase().as_str() {
                    "" => o
                        .backend
                        .show_warning_error("Blank report name in Output:Table:SummaryReports")?,
                    "ANNUALBUILDINGUTILITYPERFORMANCESUMMARY" => {
                        self.display_tabular_beps = true;
                        self.input_state.write_tabular_files = true;
                        name_found = true;
                    }
                    "COMPONENTCOSTECONOMICSSUMMARY" => {
                        self.display_tabular_comp_costs = true;
                        self.input_state.write_tabular_files = true;
                        name_found = true;
                    }
                    "INPUTVERIFICATIONANDRESULTSSUMMARY" => {
                        self.display_tabular_veri_sum = true;
                        self.input_state.write_tabular_files = true;
                        name_found = true;
                    }
                    "COMPONENTSIZINGSUMMARY" => {
                        self.display_component_sizing = true;
                        self.input_state.write_tabular_files = true;
                        name_found = true;
                    }
                    "SURFACESHADOWINGSUMMARY" => {
                        self.display_surface_shadowing = true;
                        self.input_state.write_tabular_files = true;
                        name_found = true;
                    }
                    "DEMANDENDUSECOMPONENTSSUMMARY" => {
                        self.display_demand_end_use = true;
                        self.input_state.write_tabular_files = true;
                        name_found = true;
                    }
                    "ADAPTIVECOMFORTSUMMARY" => {
                        self.display_adaptive_comfort = true;
                        self.input_state.write_tabular_files = true;
                        name_found = true;
                    }
                    "SOURCEENERGYENDUSECOMPONENTSSUMMARY" => {
                        self.display_source_energy_end_use_summary = true;
                        self.input_state.write_tabular_files = true;
                        name_found = true;
                    }
                    "ZONECOMPONENTLOADSUMMARY" => {
                        self.display_zone_component_load_summary = true;
                        self.input_state.write_tabular_files = true;
                        name_found = true;
                    }
                    "AIRLOOPCOMPONENTLOADSUMMARY" => {
                        self.display_air_loop_component_load_summary = true;
                        self.input_state.write_tabular_files = true;
                        name_found = true;
                    }
                    "FACILITYCOMPONENTLOADSUMMARY" => {
                        self.display_facility_component_load_summary = true;
                        self.input_state.write_tabular_files = true;
                        name_found = true;
                    }
                    "LEEDSUMMARY" => {
                        self.display_leed_summary = true;
                        self.input_state.write_tabular_files = true;
                        name_found = true;
                    }
                    "LIFECYCLECOSTREPORT" => {
                        self.display_life_cycle_cost_report = true;
                        self.input_state.write_tabular_files = true;
                        name_found = true;
                    }
                    "TARIFFREPORT" => {
                        self.display_tariff_report = true;
                        self.input_state.write_tabular_files = true;
                        name_found = true;
                    }
                    "ECONOMICRESULTSUMMARY" => {
                        self.display_economic_result_summary = true;
                        self.input_state.write_tabular_files = true;
                        name_found = true;
                    }
                    "HEATEMISSIONSSUMMARY" => {
                        self.display_heat_emissions_summary = true;
                        self.input_state.write_tabular_files = true;
                        name_found = true;
                    }
                    "THERMALRESILIENCESUMMARY" => {
                        self.display_thermal_resilience_summary = true;
                        self.display_thermal_resilience_summary_explicitly = true;
                        self.input_state.write_tabular_files = true;
                        name_found = true;
                    }
                    "CO2RESILIENCESUMMARY" => {
                        self.display_co2_resilience_summary = true;
                        self.display_co2_resilience_summary_explicitly = true;
                        self.input_state.write_tabular_files = true;
                        name_found = true;
                    }
                    "VISUALRESILIENCESUMMARY" => {
                        self.display_visual_resilience_summary = true;
                        self.display_visual_resilience_summary_explicitly = true;
                        self.input_state.write_tabular_files = true;
                        name_found = true;
                    }
                    "ENERGYMETERS" => {
                        self.input_state.write_tabular_files = true;
                        name_found = true;
                    }
                    "INITIALIZATIONSUMMARY" => {
                        self.input_state.write_tabular_files = true;
                        self.display_eio_summary = true;
                        name_found = true;
                    }
                    "ALLSUMMARY" => {
                        self.source_all_summary_flags();
                        name_found = true;
                        for row in report_rows(o.registry)? {
                            row.show = true;
                        }
                    }
                    "ALLSUMMARYANDSIZINGPERIOD" => {
                        self.source_all_summary_flags();
                        name_found = true;
                        for row in report_rows(o.registry)? {
                            row.show = true;
                        }
                        self.source_component_load_flags();
                    }
                    "ALLMONTHLY" => {
                        self.input_state.write_tabular_files = true;
                        for row in self.monthly_rows()? {
                            row.show = true;
                        }
                        name_found = true;
                    }
                    "ALLSUMMARYANDMONTHLY" => {
                        self.source_all_summary_flags();
                        name_found = true;
                        for row in report_rows(o.registry)? {
                            row.show = true;
                        }
                        for row in self.monthly_rows()? {
                            row.show = true;
                        }
                    }
                    "ALLSUMMARYMONTHLYANDSIZINGPERIOD" => {
                        self.source_all_summary_flags();
                        name_found = true;
                        for row in report_rows(o.registry)? {
                            row.show = true;
                        }
                        for row in self.monthly_rows()? {
                            row.show = true;
                        }
                        self.source_component_load_flags();
                    }
                    _ => {}
                }
                // Original1671-1682 always checks both fields of every occupied
                // report row, even when an earlier report branch already matched.
                for row in report_rows(o.registry)? {
                    if name.eq_ignore_ascii_case(&row.name) {
                        self.input_state.write_tabular_files = true;
                        row.show = true;
                        name_found = true;
                    }
                    if name.eq_ignore_ascii_case(&row.abbreviation) {
                        self.input_state.write_tabular_files = true;
                        row.show = true;
                        name_found = true;
                    }
                }
                for row in self
                    .input_state
                    .named_monthly
                    .as_mut()
                    .ok_or(Error::Unavailable("actual namedMonthly allocation1684"))?
                {
                    if name.eq_ignore_ascii_case(&row.title) {
                        row.show = true;
                        self.input_state.write_tabular_files = true;
                        name_found = true;
                    }
                }
                if !name_found {
                    if name.eq_ignore_ascii_case("Standard62.1Summary") {
                        o.backend.show_warning_error(&format!(
                            "{FAMILY} Field[{}]=\"Standard62.1Summary\", Report is not enabled.",
                            at + 1
                        ))?;
                        o.backend.show_continue_error("Do Zone Sizing or Do System Sizing must be enabled in SimulationControl.")?;
                    } else {
                        o.backend.show_severe_error(&format!("{FAMILY} Field[{}]=\"{name}\", invalid report name -- will not be reported.",at+1))?;
                        // Native deliberately does not set ErrorsFound here.
                    }
                }
            }
            self.create_source_predefined_monthly_reports()?;
        } else if objects > 1 {
            o.backend.show_severe_error(&format!(
                "{FAMILY}: Only one instance of this object is allowed."
            ))?;
            errors_found = true;
        }
        if errors_found {
            return Err(source_fatal_error(
                o.backend,
                &format!("{FAMILY}: Preceding errors cause termination."),
            ));
        }
        if self.display_tabular_beps
            || self.display_demand_end_use
            || self.display_source_energy_end_use_summary
            || self.display_leed_summary
        {
            self.initialize_source_beps(o)?;
        }
        Ok(())
    }
    fn monthly_rows(&mut self) -> Result<&mut [state::SourceNamedMonthly], Error> {
        self.input_state
            .named_monthly
            .as_deref_mut()
            .ok_or(Error::Unavailable("actual namedMonthly allocated rows"))
    }
    fn source_all_summary_flags(&mut self) {
        self.input_state.write_tabular_files = true;
        self.display_tabular_beps = true;
        self.display_tabular_veri_sum = true;
        self.display_tabular_comp_costs = true;
        self.display_surface_shadowing = true;
        self.display_component_sizing = true;
        self.display_demand_end_use = true;
        self.display_adaptive_comfort = true;
        self.display_source_energy_end_use_summary = true;
        self.display_life_cycle_cost_report = true;
        self.display_tariff_report = true;
        self.display_economic_result_summary = true;
        self.display_eio_summary = true;
        self.display_leed_summary = true;
        self.display_heat_emissions_summary = true;
        self.display_thermal_resilience_summary = true;
        self.display_co2_resilience_summary = true;
        self.display_visual_resilience_summary = true;
    }
    fn source_component_load_flags(&mut self) {
        self.display_zone_component_load_summary = true;
        self.display_air_loop_component_load_summary = true;
        self.display_facility_component_load_summary = true;
    }
    fn initialize_source_monthly_titles(
        &mut self,
        o: &mut SourceTabularInputOwners<'_, '_>,
    ) -> Result<(), Error> {
        use constants::{DATA_OUTPUTS_MONTHLY, MONTHLY_TITLES};
        if self.input_state.named_monthly.is_some() {
            return Err(Error::Unavailable(
                "native repeated namedMonthly.allocate2010",
            ));
        }
        self.input_state.named_monthly = Some(vec![
            state::SourceNamedMonthly::default();
            MONTHLY_TITLES.len()
        ]);
        for (row, title) in self.monthly_rows()?.iter_mut().zip(MONTHLY_TITLES) {
            row.title = title.into();
        }
        if MONTHLY_TITLES.len() != DATA_OUTPUTS_MONTHLY.len() {
            return Err(source_fatal_error(
                o.backend,
                &format!(
                    "InitializePredefinedMonthlyTitles: Number of Monthly Reports in OutputReportTabular=[{}] does not match number in DataOutputs=[{}].",
                    MONTHLY_TITLES.len(),
                    DATA_OUTPUTS_MONTHLY.len()
                ),
            ));
        }
        for (row, independent) in self.monthly_rows()?.iter().zip(DATA_OUTPUTS_MONTHLY) {
            if !row.title.eq_ignore_ascii_case(independent) {
                o.backend.show_severe_error("InitializePredefinedMonthlyTitles: Monthly Report Titles in OutputReportTabular do not match titles in DataOutput.")?;
                o.backend.show_continue_error(&format!(
                    "first mismatch at ORT [{}] =\"{}\".",
                    MONTHLY_TITLES.len(),
                    row.title
                ))?;
                o.backend
                    .show_continue_error(&format!("same location in DO =\"{independent}\"."))?;
                return Err(source_fatal_error(
                    o.backend,
                    "Preceding condition causes termination.",
                ));
            }
        }
        Ok(())
    }
    fn create_source_predefined_monthly_reports(&self) -> Result<(), Error> {
        // The actual 63 independent guards2135-2770 occur in ascending native
        // ordinal order. Every positive body starts with AddMonthlyReport; no
        // monthly count/report/fieldset effect is skipped on this boundary.
        let rows = self
            .input_state
            .named_monthly
            .as_ref()
            .ok_or(Error::Unavailable("actual namedMonthly2135 owner"))?;
        for row in rows {
            if row.show {
                return Err(Error::Unavailable(
                    "actual CreatePredefinedMonthlyReports AddMonthlyReport/AddMonthlyFieldSetInput positive child",
                ));
            }
        }
        Ok(())
    }
}
