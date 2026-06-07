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
typed_id!(ThermostatSetpointId);
typed_id!(ZoneThermostatId);
typed_id!(IdealLoadsAirSystemId);
typed_id!(ZoneEquipmentListId);
typed_id!(ZoneEquipmentConnectionId);
typed_id!(NodeId);
typed_id!(NodeListId);
typed_id!(ComponentId);
typed_id!(LoopId);
typed_id!(BranchId);
typed_id!(BranchListId);
typed_id!(ConnectorId);
typed_id!(ConnectorListId);
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

/// Numeric field that may be set to EnergyPlus Autosize.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AutosizeOrNumber {
    /// EnergyPlus should autosize the value.
    Autosize,
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

/// Supported thermostat control object type for the first HVAC subset.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ThermostatControlObjectType {
    /// `ThermostatSetpoint:DualSetpoint`.
    DualSetpoint,
}

/// Heating and cooling setpoint schedules.
#[derive(Clone, Debug, PartialEq)]
pub struct ThermostatDualSetpoint {
    /// Typed ID.
    pub id: ThermostatSetpointId,
    /// Object name.
    pub name: NormalizedName,
    /// Heating setpoint schedule.
    pub heating_setpoint_schedule: ScheduleId,
    /// Cooling setpoint schedule.
    pub cooling_setpoint_schedule: ScheduleId,
}

/// One control entry inside `ZoneControl:Thermostat`.
#[derive(Clone, Debug, PartialEq)]
pub struct ZoneThermostatControl {
    /// Control object type.
    pub object_type: ThermostatControlObjectType,
    /// Referenced dual setpoint object.
    pub dual_setpoint: ThermostatSetpointId,
}

/// Zone thermostat assignment.
#[derive(Clone, Debug, PartialEq)]
pub struct ZoneThermostat {
    /// Typed ID.
    pub id: ZoneThermostatId,
    /// Object name.
    pub name: NormalizedName,
    /// Controlled zone.
    pub zone: ZoneId,
    /// Schedule containing thermostat control type integers.
    pub control_type_schedule: ScheduleId,
    /// Thermostat control entries in EnergyPlus order.
    pub controls: Vec<ZoneThermostatControl>,
    /// Temperature difference between cutout and setpoint in delta C.
    pub temperature_difference_between_cutout_and_setpoint_delta_c: f64,
}

/// `ZoneHVAC:IdealLoadsAirSystem` limit mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IdealLoadsLimit {
    /// No flow or capacity limit.
    NoLimit,
    /// Limit flow rate only.
    LimitFlowRate,
    /// Limit capacity only.
    LimitCapacity,
    /// Limit both flow rate and capacity.
    LimitFlowRateAndCapacity,
}

/// Ideal loads dehumidification control mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DehumidificationControlType {
    /// No dehumidification.
    None,
    /// Constant sensible heat ratio.
    ConstantSensibleHeatRatio,
    /// Constant supply humidity ratio.
    ConstantSupplyHumidityRatio,
    /// Humidistat-controlled.
    Humidistat,
}

/// Ideal loads humidification control mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HumidificationControlType {
    /// No humidification.
    None,
    /// Constant supply humidity ratio.
    ConstantSupplyHumidityRatio,
    /// Humidistat-controlled.
    Humidistat,
}

/// Demand-controlled ventilation mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DemandControlledVentilationType {
    /// No DCV.
    None,
    /// Occupancy schedule DCV.
    OccupancySchedule,
    /// CO2 setpoint DCV.
    Co2Setpoint,
}

/// Outdoor-air economizer mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OutdoorAirEconomizerType {
    /// No economizer.
    NoEconomizer,
    /// Differential dry-bulb economizer.
    DifferentialDryBulb,
    /// Differential enthalpy economizer.
    DifferentialEnthalpy,
}

/// Heat recovery mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeatRecoveryType {
    /// No heat recovery.
    None,
    /// Sensible heat recovery.
    Sensible,
    /// Enthalpy heat recovery.
    Enthalpy,
}

