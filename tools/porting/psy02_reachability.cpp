// Read-only reachability observations from an actual original EnergyPlus run.
// The state is constructed, simulated and destroyed by the verified GCC DLL.
// This executable never calls a psychrometric kernel or changes its state.
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
#include <EnergyPlus/Psychrometrics.hh>
#include <EnergyPlus/WeatherManager.hh>
#include <nlohmann/json.hpp>

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
#if !defined(EP_cache_PsyTsatFnHPb) || !defined(EP_psych_errors) || defined(EP_psych_stats)
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

json cache_snapshot(EnergyPlus::EnergyPlusData const &state, std::string const &phase)
{
    require(state.dataPsychCache != nullptr && state.dataPsychrometrics != nullptr,
            "Original constructor psychrometric state missing");
    auto const &cache = state.dataPsychCache->cached_Tsat_HPb;
    json occupied = json::array();
    std::size_t positive_pressure_tags = 0;
    for (std::size_t i = 0; i < cache.size(); ++i) {
        auto const &entry = cache[i];
        if (entry.iH == 0 && entry.iPb == 0 && std::bit_cast<std::uint64_t>(entry.Tsat) == 0) continue;
        if (entry.iPb > 0) ++positive_pressure_tags;
        occupied.push_back({{"index", i}, {"i_h", entry.iH}, {"i_pb", entry.iPb}, {"value_bits", bits(entry.Tsat)}});
    }
    auto canonical = occupied.dump();
    Sha256 rows_hash;
    rows_hash.append(canonical.data(), canonical.size());
    auto const &p = *state.dataPsychrometrics;
    return {{"phase", phase}, {"capacity", cache.size()}, {"occupied_slot_count", occupied.size()},
            {"positive_nonzero_pressure_tag_count", positive_pressure_tags},
            {"occupied_rows_sha256", rows_hash.finish()}, {"occupied_rows", occupied},
            {"init_constant_state_called", state.init_constant_state_called},
            {"scalar_state", {{"last_patm_bits", bits(p.last_Patm)}, {"last_t_boil_bits", bits(p.last_tBoil)},
                              {"press_save_bits", bits(p.Press_Save)}, {"t_sat_save_bits", bits(p.tSat_Save)},
                              {"iconv_tol_bits", bits(p.iconvTol)}, {"use_interpolation", p.useInterpolationPsychTsatFnPb}}}};
}

struct Observations
{
    std::size_t physical_zone_callbacks = 0;
    std::size_t warmup_zone_callbacks = 0;
    json environment_begins = json::array();
    json completed_weather_environments = json::array();
    std::string callback_error;
};
Observations *active = nullptr;

void environment_begin(State opaque)
{
    try {
        auto const &state = *static_cast<EnergyPlus::EnergyPlusData const *>(opaque);
        auto row = cache_snapshot(state, "original-begin-new-environment-callback");
        row["native_environment_number"] = state.dataWeather->Envrn;
        row["kind_of_sim"] = static_cast<int>(state.dataGlobal->KindOfSim);
        active->environment_begins.push_back(std::move(row));
    } catch (std::exception const &error) {
        active->callback_error = error.what();
    }
}

