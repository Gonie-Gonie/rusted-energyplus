//! Checked access to actual existing Zone/Resilience cells; no fallback values.
use super::{Error, SelectedZoneInputOwners};
#[derive(Clone, Copy)]
pub(super) enum Field {
    HeatIndex,
    HeatIndexOccu,
    HeatIndexOccupied,
    Humidex,
    HumidexOccu,
    HumidexOccupied,
    LowSet,
    HighSet,
    ColdSafety,
    HeatSafety,
    Unmet,
    DiscomfortOccu,
    DiscomfortOccupied,
    Co2,
    Co2Occu,
    Co2Occupied,
    Lighting,
    LightingOccu,
    LightingOccupied,
}
pub(super) fn count(n: i32) -> Result<usize, Error> {
    usize::try_from(n).map_err(|_| Error::Unavailable("actual resilience array extent"))
}
pub(super) fn index(n: i32) -> Result<usize, Error> {
    count(
        n.checked_sub(1)
            .ok_or(Error::Unavailable("actual resilience ordinal subtraction"))?,
    )
}
pub(super) fn zone_name(zones: &SelectedZoneInputOwners, n: i32) -> Result<&str, Error> {
    zones
        .zones
        .as_ref()
        .and_then(|v| index(n).ok().and_then(|i| v.get(i)))
        .map(|v| v.name.as_str())
        .ok_or(Error::Unavailable("actual resilience Zone.Name owner"))
}
pub(super) fn value(
    zones: &SelectedZoneInputOwners,
    n: i32,
    field: Field,
    j: usize,
) -> Result<f64, Error> {
    let row = zones
        .resilience
        .as_ref()
        .and_then(|v| index(n).ok().and_then(|i| v.get(i)))
        .ok_or(Error::Unavailable("actual retained ZoneResilience owner"))?;
    let values: &[f64] = match field {
        Field::HeatIndex => &row.zone_heat_index_hour_bins,
        Field::HeatIndexOccu => &row.zone_heat_index_occu_hour_bins,
        Field::HeatIndexOccupied => &row.zone_heat_index_occupied_hour_bins,
        Field::Humidex => &row.zone_humidex_hour_bins,
        Field::HumidexOccu => &row.zone_humidex_occu_hour_bins,
        Field::HumidexOccupied => &row.zone_humidex_occupied_hour_bins,
        Field::LowSet => &row.zone_low_set_hours,
        Field::HighSet => &row.zone_high_set_hours,
        Field::ColdSafety => &row.zone_cold_hour_of_safety_bins,
        Field::HeatSafety => &row.zone_heat_hour_of_safety_bins,
        Field::Unmet => &row.zone_unmet_degree_hour_bins,
        Field::DiscomfortOccu => &row.zone_discomfort_wt_exceed_occu_hour_bins,
        Field::DiscomfortOccupied => &row.zone_discomfort_wt_exceed_occupied_hour_bins,
        Field::Co2 => &row.zone_co2_level_hour_bins,
        Field::Co2Occu => &row.zone_co2_level_occu_hour_bins,
        Field::Co2Occupied => &row.zone_co2_level_occupied_hour_bins,
        Field::Lighting => &row.zone_lighting_level_hour_bins,
        Field::LightingOccu => &row.zone_lighting_level_occu_hour_bins,
        Field::LightingOccupied => &row.zone_lighting_level_occupied_hour_bins,
    };
    values.get(j).copied().ok_or(Error::Unavailable(
        "actual resilience std::array.at bin ordinal",
    ))
}
pub(super) fn date(value: f64) -> Result<Vec<u8>, Error> {
    // Native13236/13104 converts double to the actual DateToString int argument.
    let integer = value.trunc();
    if !integer.is_finite() || integer < f64::from(i32::MIN) || integer > f64::from(i32::MAX) {
        return Err(Error::Unavailable(
            "actual resilience DateToString double-to-int defined domain",
        ));
    }
    Ok(super::super::remaining::source_date_to_string(integer as i32)?.into_bytes())
}
