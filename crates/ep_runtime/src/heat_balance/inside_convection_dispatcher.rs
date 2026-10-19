//! Retained selected InitIntConvCoeff owners and its genuine no-user setup.
//!
//! This shared entry does not install ordinary timestep cadence. Callers supply
//! validated identities, separate space MAT, actual static geometry, readiness,
//! full named temperature owner and the existing borrowed report/error backend.

mod geometry;
pub(crate) mod source;
use super::convection::tarp_inside::{
    detailed_inside_coefficient_w_per_m2_k, natural_coefficient_w_per_m2_k,
};
use super::error_backend::{SourceErrorBackend, SourceErrorBackendFailure};
use super::inside_convection_layout::{
    InsideConvectionAssociationUnavailable, InsideConvectionLayout,
};
use super::report_scan::{
    ReportScanDiagnostic, ReportScanFailure, ReportScanState, ReportScanUnavailable,
    try_scan_for_reports,
};
use ep_model::{ReportInputs, SpaceId, SurfaceId, SurfaceType, ZoneId};
pub use geometry::ConvectionGeometrySummary;
use std::collections::BTreeMap;

/// A missing or unsupported actual owner; never an invented Native outcome.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InsideConvectionUnavailable {
    /// Duplicate/missing/stale zone, space or surface association.
    Association,
    /// Static metadata inputs are outside the represented finite geometry domain.
    Geometry,
    /// One of the six actual user-convection object counts is nonzero.
    UserConvectionInput,
    /// Selected opaque TARP entry cannot represent this actual algorithm/branch.
    AlgorithmOrSurface,
    /// Genuine Details reporting was reached; its report writer is not ported here.
    SurfaceDetailsReport,
    /// A full temperature owner differs in membership or order from its layout.
    TemperatureOwner,
}
impl std::fmt::Display for InsideConvectionUnavailable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for InsideConvectionUnavailable {}
impl From<InsideConvectionAssociationUnavailable> for InsideConvectionUnavailable {
    fn from(_: InsideConvectionAssociationUnavailable) -> Self {
        Self::Association
    }
}

/// Retain actual diagnostic failure causes and reached report-cache prefixes.
#[derive(Debug)]
pub enum InsideConvectionError {
    /// A required selected owner or branch is unavailable.
    Unavailable(InsideConvectionUnavailable),
    /// Existing ScanForReports rejected an actual call argument.
    Scan(ReportScanUnavailable),
    /// The genuine borrowed backend stopped at this actual diagnostic.
    Backend(SourceErrorBackendFailure),
}
impl From<InsideConvectionUnavailable> for InsideConvectionError {
    fn from(value: InsideConvectionUnavailable) -> Self {
        Self::Unavailable(value)
    }
}
impl std::fmt::Display for InsideConvectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unavailable(e) => write!(f, "inside convection owner unavailable: {e:?}"),
            Self::Scan(e) => write!(f, "inside convection report scan unavailable: {e:?}"),
            Self::Backend(e) => std::fmt::Display::fmt(e, f),
        }
    }
}
impl std::error::Error for InsideConvectionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Backend(e) => Some(e),
            _ => None,
        }
    }
}

/// Six real InputProcessor getNumObjectsFound results, in literal Source order.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NoUserConvectionInput {
    counts: [usize; 6],
}
impl NoUserConvectionInput {
    /// Family names queried by the real no-user admission.
    pub const FAMILIES: [&'static str; 6] = [
        "SurfaceConvectionAlgorithm:Inside:UserCurve",
        "SurfaceConvectionAlgorithm:Outside:UserCurve",
        "SurfaceProperty:ConvectionCoefficients:MultipleSurface",
        "SurfaceProperty:ConvectionCoefficients",
        "SurfaceConvectionAlgorithm:Inside:AdaptiveModelSelections",
        "SurfaceConvectionAlgorithm:Outside:AdaptiveModelSelections",
    ];
    /// Admit actual getter results; neither an empty typed model nor absent HVAC
    /// proves these counts. Raw-input callers must supply all six real counts.
    pub fn new(counts: [usize; 6]) -> Result<Self, InsideConvectionUnavailable> {
        if counts != [0; 6] {
            return Err(InsideConvectionUnavailable::UserConvectionInput);
        }
        Ok(Self { counts })
    }
    /// The retained admitted input facts, not result flags.
    #[must_use]
    pub fn counts(&self) -> [usize; 6] {
        self.counts
    }
}

/// Independently retained selected SpaceHeatBalanceData fields.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpaceAirTemperature {
    /// Actual space identity.
    pub space_id: SpaceId,
    /// Actual owning zone.
    pub zone_id: ZoneId,
    /// Space MAT read by InitIntConvCoeff, distinct from zone MAT.
    pub mat: f64,
    /// Space ZT, not an alias of MAT.
    pub zt: f64,
    /// Space ZTAV, not an alias of MAT.
    pub ztav: f64,
}

/// One live borrowed Space air record. Temperature values remain in the sole
/// caller owner; this view retains references rather than numeric copies.
#[derive(Clone, Copy, Debug)]
pub struct SpaceAirTemperatureBorrow<'a> {
    /// Actual Space identity from the same Source metadata owner.
    pub space_id: SpaceId,
    /// Actual owning Zone identity.
    pub zone_id: ZoneId,
    /// Actual MAT member, borrowed through the next scalar coefficient read.
    pub mat: &'a f64,
    /// Actual independent ZT member.
    pub zt: &'a f64,
    /// Actual independent ZTAV member.
    pub ztav: &'a f64,
}

/// Source-connected and existing callers share the same convection body while
/// retaining their own actual live storage. No mutable temperature copy exists.
pub trait SpaceAirTemperatureOwners {
    /// Actual allocated Space row count, or unavailable before allocation.
    fn allocated_space_count(&self) -> Result<usize, InsideConvectionUnavailable>;
    /// Borrow one actual row in the retained Space array order.
    fn borrow_space_at(
        &self,
        index: usize,
    ) -> Result<SpaceAirTemperatureBorrow<'_>, InsideConvectionUnavailable>;

