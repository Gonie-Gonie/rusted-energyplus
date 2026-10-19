//! Actual independent native members read by CalcHeatEmissionReport4260-4462.
//! Selected module storage only; positive factories/record types remain unavailable.
use std::convert::Infallible;
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceHeatEmissionDeviceModules {
    /// Fans.hh457 actual Array1D<FanBase *> constructor-unallocated; shared report owner.
    pub(crate) fans: Option<Vec<Infallible>>,
    /// MixedAir.hh477 NumOAControllers constructor0; no getter is invoked here.
    pub(crate) number_oa_controllers: i32,
    /// CondenserLoopTowers.hh440 actual Array1D constructor-unallocated.
    pub(crate) towers: Option<Vec<Infallible>>,
    /// EvaporativeFluidCoolers.hh260 actual Array1D constructor-unallocated.
    pub(crate) evaporative_fluid_coolers: Option<Vec<Infallible>>,
    /// FluidCoolers.hh224 actual Array1D constructor-unallocated.
    pub(crate) fluid_coolers: Option<Vec<Infallible>>,
    /// PlantChillers.hh509-512 independent constructor0 members.
    pub(crate) number_electric_chillers: i32,
    pub(crate) number_engine_chillers: i32,
    pub(crate) number_gas_turbine_chillers: i32,
    pub(crate) number_constant_cop_chillers: i32,
    /// ChillerElectricEIR.hh253 actual Array1D constructor-unallocated.
    pub(crate) electric_eir_chillers: Option<Vec<Infallible>>,
    /// ChillerReformulatedEIR.hh274 actual Array1D constructor-unallocated.
    pub(crate) reformulated_eir_chillers: Option<Vec<Infallible>>,
    /// Boilers.hh191 actual std::vector constructor-empty, distinct from None.
    pub(crate) boilers: Vec<Infallible>,
    /// DXCoils.hh956 NumDXCoils constructor0; distinct from new Coils::CoolingDX.
    pub(crate) number_dx_coils: i32,
    /// VariableSpeedCoils.hh553 NumVarSpeedCoils constructor0.
    pub(crate) number_variable_speed_coils: i32,
    /// HeatingCoils.hh298 NumHeatingCoils constructor0.
    pub(crate) number_heating_coils: i32,
    /// PackagedThermalStorageCoil.hh497 NumTESCoils constructor0.
    pub(crate) number_thermal_storage_coils: i32,
    /// HVACVariableRefrigerantFlow.hh1007 NumVRFCond constructor0.
    pub(crate) number_vrf_condensers: i32,
    /// EvaporativeCoolers.hh445 NumEvapCool constructor0.
    pub(crate) number_evaporative_coolers: i32,
}
// Keep the explicit Native constructor/default literals and ordered member initialization.
#[allow(clippy::derivable_impls)]
impl Default for SourceHeatEmissionDeviceModules {
    fn default() -> Self {
        Self {
            fans: None,
            number_oa_controllers: 0,
            towers: None,
            evaporative_fluid_coolers: None,
            fluid_coolers: None,
            number_electric_chillers: 0,
            number_engine_chillers: 0,
            number_gas_turbine_chillers: 0,
            number_constant_cop_chillers: 0,
            electric_eir_chillers: None,
            reformulated_eir_chillers: None,
            boilers: Vec::new(),
            number_dx_coils: 0,
            number_variable_speed_coils: 0,
            number_heating_coils: 0,
            number_thermal_storage_coils: 0,
            number_vrf_condensers: 0,
            number_evaporative_coolers: 0,
        }
    }
}
