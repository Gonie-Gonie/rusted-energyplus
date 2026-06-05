//! Typed model and ID primitives.

use std::collections::BTreeMap;

/// EnergyPlus-compatible model version.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Version {
    /// Major version.
    pub major: u16,
    /// Minor version.
    pub minor: u16,
    /// Patch version.
    pub patch: u16,
}

impl Version {
    /// Initial oracle version.
    #[must_use]
    pub const fn oracle_26_1_0() -> Self {
        Self {
            major: 26,
            minor: 1,
            patch: 0,
        }
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Normalized name used only during compile, diagnostics, and export.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NormalizedName(pub String);

impl NormalizedName {
    /// Applies the first EnergyPlus-compatible name normalization rule.
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self(name.trim().to_ascii_uppercase())
    }
}

macro_rules! typed_id {
    ($name:ident) => {
        #[doc = concat!("Typed ID for ", stringify!($name), ".")]
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(pub u32);
    };
}

typed_id!(ZoneId);
typed_id!(SurfaceId);
typed_id!(ConstructionId);
typed_id!(MaterialId);
typed_id!(InternalGainId);
typed_id!(ScheduleTypeLimitId);
typed_id!(ScheduleId);
typed_id!(RunPeriodId);
typed_id!(NodeId);
typed_id!(ComponentId);
typed_id!(LoopId);
typed_id!(OutputHandle);

/// Compile-time name map from EnergyPlus names to typed IDs.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NameMap<T> {
    by_name: BTreeMap<NormalizedName, T>,
    by_id: Vec<NormalizedName>,
}

impl<T> Default for NameMap<T> {
    fn default() -> Self {
        Self {
            by_name: BTreeMap::new(),
            by_id: Vec::new(),
        }
    }
}

impl<T: Copy> NameMap<T> {
    /// Inserts a normalized name and returns the existing ID on duplicate.
    pub fn insert(&mut self, name: &str, id: T) -> Option<T> {
        let normalized = NormalizedName::new(name);
        if let Some(existing) = self.by_name.get(&normalized) {
            return Some(*existing);
        }

        self.by_name.insert(normalized.clone(), id);
        self.by_id.push(normalized);
        None
    }

    /// Resolves a raw EnergyPlus name to a typed ID.
    #[must_use]
    pub fn resolve(&self, name: &str) -> Option<T> {
        self.by_name.get(&NormalizedName::new(name)).copied()
    }

    /// Number of registered names.
    #[must_use]
    pub fn len(&self) -> usize {
        self.by_id.len()
    }

    /// Returns true when no names are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.by_id.is_empty()
    }

    /// Names in typed ID insertion order.
    #[must_use]
    pub fn names(&self) -> &[NormalizedName] {
        &self.by_id
    }
}

/// Numeric field that may be set to EnergyPlus Autocalculate.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AutoOrNumber {
    /// EnergyPlus should calculate the value from model geometry.
    AutoCalculate,
    /// User-specified numeric value.
    Value(f64),
}

/// Three-dimensional point in meters.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point3 {
    /// X coordinate.
    pub x_m: f64,
    /// Y coordinate.
    pub y_m: f64,
    /// Z coordinate.
    pub z_m: f64,
}

/// Terrain enum used by Building.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Terrain {
    /// City center.
    City,
    /// Flat open country.
    Country,
    /// Large water body within 5 km.
    Ocean,
    /// Country towns and suburbs.
    Suburbs,
    /// Urban, industrial, or forest.
    Urban,
}

/// Solar distribution enum used by Building.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SolarDistribution {
    /// Minimal shadowing.
    MinimalShadowing,
    /// Full exterior.
    FullExterior,
    /// Full exterior with reflections.
    FullExteriorWithReflections,
    /// Full interior and exterior.
    FullInteriorAndExterior,
    /// Full interior and exterior with reflections.
    FullInteriorAndExteriorWithReflections,
}

/// Building-level typed settings.
#[derive(Clone, Debug, PartialEq)]
pub struct Building {
    /// Object name.
    pub name: NormalizedName,
    /// North axis in degrees.
    pub north_axis_deg: f64,
    /// Terrain classification.
    pub terrain: Terrain,
    /// Loads convergence tolerance in watts.
    pub loads_convergence_tolerance_w: f64,
    /// Temperature convergence tolerance in delta C.
    pub temperature_convergence_tolerance_delta_c: f64,
    /// Solar distribution algorithm.
    pub solar_distribution: SolarDistribution,
    /// Maximum warmup day count.
    pub maximum_number_of_warmup_days: u32,
    /// Minimum warmup day count.
    pub minimum_number_of_warmup_days: u32,
}

