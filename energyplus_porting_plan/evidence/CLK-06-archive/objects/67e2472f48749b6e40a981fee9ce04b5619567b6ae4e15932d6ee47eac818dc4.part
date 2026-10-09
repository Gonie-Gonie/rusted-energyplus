// Genuine original GEO-03 volume/height state, with explicit input-only preparation.
// Numerical producers are whole unchanged core functions. Setup's height loop
// is an exact archived source fragment, not a rewritten height formula.
#include "geo03_reference_fields.hh"
#include <EnergyPlus/DataEnvironment.hh>
#include <EnergyPlus/DataGlobalConstants.hh>
#include <EnergyPlus/InputProcessing/InputProcessor.hh>
#include <EnergyPlus/UtilityRoutines.hh>
#include <ObjexxFCL/member.functions.hh>
#include <fstream>
#include <iostream>
#include <memory>
#include <stdexcept>
#include <string>

namespace {
using Geo03::json;
using namespace EnergyPlus;
using EnergyPlus::DataVectorTypes::Vector;
using EnergyPlus::DataSurfaces::SurfaceClass;

void require(bool condition, std::string const &message)
{
    if (!condition) throw std::runtime_error(message);
}

double from_bits(std::string const &token)
{
    require(token.size() == 16 && token.find_first_not_of("0123456789abcdef") == std::string::npos, "Invalid input binary64 bits");
    return std::bit_cast<double>(static_cast<std::uint64_t>(std::stoull(token, nullptr, 16)));
}

double input_scalar(json const &value)
{
    if (value.is_string()) {
        require(value == "AutoCalculate", "Unknown numerical input token");
        return Constant::AutoCalculate;
    }
    require(value.is_number() && !value.is_boolean(), "Expected finite binary64 input");
    double const number = value.get<double>();
    require(std::isfinite(number), "Nonfinite prepared input excluded");
    return number;
}

void validate_request_inputs(json const &item)
{
    require(item.at("kind") == "closed_box" || item.at("kind") == "source_only_nonclosed" ||
            item.at("kind") == "source_only_reversed_winding", "Unknown prepared helper kind");
    bool const reversed = item.at("kind") == "source_only_reversed_winding";
    require(item.at("route") == (reversed ? "prepared-reversed-winding-direct-helpers-bypass-GetVertices" :
                                "original-GetVertices-declared-preparation-original-height-fragment-CalculateZoneVolume"),
            "Frozen helper kind/route differs");
    for (char const *group : {"zone_input", "prepared_zone", "prepared_implicit_space"}) {
        auto const &values = item.at(group);
        auto const &authoritative = item.at("input_scalar_bits").at(group);
        std::size_t numeric_count = 0;
        for (auto it = values.begin(); it != values.end(); ++it) {
            if (it.value().is_number()) {
                ++numeric_count;
                require(authoritative.contains(it.key()) && Geo02::bits(input_scalar(it.value())) == authoritative.at(it.key()).get<std::string>(),
                        "Independently parsed scalar differs from frozen input bits");
            } else if (it.value().is_string()) {
                input_scalar(it.value());
            } else {
                require(it.value().is_boolean(), "Unexpected prepared field type");
            }
        }
        require(authoritative.size() == numeric_count, "Unbound or extra scalar input-bit fields");
    }
    for (auto const &face : item.at("surfaces")) {
        require(face.at("vertices_m").size() == face.at("input_vertex_bits").size(), "Input vertex cardinality differs");
        for (std::size_t i = 0; i < face.at("vertices_m").size(); ++i) {
            require(face.at("vertices_m").at(i).size() == 3 && face.at("input_vertex_bits").at(i).size() == 3,
                    "Input coordinate cardinality differs");
            for (int j = 0; j < 3; ++j) {
                double const value = input_scalar(face.at("vertices_m").at(i).at(j));
                require(Geo02::bits(value) == face.at("input_vertex_bits").at(i).at(j).get<std::string>(), "Parsed vertex differs from frozen input bits");
                if (item.at("kind") == "closed_box")
                    require(value != 0.0 || !std::signbit(value), "Paired World helper requires positive geometric zero");
            }
        }
    }
    if (item.at("kind") == "closed_box") {
        require(input_scalar(item.at("prepared_zone").at("geometric_floor_area_m2")) > 0.0 &&
                input_scalar(item.at("prepared_zone").at("geometric_ceiling_area_m2")) > 0.0,
                "Paired original height preparation requires positive geometric floor/ceiling denominators");
    }
}

void original_trig_preparation(EnergyPlus::EnergyPlusData &state)
{
    // Existing immutable fragment273-289, included without byte changes.
    // Explicitly not the full SetupZoneGeometry or input parser.
#include GEO03_TRIG_FRAGMENT
}

void original_height_preparation(EnergyPlus::EnergyPlusData &state)
{
    bool const DetailedWWR = false; // Frozen input: debug reporting inactive.
    // Exact source caller declaration at SurfaceGeometry.cc267.
    static constexpr std::string_view RoutineName("SetUpZoneGeometry: ");
    // Byte-exact original source455-546, copied and SHA-bound before compile.
#include GEO03_HEIGHT_FRAGMENT
}

SurfaceClass surface_class(std::string const &name)
{
    if (name == "Wall") return SurfaceClass::Wall;
    if (name == "Floor") return SurfaceClass::Floor;
    if (name == "Roof") return SurfaceClass::Roof;
    throw std::runtime_error("Unsupported prepared face class");
}

void prepare_surface(DataSurfaces::SurfaceData &surface, json const &item, int index)
{
    surface.Name = item.at("name").get<std::string>();
    surface.Zone = 1;
    surface.ZoneName = "PREPARED-ONE-ZONE";
    surface.spaceNum = 1;
    surface.Class = surface_class(item.at("class"));
    surface.BaseSurf = index;
    surface.HeatTransSurf = true;
    surface.Sides = static_cast<int>(item.at("input_vertex_bits").size());
    require(surface.Sides == 4, "Prepared helper admits four-vertex faces only");
    surface.Vertex.allocate(surface.Sides); // No unwritten values are read.
}

void request_vertices(DataSurfaces::SurfaceData &surface, json const &item)
{
    for (int i = 1; i <= surface.Sides; ++i) {
        auto const &v = item.at("input_vertex_bits").at(i - 1);
        require(v.size() == 3, "Vertex coordinate count differs");
        surface.Vertex(i) = Vector(from_bits(v.at(0)), from_bits(v.at(1)), from_bits(v.at(2)));
    }
}

void reversed_geometry_prerequisites(DataSurfaces::SurfaceData &surface, json const &item)
{
    // Source-only wrong-winding diagnostic bypasses GetVertices' roof/floor
    // auto-reversal. All geometry prerequisites use original public helpers.
    request_vertices(surface, item);
    Vectors::CreateNewellAreaVector(surface.Vertex, surface.Sides, surface.NewellAreaVector);
    Vectors::CreateNewellSurfaceNormalVector(surface.Vertex, surface.Sides, surface.NewellSurfaceNormalVector);
    surface.Area = Vectors::AreaPolygon(surface.Sides, surface.Vertex);
    surface.GrossArea = Vectors::VecLength(surface.NewellAreaVector);
    Vectors::DetermineAzimuthAndTilt(surface.Vertex, surface.Azimuth, surface.Tilt,
                                    surface.lcsx, surface.lcsy, surface.lcsz, surface.NewellSurfaceNormalVector);
}

void prepare_read_fields(EnergyPlus::EnergyPlusData &state, json const &item)
{
    auto &zone = state.dataHeatBal->Zone(1);
    auto &space = state.dataHeatBal->space(1);
    auto const &input = item.at("zone_input");
    zone.CeilingHeight = input_scalar(input.at("ceiling_height_m"));
    zone.Volume = input_scalar(input.at("volume_m3"));
    zone.UserEnteredFloorArea = input_scalar(input.at("user_entered_floor_area_m2"));
    auto const &prepared = item.at("prepared_zone");
    zone.FloorArea = input_scalar(prepared.at("floor_area_m2"));
    zone.geometricFloorArea = input_scalar(prepared.at("geometric_floor_area_m2"));
    zone.CeilingArea = input_scalar(prepared.at("ceiling_area_m2"));
    zone.geometricCeilingArea = input_scalar(prepared.at("geometric_ceiling_area_m2"));
    zone.HasFloor = prepared.at("has_floor").get<bool>();
    zone.HasRoof = prepared.at("has_roof").get<bool>();
    auto const &implicit = item.at("prepared_implicit_space");
    space.FloorArea = input_scalar(implicit.at("floor_area_m2"));
    space.Volume = input_scalar(implicit.at("volume_m3"));
    space.hasFloor = implicit.at("has_floor").get<bool>();
    space.fracZoneFloorArea = input_scalar(implicit.at("frac_zone_floor_area"));
    // These read fields are declared factory inputs. They are not presented as
    // original GetSurfaceData floor-area or implicit-space producer outputs.
}

json auxiliary_helpers(EnergyPlus::EnergyPlusData &state)
{
    Vectors::Polyhedron poly;
    poly.NumSurfaceFaces = state.dataSurface->TotSurfaces;
    poly.SurfaceFace.allocate(poly.NumSurfaceFaces);
    for (int i = 1; i <= poly.NumSurfaceFaces; ++i) {
        auto const &s = state.dataSurface->Surface(i);
        auto &face = poly.SurfaceFace(i);
        face.NSides = s.Sides;
        face.SurfNum = i;
        face.FacePoints.allocate(s.Sides);
        face.FacePoints = s.Vertex;
        Vectors::CreateNewellAreaVector(face.FacePoints, face.NSides, face.NewellAreaVector);
    }
    auto const unique_vertices = SurfaceGeometry::makeListOfUniqueVertices(poly);
    auto const initial_edges = SurfaceGeometry::edgesNotTwoForEnclosedVolumeTest(poly, unique_vertices);
    std::vector<SurfaceGeometry::EdgeOfSurf> edges;
    bool const enclosed = SurfaceGeometry::isEnclosedVolume(poly, edges);
    auto const [floor_horizontal, roof_horizontal, walls_vertical] = SurfaceGeometry::areSurfaceHorizAndVert(state, poly);
    bool const same_wall_height = SurfaceGeometry::areWallHeightSame(state, poly);
    double const signed_volume = Vectors::CalcPolyhedronVolume(state, poly);
    auto edge_rows = [](std::vector<SurfaceGeometry::EdgeOfSurf> const &observed) {
        json rows = json::array();
        for (auto const &edge : observed)
            rows.push_back({{"surface_id", edge.surfNum}, {"count", edge.count}, {"other_surface_ids", edge.otherSurfNums},
                            {"start_m", Geo03::vector(edge.start)}, {"end_m", Geo03::vector(edge.end)}});
        return rows;
    };
    return {{"method", "additional-original-helper-observations-not-internal-local-trace"},
            {"initial_unique_vertex_count", unique_vertices.size()}, {"initial_edges_not_used_twice", edge_rows(initial_edges)},
            {"enclosed", enclosed}, {"edges_not_used_twice", edge_rows(edges)},
            {"floor_horizontal", floor_horizontal}, {"roof_horizontal", roof_horizontal}, {"walls_vertical", walls_vertical},
            {"same_wall_height", same_wall_height}, {"signed_polyhedron_volume_m3", Geo03::scalar(signed_volume)},
            {"unobservable_CalculateZoneVolume_local_method_reported", false}};
}

json case_result(json const &item)
{
    auto state = std::make_unique<EnergyPlus::EnergyPlusData>();
    auto errors = std::make_unique<std::ostringstream>();
    auto *error_text = errors.get();
    state->files.err_stream = std::move(errors);
    json messages = json::array();
    state->dataGlobal->errorCallback = [&](Error level, std::string const &message) {
        messages.push_back({{"level", static_cast<int>(level)}, {"message", message}});
    };
    auto constructor = Geo03::snapshot(*state, "genuine-original-constructor", false);
    state->init_constant_state(*state);
    int const count = static_cast<int>(item.at("surfaces").size());
    require(count >= 5 && count <= 7, "Frozen helper face count out of domain");
    state->dataGlobal->NumOfZones = 1;
    state->dataHeatBal->Zone.allocate(1);
    state->dataHeatBal->space.allocate(1);
    auto &surfaces = *state->dataSurface;
    auto &geometry = *state->dataSurfaceGeometry;
    surfaces.Surface.allocate(count);
    geometry.SurfaceTmp.allocate(count);
    // Genuine constructed owner values before the factory declares names,
    // topology, numeric inputs or any written Vertex values.
    auto allocated_defaults = Geo03::snapshot(*state, "allocated-original-owner-defaults-before-input-writes", false);
    auto &zone = state->dataHeatBal->Zone(1);
    auto &space = state->dataHeatBal->space(1);
    zone.Name = "PREPARED-ONE-ZONE";
    zone.numSpaces = 1;
    zone.spaceIndexes.push_back(1);
    zone.AllSurfaceFirst = 1;
    zone.AllSurfaceLast = count;
    space.Name = "PREPARED-IMPLICIT-SPACE";
    space.zoneNum = 1;
    space.AllSurfaceFirst = 1;
    space.AllSurfaceLast = count;
    surfaces.TotSurfaces = count;
    surfaces.Corner = DataSurfaces::UpperLeftCorner;
    surfaces.CCW = true;
    surfaces.WorldCoordSystem = true;
    original_trig_preparation(*state);
    bool const raw_reversed = item.at("route") == "prepared-reversed-winding-direct-helpers-bypass-GetVertices";
    bool identity = true;
    for (int i = 1; i <= count; ++i) {
        auto const &face = item.at("surfaces").at(i - 1);
        auto &owned = surfaces.Surface(i);
        prepare_surface(owned, face, i);
        space.surfaces.push_back(i);
        if (raw_reversed) {
            reversed_geometry_prerequisites(owned, face);
        } else {
            auto &temporary = geometry.SurfaceTmp(i);
            prepare_surface(temporary, face, i);
            Array1D<Real64> numeric(12);
            int pointer = 1;
            for (auto const &vertex : face.at("input_vertex_bits")) {
                for (auto const &token : vertex) {
                    double const value = from_bits(token);
                    require(std::isfinite(value), "Finite geometry required");
                    numeric(pointer++) = value;
                }
            }
            SurfaceGeometry::GetVertices(*state, i, 4, numeric);
            owned = temporary;
        }
        identity = identity && owned.Sides == 4 && Geo02::vertex_bits(owned) == face.at("input_vertex_bits");
    }
    json states = {{"constructor", constructor}, {"allocated_owner_defaults", allocated_defaults},
                   {"geometry_prepared", Geo03::snapshot(*state, "after-original-geometry-prerequisites", true)}};
    bool const paired = item.at("kind") == "closed_box";
    if (paired) {
        require(identity, "Original GetVertices changed paired request vertex bits; pairing must stop");
        require(!raw_reversed && count == 6 && !surfaces.AspectTransform && geometry.noTransform &&
                state->dataErrTracking->TotalCoincidentVertices == 0 && state->dataErrTracking->TotalDegenerateSurfaces == 0,
                "Paired helper activated excluded geometry correction");
    }
    prepare_read_fields(*state, item);
    states["declared_pre_volume_read_fields"] = Geo03::snapshot(*state, "declared-input-only-prepared-read-fields", true);
    original_height_preparation(*state);
    states["before_volume"] = Geo03::snapshot(*state, "after-exact-height-fragment-before-original-volume", true);
    auto auxiliaries = auxiliary_helpers(*state);
    if (paired) require(auxiliaries.at("enclosed") == true && auxiliaries.at("initial_edges_not_used_twice").empty() &&
                        auxiliaries.at("edges_not_used_twice").empty(),
                        "Paired helper is not originally enclosed; pairing must stop");
    SurfaceGeometry::CalculateZoneVolume(*state);
    states["after_volume"] = Geo03::snapshot(*state, "after-first-original-CalculateZoneVolume", true);
    SurfaceGeometry::CalculateZoneVolume(*state);
    states["after_second_volume"] = Geo03::snapshot(*state, "after-second-original-CalculateZoneVolume-without-height-fragment-repeat", true);
    bool const second_retained = Geo03::retained_identity(states.at("after_volume")) == Geo03::retained_identity(states.at("after_second_volume"));
    return {{"case_id", item.at("case_id")}, {"kind", item.at("kind")}, {"input", item},
            {"status", paired ? "source_complete" : "unsupported_source_only"}, {"route", item.at("route")},
            {"kernel_input_identity_checked", paired && identity}, {"input_vertex_identity_exact", identity},
            {"zone", Geo03::zone(zone, 1)}, {"source_state", states}, {"auxiliary_original_helpers", auxiliaries},
            {"source_only_diagnostics", {{"messages", messages}, {"error_stream", error_text->str()},
                 {"second_call_current_positive_volume_retention", second_retained},
                 {"source_local_CalcVolume_and_method_observed", false}}},
            {"original_parser_admission_claimed", false}, {"physics_executed", false}};
}
} // namespace

int main(int argc, char const *argv[])
{
    try {
        require(argc == 2, "Usage: geo03_reference_helper <input-only-helper-request.json>");
        std::ifstream input(argv[1]);
        require(input.is_open(), "Cannot open input-only helper request");
        auto request = json::parse(input);
        require(request.at("schema") == "geo03-helper-cases.v1", "Wrong helper schema");
        json cases = json::array();
        for (auto const &item : request.at("cases")) {
            validate_request_inputs(item);
            cases.push_back(case_result(item));
        }
        std::cout << json({{"schema", "geo03-helper-results.v1"}, {"cases", cases},
                          {"expected_answers_supplied", false}, {"original_parser_admission_claimed", false},
                          {"physics_executed", false}, {"gates_updated", false}}).dump() << '\n';
        return 0;
    } catch (std::exception const &error) {
        std::cerr << error.what() << '\n';
        return 2;
    }
}
