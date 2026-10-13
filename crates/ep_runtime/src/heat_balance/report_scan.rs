//! Literal General.cc1130–1469 report-cache processing; no output backend activation.

use ep_model::{ReportInputObjectType as Kind, ReportInputs};

mod request;
mod source_input;
pub(crate) use source_input::SourceReportScanInput;

/// General's retained report flags/options and the directly written companion flags.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReportScanState {
    /// GeneralData::GetReportInput, initially true and cleared after the first scan.
    pub get_report_input: bool,
    /// GeneralData::SurfVert.
    pub surface_vertices: bool,
    /// GeneralData::SurfDet.
    pub surface_details: bool,
    /// GeneralData::SurfDetWVert.
    pub surface_details_with_vertices: bool,
    /// GeneralData::LineRpt.
    pub lines: bool,
    /// GeneralData::LineRptOption1.
    pub line_option: String,
    /// GeneralData::CostInfo.
    pub cost_info: bool,
    /// GeneralData::ViewFactorInfo.
    pub view_factor_info: bool,
    /// GeneralData::ViewRptOption1.
    pub view_factor_option: String,
    /// GeneralData::DXFReport.
    pub dxf: bool,
    /// GeneralData::DXFOption1/2.
    pub dxf_options: [String; 2],
    /// GeneralData::DXFWFReport.
    pub dxf_wireframe: bool,
    /// GeneralData::DXFWFOption1/2.
    pub dxf_wireframe_options: [String; 2],
    /// GeneralData::VRMLReport.
    pub vrml: bool,
    /// GeneralData::VRMLOption1/2.
    pub vrml_options: [String; 2],
    /// GeneralData::VarDict.
    pub variable_dictionary: bool,
    /// GeneralData::VarDictOption1/2.
    pub variable_dictionary_options: [String; 2],
    /// GeneralData::Constructions.
    pub constructions: bool,
    /// GeneralData::Materials.
    pub materials: bool,
    /// GeneralData::EMSoutput.
    pub ems_output: bool,
    /// DataGlobal::ShowDecayCurvesInEIO.
    pub show_decay_curves_in_eio: bool,
    /// RuntimeLanguage::OutputEMSActuatorAvailSmall/Full.
    pub ems_actuator_availability: [bool; 2],
    /// RuntimeLanguage::OutputEMSInternalVarsSmall/Full.
    pub ems_internal_variable_availability: [bool; 2],
    /// RuntimeLanguage::OutputEMSErrors.
    pub ems_errors: bool,
    /// RuntimeLanguage::OutputFullEMSTrace.
    pub ems_full_trace: bool,
}

impl Default for ReportScanState {
    fn default() -> Self {
        Self {
            get_report_input: true,
            surface_vertices: false,
            surface_details: false,
            surface_details_with_vertices: false,
            lines: false,
            line_option: String::new(),
            cost_info: false,
            view_factor_info: false,
            view_factor_option: String::new(),
            dxf: false,
            dxf_options: Default::default(),
            dxf_wireframe: false,
            dxf_wireframe_options: Default::default(),
            vrml: false,
            vrml_options: Default::default(),
            variable_dictionary: false,
            variable_dictionary_options: Default::default(),
            constructions: false,
            materials: false,
            ems_output: false,
            show_decay_curves_in_eio: false,
            ems_actuator_availability: [false; 2],
            ems_internal_variable_availability: [false; 2],
            ems_errors: false,
            ems_full_trace: false,
        }
    }
}

/// Actual source diagnostic call arguments; a caller must provide its real backend.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReportScanDiagnostic {
    /// ShowWarningError argument; not an inferred Native backend result.
    Warning(String),
    /// ShowContinueError argument, after the actual warning argument.
    Continue(String),
}

/// A source call that cannot be represented as a successful supported request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReportScanUnavailable {
    /// Native dereferences absent ReportKey for Surfaces; no success is fabricated.
    MissingSurfacesKey,
    /// Actual report projection or complete shared getter allocation is inconsistent.
    SourceInputAssociation,
}

