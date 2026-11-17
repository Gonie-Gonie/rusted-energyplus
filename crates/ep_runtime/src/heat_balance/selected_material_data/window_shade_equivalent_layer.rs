//! Material.cc1313-1400: genuine MaterialShadeEQL/TAR1 and Native diagnostic order.
use super::window_blind::SourceBlindTraAbsRef;
use super::{MaterialDataOwners, SelectedMaterialData, SourceMaterialGroup, typed_id, unavailable};
use crate::error::RuntimeError;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError;
use ep_model::{MaterialKind, MaterialSurfaceRoughness};

#[derive(Clone, Debug, Default, PartialEq)]
pub(super) struct SourceShadeEquivalentLayerMaterial {
    pub roughness: Option<MaterialSurfaceRoughness>,
    pub thickness: f64,
    pub conductivity: f64,
    pub resistance: f64,
    pub density: f64,
    pub specific_heat: f64,
    pub thermal_absorptance: f64,
    pub thermal_absorptance_input: f64,
    pub solar_absorptance: f64,
    pub solar_absorptance_input: f64,
    pub visible_absorptance: f64,
    pub visible_absorptance_input: f64,
    pub transmittance: f64,
    pub visible_transmittance: f64,
    pub thermal_transmittance: f64,
    pub shade_reflectance: f64,
    pub shade_visible_reflectance: f64,
    pub to_glass_distance: f64,
    pub top_opening: f64,
    pub bottom_opening: f64,
    pub left_opening: f64,
    pub right_opening: f64,
    pub airflow_permeability: f64,
    pub thermal_absorptance_front: f64,
    pub thermal_absorptance_back: f64,
    pub tar: SourceBlindTraAbsRef<1>,
}

fn severe(owners: &mut MaterialDataOwners<'_, '_>, message: &str) -> Result<(), RuntimeError> {
    let flags = owners
        .diagnostic_flags
        .as_ref()
        .ok_or_else(|| unavailable("actual ShadeEQL diagnostic phase owners"))?;
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
    pub(super) fn get_window_shade_equivalent_layer_data(
        &mut self,
        raw: &super::SelectedMaterialInput,
        owners: &mut MaterialDataOwners<'_, '_>,
    ) -> Result<(), RuntimeError> {
        owners.shortcut.current_module_object = "WindowMaterial:Shade:EquivalentLayer".into();
        self.num_eql_shades = i32::try_from(
            owners
                .input
                .get_num_objects_found("WindowMaterial:Shade:EquivalentLayer"),
        )
        .map_err(|_| unavailable("ShadeEQL count exceeds Native integer"))?;
        if usize::try_from(self.num_eql_shades).ok()
            != Some(raw.window_shade_equivalent_layer.len())
        {
            return Err(unavailable("ShadeEQL raw/IP count"));
        }
        for object in &raw.window_shade_equivalent_layer {
            let (alphas, numbers) = raw.get_window_shade_equivalent_layer_item(object, owners)?;
            if alphas != 1 || !(5..=11).contains(&numbers) {
                return Err(unavailable("ShadeEQL1A11N/min6 primitive"));
            }
            let name = owners.shortcut.alpha_args[0].clone();
            if self.material_map.contains_key(&name) {
                // Native duplicate diagnostic IO precedes flag mutation/allocation.
                severe(
                    owners,
                    &format!(
                        "GetMaterialData: WindowMaterial:Shade:EquivalentLayer = {name}, duplicate name."
                    ),
                )?;
                *owners.errors_found = true;
                continue;
            }
            let id = typed_id(
                owners.model,
                &name,
                MaterialKind::WindowShadeEquivalentLayer,
            )?;
            self.insert(&name, id, SourceMaterialGroup::ShadeEql)?;
            let material = self.window_shade_equivalent_layer.entry(id).or_default();
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
            // All Native property writes precede all five flag/diagnostic branches.
            for (a, b, c) in [(0, 1, 3), (0, 2, 4), (5, 6, 7)] {
                if owners.shortcut.numeric_args[a]
                    + owners.shortcut.numeric_args[b]
                    + owners.shortcut.numeric_args[c]
                    >= 1.0
                {
                    *owners.errors_found = true;
                    severe(
                        owners,
                        &format!(
                            "WindowMaterial:Shade:EquivalentLayer=\"{name}\", Illegal value combination."
                        ),
                    )?;
                    // Native intentionally omits the space before "not" here.
                    continuation(
                        owners,
                        &format!(
                            "{} + {} + {}not < 1.0",
                            owners.shortcut.numeric_field_names[a],
                            owners.shortcut.numeric_field_names[b],
                            owners.shortcut.numeric_field_names[c]
                        ),
                    )?;
                }
            }
            for b in [9, 10] {
                if owners.shortcut.numeric_args[8] + owners.shortcut.numeric_args[b] >= 1.0 {
                    *owners.errors_found = true;
                    severe(
                        owners,
                        &format!(
                            "WindowMaterial:Shade:EquivalentLayer=\"{name}\", Illegal value combination."
                        ),
                    )?;
                    continuation(
                        owners,
                        &format!(
                            "{} + {} not < 1.0",
                            owners.shortcut.numeric_field_names[8],
                            owners.shortcut.numeric_field_names[b]
                        ),
                    )?;
                }
            }
        }
        Ok(())
    }
}
