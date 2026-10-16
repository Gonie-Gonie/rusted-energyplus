//! Literal opaque/no-source/no-movable CalcOutsideSurfTemp9470-9764 expressions.
//! The old compatibility helpers have denominator fallbacks and are not invoked.
use super::{
    Error, OrdinarySurfaceInitialization, SourceOutsideCtfOwners, index, outside_history, set,
    value, write_outside_history,
};
use crate::heat_balance::selected_heat_balance_input::surface::SurfaceClass;

pub(super) fn calculate(
    surface: &mut OrdinarySurfaceInitialization,
    o: &mut SourceOutsideCtfOwners<'_, '_>,
    number: i32,
    space: i32,
    construction: i32,
    h_movable: f64,
    temperature_ext: f64,
) -> Result<(), Error> {
    let i = index(number)?;
    let usage = o
        .control
        .construction_usage
        .get(index(construction)?)
        .ok_or(Error::Unavailable(
            "CalcOutsideSurfTemp actual Construction",
        ))?;
    let id = usage.construction_id.ok_or(Error::Unavailable(
        "CalcOutsideSurfTemp actual returned Construction identity",
    ))?;
    let c = &o
        .public
        .constructions
        .iter()
        .find(|row| row.construction_id == id)
        .ok_or(Error::Unavailable(
            "CalcOutsideSurfTemp same public Construction",
        ))?
        .result
        .as_ref()
        .map_err(|_| {
            Error::Unavailable("CalcOutsideSurfTemp unavailable actual public coefficients")
        })?
        .storage;
    let movable_present = h_movable > 0.0;
    // F1 is truly unwritten when the strict Native .01 branch is false.
    let f1 = if c.cross[0] > 0.01 {
        Some(c.cross[0] / (c.inside[0] + value(&surface.arrays.surf_h_conv_int, i)?))
    } else {
        None
    };
    let t_sky = o.environment.current_weather.sky_temp;
    let t_ground = o.environment.current_weather.out_dry_bulb_temp;
    let t_surround = 0.0;
    let actual = o
        .geometry
        .final_surfaces
        .as_ref()
        .and_then(|rows| rows.get(i))
        .ok_or(Error::Unavailable("CalcOutsideSurfTemp actual Surface"))?;
    if actual.has_surrounding_surface_property {
        return Err(Error::Unavailable(
            "CalcOutsideSurfTemp actual surrounding temperatures9538",
        ));
    }
    if actual.use_ground_surface_temperature {
        return Err(Error::Unavailable(
            "CalcOutsideSurfTemp actual GroundSurfsProperty temperature9549",
        ));
    }
    if actual.row.class == SurfaceClass::TddDome {
        return Err(Error::Unavailable(
            "CalcOutsideSurfTemp actual TDD_Dome PipeNum9569",
        ));
    }
    if movable_present {
        return Err(Error::Unavailable(
            "CalcOutsideSurfTemp actual positive movable F2 branch9685",
        ));
    }
    if actual.oscm_ptr != 0 {
        return Err(Error::Unavailable(
            "CalcOutsideSurfTemp actual positive OSCM radiation owner9607",
        ));
    }
    if usage.source_sink_present {
        return Err(Error::Unavailable(
            "CalcOutsideSurfTemp actual positive source/sink history9589",
        ));
    }
    let temperature = if let Some(f1) = f1 {
        // Original quick/no-source/no-OSCM9600..9660. Preserve every add and
        // multiplication grouping, including surrounding0 terms and live MAT.
        (-value(&surface.arrays.surf_ctf_const_out_part, i)?
            + value(&surface.arrays.surf_opaq_q_rad_sw_out_abs, i)?
            + value(&surface.arrays.surf_h_srd_surf_ext, i)? * t_surround
            + (value(&surface.arrays.surf_h_conv_ext, i)?
                + value(&surface.arrays.surf_h_air_ext, i)?)
                * temperature_ext
            + value(&surface.arrays.surf_q_additional_heat_source_outside, i)?
            + value(&surface.arrays.surf_h_sky_ext, i)? * t_sky
            + value(&surface.arrays.surf_h_grd_ext, i)? * t_ground
            + f1 * (value(&surface.arrays.surf_ctf_const_in_part, i)?
                + value(&surface.arrays.surf_opaq_q_rad_sw_in_abs, i)?
                + value(&surface.arrays.surf_qdot_rad_int_gains_in_per_area, i)?
                + value(&surface.arrays.surf_h_conv_int, i)? * *o.air.source_space_mat(space)?
                + value(&surface.arrays.surf_qdot_rad_net_lw_in_per_area, i)?))
            / (c.outside[0]
                + value(&surface.arrays.surf_h_conv_ext, i)?
                + value(&surface.arrays.surf_h_air_ext, i)?
                + value(&surface.arrays.surf_h_sky_ext, i)?
                + value(&surface.arrays.surf_h_grd_ext, i)?
                + value(&surface.arrays.surf_h_srd_surf_ext, i)?
                - f1 * c.cross[0])
    } else {
        (-value(&surface.arrays.surf_ctf_const_out_part, i)?
            + value(&surface.arrays.surf_opaq_q_rad_sw_out_abs, i)?
            + value(&surface.arrays.surf_h_srd_surf_ext, i)? * t_surround
            + (value(&surface.arrays.surf_h_conv_ext, i)?
                + value(&surface.arrays.surf_h_air_ext, i)?)
                * temperature_ext
            + value(&surface.arrays.surf_q_additional_heat_source_outside, i)?
            + value(&surface.arrays.surf_h_sky_ext, i)? * t_sky
            + value(&surface.arrays.surf_h_grd_ext, i)? * t_ground
            + c.cross[0] * value(&surface.arrays.surf_temp_in, i)?)
            / (c.outside[0]
                + value(&surface.arrays.surf_h_conv_ext, i)?
                + value(&surface.arrays.surf_h_air_ext, i)?
                + value(&surface.arrays.surf_h_sky_ext, i)?
                + value(&surface.arrays.surf_h_grd_ext, i)?
                + value(&surface.arrays.surf_h_srd_surf_ext, i)?)
    };
    write_outside_history(o, number, temperature)?;
    // Original report reads the just-written TH11 field, rather than a retained
    // old Surface temperature, and preserves the literal leading unary minus.
    let report = -(value(&surface.arrays.surf_h_sky_ext, i)?
        * (outside_history(o, number)? - t_sky)
        + value(&surface.arrays.surf_h_air_ext, i)?
            * (outside_history(o, number)? - temperature_ext)
        + value(&surface.arrays.surf_h_grd_ext, i)? * (outside_history(o, number)? - t_ground)
        + value(&surface.arrays.surf_h_srd_surf_ext, i)?
            * (outside_history(o, number)? - t_surround));
    set(
        &mut surface.arrays.surf_qdot_rad_out_rep_per_area,
        i,
        report,
    )?;
    Ok(())
}