/// Ideal loads purchased energy fuel type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IdealLoadsFuelType {
    /// Coal.
    Coal,
    /// Diesel.
    Diesel,
    /// District cooling.
    DistrictCooling,
    /// District heating steam.
    DistrictHeatingSteam,
    /// District heating water.
    DistrictHeatingWater,
    /// Electricity.
    Electricity,
    /// Fuel oil no. 1.
    FuelOilNo1,
    /// Fuel oil no. 2.
    FuelOilNo2,
    /// Gasoline.
    Gasoline,
    /// Natural gas.
    NaturalGas,
    /// Other fuel 1.
    OtherFuel1,
    /// Other fuel 2.
    OtherFuel2,
    /// Propane.
    Propane,
}

/// Typed IdealLoads air system inputs needed before load-solver parity.
#[derive(Clone, Debug, PartialEq)]
pub struct IdealLoadsAirSystem {
    /// Typed ID.
    pub id: IdealLoadsAirSystemId,
    /// Object name.
    pub name: NormalizedName,
    /// Overall availability schedule.
    pub availability_schedule: Option<ScheduleId>,
    /// Zone supply air node or node list name.
    pub zone_supply_air_node_name: NormalizedName,
    /// Optional zone exhaust air node name.
    pub zone_exhaust_air_node_name: Option<NormalizedName>,
    /// Optional system inlet air node name.
    pub system_inlet_air_node_name: Option<NormalizedName>,
    /// Maximum heating supply air temperature in C.
    pub maximum_heating_supply_air_temperature_c: f64,
    /// Minimum cooling supply air temperature in C.
    pub minimum_cooling_supply_air_temperature_c: f64,
    /// Maximum heating supply humidity ratio.
    pub maximum_heating_supply_air_humidity_ratio: f64,
    /// Minimum cooling supply humidity ratio.
    pub minimum_cooling_supply_air_humidity_ratio: f64,
    /// Heating limit mode.
    pub heating_limit: IdealLoadsLimit,
    /// Maximum heating air flow rate.
    pub maximum_heating_air_flow_rate_m3_per_s: Option<AutosizeOrNumber>,
    /// Maximum sensible heating capacity.
    pub maximum_sensible_heating_capacity_w: Option<AutosizeOrNumber>,
    /// Cooling limit mode.
    pub cooling_limit: IdealLoadsLimit,
    /// Maximum cooling air flow rate.
    pub maximum_cooling_air_flow_rate_m3_per_s: Option<AutosizeOrNumber>,
    /// Maximum total cooling capacity.
    pub maximum_total_cooling_capacity_w: Option<AutosizeOrNumber>,
    /// Heating availability schedule.
    pub heating_availability_schedule: Option<ScheduleId>,
    /// Cooling availability schedule.
    pub cooling_availability_schedule: Option<ScheduleId>,
    /// Dehumidification control type.
    pub dehumidification_control_type: DehumidificationControlType,
    /// Cooling sensible heat ratio.
    pub cooling_sensible_heat_ratio: f64,
    /// Humidification control type.
    pub humidification_control_type: HumidificationControlType,
    /// Optional design specification outdoor air object name.
    pub design_specification_outdoor_air_object_name: Option<NormalizedName>,
    /// Optional outdoor air inlet node name.
    pub outdoor_air_inlet_node_name: Option<NormalizedName>,
    /// Demand-controlled ventilation type.
    pub demand_controlled_ventilation_type: DemandControlledVentilationType,
    /// Outdoor air economizer type.
    pub outdoor_air_economizer_type: OutdoorAirEconomizerType,
    /// Heat recovery type.
    pub heat_recovery_type: HeatRecoveryType,
    /// Sensible heat recovery effectiveness.
    pub sensible_heat_recovery_effectiveness: f64,
    /// Latent heat recovery effectiveness.
    pub latent_heat_recovery_effectiveness: f64,
    /// Optional zone HVAC sizing object name.
    pub design_specification_zonehvac_sizing_object_name: Option<NormalizedName>,
    /// Optional heating fuel efficiency schedule.
    pub heating_fuel_efficiency_schedule: Option<ScheduleId>,
    /// Heating fuel type.
    pub heating_fuel_type: IdealLoadsFuelType,
    /// Optional cooling fuel efficiency schedule.
    pub cooling_fuel_efficiency_schedule: Option<ScheduleId>,
    /// Cooling fuel type.
    pub cooling_fuel_type: IdealLoadsFuelType,
}

