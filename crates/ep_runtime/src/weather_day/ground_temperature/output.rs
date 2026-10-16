//! Literal BaseGroundTemperatureModel.hh report header, join and fixed6.2.
use super::*;
pub(super) fn write_ground_temperatures(
    eio: &mut dyn Write,
    name: &str,
    values: &[f64; 12],
) -> Result<(), Error> {
    writeln!(
        eio,
        "! <Site:GroundTemperature:{name}>,Jan{{C}},Feb{{C}},Mar{{C}},Apr{{C}},May{{C}},Jun{{C}},Jul{{C}},Aug{{C}},Sep{{C}},Oct{{C}},Nov{{C}},Dec{{C}}"
    )?;
    let mut formatted = Vec::with_capacity(12);
    for value in values {
        if !value.is_finite() {
            return Err(Error::Format(
                "actual ground-temperature fixed6.2 nonfinite domain".into(),
            ));
        }
        formatted.push(format!("{value:6.2}"));
    }
    writeln!(
        eio,
        " Site:GroundTemperature:{name}, {}",
        formatted.join(", ")
    )?;
    Ok(())
}
