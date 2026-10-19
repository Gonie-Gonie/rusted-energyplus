//! Consumed solar initialization prefix; ordinary interior exchange remains held.
//!
//! Original HeatBalanceIntRadExchange.cc811-1103. No
//! diagnostic probe supplies a caller flag, iteration count or cache result.

mod enclosure_setup;
mod initialization;
#[cfg(all(test, feature = "native-controlled-air-cohort"))]
mod native_gap_cohort;
mod radiant_exchange;
pub(crate) mod shading_report;
pub(crate) mod solar_output_registration;
pub(crate) mod solar_timestep;
pub(crate) mod source_children;
mod source_initialization;
pub(crate) use enclosure_setup::EnclosureOutputOwners;
pub use radiant_exchange::OrdinaryRadiantExchangeState;
pub(crate) use radiant_exchange::{
    RadiantExchangeCall, RadiantExchangeSource, RadiantInitializationOutput,
};

use crate::error::{RuntimeError, WarmupOutputError};
use crate::heat_balance::inside_convection_layout::InsideConvectionLayout;
use crate::heat_balance::state::SurfaceHeatBalanceState;
use ep_model::{SpaceId, SurfaceId, TypedModel, ZoneId};

/// Input projection from the existing complete ordinary raw admission.
/// Private fields prevent changing the selected branch after admission.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OrdinaryRadiationInput {
    user_view_factor_count: usize,
    performance_precision_count: usize,
    shelf_count: usize,
    advanced_report_variables: bool,
}

impl OrdinaryRadiationInput {
    /// Actual GetProjectData/environment flag, applied before the first Solar owner.
    pub(crate) fn apply_source_advanced_reporting(&mut self, value: bool) {
        self.advanced_report_variables = value;
    }

    /// Retain actual query counts after the existing positive-family/field
    /// admission has succeeded. This projection is not another allowlist.
    ///
    /// # Errors
    /// Positive user-F, precision or shelf input needs its real unported owner.
    pub fn from_admitted_inventory<'a>(
        inventory: impl IntoIterator<Item = (&'a str, usize)>,
        advanced_report_variables: bool,
    ) -> Result<Self, OrdinaryRadiationUnavailable> {
        let mut counts = [0; 3];
        let mut seen = [false; 3];
        for (family, count) in inventory {
            if let Some(index) = [
                "ZoneProperty:UserViewFactors:BySurfaceName",
                "PerformancePrecisionTradeoffs",
                "DaylightingDevice:Shelf",
            ]
            .iter()
            .position(|name| *name == family)
            {
                if seen[index] || count != 0 {
                    return Err(OrdinaryRadiationUnavailable::InputBranch);
                }
                seen[index] = true;
                counts[index] = count;
            }
        }
        Ok(Self {
            user_view_factor_count: counts[0],
            performance_precision_count: counts[1],
            shelf_count: counts[2],
            advanced_report_variables,
        })
    }
}

/// A missing real producer/caller or unsupported Original branch.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OrdinaryRadiationUnavailable {
    /// Raw solar input requires the actual caller-owned initialization context.
    MissingInitializationContext,
    /// User F/precision/shelf/representative/Carroll input is outside this slice.
    InputBranch,
    /// Complete typed geometry/report associations are inconsistent.
    EnclosureDomain,
    /// Actual operand is outside the existing warning-free primitive domain.
    NumericDomain,
    /// Requested full ViewFactorInfo output remains unported.
    ViewFactorReport,
    /// Actual selected input/storage bounds do not cover the full domain.
    StorageDomain,
    /// The real report scan stopped on an unsupported query.
    ReportScan,
}
impl std::fmt::Display for OrdinaryRadiationUnavailable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ordinary interior radiation unavailable: {self:?}")
    }
}
impl std::error::Error for OrdinaryRadiationUnavailable {}

