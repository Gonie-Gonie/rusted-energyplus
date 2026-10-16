//! Genuine EconomicTariff219-256 on the admitted zero-tariff domain.
//! Positive tariff construction and positive CurrencyType getter remain explicit.
mod compute_tariff;
use super::error_backend::SourceErrorBackend;
use super::selected_heat_balance_input::HeatBalanceInputError as Error;
use super::selected_input_processor::{SelectedInputProcessor, SourceInputProcessorShortCut};
use crate::weather::day::WeatherGlobalState;
use std::convert::Infallible;

/// Actual monetaryUnitType three empty-string constructor members.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct SourceMonetaryUnit {
    pub(crate) code: String,
    pub(crate) txt: String,
    pub(crate) html: String,
}
/// Actual CostEstimateManagerData subset; no duplicate tabular currency owner.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SourceCostEstimateMonetary {
    // CostEstimateManager.hh actual same DataCost owner constructor flags.
    get_cost_input: bool,
    do_cost_estimate: bool,
    pub(crate) selected_monetary_unit: i32,
    pub(crate) monetary_unit: Option<Vec<SourceMonetaryUnit>>,
}
impl Default for SourceCostEstimateMonetary {
    fn default() -> Self {
        Self {
            get_cost_input: true,
            do_cost_estimate: false,
            selected_monetary_unit: 0,
            monetary_unit: None,
        }
    }
}
impl SourceCostEstimateMonetary {
    /// Native1543-1892 allocates all111 constructor values, then333 literal writes.
    fn initialize_monetary_unit(&mut self) {
        let number_monetary_units = 111;
        let rows = self
            .monetary_unit
            .insert(vec![SourceMonetaryUnit::default(); number_monetary_units]);
        rows[0].code = "USD".to_owned();
        rows[1].code = "AFN".to_owned();
        rows[2].code = "ALL".to_owned();
        rows[3].code = "ANG".to_owned();
        rows[4].code = "ARS".to_owned();
        rows[5].code = "AUD".to_owned();
        rows[6].code = "AWG".to_owned();
        rows[7].code = "AZN".to_owned();
        rows[8].code = "BAM".to_owned();
        rows[9].code = "BBD".to_owned();
        rows[10].code = "BGN".to_owned();
        rows[11].code = "BMD".to_owned();
        rows[12].code = "BND".to_owned();
        rows[13].code = "BOB".to_owned();
        rows[14].code = "BRL".to_owned();
        rows[15].code = "BSD".to_owned();
        rows[16].code = "BWP".to_owned();
        rows[17].code = "BYR".to_owned();
        rows[18].code = "BZD".to_owned();
        rows[19].code = "CAD".to_owned();
        rows[20].code = "CHF".to_owned();
        rows[21].code = "CLP".to_owned();
        rows[22].code = "CNY".to_owned();
        rows[23].code = "COP".to_owned();
        rows[24].code = "CRC".to_owned();
        rows[25].code = "CUP".to_owned();
        rows[26].code = "CZK".to_owned();
        rows[27].code = "DKK".to_owned();
        rows[28].code = "DOP".to_owned();
        rows[29].code = "EEK".to_owned();
        rows[30].code = "EGP".to_owned();
        rows[31].code = "EUR".to_owned();
        rows[32].code = "FJD".to_owned();
        rows[33].code = "GBP".to_owned();
        rows[34].code = "GHC".to_owned();
        rows[35].code = "GIP".to_owned();
        rows[36].code = "GTQ".to_owned();
        rows[37].code = "GYD".to_owned();
        rows[38].code = "HKD".to_owned();
        rows[39].code = "HNL".to_owned();
        rows[40].code = "HRK".to_owned();
        rows[41].code = "HUF".to_owned();
        rows[42].code = "IDR".to_owned();
        rows[43].code = "ILS".to_owned();
        rows[44].code = "IMP".to_owned();
        rows[45].code = "INR".to_owned();
        rows[46].code = "IRR".to_owned();
        rows[47].code = "ISK".to_owned();
        rows[48].code = "JEP".to_owned();
        rows[49].code = "JMD".to_owned();
        rows[50].code = "JPY".to_owned();
        rows[51].code = "KGS".to_owned();
        rows[52].code = "KHR".to_owned();
        rows[53].code = "KPW".to_owned();
        rows[54].code = "KRW".to_owned();
        rows[55].code = "KYD".to_owned();
        rows[56].code = "KZT".to_owned();
        rows[57].code = "LAK".to_owned();
        rows[58].code = "LBP".to_owned();
        rows[59].code = "LKR".to_owned();
        rows[60].code = "LRD".to_owned();
        rows[61].code = "LTL".to_owned();
        rows[62].code = "LVL".to_owned();
        rows[63].code = "MKD".to_owned();
        rows[64].code = "MNT".to_owned();
        rows[65].code = "MUR".to_owned();
        rows[66].code = "MXN".to_owned();
        rows[67].code = "MYR".to_owned();
        rows[68].code = "MZN".to_owned();
        rows[69].code = "NAD".to_owned();
        rows[70].code = "NGN".to_owned();
        rows[71].code = "NIO".to_owned();
        rows[72].code = "NOK".to_owned();
        rows[73].code = "NPR".to_owned();
        rows[74].code = "NZD".to_owned();
        rows[75].code = "OMR".to_owned();
        rows[76].code = "PAB".to_owned();
        rows[77].code = "PEN".to_owned();
        rows[78].code = "PHP".to_owned();
        rows[79].code = "PKR".to_owned();
        rows[80].code = "PLN".to_owned();
        rows[81].code = "PYG".to_owned();
        rows[82].code = "QAR".to_owned();
        rows[83].code = "RON".to_owned();
        rows[84].code = "RSD".to_owned();
        rows[85].code = "RUB".to_owned();
        rows[86].code = "SAR".to_owned();
        rows[87].code = "SBD".to_owned();
        rows[88].code = "SCR".to_owned();
        rows[89].code = "SEK".to_owned();
        rows[90].code = "SGD".to_owned();
        rows[91].code = "SHP".to_owned();
        rows[92].code = "SOS".to_owned();
        rows[93].code = "SRD".to_owned();
        rows[94].code = "SVC".to_owned();
        rows[95].code = "SYP".to_owned();
        rows[96].code = "THB".to_owned();
        rows[97].code = "TRL".to_owned();
        rows[98].code = "TRY".to_owned();
        rows[99].code = "TTD".to_owned();
        rows[100].code = "TVD".to_owned();
        rows[101].code = "TWD".to_owned();
        rows[102].code = "UAH".to_owned();
        rows[103].code = "UYU".to_owned();
        rows[104].code = "UZS".to_owned();
        rows[105].code = "VEF".to_owned();
        rows[106].code = "VND".to_owned();
        rows[107].code = "XCD".to_owned();
        rows[108].code = "YER".to_owned();
        rows[109].code = "ZAR".to_owned();
        rows[110].code = "ZWD".to_owned();
        rows[0].txt = "$".to_owned();
        rows[1].txt = "AFN".to_owned();
        rows[2].txt = "Lek".to_owned();
        rows[3].txt = "ANG".to_owned();
        rows[4].txt = "$".to_owned();
        rows[5].txt = "$".to_owned();
        rows[6].txt = "AWG".to_owned();
        rows[7].txt = "AZN".to_owned();
        rows[8].txt = "KM".to_owned();
        rows[9].txt = "$".to_owned();
        rows[10].txt = "BGN".to_owned();
        rows[11].txt = "$".to_owned();
        rows[12].txt = "$".to_owned();
        rows[13].txt = "$b".to_owned();
        rows[14].txt = "R$".to_owned();
        rows[15].txt = "$".to_owned();
        rows[16].txt = "P".to_owned();
        rows[17].txt = "p.".to_owned();
        rows[18].txt = "BZ$".to_owned();
        rows[19].txt = "$".to_owned();
        rows[20].txt = "CHF".to_owned();
        rows[21].txt = "$".to_owned();
        rows[22].txt = "CNY".to_owned();
        rows[23].txt = "$".to_owned();
        rows[24].txt = "CRC".to_owned();
        rows[25].txt = "CUP".to_owned();
        rows[26].txt = "CZK".to_owned();
        rows[27].txt = "kr".to_owned();
        rows[28].txt = "RD$".to_owned();
        rows[29].txt = "kr".to_owned();
        rows[30].txt = "£".to_owned();
        rows[31].txt = "EUR".to_owned();
        rows[32].txt = "$".to_owned();
        rows[33].txt = "£".to_owned();
        rows[34].txt = "¢".to_owned();
        rows[35].txt = "£".to_owned();
        rows[36].txt = "Q".to_owned();
        rows[37].txt = "$".to_owned();
        rows[38].txt = "HK$".to_owned();
        rows[39].txt = "L".to_owned();
        rows[40].txt = "kn".to_owned();
        rows[41].txt = "Ft".to_owned();
        rows[42].txt = "Rp".to_owned();
        rows[43].txt = "ILS".to_owned();
        rows[44].txt = "£".to_owned();
        rows[45].txt = "INR".to_owned();
        rows[46].txt = "IRR".to_owned();
        rows[47].txt = "kr".to_owned();
        rows[48].txt = "£".to_owned();
        rows[49].txt = "J$".to_owned();
        rows[50].txt = "¥".to_owned();
        rows[51].txt = "KGS".to_owned();
        rows[52].txt = "KHR".to_owned();
        rows[53].txt = "KPW".to_owned();
        rows[54].txt = "KRW".to_owned();
        rows[55].txt = "$".to_owned();
        rows[56].txt = "KZT".to_owned();
        rows[57].txt = "LAK".to_owned();
        rows[58].txt = "£".to_owned();
        rows[59].txt = "LKR".to_owned();
        rows[60].txt = "$".to_owned();
        rows[61].txt = "Lt".to_owned();
        rows[62].txt = "Ls".to_owned();
        rows[63].txt = "MKD".to_owned();
        rows[64].txt = "MNT".to_owned();
        rows[65].txt = "MUR".to_owned();
        rows[66].txt = "$".to_owned();
        rows[67].txt = "RM".to_owned();
        rows[68].txt = "MT".to_owned();
        rows[69].txt = "$".to_owned();
        rows[70].txt = "NGN".to_owned();
        rows[71].txt = "C$".to_owned();
        rows[72].txt = "kr".to_owned();
        rows[73].txt = "NPR".to_owned();
        rows[74].txt = "$".to_owned();
        rows[75].txt = "OMR".to_owned();
        rows[76].txt = "B/.".to_owned();
        rows[77].txt = "S/.".to_owned();
        rows[78].txt = "Php".to_owned();
        rows[79].txt = "PKR".to_owned();
        rows[80].txt = "PLN".to_owned();
        rows[81].txt = "Gs".to_owned();
        rows[82].txt = "QAR".to_owned();
        rows[83].txt = "lei".to_owned();
        rows[84].txt = "RSD".to_owned();
        rows[85].txt = "RUB".to_owned();
        rows[86].txt = "SAR".to_owned();
        rows[87].txt = "$".to_owned();
        rows[88].txt = "SCR".to_owned();
        rows[89].txt = "kr".to_owned();
        rows[90].txt = "$".to_owned();
        rows[91].txt = "£".to_owned();
        rows[92].txt = "S".to_owned();
        rows[93].txt = "$".to_owned();
        rows[94].txt = "$".to_owned();
        rows[95].txt = "£".to_owned();
        rows[96].txt = "THB".to_owned();
        rows[97].txt = "TRL".to_owned();
        rows[98].txt = "YTL".to_owned();
        rows[99].txt = "TT$".to_owned();
        rows[100].txt = "$".to_owned();
        rows[101].txt = "NT$".to_owned();
        rows[102].txt = "UAH".to_owned();
        rows[103].txt = "$U".to_owned();
        rows[104].txt = "UZS".to_owned();
        rows[105].txt = "Bs".to_owned();
        rows[106].txt = "VND".to_owned();
        rows[107].txt = "$".to_owned();
        rows[108].txt = "YER".to_owned();
        rows[109].txt = "R".to_owned();
        rows[110].txt = "Z$".to_owned();
        rows[0].html = "$".to_owned();
        rows[1].html = "&#x060b;".to_owned();
        rows[2].html = "Lek".to_owned();
        rows[3].html = "&#x0192;".to_owned();
        rows[4].html = "$".to_owned();
        rows[5].html = "$".to_owned();
        rows[6].html = "&#x0192;".to_owned();
        rows[7].html = "&#x043c;&#x0430;&#x043d;".to_owned();
        rows[8].html = "KM".to_owned();
        rows[9].html = "$".to_owned();
        rows[10].html = "&#x043b;&#x0432;".to_owned();
        rows[11].html = "$".to_owned();
        rows[12].html = "$".to_owned();
        rows[13].html = "$b".to_owned();
        rows[14].html = "R$".to_owned();
        rows[15].html = "$".to_owned();
        rows[16].html = "P".to_owned();
        rows[17].html = "p.".to_owned();
        rows[18].html = "BZ$".to_owned();
        rows[19].html = "$".to_owned();
        rows[20].html = "CHF".to_owned();
        rows[21].html = "$".to_owned();
        rows[22].html = "&#x5143;".to_owned();
        rows[23].html = "$".to_owned();
        rows[24].html = "&#x20a1;".to_owned();
        rows[25].html = "&#x20b1;".to_owned();
        rows[26].html = "&#x004b;&#x010d;".to_owned();
        rows[27].html = "kr".to_owned();
        rows[28].html = "RD$".to_owned();
        rows[29].html = "kr".to_owned();
        rows[30].html = "£".to_owned();
        rows[31].html = "&#x20ac;".to_owned();
        rows[32].html = "$".to_owned();
        rows[33].html = "£".to_owned();
        rows[34].html = "¢".to_owned();
        rows[35].html = "£".to_owned();
        rows[36].html = "Q".to_owned();
        rows[37].html = "$".to_owned();
        rows[38].html = "HK$".to_owned();
        rows[39].html = "L".to_owned();
        rows[40].html = "kn".to_owned();
        rows[41].html = "Ft".to_owned();
        rows[42].html = "Rp".to_owned();
        rows[43].html = "&#x20aa;".to_owned();
        rows[44].html = "£".to_owned();
        rows[45].html = "&#x20a8;".to_owned();
        rows[46].html = "&#xfdfc;".to_owned();
        rows[47].html = "kr".to_owned();
        rows[48].html = "£".to_owned();
        rows[49].html = "J$".to_owned();
        rows[50].html = "¥".to_owned();
        rows[51].html = "&#x043b;&#x0432;".to_owned();
        rows[52].html = "&#x17db;".to_owned();
        rows[53].html = "&#x20a9;".to_owned();
        rows[54].html = "&#x20a9;".to_owned();
        rows[55].html = "$".to_owned();
        rows[56].html = "&#x043b;&#x0432;".to_owned();
        rows[57].html = "&#x20ad;".to_owned();
        rows[58].html = "£".to_owned();
        rows[59].html = "&#x20a8;".to_owned();
        rows[60].html = "$".to_owned();
        rows[61].html = "Lt".to_owned();
        rows[62].html = "Ls".to_owned();
        rows[63].html = "&#x0434;&#x0435;&#x043d;".to_owned();
        rows[64].html = "&#x20ae;".to_owned();
        rows[65].html = "&#x20a8;".to_owned();
        rows[66].html = "$".to_owned();
        rows[67].html = "RM".to_owned();
        rows[68].html = "MT".to_owned();
        rows[69].html = "$".to_owned();
        rows[70].html = "&#x20a6;".to_owned();
        rows[71].html = "C$".to_owned();
        rows[72].html = "kr".to_owned();
        rows[73].html = "&#x20a8;".to_owned();
        rows[74].html = "$".to_owned();
        rows[75].html = "&#xfdfc;".to_owned();
        rows[76].html = "B/.".to_owned();
        rows[77].html = "S/.".to_owned();
        rows[78].html = "Php".to_owned();
        rows[79].html = "&#x20a8;".to_owned();
        rows[80].html = "&#x007a;&#x0142;".to_owned();
        rows[81].html = "Gs".to_owned();
        rows[82].html = "&#xfdfc;".to_owned();
        rows[83].html = "lei".to_owned();
        rows[84].html = "&#x0414;&#x0438;&#x043d;&#x002e;".to_owned();
        rows[85].html = "&#x0440;&#x0443;&#x0431;".to_owned();
        rows[86].html = "&#xfdfc;".to_owned();
        rows[87].html = "$".to_owned();
        rows[88].html = "&#x20a8;".to_owned();
        rows[89].html = "kr".to_owned();
        rows[90].html = "$".to_owned();
        rows[91].html = "£".to_owned();
        rows[92].html = "S".to_owned();
        rows[93].html = "$".to_owned();
        rows[94].html = "$".to_owned();
        rows[95].html = "£".to_owned();
        rows[96].html = "&#x0e3f;".to_owned();
        rows[97].html = "&#x20a4;".to_owned();
        rows[98].html = "YTL".to_owned();
        rows[99].html = "TT$".to_owned();
        rows[100].html = "$".to_owned();
        rows[101].html = "NT$".to_owned();
        rows[102].html = "&#x20b4;".to_owned();
        rows[103].html = "$U".to_owned();
        rows[104].html = "&#x043b;&#x0432;".to_owned();
        rows[105].html = "Bs".to_owned();
        rows[106].html = "&#x20ab;".to_owned();
        rows[107].html = "$".to_owned();
        rows[108].html = "&#xfdfc;".to_owned();
        rows[109].html = "R".to_owned();
        rows[110].html = "Z$".to_owned();
    }
    fn get_input_currency_type(
        &mut self,
        input: &SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
        _errors_found: &mut bool,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        shortcut.current_module_object = "CurrencyType".to_owned();
        self.initialize_monetary_unit();
        let count = input.get_num_objects_found(&shortcut.current_module_object);
        self.selected_monetary_unit = 0;
        if count == 0 {
            self.selected_monetary_unit = 1;
        } else if count == 1 {
            // Native1388 first real shared getter, before its array/symbol lookup.
            return Err(Error::Unavailable("actual CurrencyType getObjectItem1388"));
        } else {
            backend.show_warning_error(&format!(
                "GetInputEconomicsCurrencyType: {} Only one instance of this object is allowed. USD will be used.",
                shortcut.current_module_object
            ))?;
            self.selected_monetary_unit = 1;
        }
        Ok(())
    }
}
/// Exact reached EconomicTariffData ctor subset, including EPVector allocation state.
/// Unrepresented TariffType records cannot be fabricated by the zero-only allocator.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SourceEconomicTariff {
    pub(crate) update_get_input: bool,
    /// Actual EconomicTariff.hh644 ctor0; positive econVar records stay unported.
    pub(crate) number_economics_variables: i32,
    pub(crate) number_tariffs: i32,
    pub(crate) tariff_allocated: bool,
    pub(crate) tariffs: Vec<Infallible>,
}
impl Default for SourceEconomicTariff {
    fn default() -> Self {
        Self {
            update_get_input: true,
            number_economics_variables: 0,
            number_tariffs: 0,
            tariff_allocated: false,
            tariffs: Vec::new(),
        }
    }
}
impl SourceEconomicTariff {
    fn get_input_tariff(&mut self, input: &SelectedInputProcessor) -> Result<(), Error> {
        // Native297 is LOCAL; it must not overwrite dataIPShortCut's module name.
        let current_module_object = "UtilityCost:Tariff";
        self.number_tariffs = i32::try_from(input.get_num_objects_found(current_module_object))
            .map_err(|_| Error::Unavailable("actual numTariff signed integer domain"))?;
        if self.number_tariffs > 0 {
            return Err(Error::Unavailable(
                "actual positive TariffType constructor at tariff.allocate299",
            ));
        }
        // Exact EPVector.allocate(0): allocated=true, resize0, empty fill.
        self.tariff_allocated = true;
        self.tariffs.clear();
        Ok(())
    }
    pub(crate) fn update_utility_bills(
        &mut self,
        cost: &mut SourceCostEstimateMonetary,
        input: &SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
        clock: &WeatherGlobalState,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        if self.update_get_input {
            let mut errors_found = false;
            self.get_input_tariff(input)?;
            cost.get_input_currency_type(input, shortcut, &mut errors_found, backend)?;
            if self.number_tariffs >= 1 {
                return Err(Error::Unavailable(
                    "actual positive CreateCategoryNativeVariables236",
                ));
            }
            self.update_get_input = false;
            if errors_found {
                return Err(
                    match backend
                        .show_fatal_error("UpdateUtilityBills: Preceding errors cause termination.")
                    {
                        Ok(never) => match never {},
                        Err(e) => Error::Fatal(e),
                    },
                );
            }
        }
        if clock.do_output_reporting && clock.kind_of_sim == 3 {
            // Native GatherForEconomics2553 reads numTariff before any record/clock.
            if self.number_tariffs >= 1 {
                return Err(Error::Unavailable(
                    "actual GatherForEconomics first tariff row2555",
                ));
            }
        }
        Ok(())
    }
}

