//! Material.cc950-1052: actual single-Gas MaterialGasMix/base rows, not typed nominal data.
use super::{MaterialDataOwners, SelectedMaterialData, SourceMaterialGroup, typed_id, unavailable};
use crate::error::RuntimeError;
use ep_model::{
    MaterialKind, MaterialSurfaceRoughness, WindowGapVentType, WindowGasPolynomialCoefficients,
    WindowGasProperties, WindowGasType,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct SourceGasRecord {
    /// None is Native Invalid; Some(Custom) is the distinct constructor state.
    pub gas_type: Option<WindowGasType>,
    pub properties: WindowGasProperties,
}
impl Default for SourceGasRecord {
    fn default() -> Self {
        Self {
            gas_type: Some(WindowGasType::Custom),
            properties: WindowGasProperties {
                conductivity: WindowGasPolynomialCoefficients::default(),
                viscosity: WindowGasPolynomialCoefficients::default(),
                specific_heat: WindowGasPolynomialCoefficients::default(),
                molecular_weight_g_per_mol: 0.0,
                specific_heat_ratio: 0.0,
            },
        }
    }
}

// MaterialBase ctor members used by the common report/geometry, plus the
// actual MaterialGasMix fields. Name/Num/group/ROnly/NominalR use the same
// SelectedMaterialData order/map/group/flag/nominal owners as every family.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct SourceWindowGasMaterial {
    pub num_gases: i32,
    pub gas_fractions: [f64; 5],
    pub gases: [SourceGasRecord; 5],
    pub gap_vent_type: WindowGapVentType,
    pub roughness: Option<MaterialSurfaceRoughness>,
    pub thickness: f64,
    pub conductivity: f64,
    pub density: f64,
    pub resistance: f64,
    pub specific_heat: f64,
    pub thermal_absorptance: f64,
    pub solar_absorptance: f64,
    pub visible_absorptance: f64,
}
impl Default for SourceWindowGasMaterial {
    fn default() -> Self {
        Self {
            num_gases: 0,
            gas_fractions: [0.0; 5],
            gases: [SourceGasRecord::default(); 5],
            gap_vent_type: WindowGapVentType::Sealed,
            roughness: None,
            thickness: 0.0,
            conductivity: 0.0,
            density: 0.0,
            resistance: 0.0,
            specific_heat: 0.0,
            thermal_absorptance: 0.0,
            solar_absorptance: 0.0,
            visible_absorptance: 0.0,
        }
    }
}

