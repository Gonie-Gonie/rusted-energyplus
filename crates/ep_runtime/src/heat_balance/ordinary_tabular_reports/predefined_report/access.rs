//! Checked actual one-based accesses; never substitutes an absent row/cell.
use super::Error;
use crate::heat_balance::ctf_reports::{PredefinedStringTable, PredefinedTableEntry};
pub(super) fn ordinal(number: i32) -> Result<usize, Error> {
    number
        .checked_sub(1)
        .and_then(|n| usize::try_from(n).ok())
        .ok_or(Error::Unavailable(
            "actual Predefined one-based array index domain",
        ))
}
pub(super) fn increment(value: i32, at: &'static str) -> Result<i32, Error> {
    value.checked_add(1).ok_or(Error::Unavailable(at))
}
pub(super) fn row<'a, T>(
    values: &'a Option<Vec<T>>,
    number: i32,
    at: &'static str,
) -> Result<&'a T, Error> {
    let index = ordinal(number)?;
    values
        .as_ref()
        .and_then(|v| v.get(index))
        .ok_or(Error::Unavailable(at))
}
pub(super) fn entry(
    table: &PredefinedStringTable,
    number: i32,
) -> Result<&PredefinedTableEntry, Error> {
    row(
        &table.entries,
        number,
        "same actual Predefined tableEntry row",
    )
}
pub(super) fn entry_mut(
    table: &mut PredefinedStringTable,
    number: i32,
) -> Result<&mut PredefinedTableEntry, Error> {
    let at = ordinal(number)?;
    table
        .entries
        .as_mut()
        .and_then(|v| v.get_mut(at))
        .ok_or(Error::Unavailable(
            "same actual Predefined mutable tableEntry row",
        ))
}
pub(super) fn written<T: Copy>(
    values: &[Option<T>],
    number: i32,
    at: &'static str,
) -> Result<T, Error> {
    values
        .get(ordinal(number)?)
        .copied()
        .flatten()
        .ok_or(Error::Unavailable(at))
}