    /// Validate the same complete ordered identity closure used by this entry.
    fn validate_space_layout(
        &self,
        layout: &InsideConvectionLayout,
    ) -> Result<(), InsideConvectionUnavailable> {
        if self.allocated_space_count()? != layout.spaces().len() {
            return Err(InsideConvectionUnavailable::Association);
        }
        for (index, &(space, zone)) in layout.spaces().iter().enumerate() {
            let row = self.borrow_space_at(index)?;
            if (row.space_id, row.zone_id) != (space, zone) {
                return Err(InsideConvectionUnavailable::Association);
            }
        }
        Ok(())
    }

    /// Resolve one actual retained identity without a numeric registry/cache.
    fn borrow_space_for_ids(
        &self,
        space: SpaceId,
        zone: ZoneId,
    ) -> Result<SpaceAirTemperatureBorrow<'_>, InsideConvectionUnavailable> {
        for index in 0..self.allocated_space_count()? {
            let row = self.borrow_space_at(index)?;
            if (row.space_id, row.zone_id) == (space, zone) {
                return Ok(row);
            }
        }
        Err(InsideConvectionUnavailable::Association)
    }
}

/// Persistent selected space-air owner with strict full-array correspondence.
#[derive(Clone, Debug, PartialEq)]
pub struct SpaceAirTemperatures {
    rows: Vec<SpaceAirTemperature>,
}
impl SpaceAirTemperatureOwners for SpaceAirTemperatures {
    fn allocated_space_count(&self) -> Result<usize, InsideConvectionUnavailable> {
        Ok(self.rows.len())
    }
    fn borrow_space_at(
        &self,
        index: usize,
    ) -> Result<SpaceAirTemperatureBorrow<'_>, InsideConvectionUnavailable> {
        let row = self
            .rows
            .get(index)
            .ok_or(InsideConvectionUnavailable::Association)?;
        Ok(SpaceAirTemperatureBorrow {
            space_id: row.space_id,
            zone_id: row.zone_id,
            mat: &row.mat,
            zt: &row.zt,
            ztav: &row.ztav,
        })
    }
}
impl SpaceAirTemperatures {
    /// Real selected constructor values; not a zone-temperature fallback.
    #[must_use]
    pub fn allocate(layout: &InsideConvectionLayout) -> Self {
        Self {
            rows: layout
                .spaces()
                .iter()
                .map(|&(space_id, zone_id)| SpaceAirTemperature {
                    space_id,
                    zone_id,
                    mat: 23.0,
                    zt: 23.0,
                    ztav: 23.0,
                })
                .collect(),
        }
    }
    /// Read the actual persistent selected array, in space array order.
    #[must_use]
    pub fn rows(&self) -> &[SpaceAirTemperature] {
        &self.rows
    }
    /// Write independent declared input owners without reconstructing a zone alias.
    pub fn write_all(
        &mut self,
        layout: &InsideConvectionLayout,
        rows: &[SpaceAirTemperature],
    ) -> Result<(), InsideConvectionUnavailable> {
        if rows.len() != layout.spaces().len()
            || rows
                .iter()
                .zip(layout.spaces())
                .any(|(a, b)| (a.space_id, a.zone_id) != *b)
        {
            return Err(InsideConvectionUnavailable::Association);
        }
        self.rows.clone_from_slice(rows);
        Ok(())
    }
    /// Genuine selected BeginEnvrn reconstruction, including spaces when space
    /// simulation is disabled. This does not claim other SpaceHeatBalance fields.
    pub fn reset_begin_environment(
        &mut self,
        layout: &InsideConvectionLayout,
    ) -> Result<(), InsideConvectionUnavailable> {
        self.validate(layout)?;
        for row in &mut self.rows {
            row.mat = 23.0;
            row.zt = 23.0;
            row.ztav = 23.0;
        }
        Ok(())
    }
    /// Actual disabled-space/sizing selected ZT/MAT copy after zone correction.
    /// No ZTAV write occurs at Original3843–3847; its previous owner is retained.
    pub fn copy_zone_after_correction(
        &mut self,
        layout: &InsideConvectionLayout,
        zone_id: ZoneId,
        zone_zt: f64,
        zone_mat: f64,
    ) -> Result<(), InsideConvectionUnavailable> {
        self.validate(layout)?;
        if !layout.zones().contains(&zone_id) {
            return Err(InsideConvectionUnavailable::Association);
        }
        for group in layout.groups().iter().filter(|g| g.zone_id == zone_id) {
            let row = self
                .rows
                .iter_mut()
                .find(|r| r.space_id == group.space_id)
                .ok_or(InsideConvectionUnavailable::Association)?;
            row.zt = zone_zt;
            row.mat = zone_mat;
        }
        Ok(())
    }
    /// Snapshot the actual shared SpaceMAT for the ordinary inside-balance entry.
    pub(crate) fn reference_air_by_layout(
        &self,
        layout: &InsideConvectionLayout,
    ) -> Result<BTreeMap<SurfaceId, f64>, InsideConvectionUnavailable> {
        let rows = self.rows();
        if rows.len() != layout.spaces().len()
            || rows
                .iter()
                .zip(layout.spaces())
                .any(|(row, &(space, zone))| (row.space_id, row.zone_id) != (space, zone))
        {
            return Err(InsideConvectionUnavailable::Association);
        }
        layout
            .surfaces()
            .iter()
            .map(|surface| {
                let row = rows
                    .iter()
                    .find(|row| row.space_id == surface.space_id && row.zone_id == surface.zone_id)
                    .ok_or(InsideConvectionUnavailable::Association)?;
                Ok((surface.surface_id, row.mat))
            })
            .collect()
    }

