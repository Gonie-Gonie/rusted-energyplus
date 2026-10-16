//! Same actual PollutionData owner for startup, TAB getters and HVAC calculation.
mod calculation;
mod input;
mod output_registration;
mod reporting;
use super::error_backend::SourceErrorBackend;
use super::selected_heat_balance_input::HeatBalanceInputError as Error;
use super::selected_input_processor::{SelectedInputProcessor, SourceInputProcessorShortCut};
use super::selected_schedule_manager::{SelectedScheduleManager, SourceScheduleIndex};
use super::source_output_registration::{
    SourceOutputRegistration, SourceRealOutputOwners, TimeStepKind,
};
pub(crate) use crate::heat_balance::source_output_registration::SourceCarbonEquivalentField;
use crate::weather::day::WeatherGlobalState;
pub(crate) use output_registration::SourcePollutionOutputContext;

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourcePollutionComponent {
    pub(crate) source_value: f64,
    pub(crate) pollutant_values: [f64; 16],
}
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourcePollutionCoefficient {
    pub(crate) used: bool,
    pub(crate) source_coefficient: f64,
    pub(crate) pollutant_coefficients: [f64; 16],
    pub(crate) source_schedule: Option<SourceScheduleIndex>,
    pub(crate) pollutant_schedules: [Option<SourceScheduleIndex>; 16],
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourcePollutionModule {
    pub(crate) report_setup: bool,
    pub(crate) get_input_flag: bool,
    pub(crate) number_environmental_impact_factors: usize,
    pub(crate) number_fuel_factors: usize,
    pub(crate) components: [SourcePollutionComponent; 12],
    pub(crate) facility_meter_numbers: [i32; 16],
    pub(crate) facility_meter_values: [f64; 16],
    pub(crate) fuel_component_values: [f64; 12],
    pub(crate) pollutant_values: [f64; 16],
    pub(crate) fuel_factor_list: Vec<usize>,
    pub(crate) total_carbon_equivalent_n2o: f64,
    pub(crate) total_carbon_equivalent_ch4: f64,
    pub(crate) total_carbon_equivalent_co2: f64,
    pub(crate) coefficients: [SourcePollutionCoefficient; 10],
    pub(crate) carbon_equivalent_n2o: f64,
    pub(crate) carbon_equivalent_ch4: f64,
    pub(crate) carbon_equivalent_co2: f64,
    pub(crate) purchased_heat_efficiency: f64,
    pub(crate) purchased_cooling_cop: f64,
    pub(crate) steam_conversion_efficiency: f64,
}
impl Default for SourcePollutionModule {
    fn default() -> Self {
        let mut facility_meter_numbers = [0; 16];
        // Literal std::array<int,16>={-1}: only first is -1; remaining cells are 0.
        facility_meter_numbers[0] = -1;
        Self {
            report_setup: false,
            get_input_flag: true,
            number_environmental_impact_factors: 0,
            number_fuel_factors: 0,
            components: std::array::from_fn(|_| SourcePollutionComponent::default()),
            facility_meter_numbers,
            facility_meter_values: [0.0; 16],
            fuel_component_values: [0.0; 12],
            pollutant_values: [0.0; 16],
            fuel_factor_list: Vec::new(),
            total_carbon_equivalent_n2o: 0.0,
            total_carbon_equivalent_ch4: 0.0,
            total_carbon_equivalent_co2: 0.0,
            coefficients: std::array::from_fn(|_| SourcePollutionCoefficient::default()),
            carbon_equivalent_n2o: 0.0,
            carbon_equivalent_ch4: 0.0,
            carbon_equivalent_co2: 0.0,
            purchased_heat_efficiency: 0.0,
            purchased_cooling_cop: 0.0,
            steam_conversion_efficiency: 0.0,
        }
    }
}
/// Native eFuel identities admitted by actual fuel2pollFuel table185-198.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SourcePollutionFuel {
    Electricity,
    NaturalGas,
    FuelOil1,
    FuelOil2,
    Coal,
    Gasoline,
    Propane,
    Diesel,
    OtherFuel1,
    OtherFuel2,
    DistrictCooling,
    DistrictHeatingWater,
    DistrictHeatingSteam,
}
impl SourcePollutionFuel {
    fn coefficient(self) -> usize {
        match self {
            Self::Electricity | Self::DistrictCooling => 0,
            Self::NaturalGas | Self::DistrictHeatingWater | Self::DistrictHeatingSteam => 1,
            Self::FuelOil1 => 2,
            Self::FuelOil2 => 3,
            Self::Coal => 4,
            Self::Gasoline => 5,
            Self::Propane => 6,
            Self::Diesel => 7,
            Self::OtherFuel1 => 8,
            Self::OtherFuel2 => 9,
        }
    }
}
fn current_schedule(
    index: SourceScheduleIndex,
    schedules: &SelectedScheduleManager,
) -> Result<f64, Error> {
    schedules
        .schedules
        .get(index.0)
        .map(|r| r.current_value)
        .ok_or(Error::Unavailable(
            "same actual Pollution schedule getCurrentVal",
        ))
}
