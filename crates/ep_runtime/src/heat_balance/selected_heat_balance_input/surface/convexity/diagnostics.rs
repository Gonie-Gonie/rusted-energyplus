//! Literal CheckConvexity diagnostics and genuine no-width .2T Area caller.
use super::{
    Error, SourceConvexityState, SourceErrorBackend, SurfaceGetterOwners, SurfaceTmpInput,
};

pub(super) fn fixed_8_3(value: f64) -> Result<String, Error> {
    if !value.is_finite() {
        return Err(Error::Unavailable(
            "Source fixed diagnostic nonfinite value",
        ));
    }
    Ok(format!("{value:8.3}"))
}
/// IOFiles.hh118-121,306-331. No R nudges or decimal-round substitution.
pub(super) fn trim_2(value: f64) -> Result<String, Error> {
    if !value.is_finite() {
        return Err(Error::Unavailable("Source .2T diagnostic nonfinite value"));
    }
    let threshold = f64::from_bits(0x3fb9_9999_9999_9999);
    if value >= threshold || value <= -threshold || value == 0.0 || value == -0.0 {
        let magnitude = 10.0_f64.powi(2);
        let adjusted = (value * magnitude) + 0.0001;
        let truncated = adjusted.trunc() / magnitude;
        if !truncated.is_finite() {
            return Err(Error::Unavailable("Source .2T fixed overflow"));
        }
        return Ok(format!("{truncated:.2}"));
    }
    let text = format!("{value:.4E}");
    let (mantissa, exponent) = text
        .split_once('E')
        .ok_or(Error::Unavailable("Source .2T exponent format"))?;
    let exponent: i32 = exponent
        .parse()
        .map_err(|_| Error::Unavailable("Source .2T exponent digits"))?;
    let mut text = format!("{mantissa}E{exponent:+03}");
    // Exact zero_pad_exponent string-position operation, before erasing digits.
    if text.len() > 3 && !text.as_bytes()[text.len() - 3].is_ascii_digit() {
        text.insert(text.len() - 2, '0');
    }
    if let Some(e) = text.find('E') {
        let start = e
            .checked_sub(2)
            .ok_or(Error::Unavailable("Source .2T truncation range"))?;
        text.replace_range(start..e, "");
    }
    Ok(text)
}
pub(super) fn nonconvex(
    scratch: &SourceConvexityState,
    row: &SurfaceTmpInput,
    n: usize,
    sides: usize,
    owners: &SurfaceGetterOwners<'_>,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), Error> {
    let index = usize::try_from(row.zone)
        .ok()
        .and_then(|n| n.checked_sub(1))
        .ok_or(Error::Unavailable("actual non-convex Zone index"))?;
    let zone = owners
        .zones
        .zones
        .as_ref()
        .and_then(|z| z.get(index))
        .ok_or(Error::Unavailable("actual non-convex Zone owner"))?;
    backend.show_warning_error(&format!(
        "CheckConvexity: Zone=\"{}\", Surface=\"{}\" is non-convex.",
        zone.name, row.name
    ))?;
    let mut np1 = n + 1;
    if np1 > sides {
        np1 -= sides;
    }
    let mut np2 = n + 2;
    if np2 > sides {
        np2 -= sides;
    }
    backend.show_continue_error(&format!(
        "...vertex {} to vertex {} to vertex {}",
        n, np1, np2
    ))?;
    for (label, ordinal) in [(n, n), (np1, n + 1), (np2, n + 2)] {
        backend.show_continue_error(&format!(
            "...vertex {}=[{},{},{}]",
            label,
            super::super::super::rounded(scratch.x(ordinal)?, 2)?,
            super::super::super::rounded(scratch.y(ordinal)?, 2)?,
            super::super::super::rounded(scratch.z(ordinal)?, 2)?
        ))?;
    }
    Ok(())
}
