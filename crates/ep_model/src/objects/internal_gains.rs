use crate::{InternalGainId, NormalizedName, ScheduleId, ZoneId, ZoneListId};

/// OtherEquipment design-level calculation method.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OtherEquipmentDesignLevelCalculationMethod {
    /// Use the Design Level field directly.
    EquipmentLevel,
    /// Multiply Power per Floor Area by the target zone floor area.
    WattsPerZoneFloorArea,
    /// Multiply Power per Person by the target zone design occupant count.
    WattsPerPerson,
}

/// Electric or process equipment represented as a zone internal gain.
#[derive(Clone, Debug, PartialEq)]
pub struct OtherEquipment {
    /// Typed ID.
    pub id: InternalGainId,
    /// Equipment name.
    pub name: NormalizedName,
    /// Fuel type for meters/reporting. Heat balance source terms are independent of this value.
    pub fuel_type: NormalizedName,
    /// Target zone.
    pub zone: ZoneId,
    /// Actual availability/level identity, separate from user-authored IDs.
    pub schedule: Option<InternalGainScheduleReference>,
    /// Method used to calculate the design-level heat gain.
    pub design_level_calculation_method: OtherEquipmentDesignLevelCalculationMethod,
    /// Design-level heat gain in watts.
    pub design_level_w: f64,
    /// Power per zone floor area in W/m2 for Watts/Area sizing.
    pub power_per_floor_area_w_per_m2: f64,
    /// Power per person in W/person for Watts/Person sizing.
    pub power_per_person_w: f64,
    /// Fraction of gain emitted as latent load.
    pub fraction_latent: f64,
    /// Fraction of gain emitted as radiant load.
    pub fraction_radiant: f64,
    /// Fraction of gain lost outside the zone air balance.
    pub fraction_lost: f64,
    /// CO2 generation rate per unit of power input in m3/s-W.
    pub carbon_dioxide_generation_rate_m3_per_s_w: f64,
}

/// Resolved raw internal-gain target for explicit Source input only.
/// These are typed input identities, not Native Source array numbers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SourceGainInputTarget {
    /// Native lookup found a genuine Zone before considering lists.
    Zone(ZoneId),
    /// Native lookup found a genuine ZoneList after Zone/Space lookup.
    ZoneList(ZoneListId),
}

/// Existing OtherEquipment input-target API over the shared actual identity.
pub type SourceOtherEquipmentInputTarget = SourceGainInputTarget;

/// People input targets share the same genuine Zone and ZoneList identities.
pub type SourcePeopleInputTarget = SourceGainInputTarget;

/// Explicit raw People object and its actual dense typed instance range.
/// Generated names may coincide; the ordered range preserves every owner.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourcePeopleInput {
    /// Exact original raw People key, separate from generated instance names.
    pub object_name: String,
    /// Actual resolved Zone or ZoneList identity.
    pub target: SourcePeopleInputTarget,
    /// Contiguous real IDs in TypedModel.people, not Native array numbers.
    pub instances: std::ops::Range<usize>,
}

/// Explicit raw-object association with actual dense typed gain instances.
/// Instance names and member Zone IDs reside in the referenced arena rows.
/// This range is not Native misc.spaceStartPtr, which has a different traversal.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceOtherEquipmentInput {
    /// Exact original raw object key, independent of generated instance names.
    pub object_name: String,
    /// Actual resolved Zone or ZoneList identity.
    pub target: SourceOtherEquipmentInputTarget,
    /// Contiguous range in TypedModel.other_equipment with real dense typed IDs.
    pub instances: std::ops::Range<usize>,
}

/// `People` object number-of-people calculation method.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PeopleNumberCalculationMethod {
    /// Direct design occupant count.
    People,
    /// Occupants per floor area.
    PeoplePerArea,
    /// Floor area per occupant.
    AreaPerPerson,
}

/// Resolved internal-gain schedule identity for a Source input object.
/// Built-ins remain separate from the user-authored schedule namespace and IDs.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InternalGainScheduleReference {
    /// User-authored schedule in the shared typed schedule namespace.
    User(ScheduleId),
    /// EnergyPlus's genuine built-in `Constant-0.0` schedule.
    ConstantZero,
    /// EnergyPlus's genuine built-in `Constant-1.0` schedule.
    ConstantOne,
}

/// Existing People occupancy schedule API, sharing the genuine identity type.
pub type PeopleScheduleReference = InternalGainScheduleReference;

/// Occupant internal gain represented as a design people count.
#[derive(Clone, Debug, PartialEq)]
pub struct People {
    /// Typed ID.
    pub id: InternalGainId,
    /// People object name.
    pub name: NormalizedName,
    /// Target zone.
    pub zone: ZoneId,
    /// Number-of-people schedule; absence remains distinct from either built-in.
    pub number_of_people_schedule: Option<PeopleScheduleReference>,
    /// Design people calculation method.
    pub number_of_people_calculation_method: PeopleNumberCalculationMethod,
    /// Direct design people count.
    pub number_of_people: f64,
    /// Occupants per floor area in person/m2.
    pub people_per_floor_area: f64,
    /// Floor area per occupant in m2/person.
    pub floor_area_per_person: f64,
}
