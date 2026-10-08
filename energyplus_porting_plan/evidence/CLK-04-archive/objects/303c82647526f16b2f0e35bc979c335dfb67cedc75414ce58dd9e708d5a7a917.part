// GEO-03 read-only zone/implicit-space volume observations from a genuine original model run.
// SHA/public-loader plumbing reused from psy02_reachability; no formula oracle.
// The verified GCC DLL owns parsing, geometry initialization, simulation and deletion.
#ifndef WIN32_LEAN_AND_MEAN
#define WIN32_LEAN_AND_MEAN
#endif
#ifndef NOMINMAX
#define NOMINMAX
#endif
#include <windows.h>
#include <bcrypt.h>

#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataEnvironment.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataSurfaces.hh>
#include <EnergyPlus/DataHeatBalance.hh>
#include <EnergyPlus/DataErrorTracking.hh>
#include <EnergyPlus/SurfaceGeometry.hh>
#include <EnergyPlus/InputProcessing/InputProcessor.hh>
#include <EnergyPlus/WeatherManager.hh>
#include <nlohmann/json.hpp>
#include "geo03_reference_fields.hh"

#include <array>
#include <bit>
#include <cstdint>
#include <cstring>
#include <filesystem>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

#if !defined(__MINGW32__) || !defined(_WIN64) || __GNUC__ != 13 || __GNUC_MINOR__ != 2
#error "Private original state observations require the verified GCC13.2 Windows ABI"
#endif
#if defined(_GLIBCXX_DEBUG) || defined(__FAST_MATH__) || defined(NDEBUG)
#error "Reference must use non-debug containers, no fast math, and original enabled assertions"
#endif
#if !defined(EP_psych_errors) || defined(EP_psych_stats)
#error "Observer header cache/error/stats definitions must match the original core"
#endif

