//! Actual AllocateModuleArrays1090-2600 reached opaque registration order.
use super::*;
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::{
    surface::SurfaceClass, zone::SelectedZoneInputOwners,
};

const ENCLOSURE: [Declaration; 10] = [
    Declaration {
        name: "Enclosure Windows Total Transmitted Solar Radiation Rate",
        units: Units::W,
        store: Store::Average,
        target: Target::Real(Field::ZoneTransSolar),
    },
    Declaration {
        name: "Enclosure Exterior Windows Total Transmitted Beam Solar Radiation Rate",
        units: Units::W,
        store: Store::Average,
        target: Target::Real(Field::ZoneBmSolFrExtWinsRep),
    },
    Declaration {
        name: "Enclosure Interior Windows Total Transmitted Beam Solar Radiation Rate",
        units: Units::W,
        store: Store::Average,
        target: Target::Real(Field::ZoneBmSolFrIntWinsRep),
    },
    Declaration {
        name: "Enclosure Exterior Windows Total Transmitted Diffuse Solar Radiation Rate",
        units: Units::W,
        store: Store::Average,
        target: Target::Real(Field::ZoneDifSolFrExtWinsRep),
    },
    Declaration {
        name: "Enclosure Interior Windows Total Transmitted Diffuse Solar Radiation Rate",
        units: Units::W,
        store: Store::Average,
        target: Target::Real(Field::ZoneDifSolFrIntWinsRep),
    },
    Declaration {
        name: "Enclosure Windows Total Transmitted Solar Radiation Energy",
        units: Units::J,
        store: Store::Sum,
        target: Target::Real(Field::ZoneTransSolarEnergy),
    },
    Declaration {
        name: "Enclosure Exterior Windows Total Transmitted Beam Solar Radiation Energy",
        units: Units::J,
        store: Store::Sum,
        target: Target::Real(Field::ZoneBmSolFrExtWinsRepEnergy),
    },
    Declaration {
        name: "Enclosure Interior Windows Total Transmitted Beam Solar Radiation Energy",
        units: Units::J,
        store: Store::Sum,
        target: Target::Real(Field::ZoneBmSolFrIntWinsRepEnergy),
    },
    Declaration {
        name: "Enclosure Exterior Windows Total Transmitted Diffuse Solar Radiation Energy",
        units: Units::J,
        store: Store::Sum,
        target: Target::Real(Field::ZoneDifSolFrExtWinsRepEnergy),
    },
    Declaration {
        name: "Enclosure Interior Windows Total Transmitted Diffuse Solar Radiation Energy",
        units: Units::J,
        store: Store::Sum,
        target: Target::Real(Field::ZoneDifSolFrIntWinsRepEnergy),
    },
];

const ZONE: [Declaration; 4] = [
    Declaration {
        name: "Zone Windows Total Heat Gain Rate",
        units: Units::W,
        store: Store::Average,
        target: Target::Real(Field::ZoneWinHeatGainRep),
    },
    Declaration {
        name: "Zone Windows Total Heat Loss Rate",
        units: Units::W,
        store: Store::Average,
        target: Target::Real(Field::ZoneWinHeatLossRep),
    },
    Declaration {
        name: "Zone Windows Total Heat Gain Energy",
        units: Units::J,
        store: Store::Sum,
        target: Target::Real(Field::ZoneWinHeatGainRepEnergy),
    },
    Declaration {
        name: "Zone Windows Total Heat Loss Energy",
        units: Units::J,
        store: Store::Sum,
        target: Target::Real(Field::ZoneWinHeatLossRepEnergy),
    },
];

