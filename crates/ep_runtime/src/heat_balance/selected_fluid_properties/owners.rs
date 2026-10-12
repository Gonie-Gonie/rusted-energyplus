//! Literal retained FluidData, refrigerant, raw glycol and mixture storage.
use std::iter;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct FluidErrorCount {
    pub(crate) index: i32,
    pub(crate) count: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct SpecificHeatCacheCell {
    pub(crate) temperature_bits: u64,
    pub(crate) specific_heat: f64,
}

/// Non-INIT Source scalar storage has no numerical value to copy or compare.
/// None is custody of an allocated unknown cell, never a replacement +0.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct UninitializedSteamMatrix {
    pub(crate) pressure_points: usize,
    pub(crate) temperature_points: usize,
    pub(crate) cells: Vec<Option<f64>>,
}
impl UninitializedSteamMatrix {
    pub(crate) fn allocate(pressure_points: usize, temperature_points: usize) -> Self {
        Self {
            pressure_points,
            temperature_points,
            cells: vec![None; pressure_points * temperature_points],
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct RawGlycolProperty {
    pub(crate) temperature_array_name: String,
    pub(crate) data_present: bool,
    pub(crate) number_temperature_points: usize,
    pub(crate) number_concentration_points: usize,
    pub(crate) temperatures: Vec<f64>,
    pub(crate) concentrations: Vec<f64>,
    /// Source (concentration, temperature) order, temperature varies fastest.
    pub(crate) values: Vec<f64>,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct RawGlycolProperties {
    pub(crate) name: String,
    pub(crate) number: usize,
    pub(crate) cp: RawGlycolProperty,
    pub(crate) rho: RawGlycolProperty,
    pub(crate) conductivity: RawGlycolProperty,
    pub(crate) viscosity: RawGlycolProperty,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct GlycolProperty {
    pub(crate) data_present: bool,
    pub(crate) number_temperature_points: usize,
    pub(crate) low_temperature_value: f64,
    pub(crate) high_temperature_value: f64,
    pub(crate) low_temperature_index: usize,
    pub(crate) high_temperature_index: usize,
    pub(crate) temperatures: Vec<f64>,
    pub(crate) values: Vec<f64>,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct GlycolProperties {
    pub(crate) name: String,
    pub(crate) number: usize,
    pub(crate) used: bool,
    pub(crate) glycol_name: String,
    pub(crate) base_glycol_index: i32,
    pub(crate) concentration: f64,
    pub(crate) cp: GlycolProperty,
    pub(crate) rho: GlycolProperty,
    pub(crate) conductivity: GlycolProperty,
    pub(crate) viscosity: GlycolProperty,
    pub(crate) errors: [FluidErrorCount; 8],
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct RefrigerantValueSeries {
    /// Despite the Source *TempValue name, these hold property values.
    pub(crate) low_temperature_value: f64,
    pub(crate) high_temperature_value: f64,
    pub(crate) low_temperature_index: usize,
    pub(crate) high_temperature_index: usize,
    pub(crate) values: Vec<f64>,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct RefrigerantProperties {
    pub(crate) name: String,
    pub(crate) number: usize,
    pub(crate) used: bool,
    pub(crate) saturation_temperature_array_name: String,
    pub(crate) superheated_temperature_array_name: String,
    pub(crate) number_pressure_points: usize,
    pub(crate) pressure_low_temperature_value: f64,
    pub(crate) pressure_high_temperature_value: f64,
    pub(crate) pressure_low_temperature_index: usize,
    pub(crate) pressure_high_temperature_index: usize,
    pub(crate) low_pressure_value: f64,
    pub(crate) high_pressure_value: f64,
    pub(crate) low_pressure_index: usize,
    pub(crate) high_pressure_index: usize,
    pub(crate) pressure_temperatures: Vec<f64>,
    pub(crate) pressures: Vec<f64>,
    pub(crate) number_enthalpy_points: usize,
    pub(crate) enthalpy_temperatures: Vec<f64>,
    pub(crate) enthalpy_fluid: RefrigerantValueSeries,
    pub(crate) enthalpy_gas_fluid: RefrigerantValueSeries,
    pub(crate) number_cp_points: usize,
    pub(crate) cp_temperatures: Vec<f64>,
    pub(crate) cp_fluid: RefrigerantValueSeries,
    pub(crate) cp_gas_fluid: RefrigerantValueSeries,
    pub(crate) number_density_points: usize,
    pub(crate) density_temperatures: Vec<f64>,
    pub(crate) density_fluid: RefrigerantValueSeries,
    pub(crate) density_gas_fluid: RefrigerantValueSeries,
    pub(crate) number_superheated_temperature_points: usize,
    pub(crate) number_superheated_pressure_points: usize,
    pub(crate) superheated_temperatures: Vec<f64>,
    pub(crate) superheated_pressures: Vec<f64>,
    pub(crate) superheated_enthalpy: UninitializedSteamMatrix,
    pub(crate) superheated_density: UninitializedSteamMatrix,
    pub(crate) errors: [FluidErrorCount; 12],
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SelectedFluidProperties {
    pub(crate) debug_report_glycols: bool,
    pub(crate) debug_report_refrigerants: bool,
    pub(crate) glycol_error_limit_test: i32,
    pub(crate) refrigerant_error_limit_test: i32,
    pub(crate) refrigerants: Vec<RefrigerantProperties>,
    pub(crate) raw_glycols: Vec<RawGlycolProperties>,
    pub(crate) glycols: Vec<GlycolProperties>,
    pub(crate) glycol_error_limits: [i32; 8],
    pub(crate) saturated_error_count_superheated_enthalpy: i32,
    pub(crate) saturated_error_count_superheated_density: i32,
    pub(crate) quality_low_temperature_error_index: i32,
    pub(crate) quality_high_temperature_error_index: i32,
    pub(crate) interpolated_saturation_temperature_error_count: i32,
    pub(crate) interpolated_saturation_temperature_error_index: i32,
    /// Actual EP_cache_GlycolSpecificHeat constructor allocation in the pinned build.
    pub(crate) specific_heat_cache: Vec<SpecificHeatCacheCell>,
}
impl Default for SelectedFluidProperties {
    fn default() -> Self {
        Self {
            debug_report_glycols: false,
            debug_report_refrigerants: false,
            glycol_error_limit_test: 1,
            refrigerant_error_limit_test: 1,
            refrigerants: Vec::new(),
            raw_glycols: Vec::new(),
            glycols: Vec::new(),
            glycol_error_limits: [0; 8],
            saturated_error_count_superheated_enthalpy: 0,
            saturated_error_count_superheated_density: 0,
            quality_low_temperature_error_index: 0,
            quality_high_temperature_error_index: 0,
            interpolated_saturation_temperature_error_count: 0,
            interpolated_saturation_temperature_error_index: 0,
            specific_heat_cache: iter::repeat_n(
                SpecificHeatCacheCell {
                    temperature_bits: 1000,
                    specific_heat: 0.0,
                },
                1024 * 1024,
            )
            .collect(),
        }
    }
}
