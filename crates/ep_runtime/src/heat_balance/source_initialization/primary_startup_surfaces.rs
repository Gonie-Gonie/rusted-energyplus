//! Actual OutputReports::ReportSurfaces on the retained report cache and geometry.
//! Optional unported exporters stop only at their reached native child call.
mod wireframe;
use super::SourceInitializationFiles;
use crate::RuntimeError;
use crate::heat_balance::ctf_init_postlude::CtfInitPostludeError;
use crate::heat_balance::ctf_initialization_context::{
    CtfInitializationError, CtfInitializationReports, hb_input_error,
};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::ordinary_daylighting::SourceDaylighting;
use crate::heat_balance::report_scan::{
    ReportScanDiagnostic, ReportScanFailure, SourceReportScanInput, try_scan_for_reports,
    try_scan_for_reports_with_input,
};
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_heat_balance_input::surface::{SurfaceClass, SurfaceTmpInput};
use ep_model::{
    InsideSurfaceConvectionAlgorithm as Inside, OutsideSurfaceConvectionAlgorithm as Outside,
    ReportInputs,
};
use std::io::Write;

// DataSurfaceColors.hh65,100: this is the actual module constructor array.
const DEFAULT_COLORS: [i32; 15] = [3, 43, 143, 143, 45, 8, 15, 195, 9, 13, 174, 143, 143, 10, 5];
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceSurfaceColors {
    pub(crate) dxf_color_numbers: [i32; 15],
}
impl Default for SourceSurfaceColors {
    fn default() -> Self {
        Self {
            dxf_color_numbers: DEFAULT_COLORS,
        }
    }
}
fn unavailable(reason: &'static str) -> RuntimeError {
    hb_input_error(Error::Unavailable(reason))
}
impl CtfInitializationReports {
    fn scan_source_surface_report(
        &mut self,
        input: &ReportInputs,
        key: &str,
        option1: Option<&mut String>,
        option2: Option<&mut String>,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<bool, RuntimeError> {
        let mut diagnostic = |message| match message {
            ReportScanDiagnostic::Warning(text) => backend.show_warning_error(&text),
            ReportScanDiagnostic::Continue(text) => backend.show_continue_error(&text),
        };
        // Source already-scanned calls do not replay first-cache input validation.
        let result = if self.scan.get_report_input {
            let input_processor = self
                .selected_input_processor
                .as_mut()
                .ok_or_else(|| unavailable("actual ReportSurfaces shared InputProcessor"))?;
            let mut source = SourceReportScanInput::new(
                input_processor,
                &mut self.input_processor_shortcut,
                input,
            )
            .map_err(|value| {
                RuntimeError::from(CtfInitializationError::Postlude(
                    CtfInitPostludeError::Scan(value),
                ))
            })?;
            try_scan_for_reports_with_input(
                input,
                &mut self.scan,
                "Surfaces",
                Some(key),
                option1,
                option2,
                &mut source,
                &mut diagnostic,
            )
        } else {
            try_scan_for_reports(
                input,
                &mut self.scan,
                "Surfaces",
                Some(key),
                option1,
                option2,
                &mut diagnostic,
            )
        };
        result.map_err(|error| {
            CtfInitializationError::Postlude(match error {
                ReportScanFailure::Unavailable(value) => CtfInitPostludeError::Scan(value),
                ReportScanFailure::Diagnostic(value) => CtfInitPostludeError::Backend(value),
            })
            .into()
        })
    }
    /// Actual SimulationManager289 child, using the sole shared report/IP owners.
    /// The drawing stream is freshly opened only inside its requested exporter.
    pub(crate) fn report_source_surfaces(
        &mut self,
        input: &ReportInputs,
        files: &mut SourceInitializationFiles,
        daylighting: &SourceDaylighting,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), RuntimeError> {
        backend.state.tracking.ask_for_surfaces_report = false;
        let mut surface_details = 0;
        let mut surface_vertices = false;
        let mut surface_detail = false;
        // Native true write143 is after the currently unsupported solid-DXF child.
        let dxf_done = false;
        let mut option1 = String::new();
        let mut option2 = String::new();
        if self.scan_source_surface_report(input, "Lines", Some(&mut option1), None, backend)? {
            return Err(unavailable(
                "actual OutputReports105 LinesOut183 optional exporter",
            ));
        }
        if self.scan_source_surface_report(input, "Vertices", None, None, backend)?
            && !surface_vertices
        {
            surface_details += 1;
            surface_vertices = true;
        }
        if self.scan_source_surface_report(input, "Details", None, None, backend)?
            && !surface_detail
        {
            surface_details += 10;
            surface_detail = true;
        }
        if self.scan_source_surface_report(input, "DetailsWithVertices", None, None, backend)? {
            if !surface_detail {
                surface_details += 10;
                surface_detail = true;
            }
            if !surface_vertices {
                surface_details += 1;
                surface_vertices = true;
            }
        }
        if self.scan_source_surface_report(
            input,
            "DXF",
            Some(&mut option1),
            Some(&mut option2),
            backend,
        )? {
            if !dxf_done {
                if !option2.is_empty() {
                    return Err(unavailable(
                        "actual ReportSurfaces140 SetUpSchemeColors optional child",
                    ));
                }
                return Err(unavailable(
                    "actual ReportSurfaces142 solid DXFOut463 child",
                ));
            } else {
                backend.show_warning_error(&format!("ReportSurfaces: DXF output already generated.  DXF with option=[{option1}] will not be generated."))?;
            }
        }
        if self.scan_source_surface_report(
            input,
            "DXF:WireFrame",
            Some(&mut option1),
            Some(&mut option2),
            backend,
        )? {
            if !dxf_done {
                if !option2.is_empty() {
                    return Err(unavailable(
                        "actual ReportSurfaces154 SetUpSchemeColors optional child",
                    ));
                }
                wireframe::write_source_dxf_wireframe(self, files, daylighting, &option2, backend)
                    .map_err(hb_input_error)?;
                // Native wireframe branch does not assign DXFDone.
            } else {
                backend.show_warning_error("ReportSurfaces: DXF output already generated.  DXF:WireFrame will not be generated.")?;
            }
        }
        if self.scan_source_surface_report(
            input,
            "VRML",
            Some(&mut option1),
            Some(&mut option2),
            backend,
        )? {
            return Err(unavailable(
                "actual ReportSurfaces166 VRMLOut1502 child (local VRMLDone is false)",
            ));
        }
        if self.scan_source_surface_report(input, "CostInfo", None, None, backend)? {
            return Err(unavailable(
                "actual ReportSurfaces175 CostInfoOut1432 child",
            ));
        }
        if surface_detail || surface_vertices {
            write_source_surface_details(self, surface_details, files.eio_stream())
                .map_err(hb_input_error)?;
        }
        Ok(())
    }
}

// Literal Original OutputReports1034 header strings, including its Vertex4 Z/Z typo.
const DETAILS_HEADER: &str = "! <Zone Surfaces>,Zone Name,# Surfaces\n! <Shading Surfaces>,Number of Shading Surfaces,# Surfaces\n! <HeatTransfer Surface>,Surface Name,Surface Class,Base Surface,Heat Transfer Algorithm,Construction,Nominal U (w/o film coefs) {W/m2-K},Nominal U (with film coefs) {W/m2-K},Solar Diffusing,Area (Net) {m2},Area (Gross) {m2},Area (Sunlit Calc) {m2},Azimuth {deg},Tilt {deg},~Width {m},~Height {m},Reveal {m},ExtBoundCondition,ExtConvCoeffCalc,IntConvCoeffCalc,SunExposure,WindExposure,ViewFactorToGround,ViewFactorToSky,ViewFactorToGround-IR,ViewFactorToSky-IR,#Sides\n! <Shading Surface>,Surface Name,Surface Class,Base Surface,Heat Transfer Algorithm,Transmittance Schedule,Min Schedule Value,Max Schedule Value,Solar Diffusing,Area (Net) {m2},Area (Gross) {m2},Area (Sunlit Calc) {m2},Azimuth {deg},Tilt {deg},~Width {m},~Height {m},Reveal {m},ExtBoundCondition,ExtConvCoeffCalc,IntConvCoeffCalc,SunExposure,WindExposure,ViewFactorToGround,ViewFactorToSky,ViewFactorToGround-IR,ViewFactorToSky-IR,#Sides\n! <Frame/Divider Surface>,Surface Name,Surface Class,Base Surface,Heat Transfer Algorithm,Construction,Nominal U (w/o film coefs) {W/m2-K},Nominal U (with film coefs) {W/m2-K},Solar Diffusing,Area (Net) {m2},Area (Gross) {m2},Area (Sunlit Calc) {m2},Azimuth {deg},Tilt {deg},~Width {m},~Height {m},Reveal {m}\n";
const DETAILS_VERTICES_HEADER: &str = "! <Zone Surfaces>,Zone Name,# Surfaces, Vertices are shown starting at Upper-Left-Corner => Counter-Clockwise => World Coordinates\n! <Shading Surfaces>,Number of Shading Surfaces,# Surfaces, Vertices are shown starting at Upper-Left-Corner => Counter-Clockwise => World Coordinates\n! <HeatTransfer Surface>,Surface Name,Surface Class,Base Surface,Heat Transfer Algorithm,Construction,Nominal U (w/o film coefs) {W/m2-K},Nominal U (with film coefs) {W/m2-K},Solar Diffusing,Area (Net) {m2},Area (Gross) {m2},Area (Sunlit Calc) {m2},Azimuth {deg},Tilt {deg},~Width {m},~Height {m},Reveal {m},ExtBoundCondition,ExtConvCoeffCalc,IntConvCoeffCalc,SunExposure,WindExposure,ViewFactorToGround,ViewFactorToSky,ViewFactorToGround-IR,ViewFactorToSky-IR,#Sides,Vertex 1 X {m},Vertex 1 Y {m},Vertex 1 Z {m},Vertex 2 X {m},Vertex 2 Y {m},Vertex 2 Z {m},Vertex 3 X {m},Vertex 3 Y {m},Vertex 3 Z {m},Vertex 4 X {m},Vertex 4 Z {m},Vertex 4 Z {m},{etc}\n! <Shading Surface>,Surface Name,Surface Class,Base Surface,Heat Transfer Algorithm,Transmittance Schedule,Min Schedule Value,Max Schedule Value,Solar Diffusing,Area (Net) {m2},Area (Gross) {m2},Area (Sunlit Calc) {m2},Azimuth {deg},Tilt {deg},~Width {m},~Height {m},Reveal {m},ExtBoundCondition,ExtConvCoeffCalc,IntConvCoeffCalc,SunExposure,WindExposure,ViewFactorToGround,ViewFactorToSky,ViewFactorToGround-IR,ViewFactorToSky-IR,#Sides,Vertex 1 X {m},Vertex 1 Y {m},Vertex 1 Z {m},Vertex 2 X {m},Vertex 2 Y {m},Vertex 2 Z {m},Vertex 3 X {m},Vertex 3 Y {m},Vertex 3 Z {m},Vertex 4 X {m},Vertex 4 Z {m},Vertex 4 Z {m},{etc}\n! <Frame/Divider Surface>,Surface Name,Surface Class,Base Surface,Heat Transfer Algorithm,Construction,Nominal U (w/o film coefs) {W/m2-K},Nominal U (with film coefs) {W/m2-K},Solar Diffusing,Area (Net) {m2},Area (Gross) {m2},Area (Sunlit Calc) {m2},Azimuth {deg},Tilt {deg},~Width {m},~Height {m},Reveal {m}\n";
const VERTICES_HEADER: &str = "! <Zone Surfaces>,Zone Name,# Surfaces, Vertices are shown starting at Upper-Left-Corner => Counter-Clockwise => World Coordinates\n! <Shading Surfaces>,Number of Shading Surfaces,# Surfaces, Vertices are shown starting at Upper-Left-Corner => Counter-Clockwise => World Coordinates\n! <HeatTransfer Surface>,Surface Name,Surface Class,Base Surface,Heat Transfer Algorithm,#Sides,Vertex 1 X {m},Vertex 1 Y {m},Vertex 1 Z {m},Vertex 2 X {m},Vertex 2 Y {m},Vertex 2 Z {m},Vertex 3 X {m},Vertex 3 Y {m},Vertex 3 Z {m},Vertex 4 X {m},Vertex 4 Z {m},Vertex 4 Z {m},{etc}\n! <Shading Surface>,Surface Name,Surface Class,Base Surface,Heat Transfer Algorithm,#Sides,Vertex 1 X {m},Vertex 1 Y {m},Vertex 1 Z {m},Vertex 2 X {m},Vertex 2 Y {m},Vertex 2 Z {m},Vertex 3 X {m},Vertex 3 Y {m},Vertex 3 Z {m},Vertex 4 X {m},Vertex 4 Z {m},Vertex 4 Z {m},{etc}\n";

fn surface_report_slot(number: i32) -> Result<usize, Error> {
    number
        .checked_sub(1)
        .and_then(|n| usize::try_from(n).ok())
        .ok_or(Error::Unavailable(
            "actual DetailsForSurfaces positive owner index",
        ))
}
fn surface_report_row(
    reports: &CtfInitializationReports,
    number: i32,
) -> Result<&SurfaceTmpInput, Error> {
    reports
        .surface_geometry_input
        .final_surfaces
        .as_ref()
        .and_then(|rows| {
            surface_report_slot(number)
                .ok()
                .and_then(|slot| rows.get(slot))
        })
        .map(|surface| &surface.row)
        .ok_or(Error::Unavailable(
            "actual DetailsForSurfaces Surface array",
        ))
}
fn surface_report_round(value: f64, precision: usize) -> Result<String, Error> {
    crate::heat_balance::source_round_format::round(value, precision)
        .map_err(|error| Error::Format(format!("DetailsForSurfaces .{precision}R: {error:?}")))
}
fn surface_report_algorithm(row: &SurfaceTmpInput) -> Result<&'static str, Error> {
    match row.heat_transfer_algorithm {
        crate::heat_balance::selected_heat_balance_input::SelectedHeatTransferModel::Ctf => {
            Ok("CTF - ConductionTransferFunction")
        }
        _ => Err(Error::Unavailable(
            "actual DetailsForSurfaces unwritten/unsupported HeatTransferAlgorithm",
        )),
    }
}
fn surface_report_class(row: &SurfaceTmpInput) -> Result<&'static str, Error> {
    match row.class {
        SurfaceClass::Wall => Ok("Wall"),
        SurfaceClass::Floor => Ok("Floor"),
        SurfaceClass::Roof => Ok("Roof"),
        _ => Err(Error::Unavailable(
            "actual DetailsForSurfaces positive nonopaque/window class child",
        )),
    }
}
fn surface_report_inside(algorithm: Inside) -> Result<&'static str, Error> {
    match algorithm {
        Inside::Simple => Ok("ASHRAESimple"),
        Inside::Tarp => Ok("ASHRAETARP"),
        Inside::CeilingDiffuser => Ok("CeilingDiffuser"),
        Inside::TrombeWall => Ok("TrombeWall"),
        Inside::AdaptiveConvectionAlgorithm => Ok("AdaptiveConvectionAlgorithm"),
        Inside::AstmC1340 => Err(Error::Unavailable(
            "Original DetailsForSurfaces ASTMC1340 report index exceeds ConvCoeffCalcs",
        )),
    }
}
fn surface_report_outside(algorithm: Outside) -> &'static str {
    match algorithm {
        Outside::SimpleCombined => "ASHRAESimple",
        Outside::Tarp => "TARP",
        Outside::MoWitt => "MoWitt",
        Outside::Doe2 => "DOE-2",
        Outside::AdaptiveConvectionAlgorithm => "AdaptiveConvectionAlgorithm",
    }
}
fn surface_report_convection(
    reports: &CtfInitializationReports,
    number: i32,
    inside: &'static str,
    outside: &'static str,
) -> Result<(&'static str, &'static str), Error> {
    let index = surface_report_slot(number)?;
    let arrays = reports
        .surface_geometry_input
        .parallel_arrays
        .as_ref()
        .ok_or(Error::Unavailable(
            "actual DetailsForSurfaces surface convection arrays",
        ))?;
    let internal = arrays
        .interior_convection
        .get(index)
        .ok_or(Error::Unavailable(
            "actual DetailsForSurfaces surfIntConv row",
        ))?;
    let external = arrays
        .exterior_convection
        .get(index)
        .ok_or(Error::Unavailable(
            "actual DetailsForSurfaces surfExtConv row",
        ))?;
    if internal.user_model_num != 0 || external.user_model_num != 0 {
        return Err(Error::Unavailable(
            "actual DetailsForSurfaces positive user convection model owner",
        ));
    }
    // Literal HcInt/ExtReportVals then ConvCoeffCalcs lookup; ordinal2 is SetByZone.
    let inside = match internal.model {
        2 => inside,
        3 => "ASHRAESimple",
        4 => "ASHRAETARP",
        5 => "CeilingDiffuser",
        6 => "TrombeWall",
        7 => "AdaptiveConvectionAlgorithm",
        _ => {
            return Err(Error::Unavailable(
                "actual DetailsForSurfaces inside model report lookup",
            ));
        }
    };
    let outside = match external.model {
        2 => outside,
        3 => "ASHRAESimple",
        4 => "ASHRAETARP",
        5 => "TARP",
        6 => "MoWitt",
        7 => "DOE-2",
        8 => "BLAST",
        9 => "AdaptiveConvectionAlgorithm",
        _ => {
            return Err(Error::Unavailable(
                "actual DetailsForSurfaces outside model report lookup",
            ));
        }
    };
    Ok((inside, outside))
}
fn surface_report_construction(
    reports: &CtfInitializationReports,
    row: &SurfaceTmpInput,
) -> Result<(String, String, String), Error> {
    let control = &reports.heat_balance_input_state;
    if row.construction <= 0
        || usize::try_from(row.construction)
            .ok()
            .is_none_or(|number| number > control.total_constructions)
    {
        return Ok(("**invalid**".into(), "**".into(), "**".into()));
    }
    let index = surface_report_slot(row.construction)?;
    let name = &control
        .construction_usage
        .get(index)
        .ok_or(Error::Unavailable(
            "actual DetailsForSurfaces Construction.Name",
        ))?
        .name;
    let nominal = *control
        .nominal_u
        .as_ref()
        .and_then(|values| values.get(index))
        .ok_or(Error::Unavailable("actual DetailsForSurfaces NominalU"))?;
    let with_films = if nominal > 0.0 {
        let (inside, outside) = match row.class {
            SurfaceClass::Wall => (0.1197548, 0.0299387),
            SurfaceClass::Floor => (0.1620212, 0.0810106),
            SurfaceClass::Roof => (0.1074271, 0.0810106),
            _ => {
                return Err(Error::Unavailable(
                    "actual DetailsForSurfaces nonopaque nominal-U child",
                ));
            }
        };
        // Original OutputReports uses class-only films even for Ground/Adiabatic.
        surface_report_round(1.0 / (inside + (1.0 / nominal) + outside), 3)?
    } else {
        "[invalid]".into()
    };
    Ok((name.clone(), surface_report_round(nominal, 3)?, with_films))
}
fn surface_report_boundary<'a>(
    row: &'a SurfaceTmpInput,
    outside: &'a str,
) -> Result<(&'a str, &'a str), Error> {
    match row.exterior_boundary {
        0 => Ok(("ExternalEnvironment", outside)),
        -1 => Ok(("Ground", "N/A-Ground")),
        -5 => Ok(("FCGround", "N/A-FCGround")),
        -6 => Ok(("Foundation", "N/A-Foundation")),
        -4..=-2 => Err(Error::Unavailable(
            "actual DetailsForSurfaces positive OSC/OSCM name owner",
        )),
        _ => Ok((&row.exterior_boundary_name, "Other/Same Surface Int Conv")),
    }
}
fn surface_report_vertices(output: &mut Vec<u8>, row: &SurfaceTmpInput) -> Result<(), Error> {
    for number in 1..=row.sides {
        let vertex = row
            .vertex
            .as_ref()
            .and_then(|rows| surface_report_slot(number).ok().and_then(|at| rows.get(at)))
            .and_then(Option::as_ref)
            .ok_or(Error::Unavailable(
                "actual DetailsForSurfaces initialized Surface.Vertex",
            ))?;
        write!(
            output,
            "{},{},{}{}",
            surface_report_round(vertex[0], 2)?,
            surface_report_round(vertex[1], 2)?,
            surface_report_round(vertex[2], 2)?,
            if number == row.sides { "\n" } else { "," }
        )?;
    }
    if row.sides == 0 {
        output.write_all(b"\n")?;
    }
    Ok(())
}
/// Actual OutputReports1034-1430 opaque child over borrowed Source owners.
/// Unit callers construct the same owner types; this is not simulation activation.
/// The Native stringstream is buffered in full before one actual EIO write.
pub(crate) fn write_source_surface_details(
    reports: &CtfInitializationReports,
    report_type: i32,
    eio: &mut dyn Write,
) -> Result<(), Error> {
    let geometry = &reports.surface_geometry_input;
    if geometry.total_surfaces > 0 && geometry.final_surfaces.is_none() {
        return Ok(());
    }
    let header = match report_type {
        1 => VERTICES_HEADER,
        10 => DETAILS_HEADER,
        11 => DETAILS_VERTICES_HEADER,
        _ => {
            return Err(Error::Unavailable(
                "actual ReportSurfaces admits RptType1/10/11 only",
            ));
        }
    };
    let mut output = Vec::new();
    output.write_all(header.as_bytes())?;
    let mut surface2 = 0;
    for &number in &geometry.all_surface_list_report_order {
        surface2 = number;
        if surface_report_row(reports, number)?.zone != 0 {
            break;
        }
    }
    if surface2 - 1 > 0 {
        return Err(Error::Unavailable(
            "actual DetailsForSurfaces detached shading schedule/report child",
        ));
    }
    let zones = reports
        .selected_heat_balance_globals
        .as_ref()
        .ok_or(Error::Unavailable("actual DetailsForSurfaces NumOfZones"))?
        .num_of_zones;
    for zone_number in 1..=zones {
        let zone = reports
            .selected_zone_input
            .as_ref()
            .and_then(|owner| owner.zones.as_ref())
            .and_then(|rows| {
                surface_report_slot(zone_number)
                    .ok()
                    .and_then(|index| rows.get(index))
            })
            .ok_or(Error::Unavailable("actual DetailsForSurfaces Zone array"))?;
        writeln!(
            output,
            "Zone Surfaces,{},{}",
            zone.name,
            zone.all_surface_last - zone.all_surface_first + 1
        )?;
        for &number in &geometry.all_surface_list_report_order {
            let row = surface_report_row(reports, number)?;
            if row.zone != zone_number {
                continue;
            }
            let base = if row.base_surface == number {
                ""
            } else {
                row.base_surface_name.as_str()
            };
            let algorithm = surface_report_algorithm(row)?;
            let class = surface_report_class(row)?;
            if report_type == 10 || report_type == 11 {
                // Original performs the Zone lookups before the surface overrides.
                let inside = surface_report_inside(zone.inside_algorithm)?;
                let outside = surface_report_outside(zone.outside_algorithm);
                write!(
                    output,
                    "HeatTransfer Surface,{},{},{},{},",
                    row.name, class, base, algorithm
                )?;
                let (construction, nominal, films) = surface_report_construction(reports, row)?;
                let properties =
                    row.vertex_progress
                        .properties
                        .as_ref()
                        .ok_or(Error::Unavailable(
                            "actual DetailsForSurfaces written Azimuth/Tilt",
                        ))?;
                write!(
                    output,
                    "{construction},{nominal},{films},,{},{},{},{},{},{},{},{},",
                    surface_report_round(row.area, 2)?,
                    surface_report_round(row.gross_area, 2)?,
                    surface_report_round(row.net_area_shadow_calc, 2)?,
                    surface_report_round(properties.azimuth_deg, 2)?,
                    surface_report_round(properties.tilt_deg, 2)?,
                    surface_report_round(row.vertex_progress.width, 2)?,
                    surface_report_round(row.vertex_progress.height, 2)?,
                    surface_report_round(row.reveal, 2)?
                )?;
                let (inside, outside) =
                    surface_report_convection(reports, number, inside, outside)?;
                let (boundary, outside) = surface_report_boundary(row, outside)?;
                write!(
                    output,
                    "{boundary},{outside},{inside},{},{},{},{},{},{},{}{}",
                    if row.exterior_solar {
                        "SunExposed"
                    } else {
                        "NoSun"
                    },
                    if row.exterior_wind {
                        "WindExposed"
                    } else {
                        "NoWind"
                    },
                    surface_report_round(row.view_factor_ground, 2)?,
                    surface_report_round(row.vertex_progress.view_factor_sky, 2)?,
                    surface_report_round(row.vertex_progress.view_factor_ground_ir, 2)?,
                    surface_report_round(row.vertex_progress.view_factor_sky_ir, 2)?,
                    row.sides,
                    if report_type == 10 { "\n" } else { "," }
                )?;
                if report_type == 11 {
                    surface_report_vertices(&mut output, row)?;
                }
                if row.frame_divider > 0 {
                    return Err(Error::Unavailable(
                        "actual DetailsForSurfaces positive window frame/divider child",
                    ));
                }
            } else {
                write!(
                    output,
                    "HeatTransfer Surface,{},{},{},{},{},",
                    row.name, class, base, algorithm, row.sides
                )?;
                surface_report_vertices(&mut output, row)?;
            }
        }
    }
    eio.write_all(&output)?;
    Ok(())
}