/// A real diagnostic transport failure stops at the reached call boundary.
#[derive(Debug)]
pub enum ReportScanFailure<E> {
    /// The requested source call is outside its represented argument domain.
    Unavailable(ReportScanUnavailable),
    /// The actual borrowed diagnostic backend returned this failure.
    Diagnostic(E),
}

/// Executes actual cache assignments and the selected request, preserving option refs.
/// Diagnostic callbacks are real call arguments; full warning/fatal backend parity is
/// a separate prerequisite, and this function makes no whole-Init completion claim.
pub fn scan_for_reports(
    input: &ReportInputs,
    state: &mut ReportScanState,
    report_name: &str,
    report_key: Option<&str>,
    option1: Option<&mut String>,
    option2: Option<&mut String>,
    mut diagnostic: impl FnMut(ReportScanDiagnostic),
) -> Result<bool, ReportScanUnavailable> {
    match try_scan_for_reports(
        input,
        state,
        report_name,
        report_key,
        option1,
        option2,
        |value| {
            diagnostic(value);
            Ok::<(), std::convert::Infallible>(())
        },
    ) {
        Ok(value) => Ok(value),
        Err(ReportScanFailure::Unavailable(value)) => Err(value),
        Err(ReportScanFailure::Diagnostic(never)) => match never {},
    }
}

/// The same literal assignments with a fallible real warning/continue boundary.
/// Failed diagnostics leave GetReportInput and all preceding cache writes at
/// the exact reached prefix; later objects and requests are not evaluated.
pub fn try_scan_for_reports<E>(
    input: &ReportInputs,
    state: &mut ReportScanState,
    report_name: &str,
    report_key: Option<&str>,
    option1: Option<&mut String>,
    option2: Option<&mut String>,
    diagnostic: impl FnMut(ReportScanDiagnostic) -> Result<(), E>,
) -> Result<bool, ReportScanFailure<E>> {
    try_scan_for_reports_inner(
        input,
        state,
        report_name,
        report_key,
        option1,
        option2,
        None,
        diagnostic,
    )
}

/// Same cache/diagnostic implementation with actual first-scan input effects.
#[expect(
    clippy::too_many_arguments,
    reason = "Preserve original report request and explicit shared input borrow"
)]
pub(crate) fn try_scan_for_reports_with_input<E>(
    input: &ReportInputs,
    state: &mut ReportScanState,
    report_name: &str,
    report_key: Option<&str>,
    option1: Option<&mut String>,
    option2: Option<&mut String>,
    source: &mut SourceReportScanInput<'_>,
    diagnostic: impl FnMut(ReportScanDiagnostic) -> Result<(), E>,
) -> Result<bool, ReportScanFailure<E>> {
    try_scan_for_reports_inner(
        input,
        state,
        report_name,
        report_key,
        option1,
        option2,
        Some(source),
        diagnostic,
    )
}

