//! Actual GetZoneAirSetPoints with genuine finite positive Thermostat input.
use super::*;
use std::io::Write;
mod current;
mod thermostat;
impl SourceZoneAirPredictorCorrector {
    pub(crate) fn get_source_zone_air_setpoints(
        &mut self,
        owners: &mut SourceZoneAirOwners<'_, '_>,
        eio: &mut dyn Write,
    ) -> Result<(), Error> {
        let errors_found = false; // Actual GetZoneAirSetPoints local ctor274.
        // Actual getter-local arrays remain live through this function.
        let mut thermostat_locals = thermostat::SourceThermostatInputLocals::default();
        owners.shortcut.current_module_object = "ZoneControl:Thermostat".into();
        self.controls.number_thermostat_statements = owners
            .input
            .get_num_objects_found(&owners.shortcut.current_module_object);
        if self.controls.thermostat_objects.is_some() {
            return Err(Error::Unavailable(
                "repeated native TStatObjects allocation327",
            ));
        }
        self.controls.thermostat_objects = Some(vec![
            controls::SourceTStatObject::default();
            self.controls.number_thermostat_statements
        ]);
        self.controls.number_temperature_zones = 0;
        self.get_source_thermostat_objects(owners, &mut thermostat_locals)?;
        for (kind, family, line) in [
            (1, "THERMOSTATSETPOINT:SINGLEHEATING", 525),
            (2, "THERMOSTATSETPOINT:SINGLECOOLING", 561),
            (3, "ThermostatSetpoint:SingleHeatingOrCooling", 597),
            (4, "ThermostatSetpoint:DualSetpoint", 633),
        ] {
            owners.shortcut.current_module_object = family.into();
            self.flags.number_temperature_controls[kind] =
                owners.input.get_num_objects_found(family);
            if self.flags.number_temperature_controls[kind] > 0 {
                i32::try_from(self.flags.number_temperature_controls[kind]).map_err(|_| {
                    Error::Unavailable(
                        "actual positive thermostat allocation native int count domain",
                    )
                })?;
                self.flags.temperature_setpoint_schedules[kind] = Some(vec![
                    controls::SourceZoneSetpointSchedule::default();
                    self.flags.number_temperature_controls[kind]
                ]);
                if kind == 4 {
                    self.get_source_dual_setpoint_schedules(owners)?;
                } else {
                    return Err(Error::Unavailable(match line {
                        525 => "actual positive singleHeat first getObjectItem529",
                        561 => "actual positive singleCool first getObjectItem565",
                        _ => "actual positive singleHeatCool first getObjectItem601",
                    }));
                }
            }
            // Native false count leaves the real Array1D unallocated; no zero resize.
        }
        self.link_source_thermostat_schedules(owners, &mut thermostat_locals)?;
        owners.shortcut.current_module_object = "ZoneControl:Humidistat".into();
        self.controls.number_humidity_zones = i32::try_from(
            owners
                .input
                .get_num_objects_found(&owners.shortcut.current_module_object),
        )
        .map_err(|_| Error::Unavailable("Source humidity object count int811"))?;
        if self.controls.number_humidity_zones > 0 {
            return Err(Error::Unavailable(
                "positive HumidityControlZone allocation814",
            ));
        }
        owners.shortcut.current_module_object = "ZoneControl:Thermostat:ThermalComfort".into();
        self.controls.number_comfort_statements = owners
            .input
            .get_num_objects_found(&owners.shortcut.current_module_object);
        if self.controls.comfort_thermostat_objects.is_some() {
            return Err(Error::Unavailable(
                "repeated native ComfortTStatObjects allocation877",
            ));
        }
        self.controls.comfort_thermostat_objects = Some(vec![
            controls::SourceTStatObject::default();
            self.controls
                .number_comfort_statements
        ]);
        self.controls.number_comfort_zones = 0;
        if self.controls.number_comfort_statements > 0 {
            return Err(Error::Unavailable(
                "positive comfort thermostat getObjectItem883",
            ));
        }
        for (kind, family, line) in [
            (
                1,
                "ThermostatSetpoint:ThermalComfort:Fanger:SingleHeating",
                1170,
            ),
            (
                2,
                "ThermostatSetpoint:ThermalComfort:Fanger:SingleCooling",
                1208,
            ),
            (
                3,
                "ThermostatSetpoint:ThermalComfort:Fanger:SingleHeatingOrCooling",
                1247,
            ),
            (
                4,
                "ThermostatSetpoint:ThermalComfort:Fanger:DualSetpoint",
                1286,
            ),
        ] {
            owners.shortcut.current_module_object = family.into();
            self.flags.number_comfort_controls[kind] = owners.input.get_num_objects_found(family);
            if self.flags.number_comfort_controls[kind] > 0 {
                return Err(Error::Unavailable(match line {
                    1170 => "positive comfort singleHeat schedule allocation1170",
                    1208 => "positive comfort singleCool schedule allocation1208",
                    1247 => "positive comfort singleHeatCool schedule allocation1247",
                    _ => "positive comfort dual schedule allocation1286",
                }));
            }
        }
        self.subsystems
            .hybrid_model
            .get_source_hybrid_model_zone(owners.input)?;
        owners.shortcut.current_module_object = "ZoneCapacitanceMultiplier:ResearchSpecial".into();
        let count = owners
            .input
            .get_num_objects_found(&owners.shortcut.current_module_object);
        if count == 0 {
            for row in owners
                .zones
                .zones
                .as_mut()
                .ok_or(Error::Unavailable("actual Zone array capacitance loop1463"))?
            {
                row.sensible_capacity_multiplier = 1.0;
                row.moisture_capacity_multiplier = 1.0;
                row.carbon_dioxide_capacity_multiplier = 1.0;
                row.generic_contaminant_capacity_multiplier = 1.0;
            }
        } else {
            return Err(Error::Unavailable(
                "positive ZoneCapacitanceMultiplier getObjectItem1476",
            ));
        }
        eio.write_all(b"! <Zone Volume Capacitance Multiplier>, Sensible Heat Capacity Multiplier, Moisture Capacity Multiplier, Carbon Dioxide Capacity Multiplier, Generic Contaminant Capacity Multiplier\n")?;
        // Exact four genuine local defaults and native width8,precision3 F layout.
        writeln!(
            eio,
            "Zone Volume Capacitance Multiplier,{:8.3} ,{:8.3},{:8.3},{:8.3}",
            1.0, 1.0, 1.0, 1.0
        )?;
        owners.shortcut.current_module_object =
            "ZoneControl:Thermostat:OperativeTemperature".into();
        self.controls.number_operative_zones = owners
            .input
            .get_num_objects_found(&owners.shortcut.current_module_object);
        if self.controls.number_operative_zones > 0 {
            self.controls.any_operative_control = true;
            return Err(Error::Unavailable(
                "positive operative thermostat getObjectItem1575",
            ));
        }
        owners.shortcut.current_module_object =
            "ZoneControl:Thermostat:TemperatureAndHumidity".into();
        self.controls.number_temperature_humidity_zones = owners
            .input
            .get_num_objects_found(&owners.shortcut.current_module_object);
        if self.controls.number_temperature_humidity_zones > 0 {
            self.controls.any_temperature_humidity_control = true;
            return Err(Error::Unavailable(
                "positive temperature/humidity thermostat1765",
            ));
        }
        owners.shortcut.current_module_object = "ZoneControl:Thermostat:StagedDualSetpoint".into();
        let stages = owners
            .input
            .get_num_objects_found(&owners.shortcut.current_module_object);
        if stages > 0 {
            return Err(Error::Unavailable(
                "positive StagedTStatObjects allocation1923",
            ));
        }
        self.flags.number_stage_zones = 0;
        // The valid finite path retains the actual false local constructor;
        // unported diagnostic branches stop before their first message/write.
        if errors_found {
            return match owners.backend.show_fatal_error(
                "Errors getting Zone Control input data.  Preceding condition(s) cause termination.",
            ) {
                Err(error) => Err(Error::Fatal(error)),
                Ok(never) => match never {},
            };
        }
        Ok(())
    }
    pub(crate) fn calculate_source_air_temperature_setpoints(
        &mut self,
        owners: &mut SourceZoneAirOwners<'_, '_>,
    ) -> Result<(), Error> {
        owners
            .fan
            .thermostat
            .temp_control_type
            .as_mut()
            .ok_or(Error::Unavailable("actual TempControlType default3286"))?
            .fill(0);
        let count = usize::try_from(owners.globals.num_of_zones)
            .map_err(|_| Error::Unavailable("actual NumOfZones occupied-setpoint3290"))?;
        if self.controls.occupied_heating_setpoints.is_none() {
            self.controls.occupied_heating_setpoints = Some(vec![None; count]);
        }
        if self.controls.occupied_cooling_setpoints.is_none() {
            self.controls.occupied_cooling_setpoints = Some(vec![None; count]);
        }
        self.controls
            .occupied_heating_setpoints
            .as_mut()
            .ok_or(Error::Unavailable("actual occupied heat3295"))?
            .fill(Some(0.0));
        self.controls
            .occupied_cooling_setpoints
            .as_mut()
            .ok_or(Error::Unavailable("actual occupied cool3296"))?
            .fill(Some(100.0));
        self.calculate_source_current_temperature_controls(owners)?;
        if self.controls.number_comfort_zones > 0 {
            return Err(Error::Unavailable(
                "positive CalcZoneAirComfortSetPoints3456",
            ));
        }
        self.override_source_air_setpoints_for_ems(owners)?;
        Ok(())
    }
}