/// Retains actual output/backend failures without replacement diagnostics.
#[derive(Debug, PartialEq)]
pub enum OrdinaryRadiationError {
    /// Bounded unavailable boundary; never a manufactured Original fatal.
    Unavailable(OrdinaryRadiationUnavailable),
    /// Reuse the existing OS/backend error carrier; no warmup call is inferred.
    Output(WarmupOutputError),
    /// The genuine selected Source fatal, preserving its original backend cause.
    Input(Box<crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError>),
}
impl std::fmt::Display for OrdinaryRadiationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unavailable(error) => std::fmt::Display::fmt(error, f),
            Self::Output(error) => write!(f, "ordinary radiation output stopped: {error}"),
            Self::Input(error) => std::fmt::Display::fmt(error, f),
        }
    }
}
impl std::error::Error for OrdinaryRadiationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Unavailable(error) => Some(error),
            Self::Output(error) => Some(error),
            Self::Input(error) => Some(error.as_ref()),
        }
    }
}
impl From<OrdinaryRadiationUnavailable> for OrdinaryRadiationError {
    fn from(error: OrdinaryRadiationUnavailable) -> Self {
        Self::Unavailable(error)
    }
}
impl From<std::io::Error> for OrdinaryRadiationError {
    fn from(error: std::io::Error) -> Self {
        Self::Output(WarmupOutputError::Io(error))
    }
}
impl From<OrdinaryRadiationError> for RuntimeError {
    fn from(error: OrdinaryRadiationError) -> Self {
        Self::OrdinaryRadiation(Box::new(error))
    }
}

/// Actual radiant enclosure fields written/read by geometry setup2857.
/// The actual radiant initializer and exchange consumer extend this same owner.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RadiantRadiationEnclosure {
    /// Actual Enclosure.Name, initially empty.
    pub name: String,
    /// Actual ordered member Space names.
    pub space_names: Vec<String>,
    /// Actual one-based Source Space indices, not copied typed IDs.
    pub space_nums: Vec<i32>,
    /// Actual geometry-setup FloorArea, constructor positive zero.
    pub floor_area: f64,
    /// Actual ExtWindowArea; radiant normal branch leaves constructor zero.
    pub exterior_window_area: f64,
    /// Actual TotalSurfArea; radiant normal branch leaves constructor zero.
    pub total_surface_area: f64,
    /// Actual MRT output field, constructor zero; CalculateZoneMRT writes this same owner.
    pub mean_radiant_temperature: f64,
    /// Actual Enclosure.sumAE constructor0, first MRT computes the surface sum.
    pub sum_ae: f64,
    /// Actual Enclosure.sumAET constructor0; native zone/enclosure update order retained.
    pub sum_aet: f64,
    /// Actual Enclosure.reCalcMRT constructorfalse; actual MRT marks applicable enclosures.
    pub recalculate_mrt: bool,
    /// Actual constructor0 and InitInterior count.
    pub num_surfaces: i32,
    /// Actual unallocated/dimensioned Source radiant arrays, distinct from Solar.
    pub f: Option<Vec<f64>>,
    /// Dimensionless at InitInterior; later recalculated/scaled and diagonal-mutated.
    pub script_f: Option<Vec<f64>>,
    /// Actual reached Area dimension and ordered values.
    pub area: Option<Vec<f64>>,
    /// Actual mutable EMISS owner, including Source high-emissivity limit writes.
    pub emissivity: Option<Vec<f64>>,
    /// Actual azimuth array.
    pub azimuth: Option<Vec<f64>>,
    /// Actual tilt array.
    pub tilt: Option<Vec<f64>>,
    /// Actual one-based Source SurfacePtr, unallocated before InitInterior.
    pub source_surface_ptr: Option<Vec<i32>>,
    /// Actual appended one-based enclosure report positions.
    pub source_surface_report_nums: Vec<i32>,
    /// Actual radReCalc constructorfalse; the genuine shading caller writes it.
    pub rad_recalc: bool,
    /// Actual radQThermalRad constructor0; InitInternalHeatGains writes this owner.
    pub rad_q_thermal_rad: f64,
    /// Actual radThermAbsMult constructor0 and thermal absorption reciprocal.
    pub rad_therm_abs_mult: f64,
}

