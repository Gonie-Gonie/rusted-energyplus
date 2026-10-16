//! Exact six reached PreDefTableEntry operations; borrow actual registry/table.
use super::*;
const HUMIDITY_REPORT_NAMES: [&str; 8] = [
    "Wetbulb [C]",
    "Dewpoint [C]",
    "Enthalpy [J/kg]",
    "Humidity Ratio []",
    "Schedule []",
    "WetBulbProfileDefaultMultipliers []",
    "WetBulbProfileDifferenceSchedule []",
    "WetBulbProfileMultiplierSchedule []",
];
fn column(registry: &PredefinedRegistry, name: &str) -> Result<i32, Error> {
    let column = *registry
        .named_indices
        .get(name)
        .ok_or(Error::Unavailable("actual DesignDay predefined column"))?;
    if column <= 0 || column > registry.column_count {
        return Err(Error::Unavailable(
            "actual DesignDay predefined column ordinal",
        ));
    }
    Ok(column)
}
pub(super) fn register(
    d: &SourceDesignDayData,
    registry: &PredefinedRegistry,
    table: &mut PredefinedStringTable,
) -> Result<(), Error> {
    table.fixed_real_entry_2(
        column(registry, "pdchDDmaxDB")?,
        &d.title,
        d.maximum_dry_bulb,
    )?;
    table.fixed_real_entry_2(
        column(registry, "pdchDDrange")?,
        &d.title,
        d.daily_dry_bulb_range,
    )?;
    if d.humidity_indicator_type != 4 {
        table.fixed_real_entry_2(
            column(registry, "pdchDDhumid")?,
            &d.title,
            d.humidity_indicator_value,
        )?;
    } else {
        table.string_entry(column(registry, "pdchDDhumid")?, &d.title, "N/A")?;
    }
    let humidity_type = *HUMIDITY_REPORT_NAMES
        .get(size(d.humidity_indicator_type)?)
        .ok_or(Error::Unavailable(
            "actual DesignDay humidity report name ordinal",
        ))?;
    table.string_entry(column(registry, "pdchDDhumTyp")?, &d.title, humidity_type)?;
    table.fixed_real_entry_2(column(registry, "pdchDDwindSp")?, &d.title, d.wind_speed)?;
    table.fixed_real_entry_2(
        column(registry, "pdchDDwindDr")?,
        &d.title,
        d.wind_direction,
    )?;
    Ok(())
}
