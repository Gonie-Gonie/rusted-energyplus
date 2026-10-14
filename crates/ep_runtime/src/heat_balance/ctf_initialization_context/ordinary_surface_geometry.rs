//! Read-only views of actual retained World/Process owners, without recomputation.
use super::CtfInitializationReports;
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as Error, surface::SurfaceClass,
};
use crate::heat_balance::state::HeatBalanceState;
use ep_model::{ConstructionId, SpaceId, SurfaceId, ZoneId};

/// Ephemeral descriptors of existing Source owners, without replacement state.
pub struct OrdinarySurfaceGeometry<'a> {
    /// Actual final Source Surface order, independently of physical typed IDs.
    pub surfaces: Vec<OrdinaryGeometrySurface<'a>>,
    /// Actual same-owner Zone array, moved into the successful state.
    pub zones: Vec<OrdinaryGeometryZone<'a>>,
    /// Actual same-owner Space array, moved into the successful state.
    pub spaces: Vec<OrdinaryGeometrySpace<'a>>,
    /// Actual regular Construction order and retained getter names.
    pub constructions: Vec<OrdinaryGeometryConstruction<'a>>,
    /// Existing caller/scratch/list members, with no inferred readiness.
    pub process: OrdinaryGeometryProcess<'a>,
}

/// Actual Source Zone identity; its number is not a typed or Native ID alias.
pub struct OrdinaryGeometryZone<'a> {
    /// One-based position in the actual Source Zone array.
    pub source_number: i32,
    /// Independently bound physical identity.
    pub typed_id: ZoneId,
    /// Actual retained Zone.Name bytes.
    pub name: &'a str,
    /// Actual retained Source CeilingHeight, without recalculation.
    pub ceiling_height_m: f64,
    /// Actual retained Source ceilingHeightEntered flag.
    pub ceiling_height_entered: bool,
    /// Actual geometricFloorArea from the Setup GrossArea accumulation.
    pub geometric_floor_area_m2: f64,
    /// Actual geometricCeilingArea, distinct from CeilingArea.
    pub geometric_ceiling_area_m2: f64,
    /// Actual ExteriorTotalSurfArea.
    pub exterior_total_surface_area_m2: f64,
    /// Actual ExtGrossWallArea_Multiplied, with Source multiplication order.
    pub exterior_gross_wall_area_multiplied_m2: f64,
    /// Actual extPerimeter, accumulated from retained Process Width.
    pub exterior_perimeter_m: f64,
    /// Actual ExtGrossWallArea.
    pub exterior_gross_wall_area_m2: f64,
    /// Actual ExtNetWallArea, accumulated from Surface.Area.
    pub exterior_net_wall_area_m2: f64,
    /// Actual TotalSurfArea.
    pub total_surface_area_m2: f64,
    /// Actual ExtWindowArea, without an inferred window producer.
    pub exterior_window_area_m2: f64,
    /// Actual HasWindow.
    pub has_window: bool,
    /// Actual retained Zone.Multiplier.
    pub multiplier: i32,
    /// Actual retained Zone.ListMultiplier.
    pub list_multiplier: i32,
}

/// Actual Source Space identity and its genuine Zone association.
pub struct OrdinaryGeometrySpace<'a> {
    /// One-based position in the actual Source Space array.
    pub source_number: i32,
    /// Independently bound physical identity.
    pub typed_id: SpaceId,
    /// Actual retained Space.Name bytes.
    pub name: &'a str,
    /// Actual Source Zone association, not a physical ZoneId.
    pub zone_number: i32,
    /// Actual inherited ExtGrossWallArea.
    pub exterior_gross_wall_area_m2: f64,
    /// Actual inherited ExteriorTotalSurfArea.
    pub exterior_total_surface_area_m2: f64,
    /// Actual inherited extPerimeter.
    pub exterior_perimeter_m: f64,
    /// Actual totalSurfArea.
    pub total_surface_area_m2: f64,
    /// Actual extWindowArea.
    pub exterior_window_area_m2: f64,
}

/// Actual regular Construction identity and reached Name writer.
pub struct OrdinaryGeometryConstruction<'a> {
    /// One-based position in the actual Construction array.
    pub source_number: i32,
    /// Independently bound physical identity.
    pub typed_id: ConstructionId,
    /// Actual ConstructAlphas(0) bytes retained at Original1455.
    pub name: &'a str,
}

/// Borrowed actual ShadeV members; None means the array is unallocated.
pub struct OrdinaryGeometryShadeVertices<'a> {
    /// Actual stored ShadeV.NVert.
    pub n_vert: i32,
    /// Actual one-based XV storage, without copying values.
    pub xv: Option<&'a [f64]>,
    /// Actual one-based YV storage, without copying values.
    pub yv: Option<&'a [f64]>,
    /// Actual one-based ZV storage, without copying values.
    pub zv: Option<&'a [f64]>,
}

