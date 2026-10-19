//! Genuine BaseSizer::reportSizerOutput303-343 on the caller's retained owners.
use crate::heat_balance::predefined_tables::PredefinedRegistry;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::source_output_registration::SourceOutputDictionaryHost;
use crate::weather::day::WeatherEnvironmentState;
use std::io::Write;

/// Short-lived disjoint borrows, never a second sizing/static/report owner.
pub(crate) struct SourceSizerReportingOwners<'a, D: SourceOutputDictionaryHost> {
    pub(crate) registry: &'a mut PredefinedRegistry,
    pub(crate) environment: &'a mut WeatherEnvironmentState,
    pub(crate) dictionary_host: &'a D,
    pub(crate) eio: &'a mut dyn Write,
}

fn report_row(
    eio: &mut dyn Write,
    component_type: &str,
    component_name: &str,
    description: &str,
    value: f64,
) -> Result<(), Error> {
    // Existing literal no-width IOFiles R transport, including native nudges.
    // The table owner below still receives the original, unformatted f64 bits.
    let formatted = crate::heat_balance::source_round_format::round(value, 5)
        .map_err(|cause| Error::Format(format!("actual BaseSizer Format_991 .5R: {cause:?}")))?;
    writeln!(
        eio,
        " Component Sizing Information, {component_type}, {component_name}, {description}, {formatted}"
    )?;
    Ok(())
}

pub(crate) fn report_sizer_output<D: SourceOutputDictionaryHost>(
    o: &mut SourceSizerReportingOwners<'_, D>,
    component_type: &str,
    component_name: &str,
    description: &str,
    value: f64,
    user_description: Option<&str>,
    user_value: Option<f64>,
) -> Result<(), Error> {
    if o.environment.one_time_component_report_header_flag {
        o.eio.write_all(b"! <Component Sizing Information>, Component Type, Component Name, Input Field Description, Value\n")?;
        o.environment.one_time_component_report_header_flag = false;
    }
    report_row(o.eio, component_type, component_name, description, value)?;
    o.registry.add_component_sizing_real_entry(
        component_type,
        component_name,
        description,
        value,
    )?;

    if let (Some(description), Some(value)) = (user_description, user_value) {
        report_row(o.eio, component_type, component_name, description, value)?;
        o.registry.add_component_sizing_real_entry(
            component_type,
            component_name,
            description,
            value,
        )?;
    } else if user_description.is_some() || user_value.is_some() {
        // Preserve both earlier real writes; no guessed Native fatal effect.
        return Err(Error::Unavailable(
            "actual reportSizerOutput unmatched user optional ShowFatalError331",
        ));
    }

    if o.dictionary_host.sqlite_present() {
        return Err(Error::Unavailable(
            "actual reportSizerOutput addSQLiteComponentSizingRecord336",
        ));
    }
    if user_description.is_some() && user_value.is_some() {
        if o.dictionary_host.sqlite_present() {
            return Err(Error::Unavailable(
                "actual reportSizerOutput user addSQLiteComponentSizingRecord340",
            ));
        }
    }
    Ok(())
}
