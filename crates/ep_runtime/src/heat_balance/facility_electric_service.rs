//! Actual SimulationManager222 selected electric-facility constructor storage.
//! The same value moves to the later gain provider; no reset or second owner.
/// Selected real electric facility fields used by the zone-gain initializer.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FacilityElectricService {
    pub(crate) new_environment_internal_gains_flag: bool,
    pub(crate) get_input_flag: bool,
    pub(crate) facility_power_in_transformer_present: bool,
    pub(crate) power_out_transformer: Option<()>,
    pub(crate) number_of_load_centers: usize,
    pub(crate) load_centers: Vec<()>,
}
impl FacilityElectricService {
    pub(crate) fn new() -> Self {
        Self {
            new_environment_internal_gains_flag: true,
            get_input_flag: true,
            facility_power_in_transformer_present: false,
            power_out_transformer: None,
            number_of_load_centers: 0,
            load_centers: Vec::new(),
        }
    }
}
