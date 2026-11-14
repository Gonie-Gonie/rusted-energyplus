//! Genuine bounded GetZoneAirSetPoints humidity-input parent, Native811-871.
use super::super::controls::SourceHumidityControlledZone;
use super::*;
use crate::heat_balance::selected_input_processor::SourceInputStatisticsValue as V;

const FAMILY: &str = "ZoneControl:Humidistat";
const ALPHAS: [&str; 4] = [
    "name",
    "zone_name",
    "humidifying_relative_humidity_setpoint_schedule_name",
    "dehumidifying_relative_humidity_setpoint_schedule_name",
];

fn get_object(owners: &mut SourceZoneAirOwners<'_, '_>, number: usize) -> Result<usize, Error> {
    // The admitted producer starts with original direct epJSON. Staged IDF
    // counts/order require a genuine separate overlay before that lane opens.
    if !*owners.source_is_epjson {
        return Err(Error::Unavailable(
            "actual Humidistat IDF declaration/count metadata",
        ));
    }
    let object = owners
        .input
        .source_raw_object_names(FAMILY)
        .map_err(|_| Error::Unavailable("actual Humidistat retained source order"))?
        .get(number - 1)
        .cloned()
        .ok_or(Error::Unavailable(
            "actual Humidistat getObjectItem ordinal",
        ))?;
    if owners.shortcut.alpha_args.len() < 4
        || owners.shortcut.alpha_field_names.len() < 4
        || owners.shortcut.alpha_field_blanks.len() < 4
    {
        return Err(Error::Unavailable(
            "actual Humidistat shared alpha allocation",
        ));
    }
    // Native1011-1029 resets every shared buffer, then erases unusedInputs,
    // before producing field values. The Source raw owner is not rewritten.
    owners.shortcut.alpha_args.fill(String::new());
    owners.shortcut.alpha_field_names.fill(String::new());
    owners.shortcut.alpha_field_blanks.fill(true);
    owners.shortcut.numeric_args.fill(0.0);
    owners.shortcut.numeric_field_names.fill(String::new());
    owners.shortcut.numeric_field_blanks.fill(true);
    owners.input.mark_object_as_used(FAMILY, &object);
    let fields = owners
        .input
        .source_raw_object_fields(FAMILY, &object)
        .map_err(|_| Error::Unavailable("actual Humidistat raw fields"))?;
    let mut maximum = 3; // Genuine schema min_fields3, all fields alpha.
    for (at, &field) in ALPHAS.iter().enumerate() {
        owners.shortcut.alpha_field_names[at] = field.into();
        if at == 0 {
            owners.shortcut.alpha_args[at] = object.to_ascii_uppercase();
            owners.shortcut.alpha_field_blanks[at] = object.is_empty();
        } else if let Some(value) = fields.get(field) {
            maximum = maximum.max(at + 1);
            let V::String(value) = value else {
                return Err(Error::Unavailable("actual Humidistat alpha string getter"));
            };
            owners.shortcut.alpha_args[at] = value.to_ascii_uppercase();
            owners.shortcut.alpha_field_blanks[at] = value.is_empty();
        }
    }
    Ok(maximum)
}

impl SourceZoneAirPredictorCorrector {
    pub(super) fn get_source_humidity_controls(
        &mut self,
        owners: &mut SourceZoneAirOwners<'_, '_>,
    ) -> Result<(), Error> {
        if self.controls.number_humidity_zones <= 0 {
            return Ok(()); // Native zero leaves the real array unallocated.
        }
        let count = usize::try_from(self.controls.number_humidity_zones)
            .map_err(|_| Error::Unavailable("actual HumidityControlZone count814"))?;
        if self.controls.humidity_controlled_zones.is_some() {
            return Err(Error::Unavailable(
                "repeated actual HumidityControlZone allocation814",
            ));
        }
        self.controls.humidity_controlled_zones =
            Some(vec![SourceHumidityControlledZone::default(); count]);
        for number in 1..=count {
            let num_alphas = get_object(owners, number)?;
            let row = self
                .controls
                .humidity_controlled_zones
                .as_mut()
                .and_then(|rows| rows.get_mut(number - 1))
                .ok_or(Error::Unavailable("actual HumidityControlZone row834"))?;
            row.control_name = owners.shortcut.alpha_args[0].clone();
            // Actual module uniqueness ownership, not an advisory typed name map.
            if !self
                .controls
                .humidity_control_zone_unique_names
                .insert(owners.shortcut.alpha_args[1].clone())
            {
                return Err(Error::Unavailable(
                    "actual Humidistat duplicate-Zone severe child836",
                ));
            }
            row.zone_name = owners.shortcut.alpha_args[1].clone();
            row.actual_zone_number = match owners
                .zones
                .zones
                .as_ref()
                .ok_or(Error::Unavailable("actual Humidistat Zone array842"))?
                .iter()
                .position(|zone| zone.name == row.zone_name)
            {
                Some(at) => i32::try_from(at + 1)
                    .map_err(|_| Error::Unavailable("actual Humidistat Zone ordinal"))?,
                None => 0,
            };
            if row.actual_zone_number == 0 {
                return Err(Error::Unavailable(
                    "actual Humidistat missing-Zone severe child844",
                ));
            }
            owners
                .zones
                .zones
                .as_mut()
                .and_then(|zones| zones.get_mut(index(row.actual_zone_number).ok()?))
                .ok_or(Error::Unavailable("actual Humidistat same Zone writer853"))?
                .humidity_control_zone_index = i32::try_from(number)
                .map_err(|_| Error::Unavailable("actual humidity-control ordinal853"))?;
            if owners.shortcut.alpha_field_blanks[2] {
                return Err(Error::Unavailable(
                    "actual Humidistat empty humidifying schedule severe856",
                ));
            }
            row.humidifying_schedule = owners
                .schedules
                .get_source_schedule(&owners.shortcut.alpha_args[2])?;
            if row.humidifying_schedule.is_none() {
                return Err(Error::Unavailable(
                    "actual Humidistat missing humidifying schedule severe859",
                ));
            }
            if num_alphas < 4 || owners.shortcut.alpha_field_blanks[3] {
                row.dehumidifying_schedule = row.humidifying_schedule;
            } else {
                row.dehumidifying_schedule = owners
                    .schedules
                    .get_source_schedule(&owners.shortcut.alpha_args[3])?;
                if row.dehumidifying_schedule.is_none() {
                    return Err(Error::Unavailable(
                        "actual Humidistat missing dehumidifying schedule severe867",
                    ));
                }
            }
        }
        Ok(())
    }
}
