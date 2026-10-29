//! Ordered allocations/local arguments and admitted OtherEquipment getter.
use super::{
    E, SelectedInternalHeatGains, SourceGainReport, SourceMiscGainObject, SourceOtherEquipment,
    SourceOtherFuel, index, unavailable,
};
use crate::heat_balance::error_backend::{SourceDiagnosticFlags, SourceErrorBackend};
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::{
    SelectedHeatBalanceInput, SelectedHeatBalanceInputState, zone::SelectedZoneInputOwners,
};
use crate::heat_balance::selected_input_processor::{
    SelectedInputProcessor, SourceInputFamily, SourceInputProcessorShortCut,
};
use crate::heat_balance::selected_project_data::ProjectInputValue;
use crate::heat_balance::selected_schedule_manager::SelectedScheduleManager;
use crate::heat_balance::source_output_prescan::SourceOutputPreScan;
use crate::heat_balance::source_output_registration::{
    SourceOutputFileHost, SourceOutputRegistration,
};
use crate::weather::day::WeatherGlobalState;
use ep_model::{ReportInputs, TypedModel};
use std::io::Write;

pub(super) const FAMILIES: [&str; 10] = [
    "People",
    "Lights",
    "ElectricEquipment",
    "GasEquipment",
    "HotWaterEquipment",
    "SteamEquipment",
    "OtherEquipment",
    "ElectricEquipment:ITE:AirCooled",
    "ZoneBaseboard:OutdoorTemperatureControlled",
    "ZoneContaminantSourceAndSink:CarbonDioxide",
];

pub(super) struct SourceGainInputOwners<'a, 'b> {
    pub(super) model: &'a TypedModel,
    pub(super) raw: &'a SelectedHeatBalanceInput,
    pub(super) input: &'a mut SelectedInputProcessor,
    pub(super) globals: &'a SelectedHeatBalanceGlobals,
    pub(super) control: &'a mut SelectedHeatBalanceInputState,
    pub(super) zones: &'a mut SelectedZoneInputOwners,
    pub(super) geometry:
        &'a crate::heat_balance::selected_heat_balance_input::surface::SurfaceGeometryInputState,
    pub(super) radiation: &'a crate::heat_balance::radiation::ordinary::OrdinarySolarRadiation,
    pub(super) schedules: &'a mut SelectedScheduleManager,
    pub(super) clock: Option<&'a WeatherGlobalState>,
    pub(super) developer: &'a bool,
    pub(super) backend: &'a mut SourceErrorBackend<'b>,
    pub(super) registration: &'a mut SourceOutputRegistration,
    pub(super) prescan: &'a SourceOutputPreScan,
    pub(super) report_inputs: &'a ReportInputs,
    pub(super) minimum_report_frequency: &'a str,
    pub(super) minutes: i32,
    pub(super) host: &'a mut dyn SourceOutputFileHost,
    pub(super) dictionary_host: Option<
        &'a mut dyn crate::heat_balance::source_output_registration::SourceOutputDictionaryHost,
    >,
    pub(super) eio: &'a mut dyn Write,
    // The same actual DataOutRptPredefined registry and table-entry owner.
    pub(super) registry: &'a crate::heat_balance::predefined_tables::PredefinedRegistry,
    pub(super) table: &'a mut crate::heat_balance::ctf_reports::PredefinedStringTable,
}