void after_zone(State opaque)
{
    try {
        auto const &state = *static_cast<EnergyPlus::EnergyPlusData const *>(opaque);
        auto const &global = *state.dataGlobal;
        if (global.KindOfSim != EnergyPlus::Constant::KindOfSim::RunPeriodWeather) return;
        if (global.WarmupFlag) {
            ++active->warmup_zone_callbacks;
            return;
        }
        ++active->physical_zone_callbacks;
        if (!global.EndEnvrnFlag) return;
        auto row = cache_snapshot(state, "original-final-weather-zone-after-reporting-before-wrapup");
        row["native_environment_number"] = state.dataWeather->Envrn;
        row["kind_of_sim"] = static_cast<int>(global.KindOfSim);
        row["day_of_sim"] = global.DayOfSim;
        row["physical_zone_callback_count"] = active->physical_zone_callbacks;
        row["warmup_zone_callback_count"] = active->warmup_zone_callbacks;
        row["out_baro_press_bits"] = bits(state.dataEnvrn->OutBaroPress);
        active->completed_weather_environments.push_back(std::move(row));
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
        require(argc == 8, "Usage: psy02_reachability <repo-root> <native-core-build.json> <case-id> <input.idf> <weather.epw> <Energy+.idd> <fresh-output-dir>");
        auto repo = fs::canonical(argv[1]);
        auto receipt_path = fs::canonical(argv[2]);
        std::ifstream receipt_file(receipt_path);
        auto receipt = json::parse(receipt_file);
        require(receipt.at("checks_passed") == true && receipt.at("scientific_source_patches") == false
                && receipt.at("same_compiler_abi_required_for_private_state") == true,
                "Unverified original core provenance");
        require(receipt.at("energyplus_commit") == "6f2e40d10250a105b49966baa24d843711e61048",
                "Wrong pinned original source");
        auto library_path = fs::canonical(repo / receipt.at("artifacts").at("api_library").at("path").get<std::string>());
        auto library_ref = file_ref(library_path);
        require(library_ref.at("sha256") == receipt.at("artifacts").at("api_library").at("sha256")
                && library_ref.at("sha256") == "1b0146200b27ce5c240a46582626a7c516d9875c1bc2474da81ba7acdcd23f33",
                "This is not the verified same-compiler original API DLL");
        auto input = fs::canonical(argv[4]);
        auto weather = fs::canonical(argv[5]);
        auto idd = fs::canonical(argv[6]);
        auto output = fs::weakly_canonical(fs::absolute(argv[7]));
        auto allowed = fs::weakly_canonical(repo / ".runtime/porting/PSY-02/reachability");
        require(strict_child(output, allowed) && !fs::exists(output), "Observation output must be fresh within PSY-02/reachability");
        fs::create_directories(output);
        library = LoadLibraryExW(library_path.c_str(), nullptr, LOAD_WITH_ALTERED_SEARCH_PATH);
        require(library != nullptr, "Cannot load genuine GCC original API DLL; check original toolchain runtime PATH");
        auto create = symbol<StateNew>(library, "stateNew");
        destroy = symbol<StateDelete>(library, "stateDelete");
        auto run = symbol<Run>(library, "energyplus");
        auto register_begin = symbol<Register>(library, "callbackBeginNewEnvironment");
        auto register_after = symbol<Register>(library, "callbackEndOfZoneTimeStepAfterZoneReporting");
        opaque = create();
        require(opaque != nullptr, "Original stateNew failed");
        auto const &state = *static_cast<EnergyPlus::EnergyPlusData const *>(opaque);
        auto constructor = cache_snapshot(state, "genuine-original-stateNew-constructor");
        require(constructor.at("occupied_slot_count") == 0, "Fresh original HPb cache is not empty");
        write(output / "constructor.json", constructor);
        Observations observations;
        active = &observations;
        register_begin(opaque, environment_begin);
        register_after(opaque, after_zone);
        std::vector<std::string> arguments = {"energyplus", "-i", idd.string(), "-w", weather.string(),
                                              "-d", output.string(), input.string()};
        std::vector<char const *> pointers;
        for (auto const &argument : arguments) pointers.push_back(argument.c_str());
        int const exit_code = run(opaque, static_cast<int>(pointers.size()), pointers.data());
        auto final = cache_snapshot(state, "genuine-original-energyplus-return-before-stateDelete");
        json result = {{"schema", "psy02-original-hpb-reachability.v1"}, {"case_id", argv[3]},
            {"library", library_ref}, {"native_core_build", file_ref(receipt_path)},
            {"input", file_ref(input)}, {"weather", file_ref(weather)}, {"idd", file_ref(idd)},
            {"command", arguments}, {"energyplus_exit_code", exit_code},
            {"constructor", constructor}, {"environment_begins", observations.environment_begins},
            {"completed_weather_environments", observations.completed_weather_environments}, {"final", final},
            {"physical_zone_callback_count", observations.physical_zone_callbacks},
            {"warmup_zone_callback_count", observations.warmup_zone_callbacks},
            {"callback_error", observations.callback_error}, {"psychrometric_probe_calls_added", 0},
            {"state_reset_requested", false}, {"simulation_inputs_modified", false},
            {"scope", "Retained original cache-write reachability across genuine run including warmup; no exact call count or caller/guard attribution"},
            {"zero_interpretation_requires_source_lifetime_audit", true}, {"automatic_gate_updates", false}};
        write(output / "observation.json", result);
        active = nullptr;
        destroy(opaque);
        opaque = nullptr;
        FreeLibrary(library);
        library = nullptr;
        require(exit_code == 0 && observations.callback_error.empty(), "Original simulation/observation did not complete successfully");
        require(observations.physical_zone_callbacks > 0 && !observations.completed_weather_environments.empty(),
                "No completed genuine weather environment observed");
        std::cout << json({{"case_id", argv[3]}, {"physical_zone_callbacks", observations.physical_zone_callbacks},
                           {"final_occupied_hpb_slots", final.at("occupied_slot_count")}, {"observation", (output / "observation.json").generic_string()}}).dump() << '\n';
        return 0;
    } catch (std::exception const &error) {
        active = nullptr;
        if (opaque != nullptr && destroy != nullptr) destroy(opaque);
        if (library != nullptr) FreeLibrary(library);
        std::cerr << error.what() << '\n';
        return 2;
    }
}