impl SelectedMaterialData {
    pub(super) fn get_window_gas_mixture_data(
        &mut self,
        raw: &super::SelectedMaterialInput,
        owners: &mut MaterialDataOwners<'_, '_>,
    ) -> Result<(), RuntimeError> {
        owners.shortcut.current_module_object = "WindowMaterial:GasMixture".into();
        self.num_w5_gas_mixtures = super::count(owners.input, "WindowMaterial:GasMixture")?;
        if usize::try_from(self.num_w5_gas_mixtures).ok() != Some(raw.window_gas_mixture.len()) {
            return Err(unavailable("WindowMaterial:GasMixture raw/IP count"));
        }
        for object in &raw.window_gas_mixture {
            raw.get_window_gas_mixture_item(object, owners)?;
            let name = owners.shortcut.alpha_args[0].clone();
            if self.material_map.contains_key(&name) {
                let flags = owners.diagnostic_flags.as_ref().ok_or_else(|| {
                    unavailable("actual GasMixture duplicate diagnostic phase owners")
                })?;
                owners
                    .backend
                    .show_source_severe_error(
                        &format!(
                            "GetMaterialData: WindowMaterial:GasMixture = {name}, duplicate name."
                        ),
                        flags,
                    )
                    .map_err(|error| RuntimeError::HeatBalanceInput(Box::new(error)))?;
                *owners.errors_found = true;
                continue;
            }
            let id = typed_id(owners.model, &name, MaterialKind::WindowGasMixture)?;
            let position = self.insert(&name, id, SourceMaterialGroup::GasMixture)?;
            self.window_gas
                .insert(id, SourceWindowGasMaterial::default());
            let material = self
                .window_gas
                .get_mut(&id)
                .ok_or_else(|| unavailable("allocated GasMixture runtime row"))?;
            for gas in &mut material.gases {
                gas.gas_type = None;
            }
            let count = match owners.shortcut.numeric_args[1] {
                1.0 => 1,
                2.0 => 2,
                3.0 => 3,
                4.0 => 4,
                _ => return Err(unavailable("admitted GasMixture actual integer count")),
            };
            material.num_gases =
                i32::try_from(count).map_err(|_| unavailable("GasMixture count i32"))?;
            for index in 0..count {
                material.gases[index].gas_type =
                    WindowGasType::from_equivalent_layer_energyplus_name(
                        &owners.shortcut.alpha_args[index + 1],
                    )
                    .filter(|value| *value != WindowGasType::Custom);
                if material.gases[index].gas_type.is_none() {
                    let flags = owners.diagnostic_flags.as_ref().ok_or_else(|| {
                        unavailable("actual GasMixture invalid type diagnostic phase owners")
                    })?;
                    owners
                        .backend
                        .show_source_severe_error(
                            &format!(
                                "WindowMaterial:GasMixture=\"{}\", Illegal value.",
                                owners.shortcut.alpha_args[index]
                            ),
                            flags,
                        )
                        .map_err(|error| RuntimeError::HeatBalanceInput(Box::new(error)))?;
                    *owners.errors_found = true;
                }
            }
            material.roughness = Some(MaterialSurfaceRoughness::MediumRough);
            material.thickness = owners.shortcut.numeric_args[0];
            if material.thickness <= 0.0 {
                let flags = owners.diagnostic_flags.as_ref().ok_or_else(|| {
                    unavailable("actual GasMixture Thickness diagnostic phase owners")
                })?;
                owners
                    .backend
                    .show_source_severe_error(
                        &format!("WindowMaterial:GasMixture=\"{name}\", Illegal value."),
                        flags,
                    )
                    .map_err(|error| RuntimeError::HeatBalanceInput(Box::new(error)))?;
                owners.backend.show_continue_error(&format!("{} must be greater than 0.", owners.shortcut.numeric_field_names[0]))
                    .map_err(|error| RuntimeError::HeatBalanceInput(Box::new(crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError::Backend(error))))?;
                // Native deliberately does not set ErrorsFound for this branch.
            }
            self.resistance_only.insert(id, true);
            for index in 0..count {
                // Native's missing-active-type array index is undefined. Keep
                // the explicit safe stop, not a fabricated GasType fallback.
                let gas_type = material.gases[index].gas_type.ok_or_else(|| {
                    unavailable("undefined Native GasMixture Invalid record index")
                })?;
                material.gas_fractions[index] = owners.shortcut.numeric_args[index + 2];
                material.gases[index].properties = gas_type
                    .standard_properties()
                    .ok_or_else(|| unavailable("actual standard GasMixture record"))?;
            }
            // Unlike single Gas, this assignment is unconditional even when an
            // earlier material has set the same genuine shared ErrorsFound.
            self.nominal_r[position] =
                material.thickness / material.gases[0].properties.conductivity.at_300_k();
        }
        Ok(())
    }

