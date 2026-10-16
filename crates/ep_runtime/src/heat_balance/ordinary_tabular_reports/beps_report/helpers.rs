//! Direct RealToStr18578, scalar subscript, and same predefined overload calls.
use super::{Error, SourceBepsReportOwners};
use crate::heat_balance::ctf_reports::formatting;
use crate::heat_balance::source_output_registration::{
    SourceEndUseCategory, SourceOutputRegistration,
};

pub(super) fn count(value: i32) -> Result<usize, Error> {
    usize::try_from(value)
        .map_err(|_| Error::Unavailable("actual BEPS allocation/count nonnegative domain"))
}
pub(super) fn category(
    o: &SourceOutputRegistration,
    ordinal: usize,
) -> Result<&SourceEndUseCategory, Error> {
    o.end_use_categories
        .as_ref()
        .and_then(|rows| rows.get(ordinal.wrapping_sub(1)))
        .ok_or(Error::Unavailable(
            "same actual BEPS EndUseCategory ordinal",
        ))
}
pub(super) fn subcategory(
    o: &SourceOutputRegistration,
    end_use: usize,
    ordinal: usize,
    by_space: bool,
) -> Result<Vec<u8>, Error> {
    let row = category(o, end_use)?;
    let names = if by_space {
        &row.space_type_names
    } else {
        &row.subcategory_names
    };
    Ok(names
        .as_ref()
        .and_then(|rows| rows.get(ordinal.wrapping_sub(1)))
        .ok_or(Error::Unavailable(
            "same actual BEPS subcategory/space-type name ordinal",
        ))?
        .as_bytes()
        .to_vec())
}
pub(super) fn real(format_reals: bool, value: f64, digits: i32) -> Result<Vec<u8>, Error> {
    if !format_reals {
        return Err(Error::Unavailable(
            "actual RealToStr18599 unrounded fmt shortest-decimal operation",
        ));
    }
    let mut n = digits;
    if value < 0.0 {
        n = n.checked_sub(1).ok_or(Error::Unavailable(
            "actual RealToStr signed digit decrement",
        ))?;
    }
    if n > 9 {
        n = 9;
    }
    if n < 0 {
        n = 0;
    }
    const MAX: [f64; 10] = [
        9999999999.0,
        999999999.0,
        99999999.0,
        9999999.0,
        999999.0,
        99999.0,
        9999.0,
        999.0,
        99.0,
        9.0,
    ];
    if value.abs() > MAX[n as usize] {
        // Actual E formatting has no R/T nudge. Keep its signed two-digit
        // minimum exponent and right alignment; no decimal rescaling.
        if !value.is_finite() {
            return Err(Error::Unavailable(
                "actual RealToStr nonfinite exponential format domain",
            ));
        }
        let raw = format!("{value:.6E}");
        let (mantissa, exponent) = raw.split_once('E').ok_or(Error::Unavailable(
            "actual RealToStr exponential representation",
        ))?;
        let exponent: i32 = exponent
            .parse()
            .map_err(|_| Error::Unavailable("actual RealToStr exponent representation"))?;
        return Ok(format!("{:>12}", format!("{mantissa}E{exponent:+03}")).into_bytes());
    }
    if !value.is_finite() {
        return Err(Error::Unavailable(
            "actual RealToStr nonfinite fixed format domain",
        ));
    }
    if n == 0 {
        let mut text = formatting::fixed(value, 0, 0);
        text.push('.');
        Ok(format!("{text:>11}").into_bytes())
    } else {
        Ok(formatting::fixed(value, 12, n as usize).into_bytes())
    }
}
pub(super) fn column(o: &SourceBepsReportOwners<'_, '_>, name: &str) -> Result<i32, Error> {
    o.registry
        .named_indices
        .get(name)
        .copied()
        .ok_or(Error::Unavailable(
            "actual BEPS retained Predefined named index",
        ))
}
pub(super) fn predef_real(
    o: &mut SourceBepsReportOwners<'_, '_>,
    name: &str,
    object: &[u8],
    value: f64,
    precision: i32,
) -> Result<(), Error> {
    let index = column(o, name)?;
    predef_real_id(o, index, object, value, precision)
}
pub(super) fn predef_real_id(
    o: &mut SourceBepsReportOwners<'_, '_>,
    index: i32,
    object: &[u8],
    value: f64,
    precision: i32,
) -> Result<(), Error> {
    let object = std::str::from_utf8(object)
        .map_err(|_| Error::Unavailable("actual Predefined object nonUTF8 string domain"))?;
    match precision {
        2 => o.table.fixed_real_entry_2(index, object, value)?,
        3 => o.table.fixed_real_entry_3(index, object, value)?,
        _ => {
            return Err(Error::Unavailable(
                "actual BEPS selected PreDefTableEntry precision",
            ));
        }
    }
    Ok(())
}
pub(super) fn predef_bytes(
    o: &mut SourceBepsReportOwners<'_, '_>,
    name: &str,
    object: &[u8],
    value: &[u8],
) -> Result<(), Error> {
    let object = std::str::from_utf8(object)
        .map_err(|_| Error::Unavailable("actual Predefined object nonUTF8 string domain"))?;
    let index = column(o, name)?;
    o.table.raw_byte_entry(index, object, value)?;
    Ok(())
}
