//! Model compiler stage contracts.

use ep_model::{
    AutoOrNumber, Building, Construction, ConstructionId, InternalGainId, Material, MaterialId,
    MaterialKind, NameMap, NormalizedName, NumericType, OtherEquipment, OutsideBoundaryCondition,
    Point3, ScheduleConstant, ScheduleId, ScheduleTypeLimitId, ScheduleTypeLimits, SiteLocation,
    SolarDistribution, SunExposure, Surface, SurfaceId, SurfaceType, Terrain, TimestepConfig,
    TypedModel, Version, WindExposure, Zone, ZoneId,
};
use ep_raw_model::{FieldName, ObjectType, RawModel, RawObject, RawValue};

/// Ordered model compiler stages.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CompileStage {
    /// Parse epJSON into raw object storage.
    Parse,
    /// Validate against schema and required fields.
    SchemaValidation,
    /// Resolve defaults and canonical ordering.
    Normalize,
    /// Convert raw values to typed structs.
    TypedConversion,
    /// Resolve names to typed IDs.
    ReferenceResolution,
    /// Build model graphs.
    GraphBuild,
    /// Generate runtime execution plan.
    ExecutionPlan,
    /// Initialize runtime state and output handles.
    RuntimeInit,
}

/// Diagnostic severity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DiagnosticSeverity {
    /// Compilation cannot produce a complete typed model.
    Error,
    /// Compilation can continue, but the model needs attention.
    Warning,
}

impl std::fmt::Display for DiagnosticSeverity {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Error => write!(formatter, "error"),
            Self::Warning => write!(formatter, "warning"),
        }
    }
}

/// Structured compiler diagnostic.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelDiagnostic {
    /// Severity.
    pub severity: DiagnosticSeverity,
    /// Stable diagnostic code.
    pub code: String,
    /// EnergyPlus object type.
    pub object_type: String,
    /// EnergyPlus object name when available.
    pub object_name: Option<String>,
    /// EnergyPlus field name when available.
    pub field: Option<String>,
    /// User-facing message.
    pub message: String,
}

/// Default value applied during typed conversion.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DefaultApplication {
    /// EnergyPlus object type.
    pub object_type: String,
    /// EnergyPlus object name.
    pub object_name: String,
    /// Field that received a default.
    pub field: String,
    /// Applied value.
    pub value: String,
}

/// Typed compiler coverage status for an object type seen in RawModel.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObjectCoverageStatus {
    /// Object type is part of the current TypedModel contract.
    Typed,
    /// Object type is preserved in RawModel but not typed by this compiler stage.
    RawOnly,
}

impl std::fmt::Display for ObjectCoverageStatus {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Typed => write!(formatter, "typed"),
            Self::RawOnly => write!(formatter, "raw-only"),
        }
    }
}

/// Coverage entry for one EnergyPlus object type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObjectCoverage {
    /// EnergyPlus object type.
    pub object_type: String,
    /// Number of RawModel instances with this type.
    pub object_count: usize,
    /// Typed compiler coverage status.
    pub status: ObjectCoverageStatus,
}

/// Minimal report for a compiler pass.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompileReport {
    /// Stages that completed.
    pub completed_stages: Vec<CompileStage>,
    /// Raw object count observed at parse stage.
    pub raw_object_count: usize,
    /// Typed object count observed after typed conversion.
    pub typed_object_count: usize,
    /// Structured diagnostics.
    pub diagnostics: Vec<ModelDiagnostic>,
    /// Defaults applied while building the typed model.
    pub defaults_applied: Vec<DefaultApplication>,
    /// Object coverage observed for this compile.
    pub coverage: Vec<ObjectCoverage>,
}

/// Result of compiling a RawModel.
#[derive(Clone, Debug, PartialEq)]
pub struct CompileResult {
    /// Typed model when no error diagnostics were emitted.
    pub model: Option<TypedModel>,
    /// Compiler report.
    pub report: CompileReport,
}

impl CompileResult {
    /// Returns true when the compiler emitted at least one error.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.report
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == DiagnosticSeverity::Error)
    }
}

/// Compiles a RawModel into the first typed model subset.
#[must_use]
pub fn compile_raw_model(raw_model: &RawModel) -> CompileResult {
    Compiler::new(raw_model).compile()
}

/// Returns the current TypedModel coverage status for an object type.
#[must_use]
pub fn typed_coverage_status(object_type: &str) -> ObjectCoverageStatus {
    if TYPED_OBJECT_TYPES.contains(&object_type) {
        ObjectCoverageStatus::Typed
    } else {
        ObjectCoverageStatus::RawOnly
    }
}

/// Builds a deterministic object coverage report from RawModel contents.
#[must_use]
pub fn compile_coverage(raw_model: &RawModel) -> Vec<ObjectCoverage> {
    raw_model
        .object_type_counts()
        .into_iter()
        .map(|(object_type, object_count)| ObjectCoverage {
            status: typed_coverage_status(&object_type),
            object_type,
            object_count,
        })
        .collect()
}

const TYPED_OBJECT_TYPES: &[&str] = &[
    "Version",
    "Building",
    "Timestep",
    "Site:Location",
    "Material",
    "Material:NoMass",
    "Construction",
    "ScheduleTypeLimits",
    "Schedule:Constant",
    "OtherEquipment",
    "Zone",
    "BuildingSurface:Detailed",
];

struct Compiler<'a> {
    raw_model: &'a RawModel,
    diagnostics: Vec<ModelDiagnostic>,
    defaults_applied: Vec<DefaultApplication>,
}

