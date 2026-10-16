//! Actual selected opaque GatherForPredefinedReport, HBSM623-1404.
//! Every reached A table/count/query write is retained in its real call order.
use crate::heat_balance::ctf_reports::{CtfReportUnavailable, PredefinedStringTable};
use crate::heat_balance::predefined_tables::PredefinedRegistry;
use crate::heat_balance::report_scan::ReportScanState;
use crate::heat_balance::selected_heat_balance_input::SelectedHeatBalanceInputState;
use crate::heat_balance::selected_heat_balance_input::surface::{
    SurfaceClass, SurfaceGeometryInputState,
};
use crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneInputOwners;
use crate::heat_balance::selected_input_processor::SelectedInputProcessor;
use crate::heat_balance::selected_material_data::SelectedMaterialData;

/// Actual predefined-table failure or an unsupported Source operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceSurfacePredefinedError {
    /// Retained predefined-table failure.
    Table(CtfReportUnavailable),
    /// First Source operation unavailable for the selected route.
    Unavailable(&'static str),
}
impl From<CtfReportUnavailable> for SourceSurfacePredefinedError {
    fn from(error: CtfReportUnavailable) -> Self {
        Self::Table(error)
    }
}
impl std::fmt::Display for SourceSurfacePredefinedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for SourceSurfacePredefinedError {}
use SourceSurfacePredefinedError as Error;

fn index(number: i32) -> Result<usize, Error> {
    number
        .checked_sub(1)
        .and_then(|number| usize::try_from(number).ok())
        .ok_or(Error::Unavailable(
            "actual positive Source predefined ordinal",
        ))
}
fn column(registry: &PredefinedRegistry, name: &str) -> Result<i32, Error> {
    registry
        .named_indices
        .get(name)
        .copied()
        .ok_or(Error::Unavailable(
            "actual retained predefined named column",
        ))
}
fn string(
    table: &mut PredefinedStringTable,
    registry: &PredefinedRegistry,
    name: &str,
    key: &str,
    value: &str,
) -> Result<(), Error> {
    table
        .string_entry(column(registry, name)?, key, value)
        .map_err(Into::into)
}
fn real(
    table: &mut PredefinedStringTable,
    registry: &PredefinedRegistry,
    name: &str,
    key: &str,
    value: f64,
) -> Result<(), Error> {
    table
        .fixed_real_entry_2(column(registry, name)?, key, value)
        .map_err(Into::into)
}
fn real3(
    table: &mut PredefinedStringTable,
    registry: &PredefinedRegistry,
    name: &str,
    key: &str,
    value: f64,
) -> Result<(), Error> {
    table
        .fixed_real_entry_3(column(registry, name)?, key, value)
        .map_err(Into::into)
}
fn integer(
    table: &mut PredefinedStringTable,
    registry: &PredefinedRegistry,
    name: &str,
    key: &str,
    value: i32,
) -> Result<(), Error> {
    table
        .integer_entry(column(registry, name)?, key, value)
        .map_err(Into::into)
}
fn exterior(boundary: i32) -> bool {
    matches!(boundary, 0 | -1 | -5 | -6)
}
fn class_number(class: SurfaceClass) -> Result<usize, Error> {
    Ok(match class {
        SurfaceClass::Wall => 1,
        SurfaceClass::Floor => 2,
        SurfaceClass::Roof => 3,
        SurfaceClass::IntMass => 4,
        SurfaceClass::DetachedBuilding => 5,
        SurfaceClass::DetachedFixed => 6,
        SurfaceClass::Window => 7,
        SurfaceClass::GlassDoor => 8,
        SurfaceClass::Door => 9,
        SurfaceClass::Shading => 10,
        SurfaceClass::TddDome => 13,
        SurfaceClass::TddDiffuser => 14,
        SurfaceClass::None | SurfaceClass::Invalid => {
            return Err(Error::Unavailable(
                "Source predefined SurfaceClass assertion",
            ));
        }
    })
}
fn direction(azimuth: f64) -> Option<&'static str> {
    if azimuth >= 315.0 || azimuth < 45.0 {
        Some("N")
    } else if azimuth >= 45.0 && azimuth < 135.0 {
        Some("E")
    } else if azimuth >= 135.0 && azimuth < 225.0 {
        Some("S")
    } else if azimuth >= 225.0 && azimuth < 315.0 {
        Some("W")
    } else {
        None
    }
}

