//! Material.cc1403-1489: genuine inherited MaterialDrapeEQL/TAR1 and pleat state.
use super::window_shade_equivalent_layer::SourceShadeEquivalentLayerMaterial;
use super::{MaterialDataOwners, SelectedMaterialData, SourceMaterialGroup, typed_id, unavailable};
use crate::error::RuntimeError;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError;
use ep_model::{MaterialKind, MaterialSurfaceRoughness};

// Native MaterialDrapeEQL derives the same ShadeEQL base: exactly one owned TAR1.
#[derive(Clone, Debug, Default, PartialEq)]
pub(super) struct SourceDrapeEquivalentLayerMaterial {
    pub base: SourceShadeEquivalentLayerMaterial,
    pub is_pleated: bool,
    pub pleated_width: f64,
    pub pleated_length: f64,
}

fn severe(owners: &mut MaterialDataOwners<'_, '_>, message: &str) -> Result<(), RuntimeError> {
    let flags = owners
        .diagnostic_flags
        .as_ref()
        .ok_or_else(|| unavailable("actual DrapeEQL diagnostic phase owners"))?;
    owners
        .backend
        .show_source_severe_error(message, flags)
        .map_err(|error| RuntimeError::HeatBalanceInput(Box::new(error)))
}

fn continuation(
    owners: &mut MaterialDataOwners<'_, '_>,
    message: &str,
) -> Result<(), RuntimeError> {
    owners
        .backend
        .show_continue_error(message)
        .map_err(|error| {
            RuntimeError::HeatBalanceInput(Box::new(HeatBalanceInputError::Backend(error)))
        })
}

impl SelectedMaterialData {
    pub(super) fn get_window_drape_equivalent_layer_data(
        &mut self,
        raw: &super::SelectedMaterialInput,
        owners: &mut MaterialDataOwners<'_, '_>,
    ) -> Result<(), RuntimeError> {
        owners.shortcut.current_module_object = "WindowMaterial:Drape:EquivalentLayer".into();
        self.num_eql_drapes = i32::try_from(
            owners
                .input
                .get_num_objects_found("WindowMaterial:Drape:EquivalentLayer"),
        )
        .map_err(|_| unavailable("DrapeEQL count exceeds Native integer"))?;
        if usize::try_from(self.num_eql_drapes).ok()
            != Some(raw.window_drape_equivalent_layer.len())
        {
            return Err(unavailable("DrapeEQL raw/IP count"));
        }
        for object in &raw.window_drape_equivalent_layer {
            let (alphas, numbers) = raw.get_window_drape_equivalent_layer_item(object, owners)?;
            // Generic min_fields4 includes Name; original required N2..N5 are
            // enforced by Source admission, not an invented primitive floor5.
            if alphas != 1 || !(3..=13).contains(&numbers) {
                return Err(unavailable("DrapeEQL1A13N/min4 primitive"));
            }
            let name = owners.shortcut.alpha_args[0].clone();
            if self.material_map.contains_key(&name) {
                severe(
                    owners,
                    &format!(
                        "GetMaterialData: WindowMaterial:Drape:EquivalentLayer = {name}, duplicate name."
                    ),
                )?;
                *owners.errors_found = true;
                continue;
            }
            let id = typed_id(
                owners.model,
                &name,
                MaterialKind::WindowDrapeEquivalentLayer,
            )?;
            self.insert(&name, id, SourceMaterialGroup::DrapeEql)?;
            let drape = self.window_drape_equivalent_layer.entry(id).or_default();
            let material = &mut drape.base;
            material.roughness = Some(MaterialSurfaceRoughness::MediumRough);
            self.resistance_only.insert(id, true);
            material.tar.sol.ft.bm[0].bm_tra = owners.shortcut.numeric_args[0];
            material.tar.sol.bk.bm[0].bm_tra = owners.shortcut.numeric_args[0];
            material.tar.sol.ft.bm[0].df_tra = owners.shortcut.numeric_args[1];
            material.tar.sol.bk.bm[0].df_tra = owners.shortcut.numeric_args[2];
            material.tar.sol.ft.bm[0].df_ref = owners.shortcut.numeric_args[3];
            material.tar.sol.bk.bm[0].df_ref = owners.shortcut.numeric_args[4];
            material.tar.vis.ft.bm[0].bm_tra = owners.shortcut.numeric_args[5];
            material.tar.vis.ft.bm[0].df_tra = owners.shortcut.numeric_args[6];
            material.tar.vis.ft.bm[0].df_ref = owners.shortcut.numeric_args[7];
            material.tar.ir.ft.tra = owners.shortcut.numeric_args[8];
            material.tar.ir.bk.tra = owners.shortcut.numeric_args[8];
            material.tar.ir.ft.emi = owners.shortcut.numeric_args[9];
            material.tar.ir.bk.emi = owners.shortcut.numeric_args[10];
            material.thermal_absorptance_front = material.tar.ir.ft.emi;
            material.thermal_absorptance_back = material.tar.ir.bk.emi;
            material.thermal_transmittance = material.tar.ir.ft.tra;
            if !owners.shortcut.numeric_field_blanks[11]
                && !owners.shortcut.numeric_field_blanks[12]
            {
                if owners.shortcut.numeric_args[11] != 0.0
                    && owners.shortcut.numeric_args[12] != 0.0
                {
                    drape.pleated_width = owners.shortcut.numeric_args[11];
                    drape.pleated_length = owners.shortcut.numeric_args[12];
                    drape.is_pleated = true;
                }
            } else {
                drape.is_pleated = false;
            }
            // All inherited TAR/directional/pleat writes precede these three checks.
            if owners.shortcut.numeric_args[0]
                + owners.shortcut.numeric_args[1]
                + owners.shortcut.numeric_args[3]
                >= 1.0
            {
                *owners.errors_found = true;
                severe(
                    owners,
                    &format!(
                        "WindowMaterial:Drape:EquivalentLayer=\"{name}\", Illegal value combination."
                    ),
                )?;
                continuation(
                    owners,
                    &format!(
                        "{} + {} + {}not < 1.0",
                        owners.shortcut.numeric_field_names[0],
                        owners.shortcut.numeric_field_names[1],
                        owners.shortcut.numeric_field_names[3]
                    ),
                )?;
            }
            if owners.shortcut.numeric_args[5]
                + owners.shortcut.numeric_args[6]
                + owners.shortcut.numeric_args[7]
                >= 1.0
            {
                *owners.errors_found = true;
                severe(
                    owners,
                    &format!(
                        "WindowMaterial:Drape:EquivalentLayer=\"{name}\", Illegal value combination."
                    ),
                )?;
                // Native deliberately names N4/N5/N6 for the N6/N7/N8 condition.
                continuation(
                    owners,
                    &format!(
                        "{} + {} + {}not < 1.0",
                        owners.shortcut.numeric_field_names[3],
                        owners.shortcut.numeric_field_names[4],
                        owners.shortcut.numeric_field_names[5]
                    ),
                )?;
            }
            if owners.shortcut.numeric_args[8] + owners.shortcut.numeric_args[9] > 1.0 {
                *owners.errors_found = true;
                severe(
                    owners,
                    &format!(
                        "WindowMaterial:Drape:EquivalentLayer=\"{name}\", Illegal value combination."
                    ),
                )?;
                continuation(
                    owners,
                    &format!(
                        "{} + {} not < 1.0",
                        owners.shortcut.numeric_field_names[8],
                        owners.shortcut.numeric_field_names[9]
                    ),
                )?;
            }
        }
        Ok(())
    }
}