impl SelectedInternalHeatGains {
    pub(super) fn get_input(
        &mut self,
        owners: &mut SourceGainInputOwners<'_, '_>,
    ) -> Result<(), E> {
        let nz = usize::try_from(owners.globals.num_of_zones)
            .map_err(|_| unavailable("actual NumOfZones capacity"))?;
        let ns = usize::try_from(owners.globals.num_spaces)
            .map_err(|_| unavailable("actual numSpaces capacity"))?;
        if owners.zones.zones.as_ref().map(Vec::len) != Some(nz)
            || owners.zones.spaces.as_ref().map(Vec::len) != Some(ns)
        {
            return Err(unavailable(
                "same actual internal-gain Zone/Space count association",
            ));
        }
        self.allocate_source_internal_gains_if_unallocated(nz, ns);
        let alloc = &mut self.allocations;
        alloc.zone_report = Some(vec![SourceGainReport::default(); nz]);
        alloc.space_report = Some(vec![SourceGainReport::default(); ns]);
        alloc.zone_electric_end_use = Some(vec![[[0.0; 26]; 4]; nz]);
        alloc.refrigeration_credit = Some(vec![[0.0; 4]; nz]);
        let rep_var_set = vec![true; nz]; // Actual procedure-local owner.
        let (mut na, mut nn) = (0, 0);
        for family in FAMILIES {
            let counts = owners
                .input
                .get_object_def_max_args(family)
                .map_err(|_| unavailable("actual IHG object definition"))?;
            na = na.max(counts.alphas);
            nn = nn.max(counts.numerics);
        }
        // A distinct six-array local carrier; shared dataIPShortCut is untouched.
        let mut local = SourceInputProcessorShortCut {
            alpha_args: vec![String::new(); na],
            alpha_field_names: vec![String::new(); na],
            alpha_field_blanks: vec![true; na],
            numeric_args: vec![0.0; nn],
            numeric_field_names: vec![String::new(); nn],
            numeric_field_blanks: vec![true; nn],
            ..Default::default()
        };
        for (kind, family) in FAMILIES[..9].iter().enumerate() {
            // Native IHG924-925 initializes these procedure locals at the
            // Lights stage, before setupIHGZonesAndSpaces927. Other stages
            // do not construct them. Positive Lights still stop in their
            // existing actual getter before any total-entry write.
            let lighting_totals = if kind == 1 {
                Some((0.0_f64, 0.0_f64))
            } else {
                None
            };
            let objects = self.preprocess(family, kind, owners)?;
            if self.totals[kind] > 0 {
                if kind == 0 {
                    self.allocations.people.allocated = true;
                    self.allocations.people.rows = vec![
                            super::comfort_people::SourcePeopleComfortData::default();
                            usize::try_from(self.totals[0])
                                .map_err(|_| unavailable("TotPeople capacity"))?
                        ];
                    self.read_people(
                        objects
                            .as_deref()
                            .ok_or_else(|| unavailable("actual People misc allocation"))?,
                        &mut local,
                        owners,
                    )?;
                } else if kind == 6 {
                    self.allocations.other_equipment =
                        Some(vec![
                            SourceOtherEquipment::default();
                            usize::try_from(self.totals[kind])
                                .map_err(|_| unavailable("TotOthEquip capacity"))?
                        ]);
                    self.read_other(
                        objects
                            .as_deref()
                            .ok_or_else(|| unavailable("actual other misc allocation"))?,
                        &mut local,
                        owners,
                    )?;
                } else {
                    return Err(unavailable("positive unported IHG family getter"));
                }
            }
            if let Some((sum_area, sum_power)) = lighting_totals {
                // Native1254-1261 is OUTSIDE the TotLights>0 block and
                // precedes ElectricEquipment1263, even for no Lights.
                let density_column = *owners
                    .registry
                    .named_indices
                    .get("pdchInLtDens")
                    .ok_or_else(|| unavailable("actual IHG1254 lighting-density column"))?;
                if sum_area > 0.0 {
                    owners.table.fixed_real_entry_4(
                        density_column,
                        "Interior Lighting Total",
                        sum_power / sum_area,
                    )?;
                } else {
                    // Native DataPrecisionGlobals::constant_zero is +0.0.
                    owners.table.fixed_real_entry_4(
                        density_column,
                        "Interior Lighting Total",
                        0.0,
                    )?;
                }
                let area_column = *owners
                    .registry
                    .named_indices
                    .get("pdchInLtArea")
                    .ok_or_else(|| unavailable("actual IHG1260 lighting-area column"))?;
                owners.table.fixed_real_entry_2(
                    area_column,
                    "Interior Lighting Total",
                    sum_area,
                )?;
                let power_column = *owners
                    .registry
                    .named_indices
                    .get("pdchInLtPower")
                    .ok_or_else(|| unavailable("actual IHG1261 lighting-power column"))?;
                owners.table.fixed_real_entry_2(
                    power_column,
                    "Interior Lighting Total",
                    sum_power,
                )?;
            }
        }
        self.totals[9] = i32::try_from(owners.input.get_num_objects_found(FAMILIES[9]))
            .map_err(|_| unavailable("actual TotCO2Gen capacity"))?;
        self.allocations.co2_generation = Some(vec![
            SourceOtherEquipment::default();
            usize::try_from(self.totals[9]).map_err(
                |_| unavailable("TotCO2Gen capacity")
            )?
        ]);
        if self.totals[9] > 0 {
            return Err(unavailable("positive CO2 generation input getter"));
        }
        drop(rep_var_set); // Original2630 precedes the fatal, outputs and EIO.
        if self.errors_found {
            let message = "GetInternalHeatGains: Errors found in Getting Internal Gains Input, Program Stopped";
            if let Some(clock) = owners.clock {
                let flags = SourceDiagnosticFlags {
                    kickoff: &owners.globals.kickoff_simulation,
                    developer: owners.developer,
                    warmup: &clock.warmup_flag,
                    doing_sizing: &owners.globals.doing_sizing,
                };
                return match owners.backend.show_source_fatal_error(message, &flags) {
                    Err(error) => Err(error),
                    Ok(never) => match never {},
                };
            }
            return match owners.backend.show_fatal_error(message) {
                Err(error) => Err(E::Fatal(error)),
                Ok(never) => match never {},
            };
        }
        self.setup_outputs(owners)?;
        self.write_nominal(owners)
    }

