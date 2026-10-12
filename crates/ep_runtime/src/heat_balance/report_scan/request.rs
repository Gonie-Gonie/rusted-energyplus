//! Unchanged cached report request selection; first-scan effects stay in the parent.
use super::{ReportScanFailure, ReportScanState, ReportScanUnavailable};

fn copy_options(a: &mut Option<&mut String>, b: &mut Option<&mut String>, values: &[String; 2]) {
    if let Some(target) = a.as_deref_mut() {
        target.clone_from(&values[0]);
    }
    if let Some(target) = b.as_deref_mut() {
        target.clone_from(&values[1]);
    }
}

pub(super) fn select<E>(
    state: &ReportScanState,
    report_name: &str,
    report_key: Option<&str>,
    mut option1: Option<&mut String>,
    mut option2: Option<&mut String>,
) -> Result<bool, ReportScanFailure<E>> {
    let mut do_report = false;
    match report_name.to_ascii_uppercase().as_str() {
        "CONSTRUCTIONS" => {
            if let Some(key) = report_key {
                if key.eq_ignore_ascii_case("Constructions") {
                    do_report = state.constructions;
                }
                if key.eq_ignore_ascii_case("Materials") {
                    do_report = state.materials;
                }
            }
        }
        "VIEWFACTORINFO" => {
            do_report = state.view_factor_info;
            if let Some(target) = option1 {
                target.clone_from(&state.view_factor_option);
            }
        }
        "VARIABLEDICTIONARY" => {
            do_report = state.variable_dictionary;
            copy_options(
                &mut option1,
                &mut option2,
                &state.variable_dictionary_options,
            );
        }
        "SURFACES" => match report_key
            .ok_or(ReportScanFailure::Unavailable(
                ReportScanUnavailable::MissingSurfacesKey,
            ))?
            .to_ascii_uppercase()
            .as_str()
        {
            "COSTINFO" => do_report = state.cost_info,
            "DXF" => {
                do_report = state.dxf;
                copy_options(&mut option1, &mut option2, &state.dxf_options);
            }
            "DXF:WIREFRAME" => {
                do_report = state.dxf_wireframe;
                copy_options(&mut option1, &mut option2, &state.dxf_wireframe_options);
            }
            "VRML" => {
                do_report = state.vrml;
                copy_options(&mut option1, &mut option2, &state.vrml_options);
            }
            "VERTICES" => do_report = state.surface_vertices,
            "DETAILS" => do_report = state.surface_details,
            "DETAILSWITHVERTICES" => do_report = state.surface_details_with_vertices,
            "LINES" => {
                do_report = state.lines;
                if let Some(target) = option1 {
                    target.clone_from(&state.line_option);
                }
            }
            _ => {}
        },
        "ENERGYMANAGEMENTSYSTEM" => do_report = state.ems_output,
        _ => {}
    }
    Ok(do_report)
}
