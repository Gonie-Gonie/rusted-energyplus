//! Model compiler stage contracts.

use ep_model::{
    AirGapMaterial, AirLoopHvac, AutoOrNumber, AutosizeOrNumber, AvailabilityManagerComponent,
    BoilerHotWater, BranchId, BranchListId, Building, ChillerElectricEir, CoilComponent,
    CoilComponentKind, ComponentId, ConnectorId, ConnectorListId, Construction, ConstructionId,
    ConstructionKind, DayScheduleId, DehumidificationControlType, DemandControlledVentilationType,
    DesignSpecificationOutdoorAir, DesignSpecificationOutdoorAirId,
    DesignSpecificationOutdoorAirMethod, ExternalInterfaceFmuExportSchedule,
    ExternalInterfaceFmuImportSchedule, ExternalInterfaceSchedule, FanComponent, FanComponentKind,
    FirstHourInterpolationStartingValues, GeometryCoordinateSystem, GlobalGeometryRules,
    HeatRecoveryType, HumidificationControlType, IdealLoadsAirSystem, IdealLoadsAirSystemId,
    IdealLoadsFuelType, IdealLoadsLimit, InfraredTransparentMaterial,
    InsideSurfaceConvectionAlgorithm, InternalGainId, LoadDistributionScheme, LoopId, Material,
    MaterialDefinition, MaterialId, MaterialSurfaceRoughness, NameMap, NoMassMaterial, Node,
    NodeId, NodeList, NodeListId, NormalizedName, NumericType, OpaqueSurfaceProperties,
    OtherEquipment, OtherEquipmentDesignLevelCalculationMethod, OutdoorAirEconomizerType,
    OutsideBoundaryCondition, OutsideSurfaceConvectionAlgorithm, People,
    PeopleNumberCalculationMethod, PlantBranch, PlantBranchComponent, PlantBranchList,
    PlantConnector, PlantConnectorKind, PlantConnectorList, PlantConnectorListEntry, PlantLoop,
    Point3, PumpConstantSpeed, RegularMaterial, RunPeriod, RunPeriodDaylightSavingTime,
    RunPeriodId, RunPeriodSpecialDay, RunPeriodSpecialDayId, ScheduleCompact,
    ScheduleCompactDayProfile, ScheduleCompactPeriod, ScheduleCompactSegment, ScheduleConstant,
    ScheduleDayHourly, ScheduleDayInterval, ScheduleDayList, ScheduleDayType, ScheduleFile,
    ScheduleFileColumnSeparator, ScheduleFileShading, ScheduleFileShadingColumn, ScheduleId,
    ScheduleInterpolation, ScheduleTypeLimitId, ScheduleTypeLimits, ScheduleWeekCompact,
    ScheduleWeekDaily, ScheduleYear, SetpointManagerComponent, SiteLocation, SolarDistribution,
    SpecialDayType, StartingVertexPosition, SunExposure, Surface, SurfaceId, SurfaceType, Terrain,
    ThermostatControlObjectType, ThermostatDualSetpoint, ThermostatSetpointId, TimestepConfig,
    TypedModel, Version, VertexEntryDirection, WeekScheduleId, WindExposure, WindowGasMaterial,
    WindowGasPolynomialCoefficients, WindowGasProperties, WindowGasType,
    WindowGlazingEquivalentLayerDiffuseProperties,
    WindowGlazingEquivalentLayerDirectionalProperties, WindowGlazingEquivalentLayerMaterial,
    WindowGlazingEquivalentLayerOpticalBand, WindowGlazingRefractionExtinctionMaterial,
    WindowGlazingSpectralAverageMaterial, Zone, ZoneEquipmentConnection, ZoneEquipmentConnectionId,
    ZoneEquipmentList, ZoneEquipmentListEntry, ZoneEquipmentListId, ZoneEquipmentObjectType,
    ZoneHumidistat, ZoneHumidistatId, ZoneId, ZoneThermostat, ZoneThermostatControl,
    ZoneThermostatId, parse_calendar_date_rule,
};
use ep_raw_model::{FieldName, RawModel, RawObject, RawValue};
use std::collections::BTreeMap;
use std::path::{Component, Path};

const MAX_OPAQUE_CONSTRUCTION_LAYERS: usize = 10;

#[derive(Clone, Copy)]
struct AuxiliaryFileDiagnosticCodes {
    missing_root: &'static str,
    unsupported_path: &'static str,
    root_or_read_failed: &'static str,
    file_not_found: &'static str,
}

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
    Compiler::new(raw_model, None).compile()
}

/// Compiles a RawModel and resolves file-backed schedules below one staged auxiliary root.
#[must_use]
pub fn compile_raw_model_with_auxiliary_root(
    raw_model: &RawModel,
    auxiliary_root: &Path,
) -> CompileResult {
    Compiler::new(raw_model, Some(auxiliary_root)).compile()
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
    "GlobalGeometryRules",
    "Timestep",
    "SurfaceConvectionAlgorithm:Inside",
    "SurfaceConvectionAlgorithm:Outside",
    "RunPeriod",
    "RunPeriodControl:SpecialDays",
    "RunPeriodControl:DaylightSavingTime",
    "Site:Location",
    "Material",
    "Material:NoMass",
    "Material:AirGap",
    "Material:InfraredTransparent",
    "WindowMaterial:Glazing:RefractionExtinctionMethod",
    "WindowMaterial:Glazing:EquivalentLayer",
    "WindowMaterial:Gas",
    "Construction",
    "ScheduleTypeLimits",
    "Schedule:Constant",
    "Schedule:Compact",
    "Schedule:File",
    "Schedule:File:Shading",
    "Schedule:Day:Hourly",
    "Schedule:Day:Interval",
    "Schedule:Day:List",
    "Schedule:Week:Daily",
    "Schedule:Week:Compact",
    "Schedule:Year",
    "ExternalInterface:Schedule",
    "ExternalInterface:FunctionalMockupUnitImport:To:Schedule",
    "ExternalInterface:FunctionalMockupUnitExport:To:Schedule",
    "OtherEquipment",
    "People",
    "ThermostatSetpoint:DualSetpoint",
    "ZoneControl:Thermostat",
    "ZoneControl:Humidistat",
    "NodeList",
    "DesignSpecification:OutdoorAir",
    "ZoneHVAC:IdealLoadsAirSystem",
    "ZoneHVAC:EquipmentList",
    "ZoneHVAC:EquipmentConnections",
    "AirLoopHVAC",
    "Fan:ConstantVolume",
    "Fan:OnOff",
    "Fan:VariableVolume",
    "Fan:SystemModel",
    "Coil:Heating:Electric",
    "Coil:Heating:Fuel",
    "Coil:Heating:Water",
    "Coil:Cooling:Water",
    "Coil:Cooling:DX:SingleSpeed",
    "SetpointManager:Scheduled",
    "SetpointManager:SingleZone:Reheat",
    "AvailabilityManager:Scheduled",
    "PlantLoop",
    "Branch",
    "BranchList",
    "Connector:Splitter",
    "Connector:Mixer",
    "ConnectorList",
    "Pump:ConstantSpeed",
    "Boiler:HotWater",
    "Chiller:Electric:EIR",
    "Zone",
    "BuildingSurface:Detailed",
];

struct Compiler<'a> {
    raw_model: &'a RawModel,
    auxiliary_root: Option<&'a Path>,
    diagnostics: Vec<ModelDiagnostic>,
    defaults_applied: Vec<DefaultApplication>,
}

struct CompactSchedulePeriodBuilder {
    period: ScheduleCompactPeriod,
    assigned_day_types: [bool; 12],
}

struct CompactScheduleProfileBuilder {
    profile: ScheduleCompactDayProfile,
    pending_until_minute_of_day: Option<u32>,
    interpolation_explicit: bool,
}

impl<'a> Compiler<'a> {
    fn new(raw_model: &'a RawModel, auxiliary_root: Option<&'a Path>) -> Self {
        Self {
            raw_model,
            auxiliary_root,
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
        self.parse_global_geometry_rules(&mut model);
        self.parse_timestep(&mut model);
        self.parse_surface_convection_algorithms(&mut model);
        self.parse_run_periods(&mut model);
        self.parse_run_period_special_days(&mut model);
        self.parse_run_period_daylight_saving_time(&mut model);
        self.parse_site_location(&mut model);
        self.parse_materials(&mut model);
        self.parse_constructions(&mut model);
        self.parse_file_shading_schedule(&mut model);
        self.parse_schedule_type_limits(&mut model);
        self.parse_schedules(&mut model);
        self.parse_compact_schedules(&mut model);
        self.parse_file_schedules(&mut model);
        self.parse_day_hourly_schedules(&mut model);
        self.parse_day_interval_schedules(&mut model);
        self.parse_day_list_schedules(&mut model);
        self.parse_week_daily_schedules(&mut model);
        self.parse_week_compact_schedules(&mut model);
        self.parse_year_schedules(&mut model);
        self.parse_external_interface_schedules(&mut model);
        self.parse_external_interface_fmu_import_schedules(&mut model);
        self.parse_external_interface_fmu_export_schedules(&mut model);
        self.validate_scalar_schedule_type_limits(&model);
        self.parse_zones(&mut model);
        self.parse_thermostat_dual_setpoints(&mut model);
        self.parse_zone_thermostats(&mut model);
        self.parse_zone_humidistats(&mut model);
        self.parse_node_lists(&mut model);
        self.parse_design_specification_outdoor_air(&mut model);
        self.parse_ideal_loads_air_systems(&mut model);
        self.parse_zone_equipment_lists(&mut model);
        self.parse_zone_equipment_connections(&mut model);
        self.parse_fans(&mut model);
        self.parse_coils(&mut model);
        self.parse_setpoint_managers(&mut model);
        self.parse_availability_managers(&mut model);
        self.parse_pumps_constant_speed(&mut model);
        self.parse_boilers_hot_water(&mut model);
        self.parse_chillers_electric_eir(&mut model);
        self.parse_plant_branches(&mut model);
        self.parse_plant_branch_lists(&mut model);
        self.parse_plant_connectors(&mut model);
        self.parse_plant_connector_lists(&mut model);
        self.parse_air_loops(&mut model);
        self.parse_plant_loops(&mut model);
        self.parse_other_equipment(&mut model);
        self.parse_people(&mut model);
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

    fn parse_global_geometry_rules(&mut self, model: &mut TypedModel) {
        const OBJECT_TYPE: &str = "GlobalGeometryRules";
        let Some((name, object)) = self.single_object(OBJECT_TYPE) else {
            // Compatibility boundary: unlike EnergyPlus, the current compiler accepts snippets
            // without the source-required singleton.
            return;
        };

        let starting_vertex_position = self
            .required_string(OBJECT_TYPE, &name, &object, "starting_vertex_position")
            .and_then(|value| match parse_starting_vertex_position(&value) {
                Some(parsed) => Some(parsed),
                None => {
                    self.invalid_enum_value(OBJECT_TYPE, &name, "starting_vertex_position", &value);
                    None
                }
            });
        let vertex_entry_direction = self
            .required_string(OBJECT_TYPE, &name, &object, "vertex_entry_direction")
            .and_then(|value| match parse_vertex_entry_direction(&value) {
                Some(parsed) => Some(parsed),
                None => {
                    self.invalid_enum_value(OBJECT_TYPE, &name, "vertex_entry_direction", &value);
                    None
                }
            });
        let coordinate_system = self.enum_warning_default(
            OBJECT_TYPE,
            &name,
            (&object, "coordinate_system"),
            GeometryCoordinateSystem::World,
            "World",
            true,
            parse_geometry_coordinate_system,
        );
        let daylighting_reference_point_coordinate_system = self.enum_warning_default(
            OBJECT_TYPE,
            &name,
            (&object, "daylighting_reference_point_coordinate_system"),
            GeometryCoordinateSystem::Relative,
            "Relative",
            false,
            parse_geometry_coordinate_system,
        );
        let rectangular_surface_coordinate_system = self.enum_warning_default(
            OBJECT_TYPE,
            &name,
            (&object, "rectangular_surface_coordinate_system"),
            GeometryCoordinateSystem::Relative,
            "Relative",
            false,
            parse_geometry_coordinate_system,
        );

        let (Some(starting_vertex_position), Some(vertex_entry_direction)) =
            (starting_vertex_position, vertex_entry_direction)
        else {
            return;
        };
        model.global_geometry_rules = Some(GlobalGeometryRules {
            starting_vertex_position,
            vertex_entry_direction,
            coordinate_system,
            daylighting_reference_point_coordinate_system,
            rectangular_surface_coordinate_system,
        });
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

    fn parse_surface_convection_algorithms(&mut self, model: &mut TypedModel) {
        if let Some((name, object)) = self.single_object("SurfaceConvectionAlgorithm:Inside") {
            model.surface_convection_algorithms.inside = Some(self.enum_default(
                "SurfaceConvectionAlgorithm:Inside",
                &name,
                (&object, "algorithm"),
                InsideSurfaceConvectionAlgorithm::Tarp,
                "TARP",
                parse_inside_surface_convection_algorithm,
            ));
        }

        if let Some((name, object)) = self.single_object("SurfaceConvectionAlgorithm:Outside") {
            model.surface_convection_algorithms.outside = Some(self.enum_default(
                "SurfaceConvectionAlgorithm:Outside",
                &name,
                (&object, "algorithm"),
                OutsideSurfaceConvectionAlgorithm::Doe2,
                "DOE-2",
                parse_outside_surface_convection_algorithm,
            ));
        }
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
                first_hour_interpolation_starting_values: self.enum_default(
                    "RunPeriod",
                    &name,
                    (&object, "first_hour_interpolation_starting_values"),
                    FirstHourInterpolationStartingValues::Hour24,
                    "Hour24",
                    parse_first_hour_interpolation_starting_values,
                ),
                use_weather_file_holidays_and_special_days: self.enum_default(
                    "RunPeriod",
                    &name,
                    (&object, "use_weather_file_holidays_and_special_days"),
                    true,
                    "Yes",
                    parse_yes_no,
                ),
                use_weather_file_daylight_saving_period: self.enum_default(
                    "RunPeriod",
                    &name,
                    (&object, "use_weather_file_daylight_saving_period"),
                    true,
                    "Yes",
                    parse_yes_no,
                ),
                apply_weekend_holiday_rule: self.enum_default(
                    "RunPeriod",
                    &name,
                    (&object, "apply_weekend_holiday_rule"),
                    true,
                    "Yes",
                    parse_yes_no,
                ),
                use_weather_file_rain_indicators: self.enum_default(
                    "RunPeriod",
                    &name,
                    (&object, "use_weather_file_rain_indicators"),
                    true,
                    "Yes",
                    parse_yes_no,
                ),
                use_weather_file_snow_indicators: self.enum_default(
                    "RunPeriod",
                    &name,
                    (&object, "use_weather_file_snow_indicators"),
                    true,
                    "Yes",
                    parse_yes_no,
                ),
                treat_weather_as_actual: self.enum_default(
                    "RunPeriod",
                    &name,
                    (&object, "treat_weather_as_actual"),
                    false,
                    "No",
                    parse_yes_no,
                ),
            });
        }
    }

    fn parse_run_period_special_days(&mut self, model: &mut TypedModel) {
        const OBJECT_TYPE: &str = "RunPeriodControl:SpecialDays";
        for (name, object) in self.objects(OBJECT_TYPE) {
            let Some(id_value) =
                self.checked_id(OBJECT_TYPE, &name, model.run_period_special_days.len())
            else {
                continue;
            };
            let id = RunPeriodSpecialDayId(id_value);
            if model
                .run_period_special_day_names
                .insert(&name, id)
                .is_some()
            {
                self.duplicate_name(OBJECT_TYPE, &name);
                continue;
            }

            let Some(start_date_text) =
                self.required_string(OBJECT_TYPE, &name, &object, "start_date")
            else {
                continue;
            };
            let Some(start_date) = parse_calendar_date_rule(&start_date_text) else {
                self.error(
                    "InvalidCalendarDateRule",
                    OBJECT_TYPE,
                    Some(&name),
                    Some("start_date"),
                    format!(
                        "{OBJECT_TYPE}/{name} field start_date has unsupported date rule '{start_date_text}'"
                    ),
                );
                continue;
            };
            let duration_days = self.u32_default(OBJECT_TYPE, &name, &object, "duration", 1);
            if !(1..=366).contains(&duration_days) {
                self.error(
                    "InvalidNumericRange",
                    OBJECT_TYPE,
                    Some(&name),
                    Some("duration"),
                    format!(
                        "{OBJECT_TYPE}/{name} field duration must be between 1 and 366, got {duration_days}"
                    ),
                );
                continue;
            }
            let special_day_type = self.enum_default(
                OBJECT_TYPE,
                &name,
                (&object, "special_day_type"),
                SpecialDayType::Holiday,
                "Holiday",
                parse_special_day_type,
            );

            model.run_period_special_days.push(RunPeriodSpecialDay {
                id,
                name: NormalizedName::new(&name),
                start_date,
                duration_days,
                special_day_type,
            });
        }
    }

    fn parse_run_period_daylight_saving_time(&mut self, model: &mut TypedModel) {
        const OBJECT_TYPE: &str = "RunPeriodControl:DaylightSavingTime";
        let Some((name, object)) = self.single_object(OBJECT_TYPE) else {
            return;
        };

        let start_date =
            self.required_calendar_date_rule(OBJECT_TYPE, &name, &object, "start_date");
        let end_date = self.required_calendar_date_rule(OBJECT_TYPE, &name, &object, "end_date");
        let (Some(start_date), Some(end_date)) = (start_date, end_date) else {
            return;
        };

        model.run_period_daylight_saving_time = Some(RunPeriodDaylightSavingTime {
            start_date,
            end_date,
        });
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
        self.parse_regular_materials(model);
        self.parse_nomass_materials(model);
        self.parse_air_gap_materials(model);
        self.parse_infrared_transparent_materials(model);
        self.parse_window_glazing_materials(model);
        self.parse_window_glazing_refraction_extinction_materials(model);
        self.parse_window_glazing_equivalent_layer_materials(model);
        self.parse_window_gas_materials(model);
    }

    fn parse_regular_materials(&mut self, model: &mut TypedModel) {
        const OBJECT_TYPE: &str = "Material";
        for (name, object) in self.objects(OBJECT_TYPE) {
            let roughness = self.required_material_roughness(OBJECT_TYPE, &name, &object);
            let thickness_m =
                self.required_number_minimum(OBJECT_TYPE, &name, &object, "thickness", 0.0, false);
            let conductivity_w_per_m_k = self.required_number_minimum(
                OBJECT_TYPE,
                &name,
                &object,
                "conductivity",
                0.0,
                false,
            );
            let density_kg_per_m3 =
                self.required_number_minimum(OBJECT_TYPE, &name, &object, "density", 0.0, false);
            let specific_heat_j_per_kg_k = self.required_number_minimum(
                OBJECT_TYPE,
                &name,
                &object,
                "specific_heat",
                100.0,
                true,
            );
            let surface = self.opaque_surface_properties(OBJECT_TYPE, &name, &object);
            let (
                Some(roughness),
                Some(thickness_m),
                Some(conductivity_w_per_m_k),
                Some(density_kg_per_m3),
                Some(specific_heat_j_per_kg_k),
            ) = (
                roughness,
                thickness_m,
                conductivity_w_per_m_k,
                density_kg_per_m3,
                specific_heat_j_per_kg_k,
            )
            else {
                continue;
            };
            let Some((id, normalized_name)) =
                self.reserve_material_identity(model, OBJECT_TYPE, &name)
            else {
                continue;
            };

            model.materials.push(Material {
                id,
                name: normalized_name,
                definition: MaterialDefinition::Regular(RegularMaterial {
                    roughness,
                    thickness_m,
                    conductivity_w_per_m_k,
                    density_kg_per_m3,
                    specific_heat_j_per_kg_k,
                    surface,
                }),
            });
        }
    }

    fn parse_nomass_materials(&mut self, model: &mut TypedModel) {
        const OBJECT_TYPE: &str = "Material:NoMass";
        for (name, object) in self.objects(OBJECT_TYPE) {
            let roughness = self.required_material_roughness(OBJECT_TYPE, &name, &object);
            let thermal_resistance_m2_k_per_w = self.required_number_minimum(
                OBJECT_TYPE,
                &name,
                &object,
                "thermal_resistance",
                0.001,
                true,
            );
            let surface = self.opaque_surface_properties(OBJECT_TYPE, &name, &object);
            let (Some(roughness), Some(thermal_resistance_m2_k_per_w)) =
                (roughness, thermal_resistance_m2_k_per_w)
            else {
                continue;
            };
            let Some((id, normalized_name)) =
                self.reserve_material_identity(model, OBJECT_TYPE, &name)
            else {
                continue;
            };

            model.materials.push(Material {
                id,
                name: normalized_name,
                definition: MaterialDefinition::NoMass(NoMassMaterial {
                    roughness,
                    thermal_resistance_m2_k_per_w,
                    surface,
                }),
            });
        }
    }

    fn parse_air_gap_materials(&mut self, model: &mut TypedModel) {
        const OBJECT_TYPE: &str = "Material:AirGap";
        for (name, object) in self.objects(OBJECT_TYPE) {
            let Some(thermal_resistance_m2_k_per_w) = self.required_number_minimum(
                OBJECT_TYPE,
                &name,
                &object,
                "thermal_resistance",
                0.0,
                false,
            ) else {
                continue;
            };
            let Some((id, normalized_name)) =
                self.reserve_material_identity(model, OBJECT_TYPE, &name)
            else {
                continue;
            };

            model.materials.push(Material {
                id,
                name: normalized_name,
                definition: MaterialDefinition::AirGap(AirGapMaterial {
                    thermal_resistance_m2_k_per_w,
                }),
            });
        }
    }

    fn parse_infrared_transparent_materials(&mut self, model: &mut TypedModel) {
        const OBJECT_TYPE: &str = "Material:InfraredTransparent";
        for (name, _object) in self.objects(OBJECT_TYPE) {
            let Some((id, normalized_name)) =
                self.reserve_material_identity(model, OBJECT_TYPE, &name)
            else {
                continue;
            };

            model.materials.push(Material {
                id,
                name: normalized_name,
                definition: MaterialDefinition::InfraredTransparent(InfraredTransparentMaterial),
            });
        }
    }

    fn parse_window_glazing_materials(&mut self, model: &mut TypedModel) {
        const OBJECT_TYPE: &str = "WindowMaterial:Glazing";
        for (name, object) in self.objects(OBJECT_TYPE) {
            let optical_data_type =
                self.required_string(OBJECT_TYPE, &name, &object, "optical_data_type");
            let thickness_m =
                self.required_number_minimum(OBJECT_TYPE, &name, &object, "thickness", 0.0, false);
            let Some(optical_data_type) = optical_data_type else {
                continue;
            };
            if !optical_data_type.eq_ignore_ascii_case("SpectralAverage") {
                if ["BSDF", "Spectral", "SpectralAndAngle"]
                    .iter()
                    .any(|candidate| optical_data_type.eq_ignore_ascii_case(candidate))
                {
                    self.error(
                        "UnsupportedWindowGlazingOpticalDataType",
                        OBJECT_TYPE,
                        Some(&name),
                        Some("optical_data_type"),
                        format!(
                            "{OBJECT_TYPE}/{name} optical data type {optical_data_type} depends on a later source-order checkpoint; only SpectralAverage is currently typed"
                        ),
                    );
                } else {
                    self.invalid_enum_value(
                        OBJECT_TYPE,
                        &name,
                        "optical_data_type",
                        &optical_data_type,
                    );
                }
                continue;
            }

            let solar_transmittance_at_normal_incidence = self.number_range_default(
                OBJECT_TYPE,
                &name,
                &object,
                "solar_transmittance_at_normal_incidence",
                0.0,
                0.0..=1.0,
            );
            let front_side_solar_reflectance_at_normal_incidence = self.number_range_default(
                OBJECT_TYPE,
                &name,
                &object,
                "front_side_solar_reflectance_at_normal_incidence",
                0.0,
                0.0..=1.0,
            );
            let back_side_solar_reflectance_at_normal_incidence = self.number_range_default(
                OBJECT_TYPE,
                &name,
                &object,
                "back_side_solar_reflectance_at_normal_incidence",
                0.0,
                0.0..=1.0,
            );
            let visible_transmittance_at_normal_incidence = self.number_range_default(
                OBJECT_TYPE,
                &name,
                &object,
                "visible_transmittance_at_normal_incidence",
                0.0,
                0.0..=1.0,
            );
            let front_side_visible_reflectance_at_normal_incidence = self.number_range_default(
                OBJECT_TYPE,
                &name,
                &object,
                "front_side_visible_reflectance_at_normal_incidence",
                0.0,
                0.0..=1.0,
            );
            let back_side_visible_reflectance_at_normal_incidence = self.number_range_default(
                OBJECT_TYPE,
                &name,
                &object,
                "back_side_visible_reflectance_at_normal_incidence",
                0.0,
                0.0..=1.0,
            );
            let infrared_transmittance_at_normal_incidence = self.number_range_default(
                OBJECT_TYPE,
                &name,
                &object,
                "infrared_transmittance_at_normal_incidence",
                0.0,
                0.0..=1.0,
            );
            let front_side_infrared_hemispherical_emissivity = self.number_bounded_default(
                OBJECT_TYPE,
                &name,
                &object,
                "front_side_infrared_hemispherical_emissivity",
                0.84,
                (0.0, false),
                (1.0, false),
            );
            let back_side_infrared_hemispherical_emissivity = self.number_bounded_default(
                OBJECT_TYPE,
                &name,
                &object,
                "back_side_infrared_hemispherical_emissivity",
                0.84,
                (0.0, false),
                (1.0, false),
            );
            let conductivity_w_per_m_k = self.number_bounded_default(
                OBJECT_TYPE,
                &name,
                &object,
                "conductivity",
                0.9,
                (0.0, false),
                (f64::INFINITY, true),
            );
            let dirt_correction_factor_for_solar_and_visible_transmittance = self
                .number_bounded_default(
                    OBJECT_TYPE,
                    &name,
                    &object,
                    "dirt_correction_factor_for_solar_and_visible_transmittance",
                    1.0,
                    (0.0, false),
                    (1.0, true),
                );
            let solar_diffusing = self.enum_default(
                OBJECT_TYPE,
                &name,
                (&object, "solar_diffusing"),
                false,
                "No",
                parse_yes_no,
            );
            let youngs_modulus_pa = self.number_bounded_default(
                OBJECT_TYPE,
                &name,
                &object,
                "young_s_modulus",
                72_000_000_000.0,
                (0.0, false),
                (f64::INFINITY, true),
            );
            let poissons_ratio = self.number_bounded_default(
                OBJECT_TYPE,
                &name,
                &object,
                "poisson_s_ratio",
                0.22,
                (0.0, false),
                (1.0, false),
            );
            let Some(thickness_m) = thickness_m else {
                continue;
            };

            let optical_sums = [
                (
                    "front_side_solar_reflectance_at_normal_incidence",
                    solar_transmittance_at_normal_incidence
                        + front_side_solar_reflectance_at_normal_incidence,
                ),
                (
                    "back_side_solar_reflectance_at_normal_incidence",
                    solar_transmittance_at_normal_incidence
                        + back_side_solar_reflectance_at_normal_incidence,
                ),
                (
                    "front_side_visible_reflectance_at_normal_incidence",
                    visible_transmittance_at_normal_incidence
                        + front_side_visible_reflectance_at_normal_incidence,
                ),
                (
                    "back_side_visible_reflectance_at_normal_incidence",
                    visible_transmittance_at_normal_incidence
                        + back_side_visible_reflectance_at_normal_incidence,
                ),
                (
                    "front_side_infrared_hemispherical_emissivity",
                    infrared_transmittance_at_normal_incidence
                        + front_side_infrared_hemispherical_emissivity,
                ),
                (
                    "back_side_infrared_hemispherical_emissivity",
                    infrared_transmittance_at_normal_incidence
                        + back_side_infrared_hemispherical_emissivity,
                ),
            ];
            let mut sums_valid = true;
            for (field, sum) in optical_sums {
                if sum > 1.0 {
                    self.error(
                        "InvalidWindowGlazingOpticalSum",
                        OBJECT_TYPE,
                        Some(&name),
                        Some(field),
                        format!(
                            "{OBJECT_TYPE}/{name} transmittance plus the {field} value must be less than or equal to 1, got {sum}"
                        ),
                    );
                    sums_valid = false;
                }
            }
            if !sums_valid {
                continue;
            }

            let Some((id, normalized_name)) =
                self.reserve_material_identity(model, OBJECT_TYPE, &name)
            else {
                continue;
            };
            model.materials.push(Material {
                id,
                name: normalized_name,
                definition: MaterialDefinition::WindowGlazingSpectralAverage(
                    WindowGlazingSpectralAverageMaterial {
                        thickness_m,
                        solar_transmittance_at_normal_incidence,
                        front_side_solar_reflectance_at_normal_incidence,
                        back_side_solar_reflectance_at_normal_incidence,
                        visible_transmittance_at_normal_incidence,
                        front_side_visible_reflectance_at_normal_incidence,
                        back_side_visible_reflectance_at_normal_incidence,
                        infrared_transmittance_at_normal_incidence,
                        front_side_infrared_hemispherical_emissivity,
                        back_side_infrared_hemispherical_emissivity,
                        conductivity_w_per_m_k,
                        dirt_correction_factor_for_solar_and_visible_transmittance,
                        solar_diffusing,
                        youngs_modulus_pa,
                        poissons_ratio,
                    },
                ),
            });
        }
    }

    fn parse_window_glazing_refraction_extinction_materials(&mut self, model: &mut TypedModel) {
        const OBJECT_TYPE: &str = "WindowMaterial:Glazing:RefractionExtinctionMethod";
        for (name, object) in self.objects(OBJECT_TYPE) {
            let thickness_m =
                self.required_number_minimum(OBJECT_TYPE, &name, &object, "thickness", 0.0, false);
            let solar_index_of_refraction = self.required_number_minimum(
                OBJECT_TYPE,
                &name,
                &object,
                "solar_index_of_refraction",
                1.0,
                false,
            );
            let solar_extinction_coefficient_per_m = self.required_number_minimum(
                OBJECT_TYPE,
                &name,
                &object,
                "solar_extinction_coefficient",
                0.0,
                false,
            );
            let visible_index_of_refraction = self.required_number_minimum(
                OBJECT_TYPE,
                &name,
                &object,
                "visible_index_of_refraction",
                1.0,
                false,
            );
            let visible_extinction_coefficient_per_m = self.required_number_minimum(
                OBJECT_TYPE,
                &name,
                &object,
                "visible_extinction_coefficient",
                0.0,
                false,
            );
            let infrared_transmittance_at_normal_incidence = self.number_bounded_default(
                OBJECT_TYPE,
                &name,
                &object,
                "infrared_transmittance_at_normal_incidence",
                0.0,
                (0.0, true),
                (1.0, false),
            );
            let infrared_hemispherical_emissivity = self.number_bounded_default(
                OBJECT_TYPE,
                &name,
                &object,
                "infrared_hemispherical_emissivity",
                0.84,
                (0.0, false),
                (1.0, false),
            );
            let conductivity_w_per_m_k = self.number_bounded_default(
                OBJECT_TYPE,
                &name,
                &object,
                "conductivity",
                0.9,
                (0.0, false),
                (f64::INFINITY, true),
            );
            let dirt_correction_factor_for_solar_and_visible_transmittance = self
                .number_bounded_default(
                    OBJECT_TYPE,
                    &name,
                    &object,
                    "dirt_correction_factor_for_solar_and_visible_transmittance",
                    1.0,
                    (0.0, false),
                    (1.0, true),
                );
            let solar_diffusing = self.enum_default(
                OBJECT_TYPE,
                &name,
                (&object, "solar_diffusing"),
                false,
                "No",
                parse_yes_no,
            );

            let (
                Some(thickness_m),
                Some(solar_index_of_refraction),
                Some(solar_extinction_coefficient_per_m),
                Some(visible_index_of_refraction),
                Some(visible_extinction_coefficient_per_m),
            ) = (
                thickness_m,
                solar_index_of_refraction,
                solar_extinction_coefficient_per_m,
                visible_index_of_refraction,
                visible_extinction_coefficient_per_m,
            )
            else {
                continue;
            };

            let infrared_sum =
                infrared_transmittance_at_normal_incidence + infrared_hemispherical_emissivity;
            if infrared_sum >= 1.0 {
                self.error(
                    "InvalidWindowGlazingOpticalSum",
                    OBJECT_TYPE,
                    Some(&name),
                    Some("infrared_hemispherical_emissivity"),
                    format!(
                        "{OBJECT_TYPE}/{name} infrared_transmittance_at_normal_incidence plus infrared_hemispherical_emissivity must be less than 1, got {infrared_sum}"
                    ),
                );
                continue;
            }

            let Some((id, normalized_name)) =
                self.reserve_material_identity(model, OBJECT_TYPE, &name)
            else {
                continue;
            };
            model.materials.push(Material {
                id,
                name: normalized_name,
                definition: MaterialDefinition::WindowGlazingRefractionExtinction(
                    WindowGlazingRefractionExtinctionMaterial {
                        thickness_m,
                        solar_index_of_refraction,
                        solar_extinction_coefficient_per_m,
                        visible_index_of_refraction,
                        visible_extinction_coefficient_per_m,
                        infrared_transmittance_at_normal_incidence,
                        infrared_hemispherical_emissivity,
                        conductivity_w_per_m_k,
                        dirt_correction_factor_for_solar_and_visible_transmittance,
                        solar_diffusing,
                    },
                ),
            });
        }
    }

