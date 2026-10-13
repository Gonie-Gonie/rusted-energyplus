//! Target Construction Type* assignments at the actual GetConstructData1795 site.
//! This is not the complete CheckAndSet optical/scalar-property routine.
use super::super::selected_material_data::{SelectedMaterialData, SourceMaterialGroup as Group};
use super::{HeatBalanceInputError as Error, SelectedConstructionUsage};

impl SelectedConstructionUsage {
    pub(super) fn write_selected_type_predicates(
        &mut self,
        materials: &SelectedMaterialData,
    ) -> Result<(), Error> {
        // Native357/361 return before TypeIsWindow reset or any target writes.
        if self.total_layers == 0 {
            return Ok(());
        }
        let count = usize::try_from(self.total_layers)
            .map_err(|_| Error::Unavailable("Construction TotLayers bounds"))?;
        let inside = *self
            .layer_points
            .get(count - 1)
            .ok_or(Error::Unavailable("Construction InsideLayer bounds"))?;
        if inside <= 0 {
            return Ok(());
        }
        // Original392 resets Window only. Each nonzero layer ASSIGNS the
        // predicate at399-405; do not turn this into an accumulating OR.
        self.type_is_window = false;
        for point in &self.layer_points[..count] {
            if *point == 0 {
                continue;
            }
            let group = materials.source_group_at(*point)?;
            self.type_is_window = group.is_window();
            if !self.type_is_window && !group.is_not_window() {
                return Err(Error::Unavailable(
                    "native unclassified material group assertion",
                ));
            }
        }
        if inside == 0 {
            return Ok(());
        }
        let outside = self.layer_points[0];
        if outside == 0 {
            return Ok(());
        }
        if self.type_is_window {
            return Err(Error::Unavailable(
                "positive CheckAndSet window optical branch",
            ));
        }
        let outside_group = materials.source_group_at(outside)?;
        // Earlier non-target optical/thermal/roughness assignments retain the
        // existing qualified property producer. No duplicate physical cache is
        // computed here. AirGap producers/diagnostics are not admitted.
        if outside_group == Group::AirGap || materials.source_group_at(inside)? == Group::AirGap {
            return Err(Error::Unavailable(
                "positive CheckAndSet AirGap diagnostics",
            ));
        }
        // EcoRoof/IRT are conditional true writes, not false resets. Their
        // selected getters admit neither family; no inferred false is passed.
        if outside_group == Group::EcoRoof {
            self.type_is_eco_roof = true;
            return Err(Error::Unavailable(
                "positive CheckAndSet EcoRoof layer diagnostics",
            ));
        }
        if outside_group == Group::IrTransparent {
            self.type_is_irt = true;
            if self.total_layers != 1 {
                return Err(Error::Unavailable(
                    "positive CheckAndSet IRT layer diagnostics",
                ));
            }
        }
        Ok(())
    }
}
