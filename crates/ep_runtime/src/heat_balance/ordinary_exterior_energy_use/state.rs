//! Literal ExteriorEnergyUse.hh77-139 record constructors; no typed schedule IDs.
use super::SourceScheduleIndex;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub(super) enum SourceLightControlType {
    #[allow(
        dead_code,
        reason = "ExteriorEnergyUse.hh LightControlType retains the original invalid/count sentinels without changing the live schedule/clock identities."
    )]
    Invalid = -1,
    ScheduleOnly = 1,
    AstroClockOverride = 2,
    #[allow(
        dead_code,
        reason = "ExteriorEnergyUse.hh LightControlType retains the original invalid/count sentinels without changing the live schedule/clock identities."
    )]
    Num = 3,
}
#[derive(Clone, Debug, PartialEq)]
pub(super) struct SourceExteriorLightUsage {
    pub(super) name: String,
    pub(super) schedule: Option<SourceScheduleIndex>,
    pub(super) design_level: f64,
    pub(super) power: f64,
    pub(super) current_use: f64,
    pub(super) control: SourceLightControlType,
    pub(super) manage_demand: bool,
    pub(super) demand_limit: f64,
    pub(super) power_actuator_on: bool,
    pub(super) power_actuator_value: f64,
    pub(super) sum_consumption: f64,
    pub(super) sum_time_not_zero_consumption: f64,
}
impl Default for SourceExteriorLightUsage {
    fn default() -> Self {
        Self {
            name: String::new(),
            schedule: None,
            design_level: 0.0,
            power: 0.0,
            current_use: 0.0,
            control: SourceLightControlType::ScheduleOnly,
            manage_demand: false,
            demand_limit: 0.0,
            power_actuator_on: false,
            power_actuator_value: 0.0,
            sum_consumption: 0.0,
            sum_time_not_zero_consumption: 0.0,
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub(super) struct SourceExteriorEquipmentUsage {
    pub(super) name: String,
    pub(super) fuel_type: i32,
    pub(super) schedule: Option<SourceScheduleIndex>,
    pub(super) design_level: f64,
    pub(super) power: f64,
    pub(super) current_use: f64,
    pub(super) manage_demand: bool,
    pub(super) demand_limit: f64,
}
impl Default for SourceExteriorEquipmentUsage {
    fn default() -> Self {
        Self {
            name: String::new(),
            fuel_type: -1,
            schedule: None,
            design_level: 0.0,
            power: 0.0,
            current_use: 0.0,
            manage_demand: false,
            demand_limit: 0.0,
        }
    }
}