    fn preprocess(
        &mut self,
        family: &str,
        kind: usize,
        owners: &mut SourceGainInputOwners<'_, '_>,
    ) -> Result<Option<Vec<SourceMiscGainObject>>, E> {
        // Exact find, not a count query or inserting operator[]. Definition
        // queries above may already have inserted an actual null family.
        let names: Vec<String> = match owners.input.find_exact_family(family) {
            None => return Ok(None),
            Some(SourceInputFamily::Null) => Vec::new(),
            Some(SourceInputFamily::Objects(rows)) => {
                rows.iter().map(|r| r.object_name.clone()).collect()
            }
        };
        let mut misc = vec![SourceMiscGainObject::default(); names.len()];
        self.totals[kind] = 0;
        if names.is_empty() {
            // These two arrays are actual module globals; move their reached
            // allocated-empty owners directly, never clone a local substitute.
            if kind == 1 {
                self.lights_objects = Some(misc);
                return Ok(None);
            }
            if kind == 2 {
                self.electric_objects = Some(misc);
                return Ok(None);
            }
            return Ok(Some(misc));
        }
        if !matches!(family, "OtherEquipment" | "People") {
            if kind == 1 {
                self.lights_objects = Some(misc);
            } else if kind == 2 {
                self.electric_objects = Some(misc);
            }
            return Err(unavailable("positive IHG expansion alpha-field getter"));
        }
        let raw = owners
            .raw
            .objects
            .get(family)
            .ok_or_else(|| unavailable("same raw IHG objects"))?;
        if raw.len() != names.len() {
            return Err(unavailable("IHG raw/map object association"));
        }
        let mut local_errors = false;
        // names are actual epJSON-map order; raw is the independently retained
        // getIDFObjNum order. Write each misc row at its genuine IDF ordinal.
        for name in names {
            let (ordinal, object) = raw
                .iter()
                .enumerate()
                .find(|(_, row)| row.object_name == name)
                .ok_or_else(|| unavailable("actual IHG IDF object ordinal"))?;
            owners.input.mark_object_as_used(family, &name);
            let row = &mut misc[ordinal];
            row.name = name.to_ascii_uppercase();
            let target = match object
                .fields
                .get("zone_or_zonelist_or_space_or_spacelist_name")
            {
                Some(ProjectInputValue::String(value)) => value.to_ascii_uppercase(),
                _ => return Err(unavailable("actual IHG target alpha operand")),
            };
            let zones = owners
                .zones
                .zones
                .as_ref()
                .ok_or_else(|| unavailable("actual gain Zones"))?;
            if let Some(zone) = zones.iter().find(|zone| zone.name == target) {
                row.space_start_ptr = self.totals[kind] + 1;
                self.totals[kind] = self.totals[kind]
                    .checked_add(zone.num_spaces)
                    .ok_or_else(|| unavailable("actual IHG instance count"))?;
                row.num_spaces = zone.num_spaces;
                if zone.num_spaces != 1 || zone.space_indexes.len() != 1 {
                    return Err(unavailable("positive multi-Space IHG expansion"));
                }
                row.space_nums.push(zone.space_indexes[0]);
                row.names.push(row.name.clone());
                continue;
            }
            let spaces = owners
                .zones
                .spaces
                .as_ref()
                .ok_or_else(|| unavailable("actual gain Spaces"))?;
            if spaces.iter().any(|space| space.name == target) {
                return Err(unavailable("positive direct-Space IHG admission"));
            }
            if let Some(list) = owners
                .zones
                .zone_lists
                .as_ref()
                .and_then(|lists| lists.iter().find(|list| list.name == target))
            {
                if list.typed_zone_list_id.is_none() {
                    return Err(unavailable("unbound actual Source ZoneList"));
                }
                row.space_start_ptr = self.totals[kind]
                    .checked_add(1)
                    .ok_or_else(|| unavailable("actual IHG instance start overflow"))?;
                let mut num_spaces: i32 = 0;
                for &zone_number in list
                    .zone
                    .as_deref()
                    .ok_or_else(|| unavailable("actual IHG ZoneList member allocation"))?
                {
                    let zone = zones
                        .get(index(zone_number)?)
                        .ok_or_else(|| unavailable("actual IHG ZoneList member Zone"))?;
                    num_spaces = num_spaces
                        .checked_add(zone.num_spaces)
                        .ok_or_else(|| unavailable("actual IHG ZoneList Space count"))?;
                    if zone.num_spaces != 1 || zone.space_indexes.len() != 1 {
                        return Err(unavailable("positive multi-Space IHG list expansion"));
                    }
                    for &space_number in &zone.space_indexes {
                        let space = spaces
                            .get(index(space_number)?)
                            .ok_or_else(|| unavailable("actual IHG ZoneList member Space"))?;
                        if space.zone_num != zone_number {
                            return Err(unavailable("actual IHG ZoneList Zone/Space association"));
                        }
                        row.space_nums.push(space_number);
                        row.names.push(format!("{} {}", space.name, row.name));
                    }
                }
                self.totals[kind] = self.totals[kind]
                    .checked_add(num_spaces)
                    .ok_or_else(|| unavailable("actual IHG instance count"))?;
                row.num_spaces = num_spaces;
                row.zone_list_active = true;
                continue;
            }
            if owners
                .zones
                .space_lists
                .as_ref()
                .is_some_and(|v| !v.is_empty())
            {
                return Err(unavailable("positive IHG SpaceList expansion"));
            }
            owners.backend.show_severe_error(&format!(
                "{family}=\"{}\" invalid zone_or_zonelist_or_space_or_spacelist_name=\"{target}\" not found.", row.name))?;
            self.errors_found = true;
            local_errors = true;
        }
        if local_errors {
            owners.backend.show_severe_error(&format!(
                "setupIHGZonesAndSpaces: Errors with invalid names in {family} objects."
            ))?;
            owners
                .backend
                .show_continue_error("...These will not be read in.  Other errors may occur.")?;
            self.totals[kind] = 0;
        }
        if family == "People" && !local_errors {
            check_people_input_association(&misc, owners)?;
        }
        Ok(Some(misc))
    }