/// Actual selected solar enclosure, available through immutable owner inspection.
#[derive(Clone, Debug, PartialEq)]
pub struct SolarRadiationEnclosure {
    /// One-based Rust association, assigned by the later physical binder.
    pub id: usize,
    /// Actual enclosure Name, initially empty and written at geometry setup797.
    pub name: String,
    /// Actual ordered member Space names.
    pub space_names: Vec<String>,
    /// Actual one-based Source Space indices.
    pub space_nums: Vec<i32>,
    /// Actual constructor-zero FloorArea, written by geometry setup797.
    pub floor_area: f64,
    /// Actual constructor-zero ExtWindowArea.
    pub exterior_window_area: f64,
    /// Actual constructor-zero TotalSurfArea.
    pub total_surface_area: f64,
    /// Genuine EnclosureViewFactorInformation constructor0/false.
    pub num_surfaces: i32,
    /// Genuine constructor false; InitSolar890-893 only assigns true.
    pub has_interzone_window: bool,
    /// Actual EnclosureViewFactorInformation.hh85 constructor zero.
    pub(crate) total_enclosure_daylight_reference_points: i32,
    /// Sole Source Array1D_int pointer, unallocated until actual InitSolar874.
    pub source_surface_ptr: Option<Vec<i32>>,
    /// One-based Source enclosure positions, appended in actual report order.
    pub source_surface_report_nums: Vec<i32>,
    /// Reached allocation state of the SAME F/Area/Absorptance/Azimuth/Tilt
    /// vectors below, in Source869-873 order; constructor false means unallocated.
    pub source_array_allocated: [bool; 5],
    /// Physical Zone binding is unavailable until the genuine later binder.
    pub zone_id: Option<ZoneId>,
    /// Actual bound physical member Space identities.
    pub space_ids: Vec<SpaceId>,
    /// Physical IDs joined to Source SurfacePtr by the later selected binder.
    pub surface_ptr: Vec<SurfaceId>,
    /// Explicit joins to retained SurfaceHeatBalanceState elements.
    pub surface_indexes: Vec<usize>,
    /// Actual report-to-traversal mapping on the admitted opaque domain.
    pub surface_report_nums: Vec<usize>,
    /// Actual per-surface areas, allocated by the reached solar initializer.
    pub area: Vec<f64>,
    /// Actual inside-material solar absorptance.
    pub absorptance: Vec<f64>,
    /// Actual surface azimuths.
    pub azimuth: Vec<f64>,
    /// Actual surface tilts.
    pub tilt: Vec<f64>,
    /// Actual approximate or successfully fixed F prefix in flat Original order.
    pub f: Vec<f64>,
    fix: Option<super::FixedViewFactors>,
    /// Actual radReCalc constructorfalse, written by the same shading caller.
    pub rad_recalc: bool,
    /// Actual solVMULT constructor0 and shortwave reciprocal.
    pub sol_vmult: f64,
    /// Actual solAbsFirstCalc constructortrue, cleared after successful warning.
    pub sol_abs_first_calc: bool,
}

impl Default for SolarRadiationEnclosure {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            space_names: Vec::new(),
            space_nums: Vec::new(),
            floor_area: 0.0,
            exterior_window_area: 0.0,
            total_surface_area: 0.0,
            num_surfaces: 0,
            has_interzone_window: false,
            total_enclosure_daylight_reference_points: 0,
            source_surface_ptr: None,
            source_surface_report_nums: Vec::new(),
            source_array_allocated: [false; 5],
            zone_id: None,
            space_ids: Vec::new(),
            surface_ptr: Vec::new(),
            surface_indexes: Vec::new(),
            surface_report_nums: Vec::new(),
            area: Vec::new(),
            absorptance: Vec::new(),
            azimuth: Vec::new(),
            tilt: Vec::new(),
            f: Vec::new(),
            fix: None,
            rad_recalc: false,
            sol_vmult: 0.0,
            sol_abs_first_calc: true,
        }
    }
}

/// Sole radiation container retained from early geometry/radiant setup through
/// the later Solar/radiant producers and consumers, then moved into State.
#[derive(Clone, Debug, PartialEq)]
pub struct OrdinarySolarRadiation {
    input: OrdinaryRadiationInput,
    // Source EnclSolInfo constructor-unallocated, then actual797 allocation.
    solar: Option<Vec<SolarRadiationEnclosure>>,
    radiant: Vec<RadiantRadiationEnclosure>,
    num_radiant_enclosures: i32,
    num_solar_enclosures: i32,
    // Surface.SolarEnclIndex, RadEnclIndex, SolarEnclSurfIndex, one-based.
    // Original InitSolar writes these associations even before radiant init.
    back_pointers: Vec<(usize, usize, usize)>,
    solar_initialized: bool,
    radiant_exchange: OrdinaryRadiantExchangeState,
    pub(crate) solar_timestep: Box<solar_timestep::SourceSolarTimestepState>,
}

impl OrdinarySolarRadiation {
    /// Construct the same container at the reached geometry boundary. Solar
    /// arrays and back-pointers remain untouched until their later real binder.
    pub(crate) fn for_surface_input(input: OrdinaryRadiationInput) -> Self {
        Self {
            input,
            solar: None,
            radiant: Vec::new(),
            num_radiant_enclosures: 0,
            num_solar_enclosures: 0,
            back_pointers: Vec::new(),
            solar_initialized: false,
            radiant_exchange: OrdinaryRadiantExchangeState::default(),
            solar_timestep: Box::default(),
        }
    }

    /// Actual retained radiant rows allocated by the geometry setup call.
    #[must_use]
    pub fn radiant_enclosures(&self) -> &[RadiantRadiationEnclosure] {
        &self.radiant
    }

