//! Consumed solar initialization prefix; ordinary interior exchange remains held.
//!
//! Original HeatBalanceIntRadExchange.cc811-1103. No
//! diagnostic probe supplies a caller flag, iteration count or cache result.

mod initialization;

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
}
impl std::fmt::Display for OrdinaryRadiationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unavailable(error) => std::fmt::Display::fmt(error, f),
            Self::Output(error) => write!(f, "ordinary radiation output stopped: {error}"),
        }
    }
}
impl std::error::Error for OrdinaryRadiationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Unavailable(error) => Some(error),
            Self::Output(error) => Some(error),
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

/// Actual selected solar enclosure, available through immutable owner inspection.
#[derive(Clone, Debug, PartialEq)]
pub struct SolarRadiationEnclosure {
    /// One-based solar enclosure index.
    pub id: usize,
    /// Actual implicit-Space enclosure name.
    pub name: String,
    /// Actual owning Zone identity.
    pub zone_id: ZoneId,
    /// Actual member Space identities.
    pub space_ids: Vec<SpaceId>,
    /// Actual Wall/Floor/Roof traversal for both F axes.
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
}

/// Solar prefix retained on error by CtfInitializationReports, then moved into
/// HeatBalanceState on success. It supplies no radiant cache or caller flags.
#[derive(Clone, Debug, PartialEq)]
pub struct OrdinarySolarRadiation {
    input: OrdinaryRadiationInput,
    solar: Vec<SolarRadiationEnclosure>,
    // Surface.SolarEnclIndex, RadEnclIndex, SolarEnclSurfIndex, one-based.
    // Original InitSolar writes these associations even before radiant init.
    back_pointers: Vec<(usize, usize, usize)>,
    solar_initialized: bool,
}

impl OrdinarySolarRadiation {
    /// Retained actual solar enclosure storage and reached numeric prefix.
    #[must_use]
    pub fn enclosures(&self) -> &[SolarRadiationEnclosure] {
        &self.solar
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
        let enclosure = initialization::allocate_enclosure(model, surfaces, &layout)?;
        Ok(Self {
            input,
            solar: vec![enclosure],
            back_pointers: vec![(0, 0, 0); surfaces.len()],
            solar_initialized: false,
        })
    }
}
