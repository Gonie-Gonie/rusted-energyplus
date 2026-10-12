//! Actual consumed CurveManager input-state slice; Cost/Fault remain held.
use super::selected_input_processor::SelectedInputProcessor;
use std::collections::BTreeMap;
const CURVE_FAMILIES: [&str; 22] = [
    "Curve:Biquadratic",
    "Curve:Cubic",
    "Curve:Quartic",
    "Curve:Quadratic",
    "Curve:QuadLinear",
    "Curve:QuintLinear",
    "Curve:QuadraticLinear",
    "Curve:CubicLinear",
    "Curve:Linear",
    "Curve:Bicubic",
    "Curve:Triquadratic",
    "Curve:Exponent",
    "Table:Lookup",
    "Curve:FanPressureRise",
    "Curve:ExponentialSkewNormal",
    "Curve:Sigmoid",
    "Curve:RectangularHyperbola1",
    "Curve:RectangularHyperbola2",
    "Curve:ExponentialDecay",
    "Curve:DoubleExponentialDecay",
    "Curve:ChillerPartLoadWithLift",
    "AirflowNetwork:MultiZone:WindPressureCoefficientValues",
];

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum SelectedSetupServiceError {
    Positive(&'static str),
}
impl std::fmt::Display for SelectedSetupServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Positive(family) => write!(f, "positive {family} producer unavailable"),
        }
    }
}
impl std::error::Error for SelectedSetupServiceError {}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SelectedBtwxtManager {
    table_files: BTreeMap<std::path::PathBuf, ()>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SelectedCurves {
    btwxt: SelectedBtwxtManager,
    pressure_curves: Option<Vec<()>>,
}
impl SelectedCurves {
    fn get_curve_input(
        &mut self,
        input: &SelectedInputProcessor,
    ) -> Result<(), SelectedSetupServiceError> {
        let alpha_args = vec![String::new(); 14]; // Source689 actual local shape.
        let numeric_args = vec![None::<f64>; 10000]; // Source690 non-INIT local.
        let counts: [usize; 22] =
            std::array::from_fn(|i| input.get_num_objects_found(CURVE_FAMILIES[i]));
        if let Some(index) = counts.iter().position(|&count| count > 0) {
            return Err(SelectedSetupServiceError::Positive(CURVE_FAMILIES[index]));
        }
        // All actual analytic/WPC loops have zero iterations; no curve is added.
        if input.get_num_objects_found("Table:IndependentVariable") > 0 {
            return Err(SelectedSetupServiceError::Positive(
                "Table:IndependentVariable",
            ));
        }
        let variable_list_count = input.get_num_objects_found("Table:IndependentVariableList");
        let variable_list_limits: BTreeMap<String, Vec<(f64, f64)>> = BTreeMap::new();
        let variable_list_normalization_targets: BTreeMap<String, Vec<f64>> = BTreeMap::new();
        if variable_list_count > 0 {
            return Err(SelectedSetupServiceError::Positive(
                "Table:IndependentVariableList",
            ));
        }
        self.btwxt.table_files.clear(); // Source2718 genuine getter tail.
        drop((
            alpha_args,
            numeric_args,
            variable_list_limits,
            variable_list_normalization_targets,
        ));
        Ok(())
    }
    fn get_pressure_system_input(
        &mut self,
        input: &SelectedInputProcessor,
    ) -> Result<(), SelectedSetupServiceError> {
        let locals = (vec![String::new(); 1], vec![None::<f64>; 5]);
        let count = input.get_num_objects_found("Curve:Functional:PressureDrop");
        self.pressure_curves = Some(vec![(); count]); // Actual zero allocation3443.
        if count > 0 {
            return Err(SelectedSetupServiceError::Positive(
                "Curve:Functional:PressureDrop",
            ));
        }
        drop(locals);
        Ok(())
    }
    pub(crate) fn initialize_state(
        &mut self,
        input: &SelectedInputProcessor,
    ) -> Result<(), SelectedSetupServiceError> {
        self.get_curve_input(input)?;
        self.get_pressure_system_input(input)
    }
}
