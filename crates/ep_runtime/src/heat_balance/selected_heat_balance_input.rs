//! Selected HB input operations consumed by the ordinary source-context route.
//! Full GetHeatBalanceInput and InitHeatBalance orchestration remain incomplete.
use super::error_backend::{SourceErrorBackend, SourceErrorBackendFailure};
use super::selected_input_processor::{SelectedInputProcessor, SourceInputProcessorShortCut};
use super::selected_project_data::SelectedProjectData;
use crate::weather::day::WeatherEnvironmentState;
use ep_model::{
    ConstructionId, InsideSurfaceConvectionAlgorithm, OutsideSurfaceConvectionAlgorithm,
};
use std::collections::BTreeSet;
use std::io;

mod building;
mod construction;
mod control;
mod input;
pub(crate) mod shadowing;
pub(crate) mod zone;
pub use input::SelectedHeatBalanceInput;

/// An actual backend/stream failure, or a boundary outside this selected getter.
#[derive(Debug)]
pub enum HeatBalanceInputError {
    /// The real borrowed UtilityRoutines backend failed.
    Backend(SourceErrorBackendFailure),
    /// The real borrowed EIO stream failed.
    Io(io::Error),
    /// A real output registration/file-host failure.
    Output(super::radiation::ordinary::OrdinaryRadiationError),
    /// A raw identity, storage or unsupported branch is not represented.
    Unavailable(&'static str),
    /// The existing Source R formatter rejected a value.
    Format(String),
}
impl From<SourceErrorBackendFailure> for HeatBalanceInputError {
    fn from(value: SourceErrorBackendFailure) -> Self {
        Self::Backend(value)
    }
}
impl From<io::Error> for HeatBalanceInputError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}
impl std::fmt::Display for HeatBalanceInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Backend(e) => std::fmt::Display::fmt(e, f),
            Self::Io(e) => std::fmt::Display::fmt(e, f),
            Self::Output(e) => std::fmt::Display::fmt(e, f),
            Self::Unavailable(e) => write!(f, "selected heat-balance input unavailable: {e}"),
            Self::Format(e) => f.write_str(e),
        }
    }
}
impl std::error::Error for HeatBalanceInputError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Backend(e) => Some(e),
            Self::Io(e) => Some(e),
            Self::Output(e) => Some(e),
            Self::Unavailable(_) | Self::Format(_) => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SelectedZoneAirSolution {
    ThirdOrder,
    Euler,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SelectedSolarDistribution {
    Minimal,
    FullExterior,
    FullInteriorExterior,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SelectedHeatTransferModel {
    Ctf,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SelectedWindowThermalModel {
    BuiltIn,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SelectedWindowOpticalModel {
    Simplified,
}

/// Source constructor members and reached getter writes; retained even on Err.
/// The existing SelectedProjectData owns MinNumberOfWarmupDays and overrides.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SelectedHeatBalanceInputState {
    pub(crate) building_name: String,
    pub(crate) building_azimuth: f64,
    pub(crate) loads_convergence_tolerance: f64,
    pub(crate) temperature_convergence_tolerance: f64,
    pub(crate) max_number_of_warmup_days: i32,
    pub(crate) inside_convection_algorithm: InsideSurfaceConvectionAlgorithm,
    pub(crate) outside_convection_algorithm: OutsideSurfaceConvectionAlgorithm,
    pub(crate) solar_distribution: SelectedSolarDistribution,
    pub(crate) calculate_solar_reflections: bool,
    pub(crate) any_ctf: bool,
    pub(crate) all_ctf: bool,
    pub(crate) overall_heat_transfer_solution: SelectedHeatTransferModel,
    pub(crate) low_hconv_limit: f64,
    pub(crate) high_hconv_limit: f64,
    pub(crate) max_surface_temperature_limit: f64,
    pub(crate) max_surface_temperature_before_fatal: f64,
    pub(crate) zone_air_solution: SelectedZoneAirSolution,
    pub(crate) simulate_contaminants: bool,
    pub(crate) carbon_dioxide_simulation: bool,
    pub(crate) generic_contaminant_simulation: bool,
    pub(crate) enforce_zone_mass_balance: bool,
    pub(crate) root_algorithm: String,
    pub(crate) root_iterations_before_switch: i32,
    pub(crate) window_thermal_model: Option<SelectedWindowThermalModel>,
    pub(crate) window_optical_model: Option<SelectedWindowOpticalModel>,
    pub(crate) unique_construction_names: BTreeSet<String>,
    pub(crate) construction_order: Vec<ConstructionId>,
    pub(crate) construction_usage: Vec<SelectedConstructionUsage>,
    pub(crate) total_constructions: usize,
    pub(crate) any_internal_heat_source_in_input: bool,
    pub(crate) no_regular_materials_used: bool,
}
impl Default for SelectedHeatBalanceInputState {
    fn default() -> Self {
        Self {
            building_name: String::new(),
            building_azimuth: 0.0,
            loads_convergence_tolerance: 0.0,
            temperature_convergence_tolerance: 0.0,
            max_number_of_warmup_days: 25,
            inside_convection_algorithm: InsideSurfaceConvectionAlgorithm::Simple,
            outside_convection_algorithm: OutsideSurfaceConvectionAlgorithm::SimpleCombined,
            solar_distribution: SelectedSolarDistribution::FullExterior,
            calculate_solar_reflections: false,
            any_ctf: false,
            all_ctf: true,
            overall_heat_transfer_solution: SelectedHeatTransferModel::Ctf,
            low_hconv_limit: 0.1,
            high_hconv_limit: 1000.0,
            max_surface_temperature_limit: 200.0,
            max_surface_temperature_before_fatal: 500.0,
            zone_air_solution: SelectedZoneAirSolution::ThirdOrder,
            simulate_contaminants: false,
            carbon_dioxide_simulation: false,
            generic_contaminant_simulation: false,
            enforce_zone_mass_balance: false,
            root_algorithm: String::new(),
            root_iterations_before_switch: 5,
            window_thermal_model: None,
            window_optical_model: None,
            unique_construction_names: BTreeSet::new(),
            construction_order: Vec::new(),
            construction_usage: Vec::new(),
            total_constructions: 0,
            any_internal_heat_source_in_input: false,
            no_regular_materials_used: true,
        }
    }
}

pub(crate) struct HeatBalanceInputOwners<'a> {
    pub(crate) input: &'a mut SelectedInputProcessor,
    pub(crate) shortcut: &'a mut SourceInputProcessorShortCut,
    pub(crate) project: &'a mut SelectedProjectData,
    pub(crate) space_controls:
        &'a mut super::selected_heat_balance_space_controls::SelectedHeatBalanceSpaceControls,
    pub(crate) environment: &'a mut WeatherEnvironmentState,
    pub(crate) current_module_object: &'a mut String,
}
fn rounded(value: f64, precision: usize) -> Result<String, HeatBalanceInputError> {
    super::source_round_format::round(value, precision).map_err(|error| {
        HeatBalanceInputError::Format(format!("HB input .{precision}R: {error:?}"))
    })
}
fn absent(
    owners: &mut HeatBalanceInputOwners<'_>,
    family: &'static str,
) -> Result<(), HeatBalanceInputError> {
    *owners.current_module_object = family.into();
    if owners.input.get_num_objects_found(family) != 0 {
        return Err(HeatBalanceInputError::Unavailable(family));
    }
    Ok(())
}

/// Same Construction input owner; no per-Surface copies of these flags.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SelectedConstructionUsage {
    pub(crate) construction_id: Option<ConstructionId>,
    pub(crate) is_used: bool,
    pub(crate) is_used_ctf: bool,
    pub(crate) is_tc_master: bool,
}
impl SelectedHeatBalanceInputState {
    pub(crate) fn mark_opaque_surface_construction_used(
        &mut self,
        id: ConstructionId,
    ) -> Result<(), HeatBalanceInputError> {
        let row = self
            .construction_usage
            .iter_mut()
            .find(|row| row.construction_id == Some(id))
            .ok_or(HeatBalanceInputError::Unavailable(
                "resolved opaque construction input owner",
            ))?;
        row.is_used = true;
        Ok(())
    }
    pub(crate) fn mark_surface_ctf_construction_used(
        &mut self,
        id: ConstructionId,
    ) -> Result<(), HeatBalanceInputError> {
        let row = self
            .construction_usage
            .iter_mut()
            .find(|row| row.construction_id == Some(id))
            .ok_or(HeatBalanceInputError::Unavailable(
                "AssignHeatTransferAlgorithm construction owner",
            ))?;
        row.is_used_ctf = true;
        Ok(())
    }
}
