// Read-only projections of actual original GEO-02 fields. No geometry formulas.
#ifndef GEO02_REFERENCE_FIELDS_HH
#define GEO02_REFERENCE_FIELDS_HH
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataSurfaces.hh>
#include <EnergyPlus/DataErrorTracking.hh>
#include <EnergyPlus/SurfaceGeometry.hh>
#include <nlohmann/json.hpp>
#include <bit>
#include <cfenv>
#include <cfloat>
#include <cmath>
#include <cstdint>
#include <iomanip>
#include <sstream>

#if !defined(__MINGW32__) || !defined(_WIN64) || __GNUC__ != 13 || __GNUC_MINOR__ != 2
#error "Reference requires the verified GCC13.2 Windows ABI"
#endif
#if defined(_GLIBCXX_DEBUG) || defined(__FAST_MATH__) || defined(NDEBUG)
#error "Reference requires non-debug containers, no fast math and original enabled assertions"
#endif
#if !defined(EP_psych_errors) || defined(EP_psych_stats)
#error "Reference definitions must match the genuine original core"
#endif

namespace Geo02 {
using json = nlohmann::json;
inline std::string bits(double value)
{
    std::ostringstream out;
    out << std::hex << std::setfill('0') << std::setw(16) << std::bit_cast<std::uint64_t>(value);
    return out.str();
}
inline json scalar(double value)
{
    char const *kind = std::isnan(value) ? "nan" : std::isinf(value) ? (value > 0 ? "positive_infinity" : "negative_infinity") :
                       value == 0 ? (std::signbit(value) ? "negative_zero" : "positive_zero") : "finite";
    return {{"value", std::isfinite(value) ? json(value) : json(nullptr)}, {"value_bits", bits(value)}, {"value_class", kind}};
}
inline json vector(EnergyPlus::DataVectorTypes::Vector const &v)
{
    return json::array({scalar(v.x), scalar(v.y), scalar(v.z)});
}
inline json floating_environment()
{
    unsigned short control = 0;
    unsigned int mxcsr = 0;
    // Read-only hardware observations. Never change CRT/x87/SSE control state.
    __asm__ __volatile__("fnstcw %0" : "=m"(control));
    __asm__ __volatile__("stmxcsr %0" : "=m"(mxcsr));
    unsigned int const pc = (control >> 8) & 3;
    return {{"sizeof_float", sizeof(float)}, {"sizeof_double", sizeof(double)}, {"sizeof_long_double", sizeof(long double)},
            {"FLT_MANT_DIG", FLT_MANT_DIG}, {"DBL_MANT_DIG", DBL_MANT_DIG}, {"LDBL_MANT_DIG", LDBL_MANT_DIG},
            {"fe_round", std::fegetround()}, {"x87_control_word", control}, {"x87_precision_control_bits", pc},
            {"x87_precision_bits", pc == 0 ? json(24) : pc == 2 ? json(53) : pc == 3 ? json(64) : json(nullptr)},
            {"x87_rounding_control_bits", (control >> 10) & 3}, {"mxcsr", mxcsr}, {"mxcsr_rounding_control_bits", (mxcsr >> 13) & 3},
            {"control_writes_added", false}};
}
inline json geometry(EnergyPlus::DataSurfaces::SurfaceData const &s)
{
    return {{"area_m2", scalar(s.Area)}, {"gross_area_m2", scalar(s.GrossArea)}, {"net_area_shadow_m2", scalar(s.NetAreaShadowCalc)},
            {"azimuth_deg", scalar(s.Azimuth)}, {"tilt_deg", scalar(s.Tilt)},
            {"newell_area_vector_m2", vector(s.NewellAreaVector)}, {"newell_normal", vector(s.NewellSurfaceNormalVector)},
            {"out_norm", vector(s.OutNormVec)}, {"centroid_m", vector(s.Centroid)},
            {"lcsx", vector(s.lcsx)}, {"lcsy", vector(s.lcsy)}, {"lcsz", vector(s.lcsz)},
            {"sin_azimuth", scalar(s.SinAzim)}, {"cos_azimuth", scalar(s.CosAzim)},
            {"sin_tilt", scalar(s.SinTilt)}, {"cos_tilt", scalar(s.CosTilt)}};
}
inline json vertex_bits(EnergyPlus::DataSurfaces::SurfaceData const &s)
{
    json result = json::array();
    for (int i = 1; i <= s.Sides; ++i) {
        auto const &v = s.Vertex(i);
        result.push_back(json::array({bits(v.x), bits(v.y), bits(v.z)}));
    }
    return result;
}
template <typename Array> json array_dimensions(Array const &a)
{
    return {{"allocated", a.allocated()}, {"size", a.size()}};
}
inline json globals(EnergyPlus::EnergyPlusData const &state)
{
    auto const &s = *state.dataSurface;
    auto const &g = *state.dataSurfaceGeometry;
    return {{"max_vertices_per_surface", s.MaxVerticesPerSurface}, {"corner", s.Corner}, {"counterclockwise", s.CCW},
            {"world_coordinate_system", s.WorldCoordSystem}, {"aspect_transform", s.AspectTransform},
            {"first_time", g.firstTime}, {"no_transform", g.noTransform},
            {"process_one_time_flag", g.ProcessSurfaceVerticesOneTimeFlag}, {"input_once_flag", g.GetSurfaceDataOneTimeFlag},
            {"total_coincident_vertices", state.dataErrTracking->TotalCoincidentVertices},
            {"total_degenerate_surfaces", state.dataErrTracking->TotalDegenerateSurfaces},
            {"xpsv", array_dimensions(g.Xpsv)}, {"ypsv", array_dimensions(g.Ypsv)}, {"zpsv", array_dimensions(g.Zpsv)},
            {"triangle1", array_dimensions(g.Triangle1)}, {"triangle2", array_dimensions(g.Triangle2)},
            {"zone_cos", array_dimensions(g.CosZoneRelNorth)}, {"zone_sin", array_dimensions(g.SinZoneRelNorth)},
            {"cos_building_relative_north", scalar(g.CosBldgRelNorth)}, {"sin_building_relative_north", scalar(g.SinBldgRelNorth)},
            {"cos_appendix_g_only", scalar(g.CosBldgRotAppGonly)}, {"sin_appendix_g_only", scalar(g.SinBldgRotAppGonly)}};
}
inline json surface_snapshot(EnergyPlus::EnergyPlusData const &state, EnergyPlus::DataSurfaces::SurfaceData const &s,
                             bool vertices_written, bool triangles_written)
{
    json out = {{"globals", globals(state)}, {"geometry", geometry(s)}, {"sides", s.Sides},
                {"class", static_cast<int>(s.Class)}, {"base_surface", s.BaseSurf}, {"heat_transfer_surface", s.HeatTransSurf},
                {"vertices_processed", s.VerticesProcessed}, {"is_convex", s.IsConvex}, {"is_degenerate", s.IsDegenerate},
                {"shape", static_cast<int>(s.Shape)}, {"width_m", scalar(s.Width)}, {"height_m", scalar(s.Height)},
                {"x_shift_m", scalar(s.XShift)}, {"y_shift_m", scalar(s.YShift)},
                {"vertex_allocation", array_dimensions(s.Vertex)}, {"vertex_values_observed", vertices_written},
                {"triangle_values_observed", triangles_written}, {"floating_environment", floating_environment()}};
    if (vertices_written) out["vertex_bits"] = vertex_bits(s);
    if (triangles_written) {
        json t1 = json::array();
        json t2 = json::array();
        for (int i = 1; i <= 3; ++i) {
            t1.push_back(vector(state.dataSurfaceGeometry->Triangle1(i)));
            t2.push_back(vector(state.dataSurfaceGeometry->Triangle2(i)));
        }
        out["triangle1"] = t1;
        out["triangle2"] = t2;
    }
    return out;
}
} // namespace Geo02
#endif
