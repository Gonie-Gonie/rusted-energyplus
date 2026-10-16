//! Actual IOFiles::OutputControl and ResultsFramework option owners.
//! Source shd remains the existing sole SourceShadowingInput.output_control_shd
//! cell; this partial aggregate does not duplicate or synchronize that field.
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_input_processor::{SelectedInputProcessor, SourceInputFamily};
/// All actual IOFiles.hh559-590 booleans except the already retained shd cell.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceOutputControl {
    pub(crate) csv: bool,
    pub(crate) mtr: bool,
    pub(crate) eso: bool,
    pub(crate) eio: bool,
    pub(crate) audit: bool,
    pub(crate) spsz: bool,
    pub(crate) zsz: bool,
    pub(crate) ssz: bool,
    pub(crate) psz: bool,
    pub(crate) dxf: bool,
    pub(crate) bnd: bool,
    pub(crate) rdd: bool,
    pub(crate) mdd: bool,
    pub(crate) mtd: bool,
    pub(crate) end: bool,
    pub(crate) dfs: bool,
    pub(crate) delightin: bool,
    pub(crate) delighteldmp: bool,
    pub(crate) delightdfdmp: bool,
    pub(crate) edd: bool,
    pub(crate) dbg: bool,
    pub(crate) perflog: bool,
    pub(crate) sln: bool,
    pub(crate) sci: bool,
    pub(crate) wrl: bool,
    pub(crate) screen: bool,
    pub(crate) tarcog: bool,
    pub(crate) extshd: bool,
    pub(crate) json: bool,
    pub(crate) tabular: bool,
    pub(crate) sqlite: bool,
}
impl Default for SourceOutputControl {
    fn default() -> Self {
        Self {
            csv: false,
            mtr: true,
            eso: true,
            eio: true,
            audit: true,
            spsz: true,
            zsz: true,
            ssz: true,
            psz: true,
            dxf: true,
            bnd: true,
            rdd: true,
            mdd: true,
            mtd: true,
            end: true,
            dfs: true,
            delightin: true,
            delighteldmp: true,
            delightdfdmp: true,
            edd: true,
            dbg: true,
            perflog: true,
            sln: true,
            sci: true,
            wrl: true,
            screen: true,
            tarcog: true,
            extshd: true,
            json: true,
            tabular: true,
            sqlite: true,
        }
    }
}
/// Genuine selected members of the same ResultsFramework aggregate. The real
/// outputVariables vector used by SourceOutputDictionaryHost remains its sole
/// existing owner; these options contain no second vector or scalar registry.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceResultsFrameworkOptions {
    /// Same actual ResultsFramework.hh391 Meters frames, not gated by JSON output.
    pub(crate) meter_frames: [super::source_output_registration::SourceMeterDataFrame; 7],
    pub(crate) time_series_enabled: bool,
    pub(crate) time_series_and_tabular_enabled: bool,
    pub(crate) output_json: bool,
    pub(crate) output_cbor: bool,
    pub(crate) output_msg_pack: bool,
    pub(crate) rewrite_timestamp: bool,
}
impl Default for SourceResultsFrameworkOptions {
    fn default() -> Self {
        Self {
            meter_frames: [
                "Each Call",
                "TimeStep",
                "Hourly",
                "Daily",
                "Monthly",
                "RunPeriod",
                "Yearly",
            ]
            .map(super::source_output_registration::SourceMeterDataFrame::new),
            time_series_enabled: false,
            time_series_and_tabular_enabled: false,
            output_json: false,
            output_cbor: false,
            output_msg_pack: false,
            rewrite_timestamp: true,
        }
    }
}
fn first_actual_object(input: &SelectedInputProcessor, family: &str) -> Option<String> {
    match input.find_exact_family(family) {
        Some(SourceInputFamily::Objects(rows)) => rows.first().map(|r| r.object_name.clone()),
        Some(SourceInputFamily::Null) | None => None,
    }
}
impl SourceOutputControl {
    /// Real SimulationManager183 caller must invoke this before Results184 and
    /// debug/output opening. Absence/null/empty follows actual epJSON traversal;
    /// positive fields stop after real first-object markObjectAsUsed, before its
    /// first unported getter. No typed-model absence or readiness bool is used.
    pub(crate) fn get_source_output_control_input(
        &mut self,
        input: &mut SelectedInputProcessor,
    ) -> Result<(), Error> {
        if let Some(name) = first_actual_object(input, "OutputControl:Files") {
            input.mark_object_as_used("OutputControl:Files", &name);
            return Err(Error::Unavailable(
                "actual IOFiles362 find_input(output_csv) and positive OutputControl:Files fields",
            ));
        }
        if let Some(name) = first_actual_object(input, "OutputControl:Timestamp") {
            input.mark_object_as_used("OutputControl:Timestamp", &name);
            return Err(Error::Unavailable(
                "actual IOFiles467 positive timestamp field/ResultsFramework setters",
            ));
        }
        Ok(())
    }
    /// Native evaluates JSON even when htmlTabular is true; do not replace
    /// these local evaluations with a short-circuit whole return expression.
    pub(crate) fn source_write_tabular(&self, results: &SourceResultsFrameworkOptions) -> bool {
        let html_tabular = self.tabular;
        let json_tabular = self.json && results.time_series_and_tabular_enabled;
        let sqlite_tabular = self.sqlite;
        html_tabular || json_tabular || sqlite_tabular
    }
}
impl SourceResultsFrameworkOptions {
    /// Real SimulationManager184 child. Positive JSON getter is not replaced
    /// by an assumed output mode; preceding CSV writes/count query are retained.
    pub(crate) fn setup_source_output_options(
        &mut self,
        files: &SourceOutputControl,
        input: &SelectedInputProcessor,
    ) -> Result<(), Error> {
        if files.csv {
            self.time_series_enabled = true;
            self.time_series_and_tabular_enabled = true;
        }
        if !files.json {
            return Ok(());
        }
        let count = input.get_num_objects_found("Output:JSON");
        if count == 0 {
            return Ok(());
        }
        let _alphas = [
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
        ];
        let _numbers: [Option<f64>; 2] = [None; 2];
        Err(Error::Unavailable(
            "actual ResultsFramework887 positive Output:JSON getObjectItem before option writes",
        ))
    }
}