    fn validate(&self, layout: &InsideConvectionLayout) -> Result<(), InsideConvectionUnavailable> {
        if self.rows.len() != layout.spaces().len()
            || self
                .rows
                .iter()
                .zip(layout.spaces())
                .any(|(a, b)| (a.space_id, a.zone_id) != *b)
        {
            return Err(InsideConvectionUnavailable::Association);
        }
        Ok(())
    }
}

/// Selected zone static owners already produced by the actual loader/caller.
#[derive(Clone, Debug, PartialEq)]
pub struct InsideConvectionZone {
    /// Actual zone identity.
    pub zone_id: ZoneId,
    /// Actual IntConvAlgo; only TARP is admitted by this selected entry.
    pub tarp: bool,
    /// Actual CeilingHeight.
    pub ceiling_height: f64,
    /// Actual Volume.
    pub volume: f64,
    /// Actual FloorArea.
    pub floor_area: f64,
    /// Actual MinimumX/MaximumX/MinimumY/MaximumY.
    pub xy_bounds: [f64; 4],
    /// Actual ExtGrossWallArea, not a reconstruction from selected surfaces.
    pub exterior_gross_wall_area: f64,
    /// Actual ExtWindowArea; zero is required for this opaque selected branch.
    pub exterior_window_area: f64,
    /// Actual Multiplier.
    pub multiplier: i32,
    /// Actual ListMultiplier.
    pub list_multiplier: i32,
}

/// Actual inside algorithm selection, before SetByZone is resolved.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InsideConvectionModel {
    /// Preserve actual surface default; resolve the owning zone at the call.
    SetByZone,
    /// Explicit ASHRAETARP assignment.
    Tarp,
}
impl InsideConvectionModel {
    /// Literal Convect::HcInt enum identity for retained owner observation.
    #[must_use]
    pub const fn source_id(self) -> i32 {
        match self {
            Self::SetByZone => 2,
            Self::Tarp => 4,
        }
    }
}

/// Actual selected opaque surface static owners; values are never re-derived
/// from a temperature view, Native result, or selected coefficient.
#[derive(Clone, Debug, PartialEq)]
pub struct InsideConvectionSurface {
    /// Actual surface ID and independent Zone/spaceNum are in the layout.
    pub surface_id: SurfaceId,
    /// Actual base class, including Ceiling represented as Roof for this entry.
    pub class: SurfaceType,
    /// Actual exterior-boundary predicate.
    pub exterior: bool,
    /// Actual HeatTransSurf predicate.
    pub heat_transfer: bool,
    /// Actual construction TypeIsWindow predicate; unsupported if true.
    pub construction_is_window: bool,
    /// Actual Kiva boundary predicate; unsupported if true.
    pub kiva: bool,
    /// Actual representative calculation predicate; unsupported if true.
    pub representative_calculations: bool,
    /// Actual EMS override flag; unsupported if true.
    pub ems_override: bool,
    /// Actual CosTilt; passed unchanged to the existing detailed kernel.
    pub outward_cos_tilt: f64,
    /// Actual Tilt.
    pub tilt: f64,
    /// Actual Azimuth.
    pub azimuth: f64,
    /// Actual Area.
    pub area: f64,
    /// Actual GrossArea.
    pub gross_area: f64,
    /// Actual Width.
    pub width: f64,
    /// Actual Perimeter.
    pub perimeter: f64,
    /// Actual ordered Vertex array.
    pub vertices: Vec<[f64; 3]>,
}

/// Retained selected SurfIntConv/SurfExtConv metadata owners.
/// Unwritten classification/model-equation members remain independently stored.
#[derive(Clone, Debug, PartialEq)]
pub struct InsideConvectionSurfaceState {
    /// Actual surface identity, in actual surface array order.
    pub surface_id: SurfaceId,
    /// Retained model, not overwritten by local SetByZone resolution.
    pub model: InsideConvectionModel,
    /// SurfTAirRef and SurfTAirRefRpt, unchanged by selected TARP.
    pub reference_air: [i32; 2],
    /// SurfIntConv zoneWallHeight/zonePerimLength/zoneHorizHydrDiam/windowWallRatio.
    pub interior_geometry: [f64; 4],
    /// SurfIntConv windowLocation, retained NotSet in no-window domain.
    pub window_location: i32,
    /// SurfIntConv getsRadiantHeat and hasActiveInIt.
    pub radiant_and_active: [bool; 2],
    /// Unwritten SurfIntConv convClass/convClassRpt/userModelNum/hcModelEq/hcModelEqRpt/hcUserCurveNum.
    pub interior_classification: [i32; 6],
    /// SurfExtConv faceArea/facePerimeter/faceHeight.
    pub exterior_geometry: [f64; 3],
    /// Unwritten SurfExtConv convClass/convClassRpt/model/userModelNum/hfModelEq/hfModelEqRpt/hfUserCurveNum/hnModelEq/hnModelEqRpt/hnUserCurveNum.
    pub exterior_classification: [i32; 10],
}
impl InsideConvectionSurfaceState {
    /// Real selected surfIntConv/surfExtConv defaults plus actual caller reference air.
    #[must_use]
    pub fn new(surface_id: SurfaceId, reference_air: [i32; 2]) -> Self {
        Self {
            surface_id,
            model: InsideConvectionModel::SetByZone,
            reference_air,
            interior_geometry: [0.0; 4],
            window_location: 0,
            radiant_and_active: [false; 2],
            interior_classification: [-1, -1, 0, -1, -1, 0],
            exterior_geometry: [0.0; 3],
            exterior_classification: [-1, -1, 2, 0, -1, -1, 0, -1, -1, 0],
        }
    }
}

/// No occupied user record is representable in the selected no-user domain.
#[derive(Clone, Debug, PartialEq)]
pub enum AbsentUserConvectionRecord {}