impl<'a> Compiler<'a> {
    fn new(raw_model: &'a RawModel) -> Self {
        Self {
            raw_model,
            diagnostics: Vec::new(),
            defaults_applied: Vec::new(),
        }
    }

    fn compile(mut self) -> CompileResult {
        let mut model = TypedModel {
            version: self.parse_version(),
            ..TypedModel::default()
        };

        self.parse_building(&mut model);
        self.parse_timestep(&mut model);
        self.parse_site_location(&mut model);
        self.parse_materials(&mut model);
        self.parse_constructions(&mut model);
        self.parse_schedule_type_limits(&mut model);
        self.parse_schedules(&mut model);
        self.parse_zones(&mut model);
        self.parse_other_equipment(&mut model);
        self.parse_surfaces(&mut model);

        let typed_object_count = model.object_count();
        let has_errors = self
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == DiagnosticSeverity::Error);
        let mut completed_stages = vec![
            CompileStage::Parse,
            CompileStage::SchemaValidation,
            CompileStage::Normalize,
            CompileStage::TypedConversion,
        ];
        if !has_errors {
            completed_stages.push(CompileStage::ReferenceResolution);
        }

        let report = CompileReport {
            completed_stages,
            raw_object_count: self.raw_model.object_count(),
            typed_object_count,
            diagnostics: self.diagnostics,
            defaults_applied: self.defaults_applied,
            coverage: compile_coverage(self.raw_model),
        };

