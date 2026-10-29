//! Actual GetZoneData/ProcessZoneData and selected whole-Zone GetSpaceData.
//! Source arrays stay on reports until the real geometry stage succeeds.
use super::super::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use super::super::selected_input_processor::{
    SelectedInputProcessor, SourceInputProcessorShortCut,
};
use super::super::source_output_prescan::SourceOutputPreScan;
use super::super::source_output_registration::{
    SourceOutputFileHost, SourceOutputRegistration, SourceOutputStoreType, SourceOutputUnits,
    SourceRealOutputDeclaration, SourceReportVariableInput,
};
use super::{HeatBalanceInputError, SelectedHeatBalanceInput, SelectedHeatBalanceInputState};
use ep_model::{
    InsideSurfaceConvectionAlgorithm as Inside, NormalizedName,
    OutsideSurfaceConvectionAlgorithm as Outside, ReportInputs, TypedModel,
};
use std::io::Write;

mod input;
mod source_binding;
mod spaces;
mod storage;
mod zone_lists;
pub(crate) use storage::{SelectedZoneData, SelectedZoneInputOwners, ZonePredefined};

/// The wrapper forwards to the genuine filesystem/process host; no default sink.
pub(crate) struct ZoneOutputHost<'a>(pub(crate) &'a mut dyn SourceOutputFileHost);
impl SourceOutputFileHost for ZoneOutputHost<'_> {
    fn ensure_mtd_open(&mut self) -> std::io::Result<()> {
        self.0.ensure_mtd_open()
    }
}

pub(crate) struct ZoneGetterOwners<'a, 'backend> {
    pub(crate) input: &'a mut SelectedInputProcessor,
    pub(crate) shortcut: &'a mut SourceInputProcessorShortCut,
    pub(crate) globals: &'a mut SelectedHeatBalanceGlobals,
    pub(crate) output: &'a mut SourceOutputRegistration,
    pub(crate) prescan: &'a SourceOutputPreScan,
    pub(crate) report_input: &'a ReportInputs,
    pub(crate) minimum_report_frequency: &'a str,
    pub(crate) developer: &'a bool,
    pub(crate) clock: Option<&'a crate::weather::day::WeatherGlobalState>,
    pub(crate) backend: &'a mut crate::heat_balance::error_backend::SourceErrorBackend<'backend>,
    pub(crate) minutes_in_time_step: i32,
    pub(crate) host: &'a mut dyn SourceOutputFileHost,
    pub(crate) eio: &'a mut dyn Write,
}

