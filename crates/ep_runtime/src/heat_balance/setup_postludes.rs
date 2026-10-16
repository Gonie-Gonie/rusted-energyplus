//! Same retained CurveManager input state and genuine empty-array report traversal.
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

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SelectedCurves {
    curve_value_my_begin_time_step_flag: bool,
    friction_factor_error_has_occurred: bool,
    show_fallback_message: bool,
    curves: Option<Vec<std::convert::Infallible>>,
    curve_map: BTreeMap<String, i32>,
    btwxt: SelectedBtwxtManager,
    pressure_curves: Option<Vec<()>>,
}
impl Default for SelectedCurves {
    fn default() -> Self {
        Self {
            curve_value_my_begin_time_step_flag: false,
            friction_factor_error_has_occurred: false,
            show_fallback_message: true,
            curves: None,
            curve_map: BTreeMap::new(),
            btwxt: SelectedBtwxtManager::default(),
            pressure_curves: None,
        }
    }
}
impl SelectedCurves {
    pub(crate) fn initialize_source_curve_reporting(
        &self,
        globals: &super::selected_heat_balance_globals::SelectedHeatBalanceGlobals,
    ) -> Result<(), SelectedSetupServiceError> {
        // Native2856: actual constructor-unallocated curves have zero iterations.
        // Positive record constructors/getters remain unavailable, not unit records.
        if self.curves.as_ref().is_some_and(|rows| !rows.is_empty()) {
            return Err(SelectedSetupServiceError::Positive(
                "InitCurveReporting2857 curve numDims",
            ));
        }
        // Native2877 independently iterates the same actual PressureCurve array.
        if self
            .pressure_curves
            .as_ref()
            .is_some_and(|rows| !rows.is_empty())
        {
            return Err(SelectedSetupServiceError::Positive(
                "InitCurveReporting2878 pressure output",
            ));
        }
        if globals.any_energy_management_system_in_model
            && self.curves.as_ref().is_some_and(|rows| !rows.is_empty())
        {
            return Err(SelectedSetupServiceError::Positive(
                "InitCurveReporting2910 curve actuator",
            ));
        }
        if globals.any_energy_management_system_in_model
            && self
                .pressure_curves
                .as_ref()
                .is_some_and(|rows| !rows.is_empty())
        {
            return Err(SelectedSetupServiceError::Positive(
                "InitCurveReporting2916 pressure actuator",
            ));
        }
        Ok(())
    }
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
