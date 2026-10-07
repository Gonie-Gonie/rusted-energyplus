// Test-only genuine original geometry producers with explicitly prepared state.
// Whole helpers come from the unchanged original core/header. Only Setup's
// initialization fragment is included verbatim with its immutable range receipt.
#include "geo02_reference_fields.hh"
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataHeatBalance.hh>
#include <EnergyPlus/DataEnvironment.hh>
#include <EnergyPlus/DataGlobalConstants.hh>
#include <EnergyPlus/InputProcessing/InputProcessor.hh>
#include <EnergyPlus/Vectors.hh>
#include <fstream>
#include <iostream>
#include <memory>
#include <stdexcept>
#include <string>

namespace {
using Geo02::json;
using namespace EnergyPlus;
using EnergyPlus::DataVectorTypes::Vector;

void require(bool condition, std::string const &message)
{
    if (!condition) throw std::runtime_error(message);
}
double from_bits(std::string const &token)
{
    require(token.size() == 16 && token.find_first_not_of("0123456789abcdef") == std::string::npos, "Invalid input binary64 bits");
    return std::bit_cast<double>(static_cast<std::uint64_t>(std::stoull(token, nullptr, 16)));
}
void original_setup_fragment(EnergyPlus::EnergyPlusData &state)
{
    // The pre-build preparer copies EXACT selected source273-289 bytes.
    // This is not the complete SetupZoneGeometry or original parser admission.
#include GEO02_SETUP_FRAGMENT
}
DataSurfaces::SurfaceClass surface_class(std::string const &name)
{
    if (name == "Wall") return DataSurfaces::SurfaceClass::Wall;
    if (name == "Roof" || name == "Ceiling") return DataSurfaces::SurfaceClass::Roof;
    if (name == "Floor") return DataSurfaces::SurfaceClass::Floor;
    throw std::runtime_error("Unsupported prepared surface class");
}
void set_prepared_surface(DataSurfaces::SurfaceData &surface, json const &item)
{
    surface.Name = item.at("case_id").get<std::string>();
    surface.Zone = 1;
    surface.ZoneName = "PREPARED-ONE-ZONE";
    surface.Class = surface_class(item.at("surface_class"));
    surface.BaseSurf = 1; // Actual Process base-field indexing requires index1.
    surface.HeatTransSurf = true;
    surface.ExtWind = true;
    surface.ExtBoundCond = DataSurfaces::ExternalEnvironment;
    surface.Sides = 4;
    surface.Vertex.allocate(4); // Values unobserved until explicit/source writes.
}
void write_request_vertices(DataSurfaces::SurfaceData &surface, json const &item)
{
    for (int i = 1; i <= 4; ++i) {
        auto const &v = item.at("input_vertex_bits").at(i - 1);
        surface.Vertex(i) = Vector(from_bits(v.at(0)), from_bits(v.at(1)), from_bits(v.at(2)));
    }
}
json case_result(json const &item)
{
    auto state = std::make_unique<EnergyPlus::EnergyPlusData>();
    state->init_constant_state(*state);
    auto errors = std::make_unique<std::ostringstream>();
    auto *error_text = errors.get();
    state->files.err_stream = std::move(errors);
    json messages = json::array();
    state->dataGlobal->errorCallback = [&](EnergyPlus::Error level, std::string const &message) {
        messages.push_back({{"level", static_cast<int>(level)}, {"message", message}});
    };
    auto &surfaces = *state->dataSurface;
    auto &geometry = *state->dataSurfaceGeometry;
    // Allocate real original owners before recording their true default fields.
    surfaces.Surface.allocate(1);
    geometry.SurfaceTmp.allocate(1);
    auto &surface = surfaces.Surface(1);
    auto constructor = Geo02::surface_snapshot(*state, surface, false, false);
    state->dataGlobal->NumOfZones = 1;
    state->dataHeatBal->Zone.allocate(1);
    state->dataHeatBal->Zone(1).Name = "PREPARED-ONE-ZONE";
    state->dataHeatBal->Zone(1).RelNorth = 0.0;
    state->dataHeatBal->BuildingAzimuth = 0.0;
    state->dataHeatBal->BuildingRotationAppendixG = 0.0;
    surfaces.TotSurfaces = 1;
    surfaces.Corner = DataSurfaces::UpperLeftCorner;
    surfaces.CCW = true;
    surfaces.WorldCoordSystem = true;
    surfaces.ShadeV.allocate(1);
    surfaces.X0.allocate(1);
    surfaces.Y0.allocate(1);
    surfaces.Z0.allocate(1);
    surfaces.X0 = 0.0;
    surfaces.Y0 = 0.0;
    surfaces.Z0 = 0.0;
    set_prepared_surface(surface, item);
    auto &temporary = geometry.SurfaceTmp(1);
    set_prepared_surface(temporary, item);
    auto const &initial_centroid = item.at("initial_centroid_m");
    surface.Centroid = Vector(initial_centroid.at(0), initial_centroid.at(1), initial_centroid.at(2));
    auto prepared = Geo02::surface_snapshot(*state, surface, false, false);
    json states = {{"constructor", constructor}, {"prepared", prepared}};
    json diagnostic = json::object();
    bool const valid = item.at("kind") == "valid_quad";
    bool identity = false;
    bool second_unchanged = false;
    if (valid) {
        for (auto const &v : item.at("input_vertex_bits")) {
            for (auto const &token : v) {
                double const value = from_bits(token);
                require(std::isfinite(value) && (value != 0 || !std::signbit(value)), "Valid rawWorld helper requires finite positive geometric zero only");
            }
        }
        original_setup_fragment(*state);
        states["after_original_setup_fragment"] = Geo02::surface_snapshot(*state, surface, false, false);
        Array1D<Real64> numeric(12);
        int pointer = 1;
        for (auto const &v : item.at("input_vertex_bits")) for (auto const &token : v) numeric(pointer++) = from_bits(token);
        SurfaceGeometry::GetVertices(*state, 1, 4, numeric);
        states["after_get_vertices"] = Geo02::surface_snapshot(*state, temporary, true, false);
        identity = temporary.Sides == 4 && Geo02::vertex_bits(temporary) == item.at("input_vertex_bits");
        if (!identity) {
            return {{"case_id", item.at("case_id")}, {"kind", item.at("kind")}, {"input", item},
                    {"status", "source_input_identity_failed"}, {"input_vertex_bits", item.at("input_vertex_bits")},
                    {"retained_vertex_bits", Geo02::vertex_bits(temporary)}, {"kernel_input_identity_checked", false},
                    {"source_state", states}, {"error", "Original GetVertices changed frozen kernel input bits; stop before Rust baseline/comparison"}};
        }
        require(!surfaces.AspectTransform && geometry.noTransform && state->dataErrTracking->TotalCoincidentVertices == 0 &&
                state->dataErrTracking->TotalDegenerateSurfaces == 0, "Valid helper activated excluded geometry correction");
        // Original's own temporary output moves to original's owned Surface.
        // This is reference-internal state flow, never a Rust input.
        surface = temporary;
        SurfaceGeometry::CalcSurfaceCentroid(*state);
        states["after_centroid"] = Geo02::surface_snapshot(*state, surface, true, true);
        bool process_errors = false;
        SurfaceGeometry::ProcessSurfaceVertices(*state, 1, process_errors);
        require(!process_errors && surface.VerticesProcessed, "Original ProcessSurfaceVertices failed selected prepared base surface");
        states["after_process"] = Geo02::surface_snapshot(*state, surface, true, true);
        SurfaceGeometry::ProcessSurfaceVertices(*state, 1, process_errors);
        states["after_second_process"] = Geo02::surface_snapshot(*state, surface, true, true);
        second_unchanged = states.at("after_process") == states.at("after_second_process");
        require(!process_errors && second_unchanged, "Original second Process call did not preserve owned state");
    } else {
        require(item.at("kind") == "source_only_degenerate", "Unknown quad route");
        write_request_vertices(surface, item);
        states["prepared_written_vertices"] = Geo02::surface_snapshot(*state, surface, true, false);
        Vectors::PlaneEq plane;
        plane.x = 17.0;
        plane.y = -23.0;
        plane.z = 31.0;
        plane.w = 47.0;
        bool plane_error = false;
        Vectors::PlaneEquation(surface.Vertex, 4, plane, plane_error);
        Vector zero(0.0);
        auto const normalized_zero = Vectors::VecNormalize(zero);
        Vectors::CreateNewellSurfaceNormalVector(surface.Vertex, 4, surface.NewellSurfaceNormalVector);
        Vectors::CreateNewellAreaVector(surface.Vertex, 4, surface.NewellAreaVector);
        surface.GrossArea = Vectors::VecLength(surface.NewellAreaVector);
        surface.Area = Vectors::AreaPolygon(4, surface.Vertex);
        SurfaceGeometry::CalcSurfaceCentroid(*state);
        states["after_centroid"] = Geo02::surface_snapshot(*state, surface, true, true);
        diagnostic = {{"plane_equation_error", plane_error}, {"plane_after", json::array({Geo02::scalar(plane.x), Geo02::scalar(plane.y), Geo02::scalar(plane.z), Geo02::scalar(plane.w)})},
                      {"normalized_zero", Geo02::vector(normalized_zero)}, {"get_vertices_called", false}, {"process_called", false},
                      {"full_parser_rejection_claimed", false}, {"centroid_retention_observed", Geo02::vector(surface.Centroid)},
                      {"warning_messages", messages}};
    }
    return {{"case_id", item.at("case_id")}, {"kind", item.at("kind")}, {"input", item},
            {"status", valid ? "source_complete" : "unsupported_source_only"},
            {"route", valid ? "original-GetVertices-ownedSurface-CalcSurfaceCentroid-Process-twice" : "original-unsafe-helpers-without-parser-admission"},
            {"input_vertex_bits", item.at("input_vertex_bits")}, {"retained_vertex_bits", Geo02::vertex_bits(surface)},
            {"kernel_input_identity_checked", valid && identity}, {"second_process_owned_state_unchanged", valid && second_unchanged},
            {"geometry", Geo02::geometry(surface)}, {"source_state", states}, {"source_only_diagnostics", diagnostic},
            {"source_messages", messages}, {"source_error_stream", error_text->str()}, {"admission_checked", false}};
}
json cen_result(json const &item)
{
    auto const &tokens = item.at("x_operand_bits");
    require(tokens.size() == 3 && item.at("x_operands").size() == 3, "Cen requires three actual operands");
    Vector const a(from_bits(tokens.at(0)), 0.0, 0.0);
    Vector const b(from_bits(tokens.at(1)), 0.0, 0.0);
    Vector const c(from_bits(tokens.at(2)), 0.0, 0.0);
    auto before = Geo02::floating_environment();
    // The unchanged source-header overload owns sum, extended product and cast.
    Vector const result = ObjexxFCL::cen(a, b, c);
    auto after = Geo02::floating_environment();
    return {{"case_id", item.at("case_id")}, {"input", item}, {"route", "unchanged-ObjexxFCL-cen3"},
            {"value", Geo02::scalar(result.x)}, {"vector_value", Geo02::vector(result)},
            {"floating_environment_before", before}, {"floating_environment_after", after}};
}
} // namespace
int main(int argc, char const *argv[])
{
    try {
        require(argc == 2, "Usage: geo02_reference_helper <input-only-helper-request.json>");
        std::ifstream input(argv[1]);
        require(input.is_open(), "Cannot open helper request");
        auto request = json::parse(input);
        require(request.at("schema") == "geo02-helper-cases.v1", "Wrong helper schema");
        json cases = json::array();
        bool input_identity_passed = true;
        for (auto const &item : request.at("cases")) {
            auto row = case_result(item);
            input_identity_passed = input_identity_passed && row.at("status") != "source_input_identity_failed";
            cases.push_back(std::move(row));
        }
        json cen = json::array();
        for (auto const &item : request.at("cen_calls")) cen.push_back(cen_result(item));
        std::cout << json({{"schema", "geo02-helper-results.v1"}, {"cases", cases}, {"cen_calls", cen},
                          {"valid_kernel_input_identity_passed", input_identity_passed},
                          {"expected_answers_supplied", false}, {"original_parser_admission_claimed", false},
                          {"physics_executed", false}, {"gates_updated", false}}).dump() << '\n';
        return input_identity_passed ? 0 : 3;
    } catch (std::exception const &error) {
        std::cerr << error.what() << '\n';
        return 2;
    }
}
