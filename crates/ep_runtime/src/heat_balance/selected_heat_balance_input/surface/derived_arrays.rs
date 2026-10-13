//! Literal SurfaceWindowCalc and SurfaceShade construction at1219–1223.
//! Dynamic arrays stay unallocated; no positive window/blind/BSDF solver exists.
const PROFILE_ANGLES: usize = 37 + 1; // Material::MaxProfAngs +1, actual504.

#[allow(dead_code)] // Actual constructor-only arrays precede later consumers.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ComplexWindowDescription {
    pub(crate) num_states: i32,
    pub(crate) current_state: i32,
    pub(crate) result_all_states: Option<Vec<f64>>,
    pub(crate) states: Option<Vec<std::convert::Infallible>>,
}
impl Default for ComplexWindowDescription {
    fn default() -> Self {
        Self {
            num_states: 0,
            current_state: 1,
            result_all_states: None,
            states: None,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SurfaceWindowCalculation {
    /// No SurfaceWindowRefPt element is constructed in the unallocated array.
    pub(crate) reference_points: Option<Vec<std::convert::Infallible>>,
    pub(crate) window_center: [f64; 3],
    pub(crate) theta: f64,
    pub(crate) phi: f64,
    pub(crate) rho_ceiling_wall: f64,
    pub(crate) rho_floor_wall: f64,
    pub(crate) fraction_upgoing: f64,
    pub(crate) glazed_fraction: f64,
    pub(crate) center_glass_area: f64,
    pub(crate) edge_glass_correction: f64,
    pub(crate) screen_num: i32,
    pub(crate) light_well_efficiency: f64,
    pub(crate) theta_face: [f64; 11],
    pub(crate) outside_projection_sunlit_multiplier: [f64; 25],
    pub(crate) inside_outside_projection_sunlit_multiplier: [f64; 25],
    pub(crate) enclosure_area_minus_surface: [f64; 3],
    pub(crate) enclosure_area_reflectance_minus_surface: [f64; 3],
    pub(crate) complex_fenestration: ComplexWindowDescription,
    pub(crate) has_shade: bool,
    pub(crate) has_blind: bool,
    pub(crate) has_screen: bool,
}
impl Default for SurfaceWindowCalculation {
    fn default() -> Self {
        // C++ std::array={x} initializes element0=x, all later elements+0.
        let mut theta_face = [0.0; 11];
        theta_face[0] = 296.15;
        let mut outside_projection_sunlit_multiplier = [0.0; 25];
        outside_projection_sunlit_multiplier[0] = 1.0;
        let mut inside_outside_projection_sunlit_multiplier = [0.0; 25];
        inside_outside_projection_sunlit_multiplier[0] = 1.0;
        Self {
            reference_points: None,
            window_center: [0.0; 3],
            theta: 0.0,
            phi: 0.0,
            rho_ceiling_wall: 0.0,
            rho_floor_wall: 0.0,
            fraction_upgoing: 0.0,
            glazed_fraction: 1.0,
            center_glass_area: 0.0,
            edge_glass_correction: 1.0,
            screen_num: 0,
            light_well_efficiency: 1.0,
            theta_face,
            outside_projection_sunlit_multiplier,
            inside_outside_projection_sunlit_multiplier,
            enclosure_area_minus_surface: [0.0; 3],
            enclosure_area_reflectance_minus_surface: [0.0; 3],
            complex_fenestration: ComplexWindowDescription::default(),
            has_shade: false,
            has_blind: false,
            has_screen: false,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct BlindBeam {
    pub(crate) beam_transmittance: f64,
    pub(crate) diffuse_transmittance: f64,
    pub(crate) beam_reflectance: f64,
    pub(crate) diffuse_reflectance: f64,
    pub(crate) absorptance: f64,
}
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct BlindDiffuseGroundSky {
    pub(crate) transmittance: f64,
    pub(crate) ground_transmittance: f64,
    pub(crate) sky_transmittance: f64,
    pub(crate) reflectance: f64,
    pub(crate) ground_reflectance: f64,
    pub(crate) sky_reflectance: f64,
    pub(crate) absorptance: f64,
    pub(crate) ground_absorptance: f64,
    pub(crate) sky_absorptance: f64,
}
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct BlindBeamDiffuse {
    pub(crate) beam: [BlindBeam; PROFILE_ANGLES],
    pub(crate) diffuse: BlindDiffuseGroundSky,
}
impl Default for BlindBeamDiffuse {
    fn default() -> Self {
        Self {
            beam: [BlindBeam::default(); PROFILE_ANGLES],
            diffuse: BlindDiffuseGroundSky::default(),
        }
    }
}
#[allow(dead_code)]
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct BlindFrontBack {
    pub(crate) front: BlindBeamDiffuse,
    pub(crate) back: BlindBeamDiffuse,
}
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct BlindInfraredTransmission {
    pub(crate) transmittance: f64,
    pub(crate) emissivity: f64,
}
#[allow(dead_code)]
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct BlindInfraredFrontBack {
    pub(crate) front: BlindInfraredTransmission,
    pub(crate) back: BlindInfraredTransmission,
}
#[allow(dead_code)]
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct BlindTransmissionAbsorptanceReflectance {
    pub(crate) solar: BlindFrontBack,
    pub(crate) visible: BlindFrontBack,
    pub(crate) infrared: BlindInfraredFrontBack,
}
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SurfaceBlind {
    pub(crate) material_num: i32,
    pub(crate) movable_slats: bool,
    pub(crate) slat_angle: f64,
    pub(crate) slat_angle_degrees: f64,
    pub(crate) slat_angle_degrees_ems_on: bool,
    pub(crate) slat_angle_degrees_ems_value: f64,
    pub(crate) slat_block_beam: bool,
    pub(crate) slat_angle_low_index: i32,
    pub(crate) slat_angle_high_index: i32,
    pub(crate) slat_angle_interpolation: f64,
    pub(crate) profile_angle: f64,
    pub(crate) profile_angle_low_index: i32,
    pub(crate) profile_angle_high_index: i32,
    pub(crate) profile_angle_interpolation: f64,
    pub(crate) beam_beam_transmittance: f64,
    pub(crate) airflow_permeability: f64,
    pub(crate) tar: BlindTransmissionAbsorptanceReflectance,
}
impl Default for SurfaceBlind {
    fn default() -> Self {
        Self {
            material_num: 0,
            movable_slats: false,
            slat_angle: 0.0,
            slat_angle_degrees: 0.0,
            slat_angle_degrees_ems_on: false,
            slat_angle_degrees_ems_value: 0.0,
            slat_block_beam: false,
            slat_angle_low_index: -1,
            slat_angle_high_index: -1,
            slat_angle_interpolation: 0.0,
            profile_angle: 0.0,
            profile_angle_low_index: 0,
            profile_angle_high_index: 0,
            profile_angle_interpolation: 0.0,
            beam_beam_transmittance: 0.0,
            airflow_permeability: 0.0,
            tar: BlindTransmissionAbsorptanceReflectance::default(),
        }
    }
}
#[allow(dead_code)]
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SurfaceShadeGlass {
    pub(crate) infrared_emissivity: f64,
    pub(crate) infrared_reflectance: f64,
}
#[allow(dead_code)]
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SurfaceShade {
    pub(crate) blind: SurfaceBlind,
    pub(crate) glass: SurfaceShadeGlass,
    pub(crate) effective_shade_emissivity: f64,
    pub(crate) effective_glass_emissivity: f64,
}