const ZONE_ADVANCED: [Declaration; 10] = [
    Declaration {
        name: "Zone Opaque Surface Inside Faces Conduction Rate",
        units: Units::W,
        store: Store::Average,
        target: Target::Real(Field::ZoneOpaqSurfInsFaceCond),
    },
    Declaration {
        name: "Zone Opaque Surface Inside Faces Conduction Heat Gain Rate",
        units: Units::W,
        store: Store::Average,
        target: Target::Real(Field::ZoneOpaqSurfInsFaceCondGainRep),
    },
    Declaration {
        name: "Zone Opaque Surface Inside Faces Conduction Heat Loss Rate",
        units: Units::W,
        store: Store::Average,
        target: Target::Real(Field::ZoneOpaqSurfInsFaceCondLossRep),
    },
    Declaration {
        name: "Zone Opaque Surface Outside Faces Conduction Rate",
        units: Units::W,
        store: Store::Average,
        target: Target::Real(Field::ZoneOpaqSurfExtFaceCond),
    },
    Declaration {
        name: "Zone Opaque Surface Outside Faces Conduction Heat Gain Rate",
        units: Units::W,
        store: Store::Average,
        target: Target::Real(Field::ZoneOpaqSurfExtFaceCondGainRep),
    },
    Declaration {
        name: "Zone Opaque Surface Outside Faces Conduction Heat Loss Rate",
        units: Units::W,
        store: Store::Average,
        target: Target::Real(Field::ZoneOpaqSurfExtFaceCondLossRep),
    },
    Declaration {
        name: "Zone Opaque Surface Inside Faces Conduction Heat Gain Energy",
        units: Units::J,
        store: Store::Sum,
        target: Target::Real(Field::ZnOpqSurfInsFaceCondGnRepEnrg),
    },
    Declaration {
        name: "Zone Opaque Surface Inside Faces Conduction Heat Loss Energy",
        units: Units::J,
        store: Store::Sum,
        target: Target::Real(Field::ZnOpqSurfInsFaceCondLsRepEnrg),
    },
    Declaration {
        name: "Zone Opaque Surface Outside Faces Conduction Heat Gain Energy",
        units: Units::J,
        store: Store::Sum,
        target: Target::Real(Field::ZnOpqSurfExtFaceCondGnRepEnrg),
    },
    Declaration {
        name: "Zone Opaque Surface Outside Faces Conduction Heat Loss Energy",
        units: Units::J,
        store: Store::Sum,
        target: Target::Real(Field::ZnOpqSurfExtFaceCondLsRepEnrg),
    },
];

const SURFACE: [Declaration; 1] = [Declaration {
    name: "Surface Outside Normal Azimuth Angle",
    units: Units::Deg,
    store: Store::Average,
    target: Target::Real(Field::SurfaceAzimuth),
}];

