//! Exact reached module constructor state, retained once across HVAC/timesteps.
//! Earlier native lazy getters must write these same owners, never copied counts.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceLowTemperatureRadiantAverages {
    /// LowTempRadiantSystem.hh526 TotalNumOfRadSystems ctor0.
    pub(crate) total_num_of_radiant_systems: i32,
    /// Native LowTempRadiantSystem.hh521/523/525 independent constructor-zero counts.
    pub(crate) number_of_hydronic_systems: i32,
    pub(crate) number_of_constant_flow_systems: i32,
    pub(crate) number_of_electric_systems: i32,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceHighTemperatureRadiantAverages {
    /// HighTempRadiantSystem.hh189 NumOfHighTempRadSys ctor0.
    pub(crate) number_of_high_temperature_radiant_systems: i32,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceHotWaterBaseboardAverages {
    /// HWBaseboardRadiator.hh210 NumHWBaseboards ctor0.
    pub(crate) number_of_hot_water_baseboards: i32,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceSteamBaseboardAverages {
    /// SteamBaseboardRadiator.hh219 NumSteamBaseboards ctor0.
    pub(crate) number_of_steam_baseboards: i32,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceElectricBaseboardAverages {
    /// ElectricBaseboardRadiator.hh158 NumElecBaseboards ctor0.
    pub(crate) number_of_electric_baseboards: i32,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceCoolingPanelAverage {
    /// ChilledCeilingPanelSimple.hh149/150 actual ctor0 fields only.
    pub(crate) source: f64,
    pub(crate) source_average: f64,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceCoolingPanelAverages {
    /// ChilledCeilingPanelSimple.hh190 actual constructor-unallocated array.
    /// Some(empty) is distinct and must still reach the distribution child.
    pub(crate) panels: Option<Vec<SourceCoolingPanelAverage>>,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceSwimmingPoolAverages {
    /// SwimmingPool.hh210 NumSwimmingPools ctor0.
    pub(crate) number_of_swimming_pools: i32,
}
/// Native VentilatedSlab.hh400 independent module count, never inferred from other systems.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceVentilatedSlabGains {
    pub(crate) number_of_ventilated_slabs: i32,
}
/// Owns the seven distinct real module-state subsets, rather than a numeric
/// registry derived from TypedModel/input absence. Root moves this owner once.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceFinalSurfaceHeatBalance {
    pub(crate) low_temperature: SourceLowTemperatureRadiantAverages,
    pub(crate) ventilated_slab: SourceVentilatedSlabGains,
    pub(crate) high_temperature: SourceHighTemperatureRadiantAverages,
    pub(crate) hot_water_baseboard: SourceHotWaterBaseboardAverages,
    pub(crate) steam_baseboard: SourceSteamBaseboardAverages,
    pub(crate) electric_baseboard: SourceElectricBaseboardAverages,
    pub(crate) cooling_panel: SourceCoolingPanelAverages,
    pub(crate) swimming_pool: SourceSwimmingPoolAverages,
}
