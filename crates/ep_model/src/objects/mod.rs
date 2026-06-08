//! Typed EnergyPlus object records for the supported seed families.

use crate::{
    AutoOrNumber, AutosizeOrNumber, BranchId, BranchListId, ComponentId, ConnectorId,
    ConnectorListId, ConstructionId, IdealLoadsAirSystemId, InternalGainId, LoopId, MaterialId,
    NodeId, NodeListId, NormalizedName, Point3, RunPeriodId, ScheduleId, ScheduleTypeLimitId,
    SurfaceId, ThermostatSetpointId, ZoneEquipmentConnectionId, ZoneEquipmentListId, ZoneId,
    ZoneThermostatId,
};

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

/// Default inside surface convection algorithm.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InsideSurfaceConvectionAlgorithm {
    /// Constant natural convection.
    Simple,
    /// ASHRAE TARP natural convection.
    Tarp,
    /// Ceiling diffuser mixed convection.
    CeilingDiffuser,
    /// EnergyPlus adaptive inside convection model selection.
    AdaptiveConvectionAlgorithm,
    /// ASTM C1340 mixed convection correlations.
    AstmC1340,
}

/// Default outside surface convection algorithm.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OutsideSurfaceConvectionAlgorithm {
    /// Simple combined exterior coefficient.
    SimpleCombined,
    /// TARP exterior convection.
    Tarp,
    /// MoWiTT smooth-surface exterior convection.
    MoWitt,
    /// DOE-2 rough-surface exterior convection.
    Doe2,
    /// EnergyPlus adaptive outside convection model selection.
    AdaptiveConvectionAlgorithm,
}

/// Global surface convection algorithm settings.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SurfaceConvectionAlgorithms {
    /// Parsed `SurfaceConvectionAlgorithm:Inside` setting, when present.
    pub inside: Option<InsideSurfaceConvectionAlgorithm>,
    /// Parsed `SurfaceConvectionAlgorithm:Outside` setting, when present.
    pub outside: Option<OutsideSurfaceConvectionAlgorithm>,
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

/// EnergyPlus material surface roughness.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MaterialSurfaceRoughness {
    /// `VeryRough`.
    VeryRough,
    /// `Rough`.
    Rough,
    /// `MediumRough`.
    MediumRough,
    /// `MediumSmooth`.
    MediumSmooth,
    /// `Smooth`.
    Smooth,
    /// `VerySmooth`.
    VerySmooth,
}

impl MaterialSurfaceRoughness {
    /// Parses an EnergyPlus roughness token.
    #[must_use]
    pub fn from_energyplus_name(value: &str) -> Option<Self> {
        match value.trim().to_ascii_uppercase().as_str() {
            "VERYROUGH" => Some(Self::VeryRough),
            "ROUGH" => Some(Self::Rough),
            "MEDIUMROUGH" => Some(Self::MediumRough),
            "MEDIUMSMOOTH" => Some(Self::MediumSmooth),
            "SMOOTH" => Some(Self::Smooth),
            "VERYSMOOTH" => Some(Self::VerySmooth),
            _ => None,
        }
    }
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
    /// Surface roughness used by exterior convection algorithms.
    pub roughness: Option<MaterialSurfaceRoughness>,
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
    /// Thermal absorptance for exterior longwave heat-balance diagnostics.
    pub thermal_absorptance: Option<f64>,
    /// Solar absorptance for exterior solar heat-balance diagnostics.
    pub solar_absorptance: Option<f64>,
    /// Visible absorptance.
    pub visible_absorptance: Option<f64>,
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

/// Construction resolved to an ordered material layer stack.
#[derive(Clone, Debug, PartialEq)]
pub struct Construction {
    /// Typed ID.
    pub id: ConstructionId,
    /// Construction name.
    pub name: NormalizedName,
    /// Outside layer material.
    pub outside_layer: MaterialId,
    /// Ordered material layers from outside to inside.
    pub layers: Vec<MaterialId>,
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
