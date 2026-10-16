//! Ordered allocations/local arguments and admitted OtherEquipment getter.
use super::{
    E, SelectedInternalHeatGains, SourceGainReport, SourceMiscGainObject, SourceOtherEquipment,
    SourceOtherFuel, index, unavailable,
};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::{
    SelectedHeatBalanceInput, zone::SelectedZoneInputOwners,
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
use ep_model::ReportInputs;
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
    pub(super) raw: &'a SelectedHeatBalanceInput,
    pub(super) input: &'a mut SelectedInputProcessor,
    pub(super) globals: &'a SelectedHeatBalanceGlobals,
    pub(super) zones: &'a mut SelectedZoneInputOwners,
    pub(super) schedules: &'a mut SelectedScheduleManager,
    pub(super) backend: &'a mut SourceErrorBackend<'b>,
    pub(super) registration: &'a mut SourceOutputRegistration,
    pub(super) prescan: &'a SourceOutputPreScan,
    pub(super) report_inputs: &'a ReportInputs,
    pub(super) minimum_report_frequency: &'a str,
    pub(super) minutes: i32,
    pub(super) host: &'a mut dyn SourceOutputFileHost,
    pub(super) eio: &'a mut dyn Write,
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
        let alloc = &mut self.allocations;
        if alloc.zone_int_gain.is_none() {
            // DataHeatBalance1044-1047: each is the same retained collection.
            alloc.zone_int_gain = Some(vec![[0.0; 2]; nz]);
            alloc.space_int_gain = Some(vec![[0.0; 2]; ns]);
            alloc.space_devices = Some(vec![Default::default(); ns]);
            alloc.space_power_reduction_factor = Some(vec![1.0; ns]);
        }
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
            let objects = self.preprocess(family, kind, owners)?;
            if self.totals[kind] > 0 {
                if kind != 6 {
                    return Err(unavailable("positive unported IHG family getter"));
                }
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
            return match owners.backend.show_fatal_error(
                "GetInternalHeatGains: Errors found in Getting Internal Gains Input, Program Stopped") {
                Err(error) => Err(E::Fatal(error)), Ok(never) => match never {},
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
        if family != "OtherEquipment" {
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
            // Positive ZoneList/SpaceList are excluded by the actual admission.
            if owners.zones.num_zone_lists > 0
                || owners
                    .zones
                    .space_lists
                    .as_ref()
                    .is_some_and(|v| !v.is_empty())
            {
                return Err(unavailable("positive IHG ZoneList/SpaceList expansion"));
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
            let schedule = owners.schedules.get_source_schedule(&values.alphas[3]);
            let invalid_schedule = if local.alpha_field_blanks[3] {
                Some(format!("{} cannot be empty.", local.alpha_field_names[3]))
            } else if schedule.is_none() {
                Some(format!(
                    "{} = {}, item not found.",
                    local.alpha_field_names[3], values.alphas[3]
                ))
            } else if !owners
                .schedules
                .source_check_min(schedule.ok_or_else(|| unavailable("actual schedule"))?, 0.0)?
            {
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
            if values.alphas[4] != "EQUIPMENTLEVEL" {
                return Err(unavailable("positive OtherEquipment sizing method"));
            }
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
                if values.alphas[1] != "NONE" {
                    return Err(unavailable(
                        "positive OtherEquipment fuel validation/meters",
                    ));
                }
                row.fuel = SourceOtherFuel::None;
                if local.numeric_field_blanks[0] {
                    owners.backend.show_warning_error(&format!(
                        "GetInternalHeatGains: OtherEquipment=\"{}\", specifies {}, but that field is blank.  0 OtherEquipment will result.",
                        object.name, local.numeric_field_names[0]))?;
                    row.design_level = 0.0;
                } else if self.errors_found {
                    row.design_level = 0.0;
                } else {
                    row.design_level = values.numbers[0] * 1.0;
                }
                let schedule = schedule.ok_or_else(|| {
                    unavailable("invalid schedule before actual pointer dereference")
                })?;
                row.nominal_min_design_level =
                    row.design_level * owners.schedules.source_min(schedule)?;
                row.nominal_max_design_level =
                    row.design_level * owners.schedules.source_max(schedule)?;
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
