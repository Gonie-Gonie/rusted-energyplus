//! Copies the actual retained Source owners into the existing geometry trace.
use super::bits;
use ep_runtime::heat_balance::ctf_initialization_context::OrdinarySurfaceGeometry;
use ep_runtime::heat_balance::radiation::ordinary::OrdinarySolarRadiation;
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

fn solar(owner: Option<&OrdinarySolarRadiation>) -> Value {
    let Some(owner) = owner else {
        return Value::Null;
    };
    let storage = owner.enclosure_storage();
    json!({
        "NumOfSolarEnclosures":owner.num_solar_enclosures(),
        "NumOfRadiantEnclosures":owner.num_radiant_enclosures(),
        "EnclSolInfo_allocation":{
            "allocated":storage.is_some(), "size":storage.map(<[_]>::len),
            "operator_parentheses_indexing":"one-based Source association domain",
        },
        "EnclSolInfo":storage.map(|rows| rows.iter().enumerate().map(|(n,row)|json!({
            "index":n+1, "Name":row.name,
            "spaceNames":row.space_names, "spaceNames_size":row.space_names.len(),
            "spaceNums":row.space_nums, "spaceNums_size":row.space_nums.len(),
            "FloorArea":scalar(row.floor_area),
            "ExtWindowArea":scalar(row.exterior_window_area),
            "TotalSurfArea":scalar(row.total_surface_area),
        })).collect::<Vec<_>>()),
        "capture_boundary":"same owner retained from Setup797 and moved into State; late numeric Solar fields excluded",
    })
}

pub(super) fn projection(owners: OrdinarySurfaceGeometry<'_>) -> Value {
    let process = &owners.process;
    json!({
        "schema":"ordinary-source-geometry.v1",
        "capture_boundary":"actual successful Rust simulation return; retained selected Setup313-639 owners",
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
                "centroid_m":vector(&row.centroid_m),
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
            "bounds_m":row.bounds_m.iter().copied().map(scalar).collect::<Vec<_>>(),
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
            "solarEnclosureNum":row.solar_enclosure_number,
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
            "TotLayers":row.total_layers,
            "LayerPoint":{
                "lower":1, "upper":row.layer_points.len(), "size":row.layer_points.len(),
                "values":row.layer_points,
            },
            "TotSolidLayers":row.total_solid_layers, "TotGlassLayers":row.total_glass_layers,
            "DayltPropPtr":row.daylt_prop_ptr,
            "TypeIsWindow":row.type_is_window, "WindowTypeEQL":row.window_type_eql,
            "TypeIsEcoRoof":row.type_is_eco_roof, "TypeIsIRT":row.type_is_irt,
            "IsUsed":row.is_used, "IsUsedCTF":row.is_used_ctf,
            "InsideAbsorpVis":scalar(row.inside_absorp_vis),
            "OutsideAbsorpVis":scalar(row.outside_absorp_vis),
            "InsideAbsorpSolar":scalar(row.inside_absorp_solar),
            "OutsideAbsorpSolar":scalar(row.outside_absorp_solar),
            "InsideAbsorpThermal":scalar(row.inside_absorp_thermal),
            "OutsideAbsorpThermal":scalar(row.outside_absorp_thermal),
            "ReflectVisDiffBack":scalar(row.reflect_vis_diff_back),
            "AbsDiffShade":scalar(row.abs_diff_shade),
            "OutsideRoughness":row.outside_roughness,
        })).collect::<Vec<_>>(),
        "solar":solar(owners.solar),
        "materials":{
            "materialMap":owners.material_map,
            "Source_order_storage":{
                "size":owners.materials.len(), "one_based_indexing":true,
                "Native_allocation_and_pointer_identity":"UNPAIRED",
            },
            "rows":owners.materials.iter().map(|row|json!({
                "id":row.source_number, "typed_id":row.typed_id.0, "map_key":row.map_key,
                "group":row.group_code, "NominalR":scalar(row.nominal_r),
                "validated_typed_properties":{
                    "Roughness":row.roughness_code,
                    "AbsorpThermal":scalar(row.thermal_absorptance),
                    "AbsorpSolar":scalar(row.solar_absorptance),
                    "AbsorpVisible":scalar(row.visible_absorptance),
                },
            })).collect::<Vec<_>>(),
            "property_origin":"unique Source MaterialNum to sole typed property owner, bit-validated by actual raw getter; no physical cache reconstruction",
            "unrepresented_Native_MaterialBase_members":[
                "separate Name member", "ROnly", "Thickness", "Conductivity", "Resistance",
                "AbsorpThermalInput", "AbsorpSolarInput", "AbsorpVisibleInput",
            ],
        },
        "heat_balance_nominal":{
            "NominalRforNominalUCalculation":array(owners.nominal_r_for_u),
            "NominalU":array(owners.nominal_u),
            "NominalUBeforeAdjusted":array(owners.nominal_u_before_adjusted),
            "CoeffAdjRatio":array(owners.coefficient_adjustment_ratio),
        },
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
