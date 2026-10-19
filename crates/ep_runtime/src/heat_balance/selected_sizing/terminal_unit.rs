//! Native DataSizing.hh658-687 full TermUnitSizingData constructor.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceTerminalUnitSizingData {
    pub(crate) controlled_zone_number: i32,
    pub(crate) air_distribution_unit_name: String,
    pub(crate) air_volume_flow: f64,
    pub(crate) maximum_hot_water_volume_flow: f64,
    pub(crate) maximum_steam_volume_flow: f64,
    pub(crate) maximum_cold_water_volume_flow: f64,
    pub(crate) minimum_primary_flow_fraction: f64,
    pub(crate) induction_ratio: f64,
    pub(crate) induces_plenum_air: bool,
    pub(crate) reheat_air_flow_multiplier: f64,
    pub(crate) reheat_load_multiplier: f64,
    pub(crate) design_cooling_load: f64,
    pub(crate) design_heating_load: f64,
    pub(crate) specified_design_sensible_cooling_fraction: f64,
    pub(crate) specified_design_cooling_supply_air_temperature_ratio: f64,
    pub(crate) specified_design_sensible_heating_fraction: f64,
    pub(crate) specified_design_heating_supply_air_temperature_ratio: f64,
    pub(crate) specified_minimum_outdoor_air_fraction: f64,
    pub(crate) plenum_index: i32,
}
impl Default for SourceTerminalUnitSizingData {
    fn default() -> Self {
        Self {
            controlled_zone_number: 0,
            air_distribution_unit_name: String::new(),
            air_volume_flow: 0.0,
            maximum_hot_water_volume_flow: 0.0,
            maximum_steam_volume_flow: 0.0,
            maximum_cold_water_volume_flow: 0.0,
            minimum_primary_flow_fraction: 0.0,
            induction_ratio: 0.0,
            induces_plenum_air: false,
            reheat_air_flow_multiplier: 1.0,
            reheat_load_multiplier: 1.0,
            design_cooling_load: 0.0,
            design_heating_load: 0.0,
            specified_design_sensible_cooling_fraction: 1.0,
            specified_design_cooling_supply_air_temperature_ratio: 1.0,
            specified_design_sensible_heating_fraction: 1.0,
            specified_design_heating_supply_air_temperature_ratio: 1.0,
            specified_minimum_outdoor_air_fraction: 1.0,
            plenum_index: 0,
        }
    }
}