#[expect(
    clippy::too_many_arguments,
    reason = "Preserve original report request and explicit shared input borrow"
)]
fn try_scan_for_reports_inner<E>(
    input: &ReportInputs,
    state: &mut ReportScanState,
    report_name: &str,
    report_key: Option<&str>,
    option1: Option<&mut String>,
    option2: Option<&mut String>,
    mut source: Option<&mut SourceReportScanInput<'_>>,
    mut diagnostic: impl FnMut(ReportScanDiagnostic) -> Result<(), E>,
) -> Result<bool, ReportScanFailure<E>> {
    if state.get_report_input {
        if let Some(source) = source.as_deref_mut() {
            source.enter_family(Kind::SurfacesList);
        }
        for object in input
            .objects
            .iter()
            .filter(|o| o.object_type == Kind::SurfacesList)
        {
            if let Some(source) = source.as_deref_mut() {
                source.get_object(object);
            }
            let a = &object.alphas;
            match a[0].as_str() {
                "LINES" => {
                    state.lines = true;
                    state.line_option.clone_from(&a[1]);
                }
                "VERTICES" => state.surface_vertices = true,
                "DETAILS" | "DETAILED" | "DETAIL" => state.surface_details = true,
                "DETAILSWITHVERTICES" | "DETAILVERTICES" => {
                    state.surface_details_with_vertices = true
                }
                "COSTINFO" => state.cost_info = true,
                "VIEWFACTORINFO" => {
                    state.view_factor_info = true;
                    state.view_factor_option.clone_from(&a[1]);
                }
                "DECAYCURVESFROMCOMPONENTLOADSSUMMARY" => state.show_decay_curves_in_eio = true,
                // Native localMap[unknown] inserts EMPTY; it does not throw int.
                _ => {
                    diagnostic(ReportScanDiagnostic::Warning(format!(
                        "Output:Surfaces:List: No {} supplied.",
                        object.report_type_field_name()
                    )))
                    .map_err(ReportScanFailure::Diagnostic)?;
                    diagnostic(ReportScanDiagnostic::Continue(
                        " Legal values are: \"Lines\", \"Vertices\", \"Details\", \"DetailsWithVertices\", \"CostInfo\", \"ViewFactorIinfo\".".to_string())).map_err(ReportScanFailure::Diagnostic)?;
                }
            }
        }
        if let Some(source) = source.as_deref_mut() {
            source.enter_family(Kind::SurfacesDrawing);
        }
        for object in input
            .objects
            .iter()
            .filter(|o| o.object_type == Kind::SurfacesDrawing)
        {
            if let Some(source) = source.as_deref_mut() {
                source.get_object(object);
            }
            let a = &object.alphas;
            let values = [a[1].clone(), a[2].clone()];
            match a[0].to_ascii_uppercase().as_str() {
                "DXF" => {
                    state.dxf = true;
                    state.dxf_options = values;
                }
                "DXF:WIREFRAME" => {
                    state.dxf_wireframe = true;
                    state.dxf_wireframe_options = values;
                }
                "VRML" => {
                    state.vrml = true;
                    state.vrml_options = values;
                }
                _ => {}
            }
        }
        let legacy_sections = match source.as_deref() {
            Some(source) => source
                .legacy_sections()
                .map_err(ReportScanFailure::Unavailable)?,
            None => input.legacy_variable_dictionary_sections,
        };
        if legacy_sections > 0 {
            state.variable_dictionary = true;
            state.variable_dictionary_options = ["REGULAR".to_string(), String::new()];
        }
        if let Some(source) = source.as_deref_mut() {
            source.enter_family(Kind::VariableDictionary);
        }
        for object in input
            .objects
            .iter()
            .filter(|o| o.object_type == Kind::VariableDictionary)
        {
            if let Some(source) = source.as_deref_mut() {
                source.get_object(object);
            }
            state.variable_dictionary = true;
            state.variable_dictionary_options =
                [object.alphas[0].clone(), object.alphas[1].clone()];
        }
        if let Some(source) = source.as_deref_mut() {
            source.enter_family(Kind::Constructions);
        }
        for object in input
            .objects
            .iter()
            .filter(|o| o.object_type == Kind::Constructions)
        {
            if let Some(source) = source.as_deref_mut() {
                source.get_object(object);
            }
            for alpha in &object.alphas[..if object.num_alphas > 1 { 2 } else { 1 }] {
                if alpha.eq_ignore_ascii_case("CONSTRUCTIONS") {
                    state.constructions = true;
                } else if alpha.eq_ignore_ascii_case("MATERIALS") {
                    state.materials = true;
                }
            }
        }
        if let Some(source) = source.as_deref_mut() {
            source.enter_family(Kind::EnergyManagementSystem);
        }
        for object in input
            .objects
            .iter()
            .filter(|o| o.object_type == Kind::EnergyManagementSystem)
        {
            if let Some(source) = source.as_deref_mut() {
                source.get_object(object);
            }
            state.ems_output = true;
            let a = &object.alphas;
            state.ems_actuator_availability = [
                a[0].eq_ignore_ascii_case("NOTBYUNIQUEKEYNAMES"),
                a[0].eq_ignore_ascii_case("VERBOSE"),
            ];
            state.ems_internal_variable_availability = [
                a[1].eq_ignore_ascii_case("NOTBYUNIQUEKEYNAMES"),
                a[1].eq_ignore_ascii_case("VERBOSE"),
            ];
            state.ems_errors =
                a[2].eq_ignore_ascii_case("ERRORSONLY") || a[2].eq_ignore_ascii_case("VERBOSE");
            state.ems_full_trace = a[2].eq_ignore_ascii_case("VERBOSE");
        }
        state.get_report_input = false;
    }

    request::select(state, report_name, report_key, option1, option2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ep_model::ReportInputObject;

    fn row(kind: Kind, alphas: [&str; 3], count: u8) -> ReportInputObject {
        ReportInputObject {
            object_name: "authored direct API input".to_string(),
            object_type: kind,
            alphas: alphas.map(str::to_string),
            alpha_blank: [false; 3],
            default_applied: [false; 3],
            num_alphas: count,
            is_epjson: true,
        }
    }

    #[test]
    fn first_scan_reads_real_a_families_then_later_queries_reuse_cache()
    -> Result<(), Box<dyn std::error::Error>> {
        let input = ReportInputs {
            objects: vec![
                row(Kind::VariableDictionary, ["IDF", "", ""], 1),
                row(
                    Kind::SurfacesDrawing,
                    ["DXF:WIREFRAME", "TRIANGULATE3DFACE", ""],
                    1,
                ),
                row(Kind::Constructions, ["CONSTRUCTIONS", "", ""], 1),
            ],
            ..ReportInputs::default()
        };
        let mut state = ReportScanState::default();
        assert_eq!(
            scan_for_reports(
                &input,
                &mut state,
                "Constructions",
                Some("Constructions"),
                None,
                None,
                |_| {}
            ),
            Ok(true)
        );
        assert!(!state.get_report_input);
        assert!(state.variable_dictionary && state.dxf_wireframe);
        let mut first = "keep".to_string();
        let mut second = "keep".to_string();
        assert_eq!(
            scan_for_reports(
                &ReportInputs::default(),
                &mut state,
                "Surfaces",
                Some("dxf:wireframe"),
                Some(&mut first),
                Some(&mut second),
                |_| {}
            ),
            Ok(true)
        );
        assert_eq!((first.as_str(), second.as_str()), ("TRIANGULATE3DFACE", ""));
        assert_eq!(state.variable_dictionary_options, ["IDF", ""]);

        // Same real cache body with a full shared getter owner. Building supplies
        // the actual five numeric scratch slots even though reports use alphas.
        use crate::heat_balance::selected_input_processor::{
            SelectedInputProcessor, SelectedInputUnavailable, SourceInputFamily,
            SourceInputProcessorShortCut, SourceObjectDefinition, SourceObjectShape,
        };
        use std::collections::BTreeMap;
        let source_input =
            |reports: &ReportInputs| -> Result<SelectedInputProcessor, SelectedInputUnavailable> {
                let mut definitions = Vec::new();
                let mut families = BTreeMap::new();
                for (kind, alphas) in [
                    (Kind::SurfacesList, 2),
                    (Kind::SurfacesDrawing, 3),
                    (Kind::VariableDictionary, 2),
                    (Kind::Constructions, 2),
                    (Kind::EnergyManagementSystem, 3),
                ] {
                    definitions.push(SourceObjectDefinition {
                        object_type: kind.source_name().into(),
                        alpha_fields: alphas,
                        numeric_fields: 0,
                        alpha_extensions: 0,
                        numeric_extensions: 0,
                        extension: None,
                    });
                    let objects: Vec<_> = reports
                        .objects
                        .iter()
                        .filter(|row| row.object_type == kind)
                        .map(|row| SourceObjectShape {
                            object_name: row.object_name.clone(),
                            field_sizes: BTreeMap::new(),
                        })
                        .collect();
                    if !objects.is_empty() {
                        families.insert(
                            kind.source_name().into(),
                            SourceInputFamily::Objects(objects),
                        );
                    }
                }
                definitions.push(SourceObjectDefinition {
                    object_type: "Building".into(),
                    alpha_fields: 3,
                    numeric_fields: 5,
                    alpha_extensions: 0,
                    numeric_extensions: 0,
                    extension: None,
                });
                families.insert(
                    "Building".into(),
                    SourceInputFamily::Objects(vec![SourceObjectShape {
                        object_name: "authored Building".into(),
                        field_sizes: BTreeMap::new(),
                    }]),
                );
                SelectedInputProcessor::from_admitted_input(definitions, families)
            };
        let mut reports = input.clone();
        for object in &mut reports.objects {
            object.alpha_blank = [false, true, true];
            if object.object_type == Kind::SurfacesDrawing {
                object.default_applied[1] = true;
            }
        }
        let mut source = source_input(&reports)?;
        assert_eq!(source.get_num_sections_found("Building")?, 1);
        assert_eq!(source.get_num_sections_found("building")?, -1);
        assert_eq!(
            source.get_num_sections_found("Output:EnergyManagementSystem")?,
            -1
        );
        source.get_object_def_max_args("Output:EnergyManagementSystem")?;
        assert_eq!(
            source.get_num_sections_found("Output:EnergyManagementSystem")?,
            0
        );
        assert_eq!(
            source.get_num_sections_found("Report Variable Dictionary")?,
            -1
        );
        let mut shortcut = SourceInputProcessorShortCut::default();
        shortcut.allocate_from_input(&source)?;
        assert_eq!(
            (shortcut.alpha_args.len(), shortcut.numeric_args.len()),
            (3, 5)
        );
        shortcut.alpha_args.fill("old alpha".into());
        shortcut.numeric_args.fill(-17.0);
        shortcut.alpha_field_names.fill("old alpha field".into());
        shortcut
            .numeric_field_names
            .fill("old numeric field".into());
        let mut reached = ReportScanState::default();
        {
            let mut getters = SourceReportScanInput::new(&mut source, &mut shortcut, &reports)
                .map_err(|_| "report getter association")?;
            let result = try_scan_for_reports_with_input(
                &reports,
                &mut reached,
                "Constructions",
                Some("Constructions"),
                None,
                None,
                &mut getters,
                |_| Ok::<(), &str>(()),
            );
            assert!(matches!(result, Ok(true)));
        }
        assert_eq!(reached, state);
        for object in &reports.objects {
            assert!(source.has_object(object.object_type.source_name(), &object.object_name));
            assert!(
                !source.is_object_unused(object.object_type.source_name(), &object.object_name)
            );
        }
        assert!(source.is_object_unused("Building", "authored Building"));
        assert!(!source.is_object_unused("building", "authored Building"));
        source.mark_object_as_used("Building", "not an actual object");
        assert!(source.is_object_unused("Building", "authored Building"));
        assert_eq!(
            shortcut.current_module_object,
            "Output:EnergyManagementSystem"
        );
        assert_eq!(shortcut.alpha_args, ["CONSTRUCTIONS", "", ""]);
        assert_eq!(
            shortcut.alpha_field_names,
            ["details_type_1", "details_type_2", ""]
        );
        assert_eq!(shortcut.alpha_field_blanks, [false, true, true]);
        assert!(
            shortcut
                .numeric_args
                .iter()
                .all(|value| value.to_bits() == 0.0_f64.to_bits())
        );
        assert_eq!(shortcut.numeric_field_names, [""; 5]);
        assert_eq!(shortcut.numeric_field_blanks, [true; 5]);
        shortcut.current_module_object = "retained cached caller".into();
        shortcut.alpha_args.fill("cached alpha".into());
        shortcut.numeric_args.fill(-0.0);
        shortcut.alpha_field_names.fill("cached alpha field".into());
        shortcut
            .numeric_field_names
            .fill("cached numeric field".into());
        shortcut.alpha_field_blanks.fill(false);
        shortcut.numeric_field_blanks.fill(false);
        let retained = shortcut.clone();
        let retained_input = source.clone();
        {
            let mut getters = SourceReportScanInput::new(&mut source, &mut shortcut, &reports)
                .map_err(|_| "report getter association")?;
            assert!(matches!(
                try_scan_for_reports_with_input(
                    &reports,
                    &mut reached,
                    "Constructions",
                    Some("Constructions"),
                    None,
                    None,
                    &mut getters,
                    |_| Err::<(), &str>("cached scan must not diagnose")
                ),
                Ok(true)
            ));
        }
        assert_eq!(shortcut, retained);
        assert_eq!(source, retained_input);
        assert!(
            shortcut
                .numeric_args
                .iter()
                .all(|value| value.to_bits() == (-0.0_f64).to_bits())
        );

        // A real warning transport failure stops before the next report family;
        // all six full scratch arrays and the reached family/cache prefix remain.
        let mut failing = reports.clone();
        failing
            .objects
            .insert(0, row(Kind::SurfacesList, ["UNRECOGNIZED", "", ""], 1));
        let mut source = source_input(&failing)?;
        let mut reached = ReportScanState::default();
        {
            let mut getters = SourceReportScanInput::new(&mut source, &mut shortcut, &failing)
                .map_err(|_| "report getter association")?;
            assert!(matches!(
                try_scan_for_reports_with_input(
                    &failing,
                    &mut reached,
                    "Constructions",
                    Some("Constructions"),
                    None,
                    None,
                    &mut getters,
                    |_| Err::<(), &str>("actual transport failure")
                ),
                Err(ReportScanFailure::Diagnostic("actual transport failure"))
            ));
        }
        assert!(reached.get_report_input && !reached.dxf_wireframe && !reached.constructions);
        assert!(!source.is_object_unused("Output:Surfaces:List", &failing.objects[0].object_name));
        for object in &failing.objects[1..] {
            assert!(source.is_object_unused(object.object_type.source_name(), &object.object_name));
        }
        assert_eq!(shortcut.current_module_object, "Output:Surfaces:List");
        assert_eq!(shortcut.alpha_args, ["UNRECOGNIZED", "", ""]);
        assert_eq!(
            shortcut.alpha_field_names,
            ["report_type", "report_specifications", ""]
        );
        assert_eq!(shortcut.alpha_field_blanks, [false, false, true]);
        assert_eq!(shortcut.numeric_args, [0.0; 5]);
        assert_eq!(shortcut.numeric_field_names, [""; 5]);
        assert_eq!(shortcut.numeric_field_blanks, [true; 5]);
        // An equal family count with an incorrect permanent name is rejected
        // before any scratch/unused mutation. Already-used correct names above
        // stayed valid for the cached scan because validation is not on unused.
        let before_input = source.clone();
        let before_scratch = shortcut.clone();
        let mut wrong_identity = failing.clone();
        wrong_identity.objects[0].object_name = "not present in the raw map".into();
        assert!(matches!(
            SourceReportScanInput::new(&mut source, &mut shortcut, &wrong_identity),
            Err(ReportScanUnavailable::SourceInputAssociation)
        ));
        assert_eq!(source, before_input);
        assert_eq!(shortcut, before_scratch);
        Ok(())
    }

    #[test]
    fn unknown_surface_list_token_uses_native_map_empty_warning_not_invalid() {
        let input = ReportInputs {
            objects: vec![row(Kind::SurfacesList, ["UNRECOGNIZED", "", ""], 1)],
            ..ReportInputs::default()
        };
        let mut messages = Vec::new();
        let mut state = ReportScanState::default();
        assert_eq!(
            scan_for_reports(
                &input,
                &mut state,
                "Constructions",
                Some("Constructions"),
                None,
                None,
                |message| messages.push(message)
            ),
            Ok(false)
        );
        assert_eq!(messages, vec![
            ReportScanDiagnostic::Warning("Output:Surfaces:List: No report_type supplied.".to_string()),
            ReportScanDiagnostic::Continue(" Legal values are: \"Lines\", \"Vertices\", \"Details\", \"DetailsWithVertices\", \"CostInfo\", \"ViewFactorIinfo\".".to_string()),
        ]);
        assert!(!state.get_report_input);
    }

    #[test]
    fn materials_only_second_alpha_count_and_optional_refs_remain_distinct() {
        let input = ReportInputs {
            objects: vec![row(
                Kind::Constructions,
                ["MATERIALS", "CONSTRUCTIONS", ""],
                1,
            )],
            ..ReportInputs::default()
        };
        let mut state = ReportScanState::default();
        let mut untouched = "caller option".to_string();
        assert_eq!(
            scan_for_reports(
                &input,
                &mut state,
                "Constructions",
                Some("Constructions"),
                Some(&mut untouched),
                None,
                |_| {}
            ),
            Ok(false)
        );
        assert!(state.materials);
        assert_eq!(untouched, "caller option");
        assert_eq!(
            scan_for_reports(
                &input,
                &mut state,
                "Constructions",
                Some("Materials"),
                None,
                None,
                |_| {}
            ),
            Ok(true)
        );
        let mut two_alpha = input;
        two_alpha.objects[0].num_alphas = 2;
        let mut fresh = ReportScanState::default();
        assert_eq!(
            scan_for_reports(
                &two_alpha,
                &mut fresh,
                "Constructions",
                Some("Constructions"),
                None,
                None,
                |_| {}
            ),
            Ok(true)
        );
    }

    #[test]
    fn last_ordered_options_and_ems_assignments_replace_earlier_values() {
        let input = ReportInputs {
            legacy_variable_dictionary_sections: 1,
            output_variables: Vec::new(),
            objects: vec![
                row(Kind::VariableDictionary, ["IDF", "NAME", ""], 2),
                row(Kind::VariableDictionary, ["REGULAR", "UNSORTED", ""], 2),
                row(
                    Kind::EnergyManagementSystem,
                    ["VERBOSE", "NOTBYUNIQUEKEYNAMES", "VERBOSE"],
                    3,
                ),
                row(
                    Kind::EnergyManagementSystem,
                    ["NONE", "NONE", "ERRORSONLY"],
                    3,
                ),
            ],
        };
        let mut state = ReportScanState::default();
        assert_eq!(
            scan_for_reports(
                &input,
                &mut state,
                "EnergyManagementSystem",
                None,
                None,
                None,
                |_| {}
            ),
            Ok(true)
        );
        assert_eq!(state.variable_dictionary_options, ["REGULAR", "UNSORTED"]);
        assert_eq!(state.ems_actuator_availability, [false; 2]);
        assert_eq!(state.ems_internal_variable_availability, [false; 2]);
        assert!(state.ems_errors);
        assert!(!state.ems_full_trace);
    }

    #[test]
    fn missing_surfaces_key_is_typed_unavailable_after_real_first_cache_read() {
        let mut state = ReportScanState::default();
        assert_eq!(
            scan_for_reports(
                &ReportInputs::default(),
                &mut state,
                "Surfaces",
                None,
                None,
                None,
                |_| {}
            ),
            Err(ReportScanUnavailable::MissingSurfacesKey)
        );
        assert!(!state.get_report_input);
        assert_eq!(
            scan_for_reports(
                &ReportInputs::default(),
                &mut state,
                "unknown",
                None,
                None,
                None,
                |_| {}
            ),
            Ok(false)
        );
    }
}