impl SourceCostEstimateMonetary {
    /// Actual SimCostEstimate110-136/GetCostEstimateInput161-170 zero-item path.
    /// The same cost/currency owner is retained; positive line allocation remains
    /// an explicit boundary after its actual DoCostEstimate=true write.
    pub(crate) fn simulate_source_cost_estimate(
        &mut self,
        input: &SelectedInputProcessor,
        kickoff_simulation: bool,
    ) -> Result<(), Error> {
        if self.get_cost_input {
            let count = input.get_num_objects_found("ComponentCost:LineItem");
            let num_line_items = i32::try_from(count).map_err(|_| {
                Error::Unavailable(
                    "actual ComponentCost LineItem count outside defined Native int161",
                )
            })?;
            if num_line_items == 0 {
                self.do_cost_estimate = false;
            } else {
                self.do_cost_estimate = true;
                return Err(Error::Unavailable(
                    "actual positive ComponentCost LineItem allocation170 and getter",
                ));
            }
            self.get_cost_input = false;
        }
        if kickoff_simulation {
            return Ok(());
        }
        if self.do_cost_estimate {
            return Err(Error::Unavailable("actual positive CalcCostEstimate134"));
        }
        Ok(())
    }
}

impl SourceCostEstimateMonetary {
    // Actual DoCostEstimate operand read by WriteTableOfContents3278.
    pub(crate) fn source_do_cost_estimate(&self) -> &bool { &self.do_cost_estimate }
}