/// Typed air-side node discovered from node lists and HVAC node references.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Node {
    /// Typed ID.
    pub id: NodeId,
    /// Node name.
    pub name: NormalizedName,
}

/// Typed `NodeList` input.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NodeList {
    /// Typed ID.
    pub id: NodeListId,
    /// NodeList name.
    pub name: NormalizedName,
    /// Member nodes in declared order.
    pub nodes: Vec<NodeId>,
}

/// Zone equipment load distribution scheme.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LoadDistributionScheme {
    /// Sequential load distribution.
    SequentialLoad,
    /// Uniform load distribution.
    UniformLoad,
    /// Uniform part-load-ratio distribution.
    UniformPlr,
    /// Sequential uniform part-load-ratio distribution.
    SequentialUniformPlr,
}

/// Zone equipment object types supported by the first HVAC graph subset.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ZoneEquipmentObjectType {
    /// `ZoneHVAC:IdealLoadsAirSystem`.
    IdealLoadsAirSystem,
}

/// One item in `ZoneHVAC:EquipmentList`.
#[derive(Clone, Debug, PartialEq)]
pub struct ZoneEquipmentListEntry {
    /// Equipment object type.
    pub object_type: ZoneEquipmentObjectType,
    /// Referenced IdealLoads air system.
    pub ideal_loads_air_system: IdealLoadsAirSystemId,
    /// Cooling sequence.
    pub cooling_sequence: u32,
    /// Heating or no-load sequence.
    pub heating_or_no_load_sequence: u32,
    /// Optional sequential cooling fraction schedule.
    pub sequential_cooling_fraction_schedule: Option<ScheduleId>,
    /// Optional sequential heating fraction schedule.
    pub sequential_heating_fraction_schedule: Option<ScheduleId>,
}

/// Zone equipment list.
#[derive(Clone, Debug, PartialEq)]
pub struct ZoneEquipmentList {
    /// Typed ID.
    pub id: ZoneEquipmentListId,
    /// Object name.
    pub name: NormalizedName,
    /// Load distribution scheme.
    pub load_distribution_scheme: LoadDistributionScheme,
    /// Ordered equipment entries.
    pub equipment: Vec<ZoneEquipmentListEntry>,
}

/// Zone HVAC equipment connections.
#[derive(Clone, Debug, PartialEq)]
pub struct ZoneEquipmentConnection {
    /// Typed ID.
    pub id: ZoneEquipmentConnectionId,
    /// Connected zone.
    pub zone: ZoneId,
    /// Conditioning equipment list.
    pub equipment_list: ZoneEquipmentListId,
    /// Zone air inlet node or node list name.
    pub zone_air_inlet_node_or_nodelist_name: Option<NormalizedName>,
    /// Zone air exhaust node or node list name.
    pub zone_air_exhaust_node_or_nodelist_name: Option<NormalizedName>,
    /// Zone air node name.
    pub zone_air_node_name: NormalizedName,
    /// Zone return air node or node list name.
    pub zone_return_air_node_or_nodelist_name: Option<NormalizedName>,
    /// Optional return-air fraction schedule.
    pub zone_return_air_node_1_flow_rate_fraction_schedule: Option<ScheduleId>,
    /// Optional return-air basis node or node list.
    pub zone_return_air_node_1_flow_rate_basis_node_or_nodelist_name: Option<NormalizedName>,
}