        CompileResult {
            model: if has_errors { None } else { Some(model) },
            report,
        }
    }

    fn parse_version(&mut self) -> Version {
        let Some(version) = self.raw_model.version.as_deref() else {
            return Version::oracle_26_1_0();
        };

        let mut parts = version.split('.');
        let Some(major) = parts.next().and_then(|part| part.parse::<u16>().ok()) else {
            self.error(
                "InvalidVersion",
                "Version",
                None,
                Some("version_identifier"),
                format!("could not parse EnergyPlus version '{version}'"),
            );
            return Version::oracle_26_1_0();
        };
        let minor = parts
            .next()
            .and_then(|part| part.parse::<u16>().ok())
            .unwrap_or(0);
        let patch = parts
            .next()
            .and_then(|part| part.parse::<u16>().ok())
            .unwrap_or(0);

        Version {
            major,
            minor,
            patch,
        }
    }

    fn parse_building(&mut self, model: &mut TypedModel) {
        let Some((name, object)) = self.single_object("Building") else {
            return;
        };

        let building = Building {
            name: NormalizedName::new(&name),
            north_axis_deg: self.number_default("Building", &name, &object, "north_axis", 0.0),
            terrain: self.enum_default(
                "Building",
                &name,
                (&object, "terrain"),
                Terrain::Suburbs,
                "Suburbs",
                parse_terrain,
            ),
            loads_convergence_tolerance_w: self.number_default(
                "Building",
                &name,
                &object,
                "loads_convergence_tolerance_value",
                0.04,
            ),
            temperature_convergence_tolerance_delta_c: self.number_default(
                "Building",
                &name,
                &object,
                "temperature_convergence_tolerance_value",
                0.4,
            ),
            solar_distribution: self.enum_default(
                "Building",
                &name,
                (&object, "solar_distribution"),
                SolarDistribution::FullExterior,
                "FullExterior",
                parse_solar_distribution,
            ),
            maximum_number_of_warmup_days: self.u32_default(
                "Building",
                &name,
                &object,
                "maximum_number_of_warmup_days",
                25,
            ),
            minimum_number_of_warmup_days: self.u32_default(
                "Building",
                &name,
                &object,
                "minimum_number_of_warmup_days",
                1,
            ),
        };
        model.building = Some(building);
    }

    fn parse_timestep(&mut self, model: &mut TypedModel) {
        let Some((name, object)) = self.single_object("Timestep") else {
            return;
        };

        model.timestep = TimestepConfig {
            number_of_timesteps_per_hour: self.u32_default(
                "Timestep",
                &name,
                &object,
                "number_of_timesteps_per_hour",
                6,
            ),
        };
    }

    fn parse_site_location(&mut self, model: &mut TypedModel) {
        let Some((name, object)) = self.single_object("Site:Location") else {
            return;
        };

        model.site = Some(SiteLocation {
            name: NormalizedName::new(&name),
            latitude_deg: self.number_default("Site:Location", &name, &object, "latitude", 0.0),
            longitude_deg: self.number_default("Site:Location", &name, &object, "longitude", 0.0),
            time_zone_hours: self.number_default("Site:Location", &name, &object, "time_zone", 0.0),
            elevation_m: self.number_default("Site:Location", &name, &object, "elevation", 0.0),
        });
    }

    fn parse_materials(&mut self, model: &mut TypedModel) {
        for (object_type, kind) in [
            ("Material", MaterialKind::Mass),
            ("Material:NoMass", MaterialKind::NoMass),
        ] {
            for (name, object) in self.objects(object_type) {
                let Some(id_value) = self.checked_id(object_type, &name, model.materials.len())
                else {
                    continue;
                };
                let id = MaterialId(id_value);
                if model.material_names.insert(&name, id).is_some() {
                    self.duplicate_name(object_type, &name);
                    continue;
                }

                model.materials.push(Material {
                    id,
                    name: NormalizedName::new(&name),
                    kind,
                    conductivity_w_per_m_k: self.optional_number(
                        object_type,
                        &name,
                        &object,
                        "conductivity",
                    ),
                    density_kg_per_m3: self.optional_number(object_type, &name, &object, "density"),
                    specific_heat_j_per_kg_k: self.optional_number(
                        object_type,
                        &name,
                        &object,
                        "specific_heat",
                    ),
                    thickness_m: self.optional_number(object_type, &name, &object, "thickness"),
                    thermal_resistance_m2_k_per_w: self.optional_number(
                        object_type,
                        &name,
                        &object,
                        "thermal_resistance",
                    ),
                });
            }
        }
    }

    fn parse_constructions(&mut self, model: &mut TypedModel) {
        for (name, object) in self.objects("Construction") {
            let Some(outside_layer_name) =
                self.required_string("Construction", &name, &object, "outside_layer")
            else {
                continue;
            };
            let Some(outside_layer) = self.resolve_name(
                &model.material_names,
                "Construction",
                &name,
                "outside_layer",
                &outside_layer_name,
                "Material",
            ) else {
                continue;
            };
            let Some(id_value) = self.checked_id("Construction", &name, model.constructions.len())
            else {
                continue;
            };
            let id = ConstructionId(id_value);
            if model.construction_names.insert(&name, id).is_some() {
                self.duplicate_name("Construction", &name);
                continue;
            }

            model.constructions.push(Construction {
                id,
                name: NormalizedName::new(&name),
                outside_layer,
            });
        }
    }

    fn parse_schedule_type_limits(&mut self, model: &mut TypedModel) {
        for (name, object) in self.objects("ScheduleTypeLimits") {
            let Some(id_value) = self.checked_id(
                "ScheduleTypeLimits",
                &name,
                model.schedule_type_limits.len(),
            ) else {
                continue;
            };
            let id = ScheduleTypeLimitId(id_value);
            if model.schedule_type_limit_names.insert(&name, id).is_some() {
                self.duplicate_name("ScheduleTypeLimits", &name);
                continue;
            }

            model.schedule_type_limits.push(ScheduleTypeLimits {
                id,
                name: NormalizedName::new(&name),
                lower_limit: self.optional_number(
                    "ScheduleTypeLimits",
                    &name,
                    &object,
                    "lower_limit_value",
                ),
                upper_limit: self.optional_number(
                    "ScheduleTypeLimits",
                    &name,
                    &object,
                    "upper_limit_value",
                ),
                numeric_type: self.optional_enum(
                    "ScheduleTypeLimits",
                    &name,
                    &object,
                    "numeric_type",
                    parse_numeric_type,
                ),
            });
        }
    }

    fn parse_schedules(&mut self, model: &mut TypedModel) {
        for (name, object) in self.objects("Schedule:Constant") {
            let schedule_type_limits = match self.optional_string(
                "Schedule:Constant",
                &name,
                &object,
                "schedule_type_limits_name",
            ) {
                Some(type_limits_name) => self.resolve_name(
                    &model.schedule_type_limit_names,
                    "Schedule:Constant",
                    &name,
                    "schedule_type_limits_name",
                    &type_limits_name,
                    "ScheduleTypeLimits",
                ),
                None => None,
            };
            let Some(id_value) = self.checked_id("Schedule:Constant", &name, model.schedules.len())
            else {
                continue;
            };
            let id = ScheduleId(id_value);
            if model.schedule_names.insert(&name, id).is_some() {
                self.duplicate_name("Schedule:Constant", &name);
                continue;
            }

            model.schedules.push(ScheduleConstant {
                id,
                name: NormalizedName::new(&name),
                schedule_type_limits,
                hourly_value: self.number_default(
                    "Schedule:Constant",
                    &name,
                    &object,
                    "hourly_value",
                    0.0,
                ),
            });
        }
    }

    fn parse_zones(&mut self, model: &mut TypedModel) {
        for (name, object) in self.objects("Zone") {
            let Some(id_value) = self.checked_id("Zone", &name, model.zones.len()) else {
                continue;
            };
            let id = ZoneId(id_value);
            if model.zone_names.insert(&name, id).is_some() {
                self.duplicate_name("Zone", &name);
                continue;
            }

            model.zones.push(Zone {
                id,
                name: NormalizedName::new(&name),
                direction_of_relative_north_deg: self.number_default(
                    "Zone",
                    &name,
                    &object,
                    "direction_of_relative_north",
                    0.0,
                ),
                origin: Point3 {
                    x_m: self.number_default("Zone", &name, &object, "x_origin", 0.0),
                    y_m: self.number_default("Zone", &name, &object, "y_origin", 0.0),
                    z_m: self.number_default("Zone", &name, &object, "z_origin", 0.0),
                },
                zone_type: self.u32_default("Zone", &name, &object, "type", 1),
                multiplier: self.u32_default("Zone", &name, &object, "multiplier", 1),
                ceiling_height: self.auto_default(
                    "Zone",
                    &name,
                    &object,
                    "ceiling_height",
                    AutoOrNumber::AutoCalculate,
                    "Autocalculate",
                ),
                volume: self.auto_default(
                    "Zone",
                    &name,
                    &object,
                    "volume",
                    AutoOrNumber::AutoCalculate,
                    "Autocalculate",
                ),
            });
        }
    }

    fn parse_other_equipment(&mut self, model: &mut TypedModel) {
        for (name, object) in self.objects("OtherEquipment") {
            let Some(zone_name) = self.required_string(
                "OtherEquipment",
                &name,
                &object,
                "zone_or_zonelist_or_space_or_spacelist_name",
            ) else {
                continue;
            };
            let Some(zone) = self.resolve_name(
                &model.zone_names,
                "OtherEquipment",
                &name,
                "zone_or_zonelist_or_space_or_spacelist_name",
                &zone_name,
                "Zone",
            ) else {
                continue;
            };
            let schedule =
                match self.optional_string("OtherEquipment", &name, &object, "schedule_name") {
                    Some(schedule_name) => self.resolve_name(
                        &model.schedule_names,
                        "OtherEquipment",
                        &name,
                        "schedule_name",
                        &schedule_name,
                        "Schedule",
                    ),
                    None => None,
                };
            let Some(id_value) =
                self.checked_id("OtherEquipment", &name, model.other_equipment.len())
            else {
                continue;
            };
            let id = InternalGainId(id_value);
            if model.other_equipment_names.insert(&name, id).is_some() {
                self.duplicate_name("OtherEquipment", &name);
                continue;
            }

            model.other_equipment.push(OtherEquipment {
                id,
                name: NormalizedName::new(&name),
                zone,
                schedule,
                design_level_w: self.number_default(
                    "OtherEquipment",
                    &name,
                    &object,
                    "design_level",
                    0.0,
                ),
                fraction_latent: self.number_default(
                    "OtherEquipment",
                    &name,
                    &object,
                    "fraction_latent",
                    0.0,
                ),
                fraction_radiant: self.number_default(
                    "OtherEquipment",
                    &name,
                    &object,
                    "fraction_radiant",
                    0.0,
                ),
                fraction_lost: self.number_default(
                    "OtherEquipment",
                    &name,
                    &object,
                    "fraction_lost",
                    0.0,
                ),
            });
        }
    }

    fn parse_surfaces(&mut self, model: &mut TypedModel) {
        for (name, object) in self.objects("BuildingSurface:Detailed") {
            let Some(surface_type) = self.required_enum(
                "BuildingSurface:Detailed",
                &name,
                &object,
                "surface_type",
                parse_surface_type,
            ) else {
                continue;
            };
            let Some(construction_name) = self.required_string(
                "BuildingSurface:Detailed",
                &name,
                &object,
                "construction_name",
            ) else {
                continue;
            };
            let Some(construction) = self.resolve_name(
                &model.construction_names,
                "BuildingSurface:Detailed",
                &name,
                "construction_name",
                &construction_name,
                "Construction",
            ) else {
                continue;
            };
            let Some(zone_name) =
                self.required_string("BuildingSurface:Detailed", &name, &object, "zone_name")
            else {
                continue;
            };
            let Some(zone) = self.resolve_name(
                &model.zone_names,
                "BuildingSurface:Detailed",
                &name,
                "zone_name",
                &zone_name,
                "Zone",
            ) else {
                continue;
            };
            let Some(outside_boundary_condition) = self.required_enum(
                "BuildingSurface:Detailed",
                &name,
                &object,
                "outside_boundary_condition",
                parse_outside_boundary_condition,
            ) else {
                continue;
            };
            let Some(vertices) =
                self.vertices("BuildingSurface:Detailed", &name, &object, "vertices")
            else {
                continue;
            };
            let Some(id_value) =
                self.checked_id("BuildingSurface:Detailed", &name, model.surfaces.len())
            else {
                continue;
            };
            let id = SurfaceId(id_value);
            if model.surface_names.insert(&name, id).is_some() {
                self.duplicate_name("BuildingSurface:Detailed", &name);
                continue;
            }

            model.surfaces.push(Surface {
                id,
                name: NormalizedName::new(&name),
                surface_type,
                construction,
                zone,
                outside_boundary_condition,
                outside_boundary_condition_object: self
                    .optional_string(
                        "BuildingSurface:Detailed",
                        &name,
                        &object,
                        "outside_boundary_condition_object",
                    )
                    .map(|value| NormalizedName::new(&value)),
                sun_exposure: self.enum_default(
                    "BuildingSurface:Detailed",
                    &name,
                    (&object, "sun_exposure"),
                    SunExposure::SunExposed,
                    "SunExposed",
                    parse_sun_exposure,
                ),
                wind_exposure: self.enum_default(
                    "BuildingSurface:Detailed",
                    &name,
                    (&object, "wind_exposure"),
                    WindExposure::WindExposed,
                    "WindExposed",
                    parse_wind_exposure,
                ),
                view_factor_to_ground: self.auto_default(
                    "BuildingSurface:Detailed",
                    &name,
                    &object,
                    "view_factor_to_ground",
                    AutoOrNumber::AutoCalculate,
                    "Autocalculate",
                ),
                vertices,
            });
        }
    }

    fn objects(&self, object_type: &str) -> Vec<(String, RawObject)> {
        self.raw_model
            .objects
            .get(&ObjectType(object_type.to_string()))
            .map(|objects| {
                objects
                    .iter()
                    .map(|(name, object)| (name.0.clone(), object.clone()))
                    .collect()
            })
            .unwrap_or_default()
    }

    fn single_object(&mut self, object_type: &str) -> Option<(String, RawObject)> {
        let objects = self.objects(object_type);
        if objects.len() > 1 {
            self.error(
                "TooManyObjects",
                object_type,
                None,
                None,
                format!("{object_type} supports only one object in the v0.3 typed subset"),
            );
        }
        objects.into_iter().next()
    }

    fn checked_id(&mut self, object_type: &str, object_name: &str, index: usize) -> Option<u32> {
        match u32::try_from(index) {
            Ok(value) => Some(value),
            Err(_error) => {
                self.error(
                    "TooManyObjects",
                    object_type,
                    Some(object_name),
                    None,
                    "typed ID space exceeded u32 range".to_string(),
                );
                None
            }
        }
    }

    fn resolve_name<T: Copy>(
        &mut self,
        names: &NameMap<T>,
        object_type: &str,
        object_name: &str,
        field: &str,
        target_name: &str,
        target_type: &str,
    ) -> Option<T> {
        if let Some(id) = names.resolve(target_name) {
            return Some(id);
        }

        self.error(
            "MissingReference",
            object_type,
            Some(object_name),
            Some(field),
            format!(
                "{object_type}/{object_name} field {field} references missing {target_type} '{target_name}'"
            ),
        );
        None
    }

    fn required_string(
        &mut self,
        object_type: &str,
        object_name: &str,
        object: &RawObject,
        field: &str,
    ) -> Option<String> {
        match field_value(object, field) {
            Some(RawValue::String(value)) if !value.trim().is_empty() => Some(value.clone()),
            Some(RawValue::String(_value)) => {
                self.error(
                    "MissingRequiredField",
                    object_type,
                    Some(object_name),
                    Some(field),
                    format!("{object_type}/{object_name} requires field {field}"),
                );
                None
            }
            Some(_value) => {
                self.invalid_field_type(object_type, object_name, field, "string");
                None
            }
            None => {
                self.error(
                    "MissingRequiredField",
                    object_type,
                    Some(object_name),
                    Some(field),
                    format!("{object_type}/{object_name} requires field {field}"),
                );
                None
            }
        }
    }

    fn optional_string(
        &mut self,
        object_type: &str,
        object_name: &str,
        object: &RawObject,
        field: &str,
    ) -> Option<String> {
        match field_value(object, field) {
            Some(RawValue::String(value)) if !value.trim().is_empty() => Some(value.clone()),
            Some(RawValue::String(_)) | None => None,
            Some(_value) => {
                self.invalid_field_type(object_type, object_name, field, "string");
                None
            }
        }
    }

    fn optional_number(
        &mut self,
        object_type: &str,
        object_name: &str,
        object: &RawObject,
        field: &str,
    ) -> Option<f64> {
        match field_value(object, field) {
            Some(value) => self.number_value(object_type, object_name, field, value),
            None => None,
        }
    }

    fn number_default(
        &mut self,
        object_type: &str,
        object_name: &str,
        object: &RawObject,
        field: &str,
        default: f64,
    ) -> f64 {
        match self.optional_number(object_type, object_name, object, field) {
            Some(value) => value,
            None => {
                self.record_default(object_type, object_name, field, &format_number(default));
                default
            }
        }
    }

    fn u32_default(
        &mut self,
        object_type: &str,
        object_name: &str,
        object: &RawObject,
        field: &str,
        default: u32,
    ) -> u32 {
        match field_value(object, field) {
            Some(value) => self
                .u32_value(object_type, object_name, field, value)
                .unwrap_or(default),
            None => {
                self.record_default(object_type, object_name, field, &default.to_string());
                default
            }
        }
    }

    fn auto_default(
        &mut self,
        object_type: &str,
        object_name: &str,
        object: &RawObject,
        field: &str,
        default: AutoOrNumber,
        default_label: &str,
    ) -> AutoOrNumber {
        match field_value(object, field) {
            Some(RawValue::String(value))
                if value.trim().is_empty() || value.eq_ignore_ascii_case("Autocalculate") =>
            {
                default
            }
            Some(value) => self
                .number_value(object_type, object_name, field, value)
                .map(AutoOrNumber::Value)
                .unwrap_or(default),
            None => {
                self.record_default(object_type, object_name, field, default_label);
                default
            }
        }
    }

    fn optional_enum<T: Copy>(
        &mut self,
        object_type: &str,
        object_name: &str,
        object: &RawObject,
        field: &str,
        parser: fn(&str) -> Option<T>,
    ) -> Option<T> {
        match field_value(object, field) {
            Some(RawValue::String(value)) if value.trim().is_empty() => None,
            Some(RawValue::String(value)) => match parser(value) {
                Some(parsed) => Some(parsed),
                None => {
                    self.invalid_enum_value(object_type, object_name, field, value);
                    None
                }
            },
            Some(_value) => {
                self.invalid_field_type(object_type, object_name, field, "string enum");
                None
            }
            None => None,
        }
    }

    fn enum_default<T: Copy>(
        &mut self,
        object_type: &str,
        object_name: &str,
        field_ref: (&RawObject, &str),
        default: T,
        default_label: &str,
        parser: fn(&str) -> Option<T>,
    ) -> T {
        let (object, field) = field_ref;
        match field_value(object, field) {
            Some(RawValue::String(value)) if value.trim().is_empty() => {
                self.record_default(object_type, object_name, field, default_label);
                default
            }
            Some(RawValue::String(value)) => match parser(value) {
                Some(parsed) => parsed,
                None => {
                    self.invalid_enum_value(object_type, object_name, field, value);
                    default
                }
            },
            Some(_value) => {
                self.invalid_field_type(object_type, object_name, field, "string enum");
                default
            }
            None => {
                self.record_default(object_type, object_name, field, default_label);
                default
            }
        }
    }

    fn required_enum<T: Copy>(
        &mut self,
        object_type: &str,
        object_name: &str,
        object: &RawObject,
        field: &str,
        parser: fn(&str) -> Option<T>,
    ) -> Option<T> {
        match self.optional_enum(object_type, object_name, object, field, parser) {
            Some(value) => Some(value),
            None => {
                if field_value(object, field).is_none() {
                    self.error(
                        "MissingRequiredField",
                        object_type,
                        Some(object_name),
                        Some(field),
                        format!("{object_type}/{object_name} requires field {field}"),
                    );
                }
                None
            }
        }
    }

    fn number_value(
        &mut self,
        object_type: &str,
        object_name: &str,
        field: &str,
        value: &RawValue,
    ) -> Option<f64> {
        match value {
            RawValue::Number(text) => match text.parse::<f64>() {
                Ok(value) if value.is_finite() => Some(value),
                Ok(_) | Err(_) => {
                    self.error(
                        "InvalidNumber",
                        object_type,
                        Some(object_name),
                        Some(field),
                        format!("{object_type}/{object_name} field {field} is not a finite number"),
                    );
                    None
                }
            },
            _value => {
                self.invalid_field_type(object_type, object_name, field, "number");
                None
            }
        }
    }

    fn u32_value(
        &mut self,
        object_type: &str,
        object_name: &str,
        field: &str,
        value: &RawValue,
    ) -> Option<u32> {
        let number = self.number_value(object_type, object_name, field, value)?;
        if number.fract() == 0.0 && number >= 0.0 && number <= f64::from(u32::MAX) {
            return Some(number as u32);
        }

        self.error(
            "InvalidInteger",
            object_type,
            Some(object_name),
            Some(field),
            format!("{object_type}/{object_name} field {field} must be an unsigned integer"),
        );
        None
    }

    fn vertices(
        &mut self,
        object_type: &str,
        object_name: &str,
        object: &RawObject,
        field: &str,
    ) -> Option<Vec<Point3>> {
        let Some(value) = field_value(object, field) else {
            self.error(
                "MissingRequiredField",
                object_type,
                Some(object_name),
                Some(field),
                format!("{object_type}/{object_name} requires field {field}"),
            );
            return None;
        };
        let RawValue::Array(values) = value else {
            self.invalid_field_type(object_type, object_name, field, "array");
            return None;
        };

        let mut vertices = Vec::new();
        for (index, value) in values.iter().enumerate() {
            let RawValue::Object(fields) = value else {
                self.error(
                    "InvalidFieldType",
                    object_type,
                    Some(object_name),
                    Some(field),
                    format!("{object_type}/{object_name} vertex {index} must be an object"),
                );
                continue;
            };
            let Some(x) = self.vertex_coordinate(
                object_type,
                object_name,
                field,
                fields,
                "vertex_x_coordinate",
            ) else {
                continue;
            };
            let Some(y) = self.vertex_coordinate(
                object_type,
                object_name,
                field,
                fields,
                "vertex_y_coordinate",
            ) else {
                continue;
            };
            let Some(z) = self.vertex_coordinate(
                object_type,
                object_name,
                field,
                fields,
                "vertex_z_coordinate",
            ) else {
                continue;
            };

            vertices.push(Point3 {
                x_m: x,
                y_m: y,
                z_m: z,
            });
        }

        Some(vertices)
    }

    fn vertex_coordinate(
        &mut self,
        object_type: &str,
        object_name: &str,
        array_field: &str,
        fields: &std::collections::BTreeMap<FieldName, RawValue>,
        coordinate_field: &str,
    ) -> Option<f64> {
        let Some(value) = fields.get(&FieldName(coordinate_field.to_string())) else {
            self.error(
                "MissingRequiredField",
                object_type,
                Some(object_name),
                Some(array_field),
                format!(
                    "{object_type}/{object_name} vertex is missing coordinate field {coordinate_field}"
                ),
            );
            return None;
        };

        self.number_value(object_type, object_name, coordinate_field, value)
    }

    fn error(
        &mut self,
        code: &str,
        object_type: &str,
        object_name: Option<&str>,
        field: Option<&str>,
        message: String,
    ) {
        self.diagnostics.push(ModelDiagnostic {
            severity: DiagnosticSeverity::Error,
            code: code.to_string(),
            object_type: object_type.to_string(),
            object_name: object_name.map(str::to_string),
            field: field.map(str::to_string),
            message,
        });
    }

    fn duplicate_name(&mut self, object_type: &str, object_name: &str) {
        self.error(
            "DuplicateName",
            object_type,
            Some(object_name),
            None,
            format!("{object_type}/{object_name} duplicates an existing normalized name"),
        );
    }

    fn invalid_field_type(
        &mut self,
        object_type: &str,
        object_name: &str,
        field: &str,
        expected: &str,
    ) {
        self.error(
            "InvalidFieldType",
            object_type,
            Some(object_name),
            Some(field),
            format!("{object_type}/{object_name} field {field} must be {expected}"),
        );
    }

    fn invalid_enum_value(
        &mut self,
        object_type: &str,
        object_name: &str,
        field: &str,
        value: &str,
    ) {
        self.error(
            "InvalidEnumValue",
            object_type,
            Some(object_name),
            Some(field),
            format!("{object_type}/{object_name} field {field} has unsupported value '{value}'"),
        );
    }

    fn record_default(&mut self, object_type: &str, object_name: &str, field: &str, value: &str) {
        self.defaults_applied.push(DefaultApplication {
            object_type: object_type.to_string(),
            object_name: object_name.to_string(),
            field: field.to_string(),
            value: value.to_string(),
        });
    }
}

