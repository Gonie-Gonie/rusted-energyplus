//! GetPurchasedAir525-1015: all declarations on the same retained equipment owner.
use super::owners::SourcePurchasedAirFuel;
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::ordinary_equipment::OrdinaryEquipment;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::source_output_registration::{
    SourceGeneralMeterArguments, SourceGeneralRealRegistrationOptions, SourceIntegerOutputBinding,
    SourceIntegerOutputDeclaration, SourceIntegerOutputTarget, SourceMeterEndUse, SourceMeterGroup,
    SourceMeterResource, SourceOutputDictionaryHost, SourceOutputFileHost,
    SourceOutputStoreType as Store, SourceOutputUnits as Units,
    SourcePurchasedAirOutputField as Field, SourcePurchasedAirReportingOutputContext,
    SourceRealOutputBinding, SourceRealOutputDeclaration, SourceRealOutputTarget,
    SourceReportVariableInput, TimeStepKind,
};

fn unit(
    equipment: &OrdinaryEquipment,
    number: i32,
) -> Result<&super::owners::SourceZonePurchasedAir, Error> {
    let index = number
        .checked_sub(1)
        .and_then(|n| usize::try_from(n).ok())
        .ok_or(Error::Unavailable("actual PurchasedAir report ordinal"))?;
    equipment
        .purchased_air
        .units
        .as_ref()
        .and_then(|rows| rows.get(index))
        .ok_or(Error::Unavailable("actual PurchasedAir report member"))
}

// Exact eFuel2eResource entries consumed by finite B; no enum-order cast.
fn fuel_resource(fuel: SourcePurchasedAirFuel) -> Result<SourceMeterResource, Error> {
    match fuel {
        SourcePurchasedAirFuel::Electricity => Ok(SourceMeterResource::Electricity),
        SourcePurchasedAirFuel::DistrictCooling => Ok(SourceMeterResource::DistrictCooling),
        SourcePurchasedAirFuel::DistrictHeatingWater => {
            Ok(SourceMeterResource::DistrictHeatingWater)
        }
        _ => Err(Error::Unavailable(
            "actual PurchasedAir other-fuel meter resource",
        )),
    }
}

fn register_real<H: SourceOutputFileHost, D: SourceOutputDictionaryHost>(
    equipment: &OrdinaryEquipment,
    number: i32,
    field: Field,
    name: &str,
    units: Units,
    store_type: Store,
    meter: Option<SourceGeneralMeterArguments<'_>>,
    backend: &mut SourceErrorBackend<'_>,
    output: &mut SourcePurchasedAirReportingOutputContext<'_, H, D>,
) -> Result<(), Error> {
    let row = unit(equipment, number)?;
    let binding = SourceRealOutputBinding::new(
        SourceRealOutputTarget::PurchasedAir {
            field,
            index: number,
        },
        equipment,
    )
    .map_err(Error::Output)?;
    let mut requests = SourceReportVariableInput::new(
        output.reports,
        &mut *output.input,
        output.minimum_report_frequency,
    );
    output
        .registration
        .setup_bound_general_real(
            &mut requests,
            output.prescan,
            SourceRealOutputDeclaration {
                name,
                key: &row.name,
                units,
                store_type,
            },
            output.minutes_in_time_step,
            &mut *output.host,
            &mut *output.eio,
            binding,
            &mut *output.dictionary_host,
            SourceGeneralRealRegistrationOptions {
                time_step: TimeStepKind::System,
                meter,
            },
            backend,
        )
        .map_err(Error::Output)
}