/// Borrow actual ordered ZoneEquipConfig active-element members from their owner.
/// The dispatcher validates full cardinality and identity before any write.
pub trait ZoneEquipmentActiveElements {
    /// Number of retained Config rows in actual zone array order.
    fn zone_count(&self) -> usize;
    /// Actual zone identity at an already validated row index.
    fn zone_id(&self, index: usize) -> ZoneId;
    /// Current wall, ceiling and floor members; no persistent projection.
    fn active_elements(&self, index: usize) -> [bool; 3];
    /// Write the actual retained row after full membership validation.
    fn set_active_elements(&mut self, index: usize, flags: [bool; 3]);
}

impl ZoneEquipmentActiveElements for [(ZoneId, [bool; 3])] {
    fn zone_count(&self) -> usize {
        self.len()
    }
    fn zone_id(&self, index: usize) -> ZoneId {
        self[index].0
    }
    fn active_elements(&self, index: usize) -> [bool; 3] {
        self[index].1
    }
    fn set_active_elements(&mut self, index: usize, flags: [bool; 3]) {
        self[index].1 = flags;
    }
}

/// Borrow the caller's sole SurfHConvInt storage in actual surface array order.
/// Complete identity validation precedes all dispatcher/setup/backend writes.
pub trait InsideConvectionCoefficients {
    /// Number of actual coefficient rows, including the full surface domain.
    fn surface_count(&self) -> usize;
    /// Actual surface identity at an already validated array index.
    fn surface_id(&self, index: usize) -> SurfaceId;
    /// Commit the reached result directly to its caller-owned coefficient.
    fn set_coefficient(&mut self, index: usize, value: f64);
    /// Borrow the actual cell for genuine raw-write/clamp/read ordering.
    /// Existing custom legacy owners remain supported by the legacy entry;
    /// a Source caller without a real live borrow fails explicitly.
    fn borrow_coefficient(&self, _index: usize) -> Result<&f64, InsideConvectionUnavailable> {
        Err(InsideConvectionUnavailable::Association)
    }
}

impl InsideConvectionCoefficients for [(SurfaceId, f64)] {
    fn surface_count(&self) -> usize {
        self.len()
    }
    fn surface_id(&self, index: usize) -> SurfaceId {
        self[index].0
    }
    fn set_coefficient(&mut self, index: usize, value: f64) {
        self[index].1 = value;
    }
    fn borrow_coefficient(&self, index: usize) -> Result<&f64, InsideConvectionUnavailable> {
        self.get(index)
            .map(|row| &row.1)
            .ok_or(InsideConvectionUnavailable::Association)
    }
}

impl InsideConvectionCoefficients for [super::state::SurfaceHeatBalanceState] {
    fn surface_count(&self) -> usize {
        self.len()
    }
    fn surface_id(&self, index: usize) -> SurfaceId {
        self[index].surface_id
    }
    fn set_coefficient(&mut self, index: usize, value: f64) {
        self[index].inside_convection_coefficient_w_per_m2_k = value;
    }
    fn borrow_coefficient(&self, index: usize) -> Result<&f64, InsideConvectionUnavailable> {
        self.get(index)
            .map(|row| &row.inside_convection_coefficient_w_per_m2_k)
            .ok_or(InsideConvectionUnavailable::Association)
    }
}

/// Persistent genuine first-call/setup flags and selected surface metadata owners.
#[derive(Clone, Debug, PartialEq)]
pub struct InsideConvectionState {
    layout: InsideConvectionLayout,
    /// Actual ordered convection metadata; HConv remains caller-owned.
    pub surfaces: Vec<InsideConvectionSurfaceState>,
    /// GetUserSuppliedConvectionCoeffs, constructor true.
    pub get_user_input: bool,
    /// NodeCheck, cleared only when genuine readiness conditions are met.
    pub node_check: bool,
    /// ActiveSurfaceCheck, cleared only after the actual radiant setup call.
    pub active_surface_check: bool,
    /// MyEnvirnFlag, constructor true; actual BeginEnvrn/reset transition.
    pub my_environment: bool,
    /// FirstRoofSurf, untouched by this selected caller.
    pub first_roof_surface: bool,
    /// CubeRootOfOverallBuildingVolume.
    pub cube_root_building_volume: f64,
    /// RoofLongAxisOutwardAzimuth.
    pub roof_long_axis_outward_azimuth: f64,
    /// The four actual allocated zero-length families. None is unallocated.
    pub user_arrays: [Option<Vec<AbsentUserConvectionRecord>>; 4],
    /// TotUserIntConvModels and TotUserExtConvModels.
    pub user_model_counts: [usize; 2],
}
impl InsideConvectionState {
    /// Genuine module constructor with metadata retained solely in Source geometry.
    /// The empty legacy Vec is not an allocated Native per-Surface owner.
    pub(crate) fn new_source_control(layout: &InsideConvectionLayout) -> Self {
        Self {
            layout: layout.clone(),
            surfaces: Vec::new(),
            get_user_input: true,
            node_check: true,
            active_surface_check: true,
            my_environment: true,
            first_roof_surface: true,
            cube_root_building_volume: 0.0,
            roof_long_axis_outward_azimuth: 0.0,
            user_arrays: std::array::from_fn(|_| None),
            user_model_counts: [0; 2],
        }
    }
    /// Retain actual dispatcher seeds; HConv, SpaceAir and Config remain caller owners.
    pub fn new(
        layout: &InsideConvectionLayout,
        surfaces: Vec<InsideConvectionSurfaceState>,
    ) -> Result<Self, InsideConvectionUnavailable> {
        if surfaces.len() != layout.surfaces().len()
            || surfaces
                .iter()
                .zip(layout.surfaces())
                .any(|(a, b)| a.surface_id != b.surface_id)
        {
            return Err(InsideConvectionUnavailable::Association);
        }
        Ok(Self {
            layout: layout.clone(),
            surfaces,
            get_user_input: true,
            node_check: true,
            active_surface_check: true,
            my_environment: true,
            first_roof_surface: true,
            cube_root_building_volume: 0.0,
            roof_long_axis_outward_azimuth: 0.0,
            user_arrays: std::array::from_fn(|_| None),
            user_model_counts: [0; 2],
        })
    }
}