/// Zone timestep configuration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimestepConfig {
    /// Number of zone timesteps per hour.
    pub number_of_timesteps_per_hour: u32,
}

impl Default for TimestepConfig {
    fn default() -> Self {
        Self {
            number_of_timesteps_per_hour: 6,
        }
    }
}

/// Calendar day of week used by `RunPeriod`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DayOfWeek {
    /// Monday.
    Monday,
    /// Tuesday.
    Tuesday,
    /// Wednesday.
    Wednesday,
    /// Thursday.
    Thursday,
    /// Friday.
    Friday,
    /// Saturday.
    Saturday,
    /// Sunday.
    Sunday,
}

/// Run period date range.
#[derive(Clone, Debug, PartialEq)]
pub struct RunPeriod {
    /// Typed ID.
    pub id: RunPeriodId,
    /// Object name.
    pub name: NormalizedName,
    /// Begin month.
    pub begin_month: u32,
    /// Begin day of month.
    pub begin_day_of_month: u32,
    /// Optional begin year.
    pub begin_year: Option<u32>,
    /// End month.
    pub end_month: u32,
    /// End day of month.
    pub end_day_of_month: u32,
    /// Optional end year.
    pub end_year: Option<u32>,
    /// Optional declared start day of week.
    pub day_of_week_for_start_day: Option<DayOfWeek>,
}

/// Site location.
#[derive(Clone, Debug, PartialEq)]
pub struct SiteLocation {
    /// Object name.
    pub name: NormalizedName,
    /// Latitude in degrees.
    pub latitude_deg: f64,
    /// Longitude in degrees.
    pub longitude_deg: f64,
    /// Time zone offset in hours.
    pub time_zone_hours: f64,
    /// Elevation in meters.
    pub elevation_m: f64,
}

/// Thermal zone.
#[derive(Clone, Debug, PartialEq)]
pub struct Zone {
    /// Typed ID.
    pub id: ZoneId,
    /// Zone name.
    pub name: NormalizedName,
    /// Direction of relative north in degrees.
    pub direction_of_relative_north_deg: f64,
    /// Zone origin.
    pub origin: Point3,
    /// EnergyPlus zone type.
    pub zone_type: u32,
    /// Zone multiplier.
    pub multiplier: u32,
    /// Ceiling height.
    pub ceiling_height: AutoOrNumber,
    /// Zone volume.
    pub volume: AutoOrNumber,
}

/// Material flavor tracked by the first typed subset.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MaterialKind {
    /// Material object with mass.
    Mass,
    /// Material:NoMass object.
    NoMass,
}

/// Minimal material identity and thermal properties.
#[derive(Clone, Debug, PartialEq)]
pub struct Material {
    /// Typed ID.
    pub id: MaterialId,
    /// Material name.
    pub name: NormalizedName,
    /// Material object kind.
    pub kind: MaterialKind,
    /// Conductivity for Material objects in W/m-K.
    pub conductivity_w_per_m_k: Option<f64>,
    /// Density for Material objects in kg/m3.
    pub density_kg_per_m3: Option<f64>,
    /// Specific heat for Material objects in J/kg-K.
    pub specific_heat_j_per_kg_k: Option<f64>,
    /// Thickness for Material objects in meters.
    pub thickness_m: Option<f64>,
    /// Thermal resistance for Material:NoMass objects in m2-K/W.
    pub thermal_resistance_m2_k_per_w: Option<f64>,
}

impl Material {
    /// Returns the area-normalized thermal resistance when available.
    #[must_use]
    pub fn thermal_resistance(&self) -> Option<f64> {
        if let Some(resistance) = self.thermal_resistance_m2_k_per_w
            && resistance > 0.0
        {
            return Some(resistance);
        }

        let (Some(thickness), Some(conductivity)) = (self.thickness_m, self.conductivity_w_per_m_k)
        else {
            return None;
        };
        if thickness > 0.0 && conductivity > 0.0 {
            Some(thickness / conductivity)
        } else {
            None
        }
    }