fn field_value<'a>(object: &'a RawObject, field: &str) -> Option<&'a RawValue> {
    object.fields.get(&FieldName(field.to_string()))
}

fn format_number(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{value:.1}")
    } else {
        value.to_string()
    }
}

fn parse_terrain(value: &str) -> Option<Terrain> {
    match value {
        value if value.eq_ignore_ascii_case("City") => Some(Terrain::City),
        value if value.eq_ignore_ascii_case("Country") => Some(Terrain::Country),
        value if value.eq_ignore_ascii_case("Ocean") => Some(Terrain::Ocean),
        value if value.eq_ignore_ascii_case("Suburbs") => Some(Terrain::Suburbs),
        value if value.eq_ignore_ascii_case("Urban") => Some(Terrain::Urban),
        _ => None,
    }
}

fn parse_solar_distribution(value: &str) -> Option<SolarDistribution> {
    match value {
        value if value.eq_ignore_ascii_case("MinimalShadowing") => {
            Some(SolarDistribution::MinimalShadowing)
        }
        value if value.eq_ignore_ascii_case("FullExterior") => {
            Some(SolarDistribution::FullExterior)
        }
        value if value.eq_ignore_ascii_case("FullExteriorWithReflections") => {
            Some(SolarDistribution::FullExteriorWithReflections)
        }
        value if value.eq_ignore_ascii_case("FullInteriorAndExterior") => {
            Some(SolarDistribution::FullInteriorAndExterior)
        }
        value if value.eq_ignore_ascii_case("FullInteriorAndExteriorWithReflections") => {
            Some(SolarDistribution::FullInteriorAndExteriorWithReflections)
        }
        _ => None,
    }
}

