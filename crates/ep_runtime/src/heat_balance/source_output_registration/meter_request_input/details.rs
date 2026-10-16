//! Native multiplierString2217/ReportMeterDetails2224/DetermineIndexGroup2916.
use super::super::SourceOutVarReal;
use super::super::SourceOutputUnits;
use super::super::meters::{
    SourceMeter, SourceMeterEndUse, SourceMeterGroup, SourceMeterResource, SourceMeterType,
};
use super::{Error, SourceMeterReportingHost, SourceOutputRegistration};
fn multiplier_string(variable: &SourceOutVarReal) -> Result<String, Error> {
    if variable.zone_multiplier == 1 && variable.zone_list_multiplier == 1 {
        return Ok(String::new());
    }
    let product = variable
        .zone_multiplier
        .checked_mul(variable.zone_list_multiplier)
        .ok_or(Error::Unavailable(
            "defined native multiplierString integer product",
        ))?;
    Ok(format!(
        " * {product}  (Zone Multiplier = {}, Zone List Multiplier = {})",
        variable.zone_multiplier, variable.zone_list_multiplier
    ))
}
pub(super) fn determine_index_group(meter: &SourceMeter) -> Result<String, Error> {
    let mut group = if meter.group != SourceMeterGroup::Invalid {
        meter.group.source_name().map_err(Error::Output)?.into()
    } else {
        String::from("Facility")
    };
    if meter.resource != SourceMeterResource::Invalid {
        group += &format!(":{}", meter.resource.source_name().map_err(Error::Output)?);
    }
    if meter.end_use != SourceMeterEndUse::Invalid {
        group += &format!(":{}", meter.end_use.name().map_err(Error::Output)?);
    }
    if !meter.end_use_subcategory.is_empty() {
        group += &format!(":{}", meter.end_use_subcategory);
    }
    Ok(group)
}
pub(super) fn report_meter_details(
    registration: &SourceOutputRegistration,
    host: &mut impl SourceMeterReportingHost,
) -> Result<(), Error> {
    for variable in &registration.out_vars {
        if variable.meter_numbers.is_empty() {
            continue;
        }
        if variable.units == SourceOutputUnits::Invalid {
            return Err(Error::Unavailable(
                "defined native metered-variable unitNames index",
            ));
        }
        host.write_mtd(
            format!(
                "\n Meters for {},{} [{}]{}\n",
                variable.report_id,
                variable.key_colon_name,
                variable.units.symbol(),
                multiplier_string(variable)?
            )
            .as_bytes(),
        )?;
        for &meter_number in &variable.meter_numbers {
            let at = usize::try_from(meter_number)
                .map_err(|_| Error::Unavailable("defined native variable meter index"))?;
            let meter = registration
                .meters
                .meters
                .get(at)
                .ok_or(Error::Unavailable("actual ReportMeterDetails linked meter"))?;
            if meter.units == SourceOutputUnits::Invalid {
                return Err(Error::Unavailable(
                    "defined native linked-meter unitNames index",
                ));
            }
            host.write_mtd(
                format!(
                    "  On{}Meter={} [{}]\n",
                    if meter.meter_type == SourceMeterType::Normal {
                        ""
                    } else {
                        "Custom"
                    },
                    meter.name,
                    meter.units.symbol()
                )
                .as_bytes(),
            )?;
        }
    }
    for meter in &registration.meters.meters {
        if meter.units == SourceOutputUnits::Invalid {
            return Err(Error::Unavailable(
                "defined native meter-details unitNames index",
            ));
        }
        host.write_mtd(
            format!("\n For Meter={} [{}]", meter.name, meter.units.symbol()).as_bytes(),
        )?;
        if meter.resource != SourceMeterResource::Invalid {
            host.write_mtd(
                format!(
                    ", ResourceType={}",
                    meter.resource.source_name().map_err(Error::Output)?
                )
                .as_bytes(),
            )?;
        }
        if meter.end_use != SourceMeterEndUse::Invalid {
            host.write_mtd(
                format!(", EndUse={}", meter.end_use.name().map_err(Error::Output)?).as_bytes(),
            )?;
        }
        if meter.group != SourceMeterGroup::Invalid {
            host.write_mtd(
                format!(
                    ", Group={}",
                    meter.group.source_name().map_err(Error::Output)?
                )
                .as_bytes(),
            )?;
        }
        host.write_mtd(b", contents are:\n")?;
        if meter.meter_type == SourceMeterType::Normal {
            for &number in &meter.source_variable_numbers {
                let variable = registration.out_vars.get(number).ok_or(Error::Unavailable(
                    "actual ReportMeterDetails source variable",
                ))?;
                host.write_mtd(
                    format!(
                        "  {}{}\n",
                        variable.key_colon_name,
                        multiplier_string(variable)?
                    )
                    .as_bytes(),
                )?;
            }
        }
        // Positive Custom/CustomDec constructors/getters are unavailable before this
        // report. SourceMeterType::Invalid reaches no source-content branch, as native.
    }
    Ok(())
}
