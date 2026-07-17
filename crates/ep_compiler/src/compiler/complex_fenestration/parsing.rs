use ep_model::{
    ComplexFenestrationBasisSymmetry, ComplexFenestrationBasisType, WindowThermalCalculationModel,
    WindowThermalCalculationStandard,
};

#[derive(Clone, Copy)]
pub(super) enum ParsedDeflectionModel {
    None,
    Measured,
    TemperatureAndPressure,
}

#[derive(Clone, Copy)]
pub(super) enum ComplexLayerExpectation {
    Optical,
    Gap,
}

impl ComplexLayerExpectation {
    pub(super) const fn target_label(self) -> &'static str {
        match self {
            Self::Optical => "WindowMaterial:Glazing or WindowMaterial:ComplexShade",
            Self::Gap => "WindowMaterial:Gap",
        }
    }

    pub(super) const fn supported_boundary(self) -> &'static str {
        match self {
            Self::Optical => {
                "the bounded complex-fenestration slice accepts only SpectralAverage WindowMaterial:Glazing or WindowMaterial:ComplexShade optical layers"
            }
            Self::Gap => "complex-fenestration gap positions accept only WindowMaterial:Gap",
        }
    }
}

pub(super) fn parse_basis_type(value: &str) -> Option<ComplexFenestrationBasisType> {
    if value.trim().eq_ignore_ascii_case("LBNLWINDOW") {
        Some(ComplexFenestrationBasisType::LbnlWindow)
    } else if value.trim().eq_ignore_ascii_case("UserDefined") {
        Some(ComplexFenestrationBasisType::UserDefined)
    } else {
        None
    }
}

pub(super) fn parse_basis_symmetry(value: &str) -> Option<ComplexFenestrationBasisSymmetry> {
    if value.trim().eq_ignore_ascii_case("None") {
        Some(ComplexFenestrationBasisSymmetry::None)
    } else if value.trim().eq_ignore_ascii_case("Axisymmetric") {
        Some(ComplexFenestrationBasisSymmetry::Axisymmetric)
    } else {
        None
    }
}

pub(super) fn parse_thermal_calculation_standard(
    value: &str,
) -> Option<WindowThermalCalculationStandard> {
    if value.trim().eq_ignore_ascii_case("ISO15099") {
        Some(WindowThermalCalculationStandard::Iso15099)
    } else if value.trim().eq_ignore_ascii_case("EN673Declared") {
        Some(WindowThermalCalculationStandard::En673Declared)
    } else if value.trim().eq_ignore_ascii_case("EN673Design") {
        Some(WindowThermalCalculationStandard::En673Design)
    } else {
        None
    }
}

pub(super) fn parse_thermal_calculation_model(
    value: &str,
) -> Option<WindowThermalCalculationModel> {
    if value.trim().eq_ignore_ascii_case("ISO15099") {
        Some(WindowThermalCalculationModel::Iso15099)
    } else if value.trim().eq_ignore_ascii_case("ScaledCavityWidth") {
        Some(WindowThermalCalculationModel::ScaledCavityWidth)
    } else if value
        .trim()
        .eq_ignore_ascii_case("ConvectiveScalarModel_NoSDThickness")
    {
        Some(WindowThermalCalculationModel::ConvectiveScalarNoSdThickness)
    } else if value
        .trim()
        .eq_ignore_ascii_case("ConvectiveScalarModel_withSDThickness")
    {
        Some(WindowThermalCalculationModel::ConvectiveScalarWithSdThickness)
    } else {
        None
    }
}

pub(super) fn parse_deflection_model(value: &str) -> Option<ParsedDeflectionModel> {
    if value.trim().eq_ignore_ascii_case("NoDeflection") {
        Some(ParsedDeflectionModel::None)
    } else if value.trim().eq_ignore_ascii_case("MeasuredDeflection") {
        Some(ParsedDeflectionModel::Measured)
    } else if value
        .trim()
        .eq_ignore_ascii_case("TemperatureAndPressureInput")
    {
        Some(ParsedDeflectionModel::TemperatureAndPressure)
    } else {
        None
    }
}

pub(super) fn format_complex_number(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{value:.1}")
    } else {
        value.to_string()
    }
}
