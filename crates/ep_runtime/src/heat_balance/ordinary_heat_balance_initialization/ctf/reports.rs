//! Actual6166–6202 report/Scan/fatal suffix over the same returned stores.
use super::*;
use crate::heat_balance::ctf_reports::{
    CtfConstructionReport, CtfMaterialReport, CtfReportError, CtfReportUnavailable,
    report_ctf_layers, report_ctf_transfer_function, write_ctf_report_headers,
};
use crate::heat_balance::report_scan::{
    ReportScanDiagnostic, ReportScanFailure, SourceReportScanInput, try_scan_for_reports_with_input,
};

fn prepare<'a>(
    state: &'a SourceCtfInitialization,
    model: &'a TypedModel,
    control: &'a SelectedHeatBalanceInputState,
    materials: &'a SelectedMaterialData,
    index: usize,
) -> Result<CtfConstructionReport<'a>, CtfReportUnavailable> {
    let usage = control
        .construction_usage
        .get(index)
        .ok_or(CtfReportUnavailable::Association)?;
    let row = state
        .public
        .constructions
        .get(index)
        .ok_or(CtfReportUnavailable::PublicOwnerUnavailable)?;
    if usage.construction_id != Some(row.construction_id)
        || row.declared_index != index
        || usage.is_used_ctf != row.is_used_ctf
    {
        return Err(CtfReportUnavailable::Association);
    }
    let returned = row
        .result
        .as_ref()
        .map_err(|_| CtfReportUnavailable::PublicOwnerUnavailable)?;
    let mut layers = Vec::new();
    let count =
        usize::try_from(usage.total_layers).map_err(|_| CtfReportUnavailable::Association)?;
    let points = usage
        .layer_points
        .get(..count)
        .ok_or(CtfReportUnavailable::Association)?;
    for &number in points {
        let index = slot(number).map_err(|_| CtfReportUnavailable::Association)?;
        let id = *materials
            .materials
            .get(index)
            .ok_or(CtfReportUnavailable::Association)?;
        let material = model
            .materials
            .iter()
            .find(|v| v.id == id)
            .ok_or(CtfReportUnavailable::Material(id))?;
        let air_gap = match (materials.material_groups.get(&id), material.definition) {
            (Some(SourceMaterialGroup::Regular), MaterialDefinition::Regular(_))
            | (Some(SourceMaterialGroup::Regular), MaterialDefinition::NoMass(_)) => false,
            (Some(SourceMaterialGroup::AirGap), MaterialDefinition::AirGap(_)) => true,
            _ => return Err(CtfReportUnavailable::Material(id)),
        };
        let input =
            material_layer_inputs(&[material]).map_err(|_| CtfReportUnavailable::Material(id))?;
        let values = input.first().ok_or(CtfReportUnavailable::Material(id))?;
        layers.push(CtfMaterialReport {
            material_id: id,
            name: crate::heat_balance::ctf_initialization_context::original_material_name(
                model, id,
            )?,
            air_gap,
            values: [
                values.thickness,
                values.conductivity,
                values.density,
                values.specific_heat,
                values.resistance,
            ],
        });
    }
    Ok(CtfConstructionReport {
        construction_id: row.construction_id,
        declared_counter: i32::try_from(index + 1)
            .map_err(|_| CtfReportUnavailable::DeclaredCounter)?,
        name: &usage.name,
        layers,
        public: &returned.storage,
        absorptance: [
            usage.outside_absorp_thermal,
            usage.inside_absorp_thermal,
            usage.outside_absorp_solar,
            usage.inside_absorp_solar,
        ],
        outside_roughness: usage
            .outside_roughness
            .ok_or(CtfReportUnavailable::Association)?,
    })
}
pub(super) fn complete(
    state: &mut SourceCtfInitialization,
    owners: &mut SourceCtfInitializationOwners<'_, '_>,
) -> Result<(), CtfInitPostludeError> {
    if owners.control.any_internal_heat_source_in_input {
        state.public.aggregate_after_returned_calls.simple_ctf_only = false;
    }
    for index in 0..owners.control.construction_usage.len() {
        if !owners.control.construction_usage[index].is_used_ctf {
            continue;
        }
        let report = prepare(state, owners.model, owners.control, owners.materials, index)?;
        report_ctf_layers(
            &report,
            owners.registry.opaque_layer_columns(),
            owners.table,
        )?;
        state.progress.layer_report_declared_indices.push(index);
    }
    let mut source =
        SourceReportScanInput::new(owners.input, owners.shortcut, owners.report_inputs)
            .map_err(CtfInitPostludeError::Scan)?;
    let do_report = try_scan_for_reports_with_input(
        owners.report_inputs,
        owners.scan,
        "Constructions",
        Some("Constructions"),
        None,
        None,
        &mut source,
        &mut |diagnostic| match diagnostic {
            ReportScanDiagnostic::Warning(message) => owners.backend.show_warning_error(&message),
            ReportScanDiagnostic::Continue(message) => owners.backend.show_continue_error(&message),
        },
    )
    .map_err(|e| match e {
        ReportScanFailure::Unavailable(v) => CtfInitPostludeError::Scan(v),
        ReportScanFailure::Diagnostic(v) => CtfInitPostludeError::Backend(v),
    })?;
    state.progress.construction_scan_result = Some(do_report);
    if do_report || state.shared_flags.do_ctf_error_report {
        write_ctf_report_headers(owners.eio).map_err(CtfReportError::Io)?;
        state.progress.report_headers_written = true;
        for index in 0..owners.control.construction_usage.len() {
            if !owners.control.construction_usage[index].is_used_ctf {
                continue;
            }
            let report = prepare(state, owners.model, owners.control, owners.materials, index)?;
            report_ctf_transfer_function(&report, owners.eio)?;
            state
                .progress
                .coefficient_report_declared_indices
                .push(index);
        }
    }
    if state.shared_flags.errors_found {
        state.progress.final_fatal_invoked = true;
        return match owners.backend.show_fatal_error(
            "Program terminated for reasons listed (InitConductionTransferFunctions)",
        ) {
            Ok(never) => match never {},
            Err(error) => Err(CtfInitPostludeError::Fatal(error)),
        };
    }
    state.progress.postlude_returned = true;
    Ok(())
}