/// Actual retained per-Surface operands and Process results, never recalculated.
pub struct OrdinaryGeometrySurface<'a> {
    /// One-based final Source Surface array position.
    pub source_number: i32,
    /// Independently bound physical Surface identity.
    pub typed_id: SurfaceId,
    /// Actual final Surface.Name bytes.
    pub name: &'a str,
    /// Explicit Original class code: Wall1, Floor2, Roof3.
    pub class_code: i32,
    /// Actual Source Zone index.
    pub zone_number: i32,
    /// Actual Source Space index.
    pub space_number: i32,
    /// Actual Source Construction index.
    pub construction_number: i32,
    /// Actual remapped BaseSurf index.
    pub base_surface: i32,
    /// Actual stored Sides.
    pub sides: i32,
    /// Actual HeatTransSurf.
    pub heat_transfer_surface: bool,
    /// Actual ExtBoundCond, in the Source signed boundary/index domain.
    pub outside_boundary: i32,
    /// Actual IsAirBoundarySurf.
    pub air_boundary_surface: bool,
    /// Actual ExtSolar.
    pub ext_solar: bool,
    /// Actual MirroredSurf.
    pub mirrored_surface: bool,
    /// Actual IsShadowing.
    pub is_shadowing: bool,
    /// Complete written Vertex storage in original order, borrowing each triple.
    pub world_vertices: Vec<&'a [f64; 3]>,
    /// Actual GetVertices GrossArea value.
    pub gross_area_m2: f64,
    /// Actual retained Surface.Area, without aliasing it to GrossArea.
    pub area_m2: f64,
    /// Actual stored CosTilt value.
    pub cos_tilt: f64,
    /// Actual stored local X axis.
    pub lcsx: &'a [f64; 3],
    /// Actual stored local Y axis.
    pub lcsy: &'a [f64; 3],
    /// Actual stored local Z axis.
    pub lcsz: &'a [f64; 3],
    /// Actual Shape enum's explicit Original representation.
    pub shape_code: i32,
    /// Actual VerticesProcessed flag.
    pub vertices_processed: bool,
    /// Actual retained Width.
    pub width_m: f64,
    /// Actual retained Height.
    pub height_m: f64,
    /// Actual retained XShift.
    pub x_shift_m: f64,
    /// Actual retained YShift.
    pub y_shift_m: f64,
    /// Same per-Surface ShadeV owner.
    pub shade_vertices: OrdinaryGeometryShadeVertices<'a>,
    /// Actual X0/Y0/Z0 slots, copied only into this transient descriptor.
    pub coordinate_translation: [f64; 3],
    /// Actual SurfAirSkyRadSplit slot.
    pub surf_air_sky_rad_split: f64,
}

/// Actual retained Process/caller globals; no field is a completion surrogate.
pub struct OrdinaryGeometryProcess<'a> {
    /// Actual ProcessSurfaceVerticesOneTimeFlag.
    pub process_one_time_flag: bool,
    /// Actual MaxVerticesPerSurface producer value.
    pub max_vertices_per_surface: usize,
    /// Actual one-based Xpsv allocation and complete contents.
    pub xpsv: Option<&'a [f64]>,
    /// Actual one-based Ypsv allocation and complete contents.
    pub ypsv: Option<&'a [f64]>,
    /// Actual one-based Zpsv allocation and complete contents.
    pub zpsv: Option<&'a [f64]>,
    /// Actual Source-number list; no physical-ID remapping is inferred.
    pub all_ext_solar_and_shading: &'a [i32],
    /// Actual BuildingShadingCount.
    pub building_shading_count: i32,
    /// Actual FixedShadingCount.
    pub fixed_shading_count: i32,
    /// Actual AttachedShadingCount.
    pub attached_shading_count: i32,
    /// Actual ShadingSurfaceFirst.
    pub shading_surface_first: i32,
    /// Actual ShadingSurfaceLast.
    pub shading_surface_last: i32,
    /// Same actual DataHeatBalance caller flag.
    pub calc_window_reveal_reflection: bool,
}

fn number(index: usize) -> Result<i32, Error> {
    index
        .checked_add(1)
        .and_then(|n| i32::try_from(n).ok())
        .ok_or(Error::Unavailable("signed Source geometry array position"))
}
fn index(number: i32) -> Result<usize, Error> {
    usize::try_from(number)
        .ok()
        .and_then(|n| n.checked_sub(1))
        .ok_or(Error::Unavailable("positive Source geometry association"))
}