pub(crate) fn gather_source_surface_predefined_report(
    geometry: &SurfaceGeometryInputState,
    constructions: &SelectedHeatBalanceInputState,
    zones: &SelectedZoneInputOwners,
    materials: &SelectedMaterialData,
    scan: &ReportScanState,
    registry: &PredefinedRegistry,
    table: &mut PredefinedStringTable,
    input: &SelectedInputProcessor,
) -> Result<(), Error> {
    let total = usize::try_from(geometry.total_surfaces)
        .map_err(|_| Error::Unavailable("actual TotSurfaces"))?;
    let mut num_surfaces = [0_i32; 15];
    let mut num_exterior_surfaces = [0_i32; 15];
    let mut net_area = vec![0.0; total];
    if materials.total_frame_divider > 0 && scan.constructions {
        return Err(Error::Unavailable(
            "positive actual FenestrationAssembly EIO header",
        ));
    }
    let surfaces = geometry
        .final_surfaces
        .as_ref()
        .ok_or(Error::Unavailable("same final Source Surface owner"))?;
    let zone_rows = zones
        .zones
        .as_ref()
        .ok_or(Error::Unavailable("same Source Zone owner"))?;
    let spaces = zones
        .spaces
        .as_ref()
        .ok_or(Error::Unavailable("same Source Space owner"))?;
    for &number in &geometry.all_surface_list_report_order {
        let i = index(number)?;
        let surface = surfaces.get(i).ok_or(Error::Unavailable(
            "actual AllSurfaceListReportOrder ordinal",
        ))?;
        let row = &surface.row;
        let is_exterior = exterior(row.exterior_boundary);
        let opaque = if is_exterior {
            matches!(
                row.class,
                SurfaceClass::Wall | SurfaceClass::Floor | SurfaceClass::Roof
            )
        } else {
            matches!(
                row.class,
                SurfaceClass::Wall
                    | SurfaceClass::Floor
                    | SurfaceClass::Roof
                    | SurfaceClass::IntMass
            )
        };
        if opaque {
            let construction = constructions
                .construction_usage
                .get(index(row.construction)?)
                .ok_or(Error::Unavailable("actual predefined Construction owner"))?;
            let zone = zone_rows
                .get(index(row.zone)?)
                .ok_or(Error::Unavailable("actual predefined Zone ordinal"))?;
            let multiplier = f64::from(
                zone.multiplier
                    .checked_mul(zone.list_multiplier)
                    .ok_or(Error::Unavailable("Source signed Zone multiplier product"))?,
            );
            let space = spaces
                .get(index(row.space)?)
                .ok_or(Error::Unavailable("actual predefined Space ordinal"))?;
            let p = row
                .vertex_progress
                .properties
                .as_ref()
                .ok_or(Error::Unavailable(
                    "actual GetVertices GrossArea/Azimuth/Tilt fields",
                ))?;
            if is_exterior {
                string(table, registry, "pdchOpCons", &row.name, &construction.name)?;
                string(table, registry, "pdchOpZone", &row.name, &zone.name)?;
                string(table, registry, "pdchOpSpace", &row.name, &space.name)?;
                real(
                    table,
                    registry,
                    "pdchOpRefl",
                    &row.name,
                    1.0 - construction.outside_absorp_solar,
                )?;
                let nominal_u = *constructions
                    .nominal_u
                    .as_ref()
                    .and_then(|values| values.get(index(row.construction).ok()?))
                    .ok_or(Error::Unavailable("same actual NominalU allocation"))?;
                real3(table, registry, "pdchOpUfactNoFilm", &row.name, nominal_u)?;
                real(
                    table,
                    registry,
                    "pdchOpGrArea",
                    &row.name,
                    p.gross_area_m2 * multiplier,
                )?;
                *net_area
                    .get_mut(i)
                    .ok_or(Error::Unavailable("actual computedNetArea allocation"))? +=
                    p.gross_area_m2 * multiplier;
                let azimuth = (p.azimuth_deg * 100.0).round() / 100.0;
                real(table, registry, "pdchOpAzimuth", &row.name, azimuth)?;
                let tilt = p.tilt_deg;
                real(table, registry, "pdchOpTilt", &row.name, tilt)?;
                if tilt >= 60.0 && tilt < 180.0 {
                    if let Some(dir) = direction(azimuth) {
                        string(table, registry, "pdchOpDir", &row.name, dir)?;
                    }
                }
            } else {
                string(
                    table,
                    registry,
                    "pdchIntOpCons",
                    &row.name,
                    &construction.name,
                )?;
                string(table, registry, "pdchIntOpZone", &row.name, &zone.name)?;
                string(table, registry, "pdchIntOpSpace", &row.name, &space.name)?;
                string(
                    table,
                    registry,
                    "pdchIntOpAdjSurf",
                    &row.name,
                    &row.exterior_boundary_name,
                )?;
                real(
                    table,
                    registry,
                    "pdchIntOpRefl",
                    &row.name,
                    1.0 - construction.outside_absorp_solar,
                )?;
                let nominal_u = *constructions
                    .nominal_u
                    .as_ref()
                    .and_then(|values| values.get(index(row.construction).ok()?))
                    .ok_or(Error::Unavailable("same actual NominalU allocation"))?;
                real3(
                    table,
                    registry,
                    "pdchIntOpUfactNoFilm",
                    &row.name,
                    nominal_u,
                )?;
                real(
                    table,
                    registry,
                    "pdchIntOpGrArea",
                    &row.name,
                    p.gross_area_m2 * multiplier,
                )?;
                *net_area
                    .get_mut(i)
                    .ok_or(Error::Unavailable("actual computedNetArea allocation"))? +=
                    p.gross_area_m2 * multiplier;
                let azimuth = (p.azimuth_deg * 100.0).round() / 100.0;
                real(table, registry, "pdchIntOpAzimuth", &row.name, azimuth)?;
                let tilt = p.tilt_deg;
                real(table, registry, "pdchIntOpTilt", &row.name, tilt)?;
                if tilt >= 60.0 && tilt < 180.0 {
                    if let Some(dir) = direction(azimuth) {
                        string(table, registry, "pdchIntOpDir", &row.name, dir)?;
                    }
                }
            }
        } else if matches!(
            row.class,
            SurfaceClass::Window | SurfaceClass::TddDome | SurfaceClass::Door
        ) {
            return Err(Error::Unavailable(
                "positive actual predefined window/door reporting branch",
            ));
        }
        let class = class_number(row.class)?;
        num_surfaces[class] = num_surfaces[class]
            .checked_add(1)
            .ok_or(Error::Unavailable("Source surface count increment"))?;
        if is_exterior {
            num_exterior_surfaces[class] =
                num_exterior_surfaces[class]
                    .checked_add(1)
                    .ok_or(Error::Unavailable(
                        "Source exterior surface count increment",
                    ))?;
        }
        if row.class == SurfaceClass::Window
            && matches!(
                surface.original_class,
                SurfaceClass::GlassDoor | SurfaceClass::TddDiffuser
            )
        {
            let original = class_number(surface.original_class)?;
            num_surfaces[original] =
                num_surfaces[original]
                    .checked_add(1)
                    .ok_or(Error::Unavailable(
                        "Source original-class surface count increment",
                    ))?;
            if is_exterior {
                num_exterior_surfaces[original] = num_exterior_surfaces[original]
                    .checked_add(1)
                    .ok_or(Error::Unavailable(
                        "Source original-class exterior count increment",
                    ))?;
            }
        }
    }
    // Four genuine getNumObjectsFound calls, in Source1261-1266 order.
    let overhang = input.get_num_objects_found("Shading:Overhang");
    let overhang_projection = input.get_num_objects_found("Shading:Overhang:Projection");
    let overhangs = i32::try_from(
        overhang
            .checked_add(overhang_projection)
            .ok_or(Error::Unavailable("actual Overhang count sum"))?,
    )
    .map_err(|_| Error::Unavailable("actual Overhang signed count"))?;
    num_surfaces[11] = overhangs;
    num_exterior_surfaces[11] = overhangs;
    let fin = input.get_num_objects_found("Shading:Fin");
    let fin_projection = input.get_num_objects_found("Shading:Fin:Projection");
    let fins = i32::try_from(
        fin.checked_add(fin_projection)
            .ok_or(Error::Unavailable("actual Fin count sum"))?,
    )
    .map_err(|_| Error::Unavailable("actual Fin signed count"))?;
    num_surfaces[12] = fins;
    num_exterior_surfaces[12] = fins;
    for &number in &geometry.all_surface_list_report_order {
        let i = index(number)?;
        let surface = surfaces
            .get(i)
            .ok_or(Error::Unavailable("actual net-area report ordinal"))?;
        if matches!(
            surface.row.class,
            SurfaceClass::Wall | SurfaceClass::Floor | SurfaceClass::Roof
        ) {
            let name = if exterior(surface.row.exterior_boundary) {
                "pdchOpNetArea"
            } else {
                "pdchIntOpNetArea"
            };
            real(
                table,
                registry,
                name,
                &surface.row.name,
                *net_area
                    .get(i)
                    .ok_or(Error::Unavailable("same computedNetArea allocation"))?,
            )?;
        }
    }
    // No positive Window branch was reached, so these literal Source local
    // accumulators remain0.0. Every genuine zero-fenestration row is still written.
    for key in [
        "Total or Average",
        "North Total or Average",
        "Non-North Total or Average",
    ] {
        real(table, registry, "pdchFenArea", key, 0.0)?;
        string(table, registry, "pdchFenUfact", key, "-")?;
        string(table, registry, "pdchFenSHGC", key, "-")?;
        string(table, registry, "pdchFenVisTr", key, "-")?;
    }
    real(table, registry, "pdchIntFenArea", "Total or Average", 0.0)?;
    string(table, registry, "pdchIntFenUfact", "Total or Average", "-")?;
    string(table, registry, "pdchIntFenSHGC", "Total or Average", "-")?;
    string(table, registry, "pdchIntFenVisTr", "Total or Average", "-")?;
    for (name, class) in [
        ("Wall", 1),
        ("Floor", 2),
        ("Roof", 3),
        ("Internal Mass", 4),
        ("Building Detached Shading", 5),
        ("Fixed Detached Shading", 6),
        ("Window", 7),
        ("Door", 9),
        ("Glass Door", 8),
        ("Shading", 10),
        ("Overhang", 11),
        ("Fin", 12),
        ("Tubular Daylighting Device Dome", 13),
        ("Tubular Daylighting Device Diffuser", 14),
    ] {
        integer(table, registry, "pdchSurfCntTot", name, num_surfaces[class])?;
        integer(
            table,
            registry,
            "pdchSurfCntExt",
            name,
            num_exterior_surfaces[class],
        )?;
    }
    Ok(())
}