    /// Returns the area-normalized heat capacity when available.
    #[must_use]
    pub fn heat_capacity_per_area(&self) -> Option<f64> {
        let (Some(thickness), Some(density), Some(specific_heat)) = (
            self.thickness_m,
            self.density_kg_per_m3,
            self.specific_heat_j_per_kg_k,
        ) else {
            return None;
        };
        if thickness > 0.0 && density > 0.0 && specific_heat > 0.0 {
            Some(thickness * density * specific_heat)
        } else {
            None
        }
    }
}

/// Construction resolved to an outside layer material.
#[derive(Clone, Debug, PartialEq)]
pub struct Construction {
    /// Typed ID.
    pub id: ConstructionId,
    /// Construction name.
    pub name: NormalizedName,
    /// Outside layer material.
    pub outside_layer: MaterialId,
}

/// Schedule numeric type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NumericType {
    /// Continuous values.
    Continuous,
    /// Discrete values.
    Discrete,
}

/// Schedule type limits.
#[derive(Clone, Debug, PartialEq)]
pub struct ScheduleTypeLimits {
    /// Typed ID.
    pub id: ScheduleTypeLimitId,
    /// Object name.
    pub name: NormalizedName,
    /// Optional lower limit.
    pub lower_limit: Option<f64>,
    /// Optional upper limit.
    pub upper_limit: Option<f64>,
    /// Numeric type.
    pub numeric_type: Option<NumericType>,
}

/// Constant schedule.
#[derive(Clone, Debug, PartialEq)]
pub struct ScheduleConstant {
    /// Typed ID.
    pub id: ScheduleId,
    /// Schedule name.
    pub name: NormalizedName,
    /// Optional type limits.
    pub schedule_type_limits: Option<ScheduleTypeLimitId>,
    /// Constant hourly value.
    pub hourly_value: f64,
}

/// One value segment in a compact schedule day profile.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScheduleCompactSegment {
    /// Minute of day at which this segment ends, 1 through 1440.
    pub until_minute_of_day: u32,
    /// Segment value.
    pub value: f64,
}

/// Compact schedule subset using all-days daily `Until` segments.
#[derive(Clone, Debug, PartialEq)]
pub struct ScheduleCompact {
    /// Typed ID.
    pub id: ScheduleId,
    /// Schedule name.
    pub name: NormalizedName,
    /// Optional type limits.
    pub schedule_type_limits: Option<ScheduleTypeLimitId>,
    /// Daily all-days value segments.
    pub segments: Vec<ScheduleCompactSegment>,
}

/// Electric or process equipment represented as a zone internal gain.
#[derive(Clone, Debug, PartialEq)]
pub struct OtherEquipment {
    /// Typed ID.
    pub id: InternalGainId,
    /// Equipment name.
    pub name: NormalizedName,
    /// Target zone.
    pub zone: ZoneId,
    /// Availability or level schedule.
    pub schedule: Option<ScheduleId>,
    /// Design-level heat gain in watts.
    pub design_level_w: f64,
    /// Fraction of gain emitted as latent load.
    pub fraction_latent: f64,
    /// Fraction of gain emitted as radiant load.
    pub fraction_radiant: f64,
    /// Fraction of gain lost outside the zone air balance.
    pub fraction_lost: f64,
}

/// Building surface type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SurfaceType {
    /// Ceiling surface.
    Ceiling,
    /// Floor surface.
    Floor,
    /// Roof surface.
    Roof,
    /// Wall surface.
    Wall,
}

/// Outside boundary condition for the first detailed surface subset.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OutsideBoundaryCondition {
    /// Adiabatic boundary.
    Adiabatic,
    /// Foundation boundary.
    Foundation,
    /// Ground boundary.
    Ground,
    /// Outdoors boundary.
    Outdoors,
    /// Space boundary.
    Space,
    /// Adjacent surface boundary.
    Surface,
    /// Adjacent zone boundary.
    Zone,
    /// Other supported boundary condition represented but not simulated yet.
    Other,
}

/// Sun exposure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SunExposure {
    /// No sun exposure.
    NoSun,
    /// Sun exposed.
    SunExposed,
}

/// Wind exposure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WindExposure {
    /// No wind exposure.
    NoWind,
    /// Wind exposed.
    WindExposed,
}

