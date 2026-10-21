//! Real represented IHG state observations; Native-only cells stay separate.
use super::beam_observe::{address, bits, same, vector};
use super::*;
use hb::ctf_initialization_context::OrdinaryGainRateTarget;

fn report(
    row: &hb::ctf_initialization_context::internal_heat_gains_input::SourceGainReport,
    n: &Value,
) -> Result {
    // REPORT_FIELDS is emitted by the author from the actual source field
    // declarations/comments. It is a literal member map, never solver math.
    let f = member(n, "f64")?;
    bits(
        row.people_rad_gain,
        member(f, "PeopleRadGain")?,
        "actual report PeopleRadGain",
    )?;
    bits(
        row.people_con_gain,
        member(f, "PeopleConGain")?,
        "actual report PeopleConGain",
    )?;
    bits(
        row.people_sen_gain,
        member(f, "PeopleSenGain")?,
        "actual report PeopleSenGain",
    )?;
    bits(
        row.people_num_occ,
        member(f, "PeopleNumOcc")?,
        "actual report PeopleNumOcc",
    )?;
    bits(
        row.people_lat_gain,
        member(f, "PeopleLatGain")?,
        "actual report PeopleLatGain",
    )?;
    bits(
        row.people_tot_gain,
        member(f, "PeopleTotGain")?,
        "actual report PeopleTotGain",
    )?;
    bits(
        row.people_rad_gain_rate,
        member(f, "PeopleRadGainRate")?,
        "actual report PeopleRadGainRate",
    )?;
    bits(
        row.people_con_gain_rate,
        member(f, "PeopleConGainRate")?,
        "actual report PeopleConGainRate",
    )?;
    bits(
        row.people_sen_gain_rate,
        member(f, "PeopleSenGainRate")?,
        "actual report PeopleSenGainRate",
    )?;
    bits(
        row.people_lat_gain_rate,
        member(f, "PeopleLatGainRate")?,
        "actual report PeopleLatGainRate",
    )?;
    bits(
        row.people_tot_gain_rate,
        member(f, "PeopleTotGainRate")?,
        "actual report PeopleTotGainRate",
    )?;
    bits(
        row.lts_power,
        member(f, "LtsPower")?,
        "actual report LtsPower",
    )?;
    bits(
        row.lts_elec_consump,
        member(f, "LtsElecConsump")?,
        "actual report LtsElecConsump",
    )?;
    bits(
        row.lts_rad_gain,
        member(f, "LtsRadGain")?,
        "actual report LtsRadGain",
    )?;
    bits(
        row.lts_vis_gain,
        member(f, "LtsVisGain")?,
        "actual report LtsVisGain",
    )?;
    bits(
        row.lts_con_gain,
        member(f, "LtsConGain")?,
        "actual report LtsConGain",
    )?;
    bits(
        row.lts_ret_air_gain,
        member(f, "LtsRetAirGain")?,
        "actual report LtsRetAirGain",
    )?;
    bits(
        row.lts_tot_gain,
        member(f, "LtsTotGain")?,
        "actual report LtsTotGain",
    )?;
    bits(
        row.lts_rad_gain_rate,
        member(f, "LtsRadGainRate")?,
        "actual report LtsRadGainRate",
    )?;
    bits(
        row.lts_vis_gain_rate,
        member(f, "LtsVisGainRate")?,
        "actual report LtsVisGainRate",
    )?;
    bits(
        row.lts_con_gain_rate,
        member(f, "LtsConGainRate")?,
        "actual report LtsConGainRate",
    )?;
    bits(
        row.lts_ret_air_gain_rate,
        member(f, "LtsRetAirGainRate")?,
        "actual report LtsRetAirGainRate",
    )?;
    bits(
        row.lts_tot_gain_rate,
        member(f, "LtsTotGainRate")?,
        "actual report LtsTotGainRate",
    )?;
    bits(
        row.base_heat_power,
        member(f, "BaseHeatPower")?,
        "actual report BaseHeatPower",
    )?;
    bits(
        row.base_heat_elec_cons,
        member(f, "BaseHeatElecCons")?,
        "actual report BaseHeatElecCons",
    )?;
    bits(
        row.base_heat_rad_gain,
        member(f, "BaseHeatRadGain")?,
        "actual report BaseHeatRadGain",
    )?;
    bits(
        row.base_heat_con_gain,
        member(f, "BaseHeatConGain")?,
        "actual report BaseHeatConGain",
    )?;
    bits(
        row.base_heat_tot_gain,
        member(f, "BaseHeatTotGain")?,
        "actual report BaseHeatTotGain",
    )?;
    bits(
        row.base_heat_rad_gain_rate,
        member(f, "BaseHeatRadGainRate")?,
        "actual report BaseHeatRadGainRate",
    )?;
    bits(
        row.base_heat_con_gain_rate,
        member(f, "BaseHeatConGainRate")?,
        "actual report BaseHeatConGainRate",
    )?;
    bits(
        row.base_heat_tot_gain_rate,
        member(f, "BaseHeatTotGainRate")?,
        "actual report BaseHeatTotGainRate",
    )?;
    bits(
        row.elec_power,
        member(f, "ElecPower")?,
        "actual report ElecPower",
    )?;
    bits(
        row.elec_consump,
        member(f, "ElecConsump")?,
        "actual report ElecConsump",
    )?;
    bits(
        row.elec_rad_gain,
        member(f, "ElecRadGain")?,
        "actual report ElecRadGain",
    )?;
    bits(
        row.elec_con_gain,
        member(f, "ElecConGain")?,
        "actual report ElecConGain",
    )?;
    bits(
        row.elec_lat_gain,
        member(f, "ElecLatGain")?,
        "actual report ElecLatGain",
    )?;
    bits(
        row.elec_lost,
        member(f, "ElecLost")?,
        "actual report ElecLost",
    )?;
    bits(
        row.elec_tot_gain,
        member(f, "ElecTotGain")?,
        "actual report ElecTotGain",
    )?;
    bits(
        row.elec_rad_gain_rate,
        member(f, "ElecRadGainRate")?,
        "actual report ElecRadGainRate",
    )?;
    bits(
        row.elec_con_gain_rate,
        member(f, "ElecConGainRate")?,
        "actual report ElecConGainRate",
    )?;
    bits(
        row.elec_lat_gain_rate,
        member(f, "ElecLatGainRate")?,
        "actual report ElecLatGainRate",
    )?;
    bits(
        row.elec_lost_rate,
        member(f, "ElecLostRate")?,
        "actual report ElecLostRate",
    )?;
    bits(
        row.elec_tot_gain_rate,
        member(f, "ElecTotGainRate")?,
        "actual report ElecTotGainRate",
    )?;
    bits(
        row.gas_power,
        member(f, "GasPower")?,
        "actual report GasPower",
    )?;
    bits(
        row.gas_consump,
        member(f, "GasConsump")?,
        "actual report GasConsump",
    )?;
    bits(
        row.gas_rad_gain,
        member(f, "GasRadGain")?,
        "actual report GasRadGain",
    )?;
    bits(
        row.gas_con_gain,
        member(f, "GasConGain")?,
        "actual report GasConGain",
    )?;
    bits(
        row.gas_lat_gain,
        member(f, "GasLatGain")?,
        "actual report GasLatGain",
    )?;
    bits(row.gas_lost, member(f, "GasLost")?, "actual report GasLost")?;
    bits(
        row.gas_tot_gain,
        member(f, "GasTotGain")?,
        "actual report GasTotGain",
    )?;
    bits(
        row.gas_rad_gain_rate,
        member(f, "GasRadGainRate")?,
        "actual report GasRadGainRate",
    )?;
    bits(
        row.gas_con_gain_rate,
        member(f, "GasConGainRate")?,
        "actual report GasConGainRate",
    )?;
    bits(
        row.gas_lat_gain_rate,
        member(f, "GasLatGainRate")?,
        "actual report GasLatGainRate",
    )?;
    bits(
        row.gas_lost_rate,
        member(f, "GasLostRate")?,
        "actual report GasLostRate",
    )?;
    bits(
        row.gas_tot_gain_rate,
        member(f, "GasTotGainRate")?,
        "actual report GasTotGainRate",
    )?;
    bits(row.hw_power, member(f, "HWPower")?, "actual report HWPower")?;
    bits(
        row.hw_consump,
        member(f, "HWConsump")?,
        "actual report HWConsump",
    )?;
    bits(
        row.hw_rad_gain,
        member(f, "HWRadGain")?,
        "actual report HWRadGain",
    )?;
    bits(
        row.hw_con_gain,
        member(f, "HWConGain")?,
        "actual report HWConGain",
    )?;
    bits(
        row.hw_lat_gain,
        member(f, "HWLatGain")?,
        "actual report HWLatGain",
    )?;
    bits(row.hw_lost, member(f, "HWLost")?, "actual report HWLost")?;
    bits(
        row.hw_tot_gain,
        member(f, "HWTotGain")?,
        "actual report HWTotGain",
    )?;
    bits(
        row.hw_rad_gain_rate,
        member(f, "HWRadGainRate")?,
        "actual report HWRadGainRate",
    )?;
    bits(
        row.hw_con_gain_rate,
        member(f, "HWConGainRate")?,
        "actual report HWConGainRate",
    )?;
    bits(
        row.hw_lat_gain_rate,
        member(f, "HWLatGainRate")?,
        "actual report HWLatGainRate",
    )?;
    bits(
        row.hw_lost_rate,
        member(f, "HWLostRate")?,
        "actual report HWLostRate",
    )?;
    bits(
        row.hw_tot_gain_rate,
        member(f, "HWTotGainRate")?,
        "actual report HWTotGainRate",
    )?;
    bits(
        row.steam_power,
        member(f, "SteamPower")?,
        "actual report SteamPower",
    )?;
    bits(
        row.steam_consump,
        member(f, "SteamConsump")?,
        "actual report SteamConsump",
    )?;
    bits(
        row.steam_rad_gain,
        member(f, "SteamRadGain")?,
        "actual report SteamRadGain",
    )?;
    bits(
        row.steam_con_gain,
        member(f, "SteamConGain")?,
        "actual report SteamConGain",
    )?;
    bits(
        row.steam_lat_gain,
        member(f, "SteamLatGain")?,
        "actual report SteamLatGain",
    )?;
    bits(
        row.steam_lost,
        member(f, "SteamLost")?,
        "actual report SteamLost",
    )?;
    bits(
        row.steam_tot_gain,
        member(f, "SteamTotGain")?,
        "actual report SteamTotGain",
    )?;
    bits(
        row.steam_rad_gain_rate,
        member(f, "SteamRadGainRate")?,
        "actual report SteamRadGainRate",
    )?;
    bits(
        row.steam_con_gain_rate,
        member(f, "SteamConGainRate")?,
        "actual report SteamConGainRate",
    )?;
    bits(
        row.steam_lat_gain_rate,
        member(f, "SteamLatGainRate")?,
        "actual report SteamLatGainRate",
    )?;
    bits(
        row.steam_lost_rate,
        member(f, "SteamLostRate")?,
        "actual report SteamLostRate",
    )?;
    bits(
        row.steam_tot_gain_rate,
        member(f, "SteamTotGainRate")?,
        "actual report SteamTotGainRate",
    )?;
    bits(row.co2_rate, member(f, "CO2Rate")?, "actual report CO2Rate")?;
    bits(row.gc_rate, member(f, "GCRate")?, "actual report GCRate")?;
    bits(
        row.total[0],
        member(f, "TotRadiantGain")?,
        "actual report TotRadiantGain",
    )?;
    bits(
        row.total[1],
        member(f, "TotRadiantGainRate")?,
        "actual report TotRadiantGainRate",
    )?;
    bits(
        row.total[2],
        member(f, "TotVisHeatGain")?,
        "actual report TotVisHeatGain",
    )?;
    bits(
        row.total[3],
        member(f, "TotVisHeatGainRate")?,
        "actual report TotVisHeatGainRate",
    )?;
    bits(
        row.total[4],
        member(f, "TotConvectiveGain")?,
        "actual report TotConvectiveGain",
    )?;
    bits(
        row.total[5],
        member(f, "TotConvectiveGainRate")?,
        "actual report TotConvectiveGainRate",
    )?;
    bits(
        row.total[6],
        member(f, "TotLatentGain")?,
        "actual report TotLatentGain",
    )?;
    bits(
        row.total[7],
        member(f, "TotLatentGainRate")?,
        "actual report TotLatentGainRate",
    )?;
    bits(
        row.total[8],
        member(f, "TotTotalHeatGain")?,
        "actual report TotTotalHeatGain",
    )?;
    bits(
        row.total[9],
        member(f, "TotTotalHeatGainRate")?,
        "actual report TotTotalHeatGainRate",
    )?;
    bits(
        row.other[0],
        member(f, "OtherRadGain")?,
        "actual report OtherRadGain",
    )?;
    bits(
        row.other[1],
        member(f, "OtherRadGainRate")?,
        "actual report OtherRadGainRate",
    )?;
    bits(
        row.other[2],
        member(f, "OtherConGain")?,
        "actual report OtherConGain",
    )?;
    bits(
        row.other[3],
        member(f, "OtherConGainRate")?,
        "actual report OtherConGainRate",
    )?;
    bits(
        row.other[4],
        member(f, "OtherLatGain")?,
        "actual report OtherLatGain",
    )?;
    bits(
        row.other[5],
        member(f, "OtherLatGainRate")?,
        "actual report OtherLatGainRate",
    )?;
    bits(
        row.other[6],
        member(f, "OtherLost")?,
        "actual report OtherLost",
    )?;
    bits(
        row.other[7],
        member(f, "OtherLostRate")?,
        "actual report OtherLostRate",
    )?;
    bits(
        row.other[8],
        member(f, "OtherTotGain")?,
        "actual report OtherTotGain",
    )?;
    bits(
        row.other[9],
        member(f, "OtherTotGainRate")?,
        "actual report OtherTotGainRate",
    )?;
    vector(
        &row.other_power,
        member(n, "OtherPower")?,
        "actual OtherPower fuel array",
    )?;
    vector(
        &row.other_consumption,
        member(n, "OtherConsump")?,
        "actual OtherConsump fuel array",
    )?;
    Ok(())
}