/// One central plant loop shell.
#[derive(Clone, Debug, PartialEq)]
pub struct PlantLoop {
    /// Typed ID.
    pub id: LoopId,
    /// Plant loop name.
    pub name: NormalizedName,
    /// Fluid type as declared by EnergyPlus input.
    pub fluid_type: NormalizedName,
    /// Plant side inlet node.
    pub plant_side_inlet_node: NodeId,
    /// Plant side outlet node.
    pub plant_side_outlet_node: NodeId,
    /// Plant side branch list.
    pub plant_side_branch_list: BranchListId,
    /// Optional plant side connector list.
    pub plant_side_connector_list: Option<ConnectorListId>,
    /// Demand side inlet node.
    pub demand_side_inlet_node: NodeId,
    /// Demand side outlet node.
    pub demand_side_outlet_node: NodeId,
    /// Demand side branch list.
    pub demand_side_branch_list: BranchListId,
    /// Optional demand side connector list.
    pub demand_side_connector_list: Option<ConnectorListId>,
    /// Load distribution scheme as declared by EnergyPlus input.
    pub load_distribution_scheme: Option<NormalizedName>,
}

/// Component reference inside one plant branch.
#[derive(Clone, Debug, PartialEq)]
pub struct PlantBranchComponent {
    /// Component object type.
    pub object_type: NormalizedName,
    /// Component object name.
    pub name: NormalizedName,
    /// Component inlet node.
    pub inlet_node: NodeId,
    /// Component outlet node.
    pub outlet_node: NodeId,
}

/// Plant branch with ordered components.
#[derive(Clone, Debug, PartialEq)]
pub struct PlantBranch {
    /// Typed ID.
    pub id: BranchId,
    /// Branch name.
    pub name: NormalizedName,
    /// Ordered branch components.
    pub components: Vec<PlantBranchComponent>,
}

/// Ordered branch list.
#[derive(Clone, Debug, PartialEq)]
pub struct PlantBranchList {
    /// Typed ID.
    pub id: BranchListId,
    /// Branch list name.
    pub name: NormalizedName,
    /// Branches in EnergyPlus flow order.
    pub branches: Vec<BranchId>,
}

/// Connector type supported by the plant skeleton.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlantConnectorKind {
    /// `Connector:Splitter`.
    Splitter,
    /// `Connector:Mixer`.
    Mixer,
}

/// Plant connector with resolved branch references.
#[derive(Clone, Debug, PartialEq)]
pub struct PlantConnector {
    /// Typed ID.
    pub id: ConnectorId,
    /// Connector name.
    pub name: NormalizedName,
    /// Connector kind.
    pub kind: PlantConnectorKind,
    /// Inlet branches for the connector.
    pub inlet_branches: Vec<BranchId>,
    /// Outlet branches for the connector.
    pub outlet_branches: Vec<BranchId>,
}

/// Connector reference inside a connector list.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlantConnectorListEntry {
    /// Connector kind.
    pub kind: PlantConnectorKind,
    /// Connector ID.
    pub connector: ConnectorId,
}

/// Ordered plant connector list.
#[derive(Clone, Debug, PartialEq)]
pub struct PlantConnectorList {
    /// Typed ID.
    pub id: ConnectorListId,
    /// Connector list name.
    pub name: NormalizedName,
    /// Connector entries in EnergyPlus order.
    pub connectors: Vec<PlantConnectorListEntry>,
}

/// Typed `Pump:ConstantSpeed` identity and node endpoints.
#[derive(Clone, Debug, PartialEq)]
pub struct PumpConstantSpeed {
    /// Typed ID within the constant-speed pump subset.
    pub id: ComponentId,
    /// Pump name.
    pub name: NormalizedName,
    /// Inlet node.
    pub inlet_node: NodeId,
    /// Outlet node.
    pub outlet_node: NodeId,
    /// Optional design flow rate in m3/s.
    pub design_flow_rate_m3_per_s: Option<AutosizeOrNumber>,
    /// Optional design pump head in Pa.
    pub design_pump_head_pa: Option<f64>,
    /// Pump control type string.
    pub pump_control_type: Option<NormalizedName>,
}