/// Detailed building surface.
#[derive(Clone, Debug, PartialEq)]
pub struct Surface {
    /// Typed ID.
    pub id: SurfaceId,
    /// Surface name.
    pub name: NormalizedName,
    /// Surface type.
    pub surface_type: SurfaceType,
    /// Resolved construction ID.
    pub construction: ConstructionId,
    /// Resolved zone ID.
    pub zone: ZoneId,
    /// Outside boundary condition.
    pub outside_boundary_condition: OutsideBoundaryCondition,
    /// Optional outside boundary condition object name.
    pub outside_boundary_condition_object: Option<NormalizedName>,
    /// Sun exposure.
    pub sun_exposure: SunExposure,
    /// Wind exposure.
    pub wind_exposure: WindExposure,
    /// View factor to ground.
    pub view_factor_to_ground: AutoOrNumber,
    /// Surface vertices.
    pub vertices: Vec<Point3>,
}

/// Minimal typed model for early compiler stages.
#[derive(Clone, Debug, PartialEq)]
pub struct TypedModel {
    /// Model version.
    pub version: Version,
    /// Building settings.
    pub building: Option<Building>,
    /// Zone timestep config.
    pub timestep: TimestepConfig,
    /// Run periods.
    pub run_periods: Vec<RunPeriod>,
    /// Run period names.
    pub run_period_names: NameMap<RunPeriodId>,
    /// Site location.
    pub site: Option<SiteLocation>,
    /// Materials.
    pub materials: Vec<Material>,
    /// Material names.
    pub material_names: NameMap<MaterialId>,
    /// Constructions.
    pub constructions: Vec<Construction>,
    /// Construction names.
    pub construction_names: NameMap<ConstructionId>,
    /// Schedule type limits.
    pub schedule_type_limits: Vec<ScheduleTypeLimits>,
    /// Schedule type limit names.
    pub schedule_type_limit_names: NameMap<ScheduleTypeLimitId>,
    /// Constant schedules.
    pub schedules: Vec<ScheduleConstant>,
    /// Compact schedules.
    pub compact_schedules: Vec<ScheduleCompact>,
    /// Schedule names.
    pub schedule_names: NameMap<ScheduleId>,
    /// Zone internal gains from OtherEquipment objects.
    pub other_equipment: Vec<OtherEquipment>,
    /// OtherEquipment names.
    pub other_equipment_names: NameMap<InternalGainId>,
    /// Zones.
    pub zones: Vec<Zone>,
    /// Zone names.
    pub zone_names: NameMap<ZoneId>,
    /// Building surfaces.
    pub surfaces: Vec<Surface>,
    /// Surface names.
    pub surface_names: NameMap<SurfaceId>,
}

impl Default for TypedModel {
    fn default() -> Self {
        Self {
            version: Version::oracle_26_1_0(),
            building: None,
            timestep: TimestepConfig::default(),
            run_periods: Vec::new(),
            run_period_names: NameMap::default(),
            site: None,
            materials: Vec::new(),
            material_names: NameMap::default(),
            constructions: Vec::new(),
            construction_names: NameMap::default(),
            schedule_type_limits: Vec::new(),
            schedule_type_limit_names: NameMap::default(),
            schedules: Vec::new(),
            compact_schedules: Vec::new(),
            schedule_names: NameMap::default(),
            other_equipment: Vec::new(),
            other_equipment_names: NameMap::default(),
            zones: Vec::new(),
            zone_names: NameMap::default(),
            surfaces: Vec::new(),
            surface_names: NameMap::default(),
        }
    }
}

impl TypedModel {
    /// Number of typed object instances in the current subset.
    #[must_use]
    pub fn object_count(&self) -> usize {
        usize::from(self.building.is_some())
            + usize::from(self.site.is_some())
            + 1
            + self.run_periods.len()
            + self.materials.len()
            + self.constructions.len()
            + self.schedule_type_limits.len()
            + self.schedules.len()
            + self.compact_schedules.len()
            + self.other_equipment.len()
            + self.zones.len()
            + self.surfaces.len()
    }
}

/// Runtime-ready immutable model plus graph relations.
#[derive(Clone, Debug, PartialEq)]
pub struct SimulationModel {
    /// Typed model payload.
    pub typed: TypedModel,
    /// Static model graph.
    pub graph: ModelGraph,
}