impl SelectedZoneInputOwners {
    pub(crate) fn get_zone_data(
        &mut self,
        raw: &SelectedHeatBalanceInput,
        control: &SelectedHeatBalanceInputState,
        model: &TypedModel,
        owners: &mut ZoneGetterOwners<'_, '_>,
        errors_found: &mut bool,
    ) -> Result<(), HeatBalanceInputError> {
        owners.shortcut.current_module_object = "Zone".into();
        let count = owners.input.get_num_objects_found("Zone");
        owners.globals.num_of_zones = signed(count)?;
        self.zones = Some((0..count).map(|_| SelectedZoneData::default()).collect());
        self.zone_daylight = Some(vec![storage::ZoneDaylight::default(); count]);
        self.resilience = Some(vec![storage::ZoneResilience::default(); count]);
        for index in 0..count {
            // Actual numeric prezero precedes getObjectItem's full six-array reset.
            owners.shortcut.numeric_args.fill(0.0);
            let (num_alphas, num_numbers) =
                raw.get_zone_object_item(index, owners.input, owners.shortcut)?;
            owners.shortcut.alpha_args[0] = owners.shortcut.alpha_args[0]
                .replace('\u{1}', ",")
                .replace('\u{2}', "!");
            let zone = self
                .zones
                .as_mut()
                .and_then(|rows| rows.get_mut(index))
                .ok_or(HeatBalanceInputError::Unavailable("actual Zone allocation"))?;
            zone.name.clone_from(&owners.shortcut.alpha_args[0]);
            let numbers = &owners.shortcut.numeric_args;
            if num_numbers >= 1 {
                zone.relative_north = numbers[0];
            }
            if num_numbers >= 2 {
                zone.origin_m[0] = numbers[1];
            }
            if num_numbers >= 3 {
                zone.origin_m[1] = numbers[2];
            }
            if num_numbers >= 4 {
                zone.origin_m[2] = numbers[3];
            }
            if num_numbers >= 5 {
                zone.of_type = source_int(numbers[4])?;
            }
            zone.of_type = 1;
            if num_numbers >= 6 {
                zone.multiplier = source_int(numbers[5])?;
            }
            if num_numbers >= 7 {
                zone.ceiling_height = numbers[6];
            }
            if num_numbers >= 8 {
                zone.volume = numbers[7];
            }
            if num_numbers >= 9 {
                zone.user_entered_floor_area = numbers[8];
            }
            zone.inside_algorithm = if num_alphas > 1 && !owners.shortcut.alpha_field_blanks[1] {
                inside(&owners.shortcut.alpha_args[1])?
            } else {
                control.inside_convection_algorithm
            };
            zone.outside_algorithm = if num_alphas > 2 && !owners.shortcut.alpha_field_blanks[2] {
                outside(&owners.shortcut.alpha_args[2])?
            } else {
                control.outside_convection_algorithm
            };
            if num_alphas > 3 {
                zone.is_part_of_total_area = match owners.shortcut.alpha_args[3].as_str() {
                    "NO" => false,
                    "YES" => true,
                    _ if owners.shortcut.alpha_field_blanks[3] => true,
                    _ => {
                        return Err(HeatBalanceInputError::Unavailable(
                            "unadmitted Zone floor-area diagnostic branch",
                        ));
                    }
                };
            }
            // No normalized-name coercion or lexical reordering of the live owner.
            let mut typed = model
                .zones
                .iter()
                .filter(|row| row.name == NormalizedName::new(&zone.name));
            let typed = typed.next().ok_or(HeatBalanceInputError::Unavailable(
                "raw/typed Zone identity",
            ))?;
            if model
                .zones
                .iter()
                .filter(|row| row.name == typed.name)
                .count()
                != 1
            {
                return Err(HeatBalanceInputError::Unavailable(
                    "duplicate typed Zone identity",
                ));
            }
            zone.typed_zone_id = Some(typed.id);
            if zone.relative_north.to_bits() != typed.direction_of_relative_north_deg.to_bits()
                || zone.origin_m.map(f64::to_bits)
                    != [typed.origin.x_m, typed.origin.y_m, typed.origin.z_m].map(f64::to_bits)
                || i64::from(zone.multiplier) != i64::from(typed.multiplier)
                || zone.ceiling_height.to_bits() != auto_value(typed.ceiling_height).to_bits()
                || zone.volume.to_bits() != auto_value(typed.volume).to_bits()
                || zone.user_entered_floor_area.to_bits() != auto_value(typed.floor_area).to_bits()
                || zone.inside_algorithm != typed.inside_convection_algorithm.effective()
                || zone.outside_algorithm != typed.outside_convection_algorithm.effective()
                || zone.is_part_of_total_area != typed.is_part_of_total_floor_area
            {
                return Err(HeatBalanceInputError::Unavailable(
                    "raw Zone inputs differ from reused typed geometry operands",
                ));
            }
            let mut request_input = SourceReportVariableInput::new(
                owners.report_input,
                &mut *owners.input,
                crate::heat_balance::source_output_registration::SourceReportVariableEnvironment::from_owners(
                    owners.minimum_report_frequency, &*owners.globals, owners.developer, owners.clock,
                ),
                owners.backend,
            );
            let mut host = ZoneOutputHost(owners.host);
            for (name, units) in [
                ("Zone Outdoor Air Drybulb Temperature", SourceOutputUnits::C),
                ("Zone Outdoor Air Wetbulb Temperature", SourceOutputUnits::C),
                ("Zone Outdoor Air Wind Speed", SourceOutputUnits::MPerS),
                ("Zone Outdoor Air Wind Direction", SourceOutputUnits::Deg),
            ] {
                owners
                    .output
                    .setup_unmetered_zone_real(
                        &mut request_input,
                        owners.prescan,
                        SourceRealOutputDeclaration {
                            name,
                            key: &zone.name,
                            units,
                            store_type: SourceOutputStoreType::Average,
                        },
                        owners.minutes_in_time_step,
                        &mut host,
                        owners.eio,
                    )
                    .map_err(HeatBalanceInputError::Output)?;
            }
        }
        // Native GetZoneData1903: query the actual raw connection field for
        // each retained Zone; this does not consume the equipment object.
        if let Some(zones) = self.zones.as_mut() {
            for zone in zones {
                zone.is_nominal_controlled = owners
                    .input
                    .get_object_item_num_by_field(
                        "ZoneHVAC:EquipmentConnections",
                        "zone_name",
                        &zone.name,
                        raw.source_is_epjson,
                    )
                    .map_err(|_| {
                        HeatBalanceInputError::Unavailable(
                            "actual nominal controlled Zone raw field lookup",
                        )
                    })?
                    > 0;
            }
        }
        self.get_zone_lists(raw.source_is_epjson, model, owners, errors_found)?;
        owners.shortcut.current_module_object = "ZoneGroup".into();
        self.num_zone_groups = signed(owners.input.get_num_objects_found("ZoneGroup"))?;
        if self.num_zone_groups > 0 {
            return Err(HeatBalanceInputError::Unavailable("positive ZoneGroup"));
        }
        owners.shortcut.current_module_object = "ZoneProperty:LocalEnvironment".into();
        let local_environments = owners
            .input
            .get_num_objects_found("ZoneProperty:LocalEnvironment");
        if local_environments > 0 {
            return Err(HeatBalanceInputError::Unavailable(
                "positive Zone local environment",
            ));
        }
        self.zone_predefined = Some(vec![storage::ZonePredefined::default(); count]);
        self.get_space_data(model, owners, errors_found)
    }
}

