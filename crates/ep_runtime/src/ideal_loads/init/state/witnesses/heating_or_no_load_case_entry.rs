use ep_model::IdealLoadsAirSystemId;

use super::super::PurchasedAirRuntimeState;
use crate::ideal_loads::PurchasedAirCalcHeatingOrNoLoadCaseEntrySnapshot;

impl PurchasedAirRuntimeState {
    pub(in crate::ideal_loads) fn heating_or_no_load_case_entry_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcHeatingOrNoLoadCaseEntrySnapshot> {
        self.heating_or_no_load_case_entry_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_heating_or_no_load_case_entry_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        witness: Option<PurchasedAirCalcHeatingOrNoLoadCaseEntrySnapshot>,
    ) {
        match witness {
            Some(witness) => {
                self.heating_or_no_load_case_entry_latest_witnesses
                    .insert(system, witness);
            }
            None => {
                self.heating_or_no_load_case_entry_latest_witnesses
                    .remove(&system);
            }
        }
    }
}