/// Typed `Boiler:HotWater` identity and node endpoints.
#[derive(Clone, Debug, PartialEq)]
pub struct BoilerHotWater {
    /// Typed ID within the hot-water boiler subset.
    pub id: ComponentId,
    /// Boiler name.
    pub name: NormalizedName,
    /// Fuel type string.
    pub fuel_type: Option<NormalizedName>,
    /// Inlet node.
    pub inlet_node: NodeId,
    /// Outlet node.
    pub outlet_node: NodeId,
    /// Optional nominal capacity in W.
    pub nominal_capacity_w: Option<AutosizeOrNumber>,
    /// Optional design water flow rate in m3/s.
    pub design_water_flow_rate_m3_per_s: Option<AutosizeOrNumber>,
}

/// Typed `Chiller:Electric:EIR` identity and node endpoints.
#[derive(Clone, Debug, PartialEq)]
pub struct ChillerElectricEir {
    /// Typed ID within the electric EIR chiller subset.
    pub id: ComponentId,
    /// Chiller name.
    pub name: NormalizedName,
    /// Chilled water inlet node.
    pub chilled_water_inlet_node: NodeId,
    /// Chilled water outlet node.
    pub chilled_water_outlet_node: NodeId,
    /// Condenser inlet node, when declared.
    pub condenser_inlet_node: Option<NodeId>,
    /// Condenser outlet node, when declared.
    pub condenser_outlet_node: Option<NodeId>,
    /// Optional reference capacity in W.
    pub reference_capacity_w: Option<AutosizeOrNumber>,
    /// Optional reference COP.
    pub reference_cop: Option<f64>,
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
    /// Dual setpoint thermostat objects.
    pub thermostat_dual_setpoints: Vec<ThermostatDualSetpoint>,
    /// Dual setpoint names.
    pub thermostat_dual_setpoint_names: NameMap<ThermostatSetpointId>,
    /// Zone thermostat controls.
    pub zone_thermostats: Vec<ZoneThermostat>,
    /// Zone thermostat names.
    pub zone_thermostat_names: NameMap<ZoneThermostatId>,
    /// IdealLoads air systems.
    pub ideal_loads_air_systems: Vec<IdealLoadsAirSystem>,
    /// IdealLoads air system names.
    pub ideal_loads_air_system_names: NameMap<IdealLoadsAirSystemId>,
    /// Zone equipment lists.
    pub zone_equipment_lists: Vec<ZoneEquipmentList>,
    /// Zone equipment list names.
    pub zone_equipment_list_names: NameMap<ZoneEquipmentListId>,
    /// Zone equipment connections.
    pub zone_equipment_connections: Vec<ZoneEquipmentConnection>,
    /// Discovered air-side nodes.
    pub nodes: Vec<Node>,
    /// Node names.
    pub node_names: NameMap<NodeId>,
    /// Node lists.
    pub node_lists: Vec<NodeList>,
    /// NodeList names.
    pub node_list_names: NameMap<NodeListId>,
    /// Plant loops.
    pub plant_loops: Vec<PlantLoop>,
    /// Plant loop names.
    pub plant_loop_names: NameMap<LoopId>,
    /// Plant branches.
    pub plant_branches: Vec<PlantBranch>,
    /// Plant branch names.
    pub plant_branch_names: NameMap<BranchId>,
    /// Plant branch lists.
    pub plant_branch_lists: Vec<PlantBranchList>,
    /// Plant branch list names.
    pub plant_branch_list_names: NameMap<BranchListId>,
    /// Plant connectors.
    pub plant_connectors: Vec<PlantConnector>,
    /// Plant connector names.
    pub plant_connector_names: NameMap<ConnectorId>,
    /// Plant connector lists.
    pub plant_connector_lists: Vec<PlantConnectorList>,
    /// Plant connector list names.
    pub plant_connector_list_names: NameMap<ConnectorListId>,
    /// Constant-speed pumps.
    pub pumps_constant_speed: Vec<PumpConstantSpeed>,
    /// Constant-speed pump names.
    pub pump_constant_speed_names: NameMap<ComponentId>,
    /// Hot-water boilers.
    pub boilers_hot_water: Vec<BoilerHotWater>,
    /// Hot-water boiler names.
    pub boiler_hot_water_names: NameMap<ComponentId>,
    /// Electric EIR chillers.
    pub chillers_electric_eir: Vec<ChillerElectricEir>,
    /// Electric EIR chiller names.
    pub chiller_electric_eir_names: NameMap<ComponentId>,
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
            thermostat_dual_setpoints: Vec::new(),
            thermostat_dual_setpoint_names: NameMap::default(),
            zone_thermostats: Vec::new(),
            zone_thermostat_names: NameMap::default(),
            ideal_loads_air_systems: Vec::new(),
            ideal_loads_air_system_names: NameMap::default(),
            zone_equipment_lists: Vec::new(),
            zone_equipment_list_names: NameMap::default(),
            zone_equipment_connections: Vec::new(),
            nodes: Vec::new(),
            node_names: NameMap::default(),
            node_lists: Vec::new(),
            node_list_names: NameMap::default(),
            plant_loops: Vec::new(),
            plant_loop_names: NameMap::default(),
            plant_branches: Vec::new(),
            plant_branch_names: NameMap::default(),
            plant_branch_lists: Vec::new(),
            plant_branch_list_names: NameMap::default(),
            plant_connectors: Vec::new(),
            plant_connector_names: NameMap::default(),
            plant_connector_lists: Vec::new(),
            plant_connector_list_names: NameMap::default(),
            pumps_constant_speed: Vec::new(),
            pump_constant_speed_names: NameMap::default(),
            boilers_hot_water: Vec::new(),
            boiler_hot_water_names: NameMap::default(),
            chillers_electric_eir: Vec::new(),
            chiller_electric_eir_names: NameMap::default(),
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
            + self.thermostat_dual_setpoints.len()
            + self.zone_thermostats.len()
            + self.ideal_loads_air_systems.len()
            + self.zone_equipment_lists.len()
            + self.zone_equipment_connections.len()
            + self.node_lists.len()
            + self.plant_loops.len()
            + self.plant_branches.len()
            + self.plant_branch_lists.len()
            + self.plant_connectors.len()
            + self.plant_connector_lists.len()
            + self.pumps_constant_speed.len()
            + self.boilers_hot_water.len()
            + self.chillers_electric_eir.len()
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
    /// Zone to thermostat edges.
    pub zone_thermostats: Vec<ZoneThermostatEdge>,
    /// Thermostat to dual setpoint edges.
    pub thermostat_setpoints: Vec<ThermostatSetpointEdge>,
    /// Zone to IdealLoads equipment edges through equipment connections/lists.
    pub zone_ideal_loads: Vec<ZoneIdealLoadsEdge>,
    /// NodeList membership edges.
    pub node_list_members: Vec<NodeListMemberEdge>,
    /// IdealLoads supply-node edges.
    pub ideal_loads_supply_nodes: Vec<IdealLoadsSupplyNodeEdge>,
    /// Zone air-node edges.
    pub zone_air_nodes: Vec<ZoneAirNodeEdge>,
    /// Plant loop to branch-list edges.
    pub plant_loop_branch_lists: Vec<PlantLoopBranchListEdge>,
    /// Branch-list membership edges.
    pub plant_branch_list_members: Vec<PlantBranchListMemberEdge>,
    /// Connector-list membership edges.
    pub plant_connector_list_members: Vec<PlantConnectorListMemberEdge>,
    /// Branch to component edges.
    pub plant_branch_components: Vec<PlantBranchComponentEdge>,
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
            zone_thermostats: model
                .zone_thermostats
                .iter()
                .map(|thermostat| ZoneThermostatEdge {
                    zone: thermostat.zone,
                    thermostat: thermostat.id,
                })
                .collect(),
            thermostat_setpoints: model
                .zone_thermostats
                .iter()
                .flat_map(|thermostat| {
                    thermostat
                        .controls
                        .iter()
                        .map(move |control| ThermostatSetpointEdge {
                            thermostat: thermostat.id,
                            setpoint: control.dual_setpoint,
                        })
                })
                .collect(),
            zone_ideal_loads: sorted_zone_ideal_loads(model),
            node_list_members: model
                .node_lists
                .iter()
                .flat_map(|node_list| {
                    node_list
                        .nodes
                        .iter()
                        .enumerate()
                        .map(move |(index, node)| NodeListMemberEdge {
                            node_list: node_list.id,
                            node: *node,
                            index: index as u32,
                        })
                })
                .collect(),
            ideal_loads_supply_nodes: model
                .ideal_loads_air_systems
                .iter()
                .flat_map(|system| {
                    resolve_node_or_list(model, &system.zone_supply_air_node_name)
                        .into_iter()
                        .map(move |node| IdealLoadsSupplyNodeEdge {
                            ideal_loads_air_system: system.id,
                            node,
                        })
                })
                .collect(),
            zone_air_nodes: model
                .zone_equipment_connections
                .iter()
                .filter_map(|connection| {
                    model
                        .node_names
                        .resolve(&connection.zone_air_node_name.0)
                        .map(|node| ZoneAirNodeEdge {
                            zone: connection.zone,
                            node,
                        })
                })
                .collect(),
            plant_loop_branch_lists: plant_loop_branch_lists(model),
            plant_branch_list_members: model
                .plant_branch_lists
                .iter()
                .flat_map(|list| {
                    list.branches
                        .iter()
                        .enumerate()
                        .map(move |(index, branch)| PlantBranchListMemberEdge {
                            branch_list: list.id,
                            branch: *branch,
                            index: index as u32,
                        })
                })
                .collect(),
            plant_connector_list_members: model
                .plant_connector_lists
                .iter()
                .flat_map(|list| {
                    list.connectors
                        .iter()
                        .enumerate()
                        .map(move |(index, entry)| PlantConnectorListMemberEdge {
                            connector_list: list.id,
                            connector: entry.connector,
                            kind: entry.kind,
                            index: index as u32,
                        })
                })
                .collect(),
            plant_branch_components: model
                .plant_branches
                .iter()
                .flat_map(|branch| {
                    branch
                        .components
                        .iter()
                        .enumerate()
                        .map(move |(index, component)| PlantBranchComponentEdge {
                            branch: branch.id,
                            component_type: component.object_type.clone(),
                            component_name: component.name.clone(),
                            inlet_node: component.inlet_node,
                            outlet_node: component.outlet_node,
                            index: index as u32,
                        })
                })
                .collect(),
        }
    }
}

