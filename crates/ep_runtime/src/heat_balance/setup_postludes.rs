//! Same retained CurveManager, literal five analytic input loops and live reporting.
use super::error_backend::{SourceDiagnosticFlags, SourceErrorBackend, SourceErrorBackendFailure};
use super::radiation::ordinary::{OrdinaryRadiationError, OrdinaryRadiationUnavailable};
use super::selected_heat_balance_input::HeatBalanceInputError;
use super::selected_input_processor::{
    SelectedInputProcessor, SelectedInputUnavailable, SourceInputProcessorShortCut,
    SourceInputStatisticsValue,
};
use super::source_output_prescan::SourceOutputPreScan;
use super::source_output_registration::{
    SourceOutputDictionaryHost, SourceOutputFileHost, SourceOutputRegistration,
    SourceOutputStoreType, SourceOutputUnits, SourceRealOutputBinding, SourceRealOutputDeclaration,
    SourceRealOutputOwners, SourceRealOutputTarget, SourceRealRegistrationOptions,
    SourceReportVariableEnvironment, SourceReportVariableInput, TimeStepKind,
};
use std::collections::BTreeMap;
use std::io::Write;

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

#[derive(Debug)]
pub(crate) enum SelectedSetupServiceError {
    Positive(&'static str),
    Input(SelectedInputUnavailable),
    Backend(SourceErrorBackendFailure),
    Diagnostic(HeatBalanceInputError),
    Output(OrdinaryRadiationError),
    Format(super::source_round_format::SourceRoundFormatUnavailable),
}
impl std::fmt::Display for SelectedSetupServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Positive(family) => write!(f, "positive {family} producer unavailable"),
            Self::Input(error) => std::fmt::Display::fmt(error, f),
            Self::Backend(error) => std::fmt::Display::fmt(error, f),
            Self::Diagnostic(error) => std::fmt::Display::fmt(error, f),
            Self::Output(error) => std::fmt::Display::fmt(error, f),
            Self::Format(error) => write!(f, "Curve R formatter unavailable: {error:?}"),
        }
    }
}
impl std::error::Error for SelectedSetupServiceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Input(error) => Some(error),
            Self::Backend(error) => Some(error),
            Self::Diagnostic(error) => Some(error),
            Self::Output(error) => Some(error),
            Self::Positive(_) | Self::Format(_) => None,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SelectedBtwxtManager {
    table_files: BTreeMap<std::path::PathBuf, ()>,
}

