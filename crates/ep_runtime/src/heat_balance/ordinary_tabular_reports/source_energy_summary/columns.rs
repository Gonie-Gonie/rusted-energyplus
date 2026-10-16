//! Exact Native SourceEnergy column and row declarations; no output answers.
pub(super) const ENERGY_KWH: [&[u8]; 13] = [
    b"Source Electricity [kWh]",
    b"Source Natural Gas [kWh]",
    b"Source Gasoline [kWh]",
    b"Source Diesel [kWh]",
    b"Source Coal [kWh]",
    b"Source Fuel Oil No 1 [kWh]",
    b"Source Fuel Oil No 2 [kWh]",
    b"Source Propane [kWh]",
    b"Source Other Fuel 1 [kWh]",
    b"Source Other Fuel 2 [kWh]",
    b"Source District Cooling [kWh]",
    b"Source District Heating Water [kWh]",
    b"Source District Heating Steam [kWh]",
];
pub(super) const ENERGY_GJ: [&[u8]; 13] = [
    b"Source Electricity [GJ]",
    b"Source Natural Gas [GJ]",
    b"Source Gasoline [GJ]",
    b"Source Diesel [GJ]",
    b"Source Coal [GJ]",
    b"Source Fuel Oil No 1 [GJ]",
    b"Source Fuel Oil No 2 [GJ]",
    b"Source Propane [GJ]",
    b"Source Other Fuel 1 [GJ]",
    b"Source Other Fuel 2 [GJ]",
    b"Source District Cooling [GJ]",
    b"Source District Heating Water [GJ]",
    b"Source District Heating Steam [GJ]",
];
pub(super) const AREA_KWH: [&[u8]; 13] = [
    b"Source Electricity [kWh/m2]",
    b"Source Natural Gas [kWh/m2]",
    b"Source Gasoline [kWh/m2]",
    b"Source Diesel [kWh/m2]",
    b"Source Coal [kWh/m2]",
    b"Source Fuel Oil No 1 [kWh/m2]",
    b"Source Fuel Oil No 2 [kWh/m2]",
    b"Source Propane [kWh/m2]",
    b"Source Other Fuel 1 [kWh/m2]",
    b"Source Other Fuel 2 [kWh/m2]",
    b"Source District Cooling [kWh/m2]",
    b"Source District Heating Water [kWh/m2]",
    b"Source District Heating Steam [kWh/m2]",
];
pub(super) const AREA_MJ: [&[u8]; 13] = [
    b"Source Electricity [MJ/m2]",
    b"Source Natural Gas [MJ/m2]",
    b"Source Gasoline [MJ/m2]",
    b"Source Diesel [MJ/m2]",
    b"Source Coal [MJ/m2]",
    b"Source Fuel Oil No 1 [MJ/m2]",
    b"Source Fuel Oil No 2 [MJ/m2]",
    b"Source Propane [MJ/m2]",
    b"Source Other Fuel 1 [MJ/m2]",
    b"Source Other Fuel 2 [MJ/m2]",
    b"Source District Cooling [MJ/m2]",
    b"Source District Heating Water [MJ/m2]",
    b"Source District Heating Steam [MJ/m2]",
];
pub(super) const END_USE_ROWS: [&[u8]; 14] = [
    b"Heating",
    b"Cooling",
    b"Interior Lighting",
    b"Exterior Lighting",
    b"Interior Equipment",
    b"Exterior Equipment",
    b"Fans",
    b"Pumps",
    b"Heat Rejection",
    b"Humidification",
    b"Heat Recovery",
    b"Water Systems",
    b"Refrigeration",
    b"Generators",
];
pub(super) fn assign(columns: &mut [Vec<u8>], units: i32, normalized: bool) {
    let labels = match (units == 1, normalized) {
        (true, false) => &ENERGY_KWH,
        (false, false) => &ENERGY_GJ,
        (true, true) => &AREA_KWH,
        (false, true) => &AREA_MJ,
    };
    for i in 0..13 {
        columns[i] = labels[i].to_vec();
    }
}