fn plant_loop_branch_lists(model: &TypedModel) -> Vec<PlantLoopBranchListEdge> {
    model
        .plant_loops
        .iter()
        .flat_map(|plant_loop| {
            [
                PlantLoopBranchListEdge {
                    plant_loop: plant_loop.id,
                    side: PlantLoopSide::Plant,
                    branch_list: plant_loop.plant_side_branch_list,
                },
                PlantLoopBranchListEdge {
                    plant_loop: plant_loop.id,
                    side: PlantLoopSide::Demand,
                    branch_list: plant_loop.demand_side_branch_list,
                },
            ]
        })
        .collect()
}

fn sorted_zone_ideal_loads(model: &TypedModel) -> Vec<ZoneIdealLoadsEdge> {
    let mut edges: Vec<_> = model
        .zone_equipment_connections
        .iter()
        .flat_map(|connection| {
            model
                .zone_equipment_lists
                .iter()
                .find(move |list| list.id == connection.equipment_list)
                .into_iter()
                .flat_map(move |list| {
                    list.equipment.iter().map(move |entry| ZoneIdealLoadsEdge {
                        zone: connection.zone,
                        equipment_list: list.id,
                        ideal_loads_air_system: entry.ideal_loads_air_system,
                        cooling_sequence: entry.cooling_sequence,
                        heating_or_no_load_sequence: entry.heating_or_no_load_sequence,
                    })
                })
        })
        .collect();
    edges.sort_by_key(|edge| {
        (
            edge.zone,
            edge.heating_or_no_load_sequence,
            edge.cooling_sequence,
            edge.ideal_loads_air_system,
        )
    });
    edges
}