/// Distinct actual source array chosen by the caller, never a fallback override.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InsideSurfaceTemperatureOwner {
    /// Initial InitIntConvCoeff operand.
    SurfTempInTmp,
    /// Inside-iteration refresh operand.
    SurfTempIn,
}
/// Full borrowed source temperature array with explicit IDs/order.
#[derive(Clone, Copy, Debug)]
pub struct InsideSurfaceTemperatures<'a> {
    /// Actual source owner identity, retained for caller observation.
    pub owner: InsideSurfaceTemperatureOwner,
    /// Every actual surface ID and its current value in surface array order.
    pub rows: &'a [(SurfaceId, f64)],
}

/// Actual readiness/phase operands; no condition is inferred from fixture type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InsideConvectionReadiness {
    /// BeginEnvrnFlag.
    pub begin_environment: bool,
    /// SysSizingCalc.
    pub system_sizing: bool,
    /// ZoneSizingCalc.
    pub zone_sizing: bool,
    /// ZoneEquipInputsFilled.
    pub equipment_inputs_filled: bool,
    /// allocated(Node), independent of NumOfNodes.
    pub nodes_allocated: bool,
    /// ZoneEquipSimulatedOnce.
    pub equipment_simulated_once: bool,
}

/// Borrow actual Source per-Surface convection records without a mutable mirror.
pub trait InsideConvectionSurfaceMetadata {
    /// Actual allocated complete Surface metadata count.
    fn metadata_count(&self) -> Result<usize, InsideConvectionUnavailable>;
    /// Actual Surface identity at this Source array position.
    fn metadata_id(&self, index: usize) -> Result<SurfaceId, InsideConvectionUnavailable>;
    /// Actual SurfIntConv.model, preserving local SetByZone resolution.
    fn model(&self, index: usize) -> Result<InsideConvectionModel, InsideConvectionUnavailable>;
    /// Actual SurfIntConv.hasActiveInIt.
    fn has_active_element(&self, index: usize) -> Result<bool, InsideConvectionUnavailable>;
    /// Write the four reached SurfIntConv members in Source assignment order.
    fn set_interior_geometry(
        &mut self,
        index: usize,
        values: [f64; 4],
    ) -> Result<(), InsideConvectionUnavailable>;
    /// Write the three reached SurfExtConv members in Source assignment order.
    fn set_exterior_geometry(
        &mut self,
        index: usize,
        values: [f64; 3],
    ) -> Result<(), InsideConvectionUnavailable>;
}
impl InsideConvectionSurfaceMetadata for [InsideConvectionSurfaceState] {
    fn metadata_count(&self) -> Result<usize, InsideConvectionUnavailable> {
        Ok(self.len())
    }
    fn metadata_id(&self, index: usize) -> Result<SurfaceId, InsideConvectionUnavailable> {
        self.get(index)
            .map(|r| r.surface_id)
            .ok_or(InsideConvectionUnavailable::Association)
    }
    fn model(&self, index: usize) -> Result<InsideConvectionModel, InsideConvectionUnavailable> {
        self.get(index)
            .map(|r| r.model)
            .ok_or(InsideConvectionUnavailable::Association)
    }
    fn has_active_element(&self, index: usize) -> Result<bool, InsideConvectionUnavailable> {
        self.get(index)
            .map(|r| r.radiant_and_active[1])
            .ok_or(InsideConvectionUnavailable::Association)
    }
    fn set_interior_geometry(
        &mut self,
        index: usize,
        values: [f64; 4],
    ) -> Result<(), InsideConvectionUnavailable> {
        self.get_mut(index)
            .ok_or(InsideConvectionUnavailable::Association)?
            .interior_geometry = values;
        Ok(())
    }
    fn set_exterior_geometry(
        &mut self,
        index: usize,
        values: [f64; 3],
    ) -> Result<(), InsideConvectionUnavailable> {
        self.get_mut(index)
            .ok_or(InsideConvectionUnavailable::Association)?
            .exterior_geometry = values;
        Ok(())
    }
}