fn signed(value: usize) -> Result<i32, HeatBalanceInputError> {
    i32::try_from(value)
        .map_err(|_| HeatBalanceInputError::Unavailable("native signed input count overflow"))
}
fn source_int(value: f64) -> Result<i32, HeatBalanceInputError> {
    if !value.is_finite() || !(f64::from(i32::MIN)..=f64::from(i32::MAX)).contains(&value) {
        Err(HeatBalanceInputError::Unavailable(
            "undefined native signed numeric conversion",
        ))
    } else {
        Ok(value as i32)
    }
}
fn auto_value(value: ep_model::AutoOrNumber) -> f64 {
    match value {
        ep_model::AutoOrNumber::AutoCalculate => -99999.0,
        ep_model::AutoOrNumber::Value(value) => value,
    }
}
fn inside(value: &str) -> Result<Inside, HeatBalanceInputError> {
    Ok(match value {
        "SIMPLE" => Inside::Simple,
        "TARP" => Inside::Tarp,
        "CEILINGDIFFUSER" => Inside::CeilingDiffuser,
        "TROMBEWALL" => Inside::TrombeWall,
        "ADAPTIVECONVECTIONALGORITHM" => Inside::AdaptiveConvectionAlgorithm,
        "ASTMC1340" => Inside::AstmC1340,
        _ => {
            return Err(HeatBalanceInputError::Unavailable(
                "unadmitted Zone inside-convection diagnostic branch",
            ));
        }
    })
}
fn outside(value: &str) -> Result<Outside, HeatBalanceInputError> {
    Ok(match value {
        "SIMPLECOMBINED" => Outside::SimpleCombined,
        "TARP" => Outside::Tarp,
        "MOWITT" => Outside::MoWitt,
        "DOE-2" => Outside::Doe2,
        "ADAPTIVECONVECTIONALGORITHM" => Outside::AdaptiveConvectionAlgorithm,
        _ => {
            return Err(HeatBalanceInputError::Unavailable(
                "unadmitted Zone outside-convection diagnostic branch",
            ));
        }
    })
}