fn resolve_node_or_list(model: &TypedModel, name: &NormalizedName) -> Vec<NodeId> {
    if let Some(node) = model.node_names.resolve(&name.0) {
        return vec![node];
    }
    if let Some(node_list) = model.node_list_names.resolve(&name.0)
        && let Some(list) = model.node_lists.iter().find(|list| list.id == node_list)
    {
        return list.nodes.clone();
    }
    Vec::new()
}

/// NodeList membership relation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NodeListMemberEdge {
    /// NodeList ID.
    pub node_list: NodeListId,
    /// Member node ID.
    pub node: NodeId,
    /// Zero-based member index.
    pub index: u32,
}

/// IdealLoads supply-node relation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IdealLoadsSupplyNodeEdge {
    /// IdealLoads system ID.
    pub ideal_loads_air_system: IdealLoadsAirSystemId,
    /// Resolved supply node ID.
    pub node: NodeId,
}

/// Zone air-node relation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ZoneAirNodeEdge {
    /// Zone ID.
    pub zone: ZoneId,
    /// Node ID.
    pub node: NodeId,
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

/// Zone/thermostat relation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ZoneThermostatEdge {
    /// Zone ID.
    pub zone: ZoneId,
    /// Thermostat ID.
    pub thermostat: ZoneThermostatId,
}