impl CtfInitializationReports {
    /// Borrow actual retained World/Process fields after successful initialization.
    /// The legacy unentered route returns None. This never invokes a getter,
    /// geometry helper, owner reset, or readiness transition.
    ///
    /// # Errors
    /// Returns the existing unavailable error for missing selected owners,
    /// unwritten fields, incomplete arrays or unresolved physical associations.
    pub fn ordinary_surface_geometry<'a>(
        &'a self,
        state: &'a HeatBalanceState,
    ) -> Result<Option<OrdinarySurfaceGeometry<'a>>, Error> {
        let source = &self.surface_geometry_input;
        if !source.entered_once {
            return Ok(None);
        }
        let final_rows = source
            .final_surfaces
            .as_deref()
            .ok_or(Error::Unavailable("actual final Source Surface owner"))?;
        let arrays = source
            .parallel_arrays
            .as_ref()
            .ok_or(Error::Unavailable("actual Source parallel arrays"))?;
        let owner = state
            .selected_zone_input
            .as_ref()
            .ok_or(Error::Unavailable("same moved Source Zone/Space owner"))?;
        let zone_rows = owner
            .zones
            .as_deref()
            .ok_or(Error::Unavailable("actual Source Zone allocation"))?;
        let space_rows = owner
            .spaces
            .as_deref()
            .ok_or(Error::Unavailable("actual Source Space allocation"))?;
        let control = &self.heat_balance_input_state;
        if usize::try_from(source.total_surfaces).ok() != Some(final_rows.len())
            || control.construction_usage.len() != control.construction_order.len()
        {
            return Err(Error::Unavailable(
                "complete actual Source geometry associations",
            ));
        }
        let zones = zone_rows
            .iter()
            .enumerate()
            .map(|(n, row)| {
                Ok(OrdinaryGeometryZone {
                    source_number: number(n)?,
                    name: &row.name,
                    ceiling_height_m: row.ceiling_height,
                    ceiling_height_entered: row.ceiling_height_entered,
                    geometric_floor_area_m2: row.geometric_floor_area,
                    geometric_ceiling_area_m2: row.geometric_ceiling_area,
                    exterior_total_surface_area_m2: row.exterior_total_surface_area,
                    exterior_gross_wall_area_multiplied_m2: row.exterior_gross_wall_area_multiplied,
                    exterior_perimeter_m: row.exterior_perimeter,
                    exterior_gross_wall_area_m2: row.exterior_gross_wall_area,
                    exterior_net_wall_area_m2: row.exterior_net_wall_area,
                    total_surface_area_m2: row.total_surface_area,
                    exterior_window_area_m2: row.exterior_window_area,
                    has_window: row.has_window,
                    multiplier: row.multiplier,
                    list_multiplier: row.list_multiplier,
                    typed_id: row
                        .typed_zone_id
                        .ok_or(Error::Unavailable("actual bound Source ZoneId"))?,
                })
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let spaces = space_rows
            .iter()
            .enumerate()
            .map(|(n, row)| {
                Ok(OrdinaryGeometrySpace {
                    source_number: number(n)?,
                    name: &row.name,
                    zone_number: row.zone_num,
                    exterior_gross_wall_area_m2: row.exterior_gross_wall_area,
                    exterior_total_surface_area_m2: row.exterior_total_surface_area,
                    exterior_perimeter_m: row.exterior_perimeter,
                    total_surface_area_m2: row.total_surface_area,
                    exterior_window_area_m2: row.exterior_window_area,
                    typed_id: row
                        .typed_space_id
                        .ok_or(Error::Unavailable("actual bound Source SpaceId"))?,
                })
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let constructions = control
            .construction_usage
            .iter()
            .enumerate()
            .map(|(n, row)| {
                let typed_id = row
                    .construction_id
                    .ok_or(Error::Unavailable("actual bound Source ConstructionId"))?;
                if row.name.is_empty() || control.construction_order[n] != typed_id {
                    return Err(Error::Unavailable(
                        "actual Source Construction.Name and identity",
                    ));
                }
                Ok(OrdinaryGeometryConstruction {
                    source_number: number(n)?,
                    name: &row.name,
                    typed_id,
                })
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let mut surfaces = Vec::with_capacity(final_rows.len());
        for (n, surface) in final_rows.iter().enumerate() {
            let row = &surface.row;
            let zone = zone_rows
                .get(index(row.zone)?)
                .ok_or(Error::Unavailable("actual Surface Zone association"))?;
            let space = space_rows
                .get(index(row.space)?)
                .ok_or(Error::Unavailable("actual Surface Space association"))?;
            if space.zone_num != row.zone || zone.typed_zone_id.is_none() {
                return Err(Error::Unavailable(
                    "same Source Surface Zone/Space association",
                ));
            }
            constructions
                .get(index(row.construction)?)
                .ok_or(Error::Unavailable(
                    "actual Surface Construction association",
                ))?;
            let vertices = row
                .vertex
                .as_deref()
                .ok_or(Error::Unavailable("actual allocated final Vertex"))?;
            if usize::try_from(row.sides).ok() != Some(vertices.len()) {
                return Err(Error::Unavailable("complete actual final Vertex bounds"));
            }
            let world_vertices = vertices
                .iter()
                .map(|v| {
                    v.as_ref()
                        .ok_or(Error::Unavailable("actual written final Vertex"))
                })
                .collect::<Result<Vec<_>, Error>>()?;
            let normal = row
                .vertex_progress
                .normal_area
                .as_ref()
                .ok_or(Error::Unavailable("actual retained GrossArea"))?;
            let properties = row
                .vertex_progress
                .properties
                .as_ref()
                .ok_or(Error::Unavailable("actual retained local axes/CosTilt"))?;
            let shade = arrays
                .shade_vertices
                .get(n)
                .ok_or(Error::Unavailable("same actual ShadeV slot"))?;
            surfaces.push(OrdinaryGeometrySurface {
                source_number: number(n)?,
                typed_id: surface
                    .typed_id
                    .ok_or(Error::Unavailable("actual bound physical SurfaceId"))?,
                name: &row.name,
                class_code: match row.class {
                    SurfaceClass::Wall => 1,
                    SurfaceClass::Floor => 2,
                    SurfaceClass::Roof => 3,
                    _ => return Err(Error::Unavailable("selected actual opaque SurfaceClass")),
                },
                zone_number: row.zone,
                space_number: row.space,
                construction_number: row.construction,
                base_surface: row.base_surface,
                sides: row.sides,
                heat_transfer_surface: row.heat_transfer_surface,
                outside_boundary: row.exterior_boundary,
                air_boundary_surface: row.is_air_boundary_surface,
                ext_solar: row.exterior_solar,
                mirrored_surface: row.mirrored_surface,
                is_shadowing: row.is_shadowing,
                world_vertices,
                gross_area_m2: normal.gross_area_m2,
                area_m2: properties.area_m2,
                cos_tilt: properties.cos_tilt,
                lcsx: &properties.lcsx,
                lcsy: &properties.lcsy,
                lcsz: &properties.lcsz,
                shape_code: row.shape as i32,
                vertices_processed: row.vertices_processed,
                width_m: row.vertex_progress.width,
                height_m: row.vertex_progress.height,
                x_shift_m: row.x_shift,
                y_shift_m: row.y_shift,
                shade_vertices: OrdinaryGeometryShadeVertices {
                    n_vert: shade.n_vert,
                    xv: shade.xv.as_deref(),
                    yv: shade.yv.as_deref(),
                    zv: shade.zv.as_deref(),
                },
                coordinate_translation: [
                    *arrays
                        .x0
                        .get(n)
                        .ok_or(Error::Unavailable("actual X0 slot"))?,
                    *arrays
                        .y0
                        .get(n)
                        .ok_or(Error::Unavailable("actual Y0 slot"))?,
                    *arrays
                        .z0
                        .get(n)
                        .ok_or(Error::Unavailable("actual Z0 slot"))?,
                ],
                surf_air_sky_rad_split: *arrays
                    .surf_air_sky_rad_split
                    .get(n)
                    .ok_or(Error::Unavailable("actual SurfAirSkyRadSplit slot"))?,
            });
        }
        let scratch = &source.process_vertices_scratch;
        let counters = &source.setup_shading;
        Ok(Some(OrdinarySurfaceGeometry {
            surfaces,
            zones,
            spaces,
            constructions,
            process: OrdinaryGeometryProcess {
                process_one_time_flag: scratch.one_time,
                max_vertices_per_surface: source.coordinate.max_vertices_per_surface,
                xpsv: scratch.x.as_deref(),
                ypsv: scratch.y.as_deref(),
                zpsv: scratch.z.as_deref(),
                all_ext_solar_and_shading: &source.surface_lists.all_exterior_solar_and_shading,
                building_shading_count: counters.building,
                fixed_shading_count: counters.fixed,
                attached_shading_count: counters.attached,
                shading_surface_first: counters.first,
                shading_surface_last: counters.last,
                calc_window_reveal_reflection: control.calculate_window_reveal_reflection,
            },
        }))
    }
}
