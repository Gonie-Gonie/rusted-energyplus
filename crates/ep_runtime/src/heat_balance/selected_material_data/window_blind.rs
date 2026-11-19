//! Material.cc1807-2069 real ordinary Blind rows, raw slat TAR and owned tables.
use super::{MaterialDataOwners, SelectedMaterialData, SourceMaterialGroup, typed_id, unavailable};
use crate::error::RuntimeError;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError;
use ep_model::{MaterialKind, MaterialSurfaceRoughness};

pub(crate) const SOURCE_BLIND_MAX_SLAT_ANGLES: usize = 181;
pub(crate) const SOURCE_BLIND_MAX_PROFILE_ANGLES: usize = 37;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct SourceBlindBmTar {
    pub(crate) bm_tra: f64,
    pub(crate) df_tra: f64,
    pub(crate) bm_ref: f64,
    pub(crate) df_ref: f64,
    pub(crate) abs: f64,
}
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct SourceBlindDfTarGs {
    pub(crate) tra: f64,
    pub(crate) tra_gnd: f64,
    pub(crate) tra_sky: f64,
    pub(crate) refl: f64,
    pub(crate) refl_gnd: f64,
    pub(crate) refl_sky: f64,
    pub(crate) abs: f64,
    pub(crate) abs_gnd: f64,
    pub(crate) abs_sky: f64,
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceBlindBmDf<const P: usize> {
    pub(crate) bm: [SourceBlindBmTar; P],
    pub(crate) df: SourceBlindDfTarGs,
}
impl<const P: usize> Default for SourceBlindBmDf<P> {
    fn default() -> Self {
        Self {
            bm: [SourceBlindBmTar::default(); P],
            df: SourceBlindDfTarGs::default(),
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceBlindFtBk<const P: usize> {
    pub(crate) ft: SourceBlindBmDf<P>,
    pub(crate) bk: SourceBlindBmDf<P>,
}
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct SourceBlindTraEmi {
    pub(crate) tra: f64,
    pub(crate) emi: f64,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceBlindFtBkIr {
    pub(crate) ft: SourceBlindTraEmi,
    pub(crate) bk: SourceBlindTraEmi,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceBlindTraAbsRef<const P: usize> {
    pub(crate) sol: SourceBlindFtBk<P>,
    pub(crate) vis: SourceBlindFtBk<P>,
    pub(crate) ir: SourceBlindFtBkIr,
}

/// Same native MaterialBlind owner; computed tables are never copied by the producer.
#[derive(Debug, PartialEq)]
pub(crate) struct SourceBlindMaterial {
    pub(crate) roughness: Option<MaterialSurfaceRoughness>,
    pub(crate) slat_orientation: i32,
    pub(crate) slat_angle_type: i32,
    pub(crate) thickness: f64,
    pub(crate) conductivity: f64,
    pub(crate) resistance: f64,
    pub(crate) density: f64,
    pub(crate) specific_heat: f64,
    pub(crate) thermal_absorptance: f64,
    pub(crate) thermal_absorptance_input: f64,
    pub(crate) solar_absorptance: f64,
    pub(crate) solar_absorptance_input: f64,
    pub(crate) visible_absorptance: f64,
    pub(crate) visible_absorptance_input: f64,
    pub(crate) transmittance: f64,
    pub(crate) visible_transmittance: f64,
    pub(crate) thermal_transmittance: f64,
    pub(crate) shade_reflectance: f64,
    pub(crate) shade_visible_reflectance: f64,
    pub(crate) to_glass_distance: f64,
    pub(crate) top_opening: f64,
    pub(crate) bottom_opening: f64,
    pub(crate) left_opening: f64,
    pub(crate) right_opening: f64,
    pub(crate) airflow_permeability: f64,
    pub(crate) slat_width: f64,
    pub(crate) slat_separation: f64,
    pub(crate) slat_thickness: f64,
    pub(crate) slat_crown: f64,
    pub(crate) slat_angle: f64,
    pub(crate) min_slat_angle: f64,
    pub(crate) max_slat_angle: f64,
    pub(crate) slat_conductivity: f64,
    pub(crate) slat_tar: SourceBlindTraAbsRef<1>,
    pub(crate) tars: Box<[SourceBlindTraAbsRef<38>]>,
}
impl Default for SourceBlindMaterial {
    fn default() -> Self {
        // Each temporary is one 6400-byte row, never the complete 181-row array.
        // Native recursive TAR member initializers zero every slot at construction.
        let mut tars = Vec::with_capacity(SOURCE_BLIND_MAX_SLAT_ANGLES);
        for _ in 0..SOURCE_BLIND_MAX_SLAT_ANGLES {
            tars.push(SourceBlindTraAbsRef::<38>::default());
        }
        Self {
            roughness: None,
            slat_orientation: -1,
            slat_angle_type: 0,
            thickness: 0.0,
            conductivity: 0.0,
            resistance: 0.0,
            density: 0.0,
            specific_heat: 0.0,
            thermal_absorptance: 0.0,
            thermal_absorptance_input: 0.0,
            solar_absorptance: 0.0,
            solar_absorptance_input: 0.0,
            visible_absorptance: 0.0,
            visible_absorptance_input: 0.0,
            transmittance: 0.0,
            visible_transmittance: 0.0,
            thermal_transmittance: 0.0,
            shade_reflectance: 0.0,
            shade_visible_reflectance: 0.0,
            to_glass_distance: 0.0,
            top_opening: 0.0,
            bottom_opening: 0.0,
            left_opening: 0.0,
            right_opening: 0.0,
            airflow_permeability: 0.0,
            slat_width: 0.0,
            slat_separation: 0.0,
            slat_thickness: 0.0,
            slat_crown: 0.0,
            slat_angle: 0.0,
            min_slat_angle: 0.0,
            max_slat_angle: 0.0,
            slat_conductivity: 0.0,
            slat_tar: SourceBlindTraAbsRef::default(),
            tars: tars.into_boxed_slice(),
        }
    }
}
impl Clone for SourceBlindMaterial {
    fn clone(&self) -> Self {
        // Retain the existing explicit checkpoint Clone contract with heap copies.
        // No giant by-value array, shared alias, recomputation or zero-on-clone.
        let mut tars = Vec::with_capacity(self.tars.len());
        for row in self.tars.iter() {
            tars.push(row.clone());
        }
        Self {
            roughness: self.roughness,
            slat_orientation: self.slat_orientation,
            slat_angle_type: self.slat_angle_type,
            thickness: self.thickness,
            conductivity: self.conductivity,
            resistance: self.resistance,
            density: self.density,
            specific_heat: self.specific_heat,
            thermal_absorptance: self.thermal_absorptance,
            thermal_absorptance_input: self.thermal_absorptance_input,
            solar_absorptance: self.solar_absorptance,
            solar_absorptance_input: self.solar_absorptance_input,
            visible_absorptance: self.visible_absorptance,
            visible_absorptance_input: self.visible_absorptance_input,
            transmittance: self.transmittance,
            visible_transmittance: self.visible_transmittance,
            thermal_transmittance: self.thermal_transmittance,
            shade_reflectance: self.shade_reflectance,
            shade_visible_reflectance: self.shade_visible_reflectance,
            to_glass_distance: self.to_glass_distance,
            top_opening: self.top_opening,
            bottom_opening: self.bottom_opening,
            left_opening: self.left_opening,
            right_opening: self.right_opening,
            airflow_permeability: self.airflow_permeability,
            slat_width: self.slat_width,
            slat_separation: self.slat_separation,
            slat_thickness: self.slat_thickness,
            slat_crown: self.slat_crown,
            slat_angle: self.slat_angle,
            min_slat_angle: self.min_slat_angle,
            max_slat_angle: self.max_slat_angle,
            slat_conductivity: self.slat_conductivity,
            slat_tar: self.slat_tar.clone(),
            tars: tars.into_boxed_slice(),
        }
    }
}
impl SourceBlindMaterial {
    /// Native MaterialBlind::BeamBeamTrans3501-3547, reading this same row.
    pub(crate) fn beam_beam_trans(
        &self,
        prof_ang: f64,
        slat_ang: f64,
    ) -> Result<f64, ep_platform_math::MathProviderError> {
        let cos_prof_ang = ep_platform_math::cos(prof_ang)?;
        let gamma = slat_ang - prof_ang;
        let mut wbar = self.slat_separation;
        if cos_prof_ang != 0.0 {
            wbar = self.slat_width * ep_platform_math::cos(gamma)? / cos_prof_ang;
        }
        let operand = 1.0 - (wbar / self.slat_separation).abs();
        let mut transmission = if 0.0 < operand { operand } else { 0.0 };
        if transmission > 0.0 {
            let mut edge = 0.0;
            let mut edge1 = 0.0;
            if ep_platform_math::sin(gamma)?.abs() > 0.01 {
                let first_lower = slat_ang > 0.0;
                let first_upper = slat_ang <= std::f64::consts::FRAC_PI_2;
                let second_lower = slat_ang > std::f64::consts::FRAC_PI_2;
                let second_upper = slat_ang <= std::f64::consts::PI;
                if (first_lower && first_upper && prof_ang <= slat_ang)
                    || (second_lower
                        && second_upper
                        && prof_ang > -(std::f64::consts::PI - slat_ang))
                {
                    edge1 = self.slat_thickness * ep_platform_math::sin(gamma)?.abs()
                        / ((self.slat_separation
                            + self.slat_thickness / ep_platform_math::sin(slat_ang)?.abs())
                            * cos_prof_ang);
                }
                let absolute = edge1.abs();
                edge = if 1.0 < absolute { 1.0 } else { absolute };
            }
            transmission *= 1.0 - edge;
        }
        Ok(transmission)
    }
}

fn severe(owners: &mut MaterialDataOwners<'_, '_>, message: &str) -> Result<(), RuntimeError> {
    let flags = owners
        .diagnostic_flags
        .as_ref()
        .ok_or_else(|| unavailable("actual Blind diagnostic phase owners"))?;
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
fn warning(owners: &mut MaterialDataOwners<'_, '_>, message: &str) -> Result<(), RuntimeError> {
    let flags = owners
        .diagnostic_flags
        .as_ref()
        .ok_or_else(|| unavailable("actual Blind warning phase owners"))?;
    owners
        .backend
        .show_source_warning_error(message, flags)
        .map_err(|error| RuntimeError::HeatBalanceInput(Box::new(error)))
}
fn rounded(value: f64, precision: usize) -> Result<String, RuntimeError> {
    super::super::source_round_format::round(value, precision).map_err(|error| {
        RuntimeError::HeatBalanceInput(Box::new(HeatBalanceInputError::Format(format!(
            "Blind Native R formatting: {error:?}"
        ))))
    })
}

impl SelectedMaterialData {
    pub(super) fn get_window_blind_data(
        &mut self,
        raw: &super::SelectedMaterialInput,
        owners: &mut MaterialDataOwners<'_, '_>,
    ) -> Result<(), RuntimeError> {
        owners.shortcut.current_module_object = "WindowMaterial:Blind".into();
        self.num_blinds = i32::try_from(owners.input.get_num_objects_found("WindowMaterial:Blind"))
            .map_err(|_| unavailable("Blind count exceeds Native integer"))?;
        if usize::try_from(self.num_blinds).ok() != Some(raw.window_blind.len()) {
            return Err(unavailable("Blind raw/IP count"));
        }
        for object in &raw.window_blind {
            let (alphas, numbers) = raw.get_window_blind_item(object, owners)?;
            let invalid_counts = match raw.source_original_idf_fixed_counts(
                "WindowMaterial:Blind",
                object,
                owners,
            )? {
                Some(expected) => (alphas, numbers) != expected,
                None => alphas != 2 || numbers != 27,
            };
            if invalid_counts {
                return Err(unavailable("Blind2A27N/min29 primitive"));
            }
            let name = owners.shortcut.alpha_args[0].clone();
            if self.material_map.contains_key(&name) {
                severe(
                    owners,
                    &format!("GetMaterialData: WindowMaterial:Blind = {name}, duplicate name."),
                )?;
                *owners.errors_found = true;
                continue;
            }
            let id = typed_id(owners.model, &name, MaterialKind::WindowBlind)?;
            self.insert(&name, id, SourceMaterialGroup::Blind)?;
            let material = self.window_blind.entry(id).or_default();
            material.roughness = Some(MaterialSurfaceRoughness::Rough);
            self.resistance_only.insert(id, true);
            material.slat_orientation = match owners.shortcut.alpha_args[1].as_str() {
                "HORIZONTAL" => 0,
                "VERTICAL" => 1,
                _ => -1,
            };
            material.slat_width = owners.shortcut.numeric_args[0];
            material.slat_separation = owners.shortcut.numeric_args[1];
            material.slat_thickness = owners.shortcut.numeric_args[2];
            material.slat_angle = owners.shortcut.numeric_args[3];
            material.slat_conductivity = owners.shortcut.numeric_args[4];
            material.slat_tar.sol.ft.bm[0].df_tra = owners.shortcut.numeric_args[5];
            material.slat_tar.sol.ft.bm[0].df_ref = owners.shortcut.numeric_args[6];
            material.slat_tar.sol.bk.bm[0].df_ref = owners.shortcut.numeric_args[7];
            material.slat_tar.sol.ft.df.tra = owners.shortcut.numeric_args[8];
            material.slat_tar.sol.ft.df.refl = owners.shortcut.numeric_args[9];
            material.slat_tar.sol.bk.df.refl = owners.shortcut.numeric_args[10];
            material.slat_tar.vis.ft.bm[0].df_tra = owners.shortcut.numeric_args[11];
            material.slat_tar.vis.ft.bm[0].df_ref = owners.shortcut.numeric_args[12];
            material.slat_tar.vis.bk.bm[0].df_ref = owners.shortcut.numeric_args[13];
            material.slat_tar.vis.ft.df.tra = owners.shortcut.numeric_args[14];
            material.slat_tar.vis.ft.df.refl = owners.shortcut.numeric_args[15];
            material.slat_tar.vis.bk.df.refl = owners.shortcut.numeric_args[16];
            material.slat_tar.ir.ft.tra = owners.shortcut.numeric_args[17];
            material.slat_tar.ir.bk.tra = owners.shortcut.numeric_args[17];
            material.slat_tar.ir.ft.emi = owners.shortcut.numeric_args[18];
            material.slat_tar.ir.bk.emi = owners.shortcut.numeric_args[19];
            material.to_glass_distance = owners.shortcut.numeric_args[20];
            material.top_opening = owners.shortcut.numeric_args[21];
            material.bottom_opening = owners.shortcut.numeric_args[22];
            material.left_opening = owners.shortcut.numeric_args[23];
            material.right_opening = owners.shortcut.numeric_args[24];
            material.min_slat_angle = owners.shortcut.numeric_args[25];
            material.max_slat_angle = owners.shortcut.numeric_args[26];
            material.slat_angle_type = 0;
            if material.slat_width < material.slat_separation {
                warning(
                    owners,
                    &format!("WindowMaterial:Blind=\"{name}\", Slat Angles/Widths"),
                )?;
                continuation(
                    owners,
                    &format!(
                        "{} [{}] is less than {} [{}].",
                        owners.shortcut.numeric_field_names[0],
                        rounded(material.slat_width, 2)?,
                        owners.shortcut.numeric_field_names[1],
                        rounded(material.slat_separation, 2)?
                    ),
                )?;
                continuation(
                    owners,
                    "This will allow direct beam to be transmitted when Slat angle = 0.",
                )?;
            }
            if owners.shortcut.numeric_args[5] + owners.shortcut.numeric_args[6] >= 1.0 {
                *owners.errors_found = true;
                severe(
                    owners,
                    &format!("WindowMaterial:Blind=\"{name}\", Illegal value combination."),
                )?;
                continuation(
                    owners,
                    &format!(
                        "{} + {} not < 1.0",
                        owners.shortcut.numeric_field_names[5],
                        owners.shortcut.numeric_field_names[6]
                    ),
                )?;
            }
            if owners.shortcut.numeric_args[5] + owners.shortcut.numeric_args[7] >= 1.0 {
                *owners.errors_found = true;
                severe(
                    owners,
                    &format!("WindowMaterial:Blind=\"{name}\", Illegal value combination."),
                )?;
                continuation(
                    owners,
                    &format!(
                        "{} + {} not < 1.0",
                        owners.shortcut.numeric_field_names[5],
                        owners.shortcut.numeric_field_names[7]
                    ),
                )?;
            }
            if owners.shortcut.numeric_args[8] + owners.shortcut.numeric_args[9] >= 1.0 {
                *owners.errors_found = true;
                severe(
                    owners,
                    &format!("WindowMaterial:Blind=\"{name}\", Illegal value combination."),
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
            if owners.shortcut.numeric_args[8] + owners.shortcut.numeric_args[10] >= 1.0 {
                *owners.errors_found = true;
                severe(
                    owners,
                    &format!("WindowMaterial:Blind=\"{name}\", Illegal value combination."),
                )?;
                continuation(
                    owners,
                    &format!(
                        "{} + {} not < 1.0",
                        owners.shortcut.numeric_field_names[8],
                        owners.shortcut.numeric_field_names[10]
                    ),
                )?;
            }
            if owners.shortcut.numeric_args[11] + owners.shortcut.numeric_args[12] >= 1.0
                || owners.shortcut.numeric_args[11] + owners.shortcut.numeric_args[13] >= 1.0
            {
                *owners.errors_found = true;
                severe(
                    owners,
                    &format!("WindowMaterial:Blind=\"{name}\", Illegal value combination."),
                )?;
                continuation(
                    owners,
                    &format!(
                        "{} + {} not < 1.0 OR",
                        owners.shortcut.numeric_field_names[11],
                        owners.shortcut.numeric_field_names[12]
                    ),
                )?;
                continuation(
                    owners,
                    &format!(
                        "{} + {} not < 1.0",
                        owners.shortcut.numeric_field_names[11],
                        owners.shortcut.numeric_field_names[13]
                    ),
                )?;
            }
            if owners.shortcut.numeric_args[11] + owners.shortcut.numeric_args[12] >= 1.0 {
                *owners.errors_found = true;
                severe(
                    owners,
                    &format!("WindowMaterial:Blind=\"{name}\", Illegal value combination."),
                )?;
                continuation(
                    owners,
                    &format!(
                        "{} + {} not < 1.0",
                        owners.shortcut.numeric_field_names[11],
                        owners.shortcut.numeric_field_names[12]
                    ),
                )?;
            }
            if owners.shortcut.numeric_args[11] + owners.shortcut.numeric_args[13] >= 1.0 {
                *owners.errors_found = true;
                severe(
                    owners,
                    &format!("WindowMaterial:Blind=\"{name}\", Illegal value combination."),
                )?;
                continuation(
                    owners,
                    &format!(
                        "{} + {} not < 1.0",
                        owners.shortcut.numeric_field_names[11],
                        owners.shortcut.numeric_field_names[13]
                    ),
                )?;
            }
            if owners.shortcut.numeric_args[14] + owners.shortcut.numeric_args[15] >= 1.0 {
                *owners.errors_found = true;
                severe(
                    owners,
                    &format!("WindowMaterial:Blind=\"{name}\", Illegal value combination."),
                )?;
                continuation(
                    owners,
                    &format!(
                        "{} + {} not < 1.0",
                        owners.shortcut.numeric_field_names[14],
                        owners.shortcut.numeric_field_names[15]
                    ),
                )?;
            }
            if owners.shortcut.numeric_args[14] + owners.shortcut.numeric_args[16] >= 1.0 {
                *owners.errors_found = true;
                severe(
                    owners,
                    &format!("WindowMaterial:Blind=\"{name}\", Illegal value combination."),
                )?;
                continuation(
                    owners,
                    &format!(
                        "{} + {} not < 1.0",
                        owners.shortcut.numeric_field_names[14],
                        owners.shortcut.numeric_field_names[16]
                    ),
                )?;
            }
            if (owners.shortcut.numeric_args[8] - owners.shortcut.numeric_args[5]).abs() > 1.0e-5 {
                *owners.errors_found = true;
                severe(
                    owners,
                    &format!("WindowMaterial:Blind=\"{name}\", Illegal value combination."),
                )?;
                continuation(
                    owners,
                    &format!(
                        "{} must equal {}",
                        owners.shortcut.numeric_field_names[5],
                        owners.shortcut.numeric_field_names[8]
                    ),
                )?;
            }
            if (owners.shortcut.numeric_args[9] - owners.shortcut.numeric_args[6]).abs() > 1.0e-5 {
                *owners.errors_found = true;
                severe(
                    owners,
                    &format!("WindowMaterial:Blind=\"{name}\", Illegal value combination."),
                )?;
                continuation(
                    owners,
                    &format!(
                        "{} must equal {}",
                        owners.shortcut.numeric_field_names[6],
                        owners.shortcut.numeric_field_names[9]
                    ),
                )?;
            }
            if (owners.shortcut.numeric_args[10] - owners.shortcut.numeric_args[7]).abs() > 1.0e-5 {
                *owners.errors_found = true;
                severe(
                    owners,
                    &format!("WindowMaterial:Blind=\"{name}\", Illegal value combination."),
                )?;
                continuation(
                    owners,
                    &format!(
                        "{} must equal {}",
                        owners.shortcut.numeric_field_names[7],
                        owners.shortcut.numeric_field_names[10]
                    ),
                )?;
            }
            if (owners.shortcut.numeric_args[14] - owners.shortcut.numeric_args[11]).abs() > 1.0e-5
            {
                *owners.errors_found = true;
                severe(
                    owners,
                    &format!("WindowMaterial:Blind=\"{name}\", Illegal value combination."),
                )?;
                continuation(
                    owners,
                    &format!(
                        "{} must equal {}",
                        owners.shortcut.numeric_field_names[11],
                        owners.shortcut.numeric_field_names[14]
                    ),
                )?;
            }
            if (owners.shortcut.numeric_args[15] - owners.shortcut.numeric_args[12]).abs() > 1.0e-5
            {
                *owners.errors_found = true;
                severe(
                    owners,
                    &format!("WindowMaterial:Blind=\"{name}\", Illegal value combination."),
                )?;
                continuation(
                    owners,
                    &format!(
                        "{} must equal {}",
                        owners.shortcut.numeric_field_names[12],
                        owners.shortcut.numeric_field_names[15]
                    ),
                )?;
            }
            if (owners.shortcut.numeric_args[16] - owners.shortcut.numeric_args[13]).abs() > 1.0e-5
            {
                *owners.errors_found = true;
                severe(
                    owners,
                    &format!("WindowMaterial:Blind=\"{name}\", Illegal value combination."),
                )?;
                continuation(
                    owners,
                    &format!(
                        "{} must equal {}",
                        owners.shortcut.numeric_field_names[13],
                        owners.shortcut.numeric_field_names[16]
                    ),
                )?;
            }
            if owners.shortcut.numeric_args[17] + owners.shortcut.numeric_args[18] >= 1.0 {
                *owners.errors_found = true;
                severe(
                    owners,
                    &format!("WindowMaterial:Blind=\"{name}\", Illegal value combination."),
                )?;
                continuation(
                    owners,
                    &format!(
                        "{} + {} not < 1.0",
                        owners.shortcut.numeric_field_names[17],
                        owners.shortcut.numeric_field_names[18]
                    ),
                )?;
            }
            if owners.shortcut.numeric_args[17] + owners.shortcut.numeric_args[19] >= 1.0 {
                *owners.errors_found = true;
                severe(
                    owners,
                    &format!("WindowMaterial:Blind=\"{name}\", Illegal value combination."),
                )?;
                continuation(
                    owners,
                    &format!(
                        "{} + {} not < 1.0",
                        owners.shortcut.numeric_field_names[17],
                        owners.shortcut.numeric_field_names[19]
                    ),
                )?;
            }
            if material.to_glass_distance < 0.5 * material.slat_width {
                *owners.errors_found = true;
                severe(
                    owners,
                    &format!("WindowMaterial:Blind=\"{name}\", Illegal value combination."),
                )?;
                continuation(
                    owners,
                    &format!(
                        "{} is less than half of the {}",
                        owners.shortcut.numeric_field_names[20],
                        owners.shortcut.numeric_field_names[0]
                    ),
                )?;
            }
            let minimum_geometry_angle = if material.slat_width > material.slat_separation {
                ep_platform_math::asin(
                    material.slat_thickness / (material.slat_thickness + material.slat_separation),
                )
                .map_err(|error| unavailable(&format!("required Blind asin provider: {error}")))?
                    / (std::f64::consts::PI / 180.0)
            } else {
                0.0
            };
            let maximum_geometry_angle = 180.0 - minimum_geometry_angle;
            if material.slat_separation + material.slat_thickness < material.slat_width {
                if material.slat_angle < minimum_geometry_angle {
                    *owners.errors_found = true;
                    severe(
                        owners,
                        &format!("WindowMaterial:Blind=\"{name}\", Illegal value combination."),
                    )?;
                    continuation(
                        owners,
                        &format!(
                            "{}=[{}], is less than smallest allowed by slat dimensions and spacing, [{}] deg.",
                            owners.shortcut.numeric_field_names[3],
                            rounded(material.slat_angle, 1)?,
                            rounded(minimum_geometry_angle, 1)?
                        ),
                    )?;
                } else if material.slat_angle > maximum_geometry_angle {
                    *owners.errors_found = true;
                    severe(
                        owners,
                        &format!("WindowMaterial:Blind=\"{name}\", Illegal value combination."),
                    )?;
                    // Native upper branch deliberately prints MinSlatAngGeom.
                    continuation(
                        owners,
                        &format!(
                            "{}=[{}], is greater than largest allowed by slat dimensions and spacing, [{}] deg.",
                            owners.shortcut.numeric_field_names[3],
                            rounded(material.slat_angle, 1)?,
                            rounded(minimum_geometry_angle, 1)?
                        ),
                    )?;
                }
            }
            // Native's variable-angle N26/N27 checks are commented out for Fixed.
        }
        Ok(())
    }
}