    fn parse_window_glazing_equivalent_layer_materials(&mut self, model: &mut TypedModel) {
        const OBJECT_TYPE: &str = "WindowMaterial:Glazing:EquivalentLayer";
        for (name, object) in self.objects(OBJECT_TYPE) {
            let optical_data_type = match field_value(&object, "optical_data_type") {
                Some(RawValue::String(value)) if value.trim().is_empty() => {
                    self.record_default(OBJECT_TYPE, &name, "optical_data_type", "SpectralAverage");
                    "SpectralAverage".to_string()
                }
                Some(RawValue::String(value)) => value.clone(),
                Some(_) => {
                    self.invalid_field_type(OBJECT_TYPE, &name, "optical_data_type", "string");
                    continue;
                }
                None => {
                    self.record_default(OBJECT_TYPE, &name, "optical_data_type", "SpectralAverage");
                    "SpectralAverage".to_string()
                }
            };
            if optical_data_type.eq_ignore_ascii_case("Spectral") {
                self.error(
                    "UnsupportedWindowGlazingEquivalentLayerOpticalDataType",
                    OBJECT_TYPE,
                    Some(&name),
                    Some("optical_data_type"),
                    format!(
                        "{OBJECT_TYPE}/{name} optical data type Spectral is not supported by EnergyPlus 26.1; only SpectralAverage is accepted"
                    ),
                );
                continue;
            }
            if !optical_data_type.eq_ignore_ascii_case("SpectralAverage") {
                self.invalid_enum_value(
                    OBJECT_TYPE,
                    &name,
                    "optical_data_type",
                    &optical_data_type,
                );
                continue;
            }

            // EnergyPlus 26.1 ignores this reference for its only supported
            // SpectralAverage branch, but its input type is still validated.
            let _spectral_data_set_name = self.optional_string(
                OBJECT_TYPE,
                &name,
                &object,
                "window_glass_spectral_data_set_name",
            );

            let required_solar = (
                self.required_number_range(
                    OBJECT_TYPE,
                    &name,
                    &object,
                    "front_side_beam_beam_solar_transmittance",
                    0.0..=1.0,
                ),
                self.required_number_range(
                    OBJECT_TYPE,
                    &name,
                    &object,
                    "back_side_beam_beam_solar_transmittance",
                    0.0..=1.0,
                ),
                self.required_number_range(
                    OBJECT_TYPE,
                    &name,
                    &object,
                    "front_side_beam_beam_solar_reflectance",
                    0.0..=1.0,
                ),
                self.required_number_range(
                    OBJECT_TYPE,
                    &name,
                    &object,
                    "back_side_beam_beam_solar_reflectance",
                    0.0..=1.0,
                ),
            );
            let (
                Some(front_beam_beam_solar_transmittance),
                Some(back_beam_beam_solar_transmittance),
                Some(front_beam_beam_solar_reflectance),
                Some(back_beam_beam_solar_reflectance),
            ) = required_solar
            else {
                continue;
            };

            let solar = WindowGlazingEquivalentLayerOpticalBand {
                beam_beam: WindowGlazingEquivalentLayerDirectionalProperties {
                    front_transmittance: front_beam_beam_solar_transmittance,
                    back_transmittance: back_beam_beam_solar_transmittance,
                    front_reflectance: front_beam_beam_solar_reflectance,
                    back_reflectance: back_beam_beam_solar_reflectance,
                },
                beam_diffuse: WindowGlazingEquivalentLayerDirectionalProperties {
                    front_transmittance: self.number_range_default(
                        OBJECT_TYPE,
                        &name,
                        &object,
                        "front_side_beam_diffuse_solar_transmittance",
                        0.0,
                        0.0..=1.0,
                    ),
                    back_transmittance: self.number_range_default(
                        OBJECT_TYPE,
                        &name,
                        &object,
                        "back_side_beam_diffuse_solar_transmittance",
                        0.0,
                        0.0..=1.0,
                    ),
                    front_reflectance: self.number_range_default(
                        OBJECT_TYPE,
                        &name,
                        &object,
                        "front_side_beam_diffuse_solar_reflectance",
                        0.0,
                        0.0..=1.0,
                    ),
                    back_reflectance: self.number_range_default(
                        OBJECT_TYPE,
                        &name,
                        &object,
                        "back_side_beam_diffuse_solar_reflectance",
                        0.0,
                        0.0..=1.0,
                    ),
                },
                diffuse_diffuse: WindowGlazingEquivalentLayerDiffuseProperties {
                    transmittance: self.auto_range_default(
                        OBJECT_TYPE,
                        &name,
                        &object,
                        "diffuse_diffuse_solar_transmittance",
                        0.0..=1.0,
                    ),
                    front_reflectance: self.auto_range_default(
                        OBJECT_TYPE,
                        &name,
                        &object,
                        "front_side_diffuse_diffuse_solar_reflectance",
                        0.0..=1.0,
                    ),
                    back_reflectance: self.auto_range_default(
                        OBJECT_TYPE,
                        &name,
                        &object,
                        "back_side_diffuse_diffuse_solar_reflectance",
                        0.0..=1.0,
                    ),
                },
            };
            let visible = WindowGlazingEquivalentLayerOpticalBand {
                beam_beam: WindowGlazingEquivalentLayerDirectionalProperties {
                    front_transmittance: self.number_range_default(
                        OBJECT_TYPE,
                        &name,
                        &object,
                        "front_side_beam_beam_visible_solar_transmittance",
                        0.0,
                        0.0..=1.0,
                    ),
                    back_transmittance: self.number_range_default(
                        OBJECT_TYPE,
                        &name,
                        &object,
                        "back_side_beam_beam_visible_solar_transmittance",
                        0.0,
                        0.0..=1.0,
                    ),
                    front_reflectance: self.number_range_default(
                        OBJECT_TYPE,
                        &name,
                        &object,
                        "front_side_beam_beam_visible_solar_reflectance",
                        0.0,
                        0.0..=1.0,
                    ),
                    back_reflectance: self.number_range_default(
                        OBJECT_TYPE,
                        &name,
                        &object,
                        "back_side_beam_beam_visible_solar_reflectance",
                        0.0,
                        0.0..=1.0,
                    ),
                },
                beam_diffuse: WindowGlazingEquivalentLayerDirectionalProperties {
                    front_transmittance: self.number_range_default(
                        OBJECT_TYPE,
                        &name,
                        &object,
                        "front_side_beam_diffuse_visible_solar_transmittance",
                        0.0,
                        0.0..=1.0,
                    ),
                    back_transmittance: self.number_range_default(
                        OBJECT_TYPE,
                        &name,
                        &object,
                        "back_side_beam_diffuse_visible_solar_transmittance",
                        0.0,
                        0.0..=1.0,
                    ),
                    front_reflectance: self.number_range_default(
                        OBJECT_TYPE,
                        &name,
                        &object,
                        "front_side_beam_diffuse_visible_solar_reflectance",
                        0.0,
                        0.0..=1.0,
                    ),
                    back_reflectance: self.number_range_default(
                        OBJECT_TYPE,
                        &name,
                        &object,
                        "back_side_beam_diffuse_visible_solar_reflectance",
                        0.0,
                        0.0..=1.0,
                    ),
                },
                diffuse_diffuse: WindowGlazingEquivalentLayerDiffuseProperties {
                    transmittance: self.auto_range_default(
                        OBJECT_TYPE,
                        &name,
                        &object,
                        "diffuse_diffuse_visible_solar_transmittance",
                        0.0..=1.0,
                    ),
                    front_reflectance: self.auto_range_default(
                        OBJECT_TYPE,
                        &name,
                        &object,
                        "front_side_diffuse_diffuse_visible_solar_reflectance",
                        0.0..=1.0,
                    ),
                    back_reflectance: self.auto_range_default(
                        OBJECT_TYPE,
                        &name,
                        &object,
                        "back_side_diffuse_diffuse_visible_solar_reflectance",
                        0.0..=1.0,
                    ),
                },
            };
            let infrared_transmittance = self.number_range_default(
                OBJECT_TYPE,
                &name,
                &object,
                "infrared_transmittance_applies_to_front_and_back_",
                0.0,
                0.0..=1.0,
            );
            let front_infrared_emissivity = self.number_bounded_default(
                OBJECT_TYPE,
                &name,
                &object,
                "front_side_infrared_emissivity",
                0.84,
                (0.0, false),
                (1.0, false),
            );
            let back_infrared_emissivity = self.number_bounded_default(
                OBJECT_TYPE,
                &name,
                &object,
                "back_side_infrared_emissivity",
                0.84,
                (0.0, false),
                (1.0, false),
            );
            let thermal_resistance_m2_k_per_w = self.number_bounded_default(
                OBJECT_TYPE,
                &name,
                &object,
                "thermal_resistance",
                0.158,
                (0.0, false),
                (f64::INFINITY, true),
            );

            let Some((id, normalized_name)) =
                self.reserve_material_identity(model, OBJECT_TYPE, &name)
            else {
                continue;
            };
            model.materials.push(Material {
                id,
                name: normalized_name,
                definition: MaterialDefinition::WindowGlazingEquivalentLayer(
                    WindowGlazingEquivalentLayerMaterial {
                        solar,
                        visible,
                        infrared_transmittance,
                        front_infrared_emissivity,
                        back_infrared_emissivity,
                        thermal_resistance_m2_k_per_w,
                    },
                ),
            });
        }
    }

    fn parse_window_gas_materials(&mut self, model: &mut TypedModel) {
        const OBJECT_TYPE: &str = "WindowMaterial:Gas";
        const CUSTOM_COEFFICIENT_FIELDS: [&str; 9] = [
            "conductivity_coefficient_a",
            "conductivity_coefficient_b",
            "conductivity_coefficient_c",
            "viscosity_coefficient_a",
            "viscosity_coefficient_b",
            "viscosity_coefficient_c",
            "specific_heat_coefficient_a",
            "specific_heat_coefficient_b",
            "specific_heat_coefficient_c",
        ];

        for (name, object) in self.objects(OBJECT_TYPE) {
            let gas_type = self.required_enum(
                OBJECT_TYPE,
                &name,
                &object,
                "gas_type",
                WindowGasType::from_energyplus_name,
            );
            let thickness_m =
                self.required_number_minimum(OBJECT_TYPE, &name, &object, "thickness", 0.0, false);
            let (Some(gas_type), Some(thickness_m)) = (gas_type, thickness_m) else {
                continue;
            };

            let conductivity_a =
                self.optional_number(OBJECT_TYPE, &name, &object, CUSTOM_COEFFICIENT_FIELDS[0]);
            let conductivity_b =
                self.optional_number(OBJECT_TYPE, &name, &object, CUSTOM_COEFFICIENT_FIELDS[1]);
            let conductivity_c =
                self.optional_number(OBJECT_TYPE, &name, &object, CUSTOM_COEFFICIENT_FIELDS[2]);
            let viscosity_a = self.optional_number_bounded(
                OBJECT_TYPE,
                &name,
                &object,
                CUSTOM_COEFFICIENT_FIELDS[3],
                (0.0, false),
                (f64::INFINITY, true),
            );
            let viscosity_b =
                self.optional_number(OBJECT_TYPE, &name, &object, CUSTOM_COEFFICIENT_FIELDS[4]);
            let viscosity_c =
                self.optional_number(OBJECT_TYPE, &name, &object, CUSTOM_COEFFICIENT_FIELDS[5]);
            let specific_heat_a = self.optional_number_bounded(
                OBJECT_TYPE,
                &name,
                &object,
                CUSTOM_COEFFICIENT_FIELDS[6],
                (0.0, false),
                (f64::INFINITY, true),
            );
            let specific_heat_b =
                self.optional_number(OBJECT_TYPE, &name, &object, CUSTOM_COEFFICIENT_FIELDS[7]);
            let specific_heat_c =
                self.optional_number(OBJECT_TYPE, &name, &object, CUSTOM_COEFFICIENT_FIELDS[8]);
            let molecular_weight_g_per_mol = self.optional_number_bounded(
                OBJECT_TYPE,
                &name,
                &object,
                "molecular_weight",
                (20.0, true),
                (200.0, true),
            );
            let specific_heat_ratio = self.optional_number_bounded(
                OBJECT_TYPE,
                &name,
                &object,
                "specific_heat_ratio",
                (1.0, false),
                (f64::INFINITY, true),
            );

            let supplied_properties = [
                (CUSTOM_COEFFICIENT_FIELDS[0], conductivity_a),
                (CUSTOM_COEFFICIENT_FIELDS[1], conductivity_b),
                (CUSTOM_COEFFICIENT_FIELDS[2], conductivity_c),
                (CUSTOM_COEFFICIENT_FIELDS[3], viscosity_a),
                (CUSTOM_COEFFICIENT_FIELDS[4], viscosity_b),
                (CUSTOM_COEFFICIENT_FIELDS[5], viscosity_c),
                (CUSTOM_COEFFICIENT_FIELDS[6], specific_heat_a),
                (CUSTOM_COEFFICIENT_FIELDS[7], specific_heat_b),
                (CUSTOM_COEFFICIENT_FIELDS[8], specific_heat_c),
                ("molecular_weight", molecular_weight_g_per_mol),
                ("specific_heat_ratio", specific_heat_ratio),
            ];
            let mut properties_valid = supplied_properties
                .iter()
                .all(|(field, value)| field_value(&object, field).is_none() || value.is_some());

            let properties = if let Some(properties) = gas_type.standard_properties() {
                properties
            } else {
                for (field, value) in [
                    (CUSTOM_COEFFICIENT_FIELDS[3], viscosity_a),
                    (CUSTOM_COEFFICIENT_FIELDS[6], specific_heat_a),
                    ("molecular_weight", molecular_weight_g_per_mol),
                ] {
                    if value.is_none() && field_value(&object, field).is_none() {
                        self.error(
                            "MissingCustomWindowGasProperty",
                            OBJECT_TYPE,
                            Some(&name),
                            Some(field),
                            format!(
                                "{OBJECT_TYPE}/{name} custom gas field {field} is effectively required because EnergyPlus reads a blank value as zero and requires it to be positive"
                            ),
                        );
                        properties_valid = false;
                    }
                }

                let properties = WindowGasProperties {
                    conductivity: WindowGasPolynomialCoefficients {
                        coefficient_a: conductivity_a.unwrap_or(0.0),
                        coefficient_b: conductivity_b.unwrap_or(0.0),
                        coefficient_c: conductivity_c.unwrap_or(0.0),
                    },
                    viscosity: WindowGasPolynomialCoefficients {
                        coefficient_a: viscosity_a.unwrap_or(0.0),
                        coefficient_b: viscosity_b.unwrap_or(0.0),
                        coefficient_c: viscosity_c.unwrap_or(0.0),
                    },
                    specific_heat: WindowGasPolynomialCoefficients {
                        coefficient_a: specific_heat_a.unwrap_or(0.0),
                        coefficient_b: specific_heat_b.unwrap_or(0.0),
                        coefficient_c: specific_heat_c.unwrap_or(0.0),
                    },
                    molecular_weight_g_per_mol: molecular_weight_g_per_mol.unwrap_or(0.0),
                    // EnergyPlus 26.1 accepts a blank custom specific-heat ratio
                    // and stores the input processor's numeric zero.
                    specific_heat_ratio: specific_heat_ratio.unwrap_or(0.0),
                };

                let conductivity_fields_well_typed = [
                    (CUSTOM_COEFFICIENT_FIELDS[0], conductivity_a),
                    (CUSTOM_COEFFICIENT_FIELDS[1], conductivity_b),
                    (CUSTOM_COEFFICIENT_FIELDS[2], conductivity_c),
                ]
                .iter()
                .all(|(field, value)| field_value(&object, field).is_none() || value.is_some());
                let conductivity_at_300_k = properties.conductivity.at_300_k();
                if properties_valid
                    && conductivity_fields_well_typed
                    && conductivity_at_300_k <= 0.0
                {
                    self.error(
                        "InvalidWindowGasConductivityAt300K",
                        OBJECT_TYPE,
                        Some(&name),
                        Some("conductivity_coefficient_a"),
                        format!(
                            "{OBJECT_TYPE}/{name} conductivity A + 300*B + 90000*C must be greater than zero; A={}, B={}, C={}, k300={conductivity_at_300_k}",
                            properties.conductivity.coefficient_a,
                            properties.conductivity.coefficient_b,
                            properties.conductivity.coefficient_c,
                        ),
                    );
                    properties_valid = false;
                }
                properties
            };

            if !properties_valid {
                continue;
            }
            let Some((id, normalized_name)) =
                self.reserve_material_identity(model, OBJECT_TYPE, &name)
            else {
                continue;
            };
            model.materials.push(Material {
                id,
                name: normalized_name,
                definition: MaterialDefinition::WindowGas(WindowGasMaterial {
                    gas_type,
                    thickness_m,
                    properties,
                }),
            });
        }
    }

