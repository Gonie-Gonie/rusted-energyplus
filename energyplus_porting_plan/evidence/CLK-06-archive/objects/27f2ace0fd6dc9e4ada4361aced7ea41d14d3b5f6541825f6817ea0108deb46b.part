// ZON-01 passive initialization-stage observations from a genuine original model run.
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
#include "zon01_reference_fields.hh"

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
        rows.push_back({{"path", path}, {"bits", Zon01::bits(value.get<double>())}});
    } else if (value.is_array()) {
        for (std::size_t i = 0; i < value.size(); ++i) number_bits(value[i], path + "/" + std::to_string(i), rows);
    } else if (value.is_object()) {
        for (auto it = value.begin(); it != value.end(); ++it) number_bits(it.value(), path + "/" + it.key(), rows);
    }
}
json parsed_input(EnergyPlus::EnergyPlusData const &state)
{
    auto const &source = state.dataInputProcessing->inputProcessor->epJSON;
    json selected = json::object();
    for (char const *kind : {"Building", "Zone", "RunPeriod", "Timestep", "SimulationControl", "ZoneAirHeatBalanceAlgorithm",
                             "ZoneControl:Thermostat", "ZoneHVAC:EquipmentConnections", "ZoneHVAC:IdealLoadsAirSystem"}) {
        if (source.contains(kind)) selected[kind] = source.at(kind);
    }
    json rows = json::array();
    number_bits(selected, "", rows);
    return {{"objects", selected}, {"numeric_bits", rows}, {"is_epjson", state.dataGlobal->isEpJSON},
            {"preserve_idf_order", state.dataGlobal->preserveIDFOrder},
            {"method", "copied actual InputProcessor.epJSON; no reconstructed initialization inputs"}};
}
struct Observations
{
    std::ofstream trace;
    json first_by_phase = json::object();
    json first_physical_by_phase = json::object();
    json last_by_phase = json::object();
    json counts = json::object();
    json input = nullptr;
    std::uint64_t sequence = 0;
    std::uint64_t callback_attempts = 0;
    std::uint64_t omitted_callbacks = 0;
    std::string callback_error;
};
Observations *active = nullptr;
void observe(State opaque, char const *phase, char const *source_stage)
{
    ++active->callback_attempts;
    try {
        auto const &state = *static_cast<EnergyPlus::EnergyPlusData const *>(opaque);
        auto row = Zon01::snapshot(state, phase);
        row["schema"] = "zon01-native-stage-row.v1";
        row["sequence"] = ++active->sequence;
        row["source_stage"] = source_stage;
        row["pristine_begin_environment_result_claimed"] = false;
        row["source_guard_eligibility"] = state.dataZoneTempPredictorCorrector->MyEnvrnFlag && state.dataGlobal->BeginEnvrnFlag;
        bool const physical = state.dataGlobal->KindOfSim == EnergyPlus::Constant::KindOfSim::RunPeriodWeather &&
                              !state.dataGlobal->WarmupFlag && !state.dataGlobal->KickOffSimulation;
        std::string const category = physical ? "physical_weather" : state.dataGlobal->KickOffSimulation ? "kickoff" :
                                     state.dataGlobal->WarmupFlag ? "warmup" : "other";
        row["callback_category"] = category;
        if (!active->counts.contains(phase)) active->counts[phase] = {{"total", 0}, {"physical_weather", 0}, {"kickoff", 0}, {"warmup", 0}, {"other", 0}};
        auto &counts = active->counts[phase];
        counts["total"] = counts.at("total").get<std::uint64_t>() + 1;
        counts[category] = counts.at(category).get<std::uint64_t>() + 1;
        if (!active->first_by_phase.contains(phase)) active->first_by_phase[phase] = row;
        if (physical && !active->first_physical_by_phase.contains(phase)) active->first_physical_by_phase[phase] = row;
        active->last_by_phase[phase] = row;
        if (active->input.is_null()) active->input = parsed_input(state);
        active->trace << row.dump() << '\n';
        require(active->trace.good(), "Cannot append original passive callback row");
    } catch (std::exception const &error) {
        ++active->omitted_callbacks;
        if (active->callback_error.empty()) active->callback_error = error.what();
    }
}
void after_heat_balance_init(State state)
{
    observe(state, "after_init_heat_balance", "HeatBalanceManager.cc199-200 after InitHeatBalance but before surface bulk reconstruction");
}
void before_predictor(State state)
{
    observe(state, "before_predictor", "HVACManager.cc217 after bulk and HVAC165-170 mutations; before GetZoneSetPoints guard/member");
}
void after_predictor(State state)
{
    observe(state, "after_predictor_before_hvac_managers", "HVACManager.cc844 after prediction working-history/capacity/load updates; not pristine member output");
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
        require(argc == 8, "Usage: zon01_reference <repo-root> <native-core-build.json> <case-id> <input.idf> <weather.epw> <Energy+.idd> <fresh-output-dir>");
        auto repo = fs::canonical(argv[1]);
        auto core_path = fs::canonical(argv[2]);
        std::ifstream stream(core_path);
        auto core = json::parse(stream);
        require(core.at("checks_passed") == true && core.at("scientific_source_patches") == false &&
                core.at("same_compiler_abi_required_for_private_state") == true, "Unverified original core provenance");
        require(core.at("energyplus_commit") == "6f2e40d10250a105b49966baa24d843711e61048", "Wrong original source commit");
        auto library_path = fs::canonical(repo / core.at("artifacts").at("api_library").at("path").get<std::string>());
        auto library_ref = file_ref(library_path);
        require(library_ref.at("sha256") == core.at("artifacts").at("api_library").at("sha256") &&
                library_ref.at("sha256") == "1b0146200b27ce5c240a46582626a7c516d9875c1bc2474da81ba7acdcd23f33", "Wrong genuine same-GCC API DLL");
        auto input = fs::canonical(argv[4]);
        auto weather = fs::canonical(argv[5]);
        auto idd = fs::canonical(argv[6]);
        auto output = fs::weakly_canonical(fs::absolute(argv[7]));
        require(strict_child(output, fs::weakly_canonical(repo / ".runtime/porting/ZON-01")) && !fs::exists(output), "Fresh contained ZON-01 output required");
        fs::create_directories(output);
        library = LoadLibraryExW(library_path.c_str(), nullptr, LOAD_WITH_ALTERED_SEARCH_PATH);
        require(library != nullptr, "Cannot load genuine GCC API DLL; matching runtime PATH required");
        auto create = symbol<StateNew>(library, "stateNew");
        destroy = symbol<StateDelete>(library, "stateDelete");
        auto run = symbol<Run>(library, "energyplus");
        opaque = create();
        require(opaque != nullptr, "Genuine stateNew failed");
        auto const &state = *static_cast<EnergyPlus::EnergyPlusData const *>(opaque);
        auto constructor = Zon01::snapshot(state, "genuine-public-stateNew-no-fabricated-zone-rows");
        write(output / "constructor.json", constructor);
        Observations observations;
        auto trace_path = output / "zone-air-stage-observations.jsonl";
        observations.trace.open(trace_path, std::ios::binary);
        require(observations.trace.is_open(), "Cannot create passive source-stage trace");
        active = &observations;
        symbol<Register>(library, "callbackBeginZoneTimeStepAfterInitHeatBalance")(opaque, after_heat_balance_init);
        symbol<Register>(library, "callbackBeginTimeStepBeforePredictor")(opaque, before_predictor);
        symbol<Register>(library, "callbackAfterPredictorBeforeHVACManagers")(opaque, after_predictor);
        std::vector<std::string> arguments = {"energyplus", "-i", idd.string(), "-w", weather.string(), "-d", output.string(), input.string()};
        std::vector<char const *> pointers;
        for (auto const &argument : arguments) pointers.push_back(argument.c_str());
        int const code = run(opaque, static_cast<int>(pointers.size()), pointers.data());
        observations.trace.flush();
        require(observations.trace.good(), "Incomplete passive callback trace");
        observations.trace.close();
        json result = {{"schema", "zon01-original-stages.v1"}, {"case_id", argv[3]}, {"native_core_build", file_ref(core_path)},
            {"library", library_ref}, {"input", file_ref(input)}, {"weather", file_ref(weather)}, {"idd", file_ref(idd)},
            {"command", arguments}, {"energyplus_exit_code", code}, {"constructor", constructor},
            {"parsed_input", observations.input}, {"callback_counts", observations.counts},
            {"first_by_phase", observations.first_by_phase}, {"first_physical_by_phase", observations.first_physical_by_phase},
            {"last_by_phase", observations.last_by_phase}, {"final", Zon01::snapshot(state, "original-energyplus-return-before-stateDelete")},
            {"ordered_observations", file_ref(trace_path)}, {"recorded_callback_count", observations.sequence},
            {"callback_attempt_count", observations.callback_attempts}, {"omitted_callback_count", observations.omitted_callbacks},
            {"callback_error", observations.callback_error},
            {"initializer_probe_calls_added", 0}, {"member_call_count_observed", false},
            {"state_reset_requested", false}, {"simulation_inputs_modified", false}, {"gates_updated", false},
            {"scope", "Actual source callback stages and inputs/guards; neither direct post-member output nor warmup/solver parity"}};
        write(output / "zone-air-observation.json", result);
        active = nullptr;
        destroy(opaque);
        opaque = nullptr;
        FreeLibrary(library);
        library = nullptr;
        require(observations.callback_error.empty(), "Original observer failed; raw data retained");
        std::cout << json({{"case_id", argv[3]}, {"energyplus_exit_code", code}, {"recorded_callback_count", observations.sequence},
                         {"observation", (output / "zone-air-observation.json").generic_string()}}).dump() << '\n';
        return 0;
    } catch (std::exception const &error) {
        active = nullptr;
        if (opaque != nullptr && destroy != nullptr) destroy(opaque);
        if (library != nullptr) FreeLibrary(library);
        std::cerr << error.what() << '\n';
        return 2;
    }
}
