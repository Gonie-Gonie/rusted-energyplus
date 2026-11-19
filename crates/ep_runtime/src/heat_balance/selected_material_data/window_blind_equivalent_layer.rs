//! Material.cc2073-2223/Material.hh758-772: one genuine MaterialBlindEQL/TAR1 owner.
use super::window_shade_equivalent_layer::SourceShadeEquivalentLayerMaterial;
use super::{MaterialDataOwners, SelectedMaterialData, SourceMaterialGroup, typed_id, unavailable};
use crate::error::RuntimeError;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError;
use ep_model::{MaterialKind, MaterialSurfaceRoughness};

#[derive(Clone, Debug, PartialEq)]
pub(super) struct SourceBlindEquivalentLayerMaterial {
    pub base: SourceShadeEquivalentLayerMaterial,
    pub slat_width: f64,
    pub slat_separation: f64,
    pub slat_crown: f64,
    pub slat_angle: f64,
    pub slat_orientation: i32,
    pub slat_angle_type: i32,
}
impl Default for SourceBlindEquivalentLayerMaterial {
    fn default() -> Self {
        Self {
            base: SourceShadeEquivalentLayerMaterial::default(),
            slat_width: 0.0,
            slat_separation: 0.0,
            slat_crown: 0.0,
            slat_angle: 0.0,
            slat_orientation: -1,
            slat_angle_type: 0,
        }
    }
}
fn severe(owners: &mut MaterialDataOwners<'_, '_>, message: &str) -> Result<(), RuntimeError> {
    let flags = owners
        .diagnostic_flags
        .as_ref()
        .ok_or_else(|| unavailable("actual BlindEQL diagnostic phase owners"))?;
    owners
        .backend
        .show_source_severe_error(message, flags)
        .map_err(|error| RuntimeError::HeatBalanceInput(Box::new(error)))
}
fn warning(owners: &mut MaterialDataOwners<'_, '_>, message: &str) -> Result<(), RuntimeError> {
    let flags = owners
        .diagnostic_flags
        .as_ref()
        .ok_or_else(|| unavailable("actual BlindEQL warning phase owners"))?;
    owners
        .backend
        .show_source_warning_error(message, flags)
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
fn rounded(value: f64) -> Result<String, RuntimeError> {
    super::super::source_round_format::round(value, 2).map_err(|error| {
        RuntimeError::HeatBalanceInput(Box::new(HeatBalanceInputError::Format(format!(
            "BlindEQL Native R formatting: {error:?}"
        ))))
    })
}
impl SelectedMaterialData {
    pub(super) fn get_window_blind_equivalent_layer_data(
        &mut self,
        raw: &super::SelectedMaterialInput,
        owners: &mut MaterialDataOwners<'_, '_>,
    ) -> Result<(), RuntimeError> {
        owners.shortcut.current_module_object = "WindowMaterial:Blind:EquivalentLayer".into();
        self.num_eql_blinds = i32::try_from(
            owners
                .input
                .get_num_objects_found("WindowMaterial:Blind:EquivalentLayer"),
        )
        .map_err(|_| unavailable("BlindEQL count exceeds Native integer"))?;
        if usize::try_from(self.num_eql_blinds).ok()
            != Some(raw.window_blind_equivalent_layer.len())
        {
            return Err(unavailable("BlindEQL raw/IP count"));
        }
        for object in &raw.window_blind_equivalent_layer {
            let (alphas, numbers) = raw.get_window_blind_equivalent_layer_item(object, owners)?;
            let invalid_counts = match raw.source_original_idf_fixed_counts(
                "WindowMaterial:Blind:EquivalentLayer",
                object,
                owners,
            )? {
                Some(expected) => (alphas, numbers) != expected,
                None => {
                    !(2..=3).contains(&alphas)
                        || !(15..=21).contains(&numbers)
                        || (alphas == 3 && numbers != 21)
                }
            };
            if invalid_counts {
                return Err(unavailable("BlindEQL3A21N/min10 legal15..21N primitive"));
            }
            let name = owners.shortcut.alpha_args[0].clone();
            if self.material_map.contains_key(&name) {
                severe(
                    owners,
                    &format!(
                        "GetMaterialData: WindowMaterial:Blind:EquivalentLayer = {name}, duplicate name."
                    ),
                )?;
                *owners.errors_found = true;
                continue;
            }
            let id = typed_id(
                owners.model,
                &name,
                MaterialKind::WindowBlindEquivalentLayer,
            )?;
            self.insert(&name, id, SourceMaterialGroup::BlindEql)?;
            let row = self.window_blind_equivalent_layer.entry(id).or_default();
            row.base.roughness = Some(MaterialSurfaceRoughness::Rough);
            self.resistance_only.insert(id, true);
            row.slat_orientation = match owners.shortcut.alpha_args[1].as_str() {
                "HORIZONTAL" => 0,
                "VERTICAL" => 1,
                _ => -1,
            };
            row.slat_width = owners.shortcut.numeric_args[0];
            row.slat_separation = owners.shortcut.numeric_args[1];
            row.slat_crown = owners.shortcut.numeric_args[2];
            row.slat_angle = owners.shortcut.numeric_args[3];
            row.base.tar.sol.ft.bm[0].df_tra = owners.shortcut.numeric_args[4];
            row.base.tar.sol.bk.bm[0].df_tra = owners.shortcut.numeric_args[5];
            row.base.tar.sol.ft.bm[0].df_ref = owners.shortcut.numeric_args[6];
            row.base.tar.sol.bk.bm[0].df_ref = owners.shortcut.numeric_args[7];
            if (8_usize..12).all(|i| !owners.shortcut.numeric_field_blanks[i]) {
                row.base.tar.vis.ft.bm[0].df_tra = owners.shortcut.numeric_args[8];
                row.base.tar.vis.bk.bm[0].df_tra = owners.shortcut.numeric_args[9];
                row.base.tar.vis.ft.bm[0].df_ref = owners.shortcut.numeric_args[10];
                row.base.tar.vis.bk.bm[0].df_ref = owners.shortcut.numeric_args[11];
            }
            if (12_usize..15).all(|i| !owners.shortcut.numeric_field_blanks[i]) {
                row.base.tar.sol.ft.df.tra = owners.shortcut.numeric_args[12];
                row.base.tar.sol.ft.df.refl = owners.shortcut.numeric_args[13];
                row.base.tar.sol.bk.df.refl = owners.shortcut.numeric_args[14];
            }
            if (15_usize..18).all(|i| !owners.shortcut.numeric_field_blanks[i]) {
                // Native intentionally guards16..18 but copies RAW13..15 independently.
                row.base.tar.vis.ft.df.tra = owners.shortcut.numeric_args[12];
                row.base.tar.vis.ft.df.refl = owners.shortcut.numeric_args[13];
                row.base.tar.vis.bk.df.refl = owners.shortcut.numeric_args[14];
            }
            if !owners.shortcut.numeric_field_blanks[18] {
                row.base.tar.ir.ft.tra = owners.shortcut.numeric_args[18];
                row.base.tar.ir.bk.tra = owners.shortcut.numeric_args[18];
            }
            if !owners.shortcut.numeric_field_blanks[19] {
                row.base.tar.ir.ft.emi = owners.shortcut.numeric_args[19];
            }
            if !owners.shortcut.numeric_field_blanks[20] {
                row.base.tar.ir.bk.emi = owners.shortcut.numeric_args[20];
            }
            row.base.thermal_absorptance_front = row.base.tar.ir.ft.emi;
            row.base.thermal_absorptance_back = row.base.tar.ir.bk.emi;
            row.base.thermal_transmittance = row.base.tar.ir.ft.tra;
            row.slat_angle_type = 0;
            if !owners.shortcut.alpha_field_blanks[2] {
                row.slat_angle_type = match owners.shortcut.alpha_args[2].as_str() {
                    "FIXEDSLATANGLE" => 0,
                    "MAXIMIZESOLAR" => 1,
                    "BLOCKBEAMSOLAR" => 2,
                    _ => -1,
                };
            }
            if row.slat_width < row.slat_separation {
                warning(
                    owners,
                    &format!(
                        "WindowMaterial:Blind:EquivalentLayer=\"{name}\", Slat Separation/Width"
                    ),
                )?;
                let width = rounded(row.slat_width)?;
                let separation = rounded(row.slat_separation)?;
                continuation(
                    owners,
                    &format!(
                        "{} [{width}] is less than {} [{separation}].",
                        owners.shortcut.numeric_field_names[0],
                        owners.shortcut.numeric_field_names[1]
                    ),
                )?;
                continuation(
                    owners,
                    "This will allow direct beam to be transmitted when Slat angle = 0.",
                )?;
            }
            if row.slat_separation < 0.001 {
                warning(
                    owners,
                    &format!("WindowMaterial:Blind:EquivalentLayer=\"{name}\", Slat Separation"),
                )?;
                let separation = rounded(row.slat_separation)?;
                continuation(
                    owners,
                    &format!(
                        "{} [{separation}]. Slate spacing must be > 0.0",
                        owners.shortcut.numeric_field_names[1]
                    ),
                )?;
                continuation(
                    owners,
                    "...Setting slate spacing to default value of 0.025 m and simulation continues.",
                )?;
                row.slat_separation = 0.025;
            }
            if row.slat_width < 0.001 || row.slat_width >= 2.0 * row.slat_separation {
                warning(
                    owners,
                    &format!("WindowMaterial:Blind:EquivalentLayer=\"{name}\", Slat Width"),
                )?;
                let width = rounded(row.slat_width)?;
                continuation(
                    owners,
                    &format!(
                        "{} [{width}]. Slat width range is 0 < Width <= 2*Spacing",
                        owners.shortcut.numeric_field_names[0]
                    ),
                )?;
                continuation(
                    owners,
                    "...Setting slate width equal to slate spacing and simulation continues.",
                )?;
                row.slat_width = row.slat_separation;
            }
            if row.slat_crown < 0.0 || row.slat_crown >= 0.5 * row.slat_width {
                warning(
                    owners,
                    &format!("WindowMaterial:Blind:EquivalentLayer=\"{name}\", Slat Crown"),
                )?;
                let crown = rounded(row.slat_crown)?;
                continuation(
                    owners,
                    &format!(
                        "{} [{crown}]. Slat crwon range is 0 <= crown < 0.5*Width",
                        owners.shortcut.numeric_field_names[2]
                    ),
                )?;
                continuation(
                    owners,
                    "...Setting slate crown to 0.0 and simulation continues.",
                )?;
                row.slat_crown = 0.0;
            }
            // All admitted original angles are finite; inclusive Native endpoints stay legal.
            if !(-90.0..=90.0).contains(&row.slat_angle) {
                warning(
                    owners,
                    &format!("WindowMaterial:Blind:EquivalentLayer=\"{name}\", Slat Angle"),
                )?;
                let angle = rounded(row.slat_angle)?;
                continuation(
                    owners,
                    &format!(
                        "{} [{angle}]. Slat angle range is -90.0 <= Angle < 90.0",
                        owners.shortcut.numeric_field_names[3]
                    ),
                )?;
                continuation(
                    owners,
                    "...Setting slate angle to 0.0 and simulation continues.",
                )?;
                row.slat_angle = 0.0;
            }
            for (a, b) in [(4_usize, 6_usize), (5, 7), (8, 10), (9, 11)] {
                if owners.shortcut.numeric_args[a] + owners.shortcut.numeric_args[b] >= 1.0 {
                    *owners.errors_found = true;
                    severe(
                        owners,
                        &format!(
                            "WindowMaterial:Blind:EquivalentLayer=\"{name}\", Illegal value combination."
                        ),
                    )?;
                    continuation(
                        owners,
                        &format!(
                            "{} + {} not < 1.0",
                            owners.shortcut.numeric_field_names[a],
                            owners.shortcut.numeric_field_names[b]
                        ),
                    )?;
                }
            }
        }
        Ok(())
    }
}
