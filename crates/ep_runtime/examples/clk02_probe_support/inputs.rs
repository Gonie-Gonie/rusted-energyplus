//! Validate actual input types, canary bits and supplied file identities.

use super::{Result, digest};
use ep_runtime::weather::raw::RawEpwOutputs;
use serde_json::Value;
use std::path::{Path, PathBuf};

pub(super) fn require(condition: bool, message: &str) -> Result<()> {
    if condition {
        Ok(())
    } else {
        Err(message.to_owned().into())
    }
}

pub(super) fn text(value: &Value) -> Result<&str> {
    value
        .as_str()
        .ok_or_else(|| "typed string input required".into())
}

pub(super) fn integer(value: &Value) -> Result<i32> {
    Ok(i32::try_from(
        value.as_i64().ok_or("typed integer input required")?,
    )?)
}

pub(super) fn flag(value: &Value) -> Result<bool> {
    value
        .as_bool()
        .ok_or_else(|| "typed boolean input required".into())
}

pub(super) fn number(value: &Value) -> Result<f64> {
    let value = value.as_f64().ok_or("typed real input required")?;
    require(value.is_finite(), "finite real input required")?;
    Ok(value)
}

pub(super) fn array(value: &Value, size: usize) -> Result<&[Value]> {
    let values = value.as_array().ok_or("typed array input required")?;
    require(
        values.len() == size,
        "declared input array cardinality differs",
    )?;
    Ok(values)
}

pub(super) fn int_array<const N: usize>(value: &Value) -> Result<[i32; N]> {
    let mut values = [0; N];
    for (target, input) in values.iter_mut().zip(array(value, N)?) {
        *target = integer(input)?;
    }
    Ok(values)
}

fn bit_reals<const N: usize>(numbers: &Value, bits: &Value) -> Result<[f64; N]> {
    let mut values = [0.0; N];
    for ((target, number_input), bits_input) in values
        .iter_mut()
        .zip(array(numbers, N)?)
        .zip(array(bits, N)?)
    {
        let token = text(bits_input)?;
        require(token.len() == 16, "binary64 input token length differs")?;
        let bits = u64::from_str_radix(token, 16)?;
        require(
            number(number_input)?.to_bits() == bits,
            "input canary value/bit identity differs",
        )?;
        *target = f64::from_bits(bits);
    }
    Ok(values)
}

pub(super) fn outputs(input: &Value) -> Result<RawEpwOutputs> {
    Ok(RawEpwOutputs {
        error_found: flag(&input["ErrorFound"])?,
        dates: int_array(&input["dates"])?,
        mandatory_reals: bit_reals(&input["mandatory_reals"], &input["mandatory_real_bits"])?,
        observation_indicator: integer(&input["WObs"])?,
        weather_codes: int_array(&input["weather_codes"])?,
        optional_reals: bit_reals(&input["optional_reals"], &input["optional_real_bits"])?,
    })
}

pub(super) fn file(root: &Path, binding: &Value) -> Result<(PathBuf, Vec<u8>)> {
    let path = root.join(text(&binding["path"])?).canonicalize()?;
    require(
        path.starts_with(root),
        "input file must be inside repository",
    )?;
    let bytes = std::fs::read(&path)?;
    require(
        digest::sha256(&bytes) == text(&binding["sha256"])?,
        "actual input file hash differs",
    )?;
    Ok((path, bytes))
}

pub(super) fn line(input: &Value, text_key: &str, hash_key: &str) -> Result<String> {
    let line = text(&input[text_key])?;
    require(
        digest::sha256(line.as_bytes()) == text(&input[hash_key])?,
        "actual input line hash differs",
    )?;
    Ok(line.to_owned())
}
