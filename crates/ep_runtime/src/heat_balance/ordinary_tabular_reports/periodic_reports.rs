//! Actual WriteMonthlyTables7056, TimeBin7500 and Annual::WriteAnnualTables643.
//! These genuine empty-domain bodies read retained counts/vectors, not raw IP.
use super::{Error, SourceTabularBody, SourceTabularReports};
use std::collections::HashMap;

impl SourceTabularReports {
    pub(crate) fn write_source_monthly_tables(&mut self) -> Result<(), Error> {
        // Native7078 Array1D_string rowHead(16), before all passes/count guards.
        let mut row_head = vec![Vec::<u8>::new(); 16];
        const STORED_MAX_VAL: f64 = f64::MAX;
        const STORED_MIN_VAL: f64 = f64::MIN;
        for (row, label) in row_head.iter_mut().zip(MONTHLY_ROWS) {
            *row = label.as_bytes().to_vec();
        }
        // Original7130 unordered_map<AggType,string>, including original case
        // and spaces. No map traversal order affects any report or state.
        let agg_string: HashMap<i32, Vec<u8>> = AGGREGATION
            .iter()
            .enumerate()
            .map(|(number, (_name, text))| (number as i32, text.as_bytes().to_vec()))
            .collect();
        let very_large = 1.0E280;
        let very_small = -1.0E280;
        for current_style in &self.input_state.tabular_report_passes {
            // Native7148-7164 assignments occur even with MonthlyInputCount0.
            let (energy_units_string, energy_units_conversion_factor) =
                match current_style.units_style {
                    0 => (b"J".to_vec(), 1.0),
                    1 => (b"kWh".to_vec(), 1.0 / 3600000.0),
                    2 => (b"MJ".to_vec(), 1.0 / 1000000.0),
                    3 => (b"GJ".to_vec(), 1.0 / 1000000000.0),
                    _ => (b"J".to_vec(), 1.0),
                };
            for _input in 1..=self.input_state.monthly_input_count {
                // MonthlyInput is a distinct Native record array. Existing
                // MonthlyTables/Columns empty allocations are not that owner.
                return Err(Error::Unavailable(
                    "actual WriteMonthlyTables7169 MonthlyInput(iInput).showDigits first record read",
                ));
            }
            drop((energy_units_string, energy_units_conversion_factor));
        }
        drop((
            row_head,
            agg_string,
            very_large,
            very_small,
            STORED_MAX_VAL,
            STORED_MIN_VAL,
        ));
        Ok(())
    }

    pub(crate) fn write_source_time_bin_tables(&mut self) -> Result<(), Error> {
        // Actual7522-7527 string/stat allocations precede the style loop. The
        // numeric stat width is allocated/unwritten until a positive table body.
        let mut row_head = vec![Vec::<u8>::new(); 39];
        let column_head_stat = vec![Vec::<u8>::new(); 1];
        let column_width_stat: Vec<Option<i32>> = vec![None; 1];
        let row_head_stat = vec![Vec::<u8>::new(); 6];
        let table_body_stat = SourceTabularBody::allocate(1, 6)?;
        for _current_style in &self.input_state.tabular_report_passes {
            for (row, label) in row_head.iter_mut().zip(BIN_ROWS) {
                *row = label.as_bytes().to_vec();
            }
            for input in 1..=self.input_state.output_table_binned_count {
                let index = input
                    .checked_sub(1)
                    .and_then(|v| usize::try_from(v).ok())
                    .ok_or(Error::Unavailable(
                        "actual TimeBin7578 OutputTableBinned ordinal",
                    ))?;
                let actual = self
                    .input_state
                    .output_table_binned
                    .as_ref()
                    .and_then(|v| v.get(index))
                    .ok_or(Error::Unavailable(
                        "actual TimeBin7578 OutputTableBinned(iInObj).resIndex row",
                    ))?;
                let _first_report = actual.result_index;
                return Err(Error::Unavailable(
                    "actual WriteTimeBinTables7579 varOrMeter / Constant::unitNames formatting first operation",
                ));
            }
        }
        drop((
            row_head,
            column_head_stat,
            column_width_stat,
            row_head_stat,
            table_body_stat,
        ));
        Ok(())
    }

    pub(crate) fn write_source_annual_tables(&mut self) -> Result<(), Error> {
        // Native643-655 has no preloop report-array allocation or flag guard.
        // Reborrow the existing single AnnualData.annualTables projection.
        for _current_style in &self.input_state.tabular_report_passes {
            for _annual_table in &mut self.input_state.annual_tables {
                return Err(Error::Unavailable(
                    "actual AnnualTable::writeTable677 setupAggString1059 local vector allocation first operation",
                ));
            }
        }
        Ok(())
    }
}
const MONTHLY_ROWS: &[&str] = &[
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
    "",
    "Annual Sum or Average",
    "Minimum of Months",
    "Maximum of Months",
];
const BIN_ROWS: &[&str] = &[
    "Interval Start",
    "Interval End",
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
    "12:01 to  1:00 am",
    " 1:01 to  2:00 am",
    " 2:01 to  3:00 am",
    " 3:01 to  4:00 am",
    " 4:01 to  5:00 am",
    " 5:01 to  6:00 am",
    " 6:01 to  7:00 am",
    " 7:01 to  8:00 am",
    " 8:01 to  9:00 am",
    " 9:01 to 10:00 am",
    "10:01 to 11:00 am",
    "11:01 to 12:00 pm",
    "12:01 to  1:00 pm",
    " 1:01 to  2:00 pm",
    " 2:01 to  3:00 pm",
    " 3:01 to  4:00 pm",
    " 4:01 to  5:00 pm",
    " 5:01 to  6:00 pm",
    " 6:01 to  7:00 pm",
    " 7:01 to  8:00 pm",
    " 8:01 to  9:00 pm",
    " 9:01 to 10:00 pm",
    "10:01 to 11:00 pm",
    "11:01 to 12:00 am",
    "Total",
];
const AGGREGATION: &[(&str, &str)] = &[
    ("SumOrAvg", ""),
    ("Maximum", " Maximum "),
    ("Minimum", " MINIMUM "),
    ("ValueWhenMaxMin", " AT MAX/MIN "),
    ("HoursZero", " HOURS ZERO "),
    ("HoursNonZero", " HOURS NON-ZERO "),
    ("HoursPositive", " HOURS POSITIVE "),
    ("HoursNonPositive", " HOURS NON-POSITIVE "),
    ("HoursNegative", " HOURS NEGATIVE "),
    ("HoursNonNegative", " HOURS NON-NEGATIVE "),
    ("SumOrAverageHoursShown", " FOR HOURS SHOWN "),
    ("MaximumDuringHoursShown", " MAX FOR HOURS SHOWN "),
    ("MinimumDuringHoursShown", " MIN FOR HOURS SHOWN "),
];
