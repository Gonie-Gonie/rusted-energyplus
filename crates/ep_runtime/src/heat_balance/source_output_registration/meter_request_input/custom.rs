//! GetCustomMeterInput589-1358 genuine zero-object branch and exact direct map reads.
use super::{Error, SourceMeterReportingOwners};
use crate::heat_balance::selected_input_processor::{SelectedInputProcessor, SourceInputFamily};

fn custom_names(input: &SelectedInputProcessor, family: &str) -> Result<(i32, Vec<String>), Error> {
    let mut count: i32 = 0;
    let mut names = Vec::new();
    if let Some(SourceInputFamily::Objects(rows)) = input.find_exact_family(family) {
        // Actual epJSON object-map iteration is sorted by original UTF-8 key.
        let mut map_order: Vec<_> = rows.iter().collect();
        map_order.sort_by(|a, b| a.object_name.cmp(&b.object_name));
        for row in map_order {
            names.push(row.object_name.to_ascii_uppercase());
            count = count.checked_add(1).ok_or(Error::Unavailable(
                "defined native custom meter count increment",
            ))?;
        }
    }
    Ok((count, names))
}
pub(super) fn get_custom_meter_input(
    o: &mut SourceMeterReportingOwners<'_, '_>,
    errors_found: &mut bool,
) -> Result<(), Error> {
    let _names_of_keys: Option<Vec<String>> = None;
    let _indexes_for_key_variable: Option<Vec<i32>> = None;
    let _variables_on_source_meter: Option<Vec<i32>> = None;
    let big_errors_found = false;
    let (custom_count, _custom_meter_names) = custom_names(o.input, "Meter:Custom")?;
    let (decrement_count, _custom_decrement_meter_names) =
        custom_names(o.input, "Meter:CustomDecrement")?;
    o.shortcut.current_module_object = "Meter:Custom".into();
    if custom_count > 0 {
        return Err(Error::Unavailable(
            "actual Meter:Custom first shared getObjectItem",
        ));
    }
    o.shortcut.current_module_object = "Meter:CustomDecrement".into();
    if decrement_count > 0 {
        return Err(Error::Unavailable(
            "actual Meter:CustomDecrement first shared getObjectItem",
        ));
    }
    if big_errors_found {
        *errors_found = true;
    }
    Ok(())
}