namespace {
using json = nlohmann::json;
namespace fs = std::filesystem;
using State = void *;
using Callback = void (*)(State);
using StateNew = State (*)();
using StateDelete = void (*)(State);
using Run = int (*)(State, int, char const *[]);
using Register = void (*)(State, Callback);

void require(bool condition, std::string const &message)
{
    if (!condition) throw std::runtime_error(message);
}

std::string hex(void const *data, std::size_t length)
{
    auto const *bytes = static_cast<unsigned char const *>(data);
    std::ostringstream out;
    out << std::hex << std::setfill('0');
    for (std::size_t i = 0; i < length; ++i) out << std::setw(2) << static_cast<unsigned int>(bytes[i]);
    return out.str();
}

std::string bits(double value)
{
    std::ostringstream out;
    out << std::hex << std::setfill('0') << std::setw(16) << std::bit_cast<std::uint64_t>(value);
    return out.str();
}

class Sha256
{
    BCRYPT_ALG_HANDLE algorithm_ = nullptr;
    BCRYPT_HASH_HANDLE hash_ = nullptr;
    std::vector<unsigned char> object_;
public:
    Sha256()
    {
        require(BCryptOpenAlgorithmProvider(&algorithm_, BCRYPT_SHA256_ALGORITHM, nullptr, 0) >= 0,
                "Cannot open SHA256 provider");
        DWORD length = 0;
        DWORD copied = 0;
        require(BCryptGetProperty(algorithm_, BCRYPT_OBJECT_LENGTH, reinterpret_cast<PUCHAR>(&length),
                                 sizeof(length), &copied, 0) >= 0, "Cannot query SHA256 object size");
        object_.resize(length);
        require(BCryptCreateHash(algorithm_, &hash_, object_.data(), length, nullptr, 0, 0) >= 0,
                "Cannot create SHA256 hash");
    }
    Sha256(Sha256 const &) = delete;
    Sha256 &operator=(Sha256 const &) = delete;
    ~Sha256()
    {
        if (hash_ != nullptr) BCryptDestroyHash(hash_);
        if (algorithm_ != nullptr) BCryptCloseAlgorithmProvider(algorithm_, 0);
    }
    void append(void const *data, std::size_t size)
    {
        require(size <= MAXDWORD, "SHA256 chunk too large");
        // BCrypt's input parameter is mutable in its API type but is read-only.
        auto *input = const_cast<PUCHAR>(static_cast<unsigned char const *>(data));
        require(BCryptHashData(hash_, input, static_cast<ULONG>(size), 0) >= 0, "Cannot hash input");
    }
    std::string finish()
    {
        std::array<unsigned char, 32> digest{};
        require(BCryptFinishHash(hash_, digest.data(), static_cast<ULONG>(digest.size()), 0) >= 0,
                "Cannot finish SHA256 hash");
        return hex(digest.data(), digest.size());
    }
};

std::string file_sha(fs::path const &path)
{
    std::ifstream input(path, std::ios::binary);
    require(input.is_open(), "Cannot open artifact: " + path.string());
    Sha256 hash;
    std::array<char, 65536> block{};
    while (input.read(block.data(), static_cast<std::streamsize>(block.size())) || input.gcount() > 0)
        hash.append(block.data(), static_cast<std::size_t>(input.gcount()));
    require(input.eof(), "Cannot read complete artifact: " + path.string());
    return hash.finish();
}

json file_ref(fs::path const &path)
{
    return {{"path", path.generic_string()}, {"sha256", file_sha(path)}};
}

void write(fs::path const &path, json const &data)
{
    std::ofstream out(path, std::ios::binary);
    require(out.is_open(), "Cannot create observation: " + path.string());
    out << data.dump(2) << '\n';
    require(out.good(), "Cannot write complete observation");
}

template <typename Function> Function symbol(HMODULE library, char const *name)
{
    auto address = GetProcAddress(library, name);
    require(address != nullptr, std::string("Missing original public API symbol: ") + name);
    Function function{};
    static_assert(sizeof(function) == sizeof(address));
    std::memcpy(&function, &address, sizeof(function));
    return function;
}

void number_bits(json const &value, std::string const &path, json &rows)
{
    if (value.is_number()) {
        rows.push_back({{"path", path}, {"bits", bits(value.get<double>())}});
    } else if (value.is_array()) {
        for (std::size_t i = 0; i < value.size(); ++i) number_bits(value[i], path + "/" + std::to_string(i), rows);
    } else if (value.is_object()) {
        for (auto it = value.begin(); it != value.end(); ++it) number_bits(it.value(), path + "/" + it.key(), rows);
    }
}

json parsed_input(EnergyPlus::EnergyPlusData const &state)
{
    json selected = json::object();
    auto const &epjson = state.dataInputProcessing->inputProcessor->epJSON;
    for (char const *kind : {"Building", "Zone", "GlobalGeometryRules", "BuildingSurface:Detailed", "Space", "GeometryTransform", "Compliance:Building"}) {
        if (epjson.contains(kind)) selected[kind] = epjson.at(kind);
    }
    json rows = json::array();
    number_bits(selected, "", rows);
    return {{"objects", selected}, {"numeric_bits", rows},
            {"is_epjson", state.dataGlobal->isEpJSON}, {"preserve_idf_order", state.dataGlobal->preserveIDFOrder},
            {"method", "read genuine original InputProcessor.epJSON and original parser-order flags; no reconstruction"}};
}

json volume_snapshot(EnergyPlus::EnergyPlusData const &state, std::string const &phase, bool vertices_written = true)
{
    return Geo03::snapshot(state, phase, vertices_written);
}

struct Observations
{
    json first_initialized = nullptr;
    json first_physical = nullptr;
    json final_weather = nullptr;
    json input = nullptr;
    std::size_t physical_callbacks = 0;
    std::size_t warmup_callbacks = 0;
    std::string callback_error;
};
Observations *active = nullptr;

void before_zone(State opaque)
{
    try {
        auto const &state = *static_cast<EnergyPlus::EnergyPlusData const *>(opaque);
        require(state.dataSurface->TotSurfaces > 0 && state.dataSurface->Surface.allocated(), "Before-init callback has no initialized original surfaces");
        if (active->first_initialized.is_null()) {
            active->first_initialized = volume_snapshot(state, "first-original-before-init-heat-balance-after-geometry-initialization");
            active->input = parsed_input(state);
            active->first_initialized["warmup"] = state.dataGlobal->WarmupFlag;
            active->first_initialized["kind_of_sim"] = static_cast<int>(state.dataGlobal->KindOfSim);
        }
        if (state.dataGlobal->KindOfSim != EnergyPlus::Constant::KindOfSim::RunPeriodWeather) return;
        if (state.dataGlobal->WarmupFlag) {
            ++active->warmup_callbacks;
        } else {
            ++active->physical_callbacks;
            if (active->first_physical.is_null())
                active->first_physical = volume_snapshot(state, "first-physical-weather-before-init-heat-balance");
        }
    } catch (std::exception const &error) {
        active->callback_error = error.what();
    }
}

void after_zone(State opaque)
{
    try {
        auto const &state = *static_cast<EnergyPlus::EnergyPlusData const *>(opaque);
        if (state.dataGlobal->KindOfSim != EnergyPlus::Constant::KindOfSim::RunPeriodWeather || state.dataGlobal->WarmupFlag || !state.dataGlobal->EndEnvrnFlag) return;
        active->final_weather = volume_snapshot(state, "final-physical-weather-after-zone-reporting-before-wrapup");
    } catch (std::exception const &error) {
        active->callback_error = error.what();
    }
}

bool strict_child(fs::path const &path, fs::path const &root)
{
    auto relative = path.lexically_relative(root);
    return !relative.empty() && relative != "." && *relative.begin() != ".." && !relative.is_absolute();
}
} // namespace

