//! Model compiler stage contracts.

use ep_model::{
    AutoOrNumber, AutosizeOrNumber, Building, Construction, ConstructionId,
    DehumidificationControlType, DemandControlledVentilationType, HeatRecoveryType,
    HumidificationControlType, IdealLoadsAirSystem, IdealLoadsAirSystemId, IdealLoadsFuelType,
    IdealLoadsLimit, InternalGainId, LoadDistributionScheme, Material, MaterialId, MaterialKind,
    NameMap, Node, NodeId, NodeList, NodeListId, NormalizedName, NumericType, OtherEquipment,
    OutdoorAirEconomizerType, OutsideBoundaryCondition, Point3, RunPeriod, RunPeriodId,
    ScheduleCompact, ScheduleCompactSegment, ScheduleConstant, ScheduleId, ScheduleTypeLimitId,
    ScheduleTypeLimits, SiteLocation, SolarDistribution, SunExposure, Surface, SurfaceId,
    SurfaceType, Terrain, ThermostatControlObjectType, ThermostatDualSetpoint,
    ThermostatSetpointId, TimestepConfig, TypedModel, Version, WindExposure, Zone,
    ZoneEquipmentConnection, ZoneEquipmentConnectionId, ZoneEquipmentList, ZoneEquipmentListEntry,
    ZoneEquipmentListId, ZoneEquipmentObjectType, ZoneId, ZoneThermostat, ZoneThermostatControl,
    ZoneThermostatId,
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
    "RunPeriod",
    "Site:Location",
    "Material",
    "Material:NoMass",
    "Construction",
    "ScheduleTypeLimits",
    "Schedule:Constant",
    "Schedule:Compact",
    "OtherEquipment",
    "ThermostatSetpoint:DualSetpoint",
    "ZoneControl:Thermostat",
    "NodeList",
    "ZoneHVAC:IdealLoadsAirSystem",
    "ZoneHVAC:EquipmentList",
    "ZoneHVAC:EquipmentConnections",
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
        self.parse_run_periods(&mut model);
        self.parse_site_location(&mut model);
        self.parse_materials(&mut model);
        self.parse_constructions(&mut model);
        self.parse_schedule_type_limits(&mut model);
        self.parse_schedules(&mut model);
        self.parse_compact_schedules(&mut model);
        self.parse_zones(&mut model);
        self.parse_thermostat_dual_setpoints(&mut model);
        self.parse_zone_thermostats(&mut model);
        self.parse_node_lists(&mut model);
        self.parse_ideal_loads_air_systems(&mut model);
        self.parse_zone_equipment_lists(&mut model);
        self.parse_zone_equipment_connections(&mut model);
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

    fn parse_run_periods(&mut self, model: &mut TypedModel) {
        for (name, object) in self.objects("RunPeriod") {
            let Some(id_value) = self.checked_id("RunPeriod", &name, model.run_periods.len())
            else {
                continue;
            };
            let id = RunPeriodId(id_value);
            if model.run_period_names.insert(&name, id).is_some() {
                self.duplicate_name("RunPeriod", &name);
                continue;
            }

            model.run_periods.push(RunPeriod {
                id,
                name: NormalizedName::new(&name),
                begin_month: self.u32_default("RunPeriod", &name, &object, "begin_month", 1),
                begin_day_of_month: self.u32_default(
                    "RunPeriod",
                    &name,
                    &object,
                    "begin_day_of_month",
                    1,
                ),
                begin_year: self.optional_u32("RunPeriod", &name, &object, "begin_year"),
                end_month: self.u32_default("RunPeriod", &name, &object, "end_month", 12),
                end_day_of_month: self.u32_default(
                    "RunPeriod",
                    &name,
                    &object,
                    "end_day_of_month",
                    31,
                ),
                end_year: self.optional_u32("RunPeriod", &name, &object, "end_year"),
                day_of_week_for_start_day: self.optional_enum(
                    "RunPeriod",
                    &name,
                    &object,
                    "day_of_week_for_start_day",
                    parse_day_of_week,
                ),
            });
        }
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

    fn parse_compact_schedules(&mut self, model: &mut TypedModel) {
        for (name, object) in self.objects("Schedule:Compact") {
            let schedule_type_limits = match self.optional_string(
                "Schedule:Compact",
                &name,
                &object,
                "schedule_type_limits_name",
            ) {
                Some(type_limits_name) => self.resolve_name(
                    &model.schedule_type_limit_names,
                    "Schedule:Compact",
                    &name,
                    "schedule_type_limits_name",
                    &type_limits_name,
                    "ScheduleTypeLimits",
                ),
                None => None,
            };
            let schedule_index = model.schedules.len() + model.compact_schedules.len();
            let Some(id_value) = self.checked_id("Schedule:Compact", &name, schedule_index) else {
                continue;
            };
            let id = ScheduleId(id_value);
            if model.schedule_names.insert(&name, id).is_some() {
                self.duplicate_name("Schedule:Compact", &name);
                continue;
            }
            let Some(segments) = self.compact_schedule_segments(&name, &object) else {
                continue;
            };

            model.compact_schedules.push(ScheduleCompact {
                id,
                name: NormalizedName::new(&name),
                schedule_type_limits,
                segments,
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

    fn parse_thermostat_dual_setpoints(&mut self, model: &mut TypedModel) {
        for (name, object) in self.objects("ThermostatSetpoint:DualSetpoint") {
            let Some(heating_setpoint_schedule) = self.required_schedule_reference(
                model,
                "ThermostatSetpoint:DualSetpoint",
                &name,
                &object,
                "heating_setpoint_temperature_schedule_name",
            ) else {
                continue;
            };
            let Some(cooling_setpoint_schedule) = self.required_schedule_reference(
                model,
                "ThermostatSetpoint:DualSetpoint",
                &name,
                &object,
                "cooling_setpoint_temperature_schedule_name",
            ) else {
                continue;
            };
            let Some(id_value) = self.checked_id(
                "ThermostatSetpoint:DualSetpoint",
                &name,
                model.thermostat_dual_setpoints.len(),
            ) else {
                continue;
            };
            let id = ThermostatSetpointId(id_value);
            if model
                .thermostat_dual_setpoint_names
                .insert(&name, id)
                .is_some()
            {
                self.duplicate_name("ThermostatSetpoint:DualSetpoint", &name);
                continue;
            }

            model
                .thermostat_dual_setpoints
                .push(ThermostatDualSetpoint {
                    id,
                    name: NormalizedName::new(&name),
                    heating_setpoint_schedule,
                    cooling_setpoint_schedule,
                });
        }
    }

    fn parse_zone_thermostats(&mut self, model: &mut TypedModel) {
        for (name, object) in self.objects("ZoneControl:Thermostat") {
            let Some(zone_name) = self.required_string(
                "ZoneControl:Thermostat",
                &name,
                &object,
                "zone_or_zonelist_name",
            ) else {
                continue;
            };
            let Some(zone) = self.resolve_name(
                &model.zone_names,
                "ZoneControl:Thermostat",
                &name,
                "zone_or_zonelist_name",
                &zone_name,
                "Zone",
            ) else {
                continue;
            };
            let Some(control_type_schedule) = self.required_schedule_reference(
                model,
                "ZoneControl:Thermostat",
                &name,
                &object,
                "control_type_schedule_name",
            ) else {
                continue;
            };

            let mut controls = Vec::new();
            for index in 1..=4 {
                let object_type_field = format!("control_{index}_object_type");
                let name_field = format!("control_{index}_name");
                let has_any = field_value(&object, &object_type_field).is_some()
                    || field_value(&object, &name_field).is_some();
                if index > 1 && !has_any {
                    continue;
                }

                let Some(object_type) = self.required_enum(
                    "ZoneControl:Thermostat",
                    &name,
                    &object,
                    &object_type_field,
                    parse_thermostat_control_object_type,
                ) else {
                    continue;
                };
                let Some(control_name) =
                    self.required_string("ZoneControl:Thermostat", &name, &object, &name_field)
                else {
                    continue;
                };
                let Some(dual_setpoint) = self.resolve_name(
                    &model.thermostat_dual_setpoint_names,
                    "ZoneControl:Thermostat",
                    &name,
                    &name_field,
                    &control_name,
                    "ThermostatSetpoint:DualSetpoint",
                ) else {
                    continue;
                };

                controls.push(ZoneThermostatControl {
                    object_type,
                    dual_setpoint,
                });
            }
            if controls.is_empty() {
                continue;
            }

            let Some(id_value) = self.checked_id(
                "ZoneControl:Thermostat",
                &name,
                model.zone_thermostats.len(),
            ) else {
                continue;
            };
            let id = ZoneThermostatId(id_value);
            if model.zone_thermostat_names.insert(&name, id).is_some() {
                self.duplicate_name("ZoneControl:Thermostat", &name);
                continue;
            }

            model.zone_thermostats.push(ZoneThermostat {
                id,
                name: NormalizedName::new(&name),
                zone,
                control_type_schedule,
                controls,
                temperature_difference_between_cutout_and_setpoint_delta_c: self.number_default(
                    "ZoneControl:Thermostat",
                    &name,
                    &object,
                    "temperature_difference_between_cutout_and_setpoint",
                    0.0,
                ),
            });
        }
    }

    fn parse_node_lists(&mut self, model: &mut TypedModel) {
        for (name, object) in self.objects("NodeList") {
            if model.node_names.resolve(&name).is_some() {
                self.error(
                    "DuplicateNodeOrNodeListName",
                    "NodeList",
                    Some(&name),
                    None,
                    format!("NodeList/{name} duplicates an existing node name"),
                );
                continue;
            }
            let Some(nodes) = self.node_list_members(model, &name, &object) else {
                continue;
            };
            let Some(id_value) = self.checked_id("NodeList", &name, model.node_lists.len()) else {
                continue;
            };
            let id = NodeListId(id_value);
            if model.node_list_names.insert(&name, id).is_some() {
                self.duplicate_name("NodeList", &name);
                continue;
            }

            model.node_lists.push(NodeList {
                id,
                name: NormalizedName::new(&name),
                nodes,
            });
        }
    }

    fn parse_ideal_loads_air_systems(&mut self, model: &mut TypedModel) {
        for (name, object) in self.objects("ZoneHVAC:IdealLoadsAirSystem") {
            let Some(zone_supply_air_node_name) = self.required_string(
                "ZoneHVAC:IdealLoadsAirSystem",
                &name,
                &object,
                "zone_supply_air_node_name",
            ) else {
                continue;
            };
            self.register_node_or_nodelist_name(model, &zone_supply_air_node_name);
            let zone_exhaust_air_node_name = self
                .optional_string(
                    "ZoneHVAC:IdealLoadsAirSystem",
                    &name,
                    &object,
                    "zone_exhaust_air_node_name",
                )
                .map(|value| {
                    self.register_node(model, &value);
                    NormalizedName::new(&value)
                });
            let system_inlet_air_node_name = self
                .optional_string(
                    "ZoneHVAC:IdealLoadsAirSystem",
                    &name,
                    &object,
                    "system_inlet_air_node_name",
                )
                .map(|value| {
                    self.register_node(model, &value);
                    NormalizedName::new(&value)
                });
            let outdoor_air_inlet_node_name = self
                .optional_string(
                    "ZoneHVAC:IdealLoadsAirSystem",
                    &name,
                    &object,
                    "outdoor_air_inlet_node_name",
                )
                .map(|value| {
                    self.register_node(model, &value);
                    NormalizedName::new(&value)
                });
            let Some(id_value) = self.checked_id(
                "ZoneHVAC:IdealLoadsAirSystem",
                &name,
                model.ideal_loads_air_systems.len(),
            ) else {
                continue;
            };
            let id = IdealLoadsAirSystemId(id_value);
            if model
                .ideal_loads_air_system_names
                .insert(&name, id)
                .is_some()
            {
                self.duplicate_name("ZoneHVAC:IdealLoadsAirSystem", &name);
                continue;
            }

            model.ideal_loads_air_systems.push(IdealLoadsAirSystem {
                id,
                name: NormalizedName::new(&name),
                availability_schedule: self.optional_schedule_reference(
                    model,
                    "ZoneHVAC:IdealLoadsAirSystem",
                    &name,
                    &object,
                    "availability_schedule_name",
                ),
                zone_supply_air_node_name: NormalizedName::new(&zone_supply_air_node_name),
                zone_exhaust_air_node_name,
                system_inlet_air_node_name,
                maximum_heating_supply_air_temperature_c: self.number_range_default(
                    "ZoneHVAC:IdealLoadsAirSystem",
                    &name,
                    &object,
                    "maximum_heating_supply_air_temperature",
                    50.0,
                    -100.0..=200.0,
                ),
                minimum_cooling_supply_air_temperature_c: self.number_range_default(
                    "ZoneHVAC:IdealLoadsAirSystem",
                    &name,
                    &object,
                    "minimum_cooling_supply_air_temperature",
                    13.0,
                    -100.0..=200.0,
                ),
                maximum_heating_supply_air_humidity_ratio: self.number_range_default(
                    "ZoneHVAC:IdealLoadsAirSystem",
                    &name,
                    &object,
                    "maximum_heating_supply_air_humidity_ratio",
                    0.0156,
                    0.0..=1.0,
                ),
                minimum_cooling_supply_air_humidity_ratio: self.number_range_default(
                    "ZoneHVAC:IdealLoadsAirSystem",
                    &name,
                    &object,
                    "minimum_cooling_supply_air_humidity_ratio",
                    0.0077,
                    0.0..=1.0,
                ),
                heating_limit: self.enum_default(
                    "ZoneHVAC:IdealLoadsAirSystem",
                    &name,
                    (&object, "heating_limit"),
                    IdealLoadsLimit::NoLimit,
                    "NoLimit",
                    parse_ideal_loads_limit,
                ),
                maximum_heating_air_flow_rate_m3_per_s: self
                    .optional_autosize_or_nonnegative_number(
                        "ZoneHVAC:IdealLoadsAirSystem",
                        &name,
                        &object,
                        "maximum_heating_air_flow_rate",
                    ),
                maximum_sensible_heating_capacity_w: self.optional_autosize_or_nonnegative_number(
                    "ZoneHVAC:IdealLoadsAirSystem",
                    &name,
                    &object,
                    "maximum_sensible_heating_capacity",
                ),
                cooling_limit: self.enum_default(
                    "ZoneHVAC:IdealLoadsAirSystem",
                    &name,
                    (&object, "cooling_limit"),
                    IdealLoadsLimit::NoLimit,
                    "NoLimit",
                    parse_ideal_loads_limit,
                ),
                maximum_cooling_air_flow_rate_m3_per_s: self
                    .optional_autosize_or_nonnegative_number(
                        "ZoneHVAC:IdealLoadsAirSystem",
                        &name,
                        &object,
                        "maximum_cooling_air_flow_rate",
                    ),
                maximum_total_cooling_capacity_w: self.optional_autosize_or_nonnegative_number(
                    "ZoneHVAC:IdealLoadsAirSystem",
                    &name,
                    &object,
                    "maximum_total_cooling_capacity",
                ),
                heating_availability_schedule: self.optional_schedule_reference(
                    model,
                    "ZoneHVAC:IdealLoadsAirSystem",
                    &name,
                    &object,
                    "heating_availability_schedule_name",
                ),
                cooling_availability_schedule: self.optional_schedule_reference(
                    model,
                    "ZoneHVAC:IdealLoadsAirSystem",
                    &name,
                    &object,
                    "cooling_availability_schedule_name",
                ),
                dehumidification_control_type: self.enum_default(
                    "ZoneHVAC:IdealLoadsAirSystem",
                    &name,
                    (&object, "dehumidification_control_type"),
                    DehumidificationControlType::ConstantSensibleHeatRatio,
                    "ConstantSensibleHeatRatio",
                    parse_dehumidification_control_type,
                ),
                cooling_sensible_heat_ratio: self.number_range_default(
                    "ZoneHVAC:IdealLoadsAirSystem",
                    &name,
                    &object,
                    "cooling_sensible_heat_ratio",
                    0.7,
                    0.0..=1.0,
                ),
                humidification_control_type: self.enum_default(
                    "ZoneHVAC:IdealLoadsAirSystem",
                    &name,
                    (&object, "humidification_control_type"),
                    HumidificationControlType::None,
                    "None",
                    parse_humidification_control_type,
                ),
                design_specification_outdoor_air_object_name: self
                    .optional_string(
                        "ZoneHVAC:IdealLoadsAirSystem",
                        &name,
                        &object,
                        "design_specification_outdoor_air_object_name",
                    )
                    .map(|value| NormalizedName::new(&value)),
                outdoor_air_inlet_node_name,
                demand_controlled_ventilation_type: self.enum_default(
                    "ZoneHVAC:IdealLoadsAirSystem",
                    &name,
                    (&object, "demand_controlled_ventilation_type"),
                    DemandControlledVentilationType::None,
                    "None",
                    parse_demand_controlled_ventilation_type,
                ),
                outdoor_air_economizer_type: self.enum_default(
                    "ZoneHVAC:IdealLoadsAirSystem",
                    &name,
                    (&object, "outdoor_air_economizer_type"),
                    OutdoorAirEconomizerType::NoEconomizer,
                    "NoEconomizer",
                    parse_outdoor_air_economizer_type,
                ),
                heat_recovery_type: self.enum_default(
                    "ZoneHVAC:IdealLoadsAirSystem",
                    &name,
                    (&object, "heat_recovery_type"),
                    HeatRecoveryType::None,
                    "None",
                    parse_heat_recovery_type,
                ),
                sensible_heat_recovery_effectiveness: self.number_range_default(
                    "ZoneHVAC:IdealLoadsAirSystem",
                    &name,
                    &object,
                    "sensible_heat_recovery_effectiveness",
                    0.7,
                    0.0..=1.0,
                ),
                latent_heat_recovery_effectiveness: self.number_range_default(
                    "ZoneHVAC:IdealLoadsAirSystem",
                    &name,
                    &object,
                    "latent_heat_recovery_effectiveness",
                    0.65,
                    0.0..=1.0,
                ),
                design_specification_zonehvac_sizing_object_name: self
                    .optional_string(
                        "ZoneHVAC:IdealLoadsAirSystem",
                        &name,
                        &object,
                        "design_specification_zonehvac_sizing_object_name",
                    )
                    .map(|value| NormalizedName::new(&value)),
                heating_fuel_efficiency_schedule: self.optional_schedule_reference(
                    model,
                    "ZoneHVAC:IdealLoadsAirSystem",
                    &name,
                    &object,
                    "heating_fuel_efficiency_schedule_name",
                ),
                heating_fuel_type: self.enum_default(
                    "ZoneHVAC:IdealLoadsAirSystem",
                    &name,
                    (&object, "heating_fuel_type"),
                    IdealLoadsFuelType::DistrictHeatingWater,
                    "DistrictHeatingWater",
                    parse_ideal_loads_fuel_type,
                ),
                cooling_fuel_efficiency_schedule: self.optional_schedule_reference(
                    model,
                    "ZoneHVAC:IdealLoadsAirSystem",
                    &name,
                    &object,
                    "cooling_fuel_efficiency_schedule_name",
                ),
                cooling_fuel_type: self.enum_default(
                    "ZoneHVAC:IdealLoadsAirSystem",
                    &name,
                    (&object, "cooling_fuel_type"),
                    IdealLoadsFuelType::DistrictCooling,
                    "DistrictCooling",
                    parse_ideal_loads_fuel_type,
                ),
            });
        }
    }

    fn parse_zone_equipment_lists(&mut self, model: &mut TypedModel) {
        for (name, object) in self.objects("ZoneHVAC:EquipmentList") {
            let Some(equipment) = self.zone_equipment_entries(model, &name, &object) else {
                continue;
            };
            let Some(id_value) = self.checked_id(
                "ZoneHVAC:EquipmentList",
                &name,
                model.zone_equipment_lists.len(),
            ) else {
                continue;
            };
            let id = ZoneEquipmentListId(id_value);
            if model.zone_equipment_list_names.insert(&name, id).is_some() {
                self.duplicate_name("ZoneHVAC:EquipmentList", &name);
                continue;
            }

            model.zone_equipment_lists.push(ZoneEquipmentList {
                id,
                name: NormalizedName::new(&name),
                load_distribution_scheme: self.enum_default(
                    "ZoneHVAC:EquipmentList",
                    &name,
                    (&object, "load_distribution_scheme"),
                    LoadDistributionScheme::SequentialLoad,
                    "SequentialLoad",
                    parse_load_distribution_scheme,
                ),
                equipment,
            });
        }
    }

    fn parse_zone_equipment_connections(&mut self, model: &mut TypedModel) {
        for (name, object) in self.objects("ZoneHVAC:EquipmentConnections") {
            let Some(zone_name) =
                self.required_string("ZoneHVAC:EquipmentConnections", &name, &object, "zone_name")
            else {
                continue;
            };
            let Some(zone) = self.resolve_name(
                &model.zone_names,
                "ZoneHVAC:EquipmentConnections",
                &name,
                "zone_name",
                &zone_name,
                "Zone",
            ) else {
                continue;
            };
            let Some(equipment_list_name) = self.required_string(
                "ZoneHVAC:EquipmentConnections",
                &name,
                &object,
                "zone_conditioning_equipment_list_name",
            ) else {
                continue;
            };
            let Some(equipment_list) = self.resolve_name(
                &model.zone_equipment_list_names,
                "ZoneHVAC:EquipmentConnections",
                &name,
                "zone_conditioning_equipment_list_name",
                &equipment_list_name,
                "ZoneHVAC:EquipmentList",
            ) else {
                continue;
            };
            let Some(zone_air_node_name) = self.required_string(
                "ZoneHVAC:EquipmentConnections",
                &name,
                &object,
                "zone_air_node_name",
            ) else {
                continue;
            };
            self.register_node(model, &zone_air_node_name);
            let zone_air_inlet_node_or_nodelist_name = self
                .optional_string(
                    "ZoneHVAC:EquipmentConnections",
                    &name,
                    &object,
                    "zone_air_inlet_node_or_nodelist_name",
                )
                .map(|value| {
                    self.register_node_or_nodelist_name(model, &value);
                    NormalizedName::new(&value)
                });
            let zone_air_exhaust_node_or_nodelist_name = self
                .optional_string(
                    "ZoneHVAC:EquipmentConnections",
                    &name,
                    &object,
                    "zone_air_exhaust_node_or_nodelist_name",
                )
                .map(|value| {
                    self.register_node_or_nodelist_name(model, &value);
                    NormalizedName::new(&value)
                });
            let zone_return_air_node_or_nodelist_name = self
                .optional_string(
                    "ZoneHVAC:EquipmentConnections",
                    &name,
                    &object,
                    "zone_return_air_node_or_nodelist_name",
                )
                .map(|value| {
                    self.register_node_or_nodelist_name(model, &value);
                    NormalizedName::new(&value)
                });
            let zone_return_air_node_1_flow_rate_basis_node_or_nodelist_name = self
                .optional_string(
                    "ZoneHVAC:EquipmentConnections",
                    &name,
                    &object,
                    "zone_return_air_node_1_flow_rate_basis_node_or_nodelist_name",
                )
                .map(|value| {
                    self.register_node_or_nodelist_name(model, &value);
                    NormalizedName::new(&value)
                });
            let Some(id_value) = self.checked_id(
                "ZoneHVAC:EquipmentConnections",
                &name,
                model.zone_equipment_connections.len(),
            ) else {
                continue;
            };
            if model
                .zone_equipment_connections
                .iter()
                .any(|connection| connection.zone == zone)
            {
                self.error(
                    "DuplicateZoneEquipmentConnection",
                    "ZoneHVAC:EquipmentConnections",
                    Some(&name),
                    Some("zone_name"),
                    format!(
                        "ZoneHVAC:EquipmentConnections/{name} duplicates zone equipment connection for zone '{zone_name}'"
                    ),
                );
                continue;
            }

            model
                .zone_equipment_connections
                .push(ZoneEquipmentConnection {
                    id: ZoneEquipmentConnectionId(id_value),
                    zone,
                    equipment_list,
                    zone_air_inlet_node_or_nodelist_name,
                    zone_air_exhaust_node_or_nodelist_name,
                    zone_air_node_name: NormalizedName::new(&zone_air_node_name),
                    zone_return_air_node_or_nodelist_name,
                    zone_return_air_node_1_flow_rate_fraction_schedule: self
                        .optional_schedule_reference(
                            model,
                            "ZoneHVAC:EquipmentConnections",
                            &name,
                            &object,
                            "zone_return_air_node_1_flow_rate_fraction_schedule_name",
                        ),
                    zone_return_air_node_1_flow_rate_basis_node_or_nodelist_name,
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

    fn zone_equipment_entries(
        &mut self,
        model: &TypedModel,
        object_name: &str,
        object: &RawObject,
    ) -> Option<Vec<ZoneEquipmentListEntry>> {
        let Some(value) = field_value(object, "equipment") else {
            self.error(
                "MissingRequiredField",
                "ZoneHVAC:EquipmentList",
                Some(object_name),
                Some("equipment"),
                format!("ZoneHVAC:EquipmentList/{object_name} requires field equipment"),
            );
            return None;
        };
        let RawValue::Array(values) = value else {
            self.invalid_field_type("ZoneHVAC:EquipmentList", object_name, "equipment", "array");
            return None;
        };

        let mut entries = Vec::new();
        for (index, value) in values.iter().enumerate() {
            let RawValue::Object(fields) = value else {
                self.error(
                    "InvalidFieldType",
                    "ZoneHVAC:EquipmentList",
                    Some(object_name),
                    Some("equipment"),
                    format!(
                        "ZoneHVAC:EquipmentList/{object_name} equipment entry {index} must be an object"
                    ),
                );
                continue;
            };
            let entry_object = RawObject {
                fields: fields.clone(),
                source_span: None,
            };
            let entry_name = format!("{object_name}[{index}]");
            let Some(object_type) = self.required_enum(
                "ZoneHVAC:EquipmentList",
                &entry_name,
                &entry_object,
                "zone_equipment_object_type",
                parse_zone_equipment_object_type,
            ) else {
                continue;
            };
            let Some(equipment_name) = self.required_string(
                "ZoneHVAC:EquipmentList",
                &entry_name,
                &entry_object,
                "zone_equipment_name",
            ) else {
                continue;
            };
            let Some(ideal_loads_air_system) = self.resolve_name(
                &model.ideal_loads_air_system_names,
                "ZoneHVAC:EquipmentList",
                &entry_name,
                "zone_equipment_name",
                &equipment_name,
                "ZoneHVAC:IdealLoadsAirSystem",
            ) else {
                continue;
            };
            let Some(cooling_sequence) = self.required_positive_u32(
                "ZoneHVAC:EquipmentList",
                &entry_name,
                &entry_object,
                "zone_equipment_cooling_sequence",
            ) else {
                continue;
            };
            let Some(heating_or_no_load_sequence) = self.required_positive_u32(
                "ZoneHVAC:EquipmentList",
                &entry_name,
                &entry_object,
                "zone_equipment_heating_or_no_load_sequence",
            ) else {
                continue;
            };

            entries.push(ZoneEquipmentListEntry {
                object_type,
                ideal_loads_air_system,
                cooling_sequence,
                heating_or_no_load_sequence,
                sequential_cooling_fraction_schedule: self.optional_schedule_reference(
                    model,
                    "ZoneHVAC:EquipmentList",
                    &entry_name,
                    &entry_object,
                    "zone_equipment_sequential_cooling_fraction_schedule_name",
                ),
                sequential_heating_fraction_schedule: self.optional_schedule_reference(
                    model,
                    "ZoneHVAC:EquipmentList",
                    &entry_name,
                    &entry_object,
                    "zone_equipment_sequential_heating_fraction_schedule_name",
                ),
            });
        }

        if entries.is_empty() {
            self.error(
                "MissingZoneEquipmentEntry",
                "ZoneHVAC:EquipmentList",
                Some(object_name),
                Some("equipment"),
                format!("ZoneHVAC:EquipmentList/{object_name} has no valid equipment entries"),
            );
            return None;
        }

        let mut cooling_sequences = std::collections::BTreeSet::new();
        let mut heating_sequences = std::collections::BTreeSet::new();
        for entry in &entries {
            if !cooling_sequences.insert(entry.cooling_sequence) {
                self.error(
                    "DuplicateZoneEquipmentSequence",
                    "ZoneHVAC:EquipmentList",
                    Some(object_name),
                    Some("zone_equipment_cooling_sequence"),
                    format!(
                        "ZoneHVAC:EquipmentList/{object_name} has duplicate cooling sequence {}",
                        entry.cooling_sequence
                    ),
                );
            }
            if !heating_sequences.insert(entry.heating_or_no_load_sequence) {
                self.error(
                    "DuplicateZoneEquipmentSequence",
                    "ZoneHVAC:EquipmentList",
                    Some(object_name),
                    Some("zone_equipment_heating_or_no_load_sequence"),
                    format!(
                        "ZoneHVAC:EquipmentList/{object_name} has duplicate heating/no-load sequence {}",
                        entry.heating_or_no_load_sequence
                    ),
                );
            }
        }

        Some(entries)
    }

    fn node_list_members(
        &mut self,
        model: &mut TypedModel,
        object_name: &str,
        object: &RawObject,
    ) -> Option<Vec<NodeId>> {
        let Some(value) = field_value(object, "nodes") else {
            self.error(
                "MissingRequiredField",
                "NodeList",
                Some(object_name),
                Some("nodes"),
                format!("NodeList/{object_name} requires field nodes"),
            );
            return None;
        };
        let RawValue::Array(values) = value else {
            self.invalid_field_type("NodeList", object_name, "nodes", "array");
            return None;
        };

        let mut nodes = Vec::new();
        let mut seen = std::collections::BTreeSet::new();
        for (index, value) in values.iter().enumerate() {
            let RawValue::Object(fields) = value else {
                self.error(
                    "InvalidFieldType",
                    "NodeList",
                    Some(object_name),
                    Some("nodes"),
                    format!("NodeList/{object_name} node entry {index} must be an object"),
                );
                continue;
            };
            let entry_object = RawObject {
                fields: fields.clone(),
                source_span: None,
            };
            let entry_name = format!("{object_name}[{index}]");
            let Some(node_name) =
                self.required_string("NodeList", &entry_name, &entry_object, "node_name")
            else {
                continue;
            };
            let normalized = NormalizedName::new(&node_name);
            if !seen.insert(normalized.clone()) {
                self.error(
                    "DuplicateNodeListMember",
                    "NodeList",
                    Some(object_name),
                    Some("node_name"),
                    format!("NodeList/{object_name} duplicates node '{}'", normalized.0),
                );
                continue;
            }
            if let Some(node) = self.register_node(model, &node_name) {
                nodes.push(node);
            }
        }

        if nodes.is_empty() {
            self.error(
                "MissingNodeListMember",
                "NodeList",
                Some(object_name),
                Some("nodes"),
                format!("NodeList/{object_name} has no valid node members"),
            );
            return None;
        }

        Some(nodes)
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

    fn register_node(&mut self, model: &mut TypedModel, node_name: &str) -> Option<NodeId> {
        if let Some(node) = model.node_names.resolve(node_name) {
            return Some(node);
        }
        if model.node_list_names.resolve(node_name).is_some() {
            self.error(
                "DuplicateNodeOrNodeListName",
                "Node",
                Some(node_name),
                None,
                format!("Node '{node_name}' duplicates an existing NodeList name"),
            );
            return None;
        }

        let id_value = self.checked_id("Node", node_name, model.nodes.len())?;
        let id = NodeId(id_value);
        if model.node_names.insert(node_name, id).is_some() {
            self.duplicate_name("Node", node_name);
            return None;
        }
        model.nodes.push(Node {
            id,
            name: NormalizedName::new(node_name),
        });
        Some(id)
    }

    fn register_node_or_nodelist_name(&mut self, model: &mut TypedModel, name: &str) {
        if model.node_list_names.resolve(name).is_none() {
            self.register_node(model, name);
        }
    }

    fn required_schedule_reference(
        &mut self,
        model: &TypedModel,
        object_type: &str,
        object_name: &str,
        object: &RawObject,
        field: &str,
    ) -> Option<ScheduleId> {
        let schedule_name = self.required_string(object_type, object_name, object, field)?;
        self.resolve_name(
            &model.schedule_names,
            object_type,
            object_name,
            field,
            &schedule_name,
            "Schedule",
        )
    }

    fn optional_schedule_reference(
        &mut self,
        model: &TypedModel,
        object_type: &str,
        object_name: &str,
        object: &RawObject,
        field: &str,
    ) -> Option<ScheduleId> {
        let schedule_name = self.optional_string(object_type, object_name, object, field)?;
        self.resolve_name(
            &model.schedule_names,
            object_type,
            object_name,
            field,
            &schedule_name,
            "Schedule",
        )
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

    fn optional_autosize_or_number(
        &mut self,
        object_type: &str,
        object_name: &str,
        object: &RawObject,
        field: &str,
    ) -> Option<AutosizeOrNumber> {
        match field_value(object, field) {
            Some(RawValue::String(value))
                if value.trim().is_empty() || value.eq_ignore_ascii_case("Autosize") =>
            {
                if value.eq_ignore_ascii_case("Autosize") {
                    Some(AutosizeOrNumber::Autosize)
                } else {
                    None
                }
            }
            Some(value) => self
                .number_value(object_type, object_name, field, value)
                .map(AutosizeOrNumber::Value),
            None => None,
        }
    }

    fn optional_autosize_or_nonnegative_number(
        &mut self,
        object_type: &str,
        object_name: &str,
        object: &RawObject,
        field: &str,
    ) -> Option<AutosizeOrNumber> {
        let value = self.optional_autosize_or_number(object_type, object_name, object, field)?;
        match value {
            AutosizeOrNumber::Autosize => Some(AutosizeOrNumber::Autosize),
            AutosizeOrNumber::Value(number) if number >= 0.0 => {
                Some(AutosizeOrNumber::Value(number))
            }
            AutosizeOrNumber::Value(number) => {
                self.error(
                    "InvalidNumericRange",
                    object_type,
                    Some(object_name),
                    Some(field),
                    format!(
                        "{object_type}/{object_name} field {field} must be greater than or equal to 0, got {number}"
                    ),
                );
                None
            }
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

    fn number_range_default(
        &mut self,
        object_type: &str,
        object_name: &str,
        object: &RawObject,
        field: &str,
        default: f64,
        range: std::ops::RangeInclusive<f64>,
    ) -> f64 {
        let value = self.number_default(object_type, object_name, object, field, default);
        if range.contains(&value) {
            return value;
        }

        let min = *range.start();
        let max = *range.end();
        self.error(
            "InvalidNumericRange",
            object_type,
            Some(object_name),
            Some(field),
            format!(
                "{object_type}/{object_name} field {field} must be between {min} and {max}, got {value}"
            ),
        );
        default
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

    fn optional_u32(
        &mut self,
        object_type: &str,
        object_name: &str,
        object: &RawObject,
        field: &str,
    ) -> Option<u32> {
        match field_value(object, field) {
            Some(value) => self.u32_value(object_type, object_name, field, value),
            None => None,
        }
    }

    fn required_positive_u32(
        &mut self,
        object_type: &str,
        object_name: &str,
        object: &RawObject,
        field: &str,
    ) -> Option<u32> {
        let Some(value) = self.optional_u32(object_type, object_name, object, field) else {
            self.error(
                "MissingRequiredField",
                object_type,
                Some(object_name),
                Some(field),
                format!("{object_type}/{object_name} requires field {field}"),
            );
            return None;
        };
        if value > 0 {
            return Some(value);
        }

        self.error(
            "InvalidNumericRange",
            object_type,
            Some(object_name),
            Some(field),
            format!("{object_type}/{object_name} field {field} must be greater than 0"),
        );
        None
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

    fn compact_schedule_segments(
        &mut self,
        object_name: &str,
        object: &RawObject,
    ) -> Option<Vec<ScheduleCompactSegment>> {
        let Some(value) = field_value(object, "data") else {
            self.error(
                "MissingRequiredField",
                "Schedule:Compact",
                Some(object_name),
                Some("data"),
                format!("Schedule:Compact/{object_name} requires field data"),
            );
            return None;
        };
        let RawValue::Array(values) = value else {
            self.invalid_field_type("Schedule:Compact", object_name, "data", "array");
            return None;
        };

        let mut segments = Vec::new();
        let mut pending_until = None;
        let mut saw_for_all_days = false;
        for (index, value) in values.iter().enumerate() {
            let Some(field_value) = compact_data_field(value) else {
                self.error(
                    "InvalidFieldType",
                    "Schedule:Compact",
                    Some(object_name),
                    Some("data"),
                    format!("Schedule:Compact/{object_name} data entry {index} must contain field"),
                );
                continue;
            };
            match field_value {
                RawValue::String(text) if compact_directive(text, "For") => {
                    if !text.to_ascii_lowercase().contains("alldays") {
                        self.error(
                            "UnsupportedScheduleCompact",
                            "Schedule:Compact",
                            Some(object_name),
                            Some("data"),
                            format!(
                                "Schedule:Compact/{object_name} supports only For: AllDays in the current subset"
                            ),
                        );
                    }
                    saw_for_all_days = true;
                }
                RawValue::String(text) if compact_directive(text, "Through") => {}
                RawValue::String(text) if compact_directive(text, "Until") => {
                    pending_until = parse_until_minute(text);
                    if pending_until.is_none() {
                        self.error(
                            "InvalidScheduleCompactUntil",
                            "Schedule:Compact",
                            Some(object_name),
                            Some("data"),
                            format!(
                                "Schedule:Compact/{object_name} has invalid Until directive '{text}'"
                            ),
                        );
                    }
                }
                RawValue::Number(_text) => {
                    let Some(until_minute_of_day) = pending_until.take() else {
                        self.error(
                            "InvalidScheduleCompactValue",
                            "Schedule:Compact",
                            Some(object_name),
                            Some("data"),
                            format!(
                                "Schedule:Compact/{object_name} value appears before an Until directive"
                            ),
                        );
                        continue;
                    };
                    let Some(value) =
                        self.number_value("Schedule:Compact", object_name, "data", field_value)
                    else {
                        continue;
                    };
                    segments.push(ScheduleCompactSegment {
                        until_minute_of_day,
                        value,
                    });
                }
                RawValue::String(text) => {
                    self.error(
                        "UnsupportedScheduleCompact",
                        "Schedule:Compact",
                        Some(object_name),
                        Some("data"),
                        format!(
                            "Schedule:Compact/{object_name} has unsupported directive '{text}'"
                        ),
                    );
                }
                _ => self.invalid_field_type(
                    "Schedule:Compact",
                    object_name,
                    "data",
                    "string or number",
                ),
            }
        }

        if !saw_for_all_days {
            self.error(
                "UnsupportedScheduleCompact",
                "Schedule:Compact",
                Some(object_name),
                Some("data"),
                format!(
                    "Schedule:Compact/{object_name} requires For: AllDays in the current subset"
                ),
            );
        }
        if segments.is_empty() {
            self.error(
                "UnsupportedScheduleCompact",
                "Schedule:Compact",
                Some(object_name),
                Some("data"),
                format!("Schedule:Compact/{object_name} has no supported Until/value segments"),
            );
            return None;
        }

        Some(segments)
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

fn compact_data_field(value: &RawValue) -> Option<&RawValue> {
    let RawValue::Object(fields) = value else {
        return None;
    };
    fields.get(&FieldName("field".to_string()))
}

fn compact_directive(value: &str, directive: &str) -> bool {
    value
        .trim_start()
        .get(..directive.len())
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case(directive))
        && value[directive.len()..].trim_start().starts_with(':')
}

fn parse_until_minute(value: &str) -> Option<u32> {
    let (_directive, time) = value.split_once(':')?;
    let time = time.trim();
    let (hour, minute) = time.split_once(':')?;
    let hour = hour.trim().parse::<u32>().ok()?;
    let minute = minute.trim().parse::<u32>().ok()?;
    if hour > 24 || minute >= 60 || (hour == 24 && minute != 0) {
        return None;
    }
    let minute_of_day = hour * 60 + minute;
    if minute_of_day == 0 {
        None
    } else {
        Some(minute_of_day)
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

fn parse_thermostat_control_object_type(value: &str) -> Option<ThermostatControlObjectType> {
    match value {
        value if value.eq_ignore_ascii_case("ThermostatSetpoint:DualSetpoint") => {
            Some(ThermostatControlObjectType::DualSetpoint)
        }
        _ => None,
    }
}

fn parse_ideal_loads_limit(value: &str) -> Option<IdealLoadsLimit> {
    match value {
        value if value.eq_ignore_ascii_case("NoLimit") => Some(IdealLoadsLimit::NoLimit),
        value if value.eq_ignore_ascii_case("LimitFlowRate") => {
            Some(IdealLoadsLimit::LimitFlowRate)
        }
        value if value.eq_ignore_ascii_case("LimitCapacity") => {
            Some(IdealLoadsLimit::LimitCapacity)
        }
        value if value.eq_ignore_ascii_case("LimitFlowRateAndCapacity") => {
            Some(IdealLoadsLimit::LimitFlowRateAndCapacity)
        }
        _ => None,
    }
}

fn parse_dehumidification_control_type(value: &str) -> Option<DehumidificationControlType> {
    match value {
        value if value.eq_ignore_ascii_case("None") => Some(DehumidificationControlType::None),
        value if value.eq_ignore_ascii_case("ConstantSensibleHeatRatio") => {
            Some(DehumidificationControlType::ConstantSensibleHeatRatio)
        }
        value if value.eq_ignore_ascii_case("ConstantSupplyHumidityRatio") => {
            Some(DehumidificationControlType::ConstantSupplyHumidityRatio)
        }
        value if value.eq_ignore_ascii_case("Humidistat") => {
            Some(DehumidificationControlType::Humidistat)
        }
        _ => None,
    }
}

fn parse_humidification_control_type(value: &str) -> Option<HumidificationControlType> {
    match value {
        value if value.eq_ignore_ascii_case("None") => Some(HumidificationControlType::None),
        value if value.eq_ignore_ascii_case("ConstantSupplyHumidityRatio") => {
            Some(HumidificationControlType::ConstantSupplyHumidityRatio)
        }
        value if value.eq_ignore_ascii_case("Humidistat") => {
            Some(HumidificationControlType::Humidistat)
        }
        _ => None,
    }
}

fn parse_demand_controlled_ventilation_type(
    value: &str,
) -> Option<DemandControlledVentilationType> {
    match value {
        value if value.eq_ignore_ascii_case("None") => Some(DemandControlledVentilationType::None),
        value if value.eq_ignore_ascii_case("OccupancySchedule") => {
            Some(DemandControlledVentilationType::OccupancySchedule)
        }
        value if value.eq_ignore_ascii_case("CO2Setpoint") => {
            Some(DemandControlledVentilationType::Co2Setpoint)
        }
        _ => None,
    }
}

fn parse_outdoor_air_economizer_type(value: &str) -> Option<OutdoorAirEconomizerType> {
    match value {
        value if value.eq_ignore_ascii_case("NoEconomizer") => {
            Some(OutdoorAirEconomizerType::NoEconomizer)
        }
        value if value.eq_ignore_ascii_case("DifferentialDryBulb") => {
            Some(OutdoorAirEconomizerType::DifferentialDryBulb)
        }
        value if value.eq_ignore_ascii_case("DifferentialEnthalpy") => {
            Some(OutdoorAirEconomizerType::DifferentialEnthalpy)
        }
        _ => None,
    }
}

fn parse_heat_recovery_type(value: &str) -> Option<HeatRecoveryType> {
    match value {
        value if value.eq_ignore_ascii_case("None") => Some(HeatRecoveryType::None),
        value if value.eq_ignore_ascii_case("Sensible") => Some(HeatRecoveryType::Sensible),
        value if value.eq_ignore_ascii_case("Enthalpy") => Some(HeatRecoveryType::Enthalpy),
        _ => None,
    }
}

fn parse_ideal_loads_fuel_type(value: &str) -> Option<IdealLoadsFuelType> {
    match value {
        value if value.eq_ignore_ascii_case("Coal") => Some(IdealLoadsFuelType::Coal),
        value if value.eq_ignore_ascii_case("Diesel") => Some(IdealLoadsFuelType::Diesel),
        value if value.eq_ignore_ascii_case("DistrictCooling") => {
            Some(IdealLoadsFuelType::DistrictCooling)
        }
        value if value.eq_ignore_ascii_case("DistrictHeatingSteam") => {
            Some(IdealLoadsFuelType::DistrictHeatingSteam)
        }
        value if value.eq_ignore_ascii_case("DistrictHeatingWater") => {
            Some(IdealLoadsFuelType::DistrictHeatingWater)
        }
        value if value.eq_ignore_ascii_case("Electricity") => Some(IdealLoadsFuelType::Electricity),
        value if value.eq_ignore_ascii_case("FuelOilNo1") => Some(IdealLoadsFuelType::FuelOilNo1),
        value if value.eq_ignore_ascii_case("FuelOilNo2") => Some(IdealLoadsFuelType::FuelOilNo2),
        value if value.eq_ignore_ascii_case("Gasoline") => Some(IdealLoadsFuelType::Gasoline),
        value if value.eq_ignore_ascii_case("NaturalGas") => Some(IdealLoadsFuelType::NaturalGas),
        value if value.eq_ignore_ascii_case("OtherFuel1") => Some(IdealLoadsFuelType::OtherFuel1),
        value if value.eq_ignore_ascii_case("OtherFuel2") => Some(IdealLoadsFuelType::OtherFuel2),
        value if value.eq_ignore_ascii_case("Propane") => Some(IdealLoadsFuelType::Propane),
        _ => None,
    }
}

fn parse_load_distribution_scheme(value: &str) -> Option<LoadDistributionScheme> {
    match value {
        value if value.eq_ignore_ascii_case("SequentialLoad") => {
            Some(LoadDistributionScheme::SequentialLoad)
        }
        value if value.eq_ignore_ascii_case("UniformLoad") => {
            Some(LoadDistributionScheme::UniformLoad)
        }
        value if value.eq_ignore_ascii_case("UniformPLR") => {
            Some(LoadDistributionScheme::UniformPlr)
        }
        value if value.eq_ignore_ascii_case("SequentialUniformPLR") => {
            Some(LoadDistributionScheme::SequentialUniformPlr)
        }
        _ => None,
    }
}

fn parse_zone_equipment_object_type(value: &str) -> Option<ZoneEquipmentObjectType> {
    match value {
        value if value.eq_ignore_ascii_case("ZoneHVAC:IdealLoadsAirSystem") => {
            Some(ZoneEquipmentObjectType::IdealLoadsAirSystem)
        }
        _ => None,
    }
}

fn parse_day_of_week(value: &str) -> Option<ep_model::DayOfWeek> {
    match value {
        value if value.eq_ignore_ascii_case("Monday") => Some(ep_model::DayOfWeek::Monday),
        value if value.eq_ignore_ascii_case("Tuesday") => Some(ep_model::DayOfWeek::Tuesday),
        value if value.eq_ignore_ascii_case("Wednesday") => Some(ep_model::DayOfWeek::Wednesday),
        value if value.eq_ignore_ascii_case("Thursday") => Some(ep_model::DayOfWeek::Thursday),
        value if value.eq_ignore_ascii_case("Friday") => Some(ep_model::DayOfWeek::Friday),
        value if value.eq_ignore_ascii_case("Saturday") => Some(ep_model::DayOfWeek::Saturday),
        value if value.eq_ignore_ascii_case("Sunday") => Some(ep_model::DayOfWeek::Sunday),
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
    use ep_model::{
        AutosizeOrNumber, DayOfWeek, DehumidificationControlType, HumidificationControlType,
        IdealLoadsLimit, LoadDistributionScheme, ModelGraph, OutdoorAirEconomizerType,
    };
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
    fn parses_run_period_dates() -> Result<(), Box<dyn std::error::Error>> {
        let raw_model = parse_epjson_str(
            r#"{
                "RunPeriod": {
                    "Run Period 1": {
                        "begin_month": 1,
                        "begin_day_of_month": 2,
                        "begin_year": 2013,
                        "end_month": 1,
                        "end_day_of_month": 3,
                        "end_year": 2013,
                        "day_of_week_for_start_day": "Wednesday"
                    }
                }
            }"#,
        )?;

        let result = compile_raw_model(&raw_model);

        assert!(!result.has_errors());
        let Some(model) = result.model else {
            return Err(std::io::Error::other("expected typed model").into());
        };
        assert_eq!(model.run_periods.len(), 1);
        assert_eq!(model.run_periods[0].begin_month, 1);
        assert_eq!(model.run_periods[0].begin_day_of_month, 2);
        assert_eq!(model.run_periods[0].begin_year, Some(2013));
        assert_eq!(model.run_periods[0].end_day_of_month, 3);
        assert_eq!(
            model.run_periods[0].day_of_week_for_start_day,
            Some(DayOfWeek::Wednesday)
        );

        Ok(())
    }

    #[test]
    fn parses_schedule_compact_all_days_segments() -> Result<(), Box<dyn std::error::Error>> {
        let raw_model = parse_epjson_str(
            r#"{
                "ScheduleTypeLimits": {
                    "Fraction": {
                        "lower_limit_value": 0.0,
                        "upper_limit_value": 1.0,
                        "numeric_type": "Continuous"
                    }
                },
                "Schedule:Compact": {
                    "Office Occupancy": {
                        "schedule_type_limits_name": "Fraction",
                        "data": [
                            {"field": "Through: 12/31"},
                            {"field": "For: AllDays"},
                            {"field": "Until: 08:00"},
                            {"field": 0.0},
                            {"field": "Until: 18:00"},
                            {"field": 1.0},
                            {"field": "Until: 24:00"},
                            {"field": 0.0}
                        ]
                    }
                }
            }"#,
        )?;

        let result = compile_raw_model(&raw_model);

        assert!(!result.has_errors());
        let Some(model) = result.model else {
            return Err(std::io::Error::other("expected typed model").into());
        };
        assert_eq!(model.compact_schedules.len(), 1);
        assert_eq!(model.compact_schedules[0].name.0, "OFFICE OCCUPANCY");
        assert_eq!(
            model.compact_schedules[0]
                .schedule_type_limits
                .map(|id| id.0),
            Some(0)
        );
        assert_eq!(model.compact_schedules[0].segments.len(), 3);
        assert_eq!(
            model.compact_schedules[0].segments[0].until_minute_of_day,
            8 * 60
        );
        assert_eq!(model.compact_schedules[0].segments[1].value, 1.0);

        Ok(())
    }

    #[test]
    fn parses_thermostat_and_ideal_loads_graph() -> Result<(), Box<dyn std::error::Error>> {
        let raw_model = parse_epjson_str(
            r#"{
                "Schedule:Constant": {
                    "Control Type": {"hourly_value": 4},
                    "Heating Setpoint": {"hourly_value": 21},
                    "Cooling Setpoint": {"hourly_value": 24}
                },
                "Zone": {"Zone One": {}},
                "ThermostatSetpoint:DualSetpoint": {
                    "Dual Setpoints": {
                        "heating_setpoint_temperature_schedule_name": "Heating Setpoint",
                        "cooling_setpoint_temperature_schedule_name": "Cooling Setpoint"
                    }
                },
                "ZoneControl:Thermostat": {
                    "Zone Thermostat": {
                        "zone_or_zonelist_name": "Zone One",
                        "control_type_schedule_name": "Control Type",
                        "control_1_object_type": "ThermostatSetpoint:DualSetpoint",
                        "control_1_name": "Dual Setpoints",
                        "temperature_difference_between_cutout_and_setpoint": 0.5
                    }
                },
                "NodeList": {
                    "Zone Inlets": {
                        "nodes": [
                            {"node_name": "Zone One Inlet"}
                        ]
                    }
                },
                "ZoneHVAC:IdealLoadsAirSystem": {
                    "Zone Ideal Loads": {
                        "zone_supply_air_node_name": "Zone Inlets",
                        "maximum_heating_supply_air_temperature": 50,
                        "minimum_cooling_supply_air_temperature": 13,
                        "maximum_heating_supply_air_humidity_ratio": 0.015,
                        "minimum_cooling_supply_air_humidity_ratio": 0.009,
                        "heating_limit": "LimitFlowRate",
                        "maximum_heating_air_flow_rate": "Autosize",
                        "cooling_limit": "LimitFlowRateAndCapacity",
                        "maximum_cooling_air_flow_rate": 0.25,
                        "maximum_total_cooling_capacity": "Autosize",
                        "dehumidification_control_type": "ConstantSupplyHumidityRatio",
                        "humidification_control_type": "ConstantSupplyHumidityRatio",
                        "outdoor_air_economizer_type": "NoEconomizer"
                    }
                },
                "ZoneHVAC:EquipmentList": {
                    "Zone Equipment": {
                        "load_distribution_scheme": "SequentialLoad",
                        "equipment": [
                            {
                                "zone_equipment_object_type": "ZoneHVAC:IdealLoadsAirSystem",
                                "zone_equipment_name": "Zone Ideal Loads",
                                "zone_equipment_cooling_sequence": 1,
                                "zone_equipment_heating_or_no_load_sequence": 1
                            }
                        ]
                    }
                },
                "ZoneHVAC:EquipmentConnections": {
                    "Zone One": {
                        "zone_name": "Zone One",
                        "zone_conditioning_equipment_list_name": "Zone Equipment",
                        "zone_air_inlet_node_or_nodelist_name": "Zone Inlets",
                        "zone_air_node_name": "Zone One Air Node",
                        "zone_return_air_node_or_nodelist_name": "Zone One Return"
                    }
                }
            }"#,
        )?;

        let result = compile_raw_model(&raw_model);

        assert!(!result.has_errors());
        let Some(model) = result.model else {
            return Err(std::io::Error::other("expected typed model").into());
        };
        assert_eq!(model.thermostat_dual_setpoints.len(), 1);
        assert_eq!(model.zone_thermostats.len(), 1);
        assert_eq!(model.ideal_loads_air_systems.len(), 1);
        assert_eq!(model.zone_equipment_lists.len(), 1);
        assert_eq!(model.zone_equipment_connections.len(), 1);
        assert_eq!(model.nodes.len(), 3);
        assert_eq!(model.node_lists.len(), 1);
        assert_eq!(model.node_lists[0].nodes.len(), 1);
        assert_eq!(model.zone_thermostats[0].zone.0, 0);
        assert_eq!(model.zone_thermostats[0].controls[0].dual_setpoint.0, 0);
        assert_eq!(
            model.zone_thermostats[0].temperature_difference_between_cutout_and_setpoint_delta_c,
            0.5
        );
        assert_eq!(
            model.ideal_loads_air_systems[0].heating_limit,
            IdealLoadsLimit::LimitFlowRate
        );
        assert_eq!(
            model.ideal_loads_air_systems[0].maximum_heating_air_flow_rate_m3_per_s,
            Some(AutosizeOrNumber::Autosize)
        );
        assert_eq!(
            model.ideal_loads_air_systems[0].maximum_cooling_air_flow_rate_m3_per_s,
            Some(AutosizeOrNumber::Value(0.25))
        );
        assert_eq!(
            model.ideal_loads_air_systems[0].dehumidification_control_type,
            DehumidificationControlType::ConstantSupplyHumidityRatio
        );
        assert_eq!(
            model.ideal_loads_air_systems[0].humidification_control_type,
            HumidificationControlType::ConstantSupplyHumidityRatio
        );
        assert_eq!(
            model.ideal_loads_air_systems[0].outdoor_air_economizer_type,
            OutdoorAirEconomizerType::NoEconomizer
        );
        assert_eq!(
            model.zone_equipment_lists[0].load_distribution_scheme,
            LoadDistributionScheme::SequentialLoad
        );

        let graph = ModelGraph::from_typed(&model);
        assert_eq!(graph.zone_thermostats.len(), 1);
        assert_eq!(graph.thermostat_setpoints.len(), 1);
        assert_eq!(graph.zone_ideal_loads.len(), 1);
        assert_eq!(graph.node_list_members.len(), 1);
        assert_eq!(graph.ideal_loads_supply_nodes.len(), 1);
        assert_eq!(graph.zone_air_nodes.len(), 1);
        assert_eq!(graph.zone_ideal_loads[0].cooling_sequence, 1);
        assert_eq!(graph.zone_ideal_loads[0].heating_or_no_load_sequence, 1);

        Ok(())
    }

    #[test]
    fn rejects_missing_thermostat_setpoint_schedule() -> Result<(), Box<dyn std::error::Error>> {
        let raw_model = parse_epjson_str(
            r#"{
                "Schedule:Constant": {
                    "Cooling Setpoint": {"hourly_value": 24}
                },
                "ThermostatSetpoint:DualSetpoint": {
                    "Dual Setpoints": {
                        "heating_setpoint_temperature_schedule_name": "Missing Heating",
                        "cooling_setpoint_temperature_schedule_name": "Cooling Setpoint"
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
                && diagnostic.object_type == "ThermostatSetpoint:DualSetpoint"
                && diagnostic.field.as_deref() == Some("heating_setpoint_temperature_schedule_name")
        }));

        Ok(())
    }

    #[test]
    fn rejects_unsupported_thermostat_control_type() -> Result<(), Box<dyn std::error::Error>> {
        let raw_model = parse_epjson_str(
            r#"{
                "Schedule:Constant": {
                    "Control Type": {"hourly_value": 4},
                    "Heating Setpoint": {"hourly_value": 21},
                    "Cooling Setpoint": {"hourly_value": 24}
                },
                "Zone": {"Zone One": {}},
                "ThermostatSetpoint:DualSetpoint": {
                    "Dual Setpoints": {
                        "heating_setpoint_temperature_schedule_name": "Heating Setpoint",
                        "cooling_setpoint_temperature_schedule_name": "Cooling Setpoint"
                    }
                },
                "ZoneControl:Thermostat": {
                    "Zone Thermostat": {
                        "zone_or_zonelist_name": "Zone One",
                        "control_type_schedule_name": "Control Type",
                        "control_1_object_type": "ThermostatSetpoint:SingleHeating",
                        "control_1_name": "Dual Setpoints"
                    }
                }
            }"#,
        )?;

        let result = compile_raw_model(&raw_model);

        assert!(result.has_errors());
        assert_eq!(result.model, None);
        assert!(result.report.diagnostics.iter().any(|diagnostic| {
            diagnostic.severity == DiagnosticSeverity::Error
                && diagnostic.code == "InvalidEnumValue"
                && diagnostic.object_type == "ZoneControl:Thermostat"
                && diagnostic.field.as_deref() == Some("control_1_object_type")
        }));

        Ok(())
    }

    #[test]
    fn rejects_missing_ideal_loads_equipment_reference() -> Result<(), Box<dyn std::error::Error>> {
        let raw_model = parse_epjson_str(
            r#"{
                "ZoneHVAC:EquipmentList": {
                    "Zone Equipment": {
                        "equipment": [
                            {
                                "zone_equipment_object_type": "ZoneHVAC:IdealLoadsAirSystem",
                                "zone_equipment_name": "Missing Ideal Loads",
                                "zone_equipment_cooling_sequence": 1,
                                "zone_equipment_heating_or_no_load_sequence": 1
                            }
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
                && diagnostic.object_type == "ZoneHVAC:EquipmentList"
                && diagnostic.field.as_deref() == Some("zone_equipment_name")
        }));

        Ok(())
    }

    #[test]
    fn rejects_unsupported_zone_equipment_type() -> Result<(), Box<dyn std::error::Error>> {
        let raw_model = parse_epjson_str(
            r#"{
                "ZoneHVAC:EquipmentList": {
                    "Zone Equipment": {
                        "equipment": [
                            {
                                "zone_equipment_object_type": "Fan:ConstantVolume",
                                "zone_equipment_name": "Supply Fan",
                                "zone_equipment_cooling_sequence": 1,
                                "zone_equipment_heating_or_no_load_sequence": 1
                            }
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
                && diagnostic.code == "InvalidEnumValue"
                && diagnostic.object_type == "ZoneHVAC:EquipmentList"
                && diagnostic.field.as_deref() == Some("zone_equipment_object_type")
        }));

        Ok(())
    }

    #[test]
    fn rejects_nodelist_name_that_duplicates_registered_node()
    -> Result<(), Box<dyn std::error::Error>> {
        let raw_model = parse_epjson_str(
            r#"{
                "NodeList": {
                    "A Source List": {
                        "nodes": [{"node_name": "Shared Name"}]
                    },
                    "Shared Name": {
                        "nodes": [{"node_name": "Other Node"}]
                    }
                }
            }"#,
        )?;

        let result = compile_raw_model(&raw_model);

        assert!(result.has_errors());
        assert_eq!(result.model, None);
        assert!(result.report.diagnostics.iter().any(|diagnostic| {
            diagnostic.severity == DiagnosticSeverity::Error
                && diagnostic.code == "DuplicateNodeOrNodeListName"
                && diagnostic.object_type == "NodeList"
                && diagnostic.object_name.as_deref() == Some("Shared Name")
        }));

        Ok(())
    }

    #[test]
    fn rejects_direct_node_name_that_duplicates_nodelist() -> Result<(), Box<dyn std::error::Error>>
    {
        let raw_model = parse_epjson_str(
            r#"{
                "Zone": {"Zone One": {}},
                "NodeList": {
                    "Zone Air Node": {
                        "nodes": [{"node_name": "Zone Inlet Node"}]
                    }
                },
                "ZoneHVAC:IdealLoadsAirSystem": {
                    "Zone Ideal Loads": {
                        "zone_supply_air_node_name": "Zone Inlet Node"
                    }
                },
                "ZoneHVAC:EquipmentList": {
                    "Zone Equipment": {
                        "equipment": [
                            {
                                "zone_equipment_object_type": "ZoneHVAC:IdealLoadsAirSystem",
                                "zone_equipment_name": "Zone Ideal Loads",
                                "zone_equipment_cooling_sequence": 1,
                                "zone_equipment_heating_or_no_load_sequence": 1
                            }
                        ]
                    }
                },
                "ZoneHVAC:EquipmentConnections": {
                    "Zone Connection": {
                        "zone_name": "Zone One",
                        "zone_conditioning_equipment_list_name": "Zone Equipment",
                        "zone_air_node_name": "Zone Air Node"
                    }
                }
            }"#,
        )?;

        let result = compile_raw_model(&raw_model);

        assert!(result.has_errors());
        assert_eq!(result.model, None);
        assert!(result.report.diagnostics.iter().any(|diagnostic| {
            diagnostic.severity == DiagnosticSeverity::Error
                && diagnostic.code == "DuplicateNodeOrNodeListName"
                && diagnostic.object_type == "Node"
                && diagnostic.object_name.as_deref() == Some("Zone Air Node")
        }));

        Ok(())
    }

    #[test]
    fn rejects_ideal_loads_invalid_numeric_ranges() -> Result<(), Box<dyn std::error::Error>> {
        let raw_model = parse_epjson_str(
            r#"{
                "ZoneHVAC:IdealLoadsAirSystem": {
                    "Zone Ideal Loads": {
                        "zone_supply_air_node_name": "Zone One Inlet",
                        "maximum_heating_supply_air_humidity_ratio": -0.001,
                        "minimum_cooling_supply_air_humidity_ratio": 1.2,
                        "maximum_cooling_air_flow_rate": -0.25,
                        "cooling_sensible_heat_ratio": 1.5,
                        "sensible_heat_recovery_effectiveness": -0.1,
                        "latent_heat_recovery_effectiveness": 1.1
                    }
                }
            }"#,
        )?;

        let result = compile_raw_model(&raw_model);

        assert!(result.has_errors());
        assert_eq!(result.model, None);
        let invalid_range_count = result
            .report
            .diagnostics
            .iter()
            .filter(|diagnostic| {
                diagnostic.severity == DiagnosticSeverity::Error
                    && diagnostic.code == "InvalidNumericRange"
                    && diagnostic.object_type == "ZoneHVAC:IdealLoadsAirSystem"
            })
            .count();
        assert_eq!(invalid_range_count, 6);

        Ok(())
    }

    #[test]
    fn rejects_equipment_sequence_and_connection_duplicates()
    -> Result<(), Box<dyn std::error::Error>> {
        let raw_model = parse_epjson_str(
            r#"{
                "Zone": {"Zone One": {}},
                "ZoneHVAC:IdealLoadsAirSystem": {
                    "Zone Ideal Loads One": {"zone_supply_air_node_name": "Zone One Inlet"},
                    "Zone Ideal Loads Two": {"zone_supply_air_node_name": "Zone One Inlet 2"}
                },
                "ZoneHVAC:EquipmentList": {
                    "Zone Equipment": {
                        "equipment": [
                            {
                                "zone_equipment_object_type": "ZoneHVAC:IdealLoadsAirSystem",
                                "zone_equipment_name": "Zone Ideal Loads One",
                                "zone_equipment_cooling_sequence": 1,
                                "zone_equipment_heating_or_no_load_sequence": 1
                            },
                            {
                                "zone_equipment_object_type": "ZoneHVAC:IdealLoadsAirSystem",
                                "zone_equipment_name": "Zone Ideal Loads Two",
                                "zone_equipment_cooling_sequence": 1,
                                "zone_equipment_heating_or_no_load_sequence": 1
                            }
                        ]
                    }
                },
                "ZoneHVAC:EquipmentConnections": {
                    "Zone Connection One": {
                        "zone_name": "Zone One",
                        "zone_conditioning_equipment_list_name": "Zone Equipment",
                        "zone_air_node_name": "Zone One Air Node"
                    },
                    "Zone Connection Two": {
                        "zone_name": "Zone One",
                        "zone_conditioning_equipment_list_name": "Zone Equipment",
                        "zone_air_node_name": "Zone One Air Node"
                    }
                }
            }"#,
        )?;

        let result = compile_raw_model(&raw_model);

        assert!(result.has_errors());
        assert_eq!(result.model, None);
        assert!(result.report.diagnostics.iter().any(|diagnostic| {
            diagnostic.severity == DiagnosticSeverity::Error
                && diagnostic.code == "DuplicateZoneEquipmentSequence"
                && diagnostic.field.as_deref() == Some("zone_equipment_cooling_sequence")
        }));
        assert!(result.report.diagnostics.iter().any(|diagnostic| {
            diagnostic.severity == DiagnosticSeverity::Error
                && diagnostic.code == "DuplicateZoneEquipmentSequence"
                && diagnostic.field.as_deref() == Some("zone_equipment_heating_or_no_load_sequence")
        }));
        assert!(result.report.diagnostics.iter().any(|diagnostic| {
            diagnostic.severity == DiagnosticSeverity::Error
                && diagnostic.code == "DuplicateZoneEquipmentConnection"
                && diagnostic.object_type == "ZoneHVAC:EquipmentConnections"
        }));

        Ok(())
    }

    #[test]
    fn sorts_ideal_loads_graph_edges_by_equipment_sequence()
    -> Result<(), Box<dyn std::error::Error>> {
        let raw_model = parse_epjson_str(
            r#"{
                "Zone": {"Zone One": {}},
                "ZoneHVAC:IdealLoadsAirSystem": {
                    "Zone Ideal Loads First": {"zone_supply_air_node_name": "Zone Inlet 1"},
                    "Zone Ideal Loads Second": {"zone_supply_air_node_name": "Zone Inlet 2"}
                },
                "ZoneHVAC:EquipmentList": {
                    "Zone Equipment": {
                        "equipment": [
                            {
                                "zone_equipment_object_type": "ZoneHVAC:IdealLoadsAirSystem",
                                "zone_equipment_name": "Zone Ideal Loads Second",
                                "zone_equipment_cooling_sequence": 2,
                                "zone_equipment_heating_or_no_load_sequence": 2
                            },
                            {
                                "zone_equipment_object_type": "ZoneHVAC:IdealLoadsAirSystem",
                                "zone_equipment_name": "Zone Ideal Loads First",
                                "zone_equipment_cooling_sequence": 1,
                                "zone_equipment_heating_or_no_load_sequence": 1
                            }
                        ]
                    }
                },
                "ZoneHVAC:EquipmentConnections": {
                    "Zone Connection": {
                        "zone_name": "Zone One",
                        "zone_conditioning_equipment_list_name": "Zone Equipment",
                        "zone_air_node_name": "Zone Air Node"
                    }
                }
            }"#,
        )?;

        let result = compile_raw_model(&raw_model);

        assert!(!result.has_errors());
        let Some(model) = result.model else {
            return Err(std::io::Error::other("expected typed model").into());
        };
        let graph = ModelGraph::from_typed(&model);
        assert_eq!(graph.zone_ideal_loads.len(), 2);
        assert_eq!(graph.zone_ideal_loads[0].ideal_loads_air_system.0, 0);
        assert_eq!(graph.zone_ideal_loads[0].heating_or_no_load_sequence, 1);
        assert_eq!(graph.zone_ideal_loads[1].ideal_loads_air_system.0, 1);
        assert_eq!(graph.zone_ideal_loads[1].heating_or_no_load_sequence, 2);

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
