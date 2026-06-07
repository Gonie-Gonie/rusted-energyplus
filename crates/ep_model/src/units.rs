//! Unit-bearing scalar wrappers and geometry primitives.

/// Numeric field that may be set to EnergyPlus Autocalculate.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AutoOrNumber {
    /// EnergyPlus should calculate the value from model geometry.
    AutoCalculate,
    /// User-specified numeric value.
    Value(f64),
}

/// Numeric field that may be set to EnergyPlus Autosize.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AutosizeOrNumber {
    /// EnergyPlus should autosize the value.
    Autosize,
    /// User-specified numeric value.
    Value(f64),
}

/// Three-dimensional point in meters.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point3 {
    /// X coordinate.
    pub x_m: f64,
    /// Y coordinate.
    pub y_m: f64,
    /// Z coordinate.
    pub z_m: f64,
}
