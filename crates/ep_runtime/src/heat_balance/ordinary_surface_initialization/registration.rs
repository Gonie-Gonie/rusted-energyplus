//! HBSM1607-2205 Source registrations, preserving all reached opaque calls.
//! All actual references are evaluated before Setup; retained positive real
//! targets resolve against the same live arrays and existing history reports.
use super::output::{SourceSurfaceOutputOwners, index};
use crate::heat_balance::radiation::ordinary::{
    OrdinaryRadiationError as Error, OrdinaryRadiationUnavailable as Unavailable,
};
use crate::heat_balance::selected_heat_balance_input::SelectedHeatBalanceInputState;
use crate::heat_balance::selected_heat_balance_input::surface::SurfaceClass;
use crate::heat_balance::source_output_prescan::SourceOutputPreScan;
use crate::heat_balance::source_output_registration::{
    SourceIntegerOutputDeclaration, SourceOutputDictionaryHost, SourceOutputFileHost,
    SourceOutputRegistration, SourceOutputStoreType, SourceOutputUnits, SourceRealOutputBinding,
    SourceRealOutputDeclaration, SourceRealOutputTarget, SourceReportVariableInput,
    SourceSurfaceOutputField,
};
use std::io::Write;

pub(crate) fn register_source_surface_outputs(
    owners: &SourceSurfaceOutputOwners<'_>,
    constructions: &SelectedHeatBalanceInputState,
    advanced_report_variables: bool,
    registration: &mut SourceOutputRegistration,
    input: &mut SourceReportVariableInput<'_>,
    prescan: &SourceOutputPreScan,
    minutes_in_time_step: i32,
    host: &mut impl SourceOutputFileHost,
    eio: &mut dyn Write,
    dictionary_host: &mut impl SourceOutputDictionaryHost,
) -> Result<(), Error> {
    let surfaces = owners
        .geometry
        .final_surfaces
        .as_ref()
        .ok_or(Unavailable::StorageDomain)?;
    let parallel = owners
        .geometry
        .parallel_arrays
        .as_ref()
        .ok_or(Unavailable::StorageDomain)?;
    let total =
        usize::try_from(owners.geometry.total_surfaces).map_err(|_| Unavailable::StorageDomain)?;
    for i in 0..total {
        let surface = surfaces.get(i).ok_or(Unavailable::StorageDomain)?;
        if !surface.row.heat_transfer_surface {
            continue;
        }
        let number = i32::try_from(i + 1).map_err(|_| Unavailable::StorageDomain)?;
        let binding = SourceRealOutputBinding::new(
            SourceRealOutputTarget::SurfaceHeatBalance {
                field: SourceSurfaceOutputField::SurfTempIn,
                index: number,
            },
            owners,
        )?;
        registration.setup_bound_unmetered_zone_real(
            input,
            prescan,
            SourceRealOutputDeclaration {
                name: "Surface Inside Face Temperature",
                key: &surface.row.name,
                units: SourceOutputUnits::C,
                store_type: SourceOutputStoreType::Average,
            },
            minutes_in_time_step,
            host,
            eio,
            binding,
            dictionary_host,
        )?;
        let _actual: &f64 = owners
            .arrays
            .surf_temp_in_mov_ins_rep
            .as_ref()
            .and_then(|values| values.get(i))
            .ok_or(Unavailable::StorageDomain)?;
        registration.setup_unmetered_zone_real(
            input,
            prescan,
            SourceRealOutputDeclaration {
                name: "Surface Inside Face Interior Movable Insulation Temperature",
                key: &surface.row.name,
                units: SourceOutputUnits::C,
                store_type: SourceOutputStoreType::Average,
            },
            minutes_in_time_step,
            host,
            eio,
        )?;
        if surface.row.exterior_boundary != -6 {
            let binding = SourceRealOutputBinding::new(
                SourceRealOutputTarget::SurfaceHeatBalance {
                    field: SourceSurfaceOutputField::SurfTempOut,
                    index: number,
                },
                owners,
            )?;
            registration.setup_bound_unmetered_zone_real(
                input,
                prescan,
                SourceRealOutputDeclaration {
                    name: "Surface Outside Face Temperature",
                    key: &surface.row.name,
                    units: SourceOutputUnits::C,
                    store_type: SourceOutputStoreType::Average,
                },
                minutes_in_time_step,
                host,
                eio,
                binding,
                dictionary_host,
            )?;
        }
        let _actual: &f64 = owners
            .arrays
            .surf_temp_eff_bulk_air
            .as_ref()
            .and_then(|values| values.get(i))
            .ok_or(Unavailable::StorageDomain)?;
        registration.setup_unmetered_zone_real(
            input,
            prescan,
            SourceRealOutputDeclaration {
                name: "Surface Inside Face Adjacent Air Temperature",
                key: &surface.row.name,
                units: SourceOutputUnits::C,
                store_type: SourceOutputStoreType::Average,
            },
            minutes_in_time_step,
            host,
            eio,
        )?;
        let _actual: &f64 = owners
            .arrays
            .surf_h_conv_int
            .as_ref()
            .and_then(|values| values.get(i))
            .ok_or(Unavailable::StorageDomain)?;
        registration.setup_unmetered_zone_real(
            input,
            prescan,
            SourceRealOutputDeclaration {
                name: "Surface Inside Face Convection Heat Transfer Coefficient",
                key: &surface.row.name,
                units: SourceOutputUnits::WPerM2K,
                store_type: SourceOutputStoreType::Average,
            },
            minutes_in_time_step,
            host,
            eio,
        )?;
        let _actual: &f64 = owners
            .arrays
            .surf_qdot_conv_in_rep
            .as_ref()
            .and_then(|values| values.get(i))
            .ok_or(Unavailable::StorageDomain)?;
        registration.setup_unmetered_zone_real(
            input,
            prescan,
            SourceRealOutputDeclaration {
                name: "Surface Inside Face Convection Heat Gain Rate",
                key: &surface.row.name,
                units: SourceOutputUnits::W,
                store_type: SourceOutputStoreType::Average,
            },
            minutes_in_time_step,
            host,
            eio,
        )?;
        let _actual: &f64 = owners
            .arrays
            .surf_qdot_conv_in_per_area
            .as_ref()
            .and_then(|values| values.get(i))
            .ok_or(Unavailable::StorageDomain)?;
        registration.setup_unmetered_zone_real(
            input,
            prescan,
            SourceRealOutputDeclaration {
                name: "Surface Inside Face Convection Heat Gain Rate per Area",
                key: &surface.row.name,
                units: SourceOutputUnits::WPerM2,
                store_type: SourceOutputStoreType::Average,
            },
            minutes_in_time_step,
            host,
            eio,
        )?;
        let _actual: &f64 = owners
            .arrays
            .surf_q_conv_in_rep
            .as_ref()
            .and_then(|values| values.get(i))
            .ok_or(Unavailable::StorageDomain)?;
        registration.setup_unmetered_zone_real(
            input,
            prescan,
            SourceRealOutputDeclaration {
                name: "Surface Inside Face Convection Heat Gain Energy",
                key: &surface.row.name,
                units: SourceOutputUnits::J,
                store_type: SourceOutputStoreType::Sum,
            },
            minutes_in_time_step,
            host,
            eio,
        )?;
        let _actual: &f64 = owners
            .arrays
            .surf_qdot_rad_net_surf_in_rep
            .as_ref()
            .and_then(|values| values.get(i))
            .ok_or(Unavailable::StorageDomain)?;
        registration.setup_unmetered_zone_real(
            input,
            prescan,
            SourceRealOutputDeclaration {
                name: "Surface Inside Face Net Surface Thermal Radiation Heat Gain Rate",
                key: &surface.row.name,
                units: SourceOutputUnits::W,
                store_type: SourceOutputStoreType::Average,
            },
            minutes_in_time_step,
            host,
            eio,
        )?;
        let _actual: &f64 = owners
            .arrays
            .surf_qdot_rad_net_lw_in_per_area
            .as_ref()
            .and_then(|values| values.get(i))
            .ok_or(Unavailable::StorageDomain)?;
        registration.setup_unmetered_zone_real(
            input,
            prescan,
            SourceRealOutputDeclaration {
                name: "Surface Inside Face Net Surface Thermal Radiation Heat Gain Rate per Area",
                key: &surface.row.name,
                units: SourceOutputUnits::WPerM2,
                store_type: SourceOutputStoreType::Average,
            },
            minutes_in_time_step,
            host,
            eio,
        )?;
        let _actual: &f64 = owners
            .arrays
            .surf_q_rad_net_surf_in_rep
            .as_ref()
            .and_then(|values| values.get(i))
            .ok_or(Unavailable::StorageDomain)?;
        registration.setup_unmetered_zone_real(
            input,
            prescan,
            SourceRealOutputDeclaration {
                name: "Surface Inside Face Net Surface Thermal Radiation Heat Gain Energy",
                key: &surface.row.name,
                units: SourceOutputUnits::J,
                store_type: SourceOutputStoreType::Sum,
            },
            minutes_in_time_step,
            host,
            eio,
        )?;
        if surface.row.class != SurfaceClass::Window {
            let _actual: &f64 = owners
                .arrays
                .surf_qdot_rad_solar_in_rep
                .as_ref()
                .and_then(|values| values.get(i))
                .ok_or(Unavailable::StorageDomain)?;
            registration.setup_unmetered_zone_real(
                input,
                prescan,
                SourceRealOutputDeclaration {
                    name: "Surface Inside Face Solar Radiation Heat Gain Rate",
                    key: &surface.row.name,
                    units: SourceOutputUnits::W,
                    store_type: SourceOutputStoreType::Average,
                },
                minutes_in_time_step,
                host,
                eio,
            )?;
            let _actual: &f64 = owners
                .arrays
                .surf_qdot_rad_solar_in_rep_per_area
                .as_ref()
                .and_then(|values| values.get(i))
                .ok_or(Unavailable::StorageDomain)?;
            registration.setup_unmetered_zone_real(
                input,
                prescan,
                SourceRealOutputDeclaration {
                    name: "Surface Inside Face Solar Radiation Heat Gain Rate per Area",
                    key: &surface.row.name,
                    units: SourceOutputUnits::WPerM2,
                    store_type: SourceOutputStoreType::Average,
                },
                minutes_in_time_step,
                host,
                eio,
            )?;
            let _actual: &f64 = owners
                .arrays
                .surf_q_rad_solar_in_rep
                .as_ref()
                .and_then(|values| values.get(i))
                .ok_or(Unavailable::StorageDomain)?;
            registration.setup_unmetered_zone_real(
                input,
                prescan,
                SourceRealOutputDeclaration {
                    name: "Surface Inside Face Solar Radiation Heat Gain Energy",
                    key: &surface.row.name,
                    units: SourceOutputUnits::J,
                    store_type: SourceOutputStoreType::Sum,
                },
                minutes_in_time_step,
                host,
                eio,
            )?;
            let _actual: &f64 = owners
                .arrays
                .surf_qdot_rad_lights_in_rep
                .as_ref()
                .and_then(|values| values.get(i))
                .ok_or(Unavailable::StorageDomain)?;
            registration.setup_unmetered_zone_real(
                input,
                prescan,
                SourceRealOutputDeclaration {
                    name: "Surface Inside Face Lights Radiation Heat Gain Rate",
                    key: &surface.row.name,
                    units: SourceOutputUnits::W,
                    store_type: SourceOutputStoreType::Average,
                },
                minutes_in_time_step,
                host,
                eio,
            )?;
            let _actual: &f64 = owners
                .arrays
                .surf_qdot_rad_lights_in_per_area
                .as_ref()
                .and_then(|values| values.get(i))
                .ok_or(Unavailable::StorageDomain)?;
            registration.setup_unmetered_zone_real(
                input,
                prescan,
                SourceRealOutputDeclaration {
                    name: "Surface Inside Face Lights Radiation Heat Gain Rate per Area",
                    key: &surface.row.name,
                    units: SourceOutputUnits::WPerM2,
                    store_type: SourceOutputStoreType::Average,
                },
                minutes_in_time_step,
                host,
                eio,
            )?;
            let _actual: &f64 = owners
                .arrays
                .surf_q_rad_lights_in_rep
                .as_ref()
                .and_then(|values| values.get(i))
                .ok_or(Unavailable::StorageDomain)?;
            registration.setup_unmetered_zone_real(
                input,
                prescan,
                SourceRealOutputDeclaration {
                    name: "Surface Inside Face Lights Radiation Heat Gain Energy",
                    key: &surface.row.name,
                    units: SourceOutputUnits::J,
                    store_type: SourceOutputStoreType::Sum,
                },
                minutes_in_time_step,
                host,
                eio,
            )?;
        }
        let _actual: &f64 = owners
            .arrays
            .surf_qdot_rad_int_gains_in_rep
            .as_ref()
            .and_then(|values| values.get(i))
            .ok_or(Unavailable::StorageDomain)?;
        registration.setup_unmetered_zone_real(
            input,
            prescan,
            SourceRealOutputDeclaration {
                name: "Surface Inside Face Internal Gains Radiation Heat Gain Rate",
                key: &surface.row.name,
                units: SourceOutputUnits::W,
                store_type: SourceOutputStoreType::Average,
            },
            minutes_in_time_step,
            host,
            eio,
        )?;
        let _actual: &f64 = owners
            .arrays
            .surf_qdot_rad_int_gains_in_per_area
            .as_ref()
            .and_then(|values| values.get(i))
            .ok_or(Unavailable::StorageDomain)?;
        registration.setup_unmetered_zone_real(
            input,
            prescan,
            SourceRealOutputDeclaration {
                name: "Surface Inside Face Internal Gains Radiation Heat Gain Rate per Area",
                key: &surface.row.name,
                units: SourceOutputUnits::WPerM2,
                store_type: SourceOutputStoreType::Average,
            },
            minutes_in_time_step,
            host,
            eio,
        )?;
        let _actual: &f64 = owners
            .arrays
            .surf_q_rad_int_gains_in_rep
            .as_ref()
            .and_then(|values| values.get(i))
            .ok_or(Unavailable::StorageDomain)?;
        registration.setup_unmetered_zone_real(
            input,
            prescan,
            SourceRealOutputDeclaration {
                name: "Surface Inside Face Internal Gains Radiation Heat Gain Energy",
                key: &surface.row.name,
                units: SourceOutputUnits::J,
                store_type: SourceOutputStoreType::Sum,
            },
            minutes_in_time_step,
            host,
            eio,
        )?;
        let _actual: &f64 = owners
            .arrays
            .surf_qdot_rad_hvac_in_rep
            .as_ref()
            .and_then(|values| values.get(i))
            .ok_or(Unavailable::StorageDomain)?;
        registration.setup_unmetered_zone_real(
            input,
            prescan,
            SourceRealOutputDeclaration {
                name: "Surface Inside Face System Radiation Heat Gain Rate",
                key: &surface.row.name,
                units: SourceOutputUnits::W,
                store_type: SourceOutputStoreType::Average,
            },
            minutes_in_time_step,
            host,
            eio,
        )?;
        let _actual: &f64 = owners
            .arrays
            .surf_qdot_rad_hvac_in_per_area
            .as_ref()
            .and_then(|values| values.get(i))
            .ok_or(Unavailable::StorageDomain)?;
        registration.setup_unmetered_zone_real(
            input,
            prescan,
            SourceRealOutputDeclaration {
                name: "Surface Inside Face System Radiation Heat Gain Rate per Area",
                key: &surface.row.name,
                units: SourceOutputUnits::WPerM2,
                store_type: SourceOutputStoreType::Average,
            },
            minutes_in_time_step,
            host,
            eio,
        )?;
        let _actual: &f64 = owners
            .arrays
            .surf_q_rad_hvac_in_rep
            .as_ref()
            .and_then(|values| values.get(i))
            .ok_or(Unavailable::StorageDomain)?;
        registration.setup_unmetered_zone_real(
            input,
            prescan,
            SourceRealOutputDeclaration {
                name: "Surface Inside Face System Radiation Heat Gain Energy",
                key: &surface.row.name,
                units: SourceOutputUnits::J,
                store_type: SourceOutputStoreType::Sum,
            },
            minutes_in_time_step,
            host,
            eio,
        )?;
        if surface.row.exterior_boundary == 0 || advanced_report_variables {
            let _actual: &f64 = &owners
                .physical_surface(i)?
                .outdoor_environment
                .row
                .dry_bulb_c;
            registration.setup_unmetered_zone_real(
                input,
                prescan,
                SourceRealOutputDeclaration {
                    name: "Surface Outside Face Outdoor Air Drybulb Temperature",
                    key: &surface.row.name,
                    units: SourceOutputUnits::C,
                    store_type: SourceOutputStoreType::Average,
                },
                minutes_in_time_step,
                host,
                eio,
            )?;
            let _actual: &f64 = &owners
                .physical_surface(i)?
                .outdoor_environment
                .row
                .wet_bulb_c;
            registration.setup_unmetered_zone_real(
                input,
                prescan,
                SourceRealOutputDeclaration {
                    name: "Surface Outside Face Outdoor Air Wetbulb Temperature",
                    key: &surface.row.name,
                    units: SourceOutputUnits::C,
                    store_type: SourceOutputStoreType::Average,
                },
                minutes_in_time_step,
                host,
                eio,
            )?;
            let _actual: &f64 = &owners
                .physical_surface(i)?
                .outdoor_environment
                .row
                .wind_speed_m_per_s;
            registration.setup_unmetered_zone_real(
                input,
                prescan,
                SourceRealOutputDeclaration {
                    name: "Surface Outside Face Outdoor Air Wind Speed",
                    key: &surface.row.name,
                    units: SourceOutputUnits::MPerS,
                    store_type: SourceOutputStoreType::Average,
                },
                minutes_in_time_step,
                host,
                eio,
            )?;
            let _actual: &f64 = &owners
                .physical_surface(i)?
                .outdoor_environment
                .row
                .wind_direction_deg;
            registration.setup_unmetered_zone_real(
                input,
                prescan,
                SourceRealOutputDeclaration {
                    name: "Surface Outside Face Outdoor Air Wind Direction",
                    key: &surface.row.name,
                    units: SourceOutputUnits::Deg,
                    store_type: SourceOutputStoreType::Average,
                },
                minutes_in_time_step,
                host,
                eio,
            )?;
            let _actual: &f64 = owners
                .arrays
                .surf_qdot_conv_out_rep
                .as_ref()
                .and_then(|values| values.get(i))
                .ok_or(Unavailable::StorageDomain)?;
            registration.setup_unmetered_zone_real(
                input,
                prescan,
                SourceRealOutputDeclaration {
                    name: "Surface Outside Face Convection Heat Gain Rate",
                    key: &surface.row.name,
                    units: SourceOutputUnits::W,
                    store_type: SourceOutputStoreType::Average,
                },
                minutes_in_time_step,
                host,
                eio,
            )?;
            let _actual: &f64 = owners
                .arrays
                .surf_qdot_conv_out_per_area
                .as_ref()
                .and_then(|values| values.get(i))
                .ok_or(Unavailable::StorageDomain)?;
            registration.setup_unmetered_zone_real(
                input,
                prescan,
                SourceRealOutputDeclaration {
                    name: "Surface Outside Face Convection Heat Gain Rate per Area",
                    key: &surface.row.name,
                    units: SourceOutputUnits::WPerM2,
                    store_type: SourceOutputStoreType::Average,
                },
                minutes_in_time_step,
                host,
                eio,
            )?;
            let _actual: &f64 = owners
                .arrays
                .surf_q_conv_out_report
                .as_ref()
                .and_then(|values| values.get(i))
                .ok_or(Unavailable::StorageDomain)?;
            registration.setup_unmetered_zone_real(
                input,
                prescan,
                SourceRealOutputDeclaration {
                    name: "Surface Outside Face Convection Heat Gain Energy",
                    key: &surface.row.name,
                    units: SourceOutputUnits::J,
                    store_type: SourceOutputStoreType::Sum,
                },
                minutes_in_time_step,
                host,
                eio,
            )?;
            let _actual: &f64 = owners
                .arrays
                .surf_h_conv_ext
                .as_ref()
                .and_then(|values| values.get(i))
                .ok_or(Unavailable::StorageDomain)?;
            registration.setup_unmetered_zone_real(
                input,
                prescan,
                SourceRealOutputDeclaration {
                    name: "Surface Outside Face Convection Heat Transfer Coefficient",
                    key: &surface.row.name,
                    units: SourceOutputUnits::WPerM2K,
                    store_type: SourceOutputStoreType::Average,
                },
                minutes_in_time_step,
                host,
                eio,
            )?;
            let _actual: &f64 = owners
                .arrays
                .surf_qdot_rad_out_rep
                .as_ref()
                .and_then(|values| values.get(i))
                .ok_or(Unavailable::StorageDomain)?;
            registration.setup_unmetered_zone_real(
                input,
                prescan,
                SourceRealOutputDeclaration {
                    name: "Surface Outside Face Net Thermal Radiation Heat Gain Rate",
                    key: &surface.row.name,
                    units: SourceOutputUnits::W,
                    store_type: SourceOutputStoreType::Average,
                },
                minutes_in_time_step,
                host,
                eio,
            )?;
            let _actual: &f64 = owners
                .arrays
                .surf_qdot_rad_out_rep_per_area
                .as_ref()
                .and_then(|values| values.get(i))
                .ok_or(Unavailable::StorageDomain)?;
            registration.setup_unmetered_zone_real(
                input,
                prescan,
                SourceRealOutputDeclaration {
                    name: "Surface Outside Face Net Thermal Radiation Heat Gain Rate per Area",
                    key: &surface.row.name,
                    units: SourceOutputUnits::WPerM2,
                    store_type: SourceOutputStoreType::Average,
                },
                minutes_in_time_step,
                host,
                eio,
            )?;
            let _actual: &f64 = owners
                .arrays
                .surf_q_rad_out_report
                .as_ref()
                .and_then(|values| values.get(i))
                .ok_or(Unavailable::StorageDomain)?;
            registration.setup_unmetered_zone_real(
                input,
                prescan,
                SourceRealOutputDeclaration {
                    name: "Surface Outside Face Net Thermal Radiation Heat Gain Energy",
                    key: &surface.row.name,
                    units: SourceOutputUnits::J,
                    store_type: SourceOutputStoreType::Sum,
                },
                minutes_in_time_step,
                host,
                eio,
            )?;
            let _actual: &f64 = owners
                .arrays
                .surf_h_air_ext
                .as_ref()
                .and_then(|values| values.get(i))
                .ok_or(Unavailable::StorageDomain)?;
            registration.setup_unmetered_zone_real(
                input,
                prescan,
                SourceRealOutputDeclaration {
                    name: "Surface Outside Face Thermal Radiation to Air Heat Transfer Coefficient",
                    key: &surface.row.name,
                    units: SourceOutputUnits::WPerM2K,
                    store_type: SourceOutputStoreType::Average,
                },
                minutes_in_time_step,
                host,
                eio,
            )?;
            let _actual: &f64 = owners
                .arrays
                .surf_h_sky_ext
                .as_ref()
                .and_then(|values| values.get(i))
                .ok_or(Unavailable::StorageDomain)?;
            registration.setup_unmetered_zone_real(
                input,
                prescan,
                SourceRealOutputDeclaration {
                    name: "Surface Outside Face Thermal Radiation to Sky Heat Transfer Coefficient",
                    key: &surface.row.name,
                    units: SourceOutputUnits::WPerM2K,
                    store_type: SourceOutputStoreType::Average,
                },
                minutes_in_time_step,
                host,
                eio,
            )?;
            let _actual: &f64 = owners
                .arrays
                .surf_h_grd_ext
                .as_ref()
                .and_then(|values| values.get(i))
                .ok_or(Unavailable::StorageDomain)?;
            registration.setup_unmetered_zone_real(input, prescan, SourceRealOutputDeclaration { name: "Surface Outside Face Thermal Radiation to Ground Heat Transfer Coefficient", key: &surface.row.name, units: SourceOutputUnits::WPerM2K, store_type: SourceOutputStoreType::Average }, minutes_in_time_step, host, eio)?;
            let _actual: &f64 = owners
                .arrays
                .surf_q_air_ext_report
                .as_ref()
                .and_then(|values| values.get(i))
                .ok_or(Unavailable::StorageDomain)?;
            registration.setup_unmetered_zone_real(
                input,
                prescan,
                SourceRealOutputDeclaration {
                    name: "Surface Outside Face Thermal Radiation to Air Heat Transfer Rate",
                    key: &surface.row.name,
                    units: SourceOutputUnits::W,
                    store_type: SourceOutputStoreType::Average,
                },
                minutes_in_time_step,
                host,
                eio,
            )?;
            let _actual: &f64 = owners
                .arrays
                .surf_q_heat_emi_report
                .as_ref()
                .and_then(|values| values.get(i))
                .ok_or(Unavailable::StorageDomain)?;
            registration.setup_unmetered_zone_real(
                input,
                prescan,
                SourceRealOutputDeclaration {
                    name: "Surface Outside Face Heat Emission to Air Rate",
                    key: &surface.row.name,
                    units: SourceOutputUnits::W,
                    store_type: SourceOutputStoreType::Average,
                },
                minutes_in_time_step,
                host,
                eio,
            )?;
            if surface.row.class != SurfaceClass::Window {
                let _actual: &f64 = owners
                    .arrays
                    .surf_opaq_sw_out_abs_total_report
                    .as_ref()
                    .and_then(|values| values.get(i))
                    .ok_or(Unavailable::StorageDomain)?;
                registration.setup_unmetered_zone_real(
                    input,
                    prescan,
                    SourceRealOutputDeclaration {
                        name: "Surface Outside Face Solar Radiation Heat Gain Rate",
                        key: &surface.row.name,
                        units: SourceOutputUnits::W,
                        store_type: SourceOutputStoreType::Average,
                    },
                    minutes_in_time_step,
                    host,
                    eio,
                )?;
                let _actual: &f64 = owners
                    .arrays
                    .surf_opaq_q_rad_sw_out_abs
                    .as_ref()
                    .and_then(|values| values.get(i))
                    .ok_or(Unavailable::StorageDomain)?;
                registration.setup_unmetered_zone_real(
                    input,
                    prescan,
                    SourceRealOutputDeclaration {
                        name: "Surface Outside Face Solar Radiation Heat Gain Rate per Area",
                        key: &surface.row.name,
                        units: SourceOutputUnits::WPerM2,
                        store_type: SourceOutputStoreType::Average,
                    },
                    minutes_in_time_step,
                    host,
                    eio,
                )?;
                let _actual: &f64 = owners
                    .arrays
                    .surf_opaq_sw_out_abs_energy_report
                    .as_ref()
                    .and_then(|values| values.get(i))
                    .ok_or(Unavailable::StorageDomain)?;
                registration.setup_unmetered_zone_real(
                    input,
                    prescan,
                    SourceRealOutputDeclaration {
                        name: "Surface Outside Face Solar Radiation Heat Gain Energy",
                        key: &surface.row.name,
                        units: SourceOutputUnits::J,
                        store_type: SourceOutputStoreType::Sum,
                    },
                    minutes_in_time_step,
                    host,
                    eio,
                )?;
            }
        }
        if matches!(
            surface.row.class,
            SurfaceClass::Floor
                | SurfaceClass::Wall
                | SurfaceClass::IntMass
                | SurfaceClass::Roof
                | SurfaceClass::Door
        ) {
            let binding = SourceRealOutputBinding::new(
                SourceRealOutputTarget::SurfaceHeatBalance {
                    field: SourceSurfaceOutputField::SurfOpaqInsFaceCond,
                    index: number,
                },
                owners,
            )?;
            registration.setup_bound_unmetered_zone_real(
                input,
                prescan,
                SourceRealOutputDeclaration {
                    name: "Surface Inside Face Conduction Heat Transfer Rate",
                    key: &surface.row.name,
                    units: SourceOutputUnits::W,
                    store_type: SourceOutputStoreType::Average,
                },
                minutes_in_time_step,
                host,
                eio,
                binding,
                dictionary_host,
            )?;
            let _actual: &f64 = owners
                .arrays
                .surf_opaq_ins_face_cond_gain_rep
                .as_ref()
                .and_then(|values| values.get(i))
                .ok_or(Unavailable::StorageDomain)?;
            registration.setup_unmetered_zone_real(
                input,
                prescan,
                SourceRealOutputDeclaration {
                    name: "Surface Inside Face Conduction Heat Gain Rate",
                    key: &surface.row.name,
                    units: SourceOutputUnits::W,
                    store_type: SourceOutputStoreType::Average,
                },
                minutes_in_time_step,
                host,
                eio,
            )?;
            let _actual: &f64 = owners
                .arrays
                .surf_opaq_ins_face_cond_loss_rep
                .as_ref()
                .and_then(|values| values.get(i))
                .ok_or(Unavailable::StorageDomain)?;
            registration.setup_unmetered_zone_real(
                input,
                prescan,
                SourceRealOutputDeclaration {
                    name: "Surface Inside Face Conduction Heat Loss Rate",
                    key: &surface.row.name,
                    units: SourceOutputUnits::W,
                    store_type: SourceOutputStoreType::Average,
                },
                minutes_in_time_step,
                host,
                eio,
            )?;
            let binding = SourceRealOutputBinding::new(
                SourceRealOutputTarget::SurfaceHeatBalance {
                    field: SourceSurfaceOutputField::SurfOpaqInsFaceCondFlux,
                    index: number,
                },
                owners,
            )?;
            registration.setup_bound_unmetered_zone_real(
                input,
                prescan,
                SourceRealOutputDeclaration {
                    name: "Surface Inside Face Conduction Heat Transfer Rate per Area",
                    key: &surface.row.name,
                    units: SourceOutputUnits::WPerM2,
                    store_type: SourceOutputStoreType::Average,
                },
                minutes_in_time_step,
                host,
                eio,
                binding,
                dictionary_host,
            )?;
            let _actual: &f64 = owners
                .arrays
                .surf_opaq_ins_face_cond_energy
                .as_ref()
                .and_then(|values| values.get(i))
                .ok_or(Unavailable::StorageDomain)?;
            registration.setup_unmetered_zone_real(
                input,
                prescan,
                SourceRealOutputDeclaration {
                    name: "Surface Inside Face Conduction Heat Transfer Energy",
                    key: &surface.row.name,
                    units: SourceOutputUnits::J,
                    store_type: SourceOutputStoreType::Sum,
                },
                minutes_in_time_step,
                host,
                eio,
            )?;
            if surface.row.exterior_boundary != -6 {
                let binding = SourceRealOutputBinding::new(
                    SourceRealOutputTarget::SurfaceHeatBalance {
                        field: SourceSurfaceOutputField::SurfOpaqOutFaceCond,
                        index: number,
                    },
                    owners,
                )?;
                registration.setup_bound_unmetered_zone_real(
                    input,
                    prescan,
                    SourceRealOutputDeclaration {
                        name: "Surface Outside Face Conduction Heat Transfer Rate",
                        key: &surface.row.name,
                        units: SourceOutputUnits::W,
                        store_type: SourceOutputStoreType::Average,
                    },
                    minutes_in_time_step,
                    host,
                    eio,
                    binding,
                    dictionary_host,
                )?;
                let _actual: &f64 = owners
                    .arrays
                    .surf_opaq_ext_face_cond_gain_rep
                    .as_ref()
                    .and_then(|values| values.get(i))
                    .ok_or(Unavailable::StorageDomain)?;
                registration.setup_unmetered_zone_real(
                    input,
                    prescan,
                    SourceRealOutputDeclaration {
                        name: "Surface Outside Face Conduction Heat Gain Rate",
                        key: &surface.row.name,
                        units: SourceOutputUnits::W,
                        store_type: SourceOutputStoreType::Average,
                    },
                    minutes_in_time_step,
                    host,
                    eio,
                )?;
                let _actual: &f64 = owners
                    .arrays
                    .surf_opaq_ext_face_cond_loss_rep
                    .as_ref()
                    .and_then(|values| values.get(i))
                    .ok_or(Unavailable::StorageDomain)?;
                registration.setup_unmetered_zone_real(
                    input,
                    prescan,
                    SourceRealOutputDeclaration {
                        name: "Surface Outside Face Conduction Heat Loss Rate",
                        key: &surface.row.name,
                        units: SourceOutputUnits::W,
                        store_type: SourceOutputStoreType::Average,
                    },
                    minutes_in_time_step,
                    host,
                    eio,
                )?;
                let binding = SourceRealOutputBinding::new(
                    SourceRealOutputTarget::SurfaceHeatBalance {
                        field: SourceSurfaceOutputField::SurfOpaqOutFaceCondFlux,
                        index: number,
                    },
                    owners,
                )?;
                registration.setup_bound_unmetered_zone_real(
                    input,
                    prescan,
                    SourceRealOutputDeclaration {
                        name: "Surface Outside Face Conduction Heat Transfer Rate per Area",
                        key: &surface.row.name,
                        units: SourceOutputUnits::WPerM2,
                        store_type: SourceOutputStoreType::Average,
                    },
                    minutes_in_time_step,
                    host,
                    eio,
                    binding,
                    dictionary_host,
                )?;
                let _actual: &f64 = owners
                    .arrays
                    .surf_opaq_out_face_cond_energy
                    .as_ref()
                    .and_then(|values| values.get(i))
                    .ok_or(Unavailable::StorageDomain)?;
                registration.setup_unmetered_zone_real(
                    input,
                    prescan,
                    SourceRealOutputDeclaration {
                        name: "Surface Outside Face Conduction Heat Transfer Energy",
                        key: &surface.row.name,
                        units: SourceOutputUnits::J,
                        store_type: SourceOutputStoreType::Sum,
                    },
                    minutes_in_time_step,
                    host,
                    eio,
                )?;
                let _actual: &f64 = owners
                    .arrays
                    .surf_opaq_avg_face_cond
                    .as_ref()
                    .and_then(|values| values.get(i))
                    .ok_or(Unavailable::StorageDomain)?;
                registration.setup_unmetered_zone_real(
                    input,
                    prescan,
                    SourceRealOutputDeclaration {
                        name: "Surface Average Face Conduction Heat Transfer Rate",
                        key: &surface.row.name,
                        units: SourceOutputUnits::W,
                        store_type: SourceOutputStoreType::Average,
                    },
                    minutes_in_time_step,
                    host,
                    eio,
                )?;
                let _actual: &f64 = owners
                    .arrays
                    .surf_opaq_avg_face_cond_gain_rep
                    .as_ref()
                    .and_then(|values| values.get(i))
                    .ok_or(Unavailable::StorageDomain)?;
                registration.setup_unmetered_zone_real(
                    input,
                    prescan,
                    SourceRealOutputDeclaration {
                        name: "Surface Average Face Conduction Heat Gain Rate",
                        key: &surface.row.name,
                        units: SourceOutputUnits::W,
                        store_type: SourceOutputStoreType::Average,
                    },
                    minutes_in_time_step,
                    host,
                    eio,
                )?;
                let _actual: &f64 = owners
                    .arrays
                    .surf_opaq_avg_face_cond_loss_rep
                    .as_ref()
                    .and_then(|values| values.get(i))
                    .ok_or(Unavailable::StorageDomain)?;
                registration.setup_unmetered_zone_real(
                    input,
                    prescan,
                    SourceRealOutputDeclaration {
                        name: "Surface Average Face Conduction Heat Loss Rate",
                        key: &surface.row.name,
                        units: SourceOutputUnits::W,
                        store_type: SourceOutputStoreType::Average,
                    },
                    minutes_in_time_step,
                    host,
                    eio,
                )?;
                let _actual: &f64 = owners
                    .arrays
                    .surf_opaq_avg_face_cond_flux
                    .as_ref()
                    .and_then(|values| values.get(i))
                    .ok_or(Unavailable::StorageDomain)?;
                registration.setup_unmetered_zone_real(
                    input,
                    prescan,
                    SourceRealOutputDeclaration {
                        name: "Surface Average Face Conduction Heat Transfer Rate per Area",
                        key: &surface.row.name,
                        units: SourceOutputUnits::WPerM2,
                        store_type: SourceOutputStoreType::Average,
                    },
                    minutes_in_time_step,
                    host,
                    eio,
                )?;
                let _actual: &f64 = owners
                    .arrays
                    .surf_opaq_avg_face_cond_energy
                    .as_ref()
                    .and_then(|values| values.get(i))
                    .ok_or(Unavailable::StorageDomain)?;
                registration.setup_unmetered_zone_real(
                    input,
                    prescan,
                    SourceRealOutputDeclaration {
                        name: "Surface Average Face Conduction Heat Transfer Energy",
                        key: &surface.row.name,
                        units: SourceOutputUnits::J,
                        store_type: SourceOutputStoreType::Sum,
                    },
                    minutes_in_time_step,
                    host,
                    eio,
                )?;
                let binding = SourceRealOutputBinding::new(
                    SourceRealOutputTarget::SurfaceHeatBalance {
                        field: SourceSurfaceOutputField::SurfOpaqStorageCond,
                        index: number,
                    },
                    owners,
                )?;
                registration.setup_bound_unmetered_zone_real(
                    input,
                    prescan,
                    SourceRealOutputDeclaration {
                        name: "Surface Heat Storage Rate",
                        key: &surface.row.name,
                        units: SourceOutputUnits::W,
                        store_type: SourceOutputStoreType::Average,
                    },
                    minutes_in_time_step,
                    host,
                    eio,
                    binding,
                    dictionary_host,
                )?;
                let _actual: &f64 = owners
                    .arrays
                    .surf_opaq_storage_cond_gain_rep
                    .as_ref()
                    .and_then(|values| values.get(i))
                    .ok_or(Unavailable::StorageDomain)?;
                registration.setup_unmetered_zone_real(
                    input,
                    prescan,
                    SourceRealOutputDeclaration {
                        name: "Surface Heat Storage Gain Rate",
                        key: &surface.row.name,
                        units: SourceOutputUnits::W,
                        store_type: SourceOutputStoreType::Average,
                    },
                    minutes_in_time_step,
                    host,
                    eio,
                )?;
                let _actual: &f64 = owners
                    .arrays
                    .surf_opaq_storage_cond_loss_rep
                    .as_ref()
                    .and_then(|values| values.get(i))
                    .ok_or(Unavailable::StorageDomain)?;
                registration.setup_unmetered_zone_real(
                    input,
                    prescan,
                    SourceRealOutputDeclaration {
                        name: "Surface Heat Storage Loss Rate",
                        key: &surface.row.name,
                        units: SourceOutputUnits::W,
                        store_type: SourceOutputStoreType::Average,
                    },
                    minutes_in_time_step,
                    host,
                    eio,
                )?;
                let binding = SourceRealOutputBinding::new(
                    SourceRealOutputTarget::SurfaceHeatBalance {
                        field: SourceSurfaceOutputField::SurfOpaqStorageCondFlux,
                        index: number,
                    },
                    owners,
                )?;
                registration.setup_bound_unmetered_zone_real(
                    input,
                    prescan,
                    SourceRealOutputDeclaration {
                        name: "Surface Heat Storage Rate per Area",
                        key: &surface.row.name,
                        units: SourceOutputUnits::WPerM2,
                        store_type: SourceOutputStoreType::Average,
                    },
                    minutes_in_time_step,
                    host,
                    eio,
                    binding,
                    dictionary_host,
                )?;
                let _actual: &f64 = owners
                    .arrays
                    .surf_opaq_storage_cond_energy
                    .as_ref()
                    .and_then(|values| values.get(i))
                    .ok_or(Unavailable::StorageDomain)?;
                registration.setup_unmetered_zone_real(
                    input,
                    prescan,
                    SourceRealOutputDeclaration {
                        name: "Surface Heat Storage Energy",
                        key: &surface.row.name,
                        units: SourceOutputUnits::J,
                        store_type: SourceOutputStoreType::Sum,
                    },
                    minutes_in_time_step,
                    host,
                    eio,
                )?;
            }
            let _actual: &f64 = owners
                .arrays
                .surf_opaq_ins_face_beam_sol_absorbed
                .as_ref()
                .and_then(|values| values.get(i))
                .ok_or(Unavailable::StorageDomain)?;
            registration.setup_unmetered_zone_real(
                input,
                prescan,
                SourceRealOutputDeclaration {
                    name: "Surface Inside Face Beam Solar Radiation Heat Gain Rate",
                    key: &surface.row.name,
                    units: SourceOutputUnits::W,
                    store_type: SourceOutputStoreType::Average,
                },
                minutes_in_time_step,
                host,
                eio,
            )?;
        }
        if constructions
            .construction_usage
            .get(index(surface.row.construction)?)
            .ok_or(Unavailable::StorageDomain)?
            .source_sink_present
        {
            return Err(Unavailable::InputBranch.into()); // first reached positive source-location registration
        }
        if surface.row.class == SurfaceClass::Window {
            return Err(Unavailable::InputBranch.into()); // first reached positive Window reporting branch
        }
        let _actual: &i32 = &(parallel
            .interior_convection
            .get(i)
            .ok_or(Unavailable::StorageDomain)?
            .class_report);
        registration.setup_unmetered_zone_integer(
            input,
            prescan,
            SourceIntegerOutputDeclaration {
                name: "Surface Inside Face Convection Classification Index",
                key: &surface.row.name,
                units: SourceOutputUnits::Dimensionless,
                store_type: SourceOutputStoreType::Average,
            },
            minutes_in_time_step,
            host,
            eio,
        )?;
        let _actual: &i32 = &(parallel
            .interior_convection
            .get(i)
            .ok_or(Unavailable::StorageDomain)?
            .model_equation_report);
        registration.setup_unmetered_zone_integer(
            input,
            prescan,
            SourceIntegerOutputDeclaration {
                name: "Surface Inside Face Convection Model Equation Index",
                key: &surface.row.name,
                units: SourceOutputUnits::Dimensionless,
                store_type: SourceOutputStoreType::Average,
            },
            minutes_in_time_step,
            host,
            eio,
        )?;
        let _actual: &i32 = &(*parallel
            .surf_t_air_ref_rpt
            .get(i)
            .ok_or(Unavailable::StorageDomain)?);
        registration.setup_unmetered_zone_integer(
            input,
            prescan,
            SourceIntegerOutputDeclaration {
                name: "Surface Inside Face Convection Reference Air Index",
                key: &surface.row.name,
                units: SourceOutputUnits::Dimensionless,
                store_type: SourceOutputStoreType::Average,
            },
            minutes_in_time_step,
            host,
            eio,
        )?;
        if surface.row.exterior_boundary == 0 {
            let _actual: &i32 = &(parallel
                .exterior_convection
                .get(i)
                .ok_or(Unavailable::StorageDomain)?
                .class_report);
            registration.setup_unmetered_zone_integer(
                input,
                prescan,
                SourceIntegerOutputDeclaration {
                    name: "Surface Outside Face Convection Classification Index",
                    key: &surface.row.name,
                    units: SourceOutputUnits::Dimensionless,
                    store_type: SourceOutputStoreType::Average,
                },
                minutes_in_time_step,
                host,
                eio,
            )?;
            let _actual: &i32 = &(parallel
                .exterior_convection
                .get(i)
                .ok_or(Unavailable::StorageDomain)?
                .forced_model_report);
            registration.setup_unmetered_zone_integer(
                input,
                prescan,
                SourceIntegerOutputDeclaration {
                    name: "Surface Outside Face Forced Convection Model Equation Index",
                    key: &surface.row.name,
                    units: SourceOutputUnits::Dimensionless,
                    store_type: SourceOutputStoreType::Average,
                },
                minutes_in_time_step,
                host,
                eio,
            )?;
            let _actual: &i32 = &(parallel
                .exterior_convection
                .get(i)
                .ok_or(Unavailable::StorageDomain)?
                .natural_model_report);
            registration.setup_unmetered_zone_integer(
                input,
                prescan,
                SourceIntegerOutputDeclaration {
                    name: "Surface Outside Face Natural Convection Model Equation Index",
                    key: &surface.row.name,
                    units: SourceOutputUnits::Dimensionless,
                    store_type: SourceOutputStoreType::Average,
                },
                minutes_in_time_step,
                host,
                eio,
            )?;
        }
        let _actual: &f64 = owners
            .arrays
            .surf_q_additional_heat_source_inside
            .as_ref()
            .and_then(|values| values.get(i))
            .ok_or(Unavailable::StorageDomain)?;
        registration.setup_unmetered_zone_real(
            input,
            prescan,
            SourceRealOutputDeclaration {
                name: "Surface Inside Face Heat Source Gain Rate per Area",
                key: &surface.row.name,
                units: SourceOutputUnits::WPerM2,
                store_type: SourceOutputStoreType::Average,
            },
            minutes_in_time_step,
            host,
            eio,
        )?;
        let _actual: &f64 = owners
            .arrays
            .surf_q_additional_heat_source_outside
            .as_ref()
            .and_then(|values| values.get(i))
            .ok_or(Unavailable::StorageDomain)?;
        registration.setup_unmetered_zone_real(
            input,
            prescan,
            SourceRealOutputDeclaration {
                name: "Surface Outside Face Heat Source Gain Rate per Area",
                key: &surface.row.name,
                units: SourceOutputUnits::WPerM2,
                store_type: SourceOutputStoreType::Average,
            },
            minutes_in_time_step,
            host,
            eio,
        )?;
        if advanced_report_variables {
            let _actual: &i32 = &(surface.row.construction);
            registration.setup_unmetered_zone_integer(
                input,
                prescan,
                SourceIntegerOutputDeclaration {
                    name: "Surface Construction Index",
                    key: &surface.row.name,
                    units: SourceOutputUnits::Dimensionless,
                    store_type: SourceOutputStoreType::Average,
                },
                minutes_in_time_step,
                host,
                eio,
            )?;
        }
    }
    let _actual = owners.sum_surface_heat_emission;
    registration.setup_unmetered_zone_real(
        input,
        prescan,
        SourceRealOutputDeclaration {
            name: "Site Total Surface Heat Emission to Air",
            key: "Environment",
            units: SourceOutputUnits::J,
            store_type: SourceOutputStoreType::Sum,
        },
        minutes_in_time_step,
        host,
        eio,
    )?;
    Ok(())
}
