//! Material.cc1056-1157: actual GapEQL GasMix with shared row representation and exact diagnostic/global flag order.
use super::window_gas::SourceWindowGasMaterial;
use super::{MaterialDataOwners, SelectedMaterialData, SourceMaterialGroup, typed_id, unavailable};
use crate::error::RuntimeError;
use ep_model::{MaterialKind, MaterialSurfaceRoughness, WindowGapVentType, WindowGasType};

impl SelectedMaterialData {
    pub(super) fn get_window_gap_equivalent_layer_data(
        &mut self,
        raw: &super::SelectedMaterialInput,
        owners: &mut MaterialDataOwners<'_, '_>,
    ) -> Result<(), RuntimeError> {
        owners.shortcut.current_module_object = "WindowMaterial:Gap:EquivalentLayer".into();
        self.num_eql_gaps = super::count(owners.input, "WindowMaterial:Gap:EquivalentLayer")?;
        if usize::try_from(self.num_eql_gaps).ok() != Some(raw.window_gap_equivalent_layer.len()) {
            return Err(unavailable(
                "WindowMaterial:Gap:EquivalentLayer raw/IP count",
            ));
        }
        for object in &raw.window_gap_equivalent_layer {
            let (alphas, numbers) = raw.get_window_gap_equivalent_layer_item(object, owners)?;
            let invalid_counts = match raw.source_original_idf_fixed_counts(
                "WindowMaterial:Gap:EquivalentLayer",
                object,
                owners,
            )? {
                Some(expected) => (alphas, numbers) != expected,
                None => alphas != 3 || !(1..=12).contains(&numbers),
            };
            if invalid_counts {
                return Err(unavailable("GapEQL3A12N/min3 primitive"));
            }
            let name = owners.shortcut.alpha_args[0].clone();
            if self.material_map.contains_key(&name) {
                let flags = owners.diagnostic_flags.as_ref().ok_or_else(|| {
                    unavailable("actual GapEQL duplicate diagnostic phase owners")
                })?;
                owners
                    .backend
                    .show_source_severe_error(
                        &format!("GetMaterialData: WindowMaterial:Gap:EquivalentLayer = {name}, duplicate name."),
                        flags,
                    )
                    .map_err(|error| RuntimeError::HeatBalanceInput(Box::new(error)))?;
                *owners.errors_found = true;
                continue;
            }
            let id = typed_id(owners.model, &name, MaterialKind::WindowGapEquivalentLayer)?;
            let material = SourceWindowGasMaterial::default();
            let position = self.insert(&name, id, SourceMaterialGroup::WindowGapEql)?;
            self.window_gas.insert(id, material);
            let material = self
                .window_gas
                .get_mut(&id)
                .ok_or_else(|| unavailable("allocated GapEQL GasMix runtime row"))?;
            material.num_gases = 1;
            material.gas_fractions[0] = 1.0;
            let gas_type = WindowGasType::from_equivalent_layer_energyplus_name(
                &owners.shortcut.alpha_args[1],
            )
            .ok_or_else(|| unavailable("admitted actual uppercase GapEQL gas_type"))?;
            material.gases[0].gas_type = Some(gas_type);
            material.roughness = Some(MaterialSurfaceRoughness::MediumRough);
            material.thickness = owners.shortcut.numeric_args[0];
            self.resistance_only.insert(id, true);
            if gas_type != WindowGasType::Custom {
                material.gases[0].properties = gas_type
                    .standard_properties()
                    .ok_or_else(|| unavailable("actual named GapEQL gas record"))?;
            }
            // Native tests A2 blank but reads A3; both required legal original alphas are nonblank.
            if !owners.shortcut.alpha_field_blanks[1] {
                material.gap_vent_type = match owners.shortcut.alpha_args[2].as_str() {
                    "SEALED" => WindowGapVentType::Sealed,
                    "VENTEDINDOOR" => WindowGapVentType::VentedIndoor,
                    "VENTEDOUTDOOR" => WindowGapVentType::VentedOutdoor,
                    _ => return Err(unavailable("admitted actual GapEQL gap_vent_type")),
                };
            }
            if gas_type == WindowGasType::Custom {
                // Native1113-1123 repeats the same nine assignments three times before diagnostics.
                for _ in 0..3 {
                    material.gases[0].properties.conductivity.coefficient_a =
                        owners.shortcut.numeric_args[1];
                    material.gases[0].properties.conductivity.coefficient_b =
                        owners.shortcut.numeric_args[2];
                    material.gases[0].properties.conductivity.coefficient_c =
                        owners.shortcut.numeric_args[3];
                    material.gases[0].properties.viscosity.coefficient_a =
                        owners.shortcut.numeric_args[4];
                    material.gases[0].properties.viscosity.coefficient_b =
                        owners.shortcut.numeric_args[5];
                    material.gases[0].properties.viscosity.coefficient_c =
                        owners.shortcut.numeric_args[6];
                    material.gases[0].properties.specific_heat.coefficient_a =
                        owners.shortcut.numeric_args[7];
                    material.gases[0].properties.specific_heat.coefficient_b =
                        owners.shortcut.numeric_args[8];
                    material.gases[0].properties.specific_heat.coefficient_c =
                        owners.shortcut.numeric_args[9];
                }
                material.gases[0].properties.molecular_weight_g_per_mol =
                    owners.shortcut.numeric_args[10];
                material.gases[0].properties.specific_heat_ratio = owners.shortcut.numeric_args[11];
                if material.gases[0].properties.viscosity.coefficient_a <= 0.0 {
                    *owners.errors_found = true;
                    let flags = owners.diagnostic_flags.as_ref().ok_or_else(|| {
                        unavailable("actual Custom GapEQL diagnostic phase owners")
                    })?;
                    owners
                        .backend
                        .show_source_severe_error(
                            &format!(
                                "WindowMaterial:Gap:EquivalentLayer=\"{name}\", Illegal value."
                            ),
                            flags,
                        )
                        .map_err(|error| RuntimeError::HeatBalanceInput(Box::new(error)))?;
                    owners.backend.show_continue_error(
                        &format!("{} not > 0.0", owners.shortcut.numeric_field_names[4]),
                    ).map_err(|error| RuntimeError::HeatBalanceInput(Box::new(
                        crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError::Backend(error))))?;
                }
                if material.gases[0].properties.specific_heat.coefficient_a <= 0.0 {
                    *owners.errors_found = true;
                    let flags = owners.diagnostic_flags.as_ref().ok_or_else(|| {
                        unavailable("actual Custom GapEQL diagnostic phase owners")
                    })?;
                    owners
                        .backend
                        .show_source_severe_error(
                            &format!(
                                "WindowMaterial:Gap:EquivalentLayer=\"{name}\", Illegal value."
                            ),
                            flags,
                        )
                        .map_err(|error| RuntimeError::HeatBalanceInput(Box::new(error)))?;
                    owners.backend.show_continue_error(
                        &format!("{} not > 0.0", owners.shortcut.numeric_field_names[7]),
                    ).map_err(|error| RuntimeError::HeatBalanceInput(Box::new(
                        crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError::Backend(error))))?;
                }
                if material.gases[0].properties.molecular_weight_g_per_mol <= 0.0 {
                    *owners.errors_found = true;
                    let flags = owners.diagnostic_flags.as_ref().ok_or_else(|| {
                        unavailable("actual Custom GapEQL diagnostic phase owners")
                    })?;
                    owners
                        .backend
                        .show_source_severe_error(
                            &format!(
                                "WindowMaterial:Gap:EquivalentLayer=\"{name}\", Illegal value."
                            ),
                            flags,
                        )
                        .map_err(|error| RuntimeError::HeatBalanceInput(Box::new(error)))?;
                    owners.backend.show_continue_error(
                        &format!("{} not > 0.0", owners.shortcut.numeric_field_names[10]),
                    ).map_err(|error| RuntimeError::HeatBalanceInput(Box::new(
                        crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError::Backend(error))))?;
                }
            }
            if !*owners.errors_found {
                let denominator = material.gases[0].properties.conductivity.at_300_k();
                if denominator > 0.0 {
                    // No effective-Thickness positivity or derived-finiteness
                    // predicate exists here in Native. Base Resistance stays0.
                    self.nominal_r[position] = material.thickness / denominator;
                } else {
                    let flags = owners.diagnostic_flags.as_ref().ok_or_else(|| {
                        unavailable("actual GapEQL conductivity diagnostic phase owners")
                    })?;
                    owners
                        .backend
                        .show_source_severe_error(
                            &format!(
                                "WindowMaterial:Gap:EquivalentLayer=\"{name}\", Illegal value."
                            ),
                            flags,
                        )
                        .map_err(|error| RuntimeError::HeatBalanceInput(Box::new(error)))?;
                    let formatted = crate::heat_balance::source_round_format::round(denominator, 3)
                        .map_err(|_| unavailable("actual GapEQL denominator .3R formatter"))?;
                    owners.backend.show_continue_error(&format!(
                        "Nominal resistance of gap at room temperature calculated at a negative Conductivity=[{formatted}]."
                    )).map_err(|error| RuntimeError::HeatBalanceInput(Box::new(
                        crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError::Backend(error))))?;
                    *owners.errors_found = true;
                }
            }
        }
        Ok(())
    }
}
