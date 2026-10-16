//! Stateless per-call access to the SAME property state used by weather wrappers.
//! No numerical/cache owner, copied values, constructor reset or held RefCell loan.
use super::{EnergyPlusPsychrometricFunction, EnergyPlusPsychrometricOperation};

/// A transport handle only. The one real PROPERTY_STATE remains in psy02_state.
/// Actual caller must retain a single whole-run scope; this type creates no scope.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct SourcePsychrometricAccess;
impl SourcePsychrometricAccess {
    #[track_caller]
    pub(crate) fn evaluate(
        &mut self,
        function: EnergyPlusPsychrometricFunction,
        inputs: &[f64],
    ) -> Result<EnergyPlusPsychrometricOperation, &'static str> {
        super::psy02_state::with_state(|state| state.evaluate(function, inputs))
    }
}