fn parse_numeric_type(value: &str) -> Option<NumericType> {
    match value {
        value if value.eq_ignore_ascii_case("Continuous") => Some(NumericType::Continuous),
        value if value.eq_ignore_ascii_case("Discrete") => Some(NumericType::Discrete),
        _ => None,
    }
}

fn parse_surface_type(value: &str) -> Option<SurfaceType> {
    match value {
        value if value.eq_ignore_ascii_case("Ceiling") => Some(SurfaceType::Ceiling),
        value if value.eq_ignore_ascii_case("Floor") => Some(SurfaceType::Floor),
        value if value.eq_ignore_ascii_case("Roof") => Some(SurfaceType::Roof),
        value if value.eq_ignore_ascii_case("Wall") => Some(SurfaceType::Wall),
        _ => None,
    }
}

fn parse_outside_boundary_condition(value: &str) -> Option<OutsideBoundaryCondition> {
    match value {
        value if value.eq_ignore_ascii_case("Adiabatic") => {
            Some(OutsideBoundaryCondition::Adiabatic)
        }
        value if value.eq_ignore_ascii_case("Foundation") => {
            Some(OutsideBoundaryCondition::Foundation)
        }
        value if value.eq_ignore_ascii_case("Ground") => Some(OutsideBoundaryCondition::Ground),
        value if value.eq_ignore_ascii_case("Outdoors") => Some(OutsideBoundaryCondition::Outdoors),
        value if value.eq_ignore_ascii_case("Space") => Some(OutsideBoundaryCondition::Space),
        value if value.eq_ignore_ascii_case("Surface") => Some(OutsideBoundaryCondition::Surface),
        value if value.eq_ignore_ascii_case("Zone") => Some(OutsideBoundaryCondition::Zone),
        value
            if value.eq_ignore_ascii_case("GroundBasementPreprocessorAverageFloor")
                || value.eq_ignore_ascii_case("GroundBasementPreprocessorAverageWall")
                || value.eq_ignore_ascii_case("GroundBasementPreprocessorLowerWall")
                || value.eq_ignore_ascii_case("GroundBasementPreprocessorUpperWall")
                || value.eq_ignore_ascii_case("GroundFCfactorMethod")
                || value.eq_ignore_ascii_case("GroundSlabPreprocessorAverage")
                || value.eq_ignore_ascii_case("GroundSlabPreprocessorCore")
                || value.eq_ignore_ascii_case("GroundSlabPreprocessorPerimeter")
                || value.eq_ignore_ascii_case("OtherSideCoefficients")
                || value.eq_ignore_ascii_case("OtherSideConditionsModel") =>
        {
            Some(OutsideBoundaryCondition::Other)
        }
        _ => None,
    }
}

