//! Material.cc1672-1805 and Material.hh733-742: actual ScreenEQL owner and recovery.
use super::window_shade_equivalent_layer::SourceShadeEquivalentLayerMaterial;
use super::{MaterialDataOwners, SelectedMaterialData, SourceMaterialGroup, typed_id, unavailable};
use crate::error::RuntimeError;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError;
use ep_model::{MaterialKind, MaterialSurfaceRoughness};

#[derive(Clone, Debug, Default, PartialEq)]
pub(super) struct SourceScreenEquivalentLayerMaterial {
    pub base: SourceShadeEquivalentLayerMaterial,
    pub wire_spacing: f64,
    pub wire_diameter: f64,
}
fn severe(owners: &mut MaterialDataOwners<'_, '_>, message: &str) -> Result<(), RuntimeError> {
    let flags = owners
        .diagnostic_flags
        .as_ref()
        .ok_or_else(|| unavailable("actual ScreenEQL diagnostic phase owners"))?;
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
fn rounded(value: f64) -> Result<String, RuntimeError> {
    super::super::source_round_format::round(value, 4).map_err(|error| {
        RuntimeError::HeatBalanceInput(Box::new(HeatBalanceInputError::Format(format!(
            "ScreenEQL Native R formatting: {error:?}"
        ))))
    })
}
impl SelectedMaterialData {
    pub(super) fn get_window_screen_equivalent_layer_data(
        &mut self,
        raw: &super::SelectedMaterialInput,
        owners: &mut MaterialDataOwners<'_, '_>,
    ) -> Result<(), RuntimeError> {
        owners.shortcut.current_module_object = "WindowMaterial:Screen:EquivalentLayer".into();
        self.num_eql_screens = i32::try_from(
            owners
                .input
                .get_num_objects_found("WindowMaterial:Screen:EquivalentLayer"),
        )
        .map_err(|_| unavailable("ScreenEQL count exceeds Native integer"))?;
        if usize::try_from(self.num_eql_screens).ok()
            != Some(raw.window_screen_equivalent_layer.len())
        {
            return Err(unavailable("ScreenEQL raw/IP count"));
        }
        for object in &raw.window_screen_equivalent_layer {
            if raw.get_window_screen_equivalent_layer_item(object, owners)? != (1, 10) {
                return Err(unavailable("ScreenEQL1A10N/min11 primitive"));
            }
            let name = owners.shortcut.alpha_args[0].clone();
            if self.material_map.contains_key(&name) {
                severe(
                    owners,
                    &format!(
                        "GetMaterialData: WindowMaterial:Screen:EquivalentLayer = {name}, duplicate name."
                    ),
                )?;
                *owners.errors_found = true;
                continue;
            }
            let id = typed_id(
                owners.model,
                &name,
                MaterialKind::WindowScreenEquivalentLayer,
            )?;
            self.insert(&name, id, SourceMaterialGroup::ScreenEql)?;
            let row = self.window_screen_equivalent_layer.entry(id).or_default();
            row.base.roughness = Some(MaterialSurfaceRoughness::MediumRough);
            self.resistance_only.insert(id, true);
            row.base.tar.sol.ft.bm[0].bm_tra = owners.shortcut.numeric_args[0];
            row.base.tar.sol.bk.bm[0].bm_tra = owners.shortcut.numeric_args[0];
            row.base.tar.sol.ft.bm[0].df_tra = owners.shortcut.numeric_args[1];
            row.base.tar.sol.bk.bm[0].df_tra = owners.shortcut.numeric_args[1];
            row.base.tar.sol.ft.bm[0].df_ref = owners.shortcut.numeric_args[2];
            row.base.tar.sol.bk.bm[0].df_ref = owners.shortcut.numeric_args[2];
            row.base.tar.vis.ft.bm[0].bm_tra = owners.shortcut.numeric_args[3];
            row.base.tar.vis.ft.bm[0].df_tra = owners.shortcut.numeric_args[4];
            row.base.tar.vis.ft.df.refl = owners.shortcut.numeric_args[5];
            row.base.tar.ir.ft.tra = owners.shortcut.numeric_args[6];
            row.base.tar.ir.bk.tra = owners.shortcut.numeric_args[6];
            row.base.tar.ir.ft.emi = owners.shortcut.numeric_args[7];
            row.base.tar.ir.bk.emi = owners.shortcut.numeric_args[7];
            row.base.thermal_absorptance_front = row.base.tar.ir.ft.emi;
            row.base.thermal_absorptance_back = row.base.tar.ir.bk.emi;
            row.base.thermal_transmittance = row.base.tar.ir.ft.tra;
            if owners.shortcut.numeric_args[2] < 0.0 || owners.shortcut.numeric_args[2] > 1.0 {
                *owners.errors_found = true;
                severe(
                    owners,
                    &format!("WindowMaterial:Screen:EquivalentLayer=\"{name}\", Illegal value."),
                )?;
                continuation(
                    owners,
                    &format!(
                        "{} must be >= 0 and <= 1",
                        owners.shortcut.numeric_field_names[2]
                    ),
                )?;
            }
            if owners.shortcut.numeric_args[5] < 0.0 || owners.shortcut.numeric_args[5] > 1.0 {
                *owners.errors_found = true;
                severe(
                    owners,
                    &format!("WindowMaterial:Screen:EquivalentLayer=\"{name}\", Illegal value."),
                )?;
                continuation(
                    owners,
                    &format!(
                        "{} must be >= 0 and <= 1 for material {name}.",
                        owners.shortcut.numeric_field_names[5]
                    ),
                )?;
            }
            if !owners.shortcut.numeric_field_blanks[8] {
                if owners.shortcut.numeric_args[8] > 0.00001 {
                    row.wire_spacing = owners.shortcut.numeric_args[8];
                } else {
                    severe(
                        owners,
                        &format!(
                            "WindowMaterial:Screen:EquivalentLayer=\"{name}\", Illegal value."
                        ),
                    )?;
                    continuation(
                        owners,
                        &format!("{} must be > 0.", owners.shortcut.numeric_field_names[8]),
                    )?;
                    continuation(
                        owners,
                        "...Setting screen wire spacing to a default value of 0.025m and simulation continues.",
                    )?;
                    row.wire_spacing = 0.025;
                }
            }
            if !owners.shortcut.numeric_field_blanks[9] {
                if owners.shortcut.numeric_args[9] > 0.00001
                    && owners.shortcut.numeric_args[9] < row.wire_spacing
                {
                    row.wire_diameter = owners.shortcut.numeric_args[9];
                } else {
                    severe(
                        owners,
                        &format!(
                            "WindowMaterial:Screen:EquivalentLayer=\"{name}\", Illegal value."
                        ),
                    )?;
                    continuation(
                        owners,
                        &format!("{} must be > 0.", owners.shortcut.numeric_field_names[9]),
                    )?;
                    continuation(
                        owners,
                        "...Setting screen wire diameter to a default value of 0.005m and simulation continues.",
                    )?;
                    row.wire_diameter = 0.005;
                }
            }
            if row.wire_spacing > 0.0 {
                if row.wire_diameter / row.wire_spacing >= 1.0 {
                    *owners.errors_found = true;
                    severe(
                        owners,
                        &format!(
                            "WindowMaterial:Screen:EquivalentLayer=\"{name}\", Illegal value combination."
                        ),
                    )?;
                    continuation(
                        owners,
                        &format!(
                            "{} must be less than {}",
                            owners.shortcut.numeric_field_names[9],
                            owners.shortcut.numeric_field_names[8]
                        ),
                    )?;
                } else {
                    let t = 1.0 - row.wire_diameter / row.wire_spacing;
                    let openness = t * t; // Native pow_2, preserving expression/operand order.
                    if (row.base.tar.sol.ft.bm[0].bm_tra - openness) / openness > 0.01 {
                        severe(
                            owners,
                            &format!(
                                "WindowMaterial:Screen:EquivalentLayer=\"{name}\", screen openness specified."
                            ),
                        )?;
                        continuation(
                            owners,
                            &format!(
                                "{} is > 1.0% of the value calculated from input fields:",
                                owners.shortcut.numeric_field_names[0]
                            ),
                        )?;
                        continuation(
                            owners,
                            &format!(
                                "{} and {}",
                                owners.shortcut.numeric_field_names[8],
                                owners.shortcut.numeric_field_names[9]
                            ),
                        )?;
                        continuation(owners, " using the formula (1-diameter/spacing)**2")?;
                        continuation(
                            owners,
                            " ...the screen diameter is recalculated from the material openness specified ",
                        )?;
                        continuation(
                            owners,
                            " ...and wire spacing using the formula = wire spacing * (1.0 - SQRT(Opennes))",
                        )?;
                        let root = ep_platform_math::sqrt(row.base.tar.sol.ft.bm[0].bm_tra)
                            .map_err(|error| {
                                unavailable(&format!("MaterialScreenEQL sqrt: {error}"))
                            })?;
                        row.wire_diameter = row.wire_spacing * (1.0 - root);
                        // A formatting or final-Continue failure retains that exact same-row write.
                        let diameter = rounded(row.wire_diameter)?;
                        continuation(
                            owners,
                            &format!(
                                " ...Recalculated {}={diameter} m",
                                owners.shortcut.numeric_field_names[9]
                            ),
                        )?;
                    }
                }
            }
            if row.base.tar.sol.ft.bm[0].bm_tra + row.base.tar.sol.ft.bm[0].df_ref >= 1.0 {
                *owners.errors_found = true;
                severe(
                    owners,
                    &format!(
                        "WindowMaterial:Screen:EquivalentLayer=\"{name}\", Illegal value combination."
                    ),
                )?;
                continuation(
                    owners,
                    "Calculated solar transmittance + solar reflectance not < 1.0",
                )?;
                continuation(
                    owners,
                    "See Engineering Reference for calculation procedure for solar transmittance.",
                )?;
            }
            if row.base.tar.vis.ft.bm[0].bm_tra + row.base.tar.vis.ft.df.refl >= 1.0 {
                *owners.errors_found = true;
                severe(
                    owners,
                    &format!(
                        "WindowMaterial:Screen:EquivalentLayer=\"{name}\", Illegal value combination."
                    ),
                )?;
                continuation(
                    owners,
                    "Calculated visible transmittance + visible reflectance not < 1.0",
                )?;
                continuation(
                    owners,
                    "See Engineering Reference for calculation procedure for visible solar transmittance.",
                )?;
            }
            // Native uses common base AbsorpThermal (still0), never directional N8.
            if row.base.thermal_transmittance + row.base.thermal_absorptance >= 1.0 {
                *owners.errors_found = true;
                severe(
                    owners,
                    &format!(
                        "WindowMaterial:Screen:EquivalentLayer=\"{name}\", Illegal value combination."
                    ),
                )?;
                severe(
                    owners,
                    "Thermal hemispherical emissivity plus open area fraction (1-diameter/spacing)**2 not < 1.0",
                )?;
            }
        }
        Ok(())
    }
}
