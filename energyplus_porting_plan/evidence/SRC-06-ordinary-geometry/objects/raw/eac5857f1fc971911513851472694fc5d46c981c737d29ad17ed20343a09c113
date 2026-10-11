// GEO-03 read-only original fields. Reuse the immutable GEO-02 IEEE/ABI observer.
#ifndef GEO03_REFERENCE_FIELDS_HH
#define GEO03_REFERENCE_FIELDS_HH
#include "geo02_reference_fields.hh"
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataHeatBalance.hh>
#include <EnergyPlus/Vectors.hh>

namespace Geo03 {
using Geo02::json;
using Geo02::scalar;
using Geo02::vector;

inline json zone(EnergyPlus::DataHeatBalance::ZoneData const &z, int id)
{
    json indexes = json::array();
    for (int index : z.spaceIndexes) indexes.push_back(index);
    return {{"id", id}, {"name", z.Name}, {"volume_m3", scalar(z.Volume)},
            {"ceiling_height_m", scalar(z.CeilingHeight)}, {"ceiling_height_entered", z.ceilingHeightEntered},
            {"floor_area_m2", scalar(z.FloorArea)}, {"user_entered_floor_area_m2", scalar(z.UserEnteredFloorArea)},
            {"geometric_floor_area_m2", scalar(z.geometricFloorArea)}, {"ceiling_area_m2", scalar(z.CeilingArea)},
            {"geometric_ceiling_area_m2", scalar(z.geometricCeilingArea)}, {"has_floor", z.HasFloor}, {"has_roof", z.HasRoof},
            {"all_surface_first", z.AllSurfaceFirst}, {"all_surface_last", z.AllSurfaceLast},
            {"num_spaces", z.numSpaces}, {"space_indexes", indexes}};
}

inline json space(EnergyPlus::DataHeatBalance::SpaceData const &s, int id)
{
    json indexes = json::array();
    for (int index : s.surfaces) indexes.push_back(index);
    return {{"id", id}, {"name", s.Name}, {"zone_id", s.zoneNum}, {"volume_m3", scalar(s.Volume)},
            {"ceiling_height_m", scalar(s.CeilingHeight)}, {"floor_area_m2", scalar(s.FloorArea)},
            {"user_entered_floor_area_m2", scalar(s.userEnteredFloorArea)}, {"has_floor", s.hasFloor},
            {"fraction_zone_volume", scalar(s.fracZoneVolume)}, {"fraction_zone_floor_area", scalar(s.fracZoneFloorArea)},
            {"is_remainder_space", s.isRemainderSpace}, {"surface_indexes", indexes},
            {"all_surface_first", s.AllSurfaceFirst}, {"all_surface_last", s.AllSurfaceLast}};
}

inline json globals(EnergyPlus::EnergyPlusData const &state)
{
    auto const &g = *state.dataSurfaceGeometry;
    return {{"p0_m", vector(state.dataVectors->p0)}, {"err_count", g.ErrCount}, {"err_count5", g.ErrCount5},
            {"show_zone_surface_headers", g.ShowZoneSurfaceHeaders}, {"display_extra_warnings", state.dataGlobal->DisplayExtraWarnings},
            {"is_epjson", state.dataGlobal->isEpJSON}, {"preserve_idf_order", state.dataGlobal->preserveIDFOrder},
            {"surfaces_allocated", state.dataSurface->Surface.allocated()}, {"temporary_surfaces_allocated", g.SurfaceTmp.allocated()},
            {"zones_allocated", state.dataHeatBal->Zone.allocated()}, {"spaces_allocated", state.dataHeatBal->space.allocated()},
            {"space_count", state.dataHeatBal->space.size()}, {"surface_count", state.dataSurface->TotSurfaces},
            {"zone_count", state.dataGlobal->NumOfZones}, {"geometry_context", Geo02::globals(state)}};
}

inline json surface(EnergyPlus::DataSurfaces::SurfaceData const &s, int id)
{
    return {{"id", id}, {"name", s.Name}, {"zone_id", s.Zone}, {"zone_name", s.ZoneName}, {"space_id", s.spaceNum},
            {"class", static_cast<int>(s.Class)}, {"sides", s.Sides}, {"world_vertex_bits", Geo02::vertex_bits(s)},
            {"area_m2", scalar(s.Area)}, {"gross_area_m2", scalar(s.GrossArea)},
            {"azimuth_deg", scalar(s.Azimuth)}, {"tilt_deg", scalar(s.Tilt)},
            {"newell_area_vector_m2", vector(s.NewellAreaVector)}};
}

inline json snapshot(EnergyPlus::EnergyPlusData const &state, std::string const &phase, bool vertices_written)
{
    json zones = json::array();
    json spaces = json::array();
    json surfaces = json::array();
    json ordered_faces = json::array();
    if (state.dataHeatBal->Zone.allocated()) {
        for (int i = 1; i <= static_cast<int>(state.dataHeatBal->Zone.size()); ++i) {
            zones.push_back(zone(state.dataHeatBal->Zone(i), i));
            if (vertices_written) {
                auto const &z = state.dataHeatBal->Zone(i);
                json faces = json::array();
                for (int n = z.AllSurfaceFirst; n <= z.AllSurfaceLast; ++n) {
                    auto const &s = state.dataSurface->Surface(n);
                    using EnergyPlus::DataSurfaces::SurfaceClass;
                    if (s.Class == SurfaceClass::Wall || s.Class == SurfaceClass::Floor || s.Class == SurfaceClass::Roof)
                        faces.push_back({{"own_surface_id", n}, {"name", s.Name}, {"class", static_cast<int>(s.Class)}});
                }
                ordered_faces.push_back({{"own_zone_id", i}, {"faces", faces},
                                         {"projection", "actual-contiguous-own-Surface-array-order-filtered-to-source-base-classes"}});
            }
        }
    }
    if (state.dataHeatBal->space.allocated()) {
        for (int i = 1; i <= static_cast<int>(state.dataHeatBal->space.size()); ++i)
            spaces.push_back(space(state.dataHeatBal->space(i), i));
    }
    // Non-debug Vector3 default construction leaves coordinates unwritten.
    // Allocation alone never authorizes observing their values.
    if (vertices_written && state.dataSurface->Surface.allocated()) {
        for (int i = 1; i <= state.dataSurface->TotSurfaces; ++i)
            surfaces.push_back(surface(state.dataSurface->Surface(i), i));
    }
    return {{"phase", phase}, {"floating_environment", Geo02::floating_environment()}, {"native_only_state", globals(state)},
            {"vertex_values_observed", vertices_written}, {"zones", zones}, {"spaces", spaces}, {"surfaces", surfaces},
            {"ordered_volume_base_faces", ordered_faces}};
}

inline json retained_identity(json const &snapshot)
{
    // Mutable error counters are source-only diagnostics, not retained geometry.
    return {{"zones", snapshot.at("zones")}, {"spaces", snapshot.at("spaces")}, {"surfaces", snapshot.at("surfaces")},
            {"ordered_volume_base_faces", snapshot.at("ordered_volume_base_faces")},
            {"p0_m", snapshot.at("native_only_state").at("p0_m")}};
}
} // namespace Geo03
#endif