fn parse_sun_exposure(value: &str) -> Option<SunExposure> {
    match value {
        value if value.eq_ignore_ascii_case("NoSun") => Some(SunExposure::NoSun),
        value if value.eq_ignore_ascii_case("SunExposed") => Some(SunExposure::SunExposed),
        _ => None,
    }
}

fn parse_wind_exposure(value: &str) -> Option<WindExposure> {
    match value {
        value if value.eq_ignore_ascii_case("NoWind") => Some(WindExposure::NoWind),
        value if value.eq_ignore_ascii_case("WindExposed") => Some(WindExposure::WindExposed),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{CompileStage, DiagnosticSeverity, ObjectCoverageStatus, compile_raw_model};
    use ep_raw_model::parse_epjson_str;

    #[test]
    fn compile_report_records_typed_and_reference_stages() -> Result<(), Box<dyn std::error::Error>>
    {
        let raw_model = parse_epjson_str(
            r#"{
                "Version": {"Version 1": {"version_identifier": "26.1"}},
                "Timestep": {"Timestep 1": {}},
                "Material:NoMass": {"R13": {}},
                "Construction": {"Wall Construction": {"outside_layer": "R13"}},
                "Zone": {"Zone One": {}},
                "BuildingSurface:Detailed": {
                    "Wall One": {
                        "surface_type": "Wall",
                        "construction_name": "Wall Construction",
                        "zone_name": "zone one",
                        "outside_boundary_condition": "Outdoors",
                        "vertices": [
                            {"vertex_x_coordinate": 0, "vertex_y_coordinate": 0, "vertex_z_coordinate": 0},
                            {"vertex_x_coordinate": 1, "vertex_y_coordinate": 0, "vertex_z_coordinate": 0},
                            {"vertex_x_coordinate": 1, "vertex_y_coordinate": 1, "vertex_z_coordinate": 0}
                        ]
                    }
                }
            }"#,
        )?;

        let result = compile_raw_model(&raw_model);

        assert!(!result.has_errors());
        assert_eq!(
            result.report.completed_stages,
            vec![
                CompileStage::Parse,
                CompileStage::SchemaValidation,
                CompileStage::Normalize,
                CompileStage::TypedConversion,
                CompileStage::ReferenceResolution,
            ]
        );
        let Some(model) = result.model else {
            return Err(std::io::Error::other("expected typed model").into());
        };
        assert_eq!(model.zones.len(), 1);
        assert_eq!(model.surfaces.len(), 1);
        assert_eq!(model.surfaces[0].zone.0, 0);
        assert!(!result.report.defaults_applied.is_empty());

        Ok(())
    }

    #[test]
    fn compile_report_records_typed_and_raw_only_coverage() -> Result<(), Box<dyn std::error::Error>>
    {
        let raw_model = parse_epjson_str(
            r#"{
                "Version": {"Version 1": {"version_identifier": "26.1"}},
                "Output:Variable": {"Zone Temp": {"variable_name": "Zone Air Temperature"}}
            }"#,
        )?;

        let result = compile_raw_model(&raw_model);

        let Some(version) = result
            .report
            .coverage
            .iter()
            .find(|entry| entry.object_type == "Version")
        else {
            return Err(std::io::Error::other("missing Version coverage").into());
        };
        assert_eq!(version.object_count, 1);
        assert_eq!(version.status, ObjectCoverageStatus::Typed);

        let Some(output_variable) = result
            .report
            .coverage
            .iter()
            .find(|entry| entry.object_type == "Output:Variable")
        else {
            return Err(std::io::Error::other("missing Output:Variable coverage").into());
        };
        assert_eq!(output_variable.object_count, 1);
        assert_eq!(output_variable.status, ObjectCoverageStatus::RawOnly);

        Ok(())
    }

    #[test]
    fn parses_material_properties_and_other_equipment() -> Result<(), Box<dyn std::error::Error>> {
        let raw_model = parse_epjson_str(
            r#"{
                "Material": {
                    "Concrete": {
                        "conductivity": 2.0,
                        "density": 2000.0,
                        "specific_heat": 800.0,
                        "thickness": 0.1
                    }
                },
                "Material:NoMass": {
                    "R13": {
                        "thermal_resistance": 2.29
                    }
                },
                "Schedule:Constant": {
                    "Always On": {
                        "hourly_value": 1.0
                    }
                },
                "Zone": {"Zone One": {}},
                "OtherEquipment": {
                    "Plug Load": {
                        "zone_or_zonelist_or_space_or_spacelist_name": "zone one",
                        "schedule_name": "always on",
                        "design_level": 125.0,
                        "fraction_latent": 0.1,
                        "fraction_radiant": 0.2,
                        "fraction_lost": 0.3
                    }
                }
            }"#,
        )?;

        let result = compile_raw_model(&raw_model);

        assert!(!result.has_errors());
        let Some(model) = result.model else {
            return Err(std::io::Error::other("expected typed model").into());
        };
        assert_eq!(model.materials.len(), 2);
        assert_eq!(model.materials[0].thermal_resistance(), Some(0.05));
        assert_eq!(model.materials[0].heat_capacity_per_area(), Some(160_000.0));
        assert_eq!(model.materials[1].thermal_resistance(), Some(2.29));
        assert_eq!(model.other_equipment.len(), 1);
        assert_eq!(model.other_equipment[0].zone.0, 0);
        assert_eq!(model.other_equipment[0].schedule.map(|id| id.0), Some(0));
        assert_eq!(model.other_equipment[0].design_level_w, 125.0);
        assert_eq!(model.other_equipment[0].fraction_latent, 0.1);
        assert_eq!(model.other_equipment[0].fraction_radiant, 0.2);
        assert_eq!(model.other_equipment[0].fraction_lost, 0.3);

        Ok(())
    }

    #[test]
    fn missing_surface_zone_emits_diagnostic() -> Result<(), Box<dyn std::error::Error>> {
        let raw_model = parse_epjson_str(
            r#"{
                "Material:NoMass": {"R13": {}},
                "Construction": {"Wall Construction": {"outside_layer": "R13"}},
                "BuildingSurface:Detailed": {
                    "Wall One": {
                        "surface_type": "Wall",
                        "construction_name": "Wall Construction",
                        "zone_name": "Missing Zone",
                        "outside_boundary_condition": "Outdoors",
                        "vertices": [
                            {"vertex_x_coordinate": 0, "vertex_y_coordinate": 0, "vertex_z_coordinate": 0}
                        ]
                    }
                }
            }"#,
        )?;

        let result = compile_raw_model(&raw_model);

        assert!(result.has_errors());
        assert_eq!(result.model, None);
        assert!(result.report.diagnostics.iter().any(|diagnostic| {
            diagnostic.severity == DiagnosticSeverity::Error
                && diagnostic.code == "MissingReference"
                && diagnostic.field.as_deref() == Some("zone_name")
        }));

        Ok(())
    }
}
