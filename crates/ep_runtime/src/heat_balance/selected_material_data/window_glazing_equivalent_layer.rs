//! Material.cc857-948/Material.hh877-891: actual MaterialGlassEQL:MaterialFen and TAR1.
use super::window_blind::SourceBlindTraAbsRef;
use super::{MaterialDataOwners, SelectedMaterialData, SourceMaterialGroup, typed_id, unavailable};
use crate::error::RuntimeError;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError;
use ep_model::{MaterialKind, MaterialSurfaceRoughness};

/// Direct private Fen-compatible row; no Shade opening fields or ordinaryGlass Setup/spectral cache.
#[derive(Clone, Debug, Default, PartialEq)]
pub(super) struct SourceGlazingEquivalentLayerMaterial {
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
    pub thermal_absorptance_front: f64,
    pub thermal_absorptance_back: f64,
    pub transmittance: f64,
    pub visible_transmittance: f64,
    pub thermal_transmittance: f64,
    pub reflect_solar_beam_back: f64,
    pub reflect_solar_beam_front: f64,
    pub tar: SourceBlindTraAbsRef<1>,
    pub optical_model: i32,
}
fn severe(owners: &mut MaterialDataOwners<'_, '_>, message: &str) -> Result<(), RuntimeError> {
    let flags = owners
        .diagnostic_flags
        .as_ref()
        .ok_or_else(|| unavailable("actual GlazingEQL diagnostic phase owners"))?;
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
    pub(super) fn get_window_glazing_equivalent_layer_data(
        &mut self,
        raw: &super::SelectedMaterialInput,
        owners: &mut MaterialDataOwners<'_, '_>,
    ) -> Result<(), RuntimeError> {
        owners.shortcut.current_module_object = "WindowMaterial:Glazing:EquivalentLayer".into();
        self.num_eql_glazings = i32::try_from(
            owners
                .input
                .get_num_objects_found("WindowMaterial:Glazing:EquivalentLayer"),
        )
        .map_err(|_| unavailable("GlazingEQL count exceeds Native integer"))?;
        if usize::try_from(self.num_eql_glazings).ok()
            != Some(raw.window_glazing_equivalent_layer.len())
        {
            return Err(unavailable("GlazingEQL raw/IP count"));
        }
        for object in &raw.window_glazing_equivalent_layer {
            let (alphas, numbers) = raw.get_window_glazing_equivalent_layer_item(object, owners)?;
            let invalid_counts = match raw.source_original_idf_fixed_counts(
                "WindowMaterial:Glazing:EquivalentLayer",
                object,
                owners,
            )? {
                Some(expected) => (alphas, numbers) != expected,
                None => alphas != 3 || !(8..=26).contains(&numbers),
            };
            if invalid_counts {
                return Err(unavailable("GlazingEQL3A26N/min11 primitive"));
            }
            let name = owners.shortcut.alpha_args[0].clone();
            if self.material_map.contains_key(&name) {
                severe(
                    owners,
                    &format!(
                        "GetMaterialData: WindowMaterial:Glazing:EquivalentLayer = {name}, duplicate name."
                    ),
                )?;
                *owners.errors_found = true;
                continue;
            }
            let id = typed_id(
                owners.model,
                &name,
                MaterialKind::WindowGlazingEquivalentLayer,
            )?;
            self.insert(&name, id, SourceMaterialGroup::GlassEql)?;
            let row = self.window_glazing_equivalent_layer.entry(id).or_default();
            row.roughness = Some(MaterialSurfaceRoughness::VerySmooth);
            self.resistance_only.insert(id, true);
            row.tar.sol.ft.bm[0].bm_tra = owners.shortcut.numeric_args[0];
            row.tar.sol.bk.bm[0].bm_tra = owners.shortcut.numeric_args[1];
            row.tar.sol.ft.bm[0].bm_ref = owners.shortcut.numeric_args[2];
            row.tar.sol.bk.bm[0].bm_ref = owners.shortcut.numeric_args[3];
            row.tar.vis.ft.bm[0].bm_tra = owners.shortcut.numeric_args[4];
            row.tar.vis.bk.bm[0].bm_tra = owners.shortcut.numeric_args[5];
            row.tar.vis.ft.bm[0].bm_ref = owners.shortcut.numeric_args[6];
            row.tar.vis.bk.bm[0].bm_ref = owners.shortcut.numeric_args[7];
            row.tar.sol.ft.bm[0].df_tra = owners.shortcut.numeric_args[8];
            row.tar.sol.bk.bm[0].df_tra = owners.shortcut.numeric_args[9];
            row.tar.sol.ft.bm[0].df_ref = owners.shortcut.numeric_args[10];
            row.tar.sol.bk.bm[0].df_ref = owners.shortcut.numeric_args[11];
            row.tar.vis.ft.bm[0].df_tra = owners.shortcut.numeric_args[12];
            row.tar.vis.bk.bm[0].df_tra = owners.shortcut.numeric_args[13];
            row.tar.vis.ft.bm[0].df_ref = owners.shortcut.numeric_args[14];
            row.tar.vis.bk.bm[0].df_ref = owners.shortcut.numeric_args[15];
            row.tar.sol.ft.df.tra = owners.shortcut.numeric_args[16];
            row.tar.sol.bk.df.tra = owners.shortcut.numeric_args[16];
            row.tar.sol.ft.df.refl = owners.shortcut.numeric_args[17];
            row.tar.sol.bk.df.refl = owners.shortcut.numeric_args[18];
            row.tar.vis.ft.df.tra = owners.shortcut.numeric_args[19];
            row.tar.vis.bk.df.tra = owners.shortcut.numeric_args[19];
            row.tar.vis.ft.df.refl = owners.shortcut.numeric_args[20];
            row.tar.vis.bk.df.refl = owners.shortcut.numeric_args[21];
            row.tar.ir.ft.tra = owners.shortcut.numeric_args[22];
            row.tar.ir.bk.tra = owners.shortcut.numeric_args[22];
            row.tar.ir.ft.emi = owners.shortcut.numeric_args[23];
            row.tar.ir.bk.emi = owners.shortcut.numeric_args[24];
            row.resistance = owners.shortcut.numeric_args[25];
            if row.resistance <= 0.0 {
                row.resistance = 0.158;
            }
            // Native copies N23 IR transmittance here, never emissivity N24/N25; NominalR remains0.
            row.thermal_absorptance_front = row.tar.ir.ft.tra;
            row.thermal_absorptance_back = row.tar.ir.bk.tra;
            row.thermal_transmittance = row.tar.ir.ft.tra;
            row.optical_model = match owners.shortcut.alpha_args[1].as_str() {
                "SPECTRALAVERAGE" => 0,
                "SPECTRAL" => 1,
                "BSDF" => 2,
                "SPECTRALANDANGLE" => 3,
                _ => -1,
            };
            // The Native spectral-set lookup is commented out; no A3 dependency or Auto solver is introduced.
            if row.optical_model != 0 {
                *owners.errors_found = true;
                severe(
                    owners,
                    &format!("GetMaterialData: WindowMaterial:Glazing:EquivalentLayer = {name}"),
                )?;
                continuation(
                    owners,
                    &format!(
                        "{} = {}, invalid key.",
                        owners.shortcut.alpha_field_names[1], owners.shortcut.alpha_args[1]
                    ),
                )?;
                continuation(owners, "Must be \"SpectralAverage\".")?;
            }
        }
        Ok(())
    }
}
