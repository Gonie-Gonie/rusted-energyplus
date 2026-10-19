//! Same actual array/record reads and report primitives; no result cache.
use super::{Error, SourceTabularBody, SourceVeriSumOwners};
use crate::heat_balance::selected_heat_balance_input::zone::{
    SelectedZoneData, SelectedZoneInputOwners,
};
pub(super) fn count(v: i32) -> Result<usize, Error> {
    usize::try_from(v).map_err(|_| Error::Unavailable("actual VeriSum array extent defined domain"))
}
pub(super) fn plus(v: i32, n: i32) -> Result<usize, Error> {
    count(
        v.checked_add(n)
            .ok_or(Error::Unavailable("actual VeriSum signed extent addition"))?,
    )
}
pub(super) fn index(v: i32) -> Result<usize, Error> {
    count(v.checked_sub(1).ok_or(Error::Unavailable(
        "actual VeriSum signed ordinal subtraction",
    ))?)
}
pub(super) fn label(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut s = a.to_vec();
    s.extend_from_slice(b);
    s
}
pub(super) fn people_label(unit: &[u8]) -> Result<Vec<u8>, Error> {
    let (last, prefix) = unit.split_last().ok_or(Error::Unavailable(
        "actual VeriSum unitName substr/last byte",
    ))?;
    let mut s = label(b"People ", prefix);
    s.extend_from_slice(b" per person");
    s.push(*last);
    Ok(s)
}
pub(super) fn real(formatted: bool, value: f64, digits: i32) -> Result<Vec<u8>, Error> {
    super::super::beps_report::source_report_real(formatted, value, digits)
}
pub(super) fn safe_divide(a: f64, b: f64) -> f64 {
    if b.abs() >= 1.0e-10 {
        a / b
    } else {
        a / if b >= 0.0 { 1.0e-10 } else { -1.0e-10 }
    }
}
// Keep the explicit owner-borrow relationship in this Native-facing helper signature.
#[allow(clippy::needless_lifetimes)]
pub(super) fn zone<'a>(
    zones: &'a SelectedZoneInputOwners,
    number: i32,
) -> Result<&'a SelectedZoneData, Error> {
    let at = number
        .checked_sub(1)
        .and_then(|n| usize::try_from(n).ok())
        .ok_or(Error::Unavailable("actual VeriSum Zone ordinal"))?;
    zones
        .zones
        .as_ref()
        .and_then(|rows| rows.get(at))
        .ok_or(Error::Unavailable("actual VeriSum Zone row"))
}
pub(super) fn mult(zone: &SelectedZoneData) -> Result<f64, Error> {
    Ok(f64::from(
        zone.multiplier
            .checked_mul(zone.list_multiplier)
            .ok_or(Error::Unavailable(
                "actual VeriSum Native signed multiplier product",
            ))?,
    ))
}
pub(super) struct Table {
    pub(super) rows: Vec<Vec<u8>>,
    pub(super) cols: Vec<Vec<u8>>,
    pub(super) widths: Vec<i32>,
    pub(super) body: SourceTabularBody,
}
impl Table {
    pub(super) fn new(cols: usize, rows: usize) -> Result<Self, Error> {
        let rows = vec![Vec::new(); rows];
        let columns = vec![Vec::new(); cols];
        let widths = vec![14; cols];
        let body = SourceTabularBody::allocate(cols, rows.len())?;
        Ok(Self {
            rows,
            cols: columns,
            widths,
            body,
        })
    }
}
pub(super) fn predef_real(
    registry: &crate::heat_balance::predefined_tables::PredefinedRegistry,
    table: &mut crate::heat_balance::ctf_reports::PredefinedStringTable,
    column: &str,
    object: &str,
    value: f64,
) -> Result<(), Error> {
    let id = *registry
        .named_indices
        .get(column)
        .ok_or(Error::Unavailable("actual VeriSum Predefined named column"))?;
    table.fixed_real_entry_2(id, object, value)?;
    Ok(())
}
pub(super) fn predef_zero(
    registry: &crate::heat_balance::predefined_tables::PredefinedRegistry,
    table: &mut crate::heat_balance::ctf_reports::PredefinedStringTable,
    column: &str,
    object: &str,
) -> Result<(), Error> {
    let id = *registry
        .named_indices
        .get(column)
        .ok_or(Error::Unavailable("actual VeriSum Predefined named column"))?;
    table.raw_byte_entry(id, object, b"0.00")?;
    Ok(())
}
// These helpers read only the field at the corresponding Native operation.
// A positive unrepresented family fails on its actual first row operand.
pub(super) fn gain_zone(
    o: &SourceVeriSumOwners<'_, '_>,
    family: usize,
    ordinal: i32,
) -> Result<i32, Error> {
    let at = count(ordinal - 1)?;
    match family {
        0 => Ok(o
            .gains
            .allocations
            .people
            .rows
            .get(at)
            .ok_or(Error::Unavailable("actual VeriSum People.ZonePtr row"))?
            .zone_ptr),
        6 => Ok(o
            .gains
            .allocations
            .other_equipment
            .as_ref()
            .and_then(|r| r.get(at))
            .ok_or(Error::Unavailable("actual VeriSum ZoneOtherEq.ZonePtr row"))?
            .zone_ptr),
        _ => Err(Error::Unavailable(
            "actual VeriSum positive unrepresented gain ZonePtr row read",
        )),
    }
}
pub(super) fn gain_space(
    o: &SourceVeriSumOwners<'_, '_>,
    family: usize,
    ordinal: i32,
) -> Result<i32, Error> {
    let at = count(ordinal - 1)?;
    match family {
        0 => Ok(o
            .gains
            .allocations
            .people
            .rows
            .get(at)
            .ok_or(Error::Unavailable("actual VeriSum People.spaceIndex row"))?
            .space_index),
        6 => Ok(o
            .gains
            .allocations
            .other_equipment
            .as_ref()
            .and_then(|r| r.get(at))
            .ok_or(Error::Unavailable(
                "actual VeriSum ZoneOtherEq.spaceIndex row",
            ))?
            .space_index),
        _ => Err(Error::Unavailable(
            "actual VeriSum positive unrepresented gain spaceIndex row read",
        )),
    }
}
pub(super) fn gain_value(
    o: &SourceVeriSumOwners<'_, '_>,
    family: usize,
    ordinal: i32,
) -> Result<f64, Error> {
    let at = count(ordinal - 1)?;
    match family {
        0 => Ok(o
            .gains
            .allocations
            .people
            .rows
            .get(at)
            .ok_or(Error::Unavailable(
                "actual VeriSum People.NumberOfPeople row",
            ))?
            .number_of_people),
        6 => Ok(o
            .gains
            .allocations
            .other_equipment
            .as_ref()
            .and_then(|r| r.get(at))
            .ok_or(Error::Unavailable(
                "actual VeriSum ZoneOtherEq.DesignLevel row",
            ))?
            .design_level),
        _ => Err(Error::Unavailable(
            "actual VeriSum positive unrepresented gain DesignLevel/DesignTotalPower row read",
        )),
    }
}