    fn reserve_material_identity(
        &mut self,
        model: &mut TypedModel,
        object_type: &str,
        name: &str,
    ) -> Option<(MaterialId, NormalizedName)> {
        if name.trim().is_empty() {
            self.error(
                "MissingRequiredField",
                object_type,
                Some(name),
                Some("name"),
                format!("{object_type} requires a nonblank object name"),
            );
            return None;
        }
        let id = MaterialId(self.checked_id(object_type, name, model.materials.len())?);
        if model.material_names.insert(name, id).is_some() {
            self.duplicate_name(object_type, name);
            return None;
        }
        Some((id, NormalizedName::new(name)))
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
            let mut layers = vec![outside_layer];
            let mut layers_valid = true;
            for layer_number in 2..=MAX_OPAQUE_CONSTRUCTION_LAYERS {
                let field = format!("layer_{layer_number}");
                let Some(layer_name) = self.optional_string("Construction", &name, &object, &field)
                else {
                    continue;
                };
                let Some(layer) = self.resolve_name(
                    &model.material_names,
                    "Construction",
                    &name,
                    &field,
                    &layer_name,
                    "Material",
                ) else {
                    layers_valid = false;
                    continue;
                };
                layers.push(layer);
            }
            if !layers_valid {
                continue;
            }
            let Some(kind) = self.validate_construction_material_layers(model, &name, &layers)
            else {
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
                kind,
                outside_layer,
                layers,
            });
        }
    }

    fn validate_construction_material_layers(
        &mut self,
        model: &TypedModel,
        construction_name: &str,
        layers: &[MaterialId],
    ) -> Option<ConstructionKind> {
        if let Some((layer_index, material)) =
            layers
                .iter()
                .enumerate()
                .find_map(|(layer_index, material_id)| {
                    let material = model.materials.get(material_id.0 as usize)?;
                    (material.family() == ep_model::MaterialFamily::EquivalentLayer)
                        .then_some((layer_index, material))
                })
        {
            let layer_field = if layer_index == 0 {
                "outside_layer".to_string()
            } else {
                format!("layer_{}", layer_index + 1)
            };
            self.error(
                "InvalidEquivalentLayerConstruction",
                "Construction",
                Some(construction_name),
                Some(&layer_field),
                format!(
                    "Construction/{construction_name} cannot consume WindowMaterial:Glazing:EquivalentLayer {}; only the deferred Construction:WindowEquivalentLayer object may use equivalent-layer materials",
                    material.name.0
                ),
            );
            return None;
        }

        let fenestration_layer_count = layers
            .iter()
            .filter_map(|material_id| model.materials.get(material_id.0 as usize))
            .filter(|material| material.family() == ep_model::MaterialFamily::Fenestration)
            .count();
        if fenestration_layer_count != 0 {
            if fenestration_layer_count != layers.len() {
                self.error(
                    "MixedConstructionMaterialFamilies",
                    "Construction",
                    Some(construction_name),
                    None,
                    format!(
                        "Construction/{construction_name} cannot mix opaque and fenestration materials"
                    ),
                );
                return None;
            }
            if layers.len() > 7 {
                self.error(
                    "InvalidWindowConstructionLayering",
                    "Construction",
                    Some(construction_name),
                    Some("layer_8"),
                    format!(
                        "Construction/{construction_name} has more than four glazing layers and three gas gaps"
                    ),
                );
                return None;
            }
            for (layer_index, material_id) in layers.iter().enumerate() {
                let Some(material) = model.materials.get(material_id.0 as usize) else {
                    let layer_field = if layer_index == 0 {
                        "outside_layer".to_string()
                    } else {
                        format!("layer_{}", layer_index + 1)
                    };
                    self.error(
                        "InvalidConstructionMaterialReference",
                        "Construction",
                        Some(construction_name),
                        Some(&layer_field),
                        format!(
                            "Construction/{construction_name} field {layer_field} resolved to an unavailable material ID"
                        ),
                    );
                    return None;
                };
                let is_glazing = matches!(
                    material.definition,
                    MaterialDefinition::WindowGlazingSpectralAverage(_)
                        | MaterialDefinition::WindowGlazingRefractionExtinction(_)
                );
                let is_gas = matches!(material.definition, MaterialDefinition::WindowGas(_));
                let expects_glazing = layer_index % 2 == 0;
                if (expects_glazing && is_glazing) || (!expects_glazing && is_gas) {
                    continue;
                }

                let layer_field = if layer_index == 0 {
                    "outside_layer".to_string()
                } else {
                    format!("layer_{}", layer_index + 1)
                };
                let expected = if expects_glazing {
                    "a glazing layer"
                } else {
                    "a WindowMaterial:Gas gap"
                };
                self.error(
                    "InvalidWindowConstructionLayering",
                    "Construction",
                    Some(construction_name),
                    Some(&layer_field),
                    format!(
                        "Construction/{construction_name} field {layer_field} must be {expected}; the typed window subset requires Glass (Gas Glass) repeated up to three times"
                    ),
                );
                return None;
            }
            if layers.len().is_multiple_of(2) {
                let layer_field = format!("layer_{}", layers.len());
                self.error(
                    "InvalidWindowConstructionLayering",
                    "Construction",
                    Some(construction_name),
                    Some(&layer_field),
                    format!(
                        "Construction/{construction_name} field {layer_field} is a trailing gas gap; a window construction must end with glazing"
                    ),
                );
                return None;
            }
            return Some(ConstructionKind::Fenestration);
        }

        let mut valid = true;
        if let Some(outside_material) = layers
            .first()
            .and_then(|material_id| model.materials.get(material_id.0 as usize))
            && matches!(outside_material.definition, MaterialDefinition::AirGap(_))
        {
            self.error(
                "InvalidAirGapLayerPosition",
                "Construction",
                Some(construction_name),
                Some("outside_layer"),
                format!(
                    "Construction/{construction_name} cannot use Material:AirGap {} as its outside layer",
                    outside_material.name.0
                ),
            );
            valid = false;
        }

        if layers.len() > 1
            && let Some(inside_material) = layers
                .last()
                .and_then(|material_id| model.materials.get(material_id.0 as usize))
            && matches!(inside_material.definition, MaterialDefinition::AirGap(_))
        {
            let inside_field = format!("layer_{}", layers.len());
            self.error(
                "InvalidAirGapLayerPosition",
                "Construction",
                Some(construction_name),
                Some(&inside_field),
                format!(
                    "Construction/{construction_name} cannot use Material:AirGap {} as its inside layer",
                    inside_material.name.0
                ),
            );
            valid = false;
        }

        if layers.len() != 1
            && let Some((layer_index, material)) =
                layers
                    .iter()
                    .enumerate()
                    .find_map(|(layer_index, material_id)| {
                        let material = model.materials.get(material_id.0 as usize)?;
                        matches!(
                            material.definition,
                            MaterialDefinition::InfraredTransparent(_)
                        )
                        .then_some((layer_index, material))
                    })
        {
            let layer_field = if layer_index == 0 {
                "outside_layer".to_string()
            } else {
                format!("layer_{}", layer_index + 1)
            };
            self.error(
                "InvalidInfraredTransparentConstruction",
                "Construction",
                Some(construction_name),
                Some(&layer_field),
                format!(
                    "Construction/{construction_name} must use Material:InfraredTransparent {} as its only layer",
                    material.name.0
                ),
            );
            valid = false;
        }

        valid.then_some(ConstructionKind::Opaque)
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

    fn parse_file_shading_schedule(&mut self, model: &mut TypedModel) {
        const OBJECT_TYPE: &str = "Schedule:File:Shading";
        let objects = self.objects(OBJECT_TYPE);
        if objects.len() > 1 {
            self.warning(
                "ExtraScheduleFileShadingObjectsIgnored",
                OBJECT_TYPE,
                None,
                None,
                format!(
                    "{OBJECT_TYPE} has {} objects; only the first source-ordered object is used",
                    objects.len()
                ),
            );
        }
        let Some((object_name, object)) = objects.into_iter().next() else {
            return;
        };
        let Some(file_name) = self.required_string(OBJECT_TYPE, &object_name, &object, "file_name")
        else {
            return;
        };

        let is_csv = Path::new(&file_name)
            .extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| extension.eq_ignore_ascii_case("csv"));
        if !is_csv {
            self.error(
                "UnsupportedScheduleFileShadingFormat",
                OBJECT_TYPE,
                Some(&object_name),
                Some("file_name"),
                format!(
                    "{OBJECT_TYPE}/{object_name} currently requires a staged comma-separated .csv file"
                ),
            );
            return;
        }

        let Some(contents) = self.read_staged_auxiliary_file(
            OBJECT_TYPE,
            &object_name,
            &file_name,
            AuxiliaryFileDiagnosticCodes {
                missing_root: "MissingScheduleFileShadingAuxiliaryRoot",
                unsupported_path: "UnsupportedScheduleFileShadingPath",
                root_or_read_failed: "ScheduleFileShadingReadFailed",
                file_not_found: "ScheduleFileShadingNotFound",
            },
        ) else {
            return;
        };
        let mut lines = contents.lines();
        let Some(header_line) = lines.next() else {
            self.error(
                "EmptyScheduleFileShading",
                OBJECT_TYPE,
                Some(&object_name),
                Some("file_name"),
                format!("{OBJECT_TYPE}/{object_name} requires one CSV header row"),
            );
            return;
        };
        let header_line = header_line.strip_prefix('\u{feff}').unwrap_or(header_line);
        let mut headers = match parse_delimited_row(header_line, ',') {
            Ok(headers) => headers
                .into_iter()
                .map(|header| header.trim().to_string())
                .collect::<Vec<_>>(),
            Err(reason) => {
                self.error(
                    "ScheduleFileShadingCsvParseFailed",
                    OBJECT_TYPE,
                    Some(&object_name),
                    Some("file_name"),
                    format!("{OBJECT_TYPE}/{object_name} header row could not be parsed: {reason}"),
                );
                return;
            }
        };

        let legacy_trailing_parenthesis = headers.last().is_some_and(|header| header == "()");
        if legacy_trailing_parenthesis {
            headers.pop();
            self.warning(
                "ScheduleFileShadingLegacyEmptySurfaceColumnRemoved",
                OBJECT_TYPE,
                Some(&object_name),
                Some("file_name"),
                format!(
                    "{OBJECT_TYPE}/{object_name} removed the legacy trailing () surface header"
                ),
            );
        }
        if headers.is_empty() {
            self.error(
                "InvalidScheduleFileShadingHeader",
                OBJECT_TYPE,
                Some(&object_name),
                Some("file_name"),
                format!("{OBJECT_TYPE}/{object_name} requires a timestamp header column"),
            );
            return;
        }
        if let Some(index) = headers
            .iter()
            .enumerate()
            .skip(1)
            .find_map(|(index, header)| header.is_empty().then_some(index))
        {
            self.error(
                "InvalidScheduleFileShadingHeader",
                OBJECT_TYPE,
                Some(&object_name),
                Some("file_name"),
                format!(
                    "{OBJECT_TYPE}/{object_name} surface header column {} must not be blank",
                    index + 1
                ),
            );
            return;
        }

        let data_lines = lines.collect::<Vec<_>>();
        let timesteps_per_hour = model.timestep.number_of_timesteps_per_hour;
        if timesteps_per_hour == 0 {
            self.error(
                "InvalidScheduleFileShadingTimestep",
                OBJECT_TYPE,
                Some(&object_name),
                Some("file_name"),
                format!(
                    "{OBJECT_TYPE}/{object_name} requires a positive number of timesteps per hour"
                ),
            );
            return;
        }
        let Some(items_per_day) = 24_u32.checked_mul(timesteps_per_hour) else {
            self.error(
                "InvalidScheduleFileShadingTimestep",
                OBJECT_TYPE,
                Some(&object_name),
                Some("file_name"),
                format!(
                    "{OBJECT_TYPE}/{object_name} timestep count overflowed the annual row calculation"
                ),
            );
            return;
        };
        let (Some(non_leap_row_count), Some(leap_row_count)) = (
            365_u32.checked_mul(items_per_day),
            366_u32.checked_mul(items_per_day),
        ) else {
            self.error(
                "InvalidScheduleFileShadingTimestep",
                OBJECT_TYPE,
                Some(&object_name),
                Some("file_name"),
                format!(
                    "{OBJECT_TYPE}/{object_name} timestep count overflowed the annual row calculation"
                ),
            );
            return;
        };
        let source_day_count = if usize::try_from(non_leap_row_count).ok() == Some(data_lines.len())
        {
            Some(365)
        } else if usize::try_from(leap_row_count).ok() == Some(data_lines.len()) {
            Some(366)
        } else {
            None
        };
        let Some(source_day_count) = source_day_count else {
            self.error(
                "InvalidScheduleFileShadingRowCount",
                OBJECT_TYPE,
                Some(&object_name),
                Some("file_name"),
                format!(
                    "{OBJECT_TYPE}/{object_name} requires exactly {} or {} data rows for {timesteps_per_hour} timesteps per hour, found {}",
                    non_leap_row_count,
                    leap_row_count,
                    data_lines.len()
                ),
            );
            return;
        };

        let mut first_header_indices = BTreeMap::new();
        for (index, header) in headers.iter().enumerate() {
            first_header_indices.entry(header.clone()).or_insert(index);
        }
        let selected_headers = first_header_indices
            .into_iter()
            .filter(|(_header, index)| *index != 0)
            .collect::<Vec<_>>();
        let mut column_values = selected_headers
            .iter()
            .map(|_header| Vec::with_capacity(data_lines.len()))
            .collect::<Vec<_>>();

        for (row_index, line) in data_lines.iter().enumerate() {
            let mut fields = match parse_delimited_row(line, ',') {
                Ok(fields) => fields,
                Err(reason) => {
                    self.error(
                        "ScheduleFileShadingCsvParseFailed",
                        OBJECT_TYPE,
                        Some(&object_name),
                        Some("file_name"),
                        format!(
                            "{OBJECT_TYPE}/{object_name} row {} could not be parsed: {reason}",
                            row_index + 2
                        ),
                    );
                    return;
                }
            };
            if legacy_trailing_parenthesis
                && fields.len() == headers.len() + 1
                && fields.last().is_some_and(|field| field.trim().is_empty())
            {
                fields.pop();
            }
            if fields.len() != headers.len() {
                self.error(
                    "InvalidScheduleFileShadingColumnCount",
                    OBJECT_TYPE,
                    Some(&object_name),
                    Some("file_name"),
                    format!(
                        "{OBJECT_TYPE}/{object_name} row {} has {} columns, but the header defines {}",
                        row_index + 2,
                        fields.len(),
                        headers.len()
                    ),
                );
                return;
            }

            for ((surface_header, column_index), values) in
                selected_headers.iter().zip(column_values.iter_mut())
            {
                let value = match fields[*column_index].trim().parse::<f64>() {
                    Ok(value) if value.is_finite() => value,
                    Ok(_) | Err(_) => {
                        self.error(
                            "ScheduleFileShadingColumnNonNumeric",
                            OBJECT_TYPE,
                            Some(&object_name),
                            Some("file_name"),
                            format!(
                                "{OBJECT_TYPE}/{object_name} surface column {surface_header:?} row {} is not a finite number",
                                row_index + 2
                            ),
                        );
                        return;
                    }
                };
                values.push(value);
            }
        }

        let mut columns = Vec::with_capacity(selected_headers.len());
        for ((surface_header, _column_index), values) in
            selected_headers.into_iter().zip(column_values)
        {
            let generated_name = format!("{surface_header}_shading");
            let Some(id_value) = self.checked_id(OBJECT_TYPE, &generated_name, columns.len())
            else {
                continue;
            };
            let id = ScheduleId(id_value);
            if model.schedule_names.insert(&generated_name, id).is_some() {
                self.duplicate_name(OBJECT_TYPE, &generated_name);
                continue;
            }
            columns.push(ScheduleFileShadingColumn {
                id,
                surface_header,
                schedule_name: NormalizedName::new(&generated_name),
                values,
            });
        }

        model.file_shading_schedule = Some(ScheduleFileShading {
            file_name,
            timesteps_per_hour,
            source_day_count,
            columns,
        });
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
            let schedule_index = file_shading_schedule_column_count(model) + model.schedules.len();
            let Some(id_value) = self.checked_id("Schedule:Constant", &name, schedule_index) else {
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
        let minutes_per_timestep =
            schedule_minutes_per_timestep(model.timestep.number_of_timesteps_per_hour);
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
            let schedule_index = file_shading_schedule_column_count(model)
                + model.schedules.len()
                + model.compact_schedules.len();
            let Some(id_value) = self.checked_id("Schedule:Compact", &name, schedule_index) else {
                continue;
            };
            let id = ScheduleId(id_value);
            if model.schedule_names.insert(&name, id).is_some() {
                self.duplicate_name("Schedule:Compact", &name);
                continue;
            }
            let Some(periods) = self.compact_schedule_periods(&name, &object, minutes_per_timestep)
            else {
                continue;
            };

            model.compact_schedules.push(ScheduleCompact {
                id,
                name: NormalizedName::new(&name),
                schedule_type_limits,
                periods,
            });
        }
    }

    fn parse_file_schedules(&mut self, model: &mut TypedModel) {
        for (name, object) in self.objects("Schedule:File") {
            let schedule_type_limits = match self.optional_string(
                "Schedule:File",
                &name,
                &object,
                "schedule_type_limits_name",
            ) {
                Some(type_limits_name) => self.resolve_name(
                    &model.schedule_type_limit_names,
                    "Schedule:File",
                    &name,
                    "schedule_type_limits_name",
                    &type_limits_name,
                    "ScheduleTypeLimits",
                ),
                None => None,
            };
            let Some(file_name) =
                self.required_string("Schedule:File", &name, &object, "file_name")
            else {
                continue;
            };
            let Some(column_number) =
                self.required_positive_u32("Schedule:File", &name, &object, "column_number")
            else {
                continue;
            };
            let Some(rows_to_skip_at_top) =
                self.required_u32("Schedule:File", &name, &object, "rows_to_skip_at_top")
            else {
                continue;
            };
            let number_of_hours_of_data = self.u32_default(
                "Schedule:File",
                &name,
                &object,
                "number_of_hours_of_data",
                8760,
            );
            let column_separator = self.enum_default(
                "Schedule:File",
                &name,
                (&object, "column_separator"),
                ScheduleFileColumnSeparator::Comma,
                "Comma",
                parse_schedule_file_column_separator,
            );
            let interpolate_to_timestep = self.enum_default(
                "Schedule:File",
                &name,
                (&object, "interpolate_to_timestep"),
                false,
                "No",
                parse_yes_no,
            );
            let minutes_per_item =
                self.u32_default("Schedule:File", &name, &object, "minutes_per_item", 60);
            let adjust_schedule_for_daylight_savings = self.enum_default(
                "Schedule:File",
                &name,
                (&object, "adjust_schedule_for_daylight_savings"),
                true,
                "Yes",
                parse_yes_no,
            );

            let mut supported = true;
            if number_of_hours_of_data != 8760 {
                self.error(
                    "InvalidScheduleFileHoursOfData",
                    "Schedule:File",
                    Some(&name),
                    Some("number_of_hours_of_data"),
                    format!(
                        "Schedule:File/{name} currently requires exactly 8760 hours of data; 8784 and declared/actual row-count mismatch branches are not yet ported"
                    ),
                );
                supported = false;
            }
            if minutes_per_item == 0 || 60 % minutes_per_item != 0 {
                self.error(
                    "InvalidScheduleFileMinutesPerItem",
                    "Schedule:File",
                    Some(&name),
                    Some("minutes_per_item"),
                    format!(
                        "Schedule:File/{name} field minutes_per_item must be a positive divisor of 60"
                    ),
                );
                supported = false;
            } else if minutes_per_item != 60 {
                self.error(
                    "UnsupportedScheduleFileMinutesPerItem",
                    "Schedule:File",
                    Some(&name),
                    Some("minutes_per_item"),
                    format!(
                        "Schedule:File/{name} subhourly minutes_per_item={minutes_per_item} is not yet ported"
                    ),
                );
                supported = false;
            }
            if interpolate_to_timestep {
                self.error(
                    "UnsupportedScheduleFileInterpolation",
                    "Schedule:File",
                    Some(&name),
                    Some("interpolate_to_timestep"),
                    format!("Schedule:File/{name} Interpolate to Timestep=Yes is not yet ported"),
                );
                supported = false;
            }
            if adjust_schedule_for_daylight_savings {
                self.error(
                    "UnsupportedScheduleFileDaylightSavingAdjustment",
                    "Schedule:File",
                    Some(&name),
                    Some("adjust_schedule_for_daylight_savings"),
                    format!(
                        "Schedule:File/{name} daylight-saving adjustment is not yet ported; set Adjust Schedule for Daylight Savings=No"
                    ),
                );
                supported = false;
            }

            let schedule_index = file_shading_schedule_column_count(model)
                + model.schedules.len()
                + model.compact_schedules.len()
                + model.file_schedules.len();
            let Some(id_value) = self.checked_id("Schedule:File", &name, schedule_index) else {
                continue;
            };
            let id = ScheduleId(id_value);
            if model.schedule_names.insert(&name, id).is_some() {
                self.duplicate_name("Schedule:File", &name);
                continue;
            }

            let values = if supported {
                self.schedule_file_values(
                    &name,
                    &file_name,
                    column_number,
                    rows_to_skip_at_top,
                    number_of_hours_of_data,
                    column_separator,
                )
                .unwrap_or_default()
            } else {
                Vec::new()
            };
            model.file_schedules.push(ScheduleFile {
                id,
                name: NormalizedName::new(&name),
                schedule_type_limits,
                file_name,
                column_number,
                rows_to_skip_at_top,
                number_of_hours_of_data,
                column_separator,
                interpolate_to_timestep,
                minutes_per_item,
                adjust_schedule_for_daylight_savings,
                values,
            });
        }
    }

    fn parse_day_hourly_schedules(&mut self, model: &mut TypedModel) {
        for (name, object) in self.objects("Schedule:Day:Hourly") {
            let schedule_type_limits = match self.optional_string(
                "Schedule:Day:Hourly",
                &name,
                &object,
                "schedule_type_limits_name",
            ) {
                Some(type_limits_name) => self.resolve_name(
                    &model.schedule_type_limit_names,
                    "Schedule:Day:Hourly",
                    &name,
                    "schedule_type_limits_name",
                    &type_limits_name,
                    "ScheduleTypeLimits",
                ),
                None => None,
            };
            let Some(id_value) =
                self.checked_id("Schedule:Day:Hourly", &name, model.day_schedules.len())
            else {
                continue;
            };
            let id = DayScheduleId(id_value);
            if model.day_schedule_names.insert(&name, id).is_some() {
                self.duplicate_name("Schedule:Day:Hourly", &name);
                continue;
            }

            let mut hourly_values = [0.0; 24];
            for (hour_index, hourly_value) in hourly_values.iter_mut().enumerate() {
                let field = format!("hour_{}", hour_index + 1);
                *hourly_value =
                    self.number_default("Schedule:Day:Hourly", &name, &object, &field, 0.0);
            }

            model.day_schedules.push(ScheduleDayHourly {
                id,
                name: NormalizedName::new(&name),
                schedule_type_limits,
                hourly_values,
            });
        }
    }

    fn parse_day_interval_schedules(&mut self, model: &mut TypedModel) {
        const OBJECT_TYPE: &str = "Schedule:Day:Interval";
        let minutes_per_timestep =
            schedule_minutes_per_timestep(model.timestep.number_of_timesteps_per_hour);
        for (name, object) in self.objects(OBJECT_TYPE) {
            let schedule_type_limits = match self.optional_string(
                OBJECT_TYPE,
                &name,
                &object,
                "schedule_type_limits_name",
            ) {
                Some(type_limits_name) => self.resolve_name(
                    &model.schedule_type_limit_names,
                    OBJECT_TYPE,
                    &name,
                    "schedule_type_limits_name",
                    &type_limits_name,
                    "ScheduleTypeLimits",
                ),
                None => None,
            };
            let interpolation = self.enum_default(
                OBJECT_TYPE,
                &name,
                (&object, "interpolate_to_timestep"),
                ScheduleInterpolation::No,
                "No",
                parse_schedule_interpolation,
            );
            let Some(segments) =
                self.day_interval_segments(&name, &object, interpolation, minutes_per_timestep)
            else {
                continue;
            };

            let day_schedule_index = model.day_schedules.len() + model.day_interval_schedules.len();
            let Some(id_value) = self.checked_id(OBJECT_TYPE, &name, day_schedule_index) else {
                continue;
            };
            let id = DayScheduleId(id_value);
            if model.day_schedule_names.insert(&name, id).is_some() {
                self.duplicate_name(OBJECT_TYPE, &name);
                continue;
            }

            model.day_interval_schedules.push(ScheduleDayInterval {
                id,
                name: NormalizedName::new(&name),
                schedule_type_limits,
                interpolation,
                segments,
            });
        }
    }

    fn day_interval_segments(
        &mut self,
        object_name: &str,
        object: &RawObject,
        interpolation: ScheduleInterpolation,
        minutes_per_timestep: Option<u32>,
    ) -> Option<Vec<ScheduleCompactSegment>> {
        const OBJECT_TYPE: &str = "Schedule:Day:Interval";
        const FIELD: &str = "data";
        let Some(value) = field_value(object, FIELD) else {
            self.error(
                "MissingRequiredField",
                OBJECT_TYPE,
                Some(object_name),
                Some(FIELD),
                format!("{OBJECT_TYPE}/{object_name} requires field {FIELD}"),
            );
            return None;
        };
        let RawValue::Array(values) = value else {
            self.invalid_field_type(OBJECT_TYPE, object_name, FIELD, "array");
            return None;
        };
        if values.is_empty() {
            self.error(
                "MissingScheduleDayIntervalData",
                OBJECT_TYPE,
                Some(object_name),
                Some(FIELD),
                format!(
                    "{OBJECT_TYPE}/{object_name} requires at least one source-ordered time/value entry"
                ),
            );
            return None;
        }

        let mut segments = Vec::with_capacity(values.len());
        let mut entries_valid = true;
        for (index, value) in values.iter().enumerate() {
            let RawValue::Object(fields) = value else {
                self.error(
                    "InvalidFieldType",
                    OBJECT_TYPE,
                    Some(object_name),
                    Some(FIELD),
                    format!("{OBJECT_TYPE}/{object_name} {FIELD} entry {index} must be an object"),
                );
                entries_valid = false;
                continue;
            };
            let entry = RawObject {
                fields: fields.clone(),
                source_span: None,
            };
            let entry_name = format!("{object_name}[{index}]");
            let time = self.required_string(OBJECT_TYPE, &entry_name, &entry, "time");
            let interval_value =
                self.required_number(OBJECT_TYPE, &entry_name, &entry, "value_until_time");
            let (Some(time), Some(interval_value)) = (time, interval_value) else {
                entries_valid = false;
                continue;
            };
            let Some(until_minute_of_day) = parse_schedule_time_minute(&time) else {
                self.error(
                    "InvalidScheduleDayIntervalTime",
                    OBJECT_TYPE,
                    Some(&entry_name),
                    Some("time"),
                    format!(
                        "{OBJECT_TYPE}/{entry_name} has invalid time '{time}'; expected 00:01 through 24:00"
                    ),
                );
                entries_valid = false;
                continue;
            };
            if segments
                .last()
                .is_some_and(|segment: &ScheduleCompactSegment| {
                    until_minute_of_day <= segment.until_minute_of_day
                })
            {
                self.error(
                    "InvalidScheduleDayIntervalTimeOrder",
                    OBJECT_TYPE,
                    Some(&entry_name),
                    Some("time"),
                    format!(
                        "{OBJECT_TYPE}/{object_name} times must be strictly increasing; '{time}' is not later than the prior time"
                    ),
                );
                entries_valid = false;
                continue;
            }
            if interpolation == ScheduleInterpolation::No
                && minutes_per_timestep.is_some_and(|minutes| until_minute_of_day % minutes != 0)
            {
                self.warning(
                    "ScheduleDayIntervalTimeNotAlignedToTimestep",
                    OBJECT_TYPE,
                    Some(&entry_name),
                    Some("time"),
                    format!(
                        "{OBJECT_TYPE}/{object_name} time {until_minute_of_day} minutes is not a multiple of the minutes per zone timestep"
                    ),
                );
            }
            segments.push(ScheduleCompactSegment {
                until_minute_of_day,
                value: interval_value,
            });
        }

        if segments
            .last()
            .is_some_and(|segment| segment.until_minute_of_day != 1440)
        {
            self.error(
                "IncompleteScheduleDayInterval",
                OBJECT_TYPE,
                Some(object_name),
                Some(FIELD),
                format!("{OBJECT_TYPE}/{object_name} must end with time 24:00"),
            );
            entries_valid = false;
        }
        entries_valid.then_some(segments)
    }

    fn parse_day_list_schedules(&mut self, model: &mut TypedModel) {
        const OBJECT_TYPE: &str = "Schedule:Day:List";
        for (name, object) in self.objects(OBJECT_TYPE) {
            let schedule_type_limits = match self.optional_string(
                OBJECT_TYPE,
                &name,
                &object,
                "schedule_type_limits_name",
            ) {
                Some(type_limits_name) => self.resolve_name(
                    &model.schedule_type_limit_names,
                    OBJECT_TYPE,
                    &name,
                    "schedule_type_limits_name",
                    &type_limits_name,
                    "ScheduleTypeLimits",
                ),
                None => None,
            };
            let interpolation = self.enum_default(
                OBJECT_TYPE,
                &name,
                (&object, "interpolate_to_timestep"),
                ScheduleInterpolation::No,
                "No",
                parse_schedule_interpolation,
            );
            let Some(minutes_per_item) = self.day_list_minutes_per_item(&name, &object) else {
                continue;
            };
            let Some(values) = self.day_list_values(&name, &object, minutes_per_item) else {
                continue;
            };

            let day_schedule_index = model.day_schedules.len()
                + model.day_interval_schedules.len()
                + model.day_list_schedules.len();
            let Some(id_value) = self.checked_id(OBJECT_TYPE, &name, day_schedule_index) else {
                continue;
            };
            let id = DayScheduleId(id_value);
            if model.day_schedule_names.insert(&name, id).is_some() {
                self.duplicate_name(OBJECT_TYPE, &name);
                continue;
            }

            model.day_list_schedules.push(ScheduleDayList {
                id,
                name: NormalizedName::new(&name),
                schedule_type_limits,
                interpolation,
                minutes_per_item,
                values,
            });
        }
    }

    fn day_list_minutes_per_item(&mut self, object_name: &str, object: &RawObject) -> Option<u32> {
        const OBJECT_TYPE: &str = "Schedule:Day:List";
        const FIELD: &str = "minutes_per_item";
        let minutes_per_item = self.required_u32(OBJECT_TYPE, object_name, object, FIELD)?;
        if !(1..=60).contains(&minutes_per_item) {
            self.error(
                "InvalidNumericRange",
                OBJECT_TYPE,
                Some(object_name),
                Some(FIELD),
                format!(
                    "{OBJECT_TYPE}/{object_name} field {FIELD} must be between 1 and 60, got {minutes_per_item}"
                ),
            );
            return None;
        }
        if 60 % minutes_per_item != 0 {
            self.error(
                "InvalidScheduleDayListMinutesPerItem",
                OBJECT_TYPE,
                Some(object_name),
                Some(FIELD),
                format!(
                    "{OBJECT_TYPE}/{object_name} field {FIELD} must divide 60 evenly, got {minutes_per_item}"
                ),
            );
            return None;
        }
        Some(minutes_per_item)
    }

    fn day_list_values(
        &mut self,
        object_name: &str,
        object: &RawObject,
        minutes_per_item: u32,
    ) -> Option<Vec<f64>> {
        const OBJECT_TYPE: &str = "Schedule:Day:List";
        const FIELD: &str = "extensions";
        let Some(value) = field_value(object, FIELD) else {
            self.error(
                "MissingRequiredField",
                OBJECT_TYPE,
                Some(object_name),
                Some(FIELD),
                format!("{OBJECT_TYPE}/{object_name} requires field {FIELD}"),
            );
            return None;
        };
        let RawValue::Array(entries) = value else {
            self.invalid_field_type(OBJECT_TYPE, object_name, FIELD, "array");
            return None;
        };
        let expected_count = (1440 / minutes_per_item) as usize;
        if entries.len() != expected_count {
            self.error(
                "InvalidScheduleDayListValueCount",
                OBJECT_TYPE,
                Some(object_name),
                Some(FIELD),
                format!(
                    "{OBJECT_TYPE}/{object_name} requires exactly {expected_count} source values for {minutes_per_item} minutes per item, found {}",
                    entries.len()
                ),
            );
            return None;
        }

        let mut values = Vec::with_capacity(expected_count);
        let mut entries_valid = true;
        for (index, entry) in entries.iter().enumerate() {
            let RawValue::Object(fields) = entry else {
                self.error(
                    "InvalidFieldType",
                    OBJECT_TYPE,
                    Some(object_name),
                    Some(FIELD),
                    format!("{OBJECT_TYPE}/{object_name} {FIELD} entry {index} must be an object"),
                );
                entries_valid = false;
                continue;
            };
            let entry_object = RawObject {
                fields: fields.clone(),
                source_span: None,
            };
            let entry_name = format!("{object_name}[{index}]");
            match field_value(&entry_object, "value") {
                Some(value) => match self.number_value(OBJECT_TYPE, &entry_name, "value", value) {
                    Some(value) => values.push(value),
                    None => entries_valid = false,
                },
                None => {
                    self.record_default(OBJECT_TYPE, &entry_name, "value", "0.0");
                    values.push(0.0);
                }
            }
        }
        entries_valid.then_some(values)
    }

    fn parse_week_daily_schedules(&mut self, model: &mut TypedModel) {
        const DAY_FIELDS: [&str; 12] = [
            "sunday_schedule_day_name",
            "monday_schedule_day_name",
            "tuesday_schedule_day_name",
            "wednesday_schedule_day_name",
            "thursday_schedule_day_name",
            "friday_schedule_day_name",
            "saturday_schedule_day_name",
            "holiday_schedule_day_name",
            "summerdesignday_schedule_day_name",
            "winterdesignday_schedule_day_name",
            "customday1_schedule_day_name",
            "customday2_schedule_day_name",
        ];

        for (name, object) in self.objects("Schedule:Week:Daily") {
            let mut day_schedules = [DayScheduleId(0); 12];
            let mut references_complete = true;
            for (day_schedule, field) in day_schedules.iter_mut().zip(DAY_FIELDS) {
                let Some(day_schedule_name) =
                    self.required_string("Schedule:Week:Daily", &name, &object, field)
                else {
                    references_complete = false;
                    continue;
                };
                let Some(day_schedule_id) = self.resolve_name(
                    &model.day_schedule_names,
                    "Schedule:Week:Daily",
                    &name,
                    field,
                    &day_schedule_name,
                    "Schedule:Day",
                ) else {
                    references_complete = false;
                    continue;
                };
                *day_schedule = day_schedule_id;
            }
            if !references_complete {
                continue;
            }

            let Some(id_value) =
                self.checked_id("Schedule:Week:Daily", &name, model.week_schedules.len())
            else {
                continue;
            };
            let id = WeekScheduleId(id_value);
            if model.week_schedule_names.insert(&name, id).is_some() {
                self.duplicate_name("Schedule:Week:Daily", &name);
                continue;
            }
            model.week_schedules.push(ScheduleWeekDaily {
                id,
                name: NormalizedName::new(&name),
                day_schedules,
            });
        }
    }

    fn parse_week_compact_schedules(&mut self, model: &mut TypedModel) {
        const OBJECT_TYPE: &str = "Schedule:Week:Compact";

        for (name, object) in self.objects(OBJECT_TYPE) {
            if model.week_schedule_names.resolve(&name).is_some() {
                self.duplicate_name(OBJECT_TYPE, &name);
                continue;
            }

            let Some(day_schedules) =
                self.schedule_week_compact_day_schedules(model, &name, &object)
            else {
                continue;
            };
            let week_schedule_index =
                model.week_schedules.len() + model.week_compact_schedules.len();
            let Some(id_value) = self.checked_id(OBJECT_TYPE, &name, week_schedule_index) else {
                continue;
            };
            let id = WeekScheduleId(id_value);
            if model.week_schedule_names.insert(&name, id).is_some() {
                self.duplicate_name(OBJECT_TYPE, &name);
                continue;
            }
            model.week_compact_schedules.push(ScheduleWeekCompact {
                id,
                name: NormalizedName::new(&name),
                day_schedules,
            });
        }
    }

    fn schedule_week_compact_day_schedules(
        &mut self,
        model: &TypedModel,
        object_name: &str,
        object: &RawObject,
    ) -> Option<[DayScheduleId; 12]> {
        const OBJECT_TYPE: &str = "Schedule:Week:Compact";
        const FIELD: &str = "data";
        let Some(value) = field_value(object, FIELD) else {
            self.error(
                "MissingRequiredField",
                OBJECT_TYPE,
                Some(object_name),
                Some(FIELD),
                format!("{OBJECT_TYPE}/{object_name} requires field {FIELD}"),
            );
            return None;
        };
        let RawValue::Array(entries) = value else {
            self.invalid_field_type(OBJECT_TYPE, object_name, FIELD, "array");
            return None;
        };

        let mut assigned_day_types = [false; 12];
        let mut day_schedules = [DayScheduleId(0); 12];
        let mut entries_valid = true;
        for (index, value) in entries.iter().enumerate() {
            let RawValue::Object(fields) = value else {
                self.error(
                    "InvalidFieldType",
                    OBJECT_TYPE,
                    Some(object_name),
                    Some(FIELD),
                    format!("{OBJECT_TYPE}/{object_name} {FIELD} entry {index} must be an object"),
                );
                entries_valid = false;
                continue;
            };
            let entry = RawObject {
                fields: fields.clone(),
                source_span: None,
            };
            let entry_name = format!("{object_name}[{index}]");
            let (Some(day_type_list), Some(day_schedule_name)) = (
                self.required_string(OBJECT_TYPE, &entry_name, &entry, "daytype_list"),
                self.required_string(OBJECT_TYPE, &entry_name, &entry, "schedule_day_name"),
            ) else {
                entries_valid = false;
                continue;
            };

            // EnergyPlus resolves the day schedule before processing the selector. A missing
            // reference therefore must not consume any day types for later AllOtherDays pairs.
            let Some(day_schedule_id) = self.resolve_name(
                &model.day_schedule_names,
                OBJECT_TYPE,
                &entry_name,
                "schedule_day_name",
                &day_schedule_name,
                "Schedule:Day",
            ) else {
                entries_valid = false;
                continue;
            };

            let selection = process_week_compact_day_types(&day_type_list, &mut assigned_day_types);
            if !selection.recognized {
                self.error(
                    "InvalidScheduleWeekCompactDayTypeList",
                    OBJECT_TYPE,
                    Some(&entry_name),
                    Some("daytype_list"),
                    format!(
                        "{OBJECT_TYPE}/{entry_name} has no valid day assignments in '{day_type_list}'"
                    ),
                );
                entries_valid = false;
                continue;
            }
            if selection.duplicate {
                self.error(
                    "DuplicateScheduleWeekCompactDayType",
                    OBJECT_TYPE,
                    Some(&entry_name),
                    Some("daytype_list"),
                    format!(
                        "{OBJECT_TYPE}/{entry_name} attempts a duplicate day assignment in '{day_type_list}'"
                    ),
                );
                entries_valid = false;
                continue;
            }

            for (day_schedule, selected) in day_schedules.iter_mut().zip(selection.selected) {
                if selected {
                    *day_schedule = day_schedule_id;
                }
            }
        }

        if let Some(index) = assigned_day_types.iter().position(|assigned| !assigned) {
            self.error(
                "MissingScheduleWeekCompactDayAssignments",
                OBJECT_TYPE,
                Some(object_name),
                Some(FIELD),
                format!(
                    "{OBJECT_TYPE}/{object_name} is missing an assignment for {}",
                    schedule_day_type_name(ALL_SCHEDULE_DAY_TYPES[index])
                ),
            );
            entries_valid = false;
        }

        entries_valid.then_some(day_schedules)
    }

    fn parse_year_schedules(&mut self, model: &mut TypedModel) {
        for (name, object) in self.objects("Schedule:Year") {
            let schedule_type_limits = match self.optional_string(
                "Schedule:Year",
                &name,
                &object,
                "schedule_type_limits_name",
            ) {
                Some(type_limits_name) => self.resolve_name(
                    &model.schedule_type_limit_names,
                    "Schedule:Year",
                    &name,
                    "schedule_type_limits_name",
                    &type_limits_name,
                    "ScheduleTypeLimits",
                ),
                None => None,
            };
            let Some(week_schedules) = self.schedule_year_week_pointers(model, &name, &object)
            else {
                continue;
            };

            let schedule_index = file_shading_schedule_column_count(model)
                + model.schedules.len()
                + model.compact_schedules.len()
                + model.file_schedules.len()
                + model.year_schedules.len();
            let Some(id_value) = self.checked_id("Schedule:Year", &name, schedule_index) else {
                continue;
            };
            let id = ScheduleId(id_value);
            if model.schedule_names.insert(&name, id).is_some() {
                self.duplicate_name("Schedule:Year", &name);
                continue;
            }
            model.year_schedules.push(ScheduleYear {
                id,
                name: NormalizedName::new(&name),
                schedule_type_limits,
                week_schedules,
            });
        }
    }

    fn parse_external_interface_schedules(&mut self, model: &mut TypedModel) {
        const OBJECT_TYPE: &str = "ExternalInterface:Schedule";
        const TYPE_LIMITS_FIELD: &str = "schedule_type_limits_name";
        let objects = self.objects(OBJECT_TYPE);
        if objects.is_empty() {
            return;
        }

        let live_exchange_active =
            self.objects("ExternalInterface")
                .iter()
                .any(|(_name, object)| {
                    matches!(
                        field_value(object, "name_of_external_interface"),
                        Some(RawValue::String(value)) if value.eq_ignore_ascii_case("PtolemyServer")
                    )
                });
        if !live_exchange_active {
            self.warning(
                "InactiveExternalInterfaceScheduleHeldAtInitialValue",
                OBJECT_TYPE,
                None,
                None,
                format!(
                    "{OBJECT_TYPE} BCVTB exchange is inactive; all schedules are held at their initial values"
                ),
            );
        } else {
            self.error(
                "UnsupportedExternalInterfaceLiveExchange",
                OBJECT_TYPE,
                None,
                None,
                format!(
                    "{OBJECT_TYPE} with ExternalInterface=PtolemyServer requires live BCVTB updates, which are not yet ported; compilation fails closed"
                ),
            );
        }

        for (name, object) in objects {
            let schedule_type_limits = match self.optional_string(
                OBJECT_TYPE,
                &name,
                &object,
                TYPE_LIMITS_FIELD,
            ) {
                Some(type_limits_name) => self.resolve_name(
                    &model.schedule_type_limit_names,
                    OBJECT_TYPE,
                    &name,
                    TYPE_LIMITS_FIELD,
                    &type_limits_name,
                    "ScheduleTypeLimits",
                ),
                None => {
                    let type_limits_missing = match field_value(&object, TYPE_LIMITS_FIELD) {
                        None => true,
                        Some(RawValue::String(value)) => value.trim().is_empty(),
                        Some(_) => false,
                    };
                    if type_limits_missing {
                        self.warning(
                            "MissingExternalInterfaceScheduleTypeLimits",
                            OBJECT_TYPE,
                            Some(&name),
                            Some(TYPE_LIMITS_FIELD),
                            format!(
                                "{OBJECT_TYPE}/{name} has no Schedule Type Limits Name; Schedule will not be validated."
                            ),
                        );
                    }
                    None
                }
            };
            let Some(initial_value) =
                self.required_number(OBJECT_TYPE, &name, &object, "initial_value")
            else {
                continue;
            };

            let schedule_index = file_shading_schedule_column_count(model)
                + model.schedules.len()
                + model.compact_schedules.len()
                + model.file_schedules.len()
                + model.year_schedules.len()
                + model.external_interface_schedules.len();
            let Some(id_value) = self.checked_id(OBJECT_TYPE, &name, schedule_index) else {
                continue;
            };
            let id = ScheduleId(id_value);
            if model.schedule_names.insert(&name, id).is_some() {
                self.duplicate_name(OBJECT_TYPE, &name);
                continue;
            }

            model
                .external_interface_schedules
                .push(ExternalInterfaceSchedule {
                    id,
                    name: NormalizedName::new(&name),
                    schedule_type_limits,
                    initial_value,
                });
        }
    }

    fn parse_external_interface_fmu_import_schedules(&mut self, model: &mut TypedModel) {
        const OBJECT_TYPE: &str = "ExternalInterface:FunctionalMockupUnitImport:To:Schedule";
        const TYPE_LIMITS_FIELD: &str = "schedule_type_limits_names";
        let objects = self.objects(OBJECT_TYPE);
        if objects.is_empty() {
            return;
        }

        let live_exchange_active =
            self.objects("ExternalInterface")
                .iter()
                .any(|(_name, object)| {
                    matches!(
                        field_value(object, "name_of_external_interface"),
                        Some(RawValue::String(value))
                            if value.eq_ignore_ascii_case("FunctionalMockupUnitImport")
                    )
                });
        if live_exchange_active {
            self.error(
                "UnsupportedExternalInterfaceLiveExchange",
                OBJECT_TYPE,
                None,
                None,
                format!(
                    "{OBJECT_TYPE} with ExternalInterface=FunctionalMockupUnitImport requires live FMU updates, which are not yet ported; compilation fails closed"
                ),
            );
        } else {
            self.warning(
                "InactiveExternalInterfaceFmuImportScheduleHeldAtInitialValue",
                OBJECT_TYPE,
                None,
                None,
                format!(
                    "{OBJECT_TYPE} FMU import exchange is inactive; all schedules are held at their initial values"
                ),
            );
        }

        for (name, object) in objects {
            let schedule_type_limits = match self.optional_string(
                OBJECT_TYPE,
                &name,
                &object,
                TYPE_LIMITS_FIELD,
            ) {
                Some(type_limits_name) => self.resolve_name(
                    &model.schedule_type_limit_names,
                    OBJECT_TYPE,
                    &name,
                    TYPE_LIMITS_FIELD,
                    &type_limits_name,
                    "ScheduleTypeLimits",
                ),
                None => {
                    let type_limits_missing = match field_value(&object, TYPE_LIMITS_FIELD) {
                        None => true,
                        Some(RawValue::String(value)) => value.trim().is_empty(),
                        Some(_) => false,
                    };
                    if type_limits_missing {
                        self.warning(
                            "MissingExternalInterfaceFmuImportScheduleTypeLimits",
                            OBJECT_TYPE,
                            Some(&name),
                            Some(TYPE_LIMITS_FIELD),
                            format!(
                                "{OBJECT_TYPE}/{name} has no Schedule Type Limits Name; Schedule will not be validated."
                            ),
                        );
                    }
                    None
                }
            };
            let Some(fmu_file_name) =
                self.required_string(OBJECT_TYPE, &name, &object, "fmu_file_name")
            else {
                continue;
            };
            let Some(fmu_instance_name) =
                self.required_string(OBJECT_TYPE, &name, &object, "fmu_instance_name")
            else {
                continue;
            };
            let Some(fmu_variable_name) =
                self.required_string(OBJECT_TYPE, &name, &object, "fmu_variable_name")
            else {
                continue;
            };
            let Some(initial_value) =
                self.required_number(OBJECT_TYPE, &name, &object, "initial_value")
            else {
                continue;
            };

            let schedule_index = file_shading_schedule_column_count(model)
                + model.schedules.len()
                + model.compact_schedules.len()
                + model.file_schedules.len()
                + model.year_schedules.len()
                + model.external_interface_schedules.len()
                + model.external_interface_fmu_import_schedules.len();
            let Some(id_value) = self.checked_id(OBJECT_TYPE, &name, schedule_index) else {
                continue;
            };
            let id = ScheduleId(id_value);
            if model.schedule_names.insert(&name, id).is_some() {
                self.duplicate_name(OBJECT_TYPE, &name);
                continue;
            }

            model.external_interface_fmu_import_schedules.push(
                ExternalInterfaceFmuImportSchedule {
                    id,
                    name: NormalizedName::new(&name),
                    schedule_type_limits,
                    fmu_file_name,
                    fmu_instance_name,
                    fmu_variable_name,
                    initial_value,
                },
            );
        }
    }

    fn parse_external_interface_fmu_export_schedules(&mut self, model: &mut TypedModel) {
        const OBJECT_TYPE: &str = "ExternalInterface:FunctionalMockupUnitExport:To:Schedule";
        const SCHEDULE_NAME_FIELD: &str = "schedule_name";
        const TYPE_LIMITS_FIELD: &str = "schedule_type_limits_names";
        let objects = self.objects(OBJECT_TYPE);
        if objects.is_empty() {
            return;
        }

        let live_exchange_active =
            self.objects("ExternalInterface")
                .iter()
                .any(|(_name, object)| {
                    matches!(
                        field_value(object, "name_of_external_interface"),
                        Some(RawValue::String(value))
                            if value.eq_ignore_ascii_case("FunctionalMockupUnitExport")
                    )
                });
        if live_exchange_active {
            self.error(
                "UnsupportedExternalInterfaceLiveExchange",
                OBJECT_TYPE,
                None,
                None,
                format!(
                    "{OBJECT_TYPE} with ExternalInterface=FunctionalMockupUnitExport requires live FMU updates, which are not yet ported; compilation fails closed"
                ),
            );
        } else {
            self.warning(
                "InactiveExternalInterfaceFmuExportScheduleHeldAtInitialValue",
                OBJECT_TYPE,
                None,
                None,
                format!(
                    "{OBJECT_TYPE} FMU export exchange is inactive; all schedules are held at their initial values"
                ),
            );
        }

        for (instance_key, object) in objects {
            let Some(schedule_name) =
                self.required_string(OBJECT_TYPE, &instance_key, &object, SCHEDULE_NAME_FIELD)
            else {
                continue;
            };
            let schedule_type_limits = match self.optional_string(
                OBJECT_TYPE,
                &schedule_name,
                &object,
                TYPE_LIMITS_FIELD,
            ) {
                Some(type_limits_name) => self.resolve_name(
                    &model.schedule_type_limit_names,
                    OBJECT_TYPE,
                    &schedule_name,
                    TYPE_LIMITS_FIELD,
                    &type_limits_name,
                    "ScheduleTypeLimits",
                ),
                None => {
                    let type_limits_missing = match field_value(&object, TYPE_LIMITS_FIELD) {
                        None => true,
                        Some(RawValue::String(value)) => value.trim().is_empty(),
                        Some(_) => false,
                    };
                    if type_limits_missing {
                        self.warning(
                            "MissingExternalInterfaceFmuExportScheduleTypeLimits",
                            OBJECT_TYPE,
                            Some(&schedule_name),
                            Some(TYPE_LIMITS_FIELD),
                            format!(
                                "{OBJECT_TYPE}/{schedule_name} has no Schedule Type Limits Name; Schedule will not be validated."
                            ),
                        );
                    }
                    None
                }
            };
            let Some(fmu_variable_name) =
                self.required_string(OBJECT_TYPE, &schedule_name, &object, "fmu_variable_name")
            else {
                continue;
            };
            let initial_value =
                self.number_default(OBJECT_TYPE, &schedule_name, &object, "initial_value", 0.0);

            let schedule_index = file_shading_schedule_column_count(model)
                + model.schedules.len()
                + model.compact_schedules.len()
                + model.file_schedules.len()
                + model.year_schedules.len()
                + model.external_interface_schedules.len()
                + model.external_interface_fmu_import_schedules.len()
                + model.external_interface_fmu_export_schedules.len();
            let Some(id_value) = self.checked_id(OBJECT_TYPE, &schedule_name, schedule_index)
            else {
                continue;
            };
            let id = ScheduleId(id_value);
            if model.schedule_names.insert(&schedule_name, id).is_some() {
                self.duplicate_name(OBJECT_TYPE, &schedule_name);
                continue;
            }

            model.external_interface_fmu_export_schedules.push(
                ExternalInterfaceFmuExportSchedule {
                    id,
                    name: NormalizedName::new(&schedule_name),
                    schedule_type_limits,
                    fmu_variable_name,
                    initial_value,
                },
            );
        }
    }

    fn validate_scalar_schedule_type_limits(&mut self, model: &TypedModel) {
        for schedule in &model.schedules {
            self.validate_scalar_schedule_value(
                model,
                "Schedule:Constant",
                &schedule.name.0,
                "hourly_value",
                schedule.schedule_type_limits,
                schedule.hourly_value,
            );
        }
        for schedule in &model.external_interface_schedules {
            self.validate_scalar_schedule_value(
                model,
                "ExternalInterface:Schedule",
                &schedule.name.0,
                "initial_value",
                schedule.schedule_type_limits,
                schedule.initial_value,
            );
        }
        for schedule in &model.external_interface_fmu_import_schedules {
            self.validate_scalar_schedule_value(
                model,
                "ExternalInterface:FunctionalMockupUnitImport:To:Schedule",
                &schedule.name.0,
                "initial_value",
                schedule.schedule_type_limits,
                schedule.initial_value,
            );
        }
        for schedule in &model.external_interface_fmu_export_schedules {
            self.validate_scalar_schedule_value(
                model,
                "ExternalInterface:FunctionalMockupUnitExport:To:Schedule",
                &schedule.name.0,
                "initial_value",
                schedule.schedule_type_limits,
                schedule.initial_value,
            );
        }
    }

    fn validate_scalar_schedule_value(
        &mut self,
        model: &TypedModel,
        object_type: &str,
        object_name: &str,
        field: &str,
        schedule_type_limits: Option<ScheduleTypeLimitId>,
        value: f64,
    ) {
        let Some(type_limits_id) = schedule_type_limits else {
            return;
        };
        let Some(type_limits) = model.schedule_type_limits.get(type_limits_id.0 as usize) else {
            return;
        };
        let (Some(lower_limit), Some(upper_limit)) =
            (type_limits.lower_limit, type_limits.upper_limit)
        else {
            return;
        };

        let tolerance = f64::from(f32::EPSILON);
        if lower_limit - value > tolerance || value - upper_limit > tolerance {
            self.error(
                "ScheduleValueOutsideTypeLimits",
                object_type,
                Some(object_name),
                Some(field),
                format!(
                    "{object_type}/{object_name} field {field} value {value} is outside ScheduleTypeLimits/{} inclusive range [{lower_limit}, {upper_limit}] with f32 epsilon tolerance",
                    type_limits.name.0
                ),
            );
        }
    }

    fn schedule_year_week_pointers(
        &mut self,
        model: &TypedModel,
        object_name: &str,
        object: &RawObject,
    ) -> Option<[WeekScheduleId; 366]> {
        const OBJECT_TYPE: &str = "Schedule:Year";
        const FIELD: &str = "schedule_weeks";
        let Some(value) = field_value(object, FIELD) else {
            self.error(
                "MissingRequiredField",
                OBJECT_TYPE,
                Some(object_name),
                Some(FIELD),
                format!("{OBJECT_TYPE}/{object_name} requires field {FIELD}"),
            );
            return None;
        };
        let RawValue::Array(values) = value else {
            self.invalid_field_type(OBJECT_TYPE, object_name, FIELD, "array");
            return None;
        };
        if values.is_empty() || values.len() > 53 {
            self.error(
                "InvalidScheduleYearRangeCount",
                OBJECT_TYPE,
                Some(object_name),
                Some(FIELD),
                format!(
                    "{OBJECT_TYPE}/{object_name} requires between 1 and 53 source-ordered week ranges"
                ),
            );
            return None;
        }

        let mut week_schedules = [None; 366];
        let mut assignments = [0_u8; 366];
        let mut ranges_valid = true;
        for (index, value) in values.iter().enumerate() {
            let RawValue::Object(fields) = value else {
                self.error(
                    "InvalidFieldType",
                    OBJECT_TYPE,
                    Some(object_name),
                    Some(FIELD),
                    format!("{OBJECT_TYPE}/{object_name} {FIELD} entry {index} must be an object"),
                );
                ranges_valid = false;
                continue;
            };
            let entry = RawObject {
                fields: fields.clone(),
                source_span: None,
            };
            let entry_name = format!("{object_name}[{index}]");
            let Some(schedule_week_name) =
                self.required_string(OBJECT_TYPE, &entry_name, &entry, "schedule_week_name")
            else {
                ranges_valid = false;
                continue;
            };
            let Some(week_schedule) = self.resolve_name(
                &model.week_schedule_names,
                OBJECT_TYPE,
                &entry_name,
                "schedule_week_name",
                &schedule_week_name,
                "Schedule:Week",
            ) else {
                ranges_valid = false;
                continue;
            };
            let (Some(start_month), Some(start_day), Some(end_month), Some(end_day)) = (
                self.required_u32(OBJECT_TYPE, &entry_name, &entry, "start_month"),
                self.required_u32(OBJECT_TYPE, &entry_name, &entry, "start_day"),
                self.required_u32(OBJECT_TYPE, &entry_name, &entry, "end_month"),
                self.required_u32(OBJECT_TYPE, &entry_name, &entry, "end_day"),
            ) else {
                ranges_valid = false;
                continue;
            };
            let Some(start_ordinal) = leap_schedule_ordinal(start_month, start_day) else {
                self.error(
                    "InvalidScheduleYearDate",
                    OBJECT_TYPE,
                    Some(&entry_name),
                    Some("start_month"),
                    format!(
                        "{OBJECT_TYPE}/{entry_name} has invalid start date {start_month}/{start_day}"
                    ),
                );
                ranges_valid = false;
                continue;
            };
            let Some(end_ordinal) = leap_schedule_ordinal(end_month, end_day) else {
                self.error(
                    "InvalidScheduleYearDate",
                    OBJECT_TYPE,
                    Some(&entry_name),
                    Some("end_month"),
                    format!(
                        "{OBJECT_TYPE}/{entry_name} has invalid end date {end_month}/{end_day}"
                    ),
                );
                ranges_valid = false;
                continue;
            };

            if start_ordinal <= end_ordinal {
                for ordinal in start_ordinal..=end_ordinal {
                    assignments[ordinal - 1] += 1;
                    week_schedules[ordinal - 1] = Some(week_schedule);
                }
            } else {
                for ordinal in start_ordinal..=366 {
                    assignments[ordinal - 1] += 1;
                    week_schedules[ordinal - 1] = Some(week_schedule);
                }
                for ordinal in 1..=end_ordinal {
                    assignments[ordinal - 1] += 1;
                    week_schedules[ordinal - 1] = Some(week_schedule);
                }
            }
        }

        if assignments[59] == 0 {
            assignments[59] = assignments[58];
            week_schedules[59] = week_schedules[58];
        }
        if let Some(index) = assignments.iter().position(|count| *count == 0) {
            self.error(
                "MissingScheduleYearDays",
                OBJECT_TYPE,
                Some(object_name),
                Some(FIELD),
                format!(
                    "{OBJECT_TYPE}/{object_name} leaves leap-shaped ordinal day {} unassigned",
                    index + 1
                ),
            );
            ranges_valid = false;
        }
        if let Some(index) = assignments.iter().position(|count| *count > 1) {
            self.error(
                "OverlappingScheduleYearDays",
                OBJECT_TYPE,
                Some(object_name),
                Some(FIELD),
                format!(
                    "{OBJECT_TYPE}/{object_name} assigns leap-shaped ordinal day {} more than once",
                    index + 1
                ),
            );
            ranges_valid = false;
        }
        if !ranges_valid {
            return None;
        }

        Some(week_schedules.map(|week_schedule| week_schedule.unwrap_or(WeekScheduleId(0))))
    }

    fn read_staged_auxiliary_file(
        &mut self,
        object_type: &str,
        object_name: &str,
        file_name: &str,
        diagnostic_codes: AuxiliaryFileDiagnosticCodes,
    ) -> Option<String> {
        let Some(auxiliary_root) = self.auxiliary_root else {
            self.error(
                diagnostic_codes.missing_root,
                object_type,
                Some(object_name),
                Some("file_name"),
                format!(
                    "{object_type}/{object_name} requires a staged auxiliary-file root during compilation"
                ),
            );
            return None;
        };
        let relative_path = Path::new(file_name);
        if relative_path.is_absolute()
            || relative_path.components().any(|component| {
                matches!(
                    component,
                    Component::ParentDir | Component::RootDir | Component::Prefix(_)
                )
            })
        {
            self.error(
                diagnostic_codes.unsupported_path,
                object_type,
                Some(object_name),
                Some("file_name"),
                format!(
                    "{object_type}/{object_name} file_name must stay below the staged auxiliary-file root"
                ),
            );
            return None;
        }

        let canonical_root = match std::fs::canonicalize(auxiliary_root) {
            Ok(path) => path,
            Err(error) => {
                self.error(
                    diagnostic_codes.root_or_read_failed,
                    object_type,
                    Some(object_name),
                    Some("file_name"),
                    format!(
                        "{object_type}/{object_name} could not resolve auxiliary root {}: {error}",
                        auxiliary_root.display()
                    ),
                );
                return None;
            }
        };
        let requested_path = auxiliary_root.join(relative_path);
        let canonical_path = match std::fs::canonicalize(&requested_path) {
            Ok(path) => path,
            Err(error) => {
                self.error(
                    diagnostic_codes.file_not_found,
                    object_type,
                    Some(object_name),
                    Some("file_name"),
                    format!(
                        "{object_type}/{object_name} could not open {}: {error}",
                        requested_path.display()
                    ),
                );
                return None;
            }
        };
        if !canonical_path.starts_with(&canonical_root) {
            self.error(
                diagnostic_codes.unsupported_path,
                object_type,
                Some(object_name),
                Some("file_name"),
                format!(
                    "{object_type}/{object_name} resolved outside the staged auxiliary-file root"
                ),
            );
            return None;
        }
        match std::fs::read_to_string(&canonical_path) {
            Ok(contents) => Some(contents),
            Err(error) => {
                self.error(
                    diagnostic_codes.root_or_read_failed,
                    object_type,
                    Some(object_name),
                    Some("file_name"),
                    format!(
                        "{object_type}/{object_name} failed to read {}: {error}",
                        canonical_path.display()
                    ),
                );
                None
            }
        }
    }

    fn schedule_file_values(
        &mut self,
        object_name: &str,
        file_name: &str,
        column_number: u32,
        rows_to_skip_at_top: u32,
        number_of_hours_of_data: u32,
        column_separator: ScheduleFileColumnSeparator,
    ) -> Option<Vec<f64>> {
        let contents = self.read_staged_auxiliary_file(
            "Schedule:File",
            object_name,
            file_name,
            AuxiliaryFileDiagnosticCodes {
                missing_root: "MissingScheduleFileAuxiliaryRoot",
                unsupported_path: "UnsupportedScheduleFilePath",
                root_or_read_failed: "ScheduleFileReadFailed",
                file_not_found: "ScheduleFileNotFound",
            },
        )?;

        let data_lines = contents
            .lines()
            .skip(rows_to_skip_at_top as usize)
            .collect::<Vec<_>>();
        if data_lines.len() != number_of_hours_of_data as usize {
            self.error(
                "InvalidScheduleFileRowCount",
                "Schedule:File",
                Some(object_name),
                Some("number_of_hours_of_data"),
                format!(
                    "Schedule:File/{object_name} requires exactly {number_of_hours_of_data} selected data rows after skipping {rows_to_skip_at_top}, found {}",
                    data_lines.len()
                ),
            );
            return None;
        }

        let selected_index = (column_number - 1) as usize;
        let mut values = Vec::with_capacity(data_lines.len());
        for (row_index, line) in data_lines.iter().enumerate() {
            let fields = match parse_delimited_row(line, column_separator.delimiter()) {
                Ok(fields) => fields,
                Err(reason) => {
                    self.error(
                        "ScheduleFileCsvParseFailed",
                        "Schedule:File",
                        Some(object_name),
                        Some("file_name"),
                        format!(
                            "Schedule:File/{object_name} row {} could not be parsed: {reason}",
                            rows_to_skip_at_top as usize + row_index + 1
                        ),
                    );
                    return None;
                }
            };
            let Some(selected) = fields.get(selected_index) else {
                self.error(
                    "ScheduleFileColumnOutOfRange",
                    "Schedule:File",
                    Some(object_name),
                    Some("column_number"),
                    format!(
                        "Schedule:File/{object_name} requested column {column_number}, but row {} has only {} columns",
                        rows_to_skip_at_top as usize + row_index + 1,
                        fields.len()
                    ),
                );
                return None;
            };
            let value = match selected.trim().parse::<f64>() {
                Ok(value) if value.is_finite() => value,
                Ok(_) | Err(_) => {
                    self.error(
                        "ScheduleFileSelectedColumnNonNumeric",
                        "Schedule:File",
                        Some(object_name),
                        Some("column_number"),
                        format!(
                            "Schedule:File/{object_name} selected column {column_number} row {} is not a finite number",
                            rows_to_skip_at_top as usize + row_index + 1
                        ),
                    );
                    return None;
                }
            };
            values.push(value);
        }
        Some(values)
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

    fn parse_zone_humidistats(&mut self, model: &mut TypedModel) {
        for (name, object) in self.objects("ZoneControl:Humidistat") {
            let Some(zone_name) =
                self.required_string("ZoneControl:Humidistat", &name, &object, "zone_name")
            else {
                continue;
            };
            let Some(zone) = self.resolve_name(
                &model.zone_names,
                "ZoneControl:Humidistat",
                &name,
                "zone_name",
                &zone_name,
                "Zone",
            ) else {
                continue;
            };
            let Some(humidifying_relative_humidity_setpoint_schedule) = self
                .required_schedule_reference(
                    model,
                    "ZoneControl:Humidistat",
                    &name,
                    &object,
                    "humidifying_relative_humidity_setpoint_schedule_name",
                )
            else {
                continue;
            };
            let Some(dehumidifying_relative_humidity_setpoint_schedule) = self
                .required_schedule_reference(
                    model,
                    "ZoneControl:Humidistat",
                    &name,
                    &object,
                    "dehumidifying_relative_humidity_setpoint_schedule_name",
                )
            else {
                continue;
            };
            let Some(id_value) = self.checked_id(
                "ZoneControl:Humidistat",
                &name,
                model.zone_humidistats.len(),
            ) else {
                continue;
            };
            let id = ZoneHumidistatId(id_value);
            if model.zone_humidistat_names.insert(&name, id).is_some() {
                self.duplicate_name("ZoneControl:Humidistat", &name);
                continue;
            }

            model.zone_humidistats.push(ZoneHumidistat {
                id,
                name: NormalizedName::new(&name),
                zone,
                humidifying_relative_humidity_setpoint_schedule,
                dehumidifying_relative_humidity_setpoint_schedule,
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

    fn parse_design_specification_outdoor_air(&mut self, model: &mut TypedModel) {
        for (name, object) in self.objects("DesignSpecification:OutdoorAir") {
            let Some(id_value) = self.checked_id(
                "DesignSpecification:OutdoorAir",
                &name,
                model.design_specification_outdoor_air.len(),
            ) else {
                continue;
            };
            let id = DesignSpecificationOutdoorAirId(id_value);
            if model
                .design_specification_outdoor_air_names
                .insert(&name, id)
                .is_some()
            {
                self.duplicate_name("DesignSpecification:OutdoorAir", &name);
                continue;
            }

            model
                .design_specification_outdoor_air
                .push(DesignSpecificationOutdoorAir {
                    id,
                    name: NormalizedName::new(&name),
                    method: self.enum_default(
                        "DesignSpecification:OutdoorAir",
                        &name,
                        (&object, "outdoor_air_method"),
                        DesignSpecificationOutdoorAirMethod::FlowPerPerson,
                        "Flow/Person",
                        parse_design_specification_outdoor_air_method,
                    ),
                    outdoor_air_flow_per_person_m3_per_s_person: self.number_range_default(
                        "DesignSpecification:OutdoorAir",
                        &name,
                        &object,
                        "outdoor_air_flow_per_person",
                        0.00944,
                        0.0..=f64::INFINITY,
                    ),
                    outdoor_air_flow_per_zone_floor_area_m3_per_s_m2: self.number_range_default(
                        "DesignSpecification:OutdoorAir",
                        &name,
                        &object,
                        "outdoor_air_flow_per_zone_floor_area",
                        0.0,
                        0.0..=f64::INFINITY,
                    ),
                    outdoor_air_flow_per_zone_m3_per_s: self.number_range_default(
                        "DesignSpecification:OutdoorAir",
                        &name,
                        &object,
                        "outdoor_air_flow_per_zone",
                        0.0,
                        0.0..=f64::INFINITY,
                    ),
                    outdoor_air_flow_air_changes_per_hour: self.number_range_default(
                        "DesignSpecification:OutdoorAir",
                        &name,
                        &object,
                        "outdoor_air_flow_air_changes_per_hour",
                        0.0,
                        0.0..=f64::INFINITY,
                    ),
                    outdoor_air_schedule: self.optional_schedule_reference(
                        model,
                        "DesignSpecification:OutdoorAir",
                        &name,
                        &object,
                        "outdoor_air_schedule_name",
                    ),
                    proportional_control_minimum_outdoor_air_flow_rate_schedule: self
                        .optional_schedule_reference(
                            model,
                            "DesignSpecification:OutdoorAir",
                            &name,
                            &object,
                            "proportional_control_minimum_outdoor_air_flow_rate_schedule_name",
                        ),
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

    fn parse_fans(&mut self, model: &mut TypedModel) {
        for (object_type, kind) in [
            ("Fan:ConstantVolume", FanComponentKind::ConstantVolume),
            ("Fan:OnOff", FanComponentKind::OnOff),
            ("Fan:VariableVolume", FanComponentKind::VariableVolume),
            ("Fan:SystemModel", FanComponentKind::SystemModel),
        ] {
            for (name, object) in self.objects(object_type) {
                let Some(inlet_node_name) =
                    self.required_string(object_type, &name, &object, "air_inlet_node_name")
                else {
                    continue;
                };
                let Some(outlet_node_name) =
                    self.required_string(object_type, &name, &object, "air_outlet_node_name")
                else {
                    continue;
                };
                let Some(inlet_node) = self.register_node(model, &inlet_node_name) else {
                    continue;
                };
                let Some(outlet_node) = self.register_node(model, &outlet_node_name) else {
                    continue;
                };
                let Some(id_value) = self.checked_id(object_type, &name, model.fans.len()) else {
                    continue;
                };
                let id = ComponentId(id_value);
                if model.fan_names.insert(&name, id).is_some() {
                    self.duplicate_name(object_type, &name);
                    continue;
                }

                model.fans.push(FanComponent {
                    id,
                    kind,
                    name: NormalizedName::new(&name),
                    availability_schedule: self.optional_schedule_reference(
                        model,
                        object_type,
                        &name,
                        &object,
                        "availability_schedule_name",
                    ),
                    inlet_node,
                    outlet_node,
                    maximum_flow_rate_m3_per_s: self.optional_number(
                        object_type,
                        &name,
                        &object,
                        "maximum_flow_rate",
                    ),
                    pressure_rise_pa: self.optional_number(
                        object_type,
                        &name,
                        &object,
                        "pressure_rise",
                    ),
                });
            }
        }
    }

    fn parse_coils(&mut self, model: &mut TypedModel) {
        for (object_type, kind) in [
            ("Coil:Heating:Electric", CoilComponentKind::HeatingElectric),
            ("Coil:Heating:Fuel", CoilComponentKind::HeatingFuel),
            ("Coil:Heating:Water", CoilComponentKind::HeatingWater),
            ("Coil:Cooling:Water", CoilComponentKind::CoolingWater),
            (
                "Coil:Cooling:DX:SingleSpeed",
                CoilComponentKind::CoolingDxSingleSpeed,
            ),
        ] {
            for (name, object) in self.objects(object_type) {
                let inlet_node = self
                    .optional_string(object_type, &name, &object, "air_inlet_node_name")
                    .and_then(|node_name| self.register_node(model, &node_name));
                let outlet_node = self
                    .optional_string(object_type, &name, &object, "air_outlet_node_name")
                    .and_then(|node_name| self.register_node(model, &node_name));
                let Some(id_value) = self.checked_id(object_type, &name, model.coils.len()) else {
                    continue;
                };
                let id = ComponentId(id_value);
                if model.coil_names.insert(&name, id).is_some() {
                    self.duplicate_name(object_type, &name);
                    continue;
                }

                model.coils.push(CoilComponent {
                    id,
                    kind,
                    name: NormalizedName::new(&name),
                    inlet_node,
                    outlet_node,
                    availability_schedule: self.optional_schedule_reference(
                        model,
                        object_type,
                        &name,
                        &object,
                        "availability_schedule_name",
                    ),
                });
            }
        }
    }

    fn parse_setpoint_managers(&mut self, model: &mut TypedModel) {
        for object_type in [
            "SetpointManager:Scheduled",
            "SetpointManager:SingleZone:Reheat",
        ] {
            for (name, object) in self.objects(object_type) {
                let setpoint_node = self
                    .optional_string(
                        object_type,
                        &name,
                        &object,
                        "setpoint_node_or_nodelist_name",
                    )
                    .and_then(|node_or_list| {
                        self.register_node_or_nodelist_name(model, &node_or_list);
                        model.node_names.resolve(&node_or_list)
                    });
                let Some(id_value) =
                    self.checked_id(object_type, &name, model.setpoint_managers.len())
                else {
                    continue;
                };
                let id = ComponentId(id_value);
                if model.setpoint_manager_names.insert(&name, id).is_some() {
                    self.duplicate_name(object_type, &name);
                    continue;
                }

                model.setpoint_managers.push(SetpointManagerComponent {
                    id,
                    object_type: NormalizedName::new(object_type),
                    name: NormalizedName::new(&name),
                    setpoint_node,
                });
            }
        }
    }

    fn parse_availability_managers(&mut self, model: &mut TypedModel) {
        for object_type in ["AvailabilityManager:Scheduled"] {
            for (name, object) in self.objects(object_type) {
                let Some(id_value) =
                    self.checked_id(object_type, &name, model.availability_managers.len())
                else {
                    continue;
                };
                let id = ComponentId(id_value);
                if model.availability_manager_names.insert(&name, id).is_some() {
                    self.duplicate_name(object_type, &name);
                    continue;
                }

                model
                    .availability_managers
                    .push(AvailabilityManagerComponent {
                        id,
                        object_type: NormalizedName::new(object_type),
                        name: NormalizedName::new(&name),
                        schedule: self.optional_schedule_reference(
                            model,
                            object_type,
                            &name,
                            &object,
                            "schedule_name",
                        ),
                    });
            }
        }
    }

    fn parse_pumps_constant_speed(&mut self, model: &mut TypedModel) {
        for (name, object) in self.objects("Pump:ConstantSpeed") {
            let Some(inlet_node_name) =
                self.required_string("Pump:ConstantSpeed", &name, &object, "inlet_node_name")
            else {
                continue;
            };
            let Some(outlet_node_name) =
                self.required_string("Pump:ConstantSpeed", &name, &object, "outlet_node_name")
            else {
                continue;
            };
            let Some(inlet_node) = self.register_node(model, &inlet_node_name) else {
                continue;
            };
            let Some(outlet_node) = self.register_node(model, &outlet_node_name) else {
                continue;
            };
            let Some(id_value) = self.checked_id(
                "Pump:ConstantSpeed",
                &name,
                model.pumps_constant_speed.len(),
            ) else {
                continue;
            };
            let id = ComponentId(id_value);
            if model.pump_constant_speed_names.insert(&name, id).is_some() {
                self.duplicate_name("Pump:ConstantSpeed", &name);
                continue;
            }

            model.pumps_constant_speed.push(PumpConstantSpeed {
                id,
                name: NormalizedName::new(&name),
                inlet_node,
                outlet_node,
                design_flow_rate_m3_per_s: self.optional_autosize_or_nonnegative_number(
                    "Pump:ConstantSpeed",
                    &name,
                    &object,
                    "design_flow_rate",
                ),
                design_pump_head_pa: self.optional_number(
                    "Pump:ConstantSpeed",
                    &name,
                    &object,
                    "design_pump_head",
                ),
                pump_control_type: self
                    .optional_string("Pump:ConstantSpeed", &name, &object, "pump_control_type")
                    .map(|value| NormalizedName::new(&value)),
            });
        }
    }

    fn parse_boilers_hot_water(&mut self, model: &mut TypedModel) {
        for (name, object) in self.objects("Boiler:HotWater") {
            let Some(inlet_node_name) = self.required_string(
                "Boiler:HotWater",
                &name,
                &object,
                "boiler_water_inlet_node_name",
            ) else {
                continue;
            };
            let Some(outlet_node_name) = self.required_string(
                "Boiler:HotWater",
                &name,
                &object,
                "boiler_water_outlet_node_name",
            ) else {
                continue;
            };
            let Some(inlet_node) = self.register_node(model, &inlet_node_name) else {
                continue;
            };
            let Some(outlet_node) = self.register_node(model, &outlet_node_name) else {
                continue;
            };
            let Some(id_value) =
                self.checked_id("Boiler:HotWater", &name, model.boilers_hot_water.len())
            else {
                continue;
            };
            let id = ComponentId(id_value);
            if model.boiler_hot_water_names.insert(&name, id).is_some() {
                self.duplicate_name("Boiler:HotWater", &name);
                continue;
            }

            model.boilers_hot_water.push(BoilerHotWater {
                id,
                name: NormalizedName::new(&name),
                fuel_type: self
                    .optional_string("Boiler:HotWater", &name, &object, "fuel_type")
                    .map(|value| NormalizedName::new(&value)),
                inlet_node,
                outlet_node,
                nominal_capacity_w: self.optional_autosize_or_nonnegative_number(
                    "Boiler:HotWater",
                    &name,
                    &object,
                    "nominal_capacity",
                ),
                design_water_flow_rate_m3_per_s: self.optional_autosize_or_nonnegative_number(
                    "Boiler:HotWater",
                    &name,
                    &object,
                    "design_water_flow_rate",
                ),
            });
        }
    }

    fn parse_chillers_electric_eir(&mut self, model: &mut TypedModel) {
        for (name, object) in self.objects("Chiller:Electric:EIR") {
            let Some(chilled_water_inlet_node_name) = self.required_string(
                "Chiller:Electric:EIR",
                &name,
                &object,
                "chilled_water_inlet_node_name",
            ) else {
                continue;
            };
            let Some(chilled_water_outlet_node_name) = self.required_string(
                "Chiller:Electric:EIR",
                &name,
                &object,
                "chilled_water_outlet_node_name",
            ) else {
                continue;
            };
            let Some(chilled_water_inlet_node) =
                self.register_node(model, &chilled_water_inlet_node_name)
            else {
                continue;
            };
            let Some(chilled_water_outlet_node) =
                self.register_node(model, &chilled_water_outlet_node_name)
            else {
                continue;
            };
            let condenser_inlet_node = self
                .optional_string(
                    "Chiller:Electric:EIR",
                    &name,
                    &object,
                    "condenser_inlet_node_name",
                )
                .and_then(|value| self.register_node(model, &value));
            let condenser_outlet_node = self
                .optional_string(
                    "Chiller:Electric:EIR",
                    &name,
                    &object,
                    "condenser_outlet_node_name",
                )
                .and_then(|value| self.register_node(model, &value));
            let Some(id_value) = self.checked_id(
                "Chiller:Electric:EIR",
                &name,
                model.chillers_electric_eir.len(),
            ) else {
                continue;
            };
            let id = ComponentId(id_value);
            if model.chiller_electric_eir_names.insert(&name, id).is_some() {
                self.duplicate_name("Chiller:Electric:EIR", &name);
                continue;
            }

            model.chillers_electric_eir.push(ChillerElectricEir {
                id,
                name: NormalizedName::new(&name),
                chilled_water_inlet_node,
                chilled_water_outlet_node,
                condenser_inlet_node,
                condenser_outlet_node,
                reference_capacity_w: self.optional_autosize_or_nonnegative_number(
                    "Chiller:Electric:EIR",
                    &name,
                    &object,
                    "reference_capacity",
                ),
                reference_cop: self.optional_number(
                    "Chiller:Electric:EIR",
                    &name,
                    &object,
                    "reference_cop",
                ),
            });
        }
    }

    fn parse_plant_branches(&mut self, model: &mut TypedModel) {
        for (name, object) in self.objects("Branch") {
            let Some(components) = self.plant_branch_components(model, &name, &object) else {
                continue;
            };
            let Some(id_value) = self.checked_id("Branch", &name, model.plant_branches.len())
            else {
                continue;
            };
            let id = BranchId(id_value);
            if model.plant_branch_names.insert(&name, id).is_some() {
                self.duplicate_name("Branch", &name);
                continue;
            }

            model.plant_branches.push(PlantBranch {
                id,
                name: NormalizedName::new(&name),
                components,
            });
        }
    }

    fn parse_plant_branch_lists(&mut self, model: &mut TypedModel) {
        for (name, object) in self.objects("BranchList") {
            let Some(branches) = self.plant_branch_list_members(model, &name, &object) else {
                continue;
            };
            let Some(id_value) =
                self.checked_id("BranchList", &name, model.plant_branch_lists.len())
            else {
                continue;
            };
            let id = BranchListId(id_value);
            if model.plant_branch_list_names.insert(&name, id).is_some() {
                self.duplicate_name("BranchList", &name);
                continue;
            }

            model.plant_branch_lists.push(PlantBranchList {
                id,
                name: NormalizedName::new(&name),
                branches,
            });
        }
    }

    fn parse_plant_connectors(&mut self, model: &mut TypedModel) {
        for (object_type, kind) in [
            ("Connector:Splitter", PlantConnectorKind::Splitter),
            ("Connector:Mixer", PlantConnectorKind::Mixer),
        ] {
            for (name, object) in self.objects(object_type) {
                let Some((inlet_branches, outlet_branches)) =
                    self.plant_connector_branches(model, object_type, &name, &object, kind)
                else {
                    continue;
                };
                let Some(id_value) =
                    self.checked_id(object_type, &name, model.plant_connectors.len())
                else {
                    continue;
                };
                let id = ConnectorId(id_value);
                if model.plant_connector_names.insert(&name, id).is_some() {
                    self.duplicate_name(object_type, &name);
                    continue;
                }

                model.plant_connectors.push(PlantConnector {
                    id,
                    name: NormalizedName::new(&name),
                    kind,
                    inlet_branches,
                    outlet_branches,
                });
            }
        }
    }

    fn parse_plant_connector_lists(&mut self, model: &mut TypedModel) {
        for (name, object) in self.objects("ConnectorList") {
            let Some(connectors) = self.plant_connector_list_entries(model, &name, &object) else {
                continue;
            };
            let Some(id_value) =
                self.checked_id("ConnectorList", &name, model.plant_connector_lists.len())
            else {
                continue;
            };
            let id = ConnectorListId(id_value);
            if model.plant_connector_list_names.insert(&name, id).is_some() {
                self.duplicate_name("ConnectorList", &name);
                continue;
            }

            model.plant_connector_lists.push(PlantConnectorList {
                id,
                name: NormalizedName::new(&name),
                connectors,
            });
        }
    }

    fn parse_air_loops(&mut self, model: &mut TypedModel) {
        for (name, object) in self.objects("AirLoopHVAC") {
            let branch_list = self
                .optional_string("AirLoopHVAC", &name, &object, "branch_list_name")
                .and_then(|branch_list_name| {
                    self.resolve_name(
                        &model.plant_branch_list_names,
                        "AirLoopHVAC",
                        &name,
                        "branch_list_name",
                        &branch_list_name,
                        "BranchList",
                    )
                });
            let connector_list = self
                .optional_string("AirLoopHVAC", &name, &object, "connector_list_name")
                .and_then(|connector_list_name| {
                    self.resolve_name(
                        &model.plant_connector_list_names,
                        "AirLoopHVAC",
                        &name,
                        "connector_list_name",
                        &connector_list_name,
                        "ConnectorList",
                    )
                });
            let supply_side_inlet_node = self
                .optional_string("AirLoopHVAC", &name, &object, "supply_side_inlet_node_name")
                .and_then(|node_name| self.register_node(model, &node_name));
            let demand_side_outlet_node = self
                .optional_string(
                    "AirLoopHVAC",
                    &name,
                    &object,
                    "demand_side_outlet_node_name",
                )
                .and_then(|node_name| self.register_node(model, &node_name));
            let demand_side_inlet_node_names = self.node_or_nodelist_name_array(
                model,
                "AirLoopHVAC",
                &name,
                &object,
                "demand_side_inlet_node_names",
                "demand_side_inlet_node_name",
            );
            let supply_side_outlet_node_names = self.node_or_nodelist_name_array(
                model,
                "AirLoopHVAC",
                &name,
                &object,
                "supply_side_outlet_node_names",
                "supply_side_outlet_node_name",
            );
            let Some(id_value) = self.checked_id("AirLoopHVAC", &name, model.air_loops.len())
            else {
                continue;
            };
            let id = LoopId(id_value);
            if model.air_loop_names.insert(&name, id).is_some() {
                self.duplicate_name("AirLoopHVAC", &name);
                continue;
            }

            model.air_loops.push(AirLoopHvac {
                id,
                name: NormalizedName::new(&name),
                availability_manager_list_name: self
                    .optional_string(
                        "AirLoopHVAC",
                        &name,
                        &object,
                        "availability_manager_list_name",
                    )
                    .map(|value| NormalizedName::new(&value)),
                branch_list,
                connector_list,
                supply_side_inlet_node,
                demand_side_outlet_node,
                demand_side_inlet_node_names,
                supply_side_outlet_node_names,
            });
        }
    }

    fn parse_plant_loops(&mut self, model: &mut TypedModel) {
        for (name, object) in self.objects("PlantLoop") {
            let Some(plant_side_inlet_node_name) =
                self.required_string("PlantLoop", &name, &object, "plant_side_inlet_node_name")
            else {
                continue;
            };
            let Some(plant_side_outlet_node_name) =
                self.required_string("PlantLoop", &name, &object, "plant_side_outlet_node_name")
            else {
                continue;
            };
            let Some(demand_side_inlet_node_name) =
                self.required_string("PlantLoop", &name, &object, "demand_side_inlet_node_name")
            else {
                continue;
            };
            let Some(demand_side_outlet_node_name) =
                self.required_string("PlantLoop", &name, &object, "demand_side_outlet_node_name")
            else {
                continue;
            };
            let Some(plant_side_inlet_node) =
                self.register_node(model, &plant_side_inlet_node_name)
            else {
                continue;
            };
            let Some(plant_side_outlet_node) =
                self.register_node(model, &plant_side_outlet_node_name)
            else {
                continue;
            };
            let Some(demand_side_inlet_node) =
                self.register_node(model, &demand_side_inlet_node_name)
            else {
                continue;
            };
            let Some(demand_side_outlet_node) =
                self.register_node(model, &demand_side_outlet_node_name)
            else {
                continue;
            };
            let Some(plant_side_branch_list_name) =
                self.required_string("PlantLoop", &name, &object, "plant_side_branch_list_name")
            else {
                continue;
            };
            let Some(plant_side_branch_list) = self.resolve_name(
                &model.plant_branch_list_names,
                "PlantLoop",
                &name,
                "plant_side_branch_list_name",
                &plant_side_branch_list_name,
                "BranchList",
            ) else {
                continue;
            };
            let Some(demand_side_branch_list_name) =
                self.required_string("PlantLoop", &name, &object, "demand_side_branch_list_name")
            else {
                continue;
            };
            let Some(demand_side_branch_list) = self.resolve_name(
                &model.plant_branch_list_names,
                "PlantLoop",
                &name,
                "demand_side_branch_list_name",
                &demand_side_branch_list_name,
                "BranchList",
            ) else {
                continue;
            };
            let plant_side_connector_list = self
                .optional_string(
                    "PlantLoop",
                    &name,
                    &object,
                    "plant_side_connector_list_name",
                )
                .and_then(|connector_list_name| {
                    self.resolve_name(
                        &model.plant_connector_list_names,
                        "PlantLoop",
                        &name,
                        "plant_side_connector_list_name",
                        &connector_list_name,
                        "ConnectorList",
                    )
                });
            let demand_side_connector_list = self
                .optional_string(
                    "PlantLoop",
                    &name,
                    &object,
                    "demand_side_connector_list_name",
                )
                .and_then(|connector_list_name| {
                    self.resolve_name(
                        &model.plant_connector_list_names,
                        "PlantLoop",
                        &name,
                        "demand_side_connector_list_name",
                        &connector_list_name,
                        "ConnectorList",
                    )
                });
            let Some(id_value) = self.checked_id("PlantLoop", &name, model.plant_loops.len())
            else {
                continue;
            };
            let id = LoopId(id_value);
            if model.plant_loop_names.insert(&name, id).is_some() {
                self.duplicate_name("PlantLoop", &name);
                continue;
            }

            model.plant_loops.push(PlantLoop {
                id,
                name: NormalizedName::new(&name),
                fluid_type: self
                    .optional_string("PlantLoop", &name, &object, "fluid_type")
                    .map_or_else(
                        || NormalizedName::new("Water"),
                        |value| NormalizedName::new(&value),
                    ),
                plant_side_inlet_node,
                plant_side_outlet_node,
                plant_side_branch_list,
                plant_side_connector_list,
                demand_side_inlet_node,
                demand_side_outlet_node,
                demand_side_branch_list,
                demand_side_connector_list,
                load_distribution_scheme: self
                    .optional_string("PlantLoop", &name, &object, "load_distribution_scheme")
                    .map(|value| NormalizedName::new(&value)),
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

            let fuel_type = self
                .optional_string("OtherEquipment", &name, &object, "fuel_type")
                .map(|value| NormalizedName::new(&value))
                .unwrap_or_else(|| {
                    self.record_default("OtherEquipment", &name, "fuel_type", "None");
                    NormalizedName::new("None")
                });
            let design_level_calculation_method = self.enum_default(
                "OtherEquipment",
                &name,
                (&object, "design_level_calculation_method"),
                OtherEquipmentDesignLevelCalculationMethod::EquipmentLevel,
                "EquipmentLevel",
                parse_other_equipment_design_level_calculation_method,
            );
            let fraction_latent = self.number_range_default(
                "OtherEquipment",
                &name,
                &object,
                "fraction_latent",
                0.0,
                0.0..=1.0,
            );
            let fraction_radiant = self.number_range_default(
                "OtherEquipment",
                &name,
                &object,
                "fraction_radiant",
                0.0,
                0.0..=1.0,
            );
            let fraction_lost = self.number_range_default(
                "OtherEquipment",
                &name,
                &object,
                "fraction_lost",
                0.0,
                0.0..=1.0,
            );
            let fraction_sum = fraction_latent + fraction_radiant + fraction_lost;
            if fraction_sum > 1.0 + 1.0e-12 {
                self.error(
                    "InvalidOtherEquipmentFractionSum",
                    "OtherEquipment",
                    Some(&name),
                    Some("fraction_latent+fraction_radiant+fraction_lost"),
                    format!(
                        "OtherEquipment/{name} latent+radiant+lost fractions must be less than or equal to 1.0, got {fraction_sum}"
                    ),
                );
            }

            model.other_equipment.push(OtherEquipment {
                id,
                name: NormalizedName::new(&name),
                fuel_type,
                zone,
                schedule,
                design_level_calculation_method,
                design_level_w: self.number_default(
                    "OtherEquipment",
                    &name,
                    &object,
                    "design_level",
                    0.0,
                ),
                power_per_floor_area_w_per_m2: self.number_range_default(
                    "OtherEquipment",
                    &name,
                    &object,
                    "power_per_floor_area",
                    0.0,
                    0.0..=f64::INFINITY,
                ),
                power_per_person_w: self.number_range_default(
                    "OtherEquipment",
                    &name,
                    &object,
                    "power_per_person",
                    0.0,
                    0.0..=f64::INFINITY,
                ),
                fraction_latent,
                fraction_radiant,
                fraction_lost,
                carbon_dioxide_generation_rate_m3_per_s_w: self.number_default(
                    "OtherEquipment",
                    &name,
                    &object,
                    "carbon_dioxide_generation_rate",
                    0.0,
                ),
            });
        }
    }

    fn parse_people(&mut self, model: &mut TypedModel) {
        for (name, object) in self.objects("People") {
            let Some(zone_name) = self.required_string(
                "People",
                &name,
                &object,
                "zone_or_zonelist_or_space_or_spacelist_name",
            ) else {
                continue;
            };
            let Some(zone) = self.resolve_name(
                &model.zone_names,
                "People",
                &name,
                "zone_or_zonelist_or_space_or_spacelist_name",
                &zone_name,
                "Zone",
            ) else {
                continue;
            };
            let number_of_people_schedule = self.optional_schedule_reference(
                model,
                "People",
                &name,
                &object,
                "number_of_people_schedule_name",
            );
            let Some(id_value) = self.checked_id("People", &name, model.people.len()) else {
                continue;
            };
            let id = InternalGainId(id_value);
            if model.people_names.insert(&name, id).is_some() {
                self.duplicate_name("People", &name);
                continue;
            }

            model.people.push(People {
                id,
                name: NormalizedName::new(&name),
                zone,
                number_of_people_schedule,
                number_of_people_calculation_method: self.enum_default(
                    "People",
                    &name,
                    (&object, "number_of_people_calculation_method"),
                    PeopleNumberCalculationMethod::People,
                    "People",
                    parse_people_number_calculation_method,
                ),
                number_of_people: self.number_range_default(
                    "People",
                    &name,
                    &object,
                    "number_of_people",
                    0.0,
                    0.0..=f64::INFINITY,
                ),
                people_per_floor_area: self.number_range_default(
                    "People",
                    &name,
                    &object,
                    "people_per_floor_area",
                    0.0,
                    0.0..=f64::INFINITY,
                ),
                floor_area_per_person: self.number_range_default(
                    "People",
                    &name,
                    &object,
                    "floor_area_per_person",
                    0.0,
                    0.0..=f64::INFINITY,
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
            if model
                .constructions
                .get(construction.0 as usize)
                .is_some_and(|candidate| candidate.kind != ConstructionKind::Opaque)
            {
                self.error(
                    "InvalidBuildingSurfaceConstructionKind",
                    "BuildingSurface:Detailed",
                    Some(&name),
                    Some("construction_name"),
                    format!(
                        "BuildingSurface:Detailed/{name} requires an opaque construction; {construction_name} is a fenestration construction"
                    ),
                );
                continue;
            }
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
            let Some(zone_geometry) = model.zones.iter().find(|candidate| candidate.id == zone)
            else {
                continue;
            };
            let vertices = canonical_world_surface_vertices(
                vertices,
                model.global_geometry_rules.unwrap_or_default(),
                zone_geometry.direction_of_relative_north_deg,
                zone_geometry.origin,
                model
                    .building
                    .as_ref()
                    .map_or(0.0, |building| building.north_axis_deg),
            );
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

    fn plant_branch_components(
        &mut self,
        model: &mut TypedModel,
        object_name: &str,
        object: &RawObject,
    ) -> Option<Vec<PlantBranchComponent>> {
        let Some(value) = field_value(object, "components") else {
            self.error(
                "MissingRequiredField",
                "Branch",
                Some(object_name),
                Some("components"),
                format!("Branch/{object_name} requires field components"),
            );
            return None;
        };
        let RawValue::Array(values) = value else {
            self.invalid_field_type("Branch", object_name, "components", "array");
            return None;
        };

        let mut components = Vec::new();
        for (index, value) in values.iter().enumerate() {
            let RawValue::Object(fields) = value else {
                self.error(
                    "InvalidFieldType",
                    "Branch",
                    Some(object_name),
                    Some("components"),
                    format!("Branch/{object_name} component entry {index} must be an object"),
                );
                continue;
            };
            let entry_object = RawObject {
                fields: fields.clone(),
                source_span: None,
            };
            let entry_name = format!("{object_name}[{index}]");
            let Some(component_object_type) = self.required_string(
                "Branch",
                &entry_name,
                &entry_object,
                "component_object_type",
            ) else {
                continue;
            };
            let Some(component_name) =
                self.required_string("Branch", &entry_name, &entry_object, "component_name")
            else {
                continue;
            };
            if !self.supported_plant_component_exists(
                model,
                "Branch",
                &entry_name,
                "component_name",
                &component_object_type,
                &component_name,
            ) {
                continue;
            }
            let Some(inlet_node_name) = self.required_string(
                "Branch",
                &entry_name,
                &entry_object,
                "component_inlet_node_name",
            ) else {
                continue;
            };
            let Some(outlet_node_name) = self.required_string(
                "Branch",
                &entry_name,
                &entry_object,
                "component_outlet_node_name",
            ) else {
                continue;
            };
            let Some(inlet_node) = self.register_node(model, &inlet_node_name) else {
                continue;
            };
            let Some(outlet_node) = self.register_node(model, &outlet_node_name) else {
                continue;
            };
            components.push(PlantBranchComponent {
                object_type: NormalizedName::new(&component_object_type),
                name: NormalizedName::new(&component_name),
                inlet_node,
                outlet_node,
            });
        }

        if components.is_empty() {
            self.error(
                "MissingPlantBranchComponent",
                "Branch",
                Some(object_name),
                Some("components"),
                format!("Branch/{object_name} has no valid components"),
            );
            return None;
        }

        Some(components)
    }

    fn plant_branch_list_members(
        &mut self,
        model: &TypedModel,
        object_name: &str,
        object: &RawObject,
    ) -> Option<Vec<BranchId>> {
        let Some(value) = field_value(object, "branches") else {
            self.error(
                "MissingRequiredField",
                "BranchList",
                Some(object_name),
                Some("branches"),
                format!("BranchList/{object_name} requires field branches"),
            );
            return None;
        };
        let RawValue::Array(values) = value else {
            self.invalid_field_type("BranchList", object_name, "branches", "array");
            return None;
        };

        let mut branches = Vec::new();
        let mut seen = std::collections::BTreeSet::new();
        for (index, value) in values.iter().enumerate() {
            let RawValue::Object(fields) = value else {
                self.error(
                    "InvalidFieldType",
                    "BranchList",
                    Some(object_name),
                    Some("branches"),
                    format!("BranchList/{object_name} branch entry {index} must be an object"),
                );
                continue;
            };
            let entry_object = RawObject {
                fields: fields.clone(),
                source_span: None,
            };
            let entry_name = format!("{object_name}[{index}]");
            let Some(branch_name) =
                self.required_string("BranchList", &entry_name, &entry_object, "branch_name")
            else {
                continue;
            };
            let normalized = NormalizedName::new(&branch_name);
            if !seen.insert(normalized.clone()) {
                self.error(
                    "DuplicatePlantBranchListMember",
                    "BranchList",
                    Some(object_name),
                    Some("branch_name"),
                    format!(
                        "BranchList/{object_name} duplicates branch '{}'",
                        normalized.0
                    ),
                );
                continue;
            }
            let Some(branch) = self.resolve_name(
                &model.plant_branch_names,
                "BranchList",
                &entry_name,
                "branch_name",
                &branch_name,
                "Branch",
            ) else {
                continue;
            };
            branches.push(branch);
        }

        if branches.is_empty() {
            self.error(
                "MissingPlantBranchListMember",
                "BranchList",
                Some(object_name),
                Some("branches"),
                format!("BranchList/{object_name} has no valid branch members"),
            );
            return None;
        }

        Some(branches)
    }

    fn plant_connector_branches(
        &mut self,
        model: &TypedModel,
        object_type: &str,
        object_name: &str,
        object: &RawObject,
        kind: PlantConnectorKind,
    ) -> Option<(Vec<BranchId>, Vec<BranchId>)> {
        match kind {
            PlantConnectorKind::Splitter => {
                let inlet = self.required_branch_reference(
                    model,
                    object_type,
                    object_name,
                    object,
                    "inlet_branch_name",
                )?;
                let outlets = self.branch_reference_array(
                    model,
                    object_type,
                    object_name,
                    object,
                    "branches",
                    "outlet_branch_name",
                )?;
                Some((vec![inlet], outlets))
            }
            PlantConnectorKind::Mixer => {
                let outlet = self.required_branch_reference(
                    model,
                    object_type,
                    object_name,
                    object,
                    "outlet_branch_name",
                )?;
                let inlets = self.branch_reference_array(
                    model,
                    object_type,
                    object_name,
                    object,
                    "branches",
                    "inlet_branch_name",
                )?;
                Some((inlets, vec![outlet]))
            }
        }
    }

    fn plant_connector_list_entries(
        &mut self,
        model: &TypedModel,
        object_name: &str,
        object: &RawObject,
    ) -> Option<Vec<PlantConnectorListEntry>> {
        let mut entries = Vec::new();
        for index in 1..=2 {
            let object_type_field = format!("connector_{index}_object_type");
            let name_field = format!("connector_{index}_name");
            let Some(object_type) =
                self.optional_string("ConnectorList", object_name, object, &object_type_field)
            else {
                continue;
            };
            let Some(kind) = parse_plant_connector_kind(&object_type) else {
                self.error(
                    "InvalidEnumValue",
                    "ConnectorList",
                    Some(object_name),
                    Some(&object_type_field),
                    format!("ConnectorList/{object_name} has unsupported connector object type '{object_type}'"),
                );
                continue;
            };
            let Some(connector_name) =
                self.required_string("ConnectorList", object_name, object, &name_field)
            else {
                continue;
            };
            let Some(connector) = self.resolve_name(
                &model.plant_connector_names,
                "ConnectorList",
                object_name,
                &name_field,
                &connector_name,
                &object_type,
            ) else {
                continue;
            };
            entries.push(PlantConnectorListEntry { kind, connector });
        }

        if entries.is_empty() {
            self.error(
                "MissingPlantConnectorListMember",
                "ConnectorList",
                Some(object_name),
                Some("connector_1_name"),
                format!("ConnectorList/{object_name} has no valid connector members"),
            );
            return None;
        }

        Some(entries)
    }

    fn required_branch_reference(
        &mut self,
        model: &TypedModel,
        object_type: &str,
        object_name: &str,
        object: &RawObject,
        field: &str,
    ) -> Option<BranchId> {
        let branch_name = self.required_string(object_type, object_name, object, field)?;
        self.resolve_name(
            &model.plant_branch_names,
            object_type,
            object_name,
            field,
            &branch_name,
            "Branch",
        )
    }

    fn branch_reference_array(
        &mut self,
        model: &TypedModel,
        object_type: &str,
        object_name: &str,
        object: &RawObject,
        array_field: &str,
        name_field: &str,
    ) -> Option<Vec<BranchId>> {
        let Some(value) = field_value(object, array_field) else {
            self.error(
                "MissingRequiredField",
                object_type,
                Some(object_name),
                Some(array_field),
                format!("{object_type}/{object_name} requires field {array_field}"),
            );
            return None;
        };
        let RawValue::Array(values) = value else {
            self.invalid_field_type(object_type, object_name, array_field, "array");
            return None;
        };

        let mut branches = Vec::new();
        let mut seen = std::collections::BTreeSet::new();
        for (index, value) in values.iter().enumerate() {
            let RawValue::Object(fields) = value else {
                self.error(
                    "InvalidFieldType",
                    object_type,
                    Some(object_name),
                    Some(array_field),
                    format!("{object_type}/{object_name} branch entry {index} must be an object"),
                );
                continue;
            };
            let entry_object = RawObject {
                fields: fields.clone(),
                source_span: None,
            };
            let entry_name = format!("{object_name}[{index}]");
            let branch = self.required_branch_reference(
                model,
                object_type,
                &entry_name,
                &entry_object,
                name_field,
            )?;
            if !seen.insert(branch) {
                self.error(
                    "DuplicatePlantConnectorBranch",
                    object_type,
                    Some(object_name),
                    Some(name_field),
                    format!(
                        "{object_type}/{object_name} duplicates branch id {}",
                        branch.0
                    ),
                );
                continue;
            }
            branches.push(branch);
        }

        if branches.is_empty() {
            self.error(
                "MissingPlantConnectorBranch",
                object_type,
                Some(object_name),
                Some(array_field),
                format!("{object_type}/{object_name} has no valid branch members"),
            );
            return None;
        }

        Some(branches)
    }

    fn supported_plant_component_exists(
        &mut self,
        model: &TypedModel,
        object_type: &str,
        object_name: &str,
        field: &str,
        component_object_type: &str,
        component_name: &str,
    ) -> bool {
        match component_object_type.to_ascii_lowercase().as_str() {
            "pump:constantspeed" => self
                .resolve_name(
                    &model.pump_constant_speed_names,
                    object_type,
                    object_name,
                    field,
                    component_name,
                    "Pump:ConstantSpeed",
                )
                .is_some(),
            "boiler:hotwater" => self
                .resolve_name(
                    &model.boiler_hot_water_names,
                    object_type,
                    object_name,
                    field,
                    component_name,
                    "Boiler:HotWater",
                )
                .is_some(),
            "chiller:electric:eir" => self
                .resolve_name(
                    &model.chiller_electric_eir_names,
                    object_type,
                    object_name,
                    field,
                    component_name,
                    "Chiller:Electric:EIR",
                )
                .is_some(),
            "fan:constantvolume" | "fan:onoff" | "fan:variablevolume" | "fan:systemmodel" => self
                .resolve_name(
                    &model.fan_names,
                    object_type,
                    object_name,
                    field,
                    component_name,
                    component_object_type,
                )
                .is_some(),
            "coil:heating:electric"
            | "coil:heating:fuel"
            | "coil:heating:water"
            | "coil:cooling:water"
            | "coil:cooling:dx:singlespeed" => self
                .resolve_name(
                    &model.coil_names,
                    object_type,
                    object_name,
                    field,
                    component_name,
                    component_object_type,
                )
                .is_some(),
            _ => true,
        }
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

    fn node_or_nodelist_name_array(
        &mut self,
        model: &mut TypedModel,
        object_type: &str,
        object_name: &str,
        object: &RawObject,
        array_field: &str,
        name_field: &str,
    ) -> Vec<NormalizedName> {
        let Some(value) = field_value(object, array_field) else {
            return Vec::new();
        };
        let RawValue::Array(values) = value else {
            self.invalid_field_type(object_type, object_name, array_field, "array");
            return Vec::new();
        };

        let mut names = Vec::new();
        for (index, value) in values.iter().enumerate() {
            let RawValue::Object(fields) = value else {
                self.error(
                    "InvalidFieldType",
                    object_type,
                    Some(object_name),
                    Some(array_field),
                    format!("{object_type}/{object_name} node entry {index} must be an object"),
                );
                continue;
            };
            let entry_object = RawObject {
                fields: fields.clone(),
                source_span: None,
            };
            let entry_name = format!("{object_name}[{index}]");
            let Some(node_or_list_name) =
                self.required_string(object_type, &entry_name, &entry_object, name_field)
            else {
                continue;
            };
            self.register_node_or_nodelist_name(model, &node_or_list_name);
            names.push(NormalizedName::new(&node_or_list_name));
        }
        names
    }

    fn objects(&mut self, object_type: &str) -> Vec<(String, RawObject)> {
        match self.raw_model.ordered_instances(object_type) {
            Ok(objects) => objects
                .into_iter()
                .map(|(name, object)| (name.0.clone(), object.clone()))
                .collect(),
            Err(error) => {
                self.error(
                    "InvalidIdfDeclarationOrder",
                    object_type,
                    None,
                    None,
                    error.to_string(),
                );
                Vec::new()
            }
        }
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

    fn required_calendar_date_rule(
        &mut self,
        object_type: &str,
        object_name: &str,
        object: &RawObject,
        field: &str,
    ) -> Option<ep_model::CalendarDateRule> {
        let value = self.required_string(object_type, object_name, object, field)?;
        match parse_calendar_date_rule(&value) {
            Some(rule) => Some(rule),
            None => {
                self.error(
                    "InvalidCalendarDateRule",
                    object_type,
                    Some(object_name),
                    Some(field),
                    format!(
                        "{object_type}/{object_name} field {field} has unsupported date rule '{value}'"
                    ),
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

    fn required_material_roughness(
        &mut self,
        object_type: &str,
        object_name: &str,
        object: &RawObject,
    ) -> Option<MaterialSurfaceRoughness> {
        let roughness = self.required_string(object_type, object_name, object, "roughness")?;
        match MaterialSurfaceRoughness::from_energyplus_name(&roughness) {
            Some(value) => Some(value),
            None => {
                self.error(
                    "InvalidEnumValue",
                    object_type,
                    Some(object_name),
                    Some("roughness"),
                    format!("{object_type}/{object_name} has unsupported roughness '{roughness}'"),
                );
                None
            }
        }
    }

    fn opaque_surface_properties(
        &mut self,
        object_type: &str,
        object_name: &str,
        object: &RawObject,
    ) -> OpaqueSurfaceProperties {
        OpaqueSurfaceProperties {
            thermal_absorptance: self.material_absorptance_default(
                object_type,
                object_name,
                object,
                "thermal_absorptance",
                0.9,
                false,
                0.99999,
            ),
            solar_absorptance: self.material_absorptance_default(
                object_type,
                object_name,
                object,
                "solar_absorptance",
                0.7,
                true,
                1.0,
            ),
            visible_absorptance: self.material_absorptance_default(
                object_type,
                object_name,
                object,
                "visible_absorptance",
                0.7,
                true,
                1.0,
            ),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn material_absorptance_default(
        &mut self,
        object_type: &str,
        object_name: &str,
        object: &RawObject,
        field: &str,
        default: f64,
        minimum_inclusive: bool,
        maximum: f64,
    ) -> f64 {
        let value = self.number_default(object_type, object_name, object, field, default);
        let above_minimum = if minimum_inclusive {
            value >= 0.0
        } else {
            value > 0.0
        };
        if above_minimum && value <= maximum {
            return value;
        }

        let lower_bound = if minimum_inclusive { "[0" } else { "(0" };
        self.error(
            "InvalidNumericRange",
            object_type,
            Some(object_name),
            Some(field),
            format!(
                "{object_type}/{object_name} field {field} must be in {lower_bound}, {maximum}], got {value}"
            ),
        );
        default
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

    #[allow(clippy::too_many_arguments)]
    fn optional_number_bounded(
        &mut self,
        object_type: &str,
        object_name: &str,
        object: &RawObject,
        field: &str,
        minimum: (f64, bool),
        maximum: (f64, bool),
    ) -> Option<f64> {
        let value = self.optional_number(object_type, object_name, object, field)?;
        let minimum_valid = if minimum.1 {
            value >= minimum.0
        } else {
            value > minimum.0
        };
        let maximum_valid = if maximum.1 {
            value <= maximum.0
        } else {
            value < maximum.0
        };
        if minimum_valid && maximum_valid {
            return Some(value);
        }

        let lower_bracket = if minimum.1 { "[" } else { "(" };
        let upper_bracket = if maximum.1 { "]" } else { ")" };
        self.error(
            "InvalidNumericRange",
            object_type,
            Some(object_name),
            Some(field),
            format!(
                "{object_type}/{object_name} field {field} must be in {lower_bracket}{}, {}{upper_bracket}, got {value}",
                minimum.0, maximum.0
            ),
        );
        None
    }

    fn required_number(
        &mut self,
        object_type: &str,
        object_name: &str,
        object: &RawObject,
        field: &str,
    ) -> Option<f64> {
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
        self.number_value(object_type, object_name, field, value)
    }

    fn required_number_minimum(
        &mut self,
        object_type: &str,
        object_name: &str,
        object: &RawObject,
        field: &str,
        minimum: f64,
        inclusive: bool,
    ) -> Option<f64> {
        let value = self.required_number(object_type, object_name, object, field)?;
        let valid = if inclusive {
            value >= minimum
        } else {
            value > minimum
        };
        if valid {
            return Some(value);
        }

        let comparison = if inclusive {
            "greater than or equal to"
        } else {
            "greater than"
        };
        self.error(
            "InvalidNumericRange",
            object_type,
            Some(object_name),
            Some(field),
            format!(
                "{object_type}/{object_name} field {field} must be {comparison} {minimum}, got {value}"
            ),
        );
        None
    }

    fn required_number_range(
        &mut self,
        object_type: &str,
        object_name: &str,
        object: &RawObject,
        field: &str,
        range: std::ops::RangeInclusive<f64>,
    ) -> Option<f64> {
        let value = self.required_number(object_type, object_name, object, field)?;
        if range.contains(&value) {
            return Some(value);
        }

        let minimum = *range.start();
        let maximum = *range.end();
        self.error(
            "InvalidNumericRange",
            object_type,
            Some(object_name),
            Some(field),
            format!(
                "{object_type}/{object_name} field {field} must be between {minimum} and {maximum}, got {value}"
            ),
        );
        None
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

    #[allow(clippy::too_many_arguments)]
    fn number_bounded_default(
        &mut self,
        object_type: &str,
        object_name: &str,
        object: &RawObject,
        field: &str,
        default: f64,
        minimum: (f64, bool),
        maximum: (f64, bool),
    ) -> f64 {
        let value = self.number_default(object_type, object_name, object, field, default);
        let minimum_valid = if minimum.1 {
            value >= minimum.0
        } else {
            value > minimum.0
        };
        let maximum_valid = if maximum.1 {
            value <= maximum.0
        } else {
            value < maximum.0
        };
        if minimum_valid && maximum_valid {
            return value;
        }

        let lower_bracket = if minimum.1 { "[" } else { "(" };
        let upper_bracket = if maximum.1 { "]" } else { ")" };
        self.error(
            "InvalidNumericRange",
            object_type,
            Some(object_name),
            Some(field),
            format!(
                "{object_type}/{object_name} field {field} must be in {lower_bracket}{}, {}{upper_bracket}, got {value}",
                minimum.0, maximum.0
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

    fn required_u32(
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
        Some(value)
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

    fn auto_range_default(
        &mut self,
        object_type: &str,
        object_name: &str,
        object: &RawObject,
        field: &str,
        range: std::ops::RangeInclusive<f64>,
    ) -> AutoOrNumber {
        let default = AutoOrNumber::AutoCalculate;
        let value = match field_value(object, field) {
            Some(RawValue::String(value))
                if value.trim().is_empty() || value.eq_ignore_ascii_case("Autocalculate") =>
            {
                return default;
            }
            Some(RawValue::String(value)) => {
                self.invalid_enum_value(object_type, object_name, field, value);
                return default;
            }
            Some(value) => self
                .number_value(object_type, object_name, field, value)
                .map(AutoOrNumber::Value)
                .unwrap_or(default),
            None => {
                self.record_default(object_type, object_name, field, "Autocalculate");
                return default;
            }
        };
        let AutoOrNumber::Value(number) = value else {
            return default;
        };
        if range.contains(&number) {
            return value;
        }

        let minimum = *range.start();
        let maximum = *range.end();
        self.error(
            "InvalidNumericRange",
            object_type,
            Some(object_name),
            Some(field),
            format!(
                "{object_type}/{object_name} field {field} must be between {minimum} and {maximum}, got {number}"
            ),
        );
        default
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

    #[allow(clippy::too_many_arguments)]
    fn enum_warning_default<T: Copy>(
        &mut self,
        object_type: &str,
        object_name: &str,
        field_ref: (&RawObject, &str),
        default: T,
        default_label: &str,
        warn_on_missing: bool,
        parser: fn(&str) -> Option<T>,
    ) -> T {
        let (object, field) = field_ref;
        match field_value(object, field) {
            Some(RawValue::String(value)) if value.trim().is_empty() => {
                if warn_on_missing {
                    self.warning(
                        "MissingRequiredFieldDefaulted",
                        object_type,
                        Some(object_name),
                        Some(field),
                        format!(
                            "{object_type}/{object_name} field {field} is required; defaulting to '{default_label}'"
                        ),
                    );
                }
                self.record_default(object_type, object_name, field, default_label);
                default
            }
            Some(RawValue::String(value)) => match parser(value) {
                Some(parsed) => parsed,
                None => {
                    self.warning(
                        "InvalidEnumValueDefaulted",
                        object_type,
                        Some(object_name),
                        Some(field),
                        format!(
                            "{object_type}/{object_name} field {field} has unsupported value '{value}'; defaulting to '{default_label}'"
                        ),
                    );
                    default
                }
            },
            Some(_value) => {
                self.invalid_field_type(object_type, object_name, field, "string enum");
                default
            }
            None => {
                if warn_on_missing {
                    self.warning(
                        "MissingRequiredFieldDefaulted",
                        object_type,
                        Some(object_name),
                        Some(field),
                        format!(
                            "{object_type}/{object_name} field {field} is required; defaulting to '{default_label}'"
                        ),
                    );
                }
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
                let missing = match field_value(object, field) {
                    None => true,
                    Some(RawValue::String(value)) => value.trim().is_empty(),
                    Some(_) => false,
                };
                if missing {
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

    fn compact_schedule_periods(
        &mut self,
        object_name: &str,
        object: &RawObject,
        minutes_per_timestep: Option<u32>,
    ) -> Option<Vec<ScheduleCompactPeriod>> {
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

        let mut periods = Vec::new();
        let mut current_period: Option<CompactSchedulePeriodBuilder> = None;
        let mut current_profile: Option<CompactScheduleProfileBuilder> = None;
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
                RawValue::String(text) if compact_directive(text, "Through") => {
                    self.finish_compact_schedule_profile(
                        object_name,
                        current_period.as_mut(),
                        &mut current_profile,
                    );
                    if let Some(period) = current_period.take() {
                        self.finish_compact_schedule_period(object_name, period, &mut periods);
                    }

                    let Some(through_schedule_day_of_year) = parse_compact_through_ordinal(text)
                    else {
                        self.error(
                            "InvalidScheduleCompactThrough",
                            "Schedule:Compact",
                            Some(object_name),
                            Some("data"),
                            format!(
                                "Schedule:Compact/{object_name} has invalid fixed-date Through directive '{text}'"
                            ),
                        );
                        continue;
                    };
                    if periods.last().is_some_and(|period| {
                        through_schedule_day_of_year <= period.through_schedule_day_of_year
                    }) {
                        self.error(
                            "InvalidScheduleCompactThroughOrder",
                            "Schedule:Compact",
                            Some(object_name),
                            Some("data"),
                            format!(
                                "Schedule:Compact/{object_name} Through dates must be strictly increasing; '{text}' is not later than the prior Through date"
                            ),
                        );
                    }

                    current_period = Some(CompactSchedulePeriodBuilder {
                        period: ScheduleCompactPeriod {
                            through_schedule_day_of_year,
                            day_profiles: Vec::new(),
                        },
                        assigned_day_types: [false; 12],
                    });
                }
                RawValue::String(text) if compact_directive(text, "For") => {
                    self.finish_compact_schedule_profile(
                        object_name,
                        current_period.as_mut(),
                        &mut current_profile,
                    );
                    let Some(period) = current_period.as_mut() else {
                        self.error(
                            "InvalidScheduleCompactOrder",
                            "Schedule:Compact",
                            Some(object_name),
                            Some("data"),
                            format!(
                                "Schedule:Compact/{object_name} For directive appears before a Through directive"
                            ),
                        );
                        continue;
                    };
                    let day_types = self.compact_schedule_day_types(
                        object_name,
                        text,
                        &mut period.assigned_day_types,
                    );
                    current_profile = Some(CompactScheduleProfileBuilder {
                        profile: ScheduleCompactDayProfile {
                            day_types,
                            interpolation: ScheduleInterpolation::No,
                            segments: Vec::new(),
                        },
                        pending_until_minute_of_day: None,
                        interpolation_explicit: false,
                    });
                }
                RawValue::String(text) if compact_directive(text, "Interpolate") => {
                    let Some(profile) = current_profile.as_mut() else {
                        self.error(
                            "InvalidScheduleCompactInterpolationOrder",
                            "Schedule:Compact",
                            Some(object_name),
                            Some("data"),
                            format!(
                                "Schedule:Compact/{object_name} Interpolate directive appears before a For directive"
                            ),
                        );
                        continue;
                    };
                    if profile.interpolation_explicit {
                        self.error(
                            "DuplicateScheduleCompactInterpolation",
                            "Schedule:Compact",
                            Some(object_name),
                            Some("data"),
                            format!(
                                "Schedule:Compact/{object_name} For profile has more than one Interpolate directive"
                            ),
                        );
                        continue;
                    }
                    if profile.pending_until_minute_of_day.is_some()
                        || !profile.profile.segments.is_empty()
                    {
                        self.error(
                            "InvalidScheduleCompactInterpolationOrder",
                            "Schedule:Compact",
                            Some(object_name),
                            Some("data"),
                            format!(
                                "Schedule:Compact/{object_name} Interpolate directive must immediately follow its For directive"
                            ),
                        );
                        continue;
                    }
                    let Some(interpolation) = parse_schedule_interpolation(text) else {
                        self.error(
                            "InvalidScheduleCompactInterpolation",
                            "Schedule:Compact",
                            Some(object_name),
                            Some("data"),
                            format!(
                                "Schedule:Compact/{object_name} has invalid Interpolate directive '{text}'"
                            ),
                        );
                        continue;
                    };
                    profile.profile.interpolation = interpolation;
                    profile.interpolation_explicit = true;
                }
                RawValue::String(text) if compact_directive(text, "Until") => {
                    let Some(profile) = current_profile.as_mut() else {
                        self.error(
                            "InvalidScheduleCompactOrder",
                            "Schedule:Compact",
                            Some(object_name),
                            Some("data"),
                            format!(
                                "Schedule:Compact/{object_name} Until directive appears before a For directive"
                            ),
                        );
                        continue;
                    };
                    if let Some(unconsumed_until) = profile.pending_until_minute_of_day.take() {
                        self.error(
                            "MissingScheduleCompactValue",
                            "Schedule:Compact",
                            Some(object_name),
                            Some("data"),
                            format!(
                                "Schedule:Compact/{object_name} Until {unconsumed_until} minutes has no following numeric value"
                            ),
                        );
                    }

                    let Some(until_minute_of_day) = parse_until_minute(text) else {
                        self.error(
                            "InvalidScheduleCompactUntil",
                            "Schedule:Compact",
                            Some(object_name),
                            Some("data"),
                            format!(
                                "Schedule:Compact/{object_name} has invalid Until directive '{text}'"
                            ),
                        );
                        continue;
                    };
                    if profile
                        .profile
                        .segments
                        .last()
                        .is_some_and(|segment| until_minute_of_day <= segment.until_minute_of_day)
                    {
                        self.error(
                            "InvalidScheduleCompactUntilOrder",
                            "Schedule:Compact",
                            Some(object_name),
                            Some("data"),
                            format!(
                                "Schedule:Compact/{object_name} Until times must be strictly increasing; '{text}' is not later than the prior Until time"
                            ),
                        );
                        continue;
                    }
                    if profile.profile.interpolation == ScheduleInterpolation::No
                        && minutes_per_timestep
                            .is_some_and(|minutes| until_minute_of_day % minutes != 0)
                    {
                        self.warning(
                            "ScheduleCompactUntilNotAlignedToTimestep",
                            "Schedule:Compact",
                            Some(object_name),
                            Some("data"),
                            format!(
                                "Schedule:Compact/{object_name} Until minute {until_minute_of_day} is not a multiple of the minutes per zone timestep"
                            ),
                        );
                    }
                    profile.pending_until_minute_of_day = Some(until_minute_of_day);
                }
                RawValue::Number(_text) => {
                    let Some(profile) = current_profile.as_mut() else {
                        self.error(
                            "InvalidScheduleCompactOrder",
                            "Schedule:Compact",
                            Some(object_name),
                            Some("data"),
                            format!(
                                "Schedule:Compact/{object_name} numeric value appears before a For directive"
                            ),
                        );
                        continue;
                    };
                    let Some(until_minute_of_day) = profile.pending_until_minute_of_day.take()
                    else {
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
                    profile.profile.segments.push(ScheduleCompactSegment {
                        until_minute_of_day,
                        value,
                    });
                }
                RawValue::String(text) => {
                    self.error(
                        "UnsupportedScheduleCompactDirective",
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

        self.finish_compact_schedule_profile(
            object_name,
            current_period.as_mut(),
            &mut current_profile,
        );
        if let Some(period) = current_period.take() {
            self.finish_compact_schedule_period(object_name, period, &mut periods);
        }

        if periods.is_empty() {
            self.error(
                "MissingScheduleCompactThrough",
                "Schedule:Compact",
                Some(object_name),
                Some("data"),
                format!(
                    "Schedule:Compact/{object_name} requires at least one valid Through period"
                ),
            );
            return None;
        }
        if periods
            .last()
            .is_some_and(|period| period.through_schedule_day_of_year != 366)
        {
            self.error(
                "MissingScheduleCompactFinalThrough",
                "Schedule:Compact",
                Some(object_name),
                Some("data"),
                format!("Schedule:Compact/{object_name} final Through date must be 12/31"),
            );
        }

        Some(periods)
    }

    fn compact_schedule_day_types(
        &mut self,
        object_name: &str,
        directive: &str,
        assigned_day_types: &mut [bool; 12],
    ) -> Vec<ScheduleDayType> {
        let Some((_prefix, body)) = directive.split_once(':') else {
            return Vec::new();
        };
        let tokens = body
            .split(|character: char| character.is_whitespace() || character == ',')
            .filter(|token| !token.is_empty())
            .collect::<Vec<_>>();
        if tokens.is_empty() {
            self.error(
                "InvalidScheduleCompactFor",
                "Schedule:Compact",
                Some(object_name),
                Some("data"),
                format!("Schedule:Compact/{object_name} For directive has no day types"),
            );
        }

        let mut selected_day_types = Vec::new();
        let mut include_all_other_days = false;
        for token in tokens {
            if token.eq_ignore_ascii_case("AllOtherDays") {
                include_all_other_days = true;
                continue;
            }

            let Some(expanded_day_types) = expand_compact_day_type_token(token) else {
                self.error(
                    "UnsupportedScheduleCompactDayType",
                    "Schedule:Compact",
                    Some(object_name),
                    Some("data"),
                    format!(
                        "Schedule:Compact/{object_name} has unsupported For day type '{token}'"
                    ),
                );
                continue;
            };
            for day_type in expanded_day_types {
                let day_type_index = schedule_day_type_index(day_type);
                if assigned_day_types[day_type_index] {
                    self.error(
                        "DuplicateScheduleCompactDayType",
                        "Schedule:Compact",
                        Some(object_name),
                        Some("data"),
                        format!(
                            "Schedule:Compact/{object_name} assigns {} more than once in one Through period",
                            schedule_day_type_name(day_type)
                        ),
                    );
                    continue;
                }
                assigned_day_types[day_type_index] = true;
                selected_day_types.push(day_type);
            }
        }

        if include_all_other_days {
            for day_type in ALL_SCHEDULE_DAY_TYPES {
                let day_type_index = schedule_day_type_index(day_type);
                if !assigned_day_types[day_type_index] {
                    assigned_day_types[day_type_index] = true;
                    selected_day_types.push(day_type);
                }
            }
        }

        selected_day_types
    }

    fn finish_compact_schedule_profile(
        &mut self,
        object_name: &str,
        period: Option<&mut CompactSchedulePeriodBuilder>,
        current_profile: &mut Option<CompactScheduleProfileBuilder>,
    ) {
        let Some(profile) = current_profile.take() else {
            return;
        };
        if let Some(until_minute_of_day) = profile.pending_until_minute_of_day {
            self.error(
                "MissingScheduleCompactValue",
                "Schedule:Compact",
                Some(object_name),
                Some("data"),
                format!(
                    "Schedule:Compact/{object_name} Until {until_minute_of_day} minutes has no following numeric value"
                ),
            );
        }
        match profile.profile.segments.last() {
            None => self.error(
                "MissingScheduleCompactSegments",
                "Schedule:Compact",
                Some(object_name),
                Some("data"),
                format!(
                    "Schedule:Compact/{object_name} For profile requires at least one Until/value segment"
                ),
            ),
            Some(segment) if segment.until_minute_of_day != 1440 => self.error(
                "IncompleteScheduleCompactDayProfile",
                "Schedule:Compact",
                Some(object_name),
                Some("data"),
                format!(
                    "Schedule:Compact/{object_name} For profile must end with Until: 24:00"
                ),
            ),
            Some(_) => {}
        }
        if let Some(period) = period {
            period.period.day_profiles.push(profile.profile);
        }
    }

    fn finish_compact_schedule_period(
        &mut self,
        object_name: &str,
        period: CompactSchedulePeriodBuilder,
        periods: &mut Vec<ScheduleCompactPeriod>,
    ) {
        if period.period.day_profiles.is_empty() {
            self.error(
                "MissingScheduleCompactFor",
                "Schedule:Compact",
                Some(object_name),
                Some("data"),
                format!(
                    "Schedule:Compact/{object_name} Through period requires at least one For profile"
                ),
            );
        }
        if period.assigned_day_types.iter().any(|assigned| !assigned) {
            self.error(
                "IncompleteScheduleCompactDayTypes",
                "Schedule:Compact",
                Some(object_name),
                Some("data"),
                format!(
                    "Schedule:Compact/{object_name} must assign all 12 schedule day types in each Through period"
                ),
            );
        }
        periods.push(period.period);
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

    fn warning(
        &mut self,
        code: &str,
        object_type: &str,
        object_name: Option<&str>,
        field: Option<&str>,
        message: String,
    ) {
        self.diagnostics.push(ModelDiagnostic {
            severity: DiagnosticSeverity::Warning,
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
    let value = value.trim_start();
    value
        .get(..directive.len())
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case(directive))
        && value[directive.len()..].trim_start().starts_with(':')
}

fn parse_schedule_interpolation(value: &str) -> Option<ScheduleInterpolation> {
    let value = value.trim();
    let value = if let Some((directive, interpolation)) = value.split_once(':') {
        if !directive.trim().eq_ignore_ascii_case("Interpolate") {
            return None;
        }
        interpolation.trim()
    } else {
        value
    };
    match value {
        interpolation if interpolation.eq_ignore_ascii_case("No") => {
            Some(ScheduleInterpolation::No)
        }
        interpolation if interpolation.eq_ignore_ascii_case("Average") => {
            Some(ScheduleInterpolation::Average)
        }
        interpolation if interpolation.eq_ignore_ascii_case("Linear") => {
            Some(ScheduleInterpolation::Linear)
        }
        _ => None,
    }
}

fn parse_schedule_file_column_separator(value: &str) -> Option<ScheduleFileColumnSeparator> {
    match value.trim() {
        value if value.eq_ignore_ascii_case("Comma") => Some(ScheduleFileColumnSeparator::Comma),
        value if value.eq_ignore_ascii_case("Tab") => Some(ScheduleFileColumnSeparator::Tab),
        value if value.eq_ignore_ascii_case("Space") => Some(ScheduleFileColumnSeparator::Space),
        value if value.eq_ignore_ascii_case("Semicolon") => {
            Some(ScheduleFileColumnSeparator::Semicolon)
        }
        _ => None,
    }
}

fn parse_delimited_row(line: &str, delimiter: char) -> Result<Vec<String>, &'static str> {
    let mut fields = Vec::new();
    let mut field = String::new();
    let mut characters = line.chars().peekable();
    let mut in_quotes = false;

    while let Some(character) = characters.next() {
        if in_quotes {
            if character == '"' {
                if characters.peek() == Some(&'"') {
                    field.push('"');
                    characters.next();
                } else {
                    in_quotes = false;
                }
            } else {
                field.push(character);
            }
        } else if character == delimiter {
            fields.push(std::mem::take(&mut field));
        } else if character == '"' {
            if !field.is_empty() {
                return Err("quote appeared after unquoted field content");
            }
            in_quotes = true;
        } else {
            field.push(character);
        }
    }
    if in_quotes {
        return Err("quoted field was not terminated");
    }
    fields.push(field);
    Ok(fields)
}

fn file_shading_schedule_column_count(model: &TypedModel) -> usize {
    model
        .file_shading_schedule
        .as_ref()
        .map_or(0, |schedule| schedule.columns.len())
}

fn schedule_minutes_per_timestep(timesteps_per_hour: u32) -> Option<u32> {
    (timesteps_per_hour > 0 && 60 % timesteps_per_hour == 0).then(|| 60 / timesteps_per_hour)
}

const ALL_SCHEDULE_DAY_TYPES: [ScheduleDayType; 12] = [
    ScheduleDayType::Sunday,
    ScheduleDayType::Monday,
    ScheduleDayType::Tuesday,
    ScheduleDayType::Wednesday,
    ScheduleDayType::Thursday,
    ScheduleDayType::Friday,
    ScheduleDayType::Saturday,
    ScheduleDayType::Holiday,
    ScheduleDayType::SummerDesignDay,
    ScheduleDayType::WinterDesignDay,
    ScheduleDayType::CustomDay1,
    ScheduleDayType::CustomDay2,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct WeekCompactDayTypeSelection {
    selected: [bool; 12],
    recognized: bool,
    duplicate: bool,
}

fn process_week_compact_day_types(
    value: &str,
    assigned: &mut [bool; 12],
) -> WeekCompactDayTypeSelection {
    let value = value.to_ascii_uppercase();
    let mut selection = WeekCompactDayTypeSelection {
        selected: [false; 12],
        recognized: false,
        duplicate: false,
    };

    macro_rules! select_if_present {
        ($needle:literal, [$($index:expr),+ $(,)?]) => {
            if value.contains($needle) {
                selection.recognized = true;
                select_week_compact_day_types(
                    &mut selection.selected,
                    assigned,
                    &mut selection.duplicate,
                    &[$($index),+],
                );
            }
        };
    }

    // Keep this order aligned with EnergyPlus ProcessForDayTypes. It intentionally uses
    // substring matching rather than tokenization; optional `For` and `:` text is irrelevant.
    select_if_present!("WEEKDAY", [1, 2, 3, 4, 5]);
    select_if_present!("MONDAY", [1]);
    select_if_present!("TUESDAY", [2]);
    select_if_present!("WEDNESDAY", [3]);
    select_if_present!("THURSDAY", [4]);
    select_if_present!("FRIDAY", [5]);
    select_if_present!("WEEKEND", [0, 6]);
    select_if_present!("SATURDAY", [6]);
    select_if_present!("SUNDAY", [0]);
    select_if_present!("CUSTOMDAY1", [10]);
    select_if_present!("CUSTOMDAY2", [11]);
    select_if_present!("ALLDAY", [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
    select_if_present!("HOLIDAY", [7]);
    select_if_present!("SUMMER", [8]);
    select_if_present!("WINTER", [9]);

    if value.contains("ALLOTHERDAY") {
        selection.recognized = true;
        for (selected, assigned) in selection.selected.iter_mut().zip(assigned.iter_mut()) {
            if !*assigned {
                *selected = true;
                *assigned = true;
            }
        }
    }

    selection
}

fn select_week_compact_day_types(
    selected: &mut [bool; 12],
    assigned: &mut [bool; 12],
    duplicate: &mut bool,
    indices: &[usize],
) {
    for index in indices {
        selected[*index] = true;
        if assigned[*index] {
            *duplicate = true;
        } else {
            assigned[*index] = true;
        }
    }
}

fn expand_compact_day_type_token(token: &str) -> Option<Vec<ScheduleDayType>> {
    match token.to_ascii_lowercase().as_str() {
        "alldays" => Some(ALL_SCHEDULE_DAY_TYPES.to_vec()),
        "weekdays" => Some(vec![
            ScheduleDayType::Monday,
            ScheduleDayType::Tuesday,
            ScheduleDayType::Wednesday,
            ScheduleDayType::Thursday,
            ScheduleDayType::Friday,
        ]),
        "weekends" => Some(vec![ScheduleDayType::Saturday, ScheduleDayType::Sunday]),
        "sunday" => Some(vec![ScheduleDayType::Sunday]),
        "monday" => Some(vec![ScheduleDayType::Monday]),
        "tuesday" => Some(vec![ScheduleDayType::Tuesday]),
        "wednesday" => Some(vec![ScheduleDayType::Wednesday]),
        "thursday" => Some(vec![ScheduleDayType::Thursday]),
        "friday" => Some(vec![ScheduleDayType::Friday]),
        "saturday" => Some(vec![ScheduleDayType::Saturday]),
        "holiday" => Some(vec![ScheduleDayType::Holiday]),
        "summerdesignday" => Some(vec![ScheduleDayType::SummerDesignDay]),
        "winterdesignday" => Some(vec![ScheduleDayType::WinterDesignDay]),
        "customday1" => Some(vec![ScheduleDayType::CustomDay1]),
        "customday2" => Some(vec![ScheduleDayType::CustomDay2]),
        _ => None,
    }
}

fn schedule_day_type_index(day_type: ScheduleDayType) -> usize {
    match day_type {
        ScheduleDayType::Sunday => 0,
        ScheduleDayType::Monday => 1,
        ScheduleDayType::Tuesday => 2,
        ScheduleDayType::Wednesday => 3,
        ScheduleDayType::Thursday => 4,
        ScheduleDayType::Friday => 5,
        ScheduleDayType::Saturday => 6,
        ScheduleDayType::Holiday => 7,
        ScheduleDayType::SummerDesignDay => 8,
        ScheduleDayType::WinterDesignDay => 9,
        ScheduleDayType::CustomDay1 => 10,
        ScheduleDayType::CustomDay2 => 11,
    }
}

fn schedule_day_type_name(day_type: ScheduleDayType) -> &'static str {
    match day_type {
        ScheduleDayType::Sunday => "Sunday",
        ScheduleDayType::Monday => "Monday",
        ScheduleDayType::Tuesday => "Tuesday",
        ScheduleDayType::Wednesday => "Wednesday",
        ScheduleDayType::Thursday => "Thursday",
        ScheduleDayType::Friday => "Friday",
        ScheduleDayType::Saturday => "Saturday",
        ScheduleDayType::Holiday => "Holiday",
        ScheduleDayType::SummerDesignDay => "SummerDesignDay",
        ScheduleDayType::WinterDesignDay => "WinterDesignDay",
        ScheduleDayType::CustomDay1 => "CustomDay1",
        ScheduleDayType::CustomDay2 => "CustomDay2",
    }
}

fn parse_compact_through_ordinal(value: &str) -> Option<u16> {
    let (_directive, date) = value.split_once(':')?;
    let ep_model::CalendarDateRule::MonthDay {
        month,
        day_of_month,
    } = parse_calendar_date_rule(date.trim())?
    else {
        return None;
    };
    let days_before_month = [0_u16, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335];
    let month_index = usize::try_from(month.checked_sub(1)?).ok()?;
    days_before_month
        .get(month_index)?
        .checked_add(u16::try_from(day_of_month).ok()?)
}

fn leap_schedule_ordinal(month: u32, day_of_month: u32) -> Option<usize> {
    const DAYS_IN_MONTH: [u32; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let month_index = usize::try_from(month.checked_sub(1)?).ok()?;
    let days_in_month = *DAYS_IN_MONTH.get(month_index)?;
    if day_of_month == 0 || day_of_month > days_in_month {
        return None;
    }
    let days_before_month = DAYS_IN_MONTH[..month_index]
        .iter()
        .try_fold(0_u32, |total, days| total.checked_add(*days))?;
    usize::try_from(days_before_month.checked_add(day_of_month)?).ok()
}

fn parse_schedule_time_minute(value: &str) -> Option<u32> {
    let value = value.trim();
    let time = if compact_directive(value, "Until") {
        let (_directive, time) = value.split_once(':')?;
        time.trim()
    } else {
        value
    };
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

fn parse_until_minute(value: &str) -> Option<u32> {
    compact_directive(value, "Until")
        .then(|| parse_schedule_time_minute(value))
        .flatten()
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

fn parse_inside_surface_convection_algorithm(
    value: &str,
) -> Option<InsideSurfaceConvectionAlgorithm> {
    match value {
        value if value.eq_ignore_ascii_case("Simple") => {
            Some(InsideSurfaceConvectionAlgorithm::Simple)
        }
        value if value.eq_ignore_ascii_case("TARP") => Some(InsideSurfaceConvectionAlgorithm::Tarp),
        value if value.eq_ignore_ascii_case("CeilingDiffuser") => {
            Some(InsideSurfaceConvectionAlgorithm::CeilingDiffuser)
        }
        value if value.eq_ignore_ascii_case("AdaptiveConvectionAlgorithm") => {
            Some(InsideSurfaceConvectionAlgorithm::AdaptiveConvectionAlgorithm)
        }
        value if value.eq_ignore_ascii_case("ASTMC1340") => {
            Some(InsideSurfaceConvectionAlgorithm::AstmC1340)
        }
        _ => None,
    }
}

fn parse_outside_surface_convection_algorithm(
    value: &str,
) -> Option<OutsideSurfaceConvectionAlgorithm> {
    match value {
        value if value.eq_ignore_ascii_case("SimpleCombined") => {
            Some(OutsideSurfaceConvectionAlgorithm::SimpleCombined)
        }
        value if value.eq_ignore_ascii_case("TARP") => {
            Some(OutsideSurfaceConvectionAlgorithm::Tarp)
        }
        value if value.eq_ignore_ascii_case("MoWiTT") => {
            Some(OutsideSurfaceConvectionAlgorithm::MoWitt)
        }
        value if value.eq_ignore_ascii_case("DOE-2") || value.eq_ignore_ascii_case("DOE2") => {
            Some(OutsideSurfaceConvectionAlgorithm::Doe2)
        }
        value if value.eq_ignore_ascii_case("AdaptiveConvectionAlgorithm") => {
            Some(OutsideSurfaceConvectionAlgorithm::AdaptiveConvectionAlgorithm)
        }
        _ => None,
    }
}

fn parse_other_equipment_design_level_calculation_method(
    value: &str,
) -> Option<OtherEquipmentDesignLevelCalculationMethod> {
    match value {
        value if value.eq_ignore_ascii_case("EquipmentLevel") => {
            Some(OtherEquipmentDesignLevelCalculationMethod::EquipmentLevel)
        }
        value
            if value.eq_ignore_ascii_case("Watts/Area")
                || value.eq_ignore_ascii_case("Power/Area")
                || value.eq_ignore_ascii_case("WattsPerZoneFloorArea") =>
        {
            Some(OtherEquipmentDesignLevelCalculationMethod::WattsPerZoneFloorArea)
        }
        value
            if value.eq_ignore_ascii_case("Watts/Person")
                || value.eq_ignore_ascii_case("Power/Person")
                || value.eq_ignore_ascii_case("WattsPerPerson") =>
        {
            Some(OtherEquipmentDesignLevelCalculationMethod::WattsPerPerson)
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

fn parse_people_number_calculation_method(value: &str) -> Option<PeopleNumberCalculationMethod> {
    match value {
        value if value.eq_ignore_ascii_case("People") => {
            Some(PeopleNumberCalculationMethod::People)
        }
        value
            if value.eq_ignore_ascii_case("People/Area")
                || value.eq_ignore_ascii_case("PeoplePerArea") =>
        {
            Some(PeopleNumberCalculationMethod::PeoplePerArea)
        }
        value
            if value.eq_ignore_ascii_case("Area/Person")
                || value.eq_ignore_ascii_case("AreaPerPerson") =>
        {
            Some(PeopleNumberCalculationMethod::AreaPerPerson)
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

fn parse_design_specification_outdoor_air_method(
    value: &str,
) -> Option<DesignSpecificationOutdoorAirMethod> {
    match value {
        value if value.eq_ignore_ascii_case("Flow/Person") => {
            Some(DesignSpecificationOutdoorAirMethod::FlowPerPerson)
        }
        value if value.eq_ignore_ascii_case("Flow/Area") => {
            Some(DesignSpecificationOutdoorAirMethod::FlowPerArea)
        }
        value if value.eq_ignore_ascii_case("Flow/Zone") => {
            Some(DesignSpecificationOutdoorAirMethod::FlowPerZone)
        }
        value if value.eq_ignore_ascii_case("AirChanges/Hour") => {
            Some(DesignSpecificationOutdoorAirMethod::AirChangesPerHour)
        }
        value if value.eq_ignore_ascii_case("Sum") => {
            Some(DesignSpecificationOutdoorAirMethod::Sum)
        }
        value if value.eq_ignore_ascii_case("Maximum") => {
            Some(DesignSpecificationOutdoorAirMethod::Maximum)
        }
        value if value.eq_ignore_ascii_case("IndoorAirQualityProcedure") => {
            Some(DesignSpecificationOutdoorAirMethod::IndoorAirQualityProcedure)
        }
        value if value.eq_ignore_ascii_case("ProportionalControlBasedOnDesignOccupancy") => {
            Some(DesignSpecificationOutdoorAirMethod::ProportionalControlBasedOnDesignOccupancy)
        }
        value if value.eq_ignore_ascii_case("ProportionalControlBasedOnOccupancySchedule") => {
            Some(DesignSpecificationOutdoorAirMethod::ProportionalControlBasedOnOccupancySchedule)
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

fn parse_plant_connector_kind(value: &str) -> Option<PlantConnectorKind> {
    match value {
        value if value.eq_ignore_ascii_case("Connector:Splitter") => {
            Some(PlantConnectorKind::Splitter)
        }
        value if value.eq_ignore_ascii_case("Connector:Mixer") => Some(PlantConnectorKind::Mixer),
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

fn parse_special_day_type(value: &str) -> Option<SpecialDayType> {
    match value {
        value if value.eq_ignore_ascii_case("Holiday") => Some(SpecialDayType::Holiday),
        value if value.eq_ignore_ascii_case("SummerDesignDay") => {
            Some(SpecialDayType::SummerDesignDay)
        }
        value if value.eq_ignore_ascii_case("WinterDesignDay") => {
            Some(SpecialDayType::WinterDesignDay)
        }
        value if value.eq_ignore_ascii_case("CustomDay1") => Some(SpecialDayType::CustomDay1),
        value if value.eq_ignore_ascii_case("CustomDay2") => Some(SpecialDayType::CustomDay2),
        _ => None,
    }
}

fn parse_first_hour_interpolation_starting_values(
    value: &str,
) -> Option<FirstHourInterpolationStartingValues> {
    match value {
        value if value.eq_ignore_ascii_case("Hour1") => {
            Some(FirstHourInterpolationStartingValues::Hour1)
        }
        value if value.eq_ignore_ascii_case("Hour24") => {
            Some(FirstHourInterpolationStartingValues::Hour24)
        }
        _ => None,
    }
}

fn parse_yes_no(value: &str) -> Option<bool> {
    match value {
        value if value.eq_ignore_ascii_case("Yes") => Some(true),
        value if value.eq_ignore_ascii_case("No") => Some(false),
        _ => None,
    }
}

fn canonical_world_surface_vertices(
    mut vertices: Vec<Point3>,
    rules: GlobalGeometryRules,
    zone_relative_north_deg: f64,
    zone_origin: Point3,
    building_north_axis_deg: f64,
) -> Vec<Point3> {
    if rules.vertex_entry_direction == VertexEntryDirection::Clockwise && vertices.len() > 1 {
        // EnergyPlus GetVertices preserves vertex 1 and reverses vertices 2 through N.
        vertices[1..].reverse();
    }

    // Preserve the source loop semantics rather than reducing this to a four-vertex rotation.
    let vertex_count = vertices.len();
    if vertex_count >= 3 {
        let mut this_corner = match rules.starting_vertex_position {
            StartingVertexPosition::UpperLeftCorner => 1,
            StartingVertexPosition::LowerLeftCorner => 2,
            StartingVertexPosition::LowerRightCorner => 3,
            StartingVertexPosition::UpperRightCorner => 4,
        };
        while this_corner != 1 {
            if vertex_count < 4 && this_corner == 4 {
                break;
            }
            let mut target = this_corner;
            let mut source = this_corner + 1;
            if source > vertex_count {
                source = 1;
            }
            for _ in 0..vertex_count - 1 {
                vertices.swap(target - 1, source - 1);
                target += 1;
                source += 1;
                if target > vertex_count {
                    target = 1;
                }
                if source > vertex_count {
                    source = 1;
                }
            }
            this_corner += 1;
            if this_corner > vertex_count {
                this_corner = 1;
            }
        }
    }

    if rules.coordinate_system == GeometryCoordinateSystem::Relative {
        let zone_angle_rad = (-zone_relative_north_deg).to_radians();
        let zone_cos = zone_angle_rad.cos();
        let zone_sin = zone_angle_rad.sin();
        let building_angle_rad = (-building_north_axis_deg).to_radians();
        let building_cos = building_angle_rad.cos();
        let building_sin = building_angle_rad.sin();

        for vertex in &mut vertices {
            let building_relative_x =
                vertex.x_m * zone_cos - vertex.y_m * zone_sin + zone_origin.x_m;
            let building_relative_y =
                vertex.x_m * zone_sin + vertex.y_m * zone_cos + zone_origin.y_m;
            vertex.x_m = building_relative_x * building_cos - building_relative_y * building_sin;
            vertex.y_m = building_relative_x * building_sin + building_relative_y * building_cos;
            vertex.z_m += zone_origin.z_m;
        }
    }

    vertices
}

fn parse_starting_vertex_position(value: &str) -> Option<StartingVertexPosition> {
    let value = value.trim();
    match value {
        value if value.eq_ignore_ascii_case("UpperLeftCorner") => {
            Some(StartingVertexPosition::UpperLeftCorner)
        }
        value if value.eq_ignore_ascii_case("LowerLeftCorner") => {
            Some(StartingVertexPosition::LowerLeftCorner)
        }
        value if value.eq_ignore_ascii_case("UpperRightCorner") => {
            Some(StartingVertexPosition::UpperRightCorner)
        }
        value if value.eq_ignore_ascii_case("LowerRightCorner") => {
            Some(StartingVertexPosition::LowerRightCorner)
        }
        _ => None,
    }
}

fn parse_vertex_entry_direction(value: &str) -> Option<VertexEntryDirection> {
    let value = value.trim();
    match value {
        value
            if value.eq_ignore_ascii_case("CCW")
                || value.eq_ignore_ascii_case("Counterclockwise") =>
        {
            Some(VertexEntryDirection::CounterClockwise)
        }
        value if value.eq_ignore_ascii_case("CW") || value.eq_ignore_ascii_case("Clockwise") => {
            Some(VertexEntryDirection::Clockwise)
        }
        _ => None,
    }
}

fn parse_geometry_coordinate_system(value: &str) -> Option<GeometryCoordinateSystem> {
    let value = value.trim();
    match value {
        value if value.eq_ignore_ascii_case("Relative") => Some(GeometryCoordinateSystem::Relative),
        value if value.eq_ignore_ascii_case("World") || value.eq_ignore_ascii_case("Absolute") => {
            Some(GeometryCoordinateSystem::World)
        }
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
    mod global_geometry_rules;
    mod material_variants;
    mod schedule_day_interval;
    mod schedule_day_list;
    mod schedule_external_interface;
    mod schedule_external_interface_fmu_export;
    mod schedule_external_interface_fmu_import;
    mod schedule_file;
    mod schedule_file_shading;
    mod schedule_scalar_type_limits;
    mod schedule_week_compact;
    mod schedule_year;
    mod window_material_gas;
    mod window_material_glazing;
    mod window_material_glazing_equivalent_layer;
    mod window_material_glazing_refraction_extinction;

    use super::{
        ALL_SCHEDULE_DAY_TYPES, CompileStage, DiagnosticSeverity, ObjectCoverageStatus,
        compile_raw_model,
    };
    use ep_model::{
        AutosizeOrNumber, CalendarDateRule, DayOfWeek, DehumidificationControlType,
        DesignSpecificationOutdoorAirMethod, FirstHourInterpolationStartingValues,
        HumidificationControlType, IdealLoadsLimit, InsideSurfaceConvectionAlgorithm,
        LoadDistributionScheme, MaterialSurfaceRoughness, ModelGraph,
        OtherEquipmentDesignLevelCalculationMethod, OutdoorAirEconomizerType,
        OutsideSurfaceConvectionAlgorithm, PeopleNumberCalculationMethod, PlantConnectorKind,
        ScheduleDayType, ScheduleInterpolation, SpecialDayType,
    };
    use ep_raw_model::{parse_epjson_str, parse_epjson_str_with_idf_order};

    #[test]
    fn compile_report_records_typed_and_reference_stages() -> Result<(), Box<dyn std::error::Error>>
    {
        let raw_model = parse_epjson_str(
            r#"{
                "Version": {"Version 1": {"version_identifier": "26.1"}},
                "Timestep": {"Timestep 1": {}},
                "Material:NoMass": {
                    "R13": {"roughness": "Rough", "thermal_resistance": 1.0},
                    "Finish": {"roughness": "Rough", "thermal_resistance": 0.1}
                },
                "Construction": {"Wall Construction": {"outside_layer": "R13", "layer_2": "Finish"}},
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
        assert_eq!(model.constructions[0].layers.len(), 2);
        let layer_names = model.constructions[0]
            .layers
            .iter()
            .map(|layer_id| {
                model
                    .materials
                    .iter()
                    .find(|material| material.id == *layer_id)
                    .map(|material| material.name.0.as_str())
                    .unwrap_or("")
            })
            .collect::<Vec<_>>();
        assert!(layer_names[0].eq_ignore_ascii_case("R13"));
        assert!(layer_names[1].eq_ignore_ascii_case("Finish"));
        let graph = ModelGraph::from_typed(&model);
        assert_eq!(graph.construction_materials.len(), 2);
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
    fn parses_surface_convection_algorithm_singletons() -> Result<(), Box<dyn std::error::Error>> {
        let raw_model = parse_epjson_str(
            r#"{
                "SurfaceConvectionAlgorithm:Inside": {
                    "SurfaceConvectionAlgorithm:Inside 1": {"algorithm": "TARP"}
                },
                "SurfaceConvectionAlgorithm:Outside": {
                    "SurfaceConvectionAlgorithm:Outside 1": {"algorithm": "DOE-2"}
                }
            }"#,
        )?;

        let result = compile_raw_model(&raw_model);

        assert!(!result.has_errors());
        assert_eq!(result.report.typed_object_count, 3);
        let Some(model) = result.model else {
            return Err(std::io::Error::other("expected typed model").into());
        };
        assert_eq!(
            model.surface_convection_algorithms.inside,
            Some(InsideSurfaceConvectionAlgorithm::Tarp)
        );
        assert_eq!(
            model.surface_convection_algorithms.outside,
            Some(OutsideSurfaceConvectionAlgorithm::Doe2)
        );
        let outside = result
            .report
            .coverage
            .iter()
            .find(|entry| entry.object_type == "SurfaceConvectionAlgorithm:Outside")
            .ok_or_else(|| std::io::Error::other("missing outside algorithm coverage"))?;
        assert_eq!(outside.status, ObjectCoverageStatus::Typed);

        Ok(())
    }

    #[test]
    fn parses_material_properties_and_other_equipment() -> Result<(), Box<dyn std::error::Error>> {
        let raw_model = parse_epjson_str(
            r#"{
                "Material": {
                    "Concrete": {
                        "roughness": "MediumRough",
                        "conductivity": 2.0,
                        "density": 2000.0,
                        "specific_heat": 800.0,
                        "thickness": 0.1
                    }
                },
                "Material:NoMass": {
                    "R13": {
                        "roughness": "Rough",
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
                        "fuel_type": "Electricity",
                        "zone_or_zonelist_or_space_or_spacelist_name": "zone one",
                        "schedule_name": "always on",
                        "design_level_calculation_method": "Power/Area",
                        "design_level": 125.0,
                        "power_per_floor_area": 7.5,
                        "power_per_person": 25.0,
                        "fraction_latent": 0.1,
                        "fraction_radiant": 0.2,
                        "fraction_lost": 0.3,
                        "carbon_dioxide_generation_rate": 0.0000001
                    }
                },
                "People": {
                    "Occupants": {
                        "zone_or_zonelist_or_space_or_spacelist_name": "zone one",
                        "number_of_people_schedule_name": "always on",
                        "number_of_people_calculation_method": "People",
                        "number_of_people": 5.0
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
        assert_eq!(
            model.materials[0].roughness(),
            Some(MaterialSurfaceRoughness::MediumRough)
        );
        assert_eq!(model.materials[0].thermal_resistance(), Some(0.05));
        assert_eq!(model.materials[0].heat_capacity_per_area(), Some(160_000.0));
        assert_eq!(
            model.materials[1].roughness(),
            Some(MaterialSurfaceRoughness::Rough)
        );
        assert_eq!(model.materials[1].thermal_resistance(), Some(2.29));
        assert_eq!(model.other_equipment.len(), 1);
        assert_eq!(model.other_equipment[0].fuel_type.0, "ELECTRICITY");
        assert_eq!(model.other_equipment[0].zone.0, 0);
        assert_eq!(model.other_equipment[0].schedule.map(|id| id.0), Some(0));
        assert_eq!(
            model.other_equipment[0].design_level_calculation_method,
            OtherEquipmentDesignLevelCalculationMethod::WattsPerZoneFloorArea
        );
        assert_eq!(model.other_equipment[0].design_level_w, 125.0);
        assert_eq!(model.other_equipment[0].power_per_floor_area_w_per_m2, 7.5);
        assert_eq!(model.other_equipment[0].power_per_person_w, 25.0);
        assert_eq!(model.other_equipment[0].fraction_latent, 0.1);
        assert_eq!(model.other_equipment[0].fraction_radiant, 0.2);
        assert_eq!(model.other_equipment[0].fraction_lost, 0.3);
        assert_eq!(
            model.other_equipment[0].carbon_dioxide_generation_rate_m3_per_s_w,
            0.0000001
        );
        assert_eq!(model.people.len(), 1);
        assert_eq!(model.people[0].zone.0, 0);
        assert_eq!(
            model.people[0].number_of_people_schedule.map(|id| id.0),
            Some(0)
        );
        assert_eq!(
            model.people[0].number_of_people_calculation_method,
            PeopleNumberCalculationMethod::People
        );
        assert_eq!(model.people[0].number_of_people, 5.0);

        Ok(())
    }

    #[test]
    fn other_equipment_fraction_sum_above_one_is_error() -> Result<(), Box<dyn std::error::Error>> {
        let raw_model = parse_epjson_str(
            r#"{
                "Version": {"Version 1": {"version_identifier": "26.1"}},
                "Zone": {"Zone One": {}},
                "OtherEquipment": {
                    "Bad Fractions": {
                        "zone_or_zonelist_or_space_or_spacelist_name": "zone one",
                        "design_level": 100.0,
                        "fraction_latent": 0.4,
                        "fraction_radiant": 0.4,
                        "fraction_lost": 0.3
                    }
                }
            }"#,
        )?;

        let result = compile_raw_model(&raw_model);

        assert!(result.has_errors());
        assert!(result.report.diagnostics.iter().any(|diagnostic| {
            diagnostic.severity == DiagnosticSeverity::Error
                && diagnostic.code == "InvalidOtherEquipmentFractionSum"
                && diagnostic.object_type == "OtherEquipment"
                && diagnostic.object_name.as_deref() == Some("Bad Fractions")
                && diagnostic.field.as_deref()
                    == Some("fraction_latent+fraction_radiant+fraction_lost")
        }));

        Ok(())
    }

    #[test]
    fn parses_run_period_dates_and_policies() -> Result<(), Box<dyn std::error::Error>> {
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
                        "day_of_week_for_start_day": "Wednesday",
                        "first_hour_interpolation_starting_values": "Hour1",
                        "use_weather_file_holidays_and_special_days": "No",
                        "use_weather_file_daylight_saving_period": "No",
                        "apply_weekend_holiday_rule": "No",
                        "use_weather_file_rain_indicators": "No",
                        "use_weather_file_snow_indicators": "No",
                        "treat_weather_as_actual": "Yes"
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
        assert_eq!(
            model.run_periods[0].first_hour_interpolation_starting_values,
            FirstHourInterpolationStartingValues::Hour1
        );
        assert!(!model.run_periods[0].use_weather_file_holidays_and_special_days);
        assert!(!model.run_periods[0].use_weather_file_daylight_saving_period);
        assert!(!model.run_periods[0].apply_weekend_holiday_rule);
        assert!(!model.run_periods[0].use_weather_file_rain_indicators);
        assert!(!model.run_periods[0].use_weather_file_snow_indicators);
        assert!(model.run_periods[0].treat_weather_as_actual);

        Ok(())
    }

    #[test]
    fn defaults_run_period_policies_to_energyplus_values() -> Result<(), Box<dyn std::error::Error>>
    {
        let raw_model = parse_epjson_str(
            r#"{
                "RunPeriod": {
                    "Run Period 1": {
                        "apply_weekend_holiday_rule": ""
                    }
                }
            }"#,
        )?;

        let result = compile_raw_model(&raw_model);

        assert!(!result.has_errors());
        let Some(model) = result.model else {
            return Err(std::io::Error::other("expected typed model").into());
        };
        let run_period = &model.run_periods[0];
        assert!(run_period.use_weather_file_holidays_and_special_days);
        assert!(run_period.use_weather_file_daylight_saving_period);
        assert!(run_period.apply_weekend_holiday_rule);
        assert!(run_period.use_weather_file_rain_indicators);
        assert!(run_period.use_weather_file_snow_indicators);
        assert!(!run_period.treat_weather_as_actual);

        for (field, value) in [
            ("use_weather_file_holidays_and_special_days", "Yes"),
            ("use_weather_file_daylight_saving_period", "Yes"),
            ("apply_weekend_holiday_rule", "Yes"),
            ("use_weather_file_rain_indicators", "Yes"),
            ("use_weather_file_snow_indicators", "Yes"),
            ("treat_weather_as_actual", "No"),
        ] {
            assert!(result.report.defaults_applied.iter().any(|application| {
                application.object_type == "RunPeriod"
                    && application.object_name == "Run Period 1"
                    && application.field == field
                    && application.value == value
            }));
        }

        Ok(())
    }

    #[test]
    fn rejects_non_yes_no_run_period_policies() -> Result<(), Box<dyn std::error::Error>> {
        let raw_model = parse_epjson_str(
            r#"{
                "RunPeriod": {
                    "Run Period 1": {
                        "use_weather_file_holidays_and_special_days": "Sometimes",
                        "use_weather_file_daylight_saving_period": "Sometimes",
                        "apply_weekend_holiday_rule": "Sometimes",
                        "use_weather_file_rain_indicators": "Sometimes",
                        "use_weather_file_snow_indicators": "Sometimes",
                        "treat_weather_as_actual": "Sometimes"
                    }
                }
            }"#,
        )?;

        let result = compile_raw_model(&raw_model);

        assert!(result.has_errors());
        assert!(result.model.is_none());
        for field in [
            "use_weather_file_holidays_and_special_days",
            "use_weather_file_daylight_saving_period",
            "apply_weekend_holiday_rule",
            "use_weather_file_rain_indicators",
            "use_weather_file_snow_indicators",
            "treat_weather_as_actual",
        ] {
            assert!(result.report.diagnostics.iter().any(|diagnostic| {
                diagnostic.severity == DiagnosticSeverity::Error
                    && diagnostic.code == "InvalidEnumValue"
                    && diagnostic.object_type == "RunPeriod"
                    && diagnostic.object_name.as_deref() == Some("Run Period 1")
                    && diagnostic.field.as_deref() == Some(field)
            }));
        }

        Ok(())
    }

    #[test]
    fn parses_typed_run_period_daylight_saving_time_and_coverage()
    -> Result<(), Box<dyn std::error::Error>> {
        let raw_model = parse_epjson_str(
            r#"{
                "RunPeriodControl:DaylightSavingTime": {
                    "Daylight Saving Time 1": {
                        "start_date": "2/28",
                        "end_date": "Last Sunday in October"
                    }
                }
            }"#,
        )?;

        let result = compile_raw_model(&raw_model);

        assert!(!result.has_errors());
        assert_eq!(result.report.typed_object_count, 2);
        let coverage = result
            .report
            .coverage
            .iter()
            .find(|entry| entry.object_type == "RunPeriodControl:DaylightSavingTime")
            .ok_or_else(|| std::io::Error::other("missing daylight-saving coverage"))?;
        assert_eq!(coverage.object_count, 1);
        assert_eq!(coverage.status, ObjectCoverageStatus::Typed);

        let Some(model) = result.model else {
            return Err(std::io::Error::other("expected typed model").into());
        };
        assert_eq!(model.object_count(), 2);
        let daylight_saving = model
            .run_period_daylight_saving_time
            .ok_or_else(|| std::io::Error::other("missing typed daylight-saving period"))?;
        assert_eq!(
            daylight_saving.start_date,
            CalendarDateRule::MonthDay {
                month: 2,
                day_of_month: 28
            }
        );
        assert_eq!(
            daylight_saving.end_date,
            CalendarDateRule::LastWeekdayInMonth {
                weekday: DayOfWeek::Sunday,
                month: 10
            }
        );

        Ok(())
    }

    #[test]
    fn diagnoses_each_invalid_run_period_daylight_saving_time_field()
    -> Result<(), Box<dyn std::error::Error>> {
        let raw_model = parse_epjson_str(
            r#"{
                "RunPeriodControl:DaylightSavingTime": {
                    "Bad Daylight Saving Time": {
                        "start_date": "not a calendar date"
                    }
                }
            }"#,
        )?;

        let result = compile_raw_model(&raw_model);

        assert!(result.has_errors());
        assert!(result.model.is_none());
        for (code, field) in [
            ("InvalidCalendarDateRule", "start_date"),
            ("MissingRequiredField", "end_date"),
        ] {
            assert!(result.report.diagnostics.iter().any(|diagnostic| {
                diagnostic.severity == DiagnosticSeverity::Error
                    && diagnostic.code == code
                    && diagnostic.object_type == "RunPeriodControl:DaylightSavingTime"
                    && diagnostic.object_name.as_deref() == Some("Bad Daylight Saving Time")
                    && diagnostic.field.as_deref() == Some(field)
            }));
        }

        Ok(())
    }

    #[test]
    fn rejects_duplicate_run_period_daylight_saving_time_objects()
    -> Result<(), Box<dyn std::error::Error>> {
        let raw_model = parse_epjson_str(
            r#"{
                "RunPeriodControl:DaylightSavingTime": {
                    "Daylight Saving Time 1": {
                        "start_date": "2/28",
                        "end_date": "2/29"
                    },
                    "Daylight Saving Time 2": {
                        "start_date": "3/1",
                        "end_date": "3/2"
                    }
                }
            }"#,
        )?;

        let result = compile_raw_model(&raw_model);

        assert!(result.has_errors());
        assert!(result.model.is_none());
        assert!(result.report.diagnostics.iter().any(|diagnostic| {
            diagnostic.severity == DiagnosticSeverity::Error
                && diagnostic.code == "TooManyObjects"
                && diagnostic.object_type == "RunPeriodControl:DaylightSavingTime"
                && diagnostic.object_name.is_none()
                && diagnostic.field.is_none()
        }));

        Ok(())
    }

    #[test]
    fn parses_typed_run_period_special_day_rules_types_and_coverage()
    -> Result<(), Box<dyn std::error::Error>> {
        let raw_model = parse_epjson_str(
            r#"{
                "RunPeriodControl:SpecialDays": {
                    "A Fixed Holiday": {
                        "start_date": "2/29",
                        "duration": 2,
                        "special_day_type": "Holiday"
                    },
                    "B Nth Summer": {
                        "start_date": "2nd Sunday in March",
                        "duration": 1,
                        "special_day_type": "SummerDesignDay"
                    },
                    "C Last Winter": {
                        "start_date": "Last Monday in May",
                        "duration": 3,
                        "special_day_type": "WinterDesignDay"
                    },
                    "D Custom One": {
                        "start_date": "July 4",
                        "duration": 1,
                        "special_day_type": "CustomDay1"
                    },
                    "E Custom Two": {
                        "start_date": "5 November",
                        "duration": 1,
                        "special_day_type": "CustomDay2"
                    }
                }
            }"#,
        )?;

        let result = compile_raw_model(&raw_model);

        assert!(!result.has_errors());
        assert_eq!(result.report.typed_object_count, 6);
        let coverage = result
            .report
            .coverage
            .iter()
            .find(|entry| entry.object_type == "RunPeriodControl:SpecialDays")
            .ok_or_else(|| std::io::Error::other("missing special-day coverage"))?;
        assert_eq!(coverage.object_count, 5);
        assert_eq!(coverage.status, ObjectCoverageStatus::Typed);

        let Some(model) = result.model else {
            return Err(std::io::Error::other("expected typed model").into());
        };
        assert_eq!(model.run_period_special_days.len(), 5);
        assert_eq!(model.run_period_special_day_names.len(), 5);
        assert_eq!(model.object_count(), 6);

        let fixed = &model.run_period_special_days[0];
        assert_eq!(
            fixed.start_date,
            CalendarDateRule::MonthDay {
                month: 2,
                day_of_month: 29
            }
        );
        assert_eq!(fixed.duration_days, 2);
        assert_eq!(fixed.special_day_type, SpecialDayType::Holiday);
        assert_eq!(
            model
                .run_period_special_day_names
                .resolve("a fixed holiday"),
            Some(fixed.id)
        );

        assert_eq!(
            model.run_period_special_days[1].start_date,
            CalendarDateRule::NthWeekdayInMonth {
                nth: 2,
                weekday: DayOfWeek::Sunday,
                month: 3
            }
        );
        assert_eq!(
            model.run_period_special_days[1].special_day_type,
            SpecialDayType::SummerDesignDay
        );
        assert_eq!(
            model.run_period_special_days[2].start_date,
            CalendarDateRule::LastWeekdayInMonth {
                weekday: DayOfWeek::Monday,
                month: 5
            }
        );
        assert_eq!(model.run_period_special_days[2].duration_days, 3);
        assert_eq!(
            model.run_period_special_days[2].special_day_type,
            SpecialDayType::WinterDesignDay
        );
        assert_eq!(
            model.run_period_special_days[3].special_day_type,
            SpecialDayType::CustomDay1
        );
        assert_eq!(
            model.run_period_special_days[4].special_day_type,
            SpecialDayType::CustomDay2
        );

        Ok(())
    }

    #[test]
    fn idf_overlay_orders_special_days_while_native_epjson_stays_name_sorted()
    -> Result<(), Box<dyn std::error::Error>> {
        let epjson = r#"{
            "RunPeriodControl:SpecialDays": {
                "Zulu Earlier Holiday": {
                    "start_date": "6/15",
                    "duration": 1,
                    "special_day_type": "Holiday"
                },
                "Alpha Later Custom": {
                    "start_date": "6/15",
                    "duration": 1,
                    "special_day_type": "CustomDay2"
                }
            }
        }"#;
        let idf = r#"
            RunPeriodControl:SpecialDays,
              Zulu Earlier Holiday,
              6/15,
              1,
              Holiday;
            RunPeriodControl:SpecialDays,
              Alpha Later Custom,
              6/15,
              1,
              CustomDay2;
        "#;

        let idf_raw_model = parse_epjson_str_with_idf_order(epjson, idf)?;
        let idf_result = compile_raw_model(&idf_raw_model);
        assert!(!idf_result.has_errors());
        let Some(idf_model) = idf_result.model else {
            return Err(std::io::Error::other("expected IDF-overlay typed model").into());
        };
        assert_eq!(idf_model.run_period_special_days.len(), 2);
        assert_eq!(
            idf_model.run_period_special_days[0].name.0,
            "ZULU EARLIER HOLIDAY"
        );
        assert_eq!(idf_model.run_period_special_days[0].id.0, 0);
        assert_eq!(
            idf_model.run_period_special_days[1].name.0,
            "ALPHA LATER CUSTOM"
        );
        assert_eq!(idf_model.run_period_special_days[1].id.0, 1);

        let epjson_raw_model = parse_epjson_str(epjson)?;
        let epjson_result = compile_raw_model(&epjson_raw_model);
        assert!(!epjson_result.has_errors());
        let Some(epjson_model) = epjson_result.model else {
            return Err(std::io::Error::other("expected native epJSON typed model").into());
        };
        assert_eq!(
            epjson_model.run_period_special_days[0].name.0,
            "ALPHA LATER CUSTOM"
        );
        assert_eq!(epjson_model.run_period_special_days[0].id.0, 0);
        assert_eq!(
            epjson_model.run_period_special_days[1].name.0,
            "ZULU EARLIER HOLIDAY"
        );
        assert_eq!(epjson_model.run_period_special_days[1].id.0, 1);
        Ok(())
    }

    #[test]
    fn defaults_run_period_special_day_duration_and_type() -> Result<(), Box<dyn std::error::Error>>
    {
        let raw_model = parse_epjson_str(
            r#"{
                "RunPeriodControl:SpecialDays": {
                    "Default Holiday": {
                        "start_date": "January 1",
                        "special_day_type": ""
                    }
                }
            }"#,
        )?;

        let result = compile_raw_model(&raw_model);

        assert!(!result.has_errors());
        let Some(model) = result.model else {
            return Err(std::io::Error::other("expected typed model").into());
        };
        assert_eq!(model.run_period_special_days.len(), 1);
        assert_eq!(model.run_period_special_days[0].duration_days, 1);
        assert_eq!(
            model.run_period_special_days[0].special_day_type,
            SpecialDayType::Holiday
        );
        for (field, value) in [("duration", "1"), ("special_day_type", "Holiday")] {
            assert!(result.report.defaults_applied.iter().any(|application| {
                application.object_type == "RunPeriodControl:SpecialDays"
                    && application.object_name == "Default Holiday"
                    && application.field == field
                    && application.value == value
            }));
        }

        Ok(())
    }

    #[test]
    fn rejects_invalid_run_period_special_day_inputs() -> Result<(), Box<dyn std::error::Error>> {
        let raw_model = parse_epjson_str(
            r#"{
                "RunPeriodControl:SpecialDays": {
                    "A Zero Duration": {
                        "start_date": "January 1",
                        "duration": 0,
                        "special_day_type": "Holiday"
                    },
                    "B Long Duration": {
                        "start_date": "January 2",
                        "duration": 367,
                        "special_day_type": "Holiday"
                    },
                    "C Bad Start": {
                        "start_date": "Not A Calendar Date",
                        "duration": 1,
                        "special_day_type": "Holiday"
                    },
                    "D Bad Type": {
                        "start_date": "January 4",
                        "duration": 1,
                        "special_day_type": "Festival"
                    }
                }
            }"#,
        )?;

        let result = compile_raw_model(&raw_model);

        assert!(result.has_errors());
        assert!(result.model.is_none());
        for object_name in ["A Zero Duration", "B Long Duration"] {
            assert!(result.report.diagnostics.iter().any(|diagnostic| {
                diagnostic.severity == DiagnosticSeverity::Error
                    && diagnostic.code == "InvalidNumericRange"
                    && diagnostic.object_type == "RunPeriodControl:SpecialDays"
                    && diagnostic.object_name.as_deref() == Some(object_name)
                    && diagnostic.field.as_deref() == Some("duration")
            }));
        }
        assert!(result.report.diagnostics.iter().any(|diagnostic| {
            diagnostic.severity == DiagnosticSeverity::Error
                && diagnostic.code == "InvalidCalendarDateRule"
                && diagnostic.object_type == "RunPeriodControl:SpecialDays"
                && diagnostic.object_name.as_deref() == Some("C Bad Start")
                && diagnostic.field.as_deref() == Some("start_date")
        }));
        assert!(result.report.diagnostics.iter().any(|diagnostic| {
            diagnostic.severity == DiagnosticSeverity::Error
                && diagnostic.code == "InvalidEnumValue"
                && diagnostic.object_type == "RunPeriodControl:SpecialDays"
                && diagnostic.object_name.as_deref() == Some("D Bad Type")
                && diagnostic.field.as_deref() == Some("special_day_type")
        }));

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
        assert_eq!(model.compact_schedules[0].periods.len(), 1);
        assert_eq!(
            model.compact_schedules[0].periods[0].through_schedule_day_of_year,
            366
        );
        assert_eq!(
            model.compact_schedules[0].periods[0].day_profiles[0]
                .day_types
                .len(),
            12
        );
        assert_eq!(
            model.compact_schedules[0].periods[0].day_profiles[0].interpolation,
            ScheduleInterpolation::No
        );
        let segments = &model.compact_schedules[0].periods[0].day_profiles[0].segments;
        assert_eq!(segments.len(), 3);
        assert_eq!(segments[0].until_minute_of_day, 8 * 60);
        assert_eq!(segments[1].value, 1.0);

        Ok(())
    }

    #[test]
    fn parses_schedule_compact_periods_and_source_ordered_all_other_days()
    -> Result<(), Box<dyn std::error::Error>> {
        let raw_model = parse_epjson_str(
            r#"{
                "Schedule:Compact": {
                    "Calendar Lookup": {
                        "data": [
                            {"field": "Through: 1/1"},
                            {"field": "For: Thursday"},
                            {"field": "Until: 24:00"}, {"field": 105},
                            {"field": "For: AllOtherDays"},
                            {"field": "Until: 24:00"}, {"field": 199},
                            {"field": "Through: 12/31"},
                            {"field": "For: Tuesday"},
                            {"field": "Until: 24:00"}, {"field": 103},
                            {"field": "For: Wednesday"},
                            {"field": "Until: 24:00"}, {"field": 104},
                            {"field": "For: Holiday"},
                            {"field": "Until: 24:00"}, {"field": 108},
                            {"field": "For: AllOtherDays"},
                            {"field": "Until: 24:00"}, {"field": 199}
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
        let periods = &model.compact_schedules[0].periods;
        assert_eq!(
            periods
                .iter()
                .map(|period| period.through_schedule_day_of_year)
                .collect::<Vec<_>>(),
            vec![1, 366]
        );
        assert_eq!(periods[0].day_profiles.len(), 2);
        assert_eq!(
            periods[0].day_profiles[0].day_types,
            vec![ScheduleDayType::Thursday]
        );
        assert_eq!(periods[0].day_profiles[0].segments[0].value, 105.0);
        assert_eq!(periods[0].day_profiles[1].day_types.len(), 11);
        assert!(
            !periods[0].day_profiles[1]
                .day_types
                .contains(&ScheduleDayType::Thursday)
        );
        assert_eq!(periods[1].day_profiles.len(), 4);
        assert_eq!(
            periods[1].day_profiles[0].day_types,
            vec![ScheduleDayType::Tuesday]
        );
        assert_eq!(
            periods[1].day_profiles[1].day_types,
            vec![ScheduleDayType::Wednesday]
        );
        assert_eq!(
            periods[1].day_profiles[2].day_types,
            vec![ScheduleDayType::Holiday]
        );
        assert_eq!(
            periods[1].day_profiles[3].day_types,
            vec![
                ScheduleDayType::Sunday,
                ScheduleDayType::Monday,
                ScheduleDayType::Thursday,
                ScheduleDayType::Friday,
                ScheduleDayType::Saturday,
                ScheduleDayType::SummerDesignDay,
                ScheduleDayType::WinterDesignDay,
                ScheduleDayType::CustomDay1,
                ScheduleDayType::CustomDay2,
            ]
        );

        Ok(())
    }

    #[test]
    fn rejects_schedule_compact_duplicate_group_and_all_other_assignments()
    -> Result<(), Box<dyn std::error::Error>> {
        let raw_model = parse_epjson_str(
            r#"{
                "Schedule:Compact": {
                    "Group Explicit Duplicate": {
                        "data": [
                            {"field": "Through: 12/31"},
                            {"field": "For: Weekdays Monday AllOtherDays"},
                            {"field": "Until: 24:00"}, {"field": 1}
                        ]
                    },
                    "All Other Then Explicit": {
                        "data": [
                            {"field": "Through: 12/31"},
                            {"field": "For: AllOtherDays"},
                            {"field": "Until: 24:00"}, {"field": 1},
                            {"field": "For: Sunday"},
                            {"field": "Until: 24:00"}, {"field": 2}
                        ]
                    }
                }
            }"#,
        )?;

        let result = compile_raw_model(&raw_model);

        assert!(result.has_errors());
        for object_name in ["Group Explicit Duplicate", "All Other Then Explicit"] {
            assert!(result.report.diagnostics.iter().any(|diagnostic| {
                diagnostic.code == "DuplicateScheduleCompactDayType"
                    && diagnostic.object_name.as_deref() == Some(object_name)
            }));
        }

        Ok(())
    }

    #[test]
    fn schedule_compact_all_other_days_is_applied_after_same_field_selectors()
    -> Result<(), Box<dyn std::error::Error>> {
        let raw_model = parse_epjson_str(
            r#"{
                "Schedule:Compact": {
                    "All Other Before Explicit": {
                        "data": [
                            {"field": "Through: 12/31"},
                            {"field": "For: AllOtherDays Monday AllOtherDays"},
                            {"field": "Until: 24:00"}, {"field": 1}
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
        let day_types = &model.compact_schedules[0].periods[0].day_profiles[0].day_types;
        assert_eq!(day_types.len(), 12);
        assert_eq!(day_types[0], ScheduleDayType::Monday);
        for day_type in ALL_SCHEDULE_DAY_TYPES {
            assert!(day_types.contains(&day_type));
        }

        Ok(())
    }

    #[test]
    fn expands_schedule_compact_weekday_weekend_and_special_day_tokens()
    -> Result<(), Box<dyn std::error::Error>> {
        let raw_model = parse_epjson_str(
            r#"{
                "Schedule:Compact": {
                    "Every Token": {
                        "data": [
                            {"field": "Through: 12/31"},
                            {"field": "For: Weekends"},
                            {"field": "Until: 24:00"}, {"field": 1},
                            {"field": "For: Weekdays"},
                            {"field": "Until: 24:00"}, {"field": 2},
                            {"field": "For: Holiday SummerDesignDay WinterDesignDay CustomDay1 CustomDay2"},
                            {"field": "Until: 24:00"}, {"field": 3}
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
        let profiles = &model.compact_schedules[0].periods[0].day_profiles;
        assert_eq!(
            profiles[0].day_types,
            vec![ScheduleDayType::Saturday, ScheduleDayType::Sunday]
        );
        assert_eq!(
            profiles[1].day_types,
            vec![
                ScheduleDayType::Monday,
                ScheduleDayType::Tuesday,
                ScheduleDayType::Wednesday,
                ScheduleDayType::Thursday,
                ScheduleDayType::Friday,
            ]
        );
        assert_eq!(
            profiles[2].day_types,
            vec![
                ScheduleDayType::Holiday,
                ScheduleDayType::SummerDesignDay,
                ScheduleDayType::WinterDesignDay,
                ScheduleDayType::CustomDay1,
                ScheduleDayType::CustomDay2,
            ]
        );

        Ok(())
    }

    #[test]
    fn rejects_schedule_compact_through_order_and_missing_final_date()
    -> Result<(), Box<dyn std::error::Error>> {
        let raw_model = parse_epjson_str(
            r#"{
                "Schedule:Compact": {
                    "Duplicate Through": {
                        "data": [
                            {"field": "Through: 6/30"}, {"field": "For: AllDays"},
                            {"field": "Until: 24:00"}, {"field": 1},
                            {"field": "Through: 6/30"}, {"field": "For: AllDays"},
                            {"field": "Until: 24:00"}, {"field": 2},
                            {"field": "Through: 12/31"}, {"field": "For: AllDays"},
                            {"field": "Until: 24:00"}, {"field": 3}
                        ]
                    },
                    "Decreasing Through": {
                        "data": [
                            {"field": "Through: 6/30"}, {"field": "For: AllDays"},
                            {"field": "Until: 24:00"}, {"field": 1},
                            {"field": "Through: 5/31"}, {"field": "For: AllDays"},
                            {"field": "Until: 24:00"}, {"field": 2},
                            {"field": "Through: 12/31"}, {"field": "For: AllDays"},
                            {"field": "Until: 24:00"}, {"field": 3}
                        ]
                    },
                    "Missing Final Through": {
                        "data": [
                            {"field": "Through: 6/30"}, {"field": "For: AllDays"},
                            {"field": "Until: 24:00"}, {"field": 1}
                        ]
                    }
                }
            }"#,
        )?;

        let result = compile_raw_model(&raw_model);

        assert!(result.has_errors());
        for object_name in ["Duplicate Through", "Decreasing Through"] {
            assert!(result.report.diagnostics.iter().any(|diagnostic| {
                diagnostic.code == "InvalidScheduleCompactThroughOrder"
                    && diagnostic.object_name.as_deref() == Some(object_name)
            }));
        }
        assert!(result.report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "MissingScheduleCompactFinalThrough"
                && diagnostic.object_name.as_deref() == Some("Missing Final Through")
        }));

        Ok(())
    }

    #[test]
    fn rejects_schedule_compact_until_order_and_incomplete_profiles()
    -> Result<(), Box<dyn std::error::Error>> {
        let raw_model = parse_epjson_str(
            r#"{
                "Schedule:Compact": {
                    "Decreasing Until": {
                        "data": [
                            {"field": "Through: 12/31"}, {"field": "For: AllDays"},
                            {"field": "Until: 12:00"}, {"field": 1},
                            {"field": "Until: 08:00"}, {"field": 2},
                            {"field": "Until: 24:00"}, {"field": 3}
                        ]
                    },
                    "Incomplete Day": {
                        "data": [
                            {"field": "Through: 12/31"}, {"field": "For: AllDays"},
                            {"field": "Until: 23:00"}, {"field": 1}
                        ]
                    },
                    "Missing Value": {
                        "data": [
                            {"field": "Through: 12/31"}, {"field": "For: AllDays"},
                            {"field": "Until: 24:00"}
                        ]
                    }
                }
            }"#,
        )?;

        let result = compile_raw_model(&raw_model);

        assert!(result.has_errors());
        for (object_name, code) in [
            ("Decreasing Until", "InvalidScheduleCompactUntilOrder"),
            ("Incomplete Day", "IncompleteScheduleCompactDayProfile"),
            ("Missing Value", "MissingScheduleCompactValue"),
        ] {
            assert!(result.report.diagnostics.iter().any(|diagnostic| {
                diagnostic.code == code && diagnostic.object_name.as_deref() == Some(object_name)
            }));
        }

        Ok(())
    }

    #[test]
    fn parses_schedule_compact_interpolation_modes() -> Result<(), Box<dyn std::error::Error>> {
        let raw_model = parse_epjson_str(
            r#"{
                "Schedule:Compact": {
                    "Default No": {
                        "data": [
                            {"field": "Through: 12/31"},
                            {"field": "For: AllDays"},
                            {"field": "Until: 24:00"}, {"field": 1}
                        ]
                    },
                    "Explicit No": {
                        "data": [
                            {"field": "Through: 12/31"}, {"field": "For: AllDays"},
                            {"field": "Interpolate: No"},
                            {"field": "Until: 24:00"}, {"field": 1}
                        ]
                    },
                    "Average": {
                        "data": [
                            {"field": "Through: 12/31"}, {"field": "For: AllDays"},
                            {"field": "Interpolate: Average"},
                            {"field": "Until: 24:00"}, {"field": 1}
                        ]
                    },
                    "Linear": {
                        "data": [
                            {"field": "Through: 12/31"}, {"field": "For: AllDays"},
                            {"field": "Interpolate: Linear"},
                            {"field": "Until: 24:00"}, {"field": 1}
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
        for (name, expected) in [
            ("DEFAULT NO", ScheduleInterpolation::No),
            ("EXPLICIT NO", ScheduleInterpolation::No),
            ("AVERAGE", ScheduleInterpolation::Average),
            ("LINEAR", ScheduleInterpolation::Linear),
        ] {
            let schedule = model
                .compact_schedules
                .iter()
                .find(|schedule| schedule.name.0 == name)
                .ok_or_else(|| std::io::Error::other(format!("missing schedule {name}")))?;
            assert_eq!(schedule.periods[0].day_profiles[0].interpolation, expected);
        }

        Ok(())
    }

    #[test]
    fn rejects_invalid_duplicate_and_misplaced_schedule_compact_interpolation()
    -> Result<(), Box<dyn std::error::Error>> {
        let raw_model = parse_epjson_str(
            r#"{
                "Schedule:Compact": {
                    "Invalid Mode": {
                        "data": [
                            {"field": "Through: 12/31"}, {"field": "For: AllDays"},
                            {"field": "Interpolate: Spline"},
                            {"field": "Until: 24:00"}, {"field": 1}
                        ]
                    },
                    "Duplicate Mode": {
                        "data": [
                            {"field": "Through: 12/31"}, {"field": "For: AllDays"},
                            {"field": "Interpolate: No"},
                            {"field": "Interpolate: Linear"},
                            {"field": "Until: 24:00"}, {"field": 1}
                        ]
                    },
                    "Misplaced Mode": {
                        "data": [
                            {"field": "Through: 12/31"}, {"field": "For: AllDays"},
                            {"field": "Until: 12:00"}, {"field": 0},
                            {"field": "Interpolate: Average"},
                            {"field": "Until: 24:00"}, {"field": 1}
                        ]
                    }
                }
            }"#,
        )?;

        let result = compile_raw_model(&raw_model);

        assert!(result.has_errors());
        for (object_name, code) in [
            ("Invalid Mode", "InvalidScheduleCompactInterpolation"),
            ("Duplicate Mode", "DuplicateScheduleCompactInterpolation"),
            ("Misplaced Mode", "InvalidScheduleCompactInterpolationOrder"),
        ] {
            assert!(result.report.diagnostics.iter().any(|diagnostic| {
                diagnostic.code == code && diagnostic.object_name.as_deref() == Some(object_name)
            }));
        }

        Ok(())
    }

    #[test]
    fn warns_only_for_no_interpolation_until_not_aligned_to_valid_timestep()
    -> Result<(), Box<dyn std::error::Error>> {
        let raw_model = parse_epjson_str(
            r#"{
                "Timestep": {
                    "Quarter Hour": {"number_of_timesteps_per_hour": 4}
                },
                "Schedule:Compact": {
                    "Default No": {
                        "data": [
                            {"field": "Through: 12/31"}, {"field": "For: AllDays"},
                            {"field": "Until: 00:20"}, {"field": 0},
                            {"field": "Until: 24:00"}, {"field": 1}
                        ]
                    },
                    "Explicit No": {
                        "data": [
                            {"field": "Through: 12/31"}, {"field": "For: AllDays"},
                            {"field": "Interpolate: No"},
                            {"field": "Until: 00:20"}, {"field": 0},
                            {"field": "Until: 24:00"}, {"field": 1}
                        ]
                    },
                    "Average": {
                        "data": [
                            {"field": "Through: 12/31"}, {"field": "For: AllDays"},
                            {"field": "Interpolate: Average"},
                            {"field": "Until: 00:20"}, {"field": 0},
                            {"field": "Until: 24:00"}, {"field": 1}
                        ]
                    },
                    "Linear": {
                        "data": [
                            {"field": "Through: 12/31"}, {"field": "For: AllDays"},
                            {"field": "Interpolate: Linear"},
                            {"field": "Until: 00:20"}, {"field": 0},
                            {"field": "Until: 24:00"}, {"field": 1}
                        ]
                    }
                }
            }"#,
        )?;

        let result = compile_raw_model(&raw_model);

        assert!(!result.has_errors());
        assert!(result.model.is_some());
        let warnings = result
            .report
            .diagnostics
            .iter()
            .filter(|diagnostic| {
                diagnostic.severity == DiagnosticSeverity::Warning
                    && diagnostic.code == "ScheduleCompactUntilNotAlignedToTimestep"
            })
            .collect::<Vec<_>>();
        assert_eq!(warnings.len(), 2);
        assert!(
            warnings
                .iter()
                .any(|diagnostic| { diagnostic.object_name.as_deref() == Some("Default No") })
        );
        assert!(
            warnings
                .iter()
                .any(|diagnostic| { diagnostic.object_name.as_deref() == Some("Explicit No") })
        );

        Ok(())
    }

    #[test]
    fn rejects_schedule_compact_unknown_day_type() -> Result<(), Box<dyn std::error::Error>> {
        let raw_model = parse_epjson_str(
            r#"{
                "Schedule:Compact": {
                    "Unknown Day": {
                        "data": [
                            {"field": "Through: 12/31"},
                            {"field": "For: Funday AllOtherDays"},
                            {"field": "Until: 24:00"}, {"field": 1}
                        ]
                    }
                }
            }"#,
        )?;

        let result = compile_raw_model(&raw_model);

        assert!(result.has_errors());
        assert!(result.report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "UnsupportedScheduleCompactDayType"
                && diagnostic.object_name.as_deref() == Some("Unknown Day")
        }));

        Ok(())
    }

    #[test]
    fn rejects_schedule_compact_malformed_state_order_and_nonfixed_through()
    -> Result<(), Box<dyn std::error::Error>> {
        let raw_model = parse_epjson_str(
            r#"{
                "Schedule:Compact": {
                    "Malformed Order": {
                        "data": [
                            {"field": "For: AllDays"},
                            {"field": "Until: 24:00"}, {"field": 1},
                            {"field": "Through: 12/31"},
                            {"field": "For: AllDays"},
                            {"field": 2},
                            {"field": "Until: 24:00"}
                        ]
                    },
                    "Dynamic Through": {
                        "data": [
                            {"field": "Through: 1st Monday in January"},
                            {"field": "For: AllDays"},
                            {"field": "Until: 24:00"}, {"field": 1}
                        ]
                    }
                }
            }"#,
        )?;

        let result = compile_raw_model(&raw_model);

        assert!(result.has_errors());
        for code in [
            "InvalidScheduleCompactOrder",
            "InvalidScheduleCompactValue",
            "MissingScheduleCompactValue",
        ] {
            assert!(result.report.diagnostics.iter().any(|diagnostic| {
                diagnostic.code == code
                    && diagnostic.object_name.as_deref() == Some("Malformed Order")
            }));
        }
        assert!(result.report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "InvalidScheduleCompactThrough"
                && diagnostic.object_name.as_deref() == Some("Dynamic Through")
        }));

        Ok(())
    }

    #[test]
    fn parses_thermostat_and_ideal_loads_graph() -> Result<(), Box<dyn std::error::Error>> {
        let raw_model = parse_epjson_str(
            r#"{
                "Schedule:Constant": {
                    "Control Type": {"hourly_value": 4},
                    "Heating Setpoint": {"hourly_value": 21},
                    "Cooling Setpoint": {"hourly_value": 24},
                    "Humidifying RH": {"hourly_value": 10},
                    "Dehumidifying RH": {"hourly_value": 45},
                    "OA Fraction": {"hourly_value": 0.5},
                    "OA Minimum": {"hourly_value": 0.2}
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
                "ZoneControl:Humidistat": {
                    "Zone Humidistat": {
                        "zone_name": "Zone One",
                        "humidifying_relative_humidity_setpoint_schedule_name": "Humidifying RH",
                        "dehumidifying_relative_humidity_setpoint_schedule_name": "Dehumidifying RH"
                    }
                },
                "NodeList": {
                    "Zone Inlets": {
                        "nodes": [
                            {"node_name": "Zone One Inlet"}
                        ]
                    }
                },
                "DesignSpecification:OutdoorAir": {
                    "Outdoor Air Spec": {
                        "outdoor_air_method": "Sum",
                        "outdoor_air_flow_per_person": 0.004,
                        "outdoor_air_flow_per_zone_floor_area": 0.0003,
                        "outdoor_air_flow_per_zone": 0.02,
                        "outdoor_air_flow_air_changes_per_hour": 0.5,
                        "outdoor_air_schedule_name": "OA Fraction",
                        "proportional_control_minimum_outdoor_air_flow_rate_schedule_name": "OA Minimum"
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
                        "design_specification_outdoor_air_object_name": "Outdoor Air Spec",
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
        assert_eq!(model.zone_humidistats.len(), 1);
        assert_eq!(model.design_specification_outdoor_air.len(), 1);
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
        assert_eq!(model.zone_humidistats[0].zone.0, 0);
        let humidifying_schedule = model
            .schedule_names
            .resolve("Humidifying RH")
            .expect("Humidifying RH schedule should resolve");
        let dehumidifying_schedule = model
            .schedule_names
            .resolve("Dehumidifying RH")
            .expect("Dehumidifying RH schedule should resolve");
        assert_eq!(
            model.zone_humidistats[0].humidifying_relative_humidity_setpoint_schedule,
            humidifying_schedule
        );
        assert_eq!(
            model.zone_humidistats[0].dehumidifying_relative_humidity_setpoint_schedule,
            dehumidifying_schedule
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
            model.design_specification_outdoor_air[0].method,
            DesignSpecificationOutdoorAirMethod::Sum
        );
        assert_eq!(
            model.design_specification_outdoor_air[0]
                .outdoor_air_flow_per_zone_floor_area_m3_per_s_m2,
            0.0003
        );
        assert_eq!(
            model.design_specification_outdoor_air[0].outdoor_air_schedule,
            model.schedule_names.resolve("OA Fraction")
        );
        assert_eq!(
            model.design_specification_outdoor_air[0]
                .proportional_control_minimum_outdoor_air_flow_rate_schedule,
            model.schedule_names.resolve("OA Minimum")
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
        assert_eq!(graph.ideal_loads_outdoor_air_specs.len(), 1);
        assert_eq!(graph.zone_air_nodes.len(), 1);
        assert_eq!(
            graph.ideal_loads_outdoor_air_specs[0].ideal_loads_air_system,
            model.ideal_loads_air_systems[0].id
        );
        assert_eq!(
            graph.ideal_loads_outdoor_air_specs[0].design_specification_outdoor_air,
            model.design_specification_outdoor_air[0].id
        );
        assert_eq!(graph.zone_ideal_loads[0].cooling_sequence, 1);
        assert_eq!(graph.zone_ideal_loads[0].heating_or_no_load_sequence, 1);

        Ok(())
    }

    #[test]
    fn parses_plant_loop_skeleton_graph() -> Result<(), Box<dyn std::error::Error>> {
        let raw_model = parse_epjson_str(
            r#"{
                "Pump:ConstantSpeed": {
                    "HW Pump": {
                        "inlet_node_name": "HW Supply Inlet",
                        "outlet_node_name": "HW Pump Outlet",
                        "design_flow_rate": 0.001,
                        "design_pump_head": 179352,
                        "pump_control_type": "Intermittent"
                    }
                },
                "Boiler:HotWater": {
                    "HW Boiler": {
                        "fuel_type": "NaturalGas",
                        "nominal_capacity": 10000,
                        "design_water_flow_rate": 0.001,
                        "boiler_water_inlet_node_name": "HW Pump Outlet",
                        "boiler_water_outlet_node_name": "HW Supply Outlet"
                    }
                },
                "Chiller:Electric:EIR": {
                    "CW Chiller": {
                        "reference_capacity": 12000,
                        "reference_cop": 3.2,
                        "chilled_water_inlet_node_name": "CW Supply Inlet",
                        "chilled_water_outlet_node_name": "CW Supply Outlet",
                        "condenser_inlet_node_name": "Cond Inlet",
                        "condenser_outlet_node_name": "Cond Outlet"
                    }
                },
                "Branch": {
                    "HW Supply Inlet Branch": {
                        "components": [
                            {
                                "component_object_type": "Pump:ConstantSpeed",
                                "component_name": "HW Pump",
                                "component_inlet_node_name": "HW Supply Inlet",
                                "component_outlet_node_name": "HW Pump Outlet"
                            }
                        ]
                    },
                    "HW Boiler Branch": {
                        "components": [
                            {
                                "component_object_type": "Boiler:HotWater",
                                "component_name": "HW Boiler",
                                "component_inlet_node_name": "HW Pump Outlet",
                                "component_outlet_node_name": "HW Supply Outlet"
                            }
                        ]
                    },
                    "HW Demand Branch": {
                        "components": [
                            {
                                "component_object_type": "Pipe:Adiabatic",
                                "component_name": "HW Demand Pipe",
                                "component_inlet_node_name": "HW Demand Inlet",
                                "component_outlet_node_name": "HW Demand Outlet"
                            }
                        ]
                    },
                    "CW Chiller Branch": {
                        "components": [
                            {
                                "component_object_type": "Chiller:Electric:EIR",
                                "component_name": "CW Chiller",
                                "component_inlet_node_name": "CW Supply Inlet",
                                "component_outlet_node_name": "CW Supply Outlet"
                            }
                        ]
                    }
                },
                "BranchList": {
                    "HW Supply Branches": {
                        "branches": [
                            {"branch_name": "HW Supply Inlet Branch"},
                            {"branch_name": "HW Boiler Branch"}
                        ]
                    },
                    "HW Demand Branches": {
                        "branches": [
                            {"branch_name": "HW Demand Branch"}
                        ]
                    },
                    "CW Supply Branches": {
                        "branches": [
                            {"branch_name": "CW Chiller Branch"}
                        ]
                    }
                },
                "Connector:Splitter": {
                    "HW Supply Splitter": {
                        "inlet_branch_name": "HW Supply Inlet Branch",
                        "branches": [
                            {"outlet_branch_name": "HW Boiler Branch"}
                        ]
                    }
                },
                "Connector:Mixer": {
                    "HW Supply Mixer": {
                        "outlet_branch_name": "HW Boiler Branch",
                        "branches": [
                            {"inlet_branch_name": "HW Supply Inlet Branch"}
                        ]
                    }
                },
                "ConnectorList": {
                    "HW Supply Connectors": {
                        "connector_1_object_type": "Connector:Splitter",
                        "connector_1_name": "HW Supply Splitter",
                        "connector_2_object_type": "Connector:Mixer",
                        "connector_2_name": "HW Supply Mixer"
                    }
                },
                "PlantLoop": {
                    "Hot Water Loop": {
                        "fluid_type": "Water",
                        "plant_side_inlet_node_name": "HW Supply Inlet",
                        "plant_side_outlet_node_name": "HW Supply Outlet",
                        "plant_side_branch_list_name": "HW Supply Branches",
                        "plant_side_connector_list_name": "HW Supply Connectors",
                        "demand_side_inlet_node_name": "HW Demand Inlet",
                        "demand_side_outlet_node_name": "HW Demand Outlet",
                        "demand_side_branch_list_name": "HW Demand Branches",
                        "load_distribution_scheme": "SequentialLoad"
                    }
                }
            }"#,
        )?;

        let result = compile_raw_model(&raw_model);

        assert!(!result.has_errors());
        let Some(model) = result.model else {
            return Err(std::io::Error::other("expected typed model").into());
        };
        assert_eq!(model.plant_loops.len(), 1);
        assert_eq!(model.plant_branches.len(), 4);
        assert_eq!(model.plant_branch_lists.len(), 3);
        assert_eq!(model.plant_connectors.len(), 2);
        assert_eq!(model.plant_connector_lists.len(), 1);
        assert_eq!(model.pumps_constant_speed.len(), 1);
        assert_eq!(model.boilers_hot_water.len(), 1);
        assert_eq!(model.chillers_electric_eir.len(), 1);
        assert_eq!(model.nodes.len(), 9);
        assert_eq!(model.plant_connectors[0].kind, PlantConnectorKind::Splitter);

        let graph = ModelGraph::from_typed(&model);
        assert_eq!(graph.plant_loop_branch_lists.len(), 2);
        assert_eq!(graph.plant_branch_list_members.len(), 4);
        assert_eq!(graph.plant_connector_list_members.len(), 2);
        assert_eq!(graph.plant_branch_components.len(), 4);

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
                "Material:NoMass": {
                    "R13": {"roughness": "Rough", "thermal_resistance": 1.0}
                },
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
