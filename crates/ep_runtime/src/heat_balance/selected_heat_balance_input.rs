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
mod construction_types;
mod construction_window_arrays;
pub(crate) use construction::ConstructionMaterialOwners;
use construction_window_arrays::SourceConstructionWindowArrays;
mod control;
mod input;
pub(crate) mod shadowing;
pub mod surface;
pub(crate) mod zone;
pub use super::ordinary_surface_initialization::SourceInitSurfaceError as SourceSurfaceInitializationError;
pub use super::ordinary_surface_predefined::SourceSurfacePredefinedError;
pub use input::SelectedHeatBalanceInput;
pub(crate) use input::local_getter::SourceLocalObjectArguments;

/// An actual backend/stream failure, or a boundary outside this selected getter.
#[derive(Debug)]
pub enum HeatBalanceInputError {
    /// The real borrowed UtilityRoutines backend failed.
    Backend(SourceErrorBackendFailure),
    /// The reached actual ShowFatalError/backend stop retains its real cause.
    Fatal(super::error_backend::SourceFatalStop),
    /// The real borrowed EIO stream failed.
    Io(io::Error),
    /// A real output registration/file-host failure.
    Output(super::radiation::ordinary::OrdinaryRadiationError),
    /// A raw identity, storage or unsupported branch is not represented.
    Unavailable(&'static str),
    /// The existing Source R formatter rejected a value.
    Format(String),
    /// Actual CTF scientific/diagnostic/report cause, preserved across HB callers.
    CtfInitialization(
        Box<super::ordinary_heat_balance_initialization::ctf::SourceCtfInitializationError>,
    ),
    /// Actual selected thermal-history representability cause, never a Native fatal.
    ThermalHistory(super::ctf_thermal_history::CtfThermalHistoryUnavailable),
    /// Actual surface initialization/predefined cause retained across manager callers.
    SurfaceInitialization(Box<SourceSurfaceInitializationError>),
    /// Actual generic predefined table representability cause, preserved by callers.
    PredefinedReport(super::ctf_reports::CtfReportUnavailable),
    /// Actual Source input-driver RuntimeError, transported without losing its cause.
    SourceInput(Box<crate::RuntimeError>),
}
impl From<super::ctf_reports::CtfReportUnavailable> for HeatBalanceInputError {
    fn from(error: super::ctf_reports::CtfReportUnavailable) -> Self {
        Self::PredefinedReport(error)
    }
}
impl From<SourceErrorBackendFailure> for HeatBalanceInputError {
    fn from(value: SourceErrorBackendFailure) -> Self {
        Self::Backend(value)
    }
}
impl From<super::ordinary_heat_balance_initialization::ctf::SourceCtfInitializationError>
    for HeatBalanceInputError
{
    fn from(
        error: super::ordinary_heat_balance_initialization::ctf::SourceCtfInitializationError,
    ) -> Self {
        Self::CtfInitialization(Box::new(error))
    }
}
impl From<super::ctf_thermal_history::CtfThermalHistoryUnavailable> for HeatBalanceInputError {
    fn from(value: super::ctf_thermal_history::CtfThermalHistoryUnavailable) -> Self {
        Self::ThermalHistory(value)
    }
}
impl From<SourceSurfaceInitializationError> for HeatBalanceInputError {
    fn from(error: SourceSurfaceInitializationError) -> Self {
        Self::SurfaceInitialization(Box::new(error))
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
            Self::Fatal(e) => std::fmt::Display::fmt(e, f),
            Self::Io(e) => std::fmt::Display::fmt(e, f),
            Self::Output(e) => std::fmt::Display::fmt(e, f),
            Self::Unavailable(e) => write!(f, "selected heat-balance input unavailable: {e}"),
            Self::Format(e) => f.write_str(e),
            Self::CtfInitialization(e) => std::fmt::Display::fmt(e, f),
            Self::ThermalHistory(e) => std::fmt::Display::fmt(e, f),
            Self::SurfaceInitialization(e) => std::fmt::Display::fmt(e, f),
            Self::PredefinedReport(e) => std::fmt::Display::fmt(e, f),
            Self::SourceInput(e) => std::fmt::Display::fmt(e, f),
        }
    }
}
impl std::error::Error for HeatBalanceInputError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Backend(e) => Some(e),
            Self::Fatal(e) => Some(e),
            Self::Io(e) => Some(e),
            Self::Output(e) => Some(e),
            Self::CtfInitialization(e) => Some(e),
            Self::ThermalHistory(e) => Some(e),
            Self::SurfaceInitialization(e) => Some(e),
            Self::PredefinedReport(e) => Some(e),
            Self::SourceInput(e) => Some(e),
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
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum SelectedHeatTransferModel {
    #[default]
    Invalid,
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
    pub(crate) sys_total_hvac_relief_heat_loss: f64,
    pub(crate) sys_total_hvac_reject_heat_loss: f64,
    pub(crate) building_name: String,
    pub(crate) building_azimuth: f64,
    pub(crate) building_rotation_appendix_g: f64,
    pub(crate) loads_convergence_tolerance: f64,
    pub(crate) temperature_convergence_tolerance: f64,
    pub(crate) max_number_of_warmup_days: i32,
    pub(crate) inside_convection_algorithm: InsideSurfaceConvectionAlgorithm,
    pub(crate) outside_convection_algorithm: OutsideSurfaceConvectionAlgorithm,
    pub(crate) solar_distribution: SelectedSolarDistribution,
    pub(crate) calculate_solar_reflections: bool,
    pub(crate) calculate_window_reveal_reflection: bool,
    /// DataHeatBalance.hh1984; CheckGlazingShadingStatusChange owns later writes.
    pub(crate) encl_rad_always_recalc: bool,
    pub(crate) any_ctf: bool,
    // Actual constructor false; unsupported GetSolutionAlgorithm branches do not write these.
    pub(crate) any_empd: bool,
    pub(crate) any_cond_fd: bool,
    pub(crate) any_hamt: bool,
    /// Actual DataHeatBalance.hh1826 ctorfalse; positive Kiva getter remains excluded.
    pub(crate) any_kiva: bool,
    pub(crate) all_ctf: bool,
    /// Actual DataHeatBalance.hh1828 constructorfalse; positive InitBSDFWindows remains excluded.
    pub(crate) any_bsdf: bool,
    /// DataHeatBalance.hh1883; actual positive latent-sizing writers remain excluded.
    pub(crate) do_latent_sizing: bool,
    pub(crate) zone_air_group_loads:
        super::ordinary_air_heat_balance::predictor_corrector::SourceZoneListGroupLoadReports,
    /// Actual DataHeatBalance.hh1827 constructor false; positive getter stays unported.
    pub(crate) any_air_boundary: bool,
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
    /// Sole Construction coefficient allocations; completed stores move to the public owner.
    pub(crate) source_ctf_coefficients:
        Vec<Option<super::surface_manager::ctf_public_storage::CtfPublicCoefficientStorage>>,
    /// Sole DataWindowEquivalentLayer284 ctor/count written by GetConstructData.
    pub(crate) total_equivalent_layer_constructions: i32,
    /// Actual HeatBalance constructor-unallocated arrays, dimensioned at1393-1396.
    pub(crate) nominal_r_for_u: Option<Vec<f64>>,
    pub(crate) nominal_u: Option<Vec<f64>>,
    pub(crate) nominal_u_before_adjusted: Option<Vec<f64>>,
    pub(crate) coefficient_adjustment_ratio: Option<Vec<f64>>,
    /// Actual DataHeatBalance.hh1873-1878 constructor flags; positive People input writers are unported.
    pub(crate) adaptive_comfort_requested_cen15251: bool, // Native AdaptiveComfortRequested_CEN15251
    pub(crate) adaptive_comfort_requested_ash55: bool, // Native AdaptiveComfortRequested_ASH55
    pub(crate) any_thermal_comfort_pierce_model: bool, // Native AnyThermalComfortPierceModel
    pub(crate) any_thermal_comfort_ksu_model: bool,    // Native AnyThermalComfortKSUModel
    pub(crate) any_thermal_comfort_cooling_effect_model: bool, // Native AnyThermalComfortCoolingEffectModel
    pub(crate) any_thermal_comfort_ankle_draft_model: bool, // Native AnyThermalComfortAnkleDraftModel
    pub(crate) heat_index_method: i32,                      // Native HeatIndexMethod::Simplified=0
    pub(crate) any_internal_heat_source_in_input: bool,
    pub(crate) no_regular_materials_used: bool,
}
impl Default for SelectedHeatBalanceInputState {
    fn default() -> Self {
        Self {
            sys_total_hvac_relief_heat_loss: 0.0,
            sys_total_hvac_reject_heat_loss: 0.0,
            building_name: String::new(),
            building_azimuth: 0.0,
            building_rotation_appendix_g: 0.0,
            loads_convergence_tolerance: 0.0,
            temperature_convergence_tolerance: 0.0,
            max_number_of_warmup_days: 25,
            inside_convection_algorithm: InsideSurfaceConvectionAlgorithm::Simple,
            outside_convection_algorithm: OutsideSurfaceConvectionAlgorithm::SimpleCombined,
            solar_distribution: SelectedSolarDistribution::FullExterior,
            calculate_solar_reflections: false,
            calculate_window_reveal_reflection: false,
            encl_rad_always_recalc: false,
            any_ctf: false,
            any_empd: false,
            any_cond_fd: false,
            any_hamt: false,
            any_kiva: false,
            all_ctf: true,
            any_bsdf: false,
            do_latent_sizing: false,
            zone_air_group_loads: super::ordinary_air_heat_balance::predictor_corrector::SourceZoneListGroupLoadReports::default(),
            any_air_boundary: false,
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
            source_ctf_coefficients: Vec::new(),
            total_equivalent_layer_constructions: 0,
            nominal_r_for_u: None,
            nominal_u: None,
            nominal_u_before_adjusted: None,
            coefficient_adjustment_ratio: None,
            adaptive_comfort_requested_cen15251: false,
            adaptive_comfort_requested_ash55: false,
            any_thermal_comfort_pierce_model: false,
            any_thermal_comfort_ksu_model: false,
            any_thermal_comfort_cooling_effect_model: false,
            any_thermal_comfort_ankle_draft_model: false,
            heat_index_method: 0,
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

/// Same Construction input owner; no per-Surface copies of these properties.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SelectedConstructionUsage {
    /// Construction.Name, assigned from the actual zeroth alpha after uniqueness.
    pub(crate) name: String,
    pub(crate) construction_id: Option<ConstructionId>,
    /// ConstructionProps constructor: TotLayers0 and eleven LayerPoint0.
    pub(crate) total_layers: i32,
    pub(crate) layer_points: [i32; 11],
    /// ConstructionProps ctor0; actual CheckAndSet writes at GetConstructData1795.
    pub(crate) total_solid_layers: i32,
    pub(crate) total_glass_layers: i32,
    pub(crate) inside_absorp_vis: f64,
    pub(crate) outside_absorp_vis: f64,
    pub(crate) inside_absorp_solar: f64,
    pub(crate) outside_absorp_solar: f64,
    pub(crate) inside_absorp_thermal: f64,
    pub(crate) outside_absorp_thermal: f64,
    /// Construction.hh225, independent of TypeIsWindow; opaque getters do not write it.
    pub(crate) trans_diff: f64,
    /// Exact Window-related Construction.hh constructor members, one real owner.
    pub(crate) trans_diff_vis: f64,
    pub(crate) abs_diff_back_shade: f64,
    pub(crate) shade_absorp_thermal: f64,
    pub(crate) reflect_sol_diff_back: f64,
    pub(crate) reflect_sol_diff_front: f64,
    pub(crate) reflect_vis_diff_front: f64,
    pub(crate) abs_beam_shade_coef: [f64; 6],
    pub(crate) trans_sol_beam_coef: [f64; 6],
    pub(crate) refl_sol_beam_front_coef: [f64; 6],
    pub(crate) refl_sol_beam_back_coef: [f64; 6],
    pub(crate) trans_vis_beam_coef: [f64; 6],
    pub(crate) from_window5_data_file: bool,
    pub(crate) window_type_bsdf: bool,
    /// Actual constructor-unallocated arrays; real GetConstructData1419 allocates.
    pub(crate) window_arrays: SourceConstructionWindowArrays,
    pub(crate) reflect_vis_diff_back: f64,
    pub(crate) abs_diff_shade: f64,
    pub(crate) daylt_prop_ptr: i32,
    /// None is the genuine SurfaceRoughness::Invalid constructor, not a material value.
    pub(crate) outside_roughness: Option<ep_model::MaterialSurfaceRoughness>,
    /// Constructor false; target writes at real CheckAndSet boundary1795.
    pub(crate) type_is_window: bool,
    /// Construction.hh259; genuine ctor false, positive source construction getter excluded.
    pub(crate) source_sink_present: bool,
    /// Construction.hh188 ctor0, GetConstructData1409 writes1.
    pub(crate) solution_dimensions: i32,
    pub(crate) source_after_layer: i32,
    pub(crate) temperature_after_layer: i32,
    pub(crate) thickness_perpendicular: f64,
    pub(crate) w5_frame_divider: i32,
    /// ConstructionProps constructor false; actual positive AirBoundary getter stays excluded.
    pub(crate) type_is_air_boundary: bool,
    /// Genuine ConstructionProps WindowTypeEQL constructorfalse; positive getter excluded.
    pub(crate) window_type_eql: bool,
    pub(crate) type_is_eco_roof: bool,
    pub(crate) type_is_irt: bool,
    pub(crate) is_used: bool,
    pub(crate) is_used_ctf: bool,
    /// Construction.hh270, distinct from the isTCMaster constructor member.
    pub(crate) is_tc_window: bool,
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

// RuntimeError's established equality API compares retained OS/backend identity.
impl PartialEq for HeatBalanceInputError {
    fn eq(&self, other: &Self) -> bool {
        use super::error_backend::{SourceErrorBackendFailure as B, SourceFatalStop as F};
        let io_eq = |a: &io::Error, b: &io::Error| {
            a.kind() == b.kind()
                && a.raw_os_error() == b.raw_os_error()
                && a.to_string() == b.to_string()
        };
        let backend_eq = |a: &B, b: &B| match (a, b) {
            (
                B::Effect {
                    operation: a,
                    source: x,
                },
                B::Effect {
                    operation: b,
                    source: y,
                },
            ) => a == b && io_eq(x, y),
            (
                B::UndefinedCounterIncrement { counter: a },
                B::UndefinedCounterIncrement { counter: b },
            ) => a == b,
            _ => false,
        };
        match (self, other) {
            (Self::Backend(a), Self::Backend(b)) => backend_eq(a, b),
            (Self::Io(a), Self::Io(b)) => io_eq(a, b),
            (Self::Output(a), Self::Output(b)) => a == b,
            (Self::Unavailable(a), Self::Unavailable(b)) => a == b,
            (Self::Format(a), Self::Format(b)) => a == b,
            (Self::Fatal(F::Fatal(a)), Self::Fatal(F::Fatal(b))) => a == b,
            (Self::Fatal(F::Backend(a)), Self::Fatal(F::Backend(b))) => backend_eq(a, b),
            _ => false,
        }
    }
}