    pub(super) fn get_named_window_gas_data(
        &mut self,
        raw: &super::SelectedMaterialInput,
        owners: &mut MaterialDataOwners<'_, '_>,
    ) -> Result<(), RuntimeError> {
        owners.shortcut.current_module_object = "WindowMaterial:Gas".into();
        self.num_w5_gases = super::count(owners.input, "WindowMaterial:Gas")?;
        if usize::try_from(self.num_w5_gases).ok() != Some(raw.window_gas.len()) {
            return Err(unavailable("WindowMaterial:Gas raw/IP count"));
        }
        for object in &raw.window_gas {
            raw.get_window_gas_item(object, owners)?;
            let name = owners.shortcut.alpha_args[0].clone();
            if self.material_map.contains_key(&name) {
                let flags = owners
                    .diagnostic_flags
                    .as_ref()
                    .ok_or_else(|| unavailable("actual Gas duplicate diagnostic phase owners"))?;
                owners
                    .backend
                    .show_source_severe_error(
                        &format!("GetMaterialData: WindowMaterial:Gas = {name}, duplicate name."),
                        flags,
                    )
                    .map_err(|error| RuntimeError::HeatBalanceInput(Box::new(error)))?;
                *owners.errors_found = true;
                continue;
            }
            let id = typed_id(owners.model, &name, MaterialKind::WindowGas)?;
            let material = SourceWindowGasMaterial::default();
            let position = self.insert(&name, id, SourceMaterialGroup::Gas)?;
            self.window_gas.insert(id, material);
            let material = self
                .window_gas
                .get_mut(&id)
                .ok_or_else(|| unavailable("allocated Gas runtime row"))?;
            material.num_gases = 1;
            material.gas_fractions[0] = 1.0;
            let gas_type = WindowGasType::from_equivalent_layer_energyplus_name(
                &owners.shortcut.alpha_args[1],
            )
            .ok_or_else(|| unavailable("admitted actual uppercase Gas Type"))?;
            material.gases[0].gas_type = Some(gas_type);
            material.roughness = Some(MaterialSurfaceRoughness::MediumRough);
            material.thickness = owners.shortcut.numeric_args[0];
            self.resistance_only.insert(id, true);
            if gas_type == WindowGasType::Custom {
                // Native1011-1021 writes every actual Custom field before any
                // diagnostic. Missing optional fields are the IP's real0.
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
                material.gases[0].properties.molecular_weight_g_per_mol =
                    owners.shortcut.numeric_args[10];
                material.gases[0].properties.specific_heat_ratio = owners.shortcut.numeric_args[11];
                if material.gases[0].properties.viscosity.coefficient_a <= 0.0 {
                    *owners.errors_found = true;
                    let flags = owners
                        .diagnostic_flags
                        .as_ref()
                        .ok_or_else(|| unavailable("actual Custom Gas diagnostic phase owners"))?;
                    owners
                        .backend
                        .show_source_severe_error(
                            &format!("GetMaterialData: WindowMaterial:Gas = {name}"),
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
                    let flags = owners
                        .diagnostic_flags
                        .as_ref()
                        .ok_or_else(|| unavailable("actual Custom Gas diagnostic phase owners"))?;
                    owners
                        .backend
                        .show_source_severe_error(
                            &format!("GetMaterialData: WindowMaterial:Gas = {name}"),
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
                    let flags = owners
                        .diagnostic_flags
                        .as_ref()
                        .ok_or_else(|| unavailable("actual Custom Gas diagnostic phase owners"))?;
                    owners
                        .backend
                        .show_source_severe_error(
                            &format!("WindowMaterial:Gas=\"{name}\", Illegal value."),
                            flags,
                        )
                        .map_err(|error| RuntimeError::HeatBalanceInput(Box::new(error)))?;
                    owners.backend.show_continue_error(
                        &format!("{} not > 0.0", owners.shortcut.numeric_field_names[10]),
                    ).map_err(|error| RuntimeError::HeatBalanceInput(Box::new(
                        crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError::Backend(error))))?;
                }
            } else {
                material.gases[0].properties = gas_type.standard_properties().ok_or_else(|| {
                    unavailable("Custom Gas diagnostic owner remains outside named scope")
                })?;
            }
            if !*owners.errors_found {
                let denominator = material.gases[0].properties.conductivity.at_300_k();
                if denominator > 0.0 {
                    // No effective-Thickness positivity or derived-finiteness
                    // predicate exists here in Native. Base Resistance stays0.
                    self.nominal_r[position] = material.thickness / denominator;
                } else {
                    let flags = owners.diagnostic_flags.as_ref().ok_or_else(|| {
                        unavailable("actual Gas conductivity diagnostic phase owners")
                    })?;
                    owners
                        .backend
                        .show_source_severe_error(
                            &format!("WindowMaterial:Gas=\"{name}\", Illegal value."),
                            flags,
                        )
                        .map_err(|error| RuntimeError::HeatBalanceInput(Box::new(error)))?;
                    let formatted = crate::heat_balance::source_round_format::round(denominator, 3)
                        .map_err(|_| unavailable("actual Gas denominator .3R formatter"))?;
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