pub(super) fn register_unit<H: SourceOutputFileHost, D: SourceOutputDictionaryHost>(
    equipment: &OrdinaryEquipment,
    unit_number: i32,
    backend: &mut SourceErrorBackend<'_>,
    output: &mut SourcePurchasedAirReportingOutputContext<'_, H, D>,
) -> Result<(), Error> {
    // Native declaration 1: PurchAir.SenHeatEnergy.
    register_real(
        equipment,
        unit_number,
        Field::SenHeatEnergy,
        "Zone Ideal Loads Supply Air Sensible Heating Energy",
        Units::J,
        Store::Sum,
        None,
        backend,
        output,
    )?;
    // Native declaration 2: PurchAir.LatHeatEnergy.
    register_real(
        equipment,
        unit_number,
        Field::LatHeatEnergy,
        "Zone Ideal Loads Supply Air Latent Heating Energy",
        Units::J,
        Store::Sum,
        None,
        backend,
        output,
    )?;
    // Native declaration 3: PurchAir.TotHeatEnergy.
    register_real(
        equipment,
        unit_number,
        Field::TotHeatEnergy,
        "Zone Ideal Loads Supply Air Total Heating Energy",
        Units::J,
        Store::Sum,
        None,
        backend,
        output,
    )?;
    // Native declaration 4: PurchAir.SenCoolEnergy.
    register_real(
        equipment,
        unit_number,
        Field::SenCoolEnergy,
        "Zone Ideal Loads Supply Air Sensible Cooling Energy",
        Units::J,
        Store::Sum,
        None,
        backend,
        output,
    )?;
    // Native declaration 5: PurchAir.LatCoolEnergy.
    register_real(
        equipment,
        unit_number,
        Field::LatCoolEnergy,
        "Zone Ideal Loads Supply Air Latent Cooling Energy",
        Units::J,
        Store::Sum,
        None,
        backend,
        output,
    )?;
    // Native declaration 6: PurchAir.TotCoolEnergy.
    register_real(
        equipment,
        unit_number,
        Field::TotCoolEnergy,
        "Zone Ideal Loads Supply Air Total Cooling Energy",
        Units::J,
        Store::Sum,
        None,
        backend,
        output,
    )?;
    // Native declaration 7: PurchAir.TotHeatFuelEnergy.
    register_real(
        equipment,
        unit_number,
        Field::TotHeatFuelEnergy,
        "Zone Ideal Loads Supply Air Total Heating Fuel Energy",
        Units::J,
        Store::Sum,
        Some(SourceGeneralMeterArguments {
            resource: fuel_resource(unit(equipment, unit_number)?.heating_fuel_type)?,
            end_use: SourceMeterEndUse::Heating,
            end_use_subcategory: "",
            group: SourceMeterGroup::Hvac,
            zone_name: "",
            space_type: "",
            zone_multiplier: 1,
            zone_list_multiplier: 1,
        }),
        backend,
        output,
    )?;
    // Native declaration 8: PurchAir.TotCoolFuelEnergy.
    register_real(
        equipment,
        unit_number,
        Field::TotCoolFuelEnergy,
        "Zone Ideal Loads Supply Air Total Cooling Fuel Energy",
        Units::J,
        Store::Sum,
        Some(SourceGeneralMeterArguments {
            resource: fuel_resource(unit(equipment, unit_number)?.cooling_fuel_type)?,
            end_use: SourceMeterEndUse::Cooling,
            end_use_subcategory: "",
            group: SourceMeterGroup::Hvac,
            zone_name: "",
            space_type: "",
            zone_multiplier: 1,
            zone_list_multiplier: 1,
        }),
        backend,
        output,
    )?;
    // Native declaration 9: PurchAir.ZoneSenHeatEnergy.
    register_real(
        equipment,
        unit_number,
        Field::ZoneSenHeatEnergy,
        "Zone Ideal Loads Zone Sensible Heating Energy",
        Units::J,
        Store::Sum,
        None,
        backend,
        output,
    )?;
    // Native declaration 10: PurchAir.ZoneLatHeatEnergy.
    register_real(
        equipment,
        unit_number,
        Field::ZoneLatHeatEnergy,
        "Zone Ideal Loads Zone Latent Heating Energy",
        Units::J,
        Store::Sum,
        None,
        backend,
        output,
    )?;
    // Native declaration 11: PurchAir.ZoneTotHeatEnergy.
    register_real(
        equipment,
        unit_number,
        Field::ZoneTotHeatEnergy,
        "Zone Ideal Loads Zone Total Heating Energy",
        Units::J,
        Store::Sum,
        None,
        backend,
        output,
    )?;
    // Native declaration 12: PurchAir.ZoneSenCoolEnergy.
    register_real(
        equipment,
        unit_number,
        Field::ZoneSenCoolEnergy,
        "Zone Ideal Loads Zone Sensible Cooling Energy",
        Units::J,
        Store::Sum,
        None,
        backend,
        output,
    )?;
    // Native declaration 13: PurchAir.ZoneLatCoolEnergy.
    register_real(
        equipment,
        unit_number,
        Field::ZoneLatCoolEnergy,
        "Zone Ideal Loads Zone Latent Cooling Energy",
        Units::J,
        Store::Sum,
        None,
        backend,
        output,
    )?;
    // Native declaration 14: PurchAir.ZoneTotCoolEnergy.
    register_real(
        equipment,
        unit_number,
        Field::ZoneTotCoolEnergy,
        "Zone Ideal Loads Zone Total Cooling Energy",
        Units::J,
        Store::Sum,
        None,
        backend,
        output,
    )?;
    // Native declaration 15: PurchAir.OASenHeatEnergy.
    register_real(
        equipment,
        unit_number,
        Field::OASenHeatEnergy,
        "Zone Ideal Loads Outdoor Air Sensible Heating Energy",
        Units::J,
        Store::Sum,
        None,
        backend,
        output,
    )?;
    // Native declaration 16: PurchAir.OALatHeatEnergy.
    register_real(
        equipment,
        unit_number,
        Field::OALatHeatEnergy,
        "Zone Ideal Loads Outdoor Air Latent Heating Energy",
        Units::J,
        Store::Sum,
        None,
        backend,
        output,
    )?;
    // Native declaration 17: PurchAir.OATotHeatEnergy.
    register_real(
        equipment,
        unit_number,
        Field::OATotHeatEnergy,
        "Zone Ideal Loads Outdoor Air Total Heating Energy",
        Units::J,
        Store::Sum,
        None,
        backend,
        output,
    )?;
    // Native declaration 18: PurchAir.OASenCoolEnergy.
    register_real(
        equipment,
        unit_number,
        Field::OASenCoolEnergy,
        "Zone Ideal Loads Outdoor Air Sensible Cooling Energy",
        Units::J,
        Store::Sum,
        None,
        backend,
        output,
    )?;
    // Native declaration 19: PurchAir.OALatCoolEnergy.
    register_real(
        equipment,
        unit_number,
        Field::OALatCoolEnergy,
        "Zone Ideal Loads Outdoor Air Latent Cooling Energy",
        Units::J,
        Store::Sum,
        None,
        backend,
        output,
    )?;
    // Native declaration 20: PurchAir.OATotCoolEnergy.
    register_real(
        equipment,
        unit_number,
        Field::OATotCoolEnergy,
        "Zone Ideal Loads Outdoor Air Total Cooling Energy",
        Units::J,
        Store::Sum,
        None,
        backend,
        output,
    )?;
    // Native declaration 21: PurchAir.HtRecSenHeatEnergy.
    register_real(
        equipment,
        unit_number,
        Field::HtRecSenHeatEnergy,
        "Zone Ideal Loads Heat Recovery Sensible Heating Energy",
        Units::J,
        Store::Sum,
        None,
        backend,
        output,
    )?;
    // Native declaration 22: PurchAir.HtRecLatHeatEnergy.
    register_real(
        equipment,
        unit_number,
        Field::HtRecLatHeatEnergy,
        "Zone Ideal Loads Heat Recovery Latent Heating Energy",
        Units::J,
        Store::Sum,
        None,
        backend,
        output,
    )?;
    // Native declaration 23: PurchAir.HtRecTotHeatEnergy.
    register_real(
        equipment,
        unit_number,
        Field::HtRecTotHeatEnergy,
        "Zone Ideal Loads Heat Recovery Total Heating Energy",
        Units::J,
        Store::Sum,
        None,
        backend,
        output,
    )?;
    // Native declaration 24: PurchAir.HtRecSenCoolEnergy.
    register_real(
        equipment,
        unit_number,
        Field::HtRecSenCoolEnergy,
        "Zone Ideal Loads Heat Recovery Sensible Cooling Energy",
        Units::J,
        Store::Sum,
        None,
        backend,
        output,
    )?;
    // Native declaration 25: PurchAir.HtRecLatCoolEnergy.
    register_real(
        equipment,
        unit_number,
        Field::HtRecLatCoolEnergy,
        "Zone Ideal Loads Heat Recovery Latent Cooling Energy",
        Units::J,
        Store::Sum,
        None,
        backend,
        output,
    )?;
    // Native declaration 26: PurchAir.HtRecTotCoolEnergy.
    register_real(
        equipment,
        unit_number,
        Field::HtRecTotCoolEnergy,
        "Zone Ideal Loads Heat Recovery Total Cooling Energy",
        Units::J,
        Store::Sum,
        None,
        backend,
        output,
    )?;
    // Native declaration 27: PurchAir.ZoneTotHeatFuelEnergy.
    register_real(
        equipment,
        unit_number,
        Field::ZoneTotHeatFuelEnergy,
        "Zone Ideal Loads Zone Heating Fuel Energy",
        Units::J,
        Store::Sum,
        None,
        backend,
        output,
    )?;
    // Native declaration 28: PurchAir.ZoneTotCoolFuelEnergy.
    register_real(
        equipment,
        unit_number,
        Field::ZoneTotCoolFuelEnergy,
        "Zone Ideal Loads Zone Cooling Fuel Energy",
        Units::J,
        Store::Sum,
        None,
        backend,
        output,
    )?;
    // Native declaration 29: PurchAir.SenHeatRate.
    register_real(
        equipment,
        unit_number,
        Field::SenHeatRate,
        "Zone Ideal Loads Supply Air Sensible Heating Rate",
        Units::W,
        Store::Average,
        None,
        backend,
        output,
    )?;
    // Native declaration 30: PurchAir.LatHeatRate.
    register_real(
        equipment,
        unit_number,
        Field::LatHeatRate,
        "Zone Ideal Loads Supply Air Latent Heating Rate",
        Units::W,
        Store::Average,
        None,
        backend,
        output,
    )?;
    // Native declaration 31: PurchAir.TotHeatRate.
    register_real(
        equipment,
        unit_number,
        Field::TotHeatRate,
        "Zone Ideal Loads Supply Air Total Heating Rate",
        Units::W,
        Store::Average,
        None,
        backend,
        output,
    )?;
    // Native declaration 32: PurchAir.SenCoolRate.
    register_real(
        equipment,
        unit_number,
        Field::SenCoolRate,
        "Zone Ideal Loads Supply Air Sensible Cooling Rate",
        Units::W,
        Store::Average,
        None,
        backend,
        output,
    )?;
    // Native declaration 33: PurchAir.LatCoolRate.
    register_real(
        equipment,
        unit_number,
        Field::LatCoolRate,
        "Zone Ideal Loads Supply Air Latent Cooling Rate",
        Units::W,
        Store::Average,
        None,
        backend,
        output,
    )?;
    // Native declaration 34: PurchAir.TotCoolRate.
    register_real(
        equipment,
        unit_number,
        Field::TotCoolRate,
        "Zone Ideal Loads Supply Air Total Cooling Rate",
        Units::W,
        Store::Average,
        None,
        backend,
        output,
    )?;
    // Native declaration 35: PurchAir.TotHeatFuelRate.
    register_real(
        equipment,
        unit_number,
        Field::TotHeatFuelRate,
        "Zone Ideal Loads Supply Air Total Heating Fuel Energy Rate",
        Units::W,
        Store::Average,
        None,
        backend,
        output,
    )?;
    // Native declaration 36: PurchAir.TotCoolFuelRate.
    register_real(
        equipment,
        unit_number,
        Field::TotCoolFuelRate,
        "Zone Ideal Loads Supply Air Total Cooling Fuel Energy Rate",
        Units::W,
        Store::Average,
        None,
        backend,
        output,
    )?;
    // Native declaration 37: PurchAir.ZoneSenHeatRate.
    register_real(
        equipment,
        unit_number,
        Field::ZoneSenHeatRate,
        "Zone Ideal Loads Zone Sensible Heating Rate",
        Units::W,
        Store::Average,
        None,
        backend,
        output,
    )?;
    // Native declaration 38: PurchAir.ZoneLatHeatRate.
    register_real(
        equipment,
        unit_number,
        Field::ZoneLatHeatRate,
        "Zone Ideal Loads Zone Latent Heating Rate",
        Units::W,
        Store::Average,
        None,
        backend,
        output,
    )?;
    // Native declaration 39: PurchAir.ZoneTotHeatRate.
    register_real(
        equipment,
        unit_number,
        Field::ZoneTotHeatRate,
        "Zone Ideal Loads Zone Total Heating Rate",
        Units::W,
        Store::Average,
        None,
        backend,
        output,
    )?;
    // Native declaration 40: PurchAir.ZoneSenCoolRate.
    register_real(
        equipment,
        unit_number,
        Field::ZoneSenCoolRate,
        "Zone Ideal Loads Zone Sensible Cooling Rate",
        Units::W,
        Store::Average,
        None,
        backend,
        output,
    )?;
    // Native declaration 41: PurchAir.ZoneLatCoolRate.
    register_real(
        equipment,
        unit_number,
        Field::ZoneLatCoolRate,
        "Zone Ideal Loads Zone Latent Cooling Rate",
        Units::W,
        Store::Average,
        None,
        backend,
        output,
    )?;
    // Native declaration 42: PurchAir.ZoneTotCoolRate.
    register_real(
        equipment,
        unit_number,
        Field::ZoneTotCoolRate,
        "Zone Ideal Loads Zone Total Cooling Rate",
        Units::W,
        Store::Average,
        None,
        backend,
        output,
    )?;
    // Native declaration 43: PurchAir.OASenHeatRate.
    register_real(
        equipment,
        unit_number,
        Field::OASenHeatRate,
        "Zone Ideal Loads Outdoor Air Sensible Heating Rate",
        Units::W,
        Store::Average,
        None,
        backend,
        output,
    )?;
    // Native declaration 44: PurchAir.OALatHeatRate.
    register_real(
        equipment,
        unit_number,
        Field::OALatHeatRate,
        "Zone Ideal Loads Outdoor Air Latent Heating Rate",
        Units::W,
        Store::Average,
        None,
        backend,
        output,
    )?;
    // Native declaration 45: PurchAir.OATotHeatRate.
    register_real(
        equipment,
        unit_number,
        Field::OATotHeatRate,
        "Zone Ideal Loads Outdoor Air Total Heating Rate",
        Units::W,
        Store::Average,
        None,
        backend,
        output,
    )?;
    // Native declaration 46: PurchAir.OASenCoolRate.
    register_real(
        equipment,
        unit_number,
        Field::OASenCoolRate,
        "Zone Ideal Loads Outdoor Air Sensible Cooling Rate",
        Units::W,
        Store::Average,
        None,
        backend,
        output,
    )?;
    // Native declaration 47: PurchAir.OALatCoolRate.
    register_real(
        equipment,
        unit_number,
        Field::OALatCoolRate,
        "Zone Ideal Loads Outdoor Air Latent Cooling Rate",
        Units::W,
        Store::Average,
        None,
        backend,
        output,
    )?;
    // Native declaration 48: PurchAir.OATotCoolRate.
    register_real(
        equipment,
        unit_number,
        Field::OATotCoolRate,
        "Zone Ideal Loads Outdoor Air Total Cooling Rate",
        Units::W,
        Store::Average,
        None,
        backend,
        output,
    )?;
    // Native declaration 49: PurchAir.HtRecSenHeatRate.
    register_real(
        equipment,
        unit_number,
        Field::HtRecSenHeatRate,
        "Zone Ideal Loads Heat Recovery Sensible Heating Rate",
        Units::W,
        Store::Average,
        None,
        backend,
        output,
    )?;
    // Native declaration 50: PurchAir.HtRecLatHeatRate.
    register_real(
        equipment,
        unit_number,
        Field::HtRecLatHeatRate,
        "Zone Ideal Loads Heat Recovery Latent Heating Rate",
        Units::W,
        Store::Average,
        None,
        backend,
        output,
    )?;
    // Native declaration 51: PurchAir.HtRecTotHeatRate.
    register_real(
        equipment,
        unit_number,
        Field::HtRecTotHeatRate,
        "Zone Ideal Loads Heat Recovery Total Heating Rate",
        Units::W,
        Store::Average,
        None,
        backend,
        output,
    )?;
    // Native declaration 52: PurchAir.HtRecSenCoolRate.
    register_real(
        equipment,
        unit_number,
        Field::HtRecSenCoolRate,
        "Zone Ideal Loads Heat Recovery Sensible Cooling Rate",
        Units::W,
        Store::Average,
        None,
        backend,
        output,
    )?;
    // Native declaration 53: PurchAir.HtRecLatCoolRate.
    register_real(
        equipment,
        unit_number,
        Field::HtRecLatCoolRate,
        "Zone Ideal Loads Heat Recovery Latent Cooling Rate",
        Units::W,
        Store::Average,
        None,
        backend,
        output,
    )?;
    // Native declaration 54: PurchAir.HtRecTotCoolRate.
    register_real(
        equipment,
        unit_number,
        Field::HtRecTotCoolRate,
        "Zone Ideal Loads Heat Recovery Total Cooling Rate",
        Units::W,
        Store::Average,
        None,
        backend,
        output,
    )?;
    // Native declaration 55: PurchAir.ZoneTotHeatFuelRate.
    register_real(
        equipment,
        unit_number,
        Field::ZoneTotHeatFuelRate,
        "Zone Ideal Loads Zone Heating Fuel Energy Rate",
        Units::W,
        Store::Average,
        None,
        backend,
        output,
    )?;
    // Native declaration 56: PurchAir.ZoneTotCoolFuelRate.
    register_real(
        equipment,
        unit_number,
        Field::ZoneTotCoolFuelRate,
        "Zone Ideal Loads Zone Cooling Fuel Energy Rate",
        Units::W,
        Store::Average,
        None,
        backend,
        output,
    )?;
    // Native declaration 57: PurchAir.TimeEconoActive.
    register_real(
        equipment,
        unit_number,
        Field::TimeEconoActive,
        "Zone Ideal Loads Economizer Active Time",
        Units::Hr,
        Store::Sum,
        None,
        backend,
        output,
    )?;
    // Native declaration 58: PurchAir.TimeHtRecActive.
    register_real(
        equipment,
        unit_number,
        Field::TimeHtRecActive,
        "Zone Ideal Loads Heat Recovery Active Time",
        Units::Hr,
        Store::Sum,
        None,
        backend,
        output,
    )?;
    // Native declaration 59: PurchAir.availStatus.
    {
        let row = unit(equipment, unit_number)?;
        let binding = SourceIntegerOutputBinding::new(
            SourceIntegerOutputTarget::PurchasedAirAvailabilityStatus { index: unit_number },
            equipment,
        )
        .map_err(Error::Output)?;
        let mut requests = SourceReportVariableInput::new(
            output.reports,
            &mut *output.input,
            output.minimum_report_frequency,
        );
        output
            .registration
            .setup_bound_integer(
                &mut requests,
                output.prescan,
                SourceIntegerOutputDeclaration {
                    name: "Zone Ideal Loads Hybrid Ventilation Available Status",
                    key: &row.name,
                    units: Units::Dimensionless,
                    store_type: Store::Average,
                },
                output.minutes_in_time_step,
                &mut *output.host,
                &mut *output.eio,
                binding,
                &mut *output.dictionary_host,
                TimeStepKind::System,
            )
            .map_err(Error::Output)?;
    }
    // Native declaration 60: PurchAir.OutdoorAirMassFlowRate.
    register_real(
        equipment,
        unit_number,
        Field::OutdoorAirMassFlowRate,
        "Zone Ideal Loads Outdoor Air Mass Flow Rate",
        Units::KgPerSecond,
        Store::Average,
        None,
        backend,
        output,
    )?;
    // Native declaration 61: PurchAir.OutdoorAirVolFlowRateStdRho.
    register_real(
        equipment,
        unit_number,
        Field::OutdoorAirVolFlowRateStdRho,
        "Zone Ideal Loads Outdoor Air Standard Density Volume Flow Rate",
        Units::M3PerSecond,
        Store::Average,
        None,
        backend,
        output,
    )?;
    // Native declaration 62: PurchAir.SupplyAirMassFlowRate.
    register_real(
        equipment,
        unit_number,
        Field::SupplyAirMassFlowRate,
        "Zone Ideal Loads Supply Air Mass Flow Rate",
        Units::KgPerSecond,
        Store::Average,
        None,
        backend,
        output,
    )?;
    // Native declaration 63: PurchAir.SupplyAirVolFlowRateStdRho.
    register_real(
        equipment,
        unit_number,
        Field::SupplyAirVolFlowRateStdRho,
        "Zone Ideal Loads Supply Air Standard Density Volume Flow Rate",
        Units::M3PerSecond,
        Store::Average,
        None,
        backend,
        output,
    )?;
    // Native declaration 64: PurchAir.SupplyTemp.
    register_real(
        equipment,
        unit_number,
        Field::SupplyTemp,
        "Zone Ideal Loads Supply Air Temperature",
        Units::C,
        Store::Average,
        None,
        backend,
        output,
    )?;
    // Native declaration 65: PurchAir.SupplyHumRat.
    register_real(
        equipment,
        unit_number,
        Field::SupplyHumRat,
        "Zone Ideal Loads Supply Air Humidity Ratio",
        Units::KgWaterPerKgDryAir,
        Store::Average,
        None,
        backend,
        output,
    )?;
    // Native declaration 66: PurchAir.MixedAirTemp.
    register_real(
        equipment,
        unit_number,
        Field::MixedAirTemp,
        "Zone Ideal Loads Mixed Air Temperature",
        Units::C,
        Store::Average,
        None,
        backend,
        output,
    )?;
    // Native declaration 67: PurchAir.MixedAirHumRat.
    register_real(
        equipment,
        unit_number,
        Field::MixedAirHumRat,
        "Zone Ideal Loads Mixed Air Humidity Ratio",
        Units::KgWaterPerKgDryAir,
        Store::Average,
        None,
        backend,
        output,
    )?;
    Ok(())
}
