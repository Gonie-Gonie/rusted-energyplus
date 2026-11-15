//! Material.cc950-1052: actual named MaterialGasMix/base rows, not typed nominal data.
use super::{MaterialDataOwners, SelectedMaterialData, SourceMaterialGroup, typed_id, unavailable};
use crate::error::RuntimeError;
use ep_model::{
    MaterialKind, MaterialSurfaceRoughness, WindowGapVentType, WindowGasPolynomialCoefficients,
    WindowGasProperties, WindowGasType,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct SourceGasRecord {
    pub gas_type: WindowGasType,
    pub properties: WindowGasProperties,
}
impl Default for SourceGasRecord {
    fn default() -> Self {
        Self {
            gas_type: WindowGasType::Custom,
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
            material.gases[0].gas_type = gas_type;
            material.roughness = Some(MaterialSurfaceRoughness::MediumRough);
            material.thickness = owners.shortcut.numeric_args[0];
            self.resistance_only.insert(id, true);
            material.gases[0].properties = gas_type.standard_properties().ok_or_else(|| {
                unavailable("Custom Gas diagnostic owner remains outside named scope")
            })?;
            if !*owners.errors_found {
                let denominator = material.gases[0].properties.conductivity.at_300_k();
                if denominator > 0.0 {
                    // No effective-Thickness positivity or derived-finiteness
                    // predicate exists here in Native. Base Resistance stays0.
                    self.nominal_r[position] = material.thickness / denominator;
                } else {
                    // All four admitted fixed records have a positive k300.
                    // The reachable Custom denominator diagnostic is held.
                    return Err(unavailable(
                        "nonpositive named Gas conductivity outside fixed records",
                    ));
                }
            }
        }
        Ok(())
    }
}
