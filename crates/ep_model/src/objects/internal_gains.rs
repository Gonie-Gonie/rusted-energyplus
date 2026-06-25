use crate::{InternalGainId, NormalizedName, ScheduleId, ZoneId};

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

/// Occupant internal gain represented as a design people count.
#[derive(Clone, Debug, PartialEq)]
pub struct People {
    /// Typed ID.
    pub id: InternalGainId,
    /// People object name.
    pub name: NormalizedName,
    /// Target zone.
    pub zone: ZoneId,
    /// Number-of-people schedule.
    pub number_of_people_schedule: Option<ScheduleId>,
    /// Design people calculation method.
    pub number_of_people_calculation_method: PeopleNumberCalculationMethod,
    /// Direct design people count.
    pub number_of_people: f64,
    /// Occupants per floor area in person/m2.
    pub people_per_floor_area: f64,
    /// Floor area per occupant in m2/person.
    pub floor_area_per_person: f64,
}