/// Explicit full temperature operand borrowed through the scalar kernel read.
pub trait InsideSurfaceTemperatureValues {
    /// Actual caller selection, never a fallback between the two Source arrays.
    fn temperature_owner(&self) -> InsideSurfaceTemperatureOwner;
    /// Actual complete allocated temperature length.
    fn temperature_count(&self) -> usize;
    /// Actual Surface identity in the same array order.
    fn temperature_id(&self, index: usize) -> SurfaceId;
    /// Borrow the sole actual temperature cell.
    fn borrow_temperature(&self, index: usize) -> Result<&f64, InsideConvectionUnavailable>;
}
impl InsideSurfaceTemperatureValues for InsideSurfaceTemperatures<'_> {
    fn temperature_owner(&self) -> InsideSurfaceTemperatureOwner {
        self.owner
    }
    fn temperature_count(&self) -> usize {
        self.rows.len()
    }
    fn temperature_id(&self, index: usize) -> SurfaceId {
        self.rows[index].0
    }
    fn borrow_temperature(&self, index: usize) -> Result<&f64, InsideConvectionUnavailable> {
        self.rows
            .get(index)
            .map(|r| &r.1)
            .ok_or(InsideConvectionUnavailable::TemperatureOwner)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ConvectionEntry {
    Legacy,
    Source,
}

fn validate<
    C: ZoneEquipmentActiveElements + ?Sized,
    H: InsideConvectionCoefficients + ?Sized,
    S: SpaceAirTemperatureOwners + ?Sized,
    M: InsideConvectionSurfaceMetadata + ?Sized,
    T: InsideSurfaceTemperatureValues + ?Sized,
>(
    layout: &InsideConvectionLayout,
    zones: &[InsideConvectionZone],
    surfaces: &[InsideConvectionSurface],
    state: &InsideConvectionState,
    zone_equipment: &C,
    coefficients: &H,
    space_air: &S,
    temperatures: &T,
    metadata: &M,
    entry: ConvectionEntry,
) -> Result<(), InsideConvectionUnavailable> {
    space_air.validate_space_layout(layout)?;
    if state.layout != *layout
        || coefficients.surface_count() != layout.surfaces().len()
        || layout
            .surfaces()
            .iter()
            .enumerate()
            .any(|(index, row)| coefficients.surface_id(index) != row.surface_id)
        || zones
            .iter()
            .map(|r| r.zone_id)
            .ne(layout.zones().iter().copied())
        || (entry == ConvectionEntry::Legacy
            && (zone_equipment.zone_count() != layout.zones().len()
                || layout
                    .zones()
                    .iter()
                    .enumerate()
                    .any(|(index, &zone)| zone_equipment.zone_id(index) != zone)))
        || surfaces
            .iter()
            .map(|r| r.surface_id)
            .ne(layout.surfaces().iter().map(|r| r.surface_id))
        || metadata.metadata_count()? != layout.surfaces().len()
    {
        return Err(InsideConvectionUnavailable::Association);
    }
    for (index, row) in layout.surfaces().iter().enumerate() {
        if metadata.metadata_id(index)? != row.surface_id {
            return Err(InsideConvectionUnavailable::Association);
        }
    }
    if temperatures.temperature_count() != layout.surfaces().len()
        || layout
            .surfaces()
            .iter()
            .enumerate()
            .any(|(index, row)| temperatures.temperature_id(index) != row.surface_id)
    {
        return Err(InsideConvectionUnavailable::TemperatureOwner);
    }
    validate_static_geometry(zones, surfaces)
}

fn validate_static_geometry(
    zones: &[InsideConvectionZone],
    surfaces: &[InsideConvectionSurface],
) -> Result<(), InsideConvectionUnavailable> {
    for zone in zones {
        let [xmin, xmax, ymin, ymax] = zone.xy_bounds;
        if !zone.tarp {
            return Err(InsideConvectionUnavailable::AlgorithmOrSurface);
        }
        if ![
            zone.ceiling_height,
            zone.volume,
            zone.floor_area,
            xmin,
            xmax,
            ymin,
            ymax,
            zone.exterior_gross_wall_area,
            zone.exterior_window_area,
        ]
        .iter()
        .all(|v| v.is_finite())
            || zone.ceiling_height <= 0.0
            || zone.volume <= 0.0
            || zone.floor_area <= 0.0
            || xmax <= xmin
            || ymax <= ymin
            || zone.multiplier <= 0
            || zone.list_multiplier <= 0
            || zone.exterior_gross_wall_area < 0.0
            || zone.exterior_window_area != 0.0
        {
            return Err(InsideConvectionUnavailable::Geometry);
        }
    }
    for surface in surfaces {
        if !surface.heat_transfer
            || surface.construction_is_window
            || surface.kiva
            || surface.representative_calculations
            || surface.ems_override
        {
            return Err(InsideConvectionUnavailable::AlgorithmOrSurface);
        }
        if ![
            surface.outward_cos_tilt,
            surface.tilt,
            surface.azimuth,
            surface.area,
            surface.gross_area,
            surface.width,
            surface.perimeter,
        ]
        .iter()
        .all(|v| v.is_finite())
            || !(-1.0..=1.0).contains(&surface.outward_cos_tilt)
            || !(0.0..=180.0).contains(&surface.tilt)
            || !(0.0..360.0).contains(&surface.azimuth)
            || surface.area <= 0.0
            || surface.gross_area <= 0.0
            || surface.width <= 0.0
            || surface.perimeter <= 0.0
            || surface.vertices.len() < 3
            || surface.vertices.iter().flatten().any(|v| !v.is_finite())
        {
            return Err(InsideConvectionUnavailable::Geometry);
        }
        if surface.exterior
            && surface.tilt <= 45.0
            && surface.vertices.iter().enumerate().any(|(i, p)| {
                let next = surface.vertices[(i + 1) % surface.vertices.len()];
                let t = std::array::from_fn::<_, 3, _>(|axis| next[axis] - p[axis]);
                ((t[0] * t[0]) + (t[1] * t[1]) + (t[2] * t[2])).sqrt() == 0.0
            })
        {
            // Original normalize asserts computed length != 0, including underflow.
            // A computed +Inf length remains admitted and follows Source IEEE operations.
            return Err(InsideConvectionUnavailable::Geometry);
        }
    }
    Ok(())
}

fn setup_static<M: InsideConvectionSurfaceMetadata + ?Sized>(
    layout: &InsideConvectionLayout,
    zones: &[InsideConvectionZone],
    surfaces: &[InsideConvectionSurface],
    state: &mut InsideConvectionState,
    metadata: &mut M,
) -> Result<(), InsideConvectionUnavailable> {
    let mut building_volume = 0.0;
    // Native declares this one local before the entire Zone loop. If neither
    // branch assigns it, keep its previous value (or genuinely unrepresented).
    let mut diameter = None;
    for zone in zones {
        building_volume +=
            zone.volume * f64::from(zone.multiplier) * f64::from(zone.list_multiplier);
        let mut perimeter = 0.0;
        let [xmin, xmax, ymin, ymax] = zone.xy_bounds;
        let simple_perimeter = 2.0 * (ymax - ymin) + 2.0 * (xmax - xmin);
        if simple_perimeter > 0.0 {
            diameter = Some(4.0 * zone.floor_area / simple_perimeter);
        } else if zone.floor_area > 0.0 {
            diameter = Some(zone.floor_area.sqrt());
        }
        let ratio = if zone.exterior_gross_wall_area > 0.0 {
            zone.exterior_window_area / zone.exterior_gross_wall_area
        } else {
            -999.0
        };
        for group in layout.groups().iter().filter(|g| g.zone_id == zone.zone_id) {
            for id in &group.surfaces {
                let surface = surfaces
                    .iter()
                    .find(|r| r.surface_id == *id)
                    .ok_or(InsideConvectionUnavailable::Association)?;
                if surface.exterior && surface.class == SurfaceType::Wall {
                    perimeter += surface.width;
                }
            }
        }
        for group in layout.groups().iter().filter(|g| g.zone_id == zone.zone_id) {
            for id in &group.surfaces {
                let index = layout
                    .surfaces()
                    .iter()
                    .position(|r| r.surface_id == *id)
                    .ok_or(InsideConvectionUnavailable::Association)?;
                metadata.set_interior_geometry(
                    index,
                    [
                        zone.ceiling_height,
                        perimeter,
                        diameter.ok_or(InsideConvectionUnavailable::Geometry)?,
                        ratio,
                    ],
                )?;
            }
        }
        // ExtWindowCount is genuinely zero in this admitted opaque domain.
    }
    state.cube_root_building_volume = building_volume.powf(1.0 / 3.0);
    let roof = geometry::roof(surfaces);
    state.roof_long_axis_outward_azimuth = roof.azimuth;
    let facades = geometry::facades(surfaces)?;
    for (index, surface) in surfaces.iter().enumerate() {
        if !surface.exterior || !surface.heat_transfer {
            continue;
        }
        // This distinct extrema loop retains Original's else-if ordering.
        let mut low = f64::MAX;
        let mut high = -f64::MAX;
        for p in &surface.vertices {
            if p[2] < low {
                low = p[2];
            } else if p[2] > high {
                high = p[2];
            }
        }
        let height = high - low;
        let exterior_geometry = if surface.tilt >= 45.0 && surface.tilt < 135.0 {
            let facade = facades[geometry::compass(surface.azimuth)?];
            [
                geometry::source_max(facade.area, surface.gross_area),
                geometry::source_max(facade.perimeter, surface.perimeter),
                geometry::source_max(facade.height, height),
            ]
        } else if surface.tilt < 45.0 {
            [
                geometry::source_max(roof.area, surface.gross_area),
                geometry::source_max(roof.perimeter, surface.perimeter),
                geometry::source_max(roof.height, height),
            ]
        } else {
            [surface.gross_area, surface.perimeter, height]
        };
        metadata.set_exterior_geometry(index, exterior_geometry)?;
    }
    Ok(())
}

fn initialize_user_convection_metadata<M: InsideConvectionSurfaceMetadata + ?Sized>(
    layout: &InsideConvectionLayout,
    zones: &[InsideConvectionZone],
    surfaces: &[InsideConvectionSurface],
    state: &mut InsideConvectionState,
    metadata: &mut M,
    actual_reports: &ReportInputs,
    scan: &mut ReportScanState,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), InsideConvectionError> {
    setup_static(layout, zones, surfaces, state, metadata)?;
    let details = try_scan_for_reports(
        actual_reports,
        scan,
        "Surfaces",
        Some("Details"),
        None,
        None,
        |diagnostic| match diagnostic {
            ReportScanDiagnostic::Warning(message) => backend.show_warning_error(&message),
            ReportScanDiagnostic::Continue(message) => backend.show_continue_error(&message),
        },
    )
    .map_err(|error| match error {
        ReportScanFailure::Unavailable(value) => InsideConvectionError::Scan(value),
        ReportScanFailure::Diagnostic(value) => InsideConvectionError::Backend(value),
    })?;
    if details {
        return Err(InsideConvectionUnavailable::SurfaceDetailsReport.into());
    }
    state.get_user_input = false;
    Ok(())
}

/// Execute the genuine selected no-user first call, readiness transitions and
/// opaque TARP traversal. Unsupported branches stop explicitly; success covers
/// this declared selected domain, never full adaptive/window/user setup.
pub fn initialize_inside_convection<
    C: ZoneEquipmentActiveElements + ?Sized,
    H: InsideConvectionCoefficients + ?Sized,
    S: SpaceAirTemperatureOwners + ?Sized,
>(
    layout: &InsideConvectionLayout,
    zones: &[InsideConvectionZone],
    surfaces: &[InsideConvectionSurface],
    no_user_input: NoUserConvectionInput,
    state: &mut InsideConvectionState,
    zone_equipment: &mut C,
    coefficients: &mut H,
    space_air: &S,
    temperatures: InsideSurfaceTemperatures<'_>,
    readiness: InsideConvectionReadiness,
    actual_low_limit: f64,
    actual_reports: &ReportInputs,
    scan: &mut ReportScanState,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), InsideConvectionError> {
    let mut metadata = std::mem::take(&mut state.surfaces);
    let result = initialize_inside_convection_body(
        layout,
        zones,
        surfaces,
        no_user_input,
        state,
        metadata.as_mut_slice(),
        zone_equipment,
        coefficients,
        space_air,
        &temperatures,
        readiness,
        actual_low_limit,
        actual_reports,
        scan,
        backend,
        ConvectionEntry::Legacy,
    );
    state.surfaces = metadata;
    result
}

fn initialize_inside_convection_body<
    C: ZoneEquipmentActiveElements + ?Sized,
    H: InsideConvectionCoefficients + ?Sized,
    S: SpaceAirTemperatureOwners + ?Sized,
    M: InsideConvectionSurfaceMetadata + ?Sized,
    T: InsideSurfaceTemperatureValues + ?Sized,
>(
    layout: &InsideConvectionLayout,
    zones: &[InsideConvectionZone],
    surfaces: &[InsideConvectionSurface],
    no_user_input: NoUserConvectionInput,
    state: &mut InsideConvectionState,
    metadata: &mut M,
    zone_equipment: &mut C,
    coefficients: &mut H,
    space_air: &S,
    temperatures: &T,
    readiness: InsideConvectionReadiness,
    actual_low_limit: f64,
    actual_reports: &ReportInputs,
    scan: &mut ReportScanState,
    backend: &mut SourceErrorBackend<'_>,
    entry: ConvectionEntry,
) -> Result<(), InsideConvectionError> {
    validate(
        layout,
        zones,
        surfaces,
        state,
        zone_equipment,
        coefficients,
        space_air,
        temperatures,
        metadata,
        entry,
    )?;
    if no_user_input.counts() != [0; 6] {
        return Err(InsideConvectionUnavailable::UserConvectionInput.into());
    }
    if state.get_user_input {
        // GetUser: allocate both zero user curves, then reset/count/allocate
        // both zero surface model arrays, reset counts, and run static setup.
        if entry == ConvectionEntry::Legacy {
            state.user_arrays[0] = Some(Vec::new());
            state.user_arrays[1] = Some(Vec::new());
            state.user_model_counts = [0; 2];
            state.user_arrays[2] = Some(Vec::new());
            state.user_arrays[3] = Some(Vec::new());
            state.user_model_counts = [0; 2];
        }
        initialize_user_convection_metadata(
            layout,
            zones,
            surfaces,
            state,
            metadata,
            actual_reports,
            scan,
            backend,
        )?;
    }
    if state.node_check
        && !readiness.system_sizing
        && !readiness.zone_sizing
        && readiness.equipment_inputs_filled
        && readiness.nodes_allocated
    {
        // Every genuine admitted zone is TARP: the CeilingDiffuser warning
        // loop makes no writes; do not infer readiness from that fact.
        state.node_check = false;
    }
    if state.active_surface_check
        && !readiness.system_sizing
        && !readiness.zone_sizing
        && readiness.equipment_simulated_once
    {
        if zone_equipment.zone_count() != layout.zones().len()
            || layout
                .zones()
                .iter()
                .enumerate()
                .any(|(index, &zone)| zone_equipment.zone_id(index) != zone)
        {
            return Err(InsideConvectionUnavailable::Association.into());
        }
        for (zone_index, zone) in layout.zones().iter().enumerate() {
            let mut count = [0; 3];
            let mut area = [0.0; 3];
            for group in layout.groups().iter().filter(|g| g.zone_id == *zone) {
                for id in &group.surfaces {
                    let index = layout
                        .surfaces()
                        .iter()
                        .position(|r| r.surface_id == *id)
                        .ok_or(InsideConvectionUnavailable::Association)?;
                    if !metadata.has_active_element(index)? {
                        continue;
                    }
                    let kind = match surfaces[index].class {
                        SurfaceType::Wall => 0,
                        SurfaceType::Roof | SurfaceType::Ceiling => 1,
                        SurfaceType::Floor => 2,
                    };
                    count[kind] += 1;
                    area[kind] += surfaces[index].area;
                }
            }
            zone_equipment.set_active_elements(
                zone_index,
                std::array::from_fn(|i| count[i] > 0 && area[i] > 0.0),
            );
        }
        state.active_surface_check = false;
    }
    // Both actual surface model alternatives and every admitted zone are TARP;
    // the BeginEnvrn adaptive search is genuinely false, so Node reset is not reached.
    if readiness.begin_environment && state.my_environment {
        for index in 0..metadata.metadata_count()? {
            // The selected two real model alternatives are nonadaptive.
            let _actual_model = metadata.model(index)?;
        }
        state.my_environment = false;
    }
    if !readiness.begin_environment {
        state.my_environment = true;
    }
    for group in layout.groups() {
        let mat = *space_air
            .borrow_space_for_ids(group.space_id, group.zone_id)?
            .mat;
        for id in &group.surfaces {
            let index = layout
                .surfaces()
                .iter()
                .position(|r| r.surface_id == *id)
                .ok_or(InsideConvectionUnavailable::Association)?;
            let _actual_model = metadata.model(index)?;
            // SetByZone is a local resolution only: retain the actual model owner.
            if entry == ConvectionEntry::Source {
                // Actual CalcASHRAEDetailedIntConvCoeff1967 writes the raw
                // value into the same Source cell BEFORE either clamp read.
                coefficients.set_coefficient(
                    index,
                    natural_coefficient_w_per_m2_k(
                        *temperatures.borrow_temperature(index)?,
                        mat,
                        -surfaces[index].outward_cos_tilt,
                    ),
                );
                if *coefficients.borrow_coefficient(index)? < actual_low_limit {
                    coefficients.set_coefficient(index, actual_low_limit); //1973
                }
                // Distinct caller InitIntConvCoeff338 reads that live cell again.
                if *coefficients.borrow_coefficient(index)? < actual_low_limit {
                    coefficients.set_coefficient(index, actual_low_limit); //339
                }
            } else {
                let mut coefficient = detailed_inside_coefficient_w_per_m2_k(
                    *temperatures.borrow_temperature(index)?,
                    mat,
                    surfaces[index].outward_cos_tilt,
                    actual_low_limit,
                );
                // The caller's second strict lower-limit check is distinct and retained.
                if coefficient < actual_low_limit {
                    coefficient = actual_low_limit;
                }
                coefficients.set_coefficient(index, coefficient);
            }
        }
    }
    // Actual WindowSurface ranges are empty in this selected opaque domain.
    Ok(())
}

#[cfg(test)]
mod tests;