/// Native CurveManager.hh curveType discriminants for the owned five-type slice.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(i32)]
pub(crate) enum SourceCurveType {
    #[default]
    Invalid = -1,
    BiQuadratic = 2,
    QuadraticLinear = 4,
    BiCubic = 5,
    FanPressureRise = 9,
    CubicLinear = 18,
}
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct SourceCurveLimits {
    pub(crate) min: f64,
    pub(crate) max: f64,
    pub(crate) min_present: bool,
    pub(crate) max_present: bool,
}
/// Actual constructor fields, including held Table/EMS fields, never typed coefficients.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceCurve {
    pub(crate) name: String,
    pub(crate) num: i32,
    pub(crate) curve_type: SourceCurveType,
    pub(crate) table_index: i32,
    pub(crate) num_dims: i32,
    pub(crate) grid_value_index: i32,
    pub(crate) context_string: String,
    pub(crate) coeff: [f64; 27],
    pub(crate) inputs: [f64; 6],
    pub(crate) input_limits: [SourceCurveLimits; 6],
    pub(crate) output: f64,
    pub(crate) output_limits: SourceCurveLimits,
    pub(crate) ems_override_on: bool,
    pub(crate) ems_override_curve_value: f64,
}
impl Default for SourceCurve {
    fn default() -> Self {
        Self {
            name: String::new(),
            num: 0,
            curve_type: SourceCurveType::Invalid,
            table_index: 0,
            num_dims: 0,
            grid_value_index: 0,
            context_string: String::new(),
            coeff: [0.0; 27],
            inputs: [0.0; 6],
            input_limits: [SourceCurveLimits::default(); 6],
            output: 0.0,
            output_limits: SourceCurveLimits::default(),
            ems_override_on: false,
            ems_override_curve_value: 0.0,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SourceCurveOutputField {
    /// Native dimension, one based.
    Input(usize),
    Output,
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SelectedCurves {
    curve_value_my_begin_time_step_flag: bool,
    friction_factor_error_has_occurred: bool,
    show_fallback_message: bool,
    curves: Option<Vec<SourceCurve>>,
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

struct CurveDefinition {
    family: &'static str,
    kind: SourceCurveType,
    coefficient_count: usize,
    count_index: usize,
    fields: &'static [(&'static str, bool)],
}
const ANALYTIC_2D: [CurveDefinition; 5] = [
    CurveDefinition {
        family: "Curve:Biquadratic",
        count_index: 0,
        kind: SourceCurveType::BiQuadratic,
        coefficient_count: 6,
        fields: &[
            ("name", true),
            ("coefficient1_constant", false),
            ("coefficient2_x", false),
            ("coefficient3_x_2", false),
            ("coefficient4_y", false),
            ("coefficient5_y_2", false),
            ("coefficient6_x_y", false),
            ("minimum_value_of_x", false),
            ("maximum_value_of_x", false),
            ("minimum_value_of_y", false),
            ("maximum_value_of_y", false),
            ("minimum_curve_output", false),
            ("maximum_curve_output", false),
            ("input_unit_type_for_x", true),
            ("input_unit_type_for_y", true),
            ("output_unit_type", true),
        ],
    },
    CurveDefinition {
        family: "Curve:QuadraticLinear",
        count_index: 6,
        kind: SourceCurveType::QuadraticLinear,
        coefficient_count: 6,
        fields: &[
            ("name", true),
            ("coefficient1_constant", false),
            ("coefficient2_x", false),
            ("coefficient3_x_2", false),
            ("coefficient4_y", false),
            ("coefficient5_x_y", false),
            ("coefficient6_x_2_y", false),
            ("minimum_value_of_x", false),
            ("maximum_value_of_x", false),
            ("minimum_value_of_y", false),
            ("maximum_value_of_y", false),
            ("minimum_curve_output", false),
            ("maximum_curve_output", false),
            ("input_unit_type_for_x", true),
            ("input_unit_type_for_y", true),
            ("output_unit_type", true),
        ],
    },
    CurveDefinition {
        family: "Curve:CubicLinear",
        count_index: 7,
        kind: SourceCurveType::CubicLinear,
        coefficient_count: 6,
        fields: &[
            ("name", true),
            ("coefficient1_constant", false),
            ("coefficient2_x", false),
            ("coefficient3_x_2", false),
            ("coefficient4_x_3", false),
            ("coefficient5_y", false),
            ("coefficient6_x_y", false),
            ("minimum_value_of_x", false),
            ("maximum_value_of_x", false),
            ("minimum_value_of_y", false),
            ("maximum_value_of_y", false),
            ("minimum_curve_output", false),
            ("maximum_curve_output", false),
            ("input_unit_type_for_x", true),
            ("input_unit_type_for_y", true),
            ("output_unit_type", true),
        ],
    },
    CurveDefinition {
        family: "Curve:Bicubic",
        count_index: 9,
        kind: SourceCurveType::BiCubic,
        coefficient_count: 10,
        fields: &[
            ("name", true),
            ("coefficient1_constant", false),
            ("coefficient2_x", false),
            ("coefficient3_x_2", false),
            ("coefficient4_y", false),
            ("coefficient5_y_2", false),
            ("coefficient6_x_y", false),
            ("coefficient7_x_3", false),
            ("coefficient8_y_3", false),
            ("coefficient9_x_2_y", false),
            ("coefficient10_x_y_2", false),
            ("minimum_value_of_x", false),
            ("maximum_value_of_x", false),
            ("minimum_value_of_y", false),
            ("maximum_value_of_y", false),
            ("minimum_curve_output", false),
            ("maximum_curve_output", false),
            ("input_unit_type_for_x", true),
            ("input_unit_type_for_y", true),
            ("output_unit_type", true),
        ],
    },
    CurveDefinition {
        family: "Curve:FanPressureRise",
        count_index: 13,
        kind: SourceCurveType::FanPressureRise,
        coefficient_count: 4,
        fields: &[
            ("name", true),
            ("coefficient1_c1", false),
            ("coefficient2_c2", false),
            ("coefficient3_c3", false),
            ("coefficient4_c4", false),
            ("minimum_value_of_qfan", false),
            ("maximum_value_of_qfan", false),
            ("minimum_value_of_psm", false),
            ("maximum_value_of_psm", false),
            ("minimum_curve_output", false),
            ("maximum_curve_output", false),
        ],
    },
];

/// Actual GetCurveInputData local arrays; uninitialized numerics stay None until IP fills them.
struct SourceCurveInputArrays {
    alphas: Vec<String>,
    numerics: Vec<Option<f64>>,
}
impl SourceCurveInputArrays {
    fn new() -> Self {
        Self {
            alphas: vec![String::new(); 14],
            numerics: vec![None; 10000],
        }
    }
    fn number(&self, index: usize) -> Result<f64, SelectedSetupServiceError> {
        self.numerics
            .get(index)
            .copied()
            .flatten()
            .ok_or(SelectedSetupServiceError::Input(
                SelectedInputUnavailable::Association,
            ))
    }
    /// Native caller supplies only shared NumBlank and both field-name arrays.
    fn get_item(
        &mut self,
        definition: &CurveDefinition,
        name: &str,
        input: &mut SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
    ) -> Result<(usize, usize, i32), SelectedSetupServiceError> {
        if !input.has_object(definition.family, name) {
            return Err(SelectedSetupServiceError::Input(
                SelectedInputUnavailable::Association,
            ));
        }
        let alpha_size = definition.fields.iter().filter(|(_, alpha)| *alpha).count();
        let numeric_size = definition.fields.len() - alpha_size;
        if shortcut.alpha_field_names.len() < alpha_size
            || shortcut.numeric_field_names.len() < numeric_size
            || shortcut.numeric_field_blanks.len() < numeric_size
        {
            return Err(SelectedSetupServiceError::Input(
                SelectedInputUnavailable::Size,
            ));
        }
        let maximum = {
            let fields = input
                .source_raw_object_fields(definition.family, name)
                .map_err(SelectedSetupServiceError::Input)?;
            definition
                .fields
                .iter()
                .enumerate()
                .filter(|(_, (field, _))| fields.contains_key(*field))
                .map(|(index, _)| index + 1)
                .max()
                .unwrap_or(0)
        };
        self.alphas.fill(String::new());
        self.numerics.fill(Some(0.0));
        shortcut.numeric_field_blanks.fill(true);
        shortcut.alpha_field_names.fill(String::new());
        shortcut.numeric_field_names.fill(String::new());
        input.mark_object_as_used(definition.family, name);
        let fields = input
            .source_raw_object_fields(definition.family, name)
            .map_err(SelectedSetupServiceError::Input)?;
        let (mut alpha, mut numeric, mut num_alphas, mut num_numbers) = (0, 0, 0, 0);
        for (position, (field, is_alpha)) in definition.fields.iter().enumerate() {
            if *is_alpha {
                shortcut.alpha_field_names[alpha] = (*field).into();
                if position == 0 {
                    self.alphas[alpha] = name.to_ascii_uppercase();
                    num_alphas += 1;
                } else {
                    self.alphas[alpha] = match fields.get(*field) {
                        Some(SourceInputStatisticsValue::String(value)) if !value.is_empty() => {
                            value.to_ascii_uppercase()
                        }
                        None | Some(SourceInputStatisticsValue::String(_)) => {
                            "DIMENSIONLESS".into()
                        }
                        _ => {
                            return Err(SelectedSetupServiceError::Input(
                                SelectedInputUnavailable::Association,
                            ));
                        }
                    };
                    if position < maximum {
                        num_alphas += 1;
                    }
                }
                alpha += 1;
            } else {
                shortcut.numeric_field_names[numeric] = (*field).into();
                if let Some(value) = fields.get(*field) {
                    let SourceInputStatisticsValue::Number(token) = value else {
                        return Err(SelectedSetupServiceError::Input(
                            SelectedInputUnavailable::Association,
                        ));
                    };
                    self.numerics[numeric] = Some(
                        ep_model::classify_source_json_number(token)
                            .map_err(|_| {
                                SelectedSetupServiceError::Input(
                                    SelectedInputUnavailable::Association,
                                )
                            })?
                            .as_f64(),
                    );
                    shortcut.numeric_field_blanks[numeric] = false;
                }
                if position < maximum {
                    num_numbers += 1;
                }
                numeric += 1;
            }
        }
        Ok((num_alphas, num_numbers, 1))
    }
}

/// Temporary disjoint actual owners for InitCurveReporting; no retained duplicate state.
pub(crate) struct SourceCurveReportContext<'a, 'backend, H, D> {
    pub(crate) globals: &'a super::selected_heat_balance_globals::SelectedHeatBalanceGlobals,
    pub(crate) input: &'a mut SelectedInputProcessor,
    pub(crate) schedules: Option<&'a mut super::selected_schedule_manager::SelectedScheduleManager>,
    pub(crate) reports: &'a ep_model::ReportInputs,
    pub(crate) registration: &'a mut SourceOutputRegistration,
    pub(crate) prescan: &'a SourceOutputPreScan,
    pub(crate) minimum_report_frequency: Option<&'a str>,
    pub(crate) minutes_in_time_step: i32,
    pub(crate) diagnostic_flags: Option<SourceDiagnosticFlags<'a>>,
    pub(crate) backend: &'a mut SourceErrorBackend<'backend>,
    pub(crate) host: &'a mut H,
    pub(crate) dictionary_host: &'a mut D,
    pub(crate) eio: &'a mut dyn Write,
}
impl SelectedCurves {
    fn curve_at(&self, index: i32) -> Option<&SourceCurve> {
        usize::try_from(index)
            .ok()?
            .checked_sub(1)
            .and_then(|at| self.curves.as_ref()?.get(at))
    }
    /// Native exact map lookup; caller's IP alpha already has uppercase/untrimmed spelling.
    #[allow(
        dead_code,
        reason = "Native name lookup API retained while positive CurveValue consumers remain held"
    )]
    pub(crate) fn get_curve_index(&self, name: &str) -> i32 {
        self.curve_map.get(name).copied().unwrap_or(0)
    }
    /// Native GetCurve3156 returns the same actual row selected by its NAME lookup.
    #[allow(
        dead_code,
        reason = "Native same-row GetCurve API retained while material Curve consumers remain held"
    )]
    pub(crate) fn get_curve(&self, name: &str) -> Option<&SourceCurve> {
        self.curve_at(self.get_curve_index(name))
    }
    pub(crate) fn read_curve_output(
        &self,
        field: SourceCurveOutputField,
        index: i32,
    ) -> Option<&f64> {
        let curve = self.curve_at(index)?;
        match field {
            SourceCurveOutputField::Input(dimension)
                if dimension > 0 && dimension <= usize::try_from(curve.num_dims).ok()? =>
            {
                curve.inputs.get(dimension - 1)
            }
            SourceCurveOutputField::Input(_) => None,
            SourceCurveOutputField::Output => Some(&curve.output),
        }
    }
    fn add_curve(&mut self, name: &str) -> Result<&mut SourceCurve, SelectedSetupServiceError> {
        let rows = self.curves.get_or_insert_with(Vec::new);
        let num = rows
            .len()
            .checked_add(1)
            .and_then(|n| i32::try_from(n).ok())
            .ok_or(SelectedSetupServiceError::Input(
                SelectedInputUnavailable::Size,
            ))?;
        rows.push(SourceCurve {
            name: name.into(),
            num,
            ..SourceCurve::default()
        });
        self.curve_map.insert(name.to_ascii_uppercase(), num);
        rows.last_mut().ok_or(SelectedSetupServiceError::Input(
            SelectedInputUnavailable::Association,
        ))
    }
    pub(crate) fn initialize_source_curve_reporting<
        H: SourceOutputFileHost,
        D: SourceOutputDictionaryHost,
    >(
        &self,
        o: &mut SourceCurveReportContext<'_, '_, H, D>,
    ) -> Result<(), SelectedSetupServiceError> {
        for curve in self.curves.as_deref().unwrap_or(&[]) {
            let dimensions = usize::try_from(curve.num_dims)
                .map_err(|_| SelectedSetupServiceError::Input(SelectedInputUnavailable::Size))?;
            for dimension in 1..=dimensions {
                self.register_curve_output(
                    o,
                    curve,
                    SourceCurveOutputField::Input(dimension),
                    &format!("Performance Curve Input Variable {dimension} Value"),
                )?;
            }
            self.register_curve_output(
                o,
                curve,
                SourceCurveOutputField::Output,
                "Performance Curve Output Value",
            )?;
        }
        if self
            .pressure_curves
            .as_ref()
            .is_some_and(|rows| !rows.is_empty())
        {
            return Err(SelectedSetupServiceError::Positive(
                "InitCurveReporting2878 pressure output",
            ));
        }
        if o.globals.any_energy_management_system_in_model
            && self.curves.as_ref().is_some_and(|rows| !rows.is_empty())
        {
            return Err(SelectedSetupServiceError::Positive(
                "InitCurveReporting2910 curve actuator",
            ));
        }
        if o.globals.any_energy_management_system_in_model
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
    fn register_curve_output<H: SourceOutputFileHost, D: SourceOutputDictionaryHost>(
        &self,
        o: &mut SourceCurveReportContext<'_, '_, H, D>,
        curve: &SourceCurve,
        field: SourceCurveOutputField,
        name: &str,
    ) -> Result<(), SelectedSetupServiceError> {
        let binding = SourceRealOutputBinding::new(
            SourceRealOutputTarget::Curve {
                field,
                index: curve.num,
            },
            self,
        )
        .map_err(SelectedSetupServiceError::Output)?;
        let minimum_report_frequency =
            o.minimum_report_frequency
                .ok_or(SelectedSetupServiceError::Input(
                    SelectedInputUnavailable::Association,
                ))?;
        let diagnostic_flags = o.diagnostic_flags.ok_or(SelectedSetupServiceError::Input(
            SelectedInputUnavailable::Association,
        ))?;
        let schedules = o
            .schedules
            .as_deref_mut()
            .ok_or(SelectedSetupServiceError::Input(
                SelectedInputUnavailable::Association,
            ))?;
        let mut report_input = SourceReportVariableInput::with_schedules(
            o.reports,
            o.input,
            SourceReportVariableEnvironment::new(minimum_report_frequency, Some(diagnostic_flags)),
            schedules,
            o.backend,
        );
        o.registration
            .setup_bound_real(
                &mut report_input,
                o.prescan,
                SourceRealOutputDeclaration {
                    name,
                    key: &curve.name,
                    units: SourceOutputUnits::Dimensionless,
                    store_type: SourceOutputStoreType::Average,
                },
                o.minutes_in_time_step,
                o.host,
                o.eio,
                binding,
                o.dictionary_host,
                SourceRealRegistrationOptions::unmetered(TimeStepKind::System),
            )
            .map_err(SelectedSetupServiceError::Output)
    }
    fn get_curve_input_data(
        &mut self,
        input: &mut SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
        flags: &SourceDiagnosticFlags<'_>,
        backend: &mut SourceErrorBackend<'_>,
        errors_found: &mut bool,
    ) -> Result<(), SelectedSetupServiceError> {
        let mut arrays = SourceCurveInputArrays::new();
        let counts: [usize; 22] =
            std::array::from_fn(|i| input.get_num_objects_found(CURVE_FAMILIES[i]));
        if let Some(index) = counts
            .iter()
            .enumerate()
            .position(|(i, count)| *count > 0 && !matches!(i, 0 | 6 | 7 | 9 | 13))
        {
            return Err(SelectedSetupServiceError::Positive(CURVE_FAMILIES[index]));
        }
        // Actual selected family order, independent of JSON family order or count-array order.
        for definition in &ANALYTIC_2D {
            let count = counts[definition.count_index];
            if count == 0 {
                continue;
            }
            let names = input
                .source_raw_object_names(definition.family)
                .map_err(SelectedSetupServiceError::Input)?
                .to_vec();
            if names.len() != count {
                return Err(SelectedSetupServiceError::Input(
                    SelectedInputUnavailable::Association,
                ));
            }
            for object_name in names {
                let (num_alphas, num_numbers, _io_status) =
                    arrays.get_item(definition, &object_name, input, shortcut)?;
                let name = arrays.alphas[0].clone();
                if self.curve_map.contains_key(&name) {
                    backend
                        .show_source_severe_error(
                            &format!(
                                "GetCurveInputData: {} = {name}, duplicate name.",
                                definition.family
                            ),
                            flags,
                        )
                        .map_err(SelectedSetupServiceError::Diagnostic)?;
                    *errors_found = true;
                }
                let curve = self.add_curve(&name)?;
                curve.curve_type = definition.kind;
                curve.num_dims = 2;
                for index in 0..definition.coefficient_count {
                    curve.coeff[index] = arrays.number(index)?;
                }
                let limits = definition.coefficient_count;
                curve.input_limits[0].min = arrays.number(limits)?;
                curve.input_limits[0].max = arrays.number(limits + 1)?;
                curve.input_limits[1].min = arrays.number(limits + 2)?;
                curve.input_limits[1].max = arrays.number(limits + 3)?;
                if num_numbers > limits + 4 && !shortcut.numeric_field_blanks[limits + 4] {
                    curve.output_limits.min = arrays.number(limits + 4)?;
                    curve.output_limits.min_present = true;
                }
                if num_numbers > limits + 5 && !shortcut.numeric_field_blanks[limits + 5] {
                    curve.output_limits.max = arrays.number(limits + 5)?;
                    curve.output_limits.max_present = true;
                }
                for first in [limits, limits + 2] {
                    let low = arrays.number(first)?;
                    let high = arrays.number(first + 1)?;
                    if low > high {
                        backend
                            .show_source_severe_error(
                                &format!("GetCurveInput: For {}: ", definition.family),
                                flags,
                            )
                            .map_err(SelectedSetupServiceError::Diagnostic)?;
                        let low_text = super::source_round_format::round(low, 2)
                            .map_err(SelectedSetupServiceError::Format)?;
                        let high_text = super::source_round_format::round(high, 2)
                            .map_err(SelectedSetupServiceError::Format)?;
                        backend
                            .show_continue_error(&format!(
                                "{} [{low_text}] > {} [{high_text}]",
                                shortcut.numeric_field_names[first],
                                shortcut.numeric_field_names[first + 1]
                            ))
                            .map_err(SelectedSetupServiceError::Backend)?;
                        *errors_found = true;
                    }
                }
                for (alpha, description) in [
                    (1, "Input Unit Type for X"),
                    (2, "Input Unit Type for Y"),
                    (3, "Output Unit Type"),
                ] {
                    if num_alphas > alpha {
                        let valid = if alpha == 3 {
                            curve_output_type_valid(&arrays.alphas[alpha])
                        } else {
                            curve_input_type_valid(&arrays.alphas[alpha])
                        };
                        if !valid {
                            backend
                                .show_source_warning_error(
                                    &format!(
                                        "In {} named {name} the {description} is invalid.",
                                        definition.family
                                    ),
                                    flags,
                                )
                                .map_err(SelectedSetupServiceError::Diagnostic)?;
                        }
                    }
                }
            }
        }
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
        self.btwxt.table_files.clear(); // Native2718 genuine getter tail, even after successful diagnostics.
        drop((
            arrays,
            variable_list_limits,
            variable_list_normalization_targets,
        ));
        Ok(())
    }
    fn get_curve_input(
        &mut self,
        input: &mut SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
        flags: &SourceDiagnosticFlags<'_>,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), SelectedSetupServiceError> {
        let mut errors_found = false;
        self.get_curve_input_data(input, shortcut, flags, backend, &mut errors_found)?;
        if errors_found {
            return match backend.show_source_fatal_error("GetCurveInput: Errors found in getting Curve Objects.  Preceding condition(s) cause termination.", flags) {
                Ok(never) => match never {},
                Err(error) => Err(SelectedSetupServiceError::Diagnostic(error)),
            };
        }
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
        input: &mut SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
        flags: &SourceDiagnosticFlags<'_>,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), SelectedSetupServiceError> {
        self.get_curve_input(input, shortcut, flags, backend)?;
        self.get_pressure_system_input(input)
    }
}
impl SourceRealOutputOwners for SelectedCurves {
    fn real_output(&self, target: SourceRealOutputTarget) -> Result<&f64, OrdinaryRadiationError> {
        let SourceRealOutputTarget::Curve { field, index } = target else {
            return Err(OrdinaryRadiationUnavailable::StorageDomain.into());
        };
        self.read_curve_output(field, index)
            .ok_or_else(|| OrdinaryRadiationUnavailable::StorageDomain.into())
    }
}
fn curve_input_type_valid(value: &str) -> bool {
    matches!(
        value.to_ascii_uppercase().as_str(),
        "" | "DIMENSIONLESS"
            | "TEMPERATURE"
            | "PRESSURE"
            | "VOLUMETRICFLOW"
            | "MASSFLOW"
            | "POWER"
            | "DISTANCE"
            | "WAVELENGTH"
            | "ANGLE"
            | "VOLUMETRICFLOWPERPOWER"
    )
}
fn curve_output_type_valid(value: &str) -> bool {
    matches!(
        value.to_ascii_uppercase().as_str(),
        "DIMENSIONLESS" | "PRESSURE" | "TEMPERATURE" | "CAPACITY" | "POWER"
    )
}