/// Thermostat/setpoint relation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ThermostatSetpointEdge {
    /// Thermostat ID.
    pub thermostat: ZoneThermostatId,
    /// Dual setpoint ID.
    pub setpoint: ThermostatSetpointId,
}

/// Zone/IdealLoads relation through equipment connections and lists.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ZoneIdealLoadsEdge {
    /// Zone ID.
    pub zone: ZoneId,
    /// Equipment list ID.
    pub equipment_list: ZoneEquipmentListId,
    /// IdealLoads system ID.
    pub ideal_loads_air_system: IdealLoadsAirSystemId,
    /// Cooling sequence.
    pub cooling_sequence: u32,
    /// Heating or no-load sequence.
    pub heating_or_no_load_sequence: u32,
}

/// Side of a plant loop.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlantLoopSide {
    /// Supply/plant side.
    Plant,
    /// Demand side.
    Demand,
}

/// Plant loop to branch-list relation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlantLoopBranchListEdge {
    /// Plant loop ID.
    pub plant_loop: LoopId,
    /// Loop side.
    pub side: PlantLoopSide,
    /// Branch list ID.
    pub branch_list: BranchListId,
}

/// Branch-list membership relation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlantBranchListMemberEdge {
    /// Branch list ID.
    pub branch_list: BranchListId,
    /// Branch ID.
    pub branch: BranchId,
    /// Zero-based member index.
    pub index: u32,
}

/// Connector-list membership relation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlantConnectorListMemberEdge {
    /// Connector list ID.
    pub connector_list: ConnectorListId,
    /// Connector ID.
    pub connector: ConnectorId,
    /// Connector kind.
    pub kind: PlantConnectorKind,
    /// Zero-based member index.
    pub index: u32,
}

/// Branch to component relation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlantBranchComponentEdge {
    /// Branch ID.
    pub branch: BranchId,
    /// Component object type.
    pub component_type: NormalizedName,
    /// Component name.
    pub component_name: NormalizedName,
    /// Component inlet node.
    pub inlet_node: NodeId,
    /// Component outlet node.
    pub outlet_node: NodeId,
    /// Zero-based component index.
    pub index: u32,
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
