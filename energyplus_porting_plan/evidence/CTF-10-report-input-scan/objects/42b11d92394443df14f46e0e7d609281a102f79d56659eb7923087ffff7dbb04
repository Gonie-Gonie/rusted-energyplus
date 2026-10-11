//! Input-only owners read by General::ScanForReports; no report is generated here.

/// The five actual object families visited by the first report scan.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReportInputObjectType {
    /// Output:Surfaces:List.
    SurfacesList,
    /// Output:Surfaces:Drawing.
    SurfacesDrawing,
    /// Output:VariableDictionary.
    VariableDictionary,
    /// Output:Constructions.
    Constructions,
    /// Output:EnergyManagementSystem; preserving report input does not activate EMS.
    EnergyManagementSystem,
}

impl ReportInputObjectType {
    /// Canonical source object type.
    #[must_use]
    pub const fn source_name(self) -> &'static str {
        match self {
            Self::SurfacesList => "Output:Surfaces:List",
            Self::SurfacesDrawing => "Output:Surfaces:Drawing",
            Self::VariableDictionary => "Output:VariableDictionary",
            Self::Constructions => "Output:Constructions",
            Self::EnergyManagementSystem => "Output:EnergyManagementSystem",
        }
    }
}

/// A real input object's default-applied getObjectItem alpha projection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReportInputObject {
    /// Original epJSON key, or the actual IdfParser-generated unnamed identity.
    pub object_name: String,
    /// Source family; records retain actual order within each family.
    pub object_type: ReportInputObjectType,
    /// Actual uppercase/default-applied alpha values; unused positions are empty.
    pub alphas: [String; 3],
    /// Actual blank facts, including a default substituted for missing/empty input.
    pub alpha_blank: [bool; 3],
    /// Which alpha values came from an IDD default.
    pub default_applied: [bool; 3],
    /// Actual supplied/minimum-field alpha count; defaults do not extend this.
    pub num_alphas: u8,
    /// True for direct epJSON, false for the validated staged-IDF order overlay.
    pub is_epjson: bool,
}

impl ReportInputObject {
    /// Exact field label used by ScanForReports' surface-list warning.
    #[must_use]
    pub fn report_type_field_name(&self) -> &'static str {
        if self.is_epjson {
            "report_type"
        } else {
            "Report Type"
        }
    }
}

/// Input-only report objects; no cache or observed report outcome is supplied.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReportInputs {
    /// Actual object order within each source family; cross-family scan order is fixed.
    pub objects: Vec<ReportInputObject>,
    /// Actual getNumSectionsFound result for the legacy dictionary section; -1 absent.
    pub legacy_variable_dictionary_sections: i32,
}

impl Default for ReportInputs {
    fn default() -> Self {
        Self {
            objects: Vec::new(),
            legacy_variable_dictionary_sections: -1,
        }
    }
}