    /// Actual passive view of Source radiation module state and scratch masks.
    #[must_use]
    pub fn radiant_exchange_state(&self) -> &OrdinaryRadiantExchangeState {
        &self.radiant_exchange
    }

    pub(crate) fn source_radiant_enclosures_mut(&mut self) -> &mut [RadiantRadiationEnclosure] {
        &mut self.radiant
    }

    pub(crate) fn source_solar_enclosures_mut(&mut self) -> Option<&mut [SolarRadiationEnclosure]> {
        self.solar.as_deref_mut()
    }

    /// Actual NumOfRadiantEnclosures, separate from the allocated row count.
    #[must_use]
    pub fn num_radiant_enclosures(&self) -> i32 {
        self.num_radiant_enclosures
    }

    /// Retained actual solar enclosure storage and reached numeric prefix.
    #[must_use]
    pub fn enclosures(&self) -> &[SolarRadiationEnclosure] {
        self.solar.as_deref().unwrap_or_default()
    }

    /// Actual Solar allocation, preserving constructor None versus allocated rows.
    #[must_use]
    pub fn enclosure_storage(&self) -> Option<&[SolarRadiationEnclosure]> {
        self.solar.as_deref()
    }

    /// Genuine NumOfSolarEnclosures, independent of allocated row count.
    #[must_use]
    pub fn num_solar_enclosures(&self) -> i32 {
        self.num_solar_enclosures
    }

    /// Actual one-based surface enclosure associations written by InitSolar.
    #[must_use]
    pub fn back_pointers(&self) -> &[(usize, usize, usize)] {
        &self.back_pointers
    }

    /// True only after the reached solar initializer returned normally.
    #[must_use]
    pub fn is_initialized(&self) -> bool {
        self.solar_initialized
    }

    pub(crate) fn for_initialized_geometry(
        model: &TypedModel,
        surfaces: &[SurfaceHeatBalanceState],
        input: OrdinaryRadiationInput,
    ) -> Result<Self, OrdinaryRadiationError> {
        let mut owner = Self::for_surface_input(input);
        owner.bind_initialized_solar_geometry(model, surfaces, input)?;
        Ok(owner)
    }

    /// Bind later Solar geometry into the SAME early radiant container.
    /// A failed bind leaves its actual radiant allocation and values untouched.
    pub(crate) fn bind_initialized_solar_geometry(
        &mut self,
        model: &TypedModel,
        surfaces: &[SurfaceHeatBalanceState],
        input: OrdinaryRadiationInput,
    ) -> Result<(), OrdinaryRadiationError> {
        if self.input != input {
            return Err(OrdinaryRadiationUnavailable::EnclosureDomain.into());
        }
        let layout = InsideConvectionLayout::from_typed_model(model)
            .map_err(|_| OrdinaryRadiationUnavailable::EnclosureDomain)?;
        if input.user_view_factor_count != 0
            || input.performance_precision_count != 0
            || input.shelf_count != 0
        {
            return Err(OrdinaryRadiationUnavailable::InputBranch.into());
        }
        if model.zones.len() != 1
            || model.spaces.len() != 1
            || layout.groups().len() != 1
            || surfaces.len() != model.surfaces.len()
            || surfaces.is_empty()
            || model.zones[0].spaces.as_slice() != [model.spaces[0].id]
            || model.spaces[0].origin != ep_model::SpaceOrigin::AutoZoneDefault
        {
            return Err(OrdinaryRadiationUnavailable::EnclosureDomain.into());
        }
        if let Some(rows) = &mut self.solar {
            if self.num_solar_enclosures != 1 || rows.len() != 1 {
                return Err(OrdinaryRadiationUnavailable::EnclosureDomain.into());
            }
            let enclosure = &mut rows[0];
            if enclosure.space_nums != [1]
                || enclosure.zone_id.is_some()
                || !enclosure.surface_indexes.is_empty()
            {
                return Err(OrdinaryRadiationUnavailable::EnclosureDomain.into());
            }
            // Enrich this same early Source row. No Name/Space metadata,
            // scalar totals or EnclSolInfo allocation is replaced here.
            initialization::bind_enclosure_geometry(enclosure, model, surfaces, &layout)?;
        } else {
            // Legacy callers retain their previous constructor and numeric path;
            // absence of early Source797 does not claim its metadata writes.
            self.solar = Some(vec![initialization::allocate_enclosure(
                model, surfaces, &layout,
            )?]);
        }
        self.back_pointers = vec![(0, 0, 0); surfaces.len()];
        Ok(())
    }
}