int main(int argc, char const *argv[])
{
    HMODULE library = nullptr;
    State opaque = nullptr;
    StateDelete destroy = nullptr;
    try {
        require(argc == 8, "Usage: geo03_reference <repo-root> <native-core-build.json> <case-id> <input.idf> <weather.epw> <Energy+.idd> <fresh-output-dir>");
        auto repo = fs::canonical(argv[1]);
        auto core_path = fs::canonical(argv[2]);
        std::ifstream core_file(core_path);
        auto core = json::parse(core_file);
        require(core.at("checks_passed") == true && core.at("scientific_source_patches") == false && core.at("same_compiler_abi_required_for_private_state") == true, "Unverified original core provenance");
        require(core.at("energyplus_commit") == "6f2e40d10250a105b49966baa24d843711e61048", "Wrong original source pin");
        auto library_path = fs::canonical(repo / core.at("artifacts").at("api_library").at("path").get<std::string>());
        auto library_ref = file_ref(library_path);
        require(library_ref.at("sha256") == core.at("artifacts").at("api_library").at("sha256") && library_ref.at("sha256") == "1b0146200b27ce5c240a46582626a7c516d9875c1bc2474da81ba7acdcd23f33", "Wrong same-compiler original API DLL");
        auto input = fs::canonical(argv[4]);
        auto weather = fs::canonical(argv[5]);
        auto idd = fs::canonical(argv[6]);
        auto output = fs::weakly_canonical(fs::absolute(argv[7]));
        require(strict_child(output, fs::weakly_canonical(repo / ".runtime/porting/GEO-03")) && !fs::exists(output), "Output must be fresh under GEO-03 runtime");
        fs::create_directories(output);
        library = LoadLibraryExW(library_path.c_str(), nullptr, LOAD_WITH_ALTERED_SEARCH_PATH);
        require(library != nullptr, "Cannot load genuine GCC API DLL; toolchain runtime PATH required");
        auto create = symbol<StateNew>(library, "stateNew");
        destroy = symbol<StateDelete>(library, "stateDelete");
        auto run = symbol<Run>(library, "energyplus");
        auto register_before = symbol<Register>(library, "callbackBeginZoneTimeStepBeforeInitHeatBalance");
        auto register_after = symbol<Register>(library, "callbackEndOfZoneTimeStepAfterZoneReporting");
        opaque = create();
        require(opaque != nullptr, "Original stateNew failed");
        auto const &state = *static_cast<EnergyPlus::EnergyPlusData const *>(opaque);
        auto constructor = volume_snapshot(state, "genuine-original-stateNew-constructor", false);
        write(output / "constructor.json", constructor);
        Observations observations;
        active = &observations;
        register_before(opaque, before_zone);
        register_after(opaque, after_zone);
        std::vector<std::string> arguments = {"energyplus", "-i", idd.string(), "-w", weather.string(), "-d", output.string(), input.string()};
        std::vector<char const *> pointers;
        for (auto const &argument : arguments) pointers.push_back(argument.c_str());
        int const exit_code = run(opaque, static_cast<int>(pointers.size()), pointers.data());
        // A fatal input may leave allocated but unwritten Vertex buffers. Never
        // inspect those values merely because their allocation survived.
        auto final = observations.first_initialized.is_null() ? json(nullptr) : volume_snapshot(state, "genuine-original-energyplus-return-before-stateDelete");
        bool const retained = !observations.first_initialized.is_null() && !final.is_null() && Geo03::retained_identity(observations.first_initialized) == Geo03::retained_identity(final);
        json result = {{"schema", "geo03-original-volume.v1"}, {"case_id", argv[3]},
            {"native_core_build", file_ref(core_path)}, {"library", library_ref}, {"input", file_ref(input)}, {"weather", file_ref(weather)}, {"idd", file_ref(idd)},
            {"command", arguments}, {"energyplus_exit_code", exit_code}, {"constructor", constructor},
            {"parsed_input", observations.input}, {"first_initialized", observations.first_initialized}, {"first_physical", observations.first_physical},
            {"final_weather", observations.final_weather}, {"final", final}, {"first_final_zone_space_surface_identity_exact", retained},
            {"physical_zone_callback_count", observations.physical_callbacks}, {"warmup_zone_callback_count", observations.warmup_callbacks},
            {"callback_error", observations.callback_error}, {"volume_probe_calls_added", 0}, {"state_reset_requested", false},
            {"simulation_inputs_modified", false}, {"gates_updated", false},
            {"scope", "Actual original initialized zone/implicit-space volume, height, floor and entered-height flag; internal CalculateZoneVolume locals, warning parity, multi-space and generic polyhedra unclaimed"}};
        write(output / "volume-observation.json", result);
        active = nullptr;
        destroy(opaque);
        opaque = nullptr;
        FreeLibrary(library);
        library = nullptr;
        require(observations.callback_error.empty(), "Original observation callback failed");
        // The wrapper successfully records original rejection as data. Negative
        // IDF classification belongs to the launcher, not a fabricated success.
        if (exit_code == 0) require(retained && observations.physical_callbacks > 0, "No retained actual initialized zone/space/face state or physical callbacks");
        std::cout << json({{"case_id", argv[3]}, {"physical_zone_callbacks", observations.physical_callbacks}, {"first_final_zone_space_surface_identity_exact", retained}, {"observation", (output / "volume-observation.json").generic_string()}}).dump() << '\n';
        return 0;
    } catch (std::exception const &error) {
        active = nullptr;
        if (opaque != nullptr && destroy != nullptr) destroy(opaque);
        if (library != nullptr) FreeLibrary(library);
        std::cerr << error.what() << '\n';
        return 2;
    }
}