const EXTERIOR_SOLAR: [Declaration; 16] = [
    Declaration {
        name: "Surface Outside Face Sunlit Area",
        units: Units::M2,
        store: Store::Average,
        target: Target::Real(Field::SurfaceSunlitArea),
    },
    Declaration {
        name: "Surface Outside Face Sunlit Fraction",
        units: Units::Dimensionless,
        store: Store::Average,
        target: Target::Real(Field::SurfaceSunlitFrac),
    },
    Declaration {
        name: "Surface Outside Face Incident Solar Radiation Rate per Area",
        units: Units::WPerM2,
        store: Store::Average,
        target: Target::Real(Field::SurfQRadSWOutIncident),
    },
    Declaration {
        name: "Surface Outside Face Incident Beam Solar Radiation Rate per Area",
        units: Units::WPerM2,
        store: Store::Average,
        target: Target::Real(Field::SurfQRadSWOutIncidentBeam),
    },
    Declaration {
        name: "Surface Outside Face Incident Sky Diffuse Solar Radiation Rate per Area",
        units: Units::WPerM2,
        store: Store::Average,
        target: Target::Real(Field::SurfQRadSWOutIncidentSkyDiffuse),
    },
    Declaration {
        name: "Surface Outside Face Incident Ground Diffuse Solar Radiation Rate per Area",
        units: Units::WPerM2,
        store: Store::Average,
        target: Target::Real(Field::SurfQRadSWOutIncidentGndDiffuse),
    },
    Declaration {
        name: "Surface Outside Face Beam Solar Incident Angle Cosine Value",
        units: Units::Dimensionless,
        store: Store::Average,
        target: Target::Real(Field::SurfCosIncidenceAngle),
    },
    Declaration {
        name: "Surface Outside Face Incident Sky Diffuse Ground Reflected Solar Radiation Rate per Area",
        units: Units::WPerM2,
        store: Store::Average,
        target: Target::Real(Field::SurfQRadSWOutIncSkyDiffReflGnd),
    },
    Declaration {
        name: "Surface Outside Face Incident Sky Diffuse Surface Reflected Solar Radiation Rate per Area",
        units: Units::WPerM2,
        store: Store::Average,
        target: Target::Real(Field::SurfQRadSWOutIncSkyDiffReflObs),
    },
    Declaration {
        name: "Surface Outside Face Incident Beam To Beam Surface Reflected Solar Radiation Rate per Area",
        units: Units::WPerM2,
        store: Store::Average,
        target: Target::Real(Field::SurfQRadSWOutIncBmToBmReflObs),
    },
    Declaration {
        name: "Surface Outside Face Incident Beam To Diffuse Surface Reflected Solar Radiation Rate per Area",
        units: Units::WPerM2,
        store: Store::Average,
        target: Target::Real(Field::SurfQRadSWOutIncBmToDiffReflObs),
    },
    Declaration {
        name: "Surface Outside Face Incident Beam To Diffuse Ground Reflected Solar Radiation Rate per Area",
        units: Units::WPerM2,
        store: Store::Average,
        target: Target::Real(Field::SurfQRadSWOutIncBmToDiffReflGnd),
    },
    Declaration {
        name: "Surface Anisotropic Sky Multiplier",
        units: Units::Dimensionless,
        store: Store::Average,
        target: Target::Real(Field::SurfAnisoSkyMult),
    },
    Declaration {
        name: "Surface Window BSDF Beam Direction Number",
        units: Units::Dimensionless,
        store: Store::Average,
        target: Target::IntegerBsdfDirection,
    },
    Declaration {
        name: "Surface Window BSDF Beam Theta Angle",
        units: Units::Rad,
        store: Store::Average,
        target: Target::Real(Field::SurfWinBSDFBeamThetaRep),
    },
    Declaration {
        name: "Surface Window BSDF Beam Phi Angle",
        units: Units::Rad,
        store: Store::Average,
        target: Target::Real(Field::SurfWinBSDFBeamPhiRep),
    },
];

const HEAT_TRANSFER: [Declaration; 9] = [
    Declaration {
        name: "Surface Inside Face Exterior Windows Incident Beam Solar Radiation Rate per Area",
        units: Units::WPerM2,
        store: Store::Average,
        target: Target::Real(Field::SurfBmIncInsSurfIntensRep),
    },
    Declaration {
        name: "Surface Inside Face Exterior Windows Incident Beam Solar Radiation Rate",
        units: Units::W,
        store: Store::Average,
        target: Target::Real(Field::SurfBmIncInsSurfAmountRep),
    },
    Declaration {
        name: "Surface Inside Face Interior Windows Incident Beam Solar Radiation Rate per Area",
        units: Units::WPerM2,
        store: Store::Average,
        target: Target::Real(Field::SurfIntBmIncInsSurfIntensRep),
    },
    Declaration {
        name: "Surface Inside Face Interior Windows Incident Beam Solar Radiation Rate",
        units: Units::W,
        store: Store::Average,
        target: Target::Real(Field::SurfIntBmIncInsSurfAmountRep),
    },
    Declaration {
        name: "Surface Inside Face Initial Transmitted Diffuse Absorbed Solar Radiation Rate",
        units: Units::W,
        store: Store::Average,
        target: Target::Real(Field::SurfInitialDifSolInAbsReport),
    },
    Declaration {
        name: "Surface Inside Face Initial Transmitted Diffuse Transmitted Out Window Solar Radiation Rate",
        units: Units::W,
        store: Store::Average,
        target: Target::Real(Field::SurfWinInitialDifSolInTransReport),
    },
    Declaration {
        name: "Surface Inside Face Absorbed Shortwave Radiation Rate",
        units: Units::W,
        store: Store::Average,
        target: Target::Real(Field::SurfSWInAbsTotalReport),
    },
    Declaration {
        name: "Surface Inside Face Exterior Windows Incident Beam Solar Radiation Energy",
        units: Units::J,
        store: Store::Sum,
        target: Target::Real(Field::SurfBmIncInsSurfAmountRepEnergy),
    },
    Declaration {
        name: "Surface Inside Face Interior Windows Incident Beam Solar Radiation Energy",
        units: Units::J,
        store: Store::Sum,
        target: Target::Real(Field::SurfIntBmIncInsSurfAmountRepEnergy),
    },
];

