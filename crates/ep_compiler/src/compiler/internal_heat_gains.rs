//! Bounded internal-heat-gain input sequencing.

use super::{Compiler, DiagnosticSeverity};
use ep_model::TypedModel;

impl Compiler<'_> {
    /// Parses the currently typed internal-gain families in source order.
    ///
    /// This bounded pass only projects direct-Zone `People` and
    /// `OtherEquipment` inputs. A pre-existing error suppresses the entire pass,
    /// while an error raised by `People` does not suppress the subsequent
    /// `OtherEquipment` scan.
    pub(super) fn parse_bounded_internal_heat_gains_input(&mut self, model: &mut TypedModel) {
        if self
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == DiagnosticSeverity::Error)
        {
            return;
        }

        self.parse_people(model);
        // Deferred here: Lights, Electric/Gas/HotWater/Steam equipment.
        self.parse_other_equipment(model);
        // Deferred here: ITE, outdoor-temperature baseboard, and CO2 gains.
    }
}
