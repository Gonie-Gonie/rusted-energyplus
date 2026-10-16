//! Literal OutputProcessor4689-4825; map ordering and live Next links are retained.
use super::{
    SourceDictionaryVariable, SourceOutputRegistration, SourceOutputStoreType, SourceOutputUnits,
    TimeStepKind,
};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::ordinary_output_control::{
    SourceOutputControl, SourceResultsFrameworkOptions,
};
use crate::heat_balance::source_initialization::{
    SourceInitializationError as Error, SourceInitializationFiles,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub(super) enum SourceReportVdd {
    No = 0,
    Yes = 1,
    Idf = 2,
}
fn unit(units: SourceOutputUnits) -> Result<&'static str, Error> {
    if units == SourceOutputUnits::Invalid {
        return Err(Error::Association(
            "actual ProduceRDDMDD defined unitNames index",
        ));
    }
    Ok(units.symbol())
}
fn variable(
    kind: SourceReportVdd,
    row: &mut SourceDictionaryVariable,
    unit: &str,
    linked: bool,
    files: &mut SourceInitializationFiles,
    results: &mut SourceResultsFrameworkOptions,
) -> Result<(), Error> {
    // First row uses the local array; linked rows use the distinct global array.
    let time = match row.time_step_type {
        TimeStepKind::Zone => "Zone",
        TimeStepKind::System => {
            if linked {
                "System"
            } else {
                "HVAC"
            }
        }
    };
    let store = match row.store_type {
        SourceOutputStoreType::Average => "Average",
        SourceOutputStoreType::Sum => "Sum",
    };
    match kind {
        SourceReportVdd::Yes => {
            files.write_source_rdd(format!("{time},{store},{} [{unit}]\n", row.name).as_bytes())?
        }
        SourceReportVdd::Idf => files.write_source_rdd(
            format!(
                "Output:Variable,*,{},hourly; !- {time} {store} [{unit}]\n",
                row.name
            )
            .as_bytes(),
        )?,
        SourceReportVdd::No => {
            return Err(Error::Association("actual ProduceRDDMDD variable branch"));
        }
    }
    results
        .report_data_dictionary
        .push(format!("{time},{store},{} [{unit}]", row.name));
    row.reported_on_dictionary_file = true;
    Ok(())
}
impl SourceOutputRegistration {
    #[expect(
        clippy::too_many_arguments,
        reason = "Actual ProduceRDDMDD shared owners and General output arguments"
    )]
    pub(crate) fn produce_source_dictionaries(
        &mut self,
        do_report: bool,
        option1: &str,
        option2: &str,
        sort_by_name: &mut bool,
        control: &SourceOutputControl,
        results: &mut SourceResultsFrameworkOptions,
        files: &mut SourceInitializationFiles,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        if do_report {
            self.produce_report_vdd = SourceReportVdd::Yes;
            if option1 == "IDF" {
                self.produce_report_vdd = SourceReportVdd::Idf;
            }
            if !option2.is_empty() {
                if option2.eq_ignore_ascii_case("Name")
                    || option2.eq_ignore_ascii_case("AscendingName")
                {
                    *sort_by_name = true;
                }
            }
        }
        files.ensure_source_rdd_open(control.rdd, backend)?;
        files.ensure_source_mdd_open(control.mdd, backend)?;
        match self.produce_report_vdd {
            SourceReportVdd::Yes => {
                files.write_source_rdd(
                    format!(
                        "Program Version,{},{}\n",
                        backend.state.version_string, backend.state.idd_version_string
                    )
                    .as_bytes(),
                )?;
                files.write_source_rdd(
                    b"Var Type (reported time step),Var Report Type,Variable Name [Units]\n",
                )?;
                files.write_source_mdd(
                    format!(
                        "Program Version,{},{}\n",
                        backend.state.version_string, backend.state.idd_version_string
                    )
                    .as_bytes(),
                )?;
                files.write_source_mdd(
                    b"Var Type (reported time step),Var Report Type,Variable Name [Units]\n",
                )?;
            }
            SourceReportVdd::Idf => {
                files.write_source_rdd(
                    format!(
                        "! Program Version,{},{}\n",
                        backend.state.version_string, backend.state.idd_version_string
                    )
                    .as_bytes(),
                )?;
                files.write_source_rdd(b"! Output:Variable Objects (applicable to this run)\n")?;
                files.write_source_mdd(
                    format!(
                        "! Program Version,{},{}\n",
                        backend.state.version_string, backend.state.idd_version_string
                    )
                    .as_bytes(),
                )?;
                files.write_source_mdd(b"! Output:Meter Objects (applicable to this run)\n")?;
            }
            SourceReportVdd::No => {}
        }
        if self.produce_report_vdd == SourceReportVdd::Yes
            || self.produce_report_vdd == SourceReportVdd::Idf
        {
            let mut by_name = self.dictionary_by_name.values();
            let mut array_index = 0;
            loop {
                let index = if *sort_by_name {
                    match by_name.next() {
                        Some(index) => *index,
                        None => break,
                    }
                } else {
                    if array_index == self.dictionary_variables.len() {
                        break;
                    }
                    let index = array_index;
                    array_index += 1;
                    index
                };
                let row = self
                    .dictionary_variables
                    .get_mut(index)
                    .ok_or(Error::Association("actual ProduceRDDMDD DD map index"))?;
                if row.reported_on_dictionary_file {
                    continue;
                }
                let units = unit(row.units)?;
                variable(self.produce_report_vdd, row, units, false, files, results)?;
                if *sort_by_name {
                    let mut next = row.next;
                    while let Some(index) = next {
                        let row = self
                            .dictionary_variables
                            .get_mut(index)
                            .ok_or(Error::Association("actual ProduceRDDMDD DD Next index"))?;
                        // Native does not re-read units or test the linked reported flag.
                        variable(self.produce_report_vdd, row, units, true, files, results)?;
                        next = row.next;
                    }
                }
            }
        }
        files.close_source_rdd();
        let mut by_name = self.meters.meter_map.values();
        let mut array_index = 0;
        while array_index < self.meters.meters.len() {
            let Some(mapped_index) = by_name.next() else {
                break;
            };
            let index = if *sort_by_name {
                *mapped_index
            } else {
                array_index
            };
            let row = self
                .meters
                .meters
                .get(index)
                .ok_or(Error::Association("actual ProduceRDDMDD meter map index"))?;
            let units = unit(row.units)?;
            match self.produce_report_vdd {
                SourceReportVdd::Yes => {
                    files.write_source_mdd(
                        format!("Zone,Meter,{} [{units}]\n", row.name).as_bytes(),
                    )?;
                    results
                        .meter_data_dictionary
                        .push(format!("Zone,Meter,{} [{units}]", row.name));
                }
                SourceReportVdd::Idf => {
                    files.write_source_mdd(
                        format!("Output:Meter,{},hourly; !- [{units}]\n", row.name).as_bytes(),
                    )?;
                    results
                        .meter_data_dictionary
                        .push(format!("Output:Meter,{} [{units}]", row.name));
                    files.write_source_mdd(
                        format!(
                            "Output:Meter:Cumulative,{},hourly; !- [{units}]\n",
                            row.name
                        )
                        .as_bytes(),
                    )?;
                    results
                        .meter_data_dictionary
                        .push(format!("Output:Meter:Cumulative,{} [{units}]", row.name));
                }
                SourceReportVdd::No => {}
            }
            array_index += 1;
        }
        files.close_source_mdd();
        Ok(())
    }
}