pub(crate) fn register_source_solar_module_outputs<
    H: SourceOutputFileHost,
    D: SourceOutputDictionaryHost,
>(
    owners: &SourceSolarOutputOwners<'_>,
    zones: &SelectedZoneInputOwners,
    globals: &SelectedHeatBalanceGlobals,
    context: &mut SourceSolarOutputRegistrationContext<'_, '_, H, D>,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), Error> {
    backend.display_string("Initializing Zone and Enclosure Report Variables")?;
    for number in 1..=owners.radiation.num_solar_enclosures() {
        let i = index(number)?;
        let key = &owners
            .radiation
            .enclosure_storage()
            .and_then(|rows| rows.get(i))
            .ok_or(Error::Unavailable("same actual Solar enclosure Name owner"))?
            .name;
        for declaration in ENCLOSURE {
            context.declare(owners, number, key, declaration)?;
        }
    }
    for number in 1..=globals.num_of_zones {
        let key = &zones
            .zones
            .as_deref()
            .and_then(|rows| rows.get(index(number).ok()?))
            .ok_or(Error::Unavailable("same actual Solar Zone Name owner"))?
            .name;
        for declaration in ZONE {
            context.declare(owners, number, key, declaration)?;
        }
        if globals.display_advanced_report_variables {
            for declaration in ZONE_ADVANCED {
                context.declare(owners, number, key, declaration)?;
            }
        }
    }
    backend.display_string("Initializing Surface (Shading) Report Variables")?;
    for number in 1..=owners.geometry.total_surfaces {
        let surface = owners
            .geometry
            .final_surfaces
            .as_deref()
            .and_then(|rows| rows.get(index(number).ok()?))
            .ok_or(Error::Unavailable("same actual Solar Surface owner"))?;
        let key = &surface.row.name;
        for declaration in SURFACE {
            context.declare(owners, number, key, declaration)?;
        }
        if surface.row.exterior_solar {
            for declaration in EXTERIOR_SOLAR {
                context.declare(owners, number, key, declaration)?;
            }
        }
        if !surface.row.heat_transfer_surface {
            continue;
        }
        if surface.row.class == SurfaceClass::Window {
            return Err(Error::Unavailable(
                "positive Source Solar window reporting1399",
            ));
        }
        if surface.row.class == SurfaceClass::TddDome && surface.row.exterior_solar {
            return Err(Error::Unavailable(
                "positive Source Solar TDD dome reporting2454",
            ));
        }
        if surface.original_class == SurfaceClass::TddDiffuser {
            return Err(Error::Unavailable(
                "positive Source Solar TDD diffuser reporting2486",
            ));
        }
    }
    for number in 1..=owners.geometry.total_surfaces {
        let surface = owners
            .geometry
            .final_surfaces
            .as_deref()
            .and_then(|rows| rows.get(index(number).ok()?))
            .ok_or(Error::Unavailable(
                "same actual final Solar HT Surface owner",
            ))?;
        if !surface.row.heat_transfer_surface {
            continue;
        }
        for declaration in HEAT_TRANSFER {
            context.declare(owners, number, &surface.row.name, declaration)?;
        }
    }
    Ok(())
}