    fn read_other(
        &mut self,
        misc: &[SourceMiscGainObject],
        local: &mut SourceInputProcessorShortCut,
        owners: &mut SourceGainInputOwners<'_, '_>,
    ) -> Result<(), E> {
        let mut count = 0;
        for (ordinal, object) in misc.iter().enumerate() {
            let values = owners.raw.get_object_item::<6, 7>(
                "OtherEquipment",
                ordinal,
                owners.input,
                local,
            )?;
            local.alpha_args[..6].clone_from_slice(&values.alphas);
            local.numeric_args[..7].copy_from_slice(&values.numbers);
            let schedule = owners.schedules.get_source_schedule(&values.alphas[3])?;
            let invalid_schedule = if local.alpha_field_blanks[3] {
                Some(format!("{} cannot be empty.", local.alpha_field_names[3]))
            } else if schedule.is_none() {
                Some(format!(
                    "{} = {}, item not found.",
                    local.alpha_field_names[3], values.alphas[3]
                ))
            } else if !match owners.clock {
                Some(clock) => owners.schedules.source_check_min_with_clock(
                    schedule.ok_or_else(|| unavailable("actual schedule"))?,
                    0.0,
                    clock,
                )?,
                None => owners.schedules.source_check_min(
                    schedule.ok_or_else(|| unavailable("actual schedule"))?,
                    0.0,
                )?,
            } {
                Some(format!(
                    "{} = {}, schedule contains values that are < 0",
                    local.alpha_field_names[3], values.alphas[3]
                ))
            } else {
                None
            };
            if let Some(message) = invalid_schedule {
                owners.backend.show_severe_error(&format!(
                    "GetInternalHeatGains: OtherEquipment = {}",
                    values.alphas[0]
                ))?;
                owners.backend.show_continue_error(&message)?;
                self.errors_found = true;
            }
            // Native1833-1853 selects all three operands before the Space
            // loop; the unselected direct level must never seed per-area heat.
            let level_field = match values.alphas[4].as_str() {
                "EQUIPMENTLEVEL" => 0,
                "WATTS/AREA" | "POWER/AREA" => 1,
                "WATTS/PERSON" | "POWER/PERSON" => 2,
                _ => return Err(unavailable("positive OtherEquipment sizing method")),
            };
            let level_value = values.numbers[level_field];
            let level_blank = local.numeric_field_blanks[level_field];
            let level_name = &local.numeric_field_names[level_field];
            for (position, &space_number) in object.space_nums.iter().enumerate() {
                let space = owners
                    .zones
                    .spaces
                    .as_ref()
                    .and_then(|v| v.get(index(space_number).ok()?))
                    .ok_or_else(|| unavailable("actual gain Space association"))?;
                let zone_number = space.zone_num;
                let row = self
                    .allocations
                    .other_equipment
                    .as_mut()
                    .and_then(|v| v.get_mut(count))
                    .ok_or_else(|| unavailable("actual allocated OtherEquipment row"))?;
                row.name = object.names[position].clone();
                row.space_index = space_number;
                row.zone_ptr = zone_number;
                row.schedule = schedule;
                row.fuel = SourceOtherFuel::from_source_name(&values.alphas[1]);
                if row.fuel != SourceOtherFuel::None {
                    if matches!(row.fuel, SourceOtherFuel::Invalid | SourceOtherFuel::Water) {
                        sizing_severe(
                            owners,
                            &format!(
                                "GetInternalHeatGains: OtherEquipment: invalid {} entered={} for {}={}",
                                local.alpha_field_names[1],
                                values.alphas[1],
                                local.alpha_field_names[0],
                                object.name
                            ),
                        )?;
                        self.errors_found = true;
                    }
                    let fuel = row.fuel as i32;
                    let zone = owners
                        .zones
                        .zones
                        .as_mut()
                        .and_then(|rows| index(zone_number).ok().and_then(|i| rows.get_mut(i)))
                        .ok_or_else(|| unavailable("same OtherEquipment fuel-list Zone"))?;
                    if !zone.other_equipment_fuels.contains(&fuel) {
                        zone.other_equipment_fuels.push(fuel);
                    }
                    let space = owners
                        .zones
                        .spaces
                        .as_mut()
                        .and_then(|rows| index(space_number).ok().and_then(|i| rows.get_mut(i)))
                        .ok_or_else(|| unavailable("same OtherEquipment fuel-list Space"))?;
                    if !space.other_equipment_fuels.contains(&fuel) {
                        space.other_equipment_fuels.push(fuel);
                    }
                }
                // Real setDesignLevel3431-3535. Its local result is assigned
                // to the actual equipment only AFTER every reached diagnostic
                // returns; failure preserves the constructor/previous prefix.
                let mut design_level = 0.0;
                if level_blank {
                    let message = format!(
                        "GetInternalHeatGains: OtherEquipment=\"{}\", specifies {}, but that field is blank.  0 OtherEquipment will result.",
                        object.name, level_name
                    );
                    if level_field == 0 && owners.clock.is_none() {
                        // Preserve the old legacy entry point's backend phase.
                        owners.backend.show_warning_error(&message)?;
                    } else {
                        sizing_warning(owners, &message)?;
                    }
                } else {
                    if (level_field == 1 && level_value < 0.0)
                        || (level_field == 2 && level_value <= 0.0)
                    {
                        let value = crate::heat_balance::source_round_format::round(level_value, 3)
                            .map_err(|e| E::Format(format!("IHG design level .3R: {e:?}")))?;
                        sizing_severe(
                            owners,
                            &format!(
                                "GetInternalHeatGains: OtherEquipment=\"{}\", invalid {}, value  [{}]={value}",
                                object.name,
                                level_name,
                                if level_field == 1 { "<0.0" } else { "<=0.0" }
                            ),
                        )?;
                        self.errors_found = true;
                    }
                    if !self.errors_found {
                        if level_field == 0 {
                            design_level = level_value * 1.0;
                        } else if level_field == 1 {
                            let space = owners
                                .zones
                                .spaces
                                .as_ref()
                                .and_then(|rows| index(space_number).ok().and_then(|i| rows.get(i)))
                                .ok_or_else(|| unavailable("same OtherEquipment per-area Space"))?;
                            design_level = level_value * space.floor_area;
                            if space.floor_area <= 0.0 && !space.is_remainder_space {
                                sizing_warning(
                                    owners,
                                    &format!(
                                        "GetInternalHeatGains: OtherEquipment=\"{}\", specifies {}, but Space Floor Area = 0.  0 OtherEquipment will result.",
                                        object.name, level_name
                                    ),
                                )?;
                            }
                        } else {
                            let total_occupants = owners
                                .zones
                                .spaces
                                .as_ref()
                                .and_then(|rows| index(space_number).ok().and_then(|i| rows.get(i)))
                                .ok_or_else(|| {
                                    unavailable("same OtherEquipment per-person Space totals")
                                })?
                                .total_occupants;
                            // Native3554 computes before warning and does not
                            // replace a negative design-total product with zero.
                            design_level = level_value * total_occupants;
                            if total_occupants <= 0.0 {
                                sizing_warning(
                                    owners,
                                    &format!(
                                        "GetInternalHeatGains: OtherEquipment=\"{}\", specifies {}, but Total Occupants = 0.  0 OtherEquipment will result.",
                                        object.name, level_name
                                    ),
                                )?;
                            }
                        }
                    }
                }
                row.design_level = design_level;
                if row.design_level < 0.0
                    && !matches!(row.fuel, SourceOtherFuel::Invalid | SourceOtherFuel::None)
                {
                    sizing_severe(
                        owners,
                        &format!(
                            "GetInternalHeatGains: OtherEquipment=\"{}\", {} is not allowed to be negative",
                            object.name, level_name,
                        ),
                    )?;
                    owners.backend.show_continue_error(&format!(
                        "... when a fuel type of {} is specified.",
                        row.fuel.source_name()?
                    ))?;
                    self.errors_found = true;
                }
                let schedule = schedule.ok_or_else(|| {
                    unavailable("invalid schedule before actual pointer dereference")
                })?;
                row.nominal_min_design_level = row.design_level
                    * match owners.clock {
                        Some(clock) => owners.schedules.source_min_with_clock(schedule, clock)?,
                        None => owners.schedules.source_min(schedule)?,
                    };
                row.nominal_max_design_level = row.design_level
                    * match owners.clock {
                        Some(clock) => owners.schedules.source_max_with_clock(schedule, clock)?,
                        None => owners.schedules.source_max(schedule)?,
                    };
                row.fraction_latent = values.numbers[3];
                row.fraction_radiant = values.numbers[4];
                row.fraction_lost = values.numbers[5];
                if values.num_numbers == 7 || !local.numeric_field_blanks[6] {
                    row.co2_rate_factor = values.numbers[6];
                }
                for (bad, condition) in [
                    (row.co2_rate_factor < 0.0, "< 0.0"),
                    (row.co2_rate_factor > 4.0e-7, "> 4.0E-7"),
                ] {
                    if bad {
                        let number =
                            crate::heat_balance::source_round_format::round(values.numbers[6], 2)
                                .map_err(|e| E::Format(format!("IHG CO2 .2R: {e:?}")))?;
                        owners.backend.show_severe_error(&format!("GetInternalHeatGains: OtherEquipment=\"{}\", {} {condition}, value ={number}",
                            object.name, local.numeric_field_names[6]))?;
                        self.errors_found = true;
                    }
                }
                row.fraction_convected =
                    1.0 - (row.fraction_latent + row.fraction_radiant + row.fraction_lost);
                if row.fraction_convected.abs() <= 0.001 {
                    row.fraction_convected = 0.0;
                }
                if row.fraction_convected < 0.0 {
                    owners.backend.show_severe_error(&format!(
                        "GetInternalHeatGains: OtherEquipment=\"{}\", Sum of Fractions > 1.0",
                        object.name
                    ))?;
                    self.errors_found = true;
                }
                row.end_use_subcategory = if values.num_alphas > 5 {
                    values.alphas[5].clone()
                } else {
                    "General".into()
                };
                if owners.globals.any_energy_management_system_in_model {
                    return Err(unavailable("positive IHG EMS actuator registration"));
                }
                if !self.errors_found {
                    self.setup_space_device(space_number, count, owners.backend)?;
                }
                count += 1;
            }
        }
        Ok(())
    }
}

