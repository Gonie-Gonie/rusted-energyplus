//! Copies the actual retained Source owners into the existing geometry trace.
use super::bits;
use ep_runtime::heat_balance::ctf_initialization_context::OrdinarySurfaceGeometry;
use serde_json::{Value, json};

fn scalar(value: f64) -> Value {
    json!({"value":value, "value_bits":bits(value)})
}

fn vector(values: &[f64; 3]) -> Value {
    json!(values.iter().copied().map(scalar).collect::<Vec<_>>())
}

fn array(values: Option<&[f64]>) -> Value {
    match values {
        Some(values) => json!({
            "allocated":true, "lower":1, "size":values.len(), "upper":values.len(),
            "values":values.iter().copied().map(scalar).collect::<Vec<_>>(),
        }),
        None => json!({
            "allocated":false, "lower":null, "size":null, "upper":null, "values":null,
        }),
    }
}

pub(super) fn projection(owners: OrdinarySurfaceGeometry<'_>) -> Value {
    let process = &owners.process;
    json!({
        "schema":"ordinary-source-geometry.v1",
        "capture_boundary":"actual successful Rust simulation return; retained selected Setup313-548 owners",
        "observer_supplies_inputs":false,
        "names_origin":"actual Source getter Name members; physical typed IDs remain separate",
        "array_bounds_semantics":"allocated Rust storage observed in its existing one-based Source index domain; unallocated bounds remain unknown",
        "surfaces":owners.surfaces.iter().map(|row|json!({
            "id":row.source_number, "typed_id":row.typed_id.0, "name":row.name,
            "class":row.class_code, "zone_id":row.zone_number, "space_id":row.space_number,
            "construction_id":row.construction_number, "base_surface":row.base_surface,
            "sides":row.sides, "heat_transfer_surface":row.heat_transfer_surface,
            "ext_solar":row.ext_solar, "mirrored_surface":row.mirrored_surface,
            "is_shadowing":row.is_shadowing, "outside_boundary":row.outside_boundary,
            "air_boundary_surface":row.air_boundary_surface,
            "world_vertex_bits":row.world_vertices.iter().map(|vertex|{
                vertex.iter().copied().map(bits).collect::<Vec<_>>()
            }).collect::<Vec<_>>(),
            "geometry":{
                "area_m2":scalar(row.area_m2),
                "gross_area_m2":scalar(row.gross_area_m2), "cos_tilt":scalar(row.cos_tilt),
                "lcsx":vector(row.lcsx), "lcsy":vector(row.lcsy), "lcsz":vector(row.lcsz),
            },
            "stored_Process_return":{
                "shape":row.shape_code, "vertices_processed":row.vertices_processed,
                "width_m":scalar(row.width_m), "height_m":scalar(row.height_m),
                "x_shift_m":scalar(row.x_shift_m), "y_shift_m":scalar(row.y_shift_m),
                "coordinate_translation":vector(&row.coordinate_translation),
                "surf_air_sky_rad_split":scalar(row.surf_air_sky_rad_split),
                "shade_vertices":{
                    "n_vert":row.shade_vertices.n_vert,
                    "xv":array(row.shade_vertices.xv), "yv":array(row.shade_vertices.yv),
                    "zv":array(row.shade_vertices.zv),
                },
            },
        })).collect::<Vec<_>>(),
        "zones":owners.zones.iter().map(|row|json!({
            "id":row.source_number, "typed_id":row.typed_id.0, "name":row.name,
            "ceiling_height_m":scalar(row.ceiling_height_m),
            "ceiling_height_entered":row.ceiling_height_entered,
            "volume_m3":scalar(row.volume_m3),
            "Multiplier":row.multiplier, "ListMultiplier":row.list_multiplier,
            "HasWindow":row.has_window,
            "geometric_floor_area_m2":scalar(row.geometric_floor_area_m2),
            "geometric_ceiling_area_m2":scalar(row.geometric_ceiling_area_m2),
            "exterior_areas":{
                "ExtGrossWallArea":scalar(row.exterior_gross_wall_area_m2),
                "ExtNetWallArea":scalar(row.exterior_net_wall_area_m2),
                "TotalSurfArea":scalar(row.total_surface_area_m2),
                "ExtWindowArea":scalar(row.exterior_window_area_m2),
                "ExteriorTotalSurfArea":scalar(row.exterior_total_surface_area_m2),
                "ExtGrossWallArea_Multiplied":scalar(row.exterior_gross_wall_area_multiplied_m2),
                "extPerimeter":scalar(row.exterior_perimeter_m),
            },
        })).collect::<Vec<_>>(),
        "spaces":owners.spaces.iter().map(|row|json!({
            "id":row.source_number, "typed_id":row.typed_id.0, "name":row.name,
            "zone_id":row.zone_number,
            "volume_m3":scalar(row.volume_m3),
            "fraction_zone_volume":scalar(row.fraction_zone_volume),
            "exterior_areas":{
                "ExtGrossWallArea":scalar(row.exterior_gross_wall_area_m2),
                "ExteriorTotalSurfArea":scalar(row.exterior_total_surface_area_m2),
                "extPerimeter":scalar(row.exterior_perimeter_m),
                "totalSurfArea":scalar(row.total_surface_area_m2),
                "extWindowArea":scalar(row.exterior_window_area_m2),
            },
        })).collect::<Vec<_>>(),
        "constructions":owners.constructions.iter().map(|row|json!({
            "id":row.source_number, "typed_id":row.typed_id.0, "name":row.name,
        })).collect::<Vec<_>>(),
        "geometry_context":{"err_count5":owners.err_count5},
        "global_Process_return":{
            "process_one_time_flag":process.process_one_time_flag,
            "max_vertices_per_surface":process.max_vertices_per_surface,
            "xpsv":array(process.xpsv), "ypsv":array(process.ypsv), "zpsv":array(process.zpsv),
            "all_ext_solar_and_shading":process.all_ext_solar_and_shading,
            "building_shading_count":process.building_shading_count,
            "fixed_shading_count":process.fixed_shading_count,
            "attached_shading_count":process.attached_shading_count,
            "shading_surface_first":process.shading_surface_first,
            "shading_surface_last":process.shading_surface_last,
            "calc_window_reveal_reflection":process.calc_window_reveal_reflection,
        },
        "claim_boundary":"independent stored-field observation only; comparison is external and neither full Setup/HB nor whole-engine conformance is asserted",
    })
}