fn bool_array(values: &Option<Vec<bool>>, expected: &Value, label: &str) -> Result {
    same(
        expected["allocated"].as_bool(),
        Some(values.is_some()),
        label,
    )?;
    let n = values.as_ref().map_or(0, Vec::len);
    same(count(member(expected, "size")?)?, n, label)?;
    same(int(member(expected, "lower")?)?, 1, label)?;
    same(int(member(expected, "upper")?)?, i32::try_from(n)?, label)?;
    Ok(())
}
pub(crate) struct GainIdentity {
    native: Vec<u64>,
    source: Vec<usize>,
}
impl GainFactory {
    fn gain_identity(&self, n: &Value) -> Result<GainIdentity> {
        let gains = &self.reports.selected_internal_heat_gains;
        let mut native = vec![
            address(member(n, "state_address")?)?,
            address(member(n, "manager_address")?)?,
            address(member(n, "actual_electric_service_address")?)?,
            address(member(member(n, "refrigeration")?, "address")?)?,
        ];
        let mut source = vec![
            &self.reports as *const _ as usize,
            gains as *const _ as usize,
            self.reports
                .facility_electric_service
                .as_ref()
                .ok_or("actual electric-service factory result")? as *const _ as usize,
            &self.modules.refrigeration as *const _ as usize,
        ];
        let a = &gains.allocations;
        for (key, source_rows) in [
            (
                "equipment",
                a.other_equipment
                    .as_ref()
                    .map(|v| v.iter().map(|r| r as *const _ as usize).collect::<Vec<_>>()),
            ),
            (
                "zone_reports",
                a.zone_report
                    .as_ref()
                    .map(|v| v.iter().map(|r| r as *const _ as usize).collect::<Vec<_>>()),
            ),
            (
                "space_reports",
                a.space_report
                    .as_ref()
                    .map(|v| v.iter().map(|r| r as *const _ as usize).collect::<Vec<_>>()),
            ),
        ] {
            let observed = array(member(n, key)?)?;
            let source_rows = source_rows.unwrap_or_default();
            same(observed.len(), source_rows.len(), key)?;
            for row in observed {
                native.push(address(member(row, "address")?)?);
            }
            source.extend(source_rows);
        }
        for row in array(member(n, "spaces")?)? {
            native.push(address(member(row, "address")?)?);
            for device in array(member(row, "devices")?)? {
                native.push(address(member(device, "address")?)?);
                for (_, value) in object(member(device, "pointers")?)? {
                    native.push(address(value)?);
                }
            }
        }
        for row in a.space_devices.as_deref().unwrap_or_default() {
            source.push(row as *const _ as usize);
            if row.number_of_devices > 0 {
                let devices = row.devices.as_ref().ok_or("actual device storage")?;
                for device in devices.iter().take(row.number_of_devices) {
                    source.push(device as *const _ as usize);
                }
            }
        }
        for (key, rows) in [
            (
                "zone_air_latent",
                self.air
                    .zones
                    .as_ref()
                    .map(|v| v.iter().map(|r| r as *const _ as usize).collect::<Vec<_>>()),
            ),
            (
                "space_air_latent",
                self.air
                    .spaces
                    .as_ref()
                    .map(|v| v.iter().map(|r| r as *const _ as usize).collect::<Vec<_>>()),
            ),
        ] {
            let observed = array(member(n, key)?)?;
            let rows = rows.unwrap_or_default();
            same(rows.len(), observed.len(), key)?;
            source.extend(rows);
            for row in observed {
                native.push(address(member(row, "address")?)?);
            }
        }
        if source.contains(&0) {
            return Err(invalid("actual Source nonnull gain owners").into());
        }
        Ok(GainIdentity { native, source })
    }
    pub(crate) fn retain_gain_identity(
        &self,
        n: &Value,
        identity: &mut Option<GainIdentity>,
    ) -> Result {
        let now = self.gain_identity(n)?;
        if let Some(old) = identity {
            same(
                &now.native,
                &old.native,
                "Native repeated IHG owner/row/pointer identity",
            )?;
            same(
                &now.source,
                &old.source,
                "Source repeated IHG owner/row identity",
            )?;
        } else {
            *identity = Some(now);
        }
        Ok(())
    }
    pub(crate) fn assert_gains(&self, n: &Value) -> Result {
        let gains = &self.reports.selected_internal_heat_gains;
        let a = &gains.allocations;
        let globals = self
            .reports
            .selected_heat_balance_globals
            .as_ref()
            .ok_or("actual globals")?;
        same(
            gains.get_input_flag,
            boolean(member(n, "GetInternalHeatGainsInputFlag")?)?,
            "actual GetInternalHeatGainsInputFlag",
        )?;
        for (key, value) in [
            ("NumOfZones", globals.num_of_zones),
            ("numSpaces", globals.num_spaces),
            ("TotSurfaces", self.geometry.total_surfaces),
            ("HourOfDay", self.weather.global.hour_of_day),
            ("TimeStep", self.weather.global.time_step),
        ] {
            same(value, int(member(n, key)?)?, key)?;
        }
        for (key, value) in [
            (
                "radiant_enclosures",
                self.radiation.num_radiant_enclosures(),
            ),
            ("solar_enclosures", self.radiation.num_solar_enclosures()),
        ] {
            let observed = member(n, key)?;
            if let Some(rows) = observed.as_array() {
                same(usize::try_from(value)?, rows.len(), key)?;
            } else {
                same(value, int(observed)?, key)?;
            }
        }
        same(
            self.weather.global.begin_envrn_flag,
            boolean(member(n, "BeginEnvrnFlag")?)?,
            "actual BeginEnvrnFlag",
        )?;
        bits(
            self.weather.global.time_step_zone_sec,
            member(n, "TimeStepZoneSec")?,
            "actual TimeStepZoneSec",
        )?;
        for (i, key) in [
            "TotPeople",
            "TotLights",
            "TotElecEquip",
            "TotGasEquip",
            "TotHWEquip",
            "TotStmEquip",
            "TotOthEquip",
            "TotITEquip",
            "TotBBHeat",
            "TotCO2Gen",
        ]
        .iter()
        .enumerate()
        {
            same(
                gains.totals[i],
                int(member(member(n, "counts")?, key)?)?,
                key,
            )?;
        }
        let expected = array(member(n, "equipment")?)?;
        let equipment_rows = a.other_equipment.as_deref().unwrap_or_default();
        same(
            equipment_rows.len(),
            expected.len(),
            "actual OtherEquipment shape",
        )?;
        let schedules = self
            .reports
            .selected_schedule_manager
            .as_ref()
            .ok_or("same actual schedules")?;
        for (row, n) in equipment_rows.iter().zip(expected) {
            same(
                row.name.as_str(),
                text(member(n, "Name")?)?,
                "actual OtherEquipment Name",
            )?;
            same(
                row.end_use_subcategory.as_str(),
                text(member(n, "EndUseSubcategory")?)?,
                "actual EndUseSubcategory",
            )?;
            for (key, value) in [
                ("ZonePtr", row.zone_ptr),
                ("spaceIndex", row.space_index),
                ("fuel", row.fuel as i32),
            ] {
                same(value, int(member(n, key)?)?, key)?;
            }
            same(
                row.ems_zone_equipment_override_on,
                boolean(member(n, "EMSZoneEquipOverrideOn")?)?,
                "actual EMSZoneEquipOverrideOn",
            )?;
            let f = member(n, "f64")?;
            for (key, value) in [
                ("DesignLevel", row.design_level),
                ("EMSEquipPower", row.ems_equipment_power),
                ("FractionLatent", row.fraction_latent),
                ("FractionRadiant", row.fraction_radiant),
                ("FractionLost", row.fraction_lost),
                ("FractionConvected", row.fraction_convected),
                ("CO2RateFactor", row.co2_rate_factor),
                ("NomMinDesignLevel", row.nominal_min_design_level),
                ("NomMaxDesignLevel", row.nominal_max_design_level),
            ] {
                bits(value, member(f, key)?, key)?;
            }
            for (key, value) in [
                "Power",
                "RadGainRate",
                "ConGainRate",
                "LatGainRate",
                "LostRate",
                "TotGainRate",
                "CO2GainRate",
            ]
            .iter()
            .zip(row.rates)
            {
                bits(value, member(f, key)?, key)?;
            }
            for (key, value) in [
                "Consumption",
                "RadGainEnergy",
                "ConGainEnergy",
                "LatGainEnergy",
                "LostEnergy",
                "TotGainEnergy",
            ]
            .iter()
            .zip(row.energies)
            {
                bits(value, member(f, key)?, key)?;
            }
            if let Some(index) = row.schedule {
                let schedule = schedules
                    .schedules
                    .get(index.0)
                    .ok_or("actual Schedule pointer target")?;
                address(member(n, "schedule_address")?)?;
                bits(
                    schedule.current_value,
                    member(n, "actual_schedule_current")?,
                    "actual pointed Schedule current value",
                )?;
            } else {
                same(
                    n["schedule_address"].as_u64(),
                    Some(0),
                    "actual null Schedule pointer",
                )?;
                same(
                    n.get("actual_schedule_current").is_none(),
                    true,
                    "unreached Schedule dereference",
                )?;
            }
        }
        for (key, rows) in [
            ("zone_reports", &a.zone_report),
            ("space_reports", &a.space_report),
        ] {
            let expected = array(member(n, key)?)?;
            let rows = rows.as_deref().unwrap_or_default();
            same(rows.len(), expected.len(), key)?;
            for (row, n) in rows.iter().zip(expected) {
                report(row, n)?;
            }
        }
        for (key, rows) in [
            ("zone_int_gain", &a.zone_int_gain),
            ("space_int_gain", &a.space_int_gain),
        ] {
            let expected = array(member(n, key)?)?;
            let rows = rows.as_deref().unwrap_or_default();
            same(rows.len(), expected.len(), key)?;
            for (row, n) in rows.iter().zip(expected) {
                bits(row[0], member(n, "NOFOCC")?, "actual NOFOCC")?;
                bits(row[1], member(n, "QLTSW")?, "actual QLTSW")?;
            }
        }
        for (key, values) in [
            (
                "zone_air_latent",
                self.air
                    .zones
                    .as_ref()
                    .map(|v| v.iter().map(|r| r.air.latent_gain).collect::<Vec<_>>()),
            ),
            (
                "space_air_latent",
                self.air
                    .spaces
                    .as_ref()
                    .map(|v| v.iter().map(|r| r.air.latent_gain).collect::<Vec<_>>()),
            ),
        ] {
            let expected = array(member(n, key)?)?;
            let values = values.unwrap_or_default();
            same(values.len(), expected.len(), key)?;
            for (v, n) in values.iter().zip(expected) {
                bits(*v, member(n, "latentGain")?, "actual AirHB latentGain")?;
            }
        }
        let expected = array(member(n, "zone_category_end_use")?)?;
        let rows = a.zone_electric_end_use.as_deref().unwrap_or_default();
        same(rows.len(), expected.len(), "actual ZoneIntEEuse shape")?;
        for (row, n) in rows.iter().zip(expected) {
            for (values, key) in row
                .iter()
                .zip(["EEConvected", "EERadiated", "EELost", "EELatent"])
            {
                let expected = member(n, key)?;
                same(expected["values_defined"].as_bool(), Some(true), key)?;
                same(count(member(expected, "size")?)?, values.len(), key)?;
                vector(values, member(expected, "values")?, key)?;
            }
        }
        let expected = array(member(n, "spaces")?)?;
        let spaces = a.space_devices.as_deref().unwrap_or_default();
        same(
            spaces.len(),
            expected.len(),
            "actual Space internal-gain device owners",
        )?;
        let bindings = gains.native_gap_device_bindings()?;
        same(
            bindings.len(),
            spaces.len(),
            "actual device binding owner shape",
        )?;
        for (i, (row, n)) in spaces.iter().zip(expected).enumerate() {
            same(count(member(n, "ordinal")?)?, i + 1, "actual Space ordinal")?;
            same(
                row.number_of_devices,
                count(member(n, "numberOfDevices")?)?,
                "actual active device count",
            )?;
            same(
                row.max_number_of_devices,
                count(member(n, "maxNumberOfDevices")?)?,
                "actual max device count",
            )?;
            let storage = member(n, "device_storage")?;
            same(
                storage["allocated"].as_bool(),
                Some(row.devices.is_some()),
                "actual device allocated shape",
            )?;
            same(
                count(member(storage, "size")?)?,
                row.devices.as_ref().map_or(0, Vec::len),
                "actual device storage full size",
            )?;
            same(
                int(member(storage, "lower")?)?,
                1,
                "actual device lower bound",
            )?;
            same(
                int(member(storage, "upper")?)?,
                i32::try_from(row.devices.as_ref().map_or(0, Vec::len))?,
                "actual device upper bound",
            )?;
            let expected = array(member(n, "devices")?)?;
            same(
                expected.len(),
                row.number_of_devices,
                "actual observed active device shape",
            )?;
            let devices = row.devices.as_deref().unwrap_or_default();
            if devices.len() < row.number_of_devices {
                return Err(invalid("actual active device bounds").into());
            }
            for (at, (device, n)) in devices[..row.number_of_devices]
                .iter()
                .zip(expected)
                .enumerate()
            {
                same(
                    device.component_name.as_str(),
                    text(member(n, "CompObjectName")?)?,
                    "actual device Name",
                )?;
                same(
                    device.component_type,
                    int(member(n, "CompType")?)?,
                    "actual device CompType",
                )?;
                same(
                    device.return_air_node,
                    int(member(n, "ReturnAirNodeNum")?)?,
                    "actual return node",
                )?;
                bits(
                    device.space_gain_fraction,
                    member(n, "spaceGainFrac")?,
                    "actual spaceGainFrac",
                )?;
                let pointers = member(n, "pointers")?;
                let zero = address(member(pointers, "PtrReturnAirConvGainRate")?)?;
                for (slot, (value_key, pointer_key)) in [
                    ("ConvectGainRate", "PtrConvectGainRate"),
                    ("ReturnAirConvGainRate", "PtrReturnAirConvGainRate"),
                    ("RadiantGainRate", "PtrRadiantGainRate"),
                    ("LatentGainRate", "PtrLatentGainRate"),
                    ("ReturnAirLatentGainRate", "PtrReturnAirLatentGainRate"),
                    ("CarbonDioxideGainRate", "PtrCarbonDioxideGainRate"),
                    ("GenericContamGainRate", "PtrGenericContamGainRate"),
                ]
                .iter()
                .enumerate()
                {
                    bits(
                        device.cached_rates[slot],
                        member(member(n, "values")?, value_key)?,
                        value_key,
                    )?;
                    let pointer = address(member(pointers, pointer_key)?)?;
                    match bindings[i][at][slot] {
                        Some(OrdinaryGainRateTarget::ZeroPointerValue) => {
                            same(pointer, zero, "actual shared Native zeroPointerVal alias")?
                        }
                        Some(OrdinaryGainRateTarget::OtherEquipment { row_index, field }) => {
                            let owner = equipment_rows
                                .get(row_index)
                                .ok_or("actual symbolic OtherEquipment target")?;
                            same(
                                owner.name.as_str(),
                                device.component_name.as_str(),
                                "actual symbolic device target owner",
                            )?;
                            same(
                                field,
                                match slot {
                                    0 => "ConGainRate",
                                    2 => "RadGainRate",
                                    3 => "LatGainRate",
                                    _ => {
                                        return Err(
                                            invalid("actual finite rate binding position").into()
                                        );
                                    }
                                },
                                "actual stored rate target field",
                            )?;
                            if pointer == zero {
                                return Err(invalid("Native actual equipment pointer unexpectedly aliases zero cell").into());
                            }
                        }
                        Some(OrdinaryGainRateTarget::People { .. }) => {
                            return Err(invalid(
                                "People pointer outside this OtherEquipment-only cohort",
                            )
                            .into());
                        }
                        None => return Err(invalid("actual missing stored device target").into()),
                    }
                }
            }
        }
        let r = &self.modules.refrigeration;
        let e = member(n, "refrigeration")?;
        for (key, value) in [
            ("GetRefrigerationInputFlag", r.get_input),
            ("ManageRefrigeration", r.manage_refrigeration),
            ("HaveCasesOrWalkins", r.have_cases_or_walk_ins),
            ("HaveChillers", r.have_chillers),
        ] {
            same(value, boolean(member(e, key)?)?, key)?;
        }
        for (key, value) in [
            ("NumRefrigSystems", r.counts.systems),
            ("NumTransRefrigSystems", r.counts.transcritical_systems),
            ("NumRefrigeratedRacks", r.counts.racks),
            ("NumSimulationCases", r.counts.cases),
            ("NumSimulationWalkIns", r.counts.walk_ins),
            ("NumSimulationRefrigAirChillers", r.counts.air_chillers),
        ] {
            same(value, int(member(e, key)?)?, key)?;
        }
        bool_array(
            &r.present_in_zone,
            member(e, "RefrigPresentInZone")?,
            "actual RefrigPresentInZone",
        )?;
        self.assert_diagnostics(member(n, "diagnostics")?)?;
        Ok(())
    }
}