// Bind only genuine input identities. This does not replace Native raw-order
// rows with typed arena order, nor derive a Native spaceStartPtr from typed IDs.
fn check_people_input_association(
    misc: &[SourceMiscGainObject],
    owners: &SourceGainInputOwners<'_, '_>,
) -> Result<(), E> {
    let bindings = &owners.model.source_people_inputs;
    if bindings.is_empty() && !misc.iter().any(|row| row.zone_list_active) {
        // Older direct getter boundary fixtures predate explicit Source input
        // associations. Positive list expansion always requires the real map.
        return Ok(());
    }
    if bindings.len() != misc.len() {
        return Err(unavailable("People raw/typed object association count"));
    }
    let zones = owners
        .zones
        .zones
        .as_deref()
        .ok_or_else(|| unavailable("People association actual Zones"))?;
    let spaces = owners
        .zones
        .spaces
        .as_deref()
        .ok_or_else(|| unavailable("People association actual Spaces"))?;
    let mut seen = vec![false; owners.model.people.len()];
    let raw_rows = owners
        .raw
        .objects
        .get("People")
        .ok_or_else(|| unavailable("People retained raw family identity"))?;
    if raw_rows.len() != misc.len() {
        return Err(unavailable("People retained raw ordinal count"));
    }
    for (ordinal, object) in misc.iter().enumerate() {
        let raw = &raw_rows[ordinal];
        let matching = bindings
            .iter()
            .filter(|binding| binding.object_name == raw.object_name)
            .collect::<Vec<_>>();
        if matching.len() != 1 {
            return Err(unavailable("People exact raw object association"));
        }
        let binding = matching[0];
        if raw.object_name.to_ascii_uppercase() != object.name {
            return Err(unavailable("People retained raw/Native name association"));
        }
        let target = match raw
            .fields
            .get("zone_or_zonelist_or_space_or_spacelist_name")
        {
            Some(ProjectInputValue::String(value)) => value.to_ascii_uppercase(),
            _ => return Err(unavailable("People retained raw target identity")),
        };
        let rows = owners
            .model
            .people
            .get(binding.instances.clone())
            .ok_or_else(|| unavailable("People actual dense typed instance range"))?;
        if rows.len() != object.space_nums.len()
            || rows.len() != object.names.len()
            || usize::try_from(object.num_spaces).ok() != Some(rows.len())
            || rows.is_empty()
        {
            return Err(unavailable("People raw/typed expanded instance count"));
        }
        match binding.target {
            ep_model::SourcePeopleInputTarget::Zone(id) => {
                if object.zone_list_active
                    || rows.len() != 1
                    || rows[0].zone != id
                    || !zones
                        .iter()
                        .any(|zone| zone.typed_zone_id == Some(id) && zone.name == target)
                {
                    return Err(unavailable("People direct Zone precedence association"));
                }
            }
            ep_model::SourcePeopleInputTarget::ZoneList(id) => {
                let lists = owners
                    .zones
                    .zone_lists
                    .as_deref()
                    .ok_or_else(|| unavailable("People actual ZoneList allocation"))?;
                let lists = lists
                    .iter()
                    .filter(|list| list.typed_zone_list_id == Some(id))
                    .collect::<Vec<_>>();
                let actual_zones = object
                    .space_nums
                    .iter()
                    .map(|&number| {
                        spaces
                            .get(index(number)?)
                            .map(|space| space.zone_num)
                            .ok_or_else(|| unavailable("People list member Space/Zone owner"))
                    })
                    .collect::<Result<Vec<_>, E>>()?;
                if !object.zone_list_active
                    || lists.len() != 1
                    || lists[0].name != target
                    || lists[0].zone.as_deref() != Some(actual_zones.as_slice())
                {
                    return Err(unavailable("People actual ordered ZoneList association"));
                }
            }
        }
        for (position, row) in rows.iter().enumerate() {
            let at = binding.instances.start + position;
            let slot = seen
                .get_mut(at)
                .ok_or_else(|| unavailable("People typed range coverage"))?;
            let space = spaces
                .get(index(object.space_nums[position])?)
                .ok_or_else(|| unavailable("People actual expanded Space"))?;
            let zone = zones
                .get(index(space.zone_num)?)
                .ok_or_else(|| unavailable("People actual expanded Zone"))?;
            let typed_space = owners
                .model
                .spaces
                .iter()
                .find(|typed| Some(typed.id) == space.typed_space_id)
                .ok_or_else(|| unavailable("People genuine typed implicit Space"))?;
            if *slot
                || usize::try_from(row.id.0).ok() != Some(at)
                || row.name.0 != object.names[position]
                || zone.typed_zone_id != Some(row.zone)
                || typed_space.zone != row.zone
                || typed_space.origin != ep_model::SpaceOrigin::AutoZoneDefault
                || zone.num_spaces != 1
                || zone.space_indexes.as_slice() != [object.space_nums[position]]
            {
                return Err(unavailable(
                    "People typed instance/name/Zone/implicit-Space bijection",
                ));
            }
            *slot = true;
        }
    }
    if seen.iter().any(|seen| !seen) {
        return Err(unavailable("People complete typed instance coverage"));
    }
    Ok(())
}

// New per-area diagnostics require actual Source phase owners. Legacy entry
// points have no clock and stop here instead of inventing its warmup flag.
fn sizing_warning(owners: &mut SourceGainInputOwners<'_, '_>, message: &str) -> Result<(), E> {
    let clock = owners
        .clock
        .ok_or_else(|| unavailable("actual OtherEquipment sizing diagnostic clock"))?;
    let flags = SourceDiagnosticFlags {
        kickoff: &owners.globals.kickoff_simulation,
        developer: owners.developer,
        warmup: &clock.warmup_flag,
        doing_sizing: &owners.globals.doing_sizing,
    };
    owners.backend.show_source_warning_error(message, &flags)
}
fn sizing_severe(owners: &mut SourceGainInputOwners<'_, '_>, message: &str) -> Result<(), E> {
    let clock = owners
        .clock
        .ok_or_else(|| unavailable("actual OtherEquipment sizing diagnostic clock"))?;
    let flags = SourceDiagnosticFlags {
        kickoff: &owners.globals.kickoff_simulation,
        developer: owners.developer,
        warmup: &clock.warmup_flag,
        doing_sizing: &owners.globals.doing_sizing,
    };
    owners.backend.show_source_severe_error(message, &flags)
}
