//! Actual DataSystemVariables.hh140-151 subset on the same SysVars owner.
//! Nine explicit constructors; the pinned production tree has no other writers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SourceThreadingSystemVariables {
    pub(crate) environment_set_threads: i32,
    pub(crate) energyplus_environment_set_threads: i32,
    pub(crate) idf_set_threads: i32,
    pub(crate) number_active_simulations: i32,
    pub(crate) number_active_simulations_set: bool,
    pub(crate) max_number_of_threads: i32,
    pub(crate) number_interior_radiant_threads: i32,
    pub(crate) nominal_total_surfaces: i32,
    pub(crate) enabled: bool,
}
impl Default for SourceThreadingSystemVariables {
    fn default() -> Self {
        Self {
            environment_set_threads: 0,
            energyplus_environment_set_threads: 0,
            idf_set_threads: 0,
            number_active_simulations: 1,
            number_active_simulations_set: false,
            max_number_of_threads: 1,
            number_interior_radiant_threads: 1,
            nominal_total_surfaces: 0,
            enabled: false,
        }
    }
}
