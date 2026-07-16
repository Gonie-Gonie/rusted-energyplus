use ep_model::{WindowScreenBeamReflectanceModel, WindowScreenMaterial};

const SMALL: f64 = 1.0e-9;
const AZIMUTH_STEPS: usize = 18;
const ALTITUDE_STEPS: usize = 18;
const DEG_TO_RAD: f64 = std::f64::consts::PI / 180.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct ScreenSourceOptics {
    pub(super) normal_solar_transmittance: f64,
    pub(super) normal_solar_reflectance: f64,
    pub(super) normal_visible_reflectance: f64,
    pub(super) diffuse_solar_reflectance: f64,
    pub(super) diffuse_visible_reflectance: f64,
    pub(super) diameter_to_spacing_ratio: f64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct ScreenBeamOptics {
    beam_solar_transmittance: f64,
    front_solar_reflectance: f64,
    front_visible_reflectance: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct ScreenStaticInputs {
    pub(super) diameter_to_spacing_ratio: f64,
    pub(super) cylinder_solar_reflectance: f64,
    pub(super) cylinder_visible_reflectance: f64,
}

pub(super) fn calculate_screen_source_optics(
    fields: WindowScreenMaterial,
) -> Result<ScreenSourceOptics, String> {
    let inputs = screen_static_inputs(fields)?;
    let normal = calculate_screen_beam_optics(fields.beam_reflectance_model, inputs, 0.0, 0.0)?;

    let mut sin_sun_azimuth = [0.0; AZIMUTH_STEPS];
    let mut cos_sun_azimuth = [0.0; AZIMUTH_STEPS];
    for j in 0..AZIMUTH_STEPS {
        let angle = (90.0 / AZIMUTH_STEPS as f64) * j as f64 * DEG_TO_RAD;
        sin_sun_azimuth[j] = angle.sin();
        cos_sun_azimuth[j] = angle.cos();
    }

    let mut sun_altitude = [0.0; ALTITUDE_STEPS];
    let mut sin_sun_altitude = [0.0; ALTITUDE_STEPS];
    let mut sky_area = [0.0; ALTITUDE_STEPS];
    for i in 0..ALTITUDE_STEPS {
        let angle = (90.0 / ALTITUDE_STEPS as f64) * i as f64 * DEG_TO_RAD;
        sun_altitude[i] = angle;
        sin_sun_altitude[i] = angle.sin();
        sky_area[i] = sin_sun_altitude[i] * angle.cos();
    }

    let mut relative_azimuth = [[0.0; AZIMUTH_STEPS]; ALTITUDE_STEPS];
    let mut relative_altitude = [[0.0; AZIMUTH_STEPS]; ALTITUDE_STEPS];
    for j in 0..AZIMUTH_STEPS {
        for i in 0..ALTITUDE_STEPS {
            relative_azimuth[i][j] = (sin_sun_altitude[i] * cos_sun_azimuth[j]).asin();
            relative_altitude[i][j] = (sun_altitude[i].tan() * sin_sun_azimuth[j]).atan();
        }
    }

    let mut sum_solar_reflectance = 0.0;
    let mut sum_visible_reflectance = 0.0;
    let mut sum_area = 0.0;
    // EnergyPlus deliberately traverses both axes in reverse so the final
    // beam calculation is normal incidence. Preserve that summation order.
    for j in (0..AZIMUTH_STEPS).rev() {
        for i in (0..ALTITUDE_STEPS).rev() {
            let optics = calculate_screen_beam_optics(
                fields.beam_reflectance_model,
                inputs,
                relative_altitude[i][j],
                relative_azimuth[i][j],
            )?;
            sum_solar_reflectance += optics.front_solar_reflectance * sky_area[i];
            sum_visible_reflectance += optics.front_visible_reflectance * sky_area[i];
            sum_area += sky_area[i];
        }
    }
    if sum_area == 0.0 {
        return Err(
            "WindowMaterial:Screen quarter-hemisphere integration area is zero".to_string(),
        );
    }

    Ok(ScreenSourceOptics {
        normal_solar_transmittance: normal.beam_solar_transmittance,
        normal_solar_reflectance: normal.front_solar_reflectance,
        normal_visible_reflectance: normal.front_visible_reflectance,
        diffuse_solar_reflectance: sum_solar_reflectance / sum_area,
        diffuse_visible_reflectance: sum_visible_reflectance / sum_area,
        diameter_to_spacing_ratio: inputs.diameter_to_spacing_ratio,
    })
}

pub(super) fn screen_static_inputs(
    fields: WindowScreenMaterial,
) -> Result<ScreenStaticInputs, String> {
    // CalcWindowScreenProperties reconstructs gamma from the already stored
    // open-area transmittance instead of reusing the input diameter/spacing
    // division. It likewise reconstructs cylinder reflectance from the
    // solid-fraction-adjusted assembly reflectance.
    let transmittance = fields.direct_normal_transmittance;
    if !transmittance.is_finite() || !(0.0..1.0).contains(&transmittance) {
        return Err(
            "WindowMaterial:Screen direct-normal transmittance must be finite and in [0,1)"
                .to_string(),
        );
    }
    let solid_fraction = 1.0 - transmittance;
    let diameter_to_spacing_ratio = 1.0 - transmittance.sqrt();
    let cylinder_solar_reflectance = fields.solar_reflectance / solid_fraction;
    let cylinder_visible_reflectance = fields.visible_reflectance / solid_fraction;
    if !diameter_to_spacing_ratio.is_finite()
        || diameter_to_spacing_ratio <= 0.0
        || diameter_to_spacing_ratio >= 1.0
    {
        return Err(
            "WindowMaterial:Screen source-reconstructed diameter-to-spacing ratio must be in (0,1)"
                .to_string(),
        );
    }
    if !cylinder_solar_reflectance.is_finite()
        || !cylinder_visible_reflectance.is_finite()
        || cylinder_solar_reflectance <= 0.0
        || cylinder_visible_reflectance <= 0.0
    {
        return Err(
            "bounded WindowMaterial:Screen EIO source-optics replay requires positive solar and visible reflectance; EnergyPlus CalcScreenTransmittance has a zero-reflectance NaN branch"
                .to_string(),
        );
    }

    Ok(ScreenStaticInputs {
        diameter_to_spacing_ratio,
        cylinder_solar_reflectance,
        cylinder_visible_reflectance,
    })
}

fn calculate_screen_beam_optics(
    reflectance_model: WindowScreenBeamReflectanceModel,
    inputs: ScreenStaticInputs,
    mut phi: f64,
    mut theta: f64,
) -> Result<ScreenBeamOptics, String> {
    if !phi.is_finite()
        || !theta.is_finite()
        || !(0.0..=std::f64::consts::PI).contains(&phi)
        || !(0.0..=std::f64::consts::PI).contains(&theta)
    {
        return Err(
            "WindowMaterial:Screen source-optics angles must be finite and in [0,pi]".to_string(),
        );
    }

    let sin_phi = phi.sin();
    let cos_phi = phi.cos();
    let tan_phi = sin_phi / cos_phi;
    let cos_theta = theta.cos();
    let sun_in_front = phi < std::f64::consts::FRAC_PI_2 && theta < std::f64::consts::FRAC_PI_2;
    let gamma = inputs.diameter_to_spacing_ratio;

    if phi > std::f64::consts::FRAC_PI_2 {
        phi = std::f64::consts::PI - phi;
    }
    if theta > std::f64::consts::FRAC_PI_2 {
        theta = std::f64::consts::PI - theta;
    }

    let beta = std::f64::consts::FRAC_PI_2 - theta;
    let trans_y = if beta > SMALL && (phi - std::f64::consts::FRAC_PI_2).abs() > SMALL {
        let alpha_double_prime = (tan_phi / cos_theta).atan();
        (1.0 - gamma
            * (alpha_double_prime.cos()
                + alpha_double_prime.sin() * tan_phi * (1.0 + square(1.0 / beta.tan())).sqrt()))
        .max(0.0)
    } else {
        0.0
    };

    let cos_mu = (square(cos_phi) * square(cos_theta) + square(sin_phi)).sqrt();
    let trans_x = if cos_mu <= SMALL {
        1.0 - gamma
    } else {
        let epsilon = (cos_phi * cos_theta / cos_mu).acos();
        let eta = std::f64::consts::FRAC_PI_2 - epsilon;
        if epsilon.cos() != 0.0 && eta != 0.0 {
            let mu_prime = (cos_mu.acos().tan() / epsilon.cos()).atan();
            (1.0 - gamma
                * (mu_prime.cos()
                    + mu_prime.sin()
                        * cos_mu.acos().tan()
                        * (1.0 + square(1.0 / eta.tan())).sqrt()))
            .max(0.0)
        } else {
            0.0
        }
    };
    let direct_transmittance = (trans_x * trans_y).max(0.0);

    let cylinder_solar_reflectance = inputs.cylinder_solar_reflectance;
    let cylinder_visible_reflectance = inputs.cylinder_visible_reflectance;
    let (mut scattered_solar, mut scattered_visible) = if std::f64::consts::FRAC_PI_2 - theta
        < SMALL
        || std::f64::consts::FRAC_PI_2 - phi < SMALL
    {
        (0.0, 0.0)
    } else {
        let delta_max = 89.7 - 10.0 * gamma / 0.16;
        let delta = (square(theta / DEG_TO_RAD) + square(phi / DEG_TO_RAD)).sqrt();
        let scatter_max = 0.0229 * gamma + 0.2971 * cylinder_solar_reflectance
            - 0.03624 * square(gamma)
            + 0.04763 * square(cylinder_solar_reflectance)
            - 0.44416 * gamma * cylinder_solar_reflectance;
        let scatter_max_visible = 0.0229 * gamma + 0.2971 * cylinder_visible_reflectance
            - 0.03624 * square(gamma)
            + 0.04763 * square(cylinder_visible_reflectance)
            - 0.44416 * gamma * cylinder_visible_reflectance;
        let exponent_interior = -square(delta - delta_max) / 600.0;
        let exponent_exterior = -(delta - delta_max).abs().powf(2.5) / 600.0;
        let peak_to_plateau = 1.0 / (0.2 * (1.0 - gamma) * cylinder_solar_reflectance);
        let peak_to_plateau_visible = 1.0 / (0.2 * (1.0 - gamma) * cylinder_visible_reflectance);
        if delta > delta_max {
            let mut solar = 0.2
                * (1.0 - gamma)
                * cylinder_solar_reflectance
                * scatter_max
                * (1.0 + (peak_to_plateau - 1.0) * exponent_exterior.exp());
            let mut visible = 0.2
                * (1.0 - gamma)
                * cylinder_visible_reflectance
                * scatter_max_visible
                * (1.0 + (peak_to_plateau_visible - 1.0) * exponent_exterior.exp());
            solar -= 0.2
                * (1.0 - gamma)
                * cylinder_solar_reflectance
                * scatter_max
                * ((delta - delta_max) / (90.0 - delta_max)).max(0.0);
            visible -= 0.2
                * (1.0 - gamma)
                * cylinder_visible_reflectance
                * scatter_max_visible
                * ((delta - delta_max) / (90.0 - delta_max)).max(0.0);
            (solar, visible)
        } else {
            (
                0.2 * (1.0 - gamma)
                    * cylinder_solar_reflectance
                    * scatter_max
                    * (1.0 + (peak_to_plateau - 1.0) * exponent_interior.exp()),
                0.2 * (1.0 - gamma)
                    * cylinder_visible_reflectance
                    * scatter_max_visible
                    * (1.0 + (peak_to_plateau_visible - 1.0) * exponent_interior.exp()),
            )
        }
    };

    let beam_solar_transmittance = match reflectance_model {
        WindowScreenBeamReflectanceModel::DoNotModel => {
            scattered_solar = 0.0;
            scattered_visible = 0.0;
            direct_transmittance
        }
        WindowScreenBeamReflectanceModel::ModelAsDirectBeam => {
            let beam = direct_transmittance + scattered_solar;
            scattered_solar = 0.0;
            scattered_visible = 0.0;
            beam
        }
        WindowScreenBeamReflectanceModel::ModelAsDiffuse => {
            scattered_solar = scattered_solar.max(0.0);
            scattered_visible = scattered_visible.max(0.0);
            direct_transmittance
        }
    };

    if !sun_in_front {
        return Ok(ScreenBeamOptics::default());
    }
    Ok(ScreenBeamOptics {
        beam_solar_transmittance,
        front_solar_reflectance: (cylinder_solar_reflectance * (1.0 - direct_transmittance)
            - scattered_solar)
            .max(0.0),
        front_visible_reflectance: (cylinder_visible_reflectance * (1.0 - direct_transmittance)
            - scattered_visible)
            .max(0.0),
    })
}

const fn square(value: f64) -> f64 {
    value * value
}