impl SimulationModel {
    /// Builds a runtime-ready model from an already reference-resolved typed model.
    #[must_use]
    pub fn from_typed(typed: TypedModel) -> Self {
        let graph = ModelGraph::from_typed(&typed);
        Self { typed, graph }
    }
}

/// Static model graph used for validation and execution planning.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ModelGraph {
    /// Zone to surface edges.
    pub zone_surfaces: Vec<ZoneSurfaceEdge>,
    /// Construction to material edges.
    pub construction_materials: Vec<ConstructionMaterialEdge>,
}

impl ModelGraph {
    /// Builds static graph edges from the typed subset.
    #[must_use]
    pub fn from_typed(model: &TypedModel) -> Self {
        Self {
            zone_surfaces: model
                .surfaces
                .iter()
                .map(|surface| ZoneSurfaceEdge {
                    zone: surface.zone,
                    surface: surface.id,
                })
                .collect(),
            construction_materials: model
                .constructions
                .iter()
                .map(|construction| ConstructionMaterialEdge {
                    construction: construction.id,
                    material: construction.outside_layer,
                    layer_index: 0,
                })
                .collect(),
        }
    }
}

/// Zone/surface relation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ZoneSurfaceEdge {
    /// Zone ID.
    pub zone: ZoneId,
    /// Surface ID.
    pub surface: SurfaceId,
}

/// Construction/material relation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ConstructionMaterialEdge {
    /// Construction ID.
    pub construction: ConstructionId,
    /// Material ID.
    pub material: MaterialId,
    /// Zero-based layer index.
    pub layer_index: u32,
}

#[cfg(test)]
mod tests {
    use super::{
        Construction, ConstructionId, MaterialId, MaterialKind, ModelGraph, NameMap, Surface,
        SurfaceId, TypedModel, Version, ZoneId,
    };

    #[test]
    fn default_model_uses_oracle_version() {
        let model = TypedModel::default();

        assert_eq!(model.version, Version::oracle_26_1_0());
    }

    #[test]
    fn ids_are_copyable_values() {
        let first = ZoneId(7);
        let second = first;

        assert_eq!(first, second);
    }

    #[test]
    fn name_map_resolves_trimmed_case_insensitive_names() {
        let mut names = NameMap::default();
        assert_eq!(names.insert("Zone One", ZoneId(0)), None);

        assert_eq!(names.resolve(" zone one "), Some(ZoneId(0)));
        assert_eq!(names.len(), 1);
    }

    #[test]
    fn material_derives_resistance_and_capacity() {
        let material = super::Material {
            id: MaterialId(0),
            name: super::NormalizedName::new("Concrete"),
            kind: MaterialKind::Mass,
            conductivity_w_per_m_k: Some(2.0),
            density_kg_per_m3: Some(2_000.0),
            specific_heat_j_per_kg_k: Some(800.0),
            thickness_m: Some(0.1),
            thermal_resistance_m2_k_per_w: None,
        };

        assert_eq!(material.thermal_resistance(), Some(0.05));
        assert_eq!(material.heat_capacity_per_area(), Some(160_000.0));
    }

    #[test]
    fn model_graph_links_surfaces_and_constructions() {
        let mut model = TypedModel::default();
        model.constructions.push(Construction {
            id: ConstructionId(0),
            name: super::NormalizedName::new("Wall"),
            outside_layer: MaterialId(0),
        });
        model.surfaces.push(Surface {
            id: SurfaceId(0),
            name: super::NormalizedName::new("Surface"),
            surface_type: super::SurfaceType::Wall,
            construction: ConstructionId(0),
            zone: ZoneId(0),
            outside_boundary_condition: super::OutsideBoundaryCondition::Outdoors,
            outside_boundary_condition_object: None,
            sun_exposure: super::SunExposure::SunExposed,
            wind_exposure: super::WindExposure::WindExposed,
            view_factor_to_ground: super::AutoOrNumber::AutoCalculate,
            vertices: Vec::new(),
        });

        let graph = ModelGraph::from_typed(&model);

        assert_eq!(graph.zone_surfaces[0].zone, ZoneId(0));
        assert_eq!(graph.zone_surfaces[0].surface, SurfaceId(0));
        assert_eq!(graph.construction_materials[0].material, MaterialId(0));
    }
}
