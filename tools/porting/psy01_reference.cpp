// Test-only dispatcher: all property calculations come from the pinned original header.
// Compile with assertions enabled and EP_psych_errors undefined (warning aggregation
// is outside PSY-01). The runtime header is an exact original-header copy with six
// documented read-only Cp observers; no formula, branch, or assignment is changed.
namespace psy01 {
void observe_before(bool fast, double w, double cp);
void observe_after(bool fast, double w, double cp, bool hit);
}
#include "psy01_original_instrumented.hh"
#include <nlohmann/json.hpp>

#ifdef EP_psych_errors
#error PSY-01 excludes psychrometric warning aggregation
#endif
#ifdef EP_psych_stats
#error PSY-01 excludes psychrometric statistics aggregation
#endif
#ifdef NDEBUG
#error PSY-01 fast-path precondition assertions must remain enabled
#endif
#define NOMINMAX
#include <windows.h>

#include <bit>
#include <array>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <limits>
#include <map>
#include <sstream>
#include <stdexcept>
#include <unordered_map>
#include <vector>

using json = nlohmann::json;

struct CacheObservation
{
    double before_w = 0, before_cp = 0, after_w = 0, after_cp = 0;
    bool fast = false, hit = false, observed = false;
} cache_observation;

void psy01::observe_before(bool fast, double w, double cp)
{
    cache_observation = {w, cp, 0, 0, fast, false, true};
}

void psy01::observe_after(bool fast, double w, double cp, bool hit)
{
    cache_observation.after_w = w;
    cache_observation.after_cp = cp;
    cache_observation.fast = fast;
    cache_observation.hit = hit;
}

struct NativeState
{
    HMODULE library = nullptr;
    EnergyPlus::EnergyPlusData *state = nullptr;
    using Delete = void (*)(void *);
    Delete destroy = nullptr;

    explicit NativeState(char const *path)
    {
        library = LoadLibraryA(path);
        if (!library) throw std::runtime_error("Cannot load pinned EnergyPlus state library");
        auto create = reinterpret_cast<void *(*)()>(GetProcAddress(library, "stateNew"));
        destroy = reinterpret_cast<Delete>(GetProcAddress(library, "stateDelete"));
        if (!create || !destroy) throw std::runtime_error("EnergyPlus stateNew/stateDelete exports missing");
        state = static_cast<EnergyPlus::EnergyPlusData *>(create());
        if (!state) throw std::runtime_error("EnergyPlus stateNew returned null");
    }

    ~NativeState()
    {
        if (state) destroy(state);
        if (library) FreeLibrary(library);
    }
};

double scalar(json const &value)
{
    if (value.is_number()) return value.get<double>();
    if (value == "NaN") return std::numeric_limits<double>::quiet_NaN();
    if (value == "+Infinity") return std::numeric_limits<double>::infinity();
    if (value == "-Infinity") return -std::numeric_limits<double>::infinity();
    throw std::runtime_error("Invalid numeric scalar");
}

json encoded(double value)
{
    if (std::isnan(value)) return "NaN";
    if (std::isinf(value)) return std::signbit(value) ? "-Infinity" : "+Infinity";
    return value;
}

std::string classification(double value)
{
    if (std::isnan(value)) return "nan";
    if (std::isinf(value)) return std::signbit(value) ? "-infinity" : "+infinity";
    return "finite";
}

std::string bits(double value)
{
    std::ostringstream text;
    text << std::hex << std::setfill('0') << std::setw(16) << std::bit_cast<std::uint64_t>(value);
    return text.str();
}

enum class Routine { Cp, CpFast, Rho, RhoFast, H, HFast, Tdb, W };

struct PreparedCall
{
    Routine routine;
    double t = 0, w = 0, p = 0, h = 0;
    std::string unit;
    json identity;
    json resolved;
    bool cp = false;
};

PreparedCall prepare_compressed(json const &row)
{
    PreparedCall call;
    call.identity = row;
    std::string const function = row.at("function");
    auto const &inputs = row.at("inputs");
    call.resolved = json::object();
    for (auto const &[key, value] : inputs.items()) {
        if (value.is_object()) throw std::runtime_error("Compressed production replay forbids from_call");
        call.resolved[key] = encoded(scalar(value));
    }
    auto get = [&](char const *key) { return scalar(inputs.at(key)); };
    if (function == "PsyCpAirFnW" || function == "PsyCpAirFnW_fast") {
        call.routine = function.ends_with("_fast") ? Routine::CpFast : Routine::Cp;
        call.w = get("w_kg_per_kg"); call.unit = "J/(kg*K)"; call.cp = true;
    } else if (function == "PsyRhoAirFnPbTdbW" || function == "PsyRhoAirFnPbTdbW_fast") {
        call.routine = function.ends_with("_fast") ? Routine::RhoFast : Routine::Rho;
        call.p = get("p_pa"); call.t = get("t_db_c"); call.w = get("w_kg_per_kg"); call.unit = "kg/m3";
    } else if (function == "PsyHFnTdbW" || function == "PsyHFnTdbW_fast") {
        call.routine = function.ends_with("_fast") ? Routine::HFast : Routine::H;
        call.t = get("t_db_c"); call.w = get("w_kg_per_kg"); call.unit = "J/kg";
    } else if (function == "PsyTdbFnHW") {
        call.routine = Routine::Tdb; call.h = get("h_j_per_kg"); call.w = get("w_kg_per_kg"); call.unit = "degC";
    } else if (function == "PsyWFnTdbH") {
        call.routine = Routine::W; call.h = get("h_j_per_kg"); call.t = get("t_db_c"); call.unit = "kg/kg";
    } else throw std::runtime_error("Function outside bounded reference");
    if (function.ends_with("_fast") && !(call.w >= 1e-5)) throw std::runtime_error("Invalid fast W");
    return call;
}

double evaluate_compressed(PreparedCall const &call, EnergyPlus::EnergyPlusData &state)
{
    namespace Psy = EnergyPlus::Psychrometrics;
    // Invoke the original body at EVERY event. Interning affects storage only.
    switch (call.routine) {
    case Routine::Cp: return Psy::PsyCpAirFnW(call.w);
    case Routine::CpFast: return Psy::PsyCpAirFnW_fast(call.w);
    case Routine::Rho: return Psy::PsyRhoAirFnPbTdbW(state, call.p, call.t, call.w);
    case Routine::RhoFast: return Psy::PsyRhoAirFnPbTdbW_fast(state, call.p, call.t, call.w);
    case Routine::H: return Psy::PsyHFnTdbW(call.t, call.w);
    case Routine::HFast: return Psy::PsyHFnTdbW_fast(call.t, call.w);
    case Routine::Tdb: return Psy::PsyTdbFnHW(call.h, call.w);
    case Routine::W: return Psy::PsyWFnTdbH(state, call.t, call.h);
    }
    throw std::runtime_error("Unreachable routine");
}

void add_cache(json &row)
{
    row["cache_before"] = {{"dw_save", encoded(cache_observation.before_w)}, {"cpa_save", encoded(cache_observation.before_cp)},
                           {"dw_save_bits", bits(cache_observation.before_w)}, {"cpa_save_bits", bits(cache_observation.before_cp)}};
    row["cache_after"] = {{"dw_save", encoded(cache_observation.after_w)}, {"cpa_save", encoded(cache_observation.after_cp)},
                          {"dw_save_bits", bits(cache_observation.after_w)}, {"cpa_save_bits", bits(cache_observation.after_cp)}};
    row["cache_hit"] = cache_observation.hit;
}

struct ResultKey
{
    std::array<std::uint64_t, 7> bits{};
    bool operator==(ResultKey const &) const = default;
};

struct ResultHash
{
    std::size_t operator()(ResultKey const &key) const
    {
        std::size_t hash = 0;
        for (auto value : key.bits) hash ^= std::hash<std::uint64_t>{}(value) + 0x9e3779b9 + (hash << 6) + (hash >> 2);
        return hash;
    }
};

void replay_compressed(json const &tuples, NativeState &native, char const *output_path)
{
    std::vector<PreparedCall> inputs;
    for (auto const &row : tuples.at("dictionary")) inputs.push_back(prepare_compressed(row));
    std::unordered_map<ResultKey, std::uint32_t, ResultHash> variants;
    json dictionary = json::array();
    std::vector<std::uint32_t> ordered;
    ordered.reserve(tuples.at("ordered_ids").size());
    for (auto const &id : tuples.at("ordered_ids")) {
        if (!id.is_number_unsigned() || id.get<std::uint64_t>() > std::numeric_limits<std::uint32_t>::max()) throw std::runtime_error("ordered_ids must be u32 indices");
        std::uint32_t const input_id = id.get<std::uint32_t>();
        if (input_id >= inputs.size()) throw std::runtime_error("Input dictionary index out of range");
        auto const &call = inputs[input_id];
        double const value = evaluate_compressed(call, *native.state);
        ResultKey key;
        key.bits[0] = input_id;
        key.bits[1] = std::bit_cast<std::uint64_t>(value);
        if (call.cp) {
            key.bits[2] = std::bit_cast<std::uint64_t>(cache_observation.before_w);
            key.bits[3] = std::bit_cast<std::uint64_t>(cache_observation.before_cp);
            key.bits[4] = std::bit_cast<std::uint64_t>(cache_observation.after_w);
            key.bits[5] = std::bit_cast<std::uint64_t>(cache_observation.after_cp);
            key.bits[6] = cache_observation.hit;
        }
        auto found = variants.find(key);
        if (found == variants.end()) {
            if (dictionary.size() >= std::numeric_limits<std::uint32_t>::max()) throw std::runtime_error("Reference dictionary exceeds u32");
            std::uint32_t const reference_id = static_cast<std::uint32_t>(dictionary.size());
            json row = call.identity;
            row["input_id"] = input_id;
            row["resolved_inputs"] = call.resolved;
            row["value"] = encoded(value);
            row["value_class"] = classification(value);
            row["value_bits"] = bits(value);
            row["unit"] = call.unit;
            if (call.cp) add_cache(row);
            dictionary.push_back(row);
            found = variants.emplace(key, reference_id).first;
        }
        ordered.push_back(found->second);
    }
    std::ofstream output(output_path);
    if (!output) throw std::runtime_error("Cannot write compressed results");
    output << "{\"schema\":\"psy01-results.v2\",\"dictionary\":" << dictionary.dump() << ",\"ordered_ids\":[";
    for (std::size_t i = 0; i < ordered.size(); ++i) { if (i) output << ','; output << ordered[i]; }
    output << "],\"executed_event_count\":" << ordered.size() << "}\n";
}

int main(int argc, char **argv)
{
    try {
        if (argc != 4) throw std::runtime_error("Usage: psy01_reference INPUT.json OUTPUT.json energyplusapi.dll");
        std::ifstream input(argv[1]);
        if (!input) throw std::runtime_error("Cannot read tuple input");
        json tuples;
        input >> tuples;
        NativeState native(argv[3]);
        if (tuples.at("schema") == "psy01-tuples.v2") {
            replay_compressed(tuples, native, argv[2]);
            return 0;
        }
        if (tuples.at("schema") != "psy01-tuples.v1") throw std::runtime_error("Tuple schema mismatch");
        std::map<int, std::pair<std::string, double>> previous;
        json result = {{"schema", "psy01-results.v1"}, {"calls", json::array()}};
        for (auto const &call : tuples.at("calls")) {
            int const index = call.at("call_index");
            if (previous.contains(index)) throw std::runtime_error("Duplicate call_index");
            std::string const function = call.at("function");
            json resolved = json::object();
            std::map<std::string, double> values;
            for (auto const &[key, spec] : call.at("inputs").items()) {
                double value;
                if (spec.is_object()) {
                    if (key != "h_j_per_kg" || spec.size() != 1 || !spec.contains("from_call")) {
                        throw std::runtime_error("Only h_j_per_kg supports from_call");
                    }
                    int const dependency = spec.at("from_call");
                    auto found = previous.find(dependency);
                    if (found == previous.end() || (found->second.first != "PsyHFnTdbW" && found->second.first != "PsyHFnTdbW_fast")) {
                        throw std::runtime_error("from_call must refer to a preceding enthalpy call");
                    }
                    value = found->second.second;
                } else {
                    value = scalar(spec);
                }
                values.emplace(key, value);
                resolved[key] = encoded(value);
            }
            auto get = [&](char const *key) { return values.at(key); };
            if (function.ends_with("_fast") && !(get("w_kg_per_kg") >= 1.0e-5)) {
                throw std::runtime_error("Fast path requires W >= 1e-5");
            }
            double value;
            std::string unit;
            namespace Psy = EnergyPlus::Psychrometrics;
            if (function == "PsyCpAirFnW") { value = Psy::PsyCpAirFnW(get("w_kg_per_kg")); unit = "J/(kg*K)"; }
            else if (function == "PsyCpAirFnW_fast") { value = Psy::PsyCpAirFnW_fast(get("w_kg_per_kg")); unit = "J/(kg*K)"; }
            else if (function == "PsyRhoAirFnPbTdbW") { value = Psy::PsyRhoAirFnPbTdbW(*native.state, get("p_pa"), get("t_db_c"), get("w_kg_per_kg")); unit = "kg/m3"; }
            else if (function == "PsyRhoAirFnPbTdbW_fast") { value = Psy::PsyRhoAirFnPbTdbW_fast(*native.state, get("p_pa"), get("t_db_c"), get("w_kg_per_kg")); unit = "kg/m3"; }
            else if (function == "PsyHFnTdbW") { value = Psy::PsyHFnTdbW(get("t_db_c"), get("w_kg_per_kg")); unit = "J/kg"; }
            else if (function == "PsyHFnTdbW_fast") { value = Psy::PsyHFnTdbW_fast(get("t_db_c"), get("w_kg_per_kg")); unit = "J/kg"; }
            else if (function == "PsyTdbFnHW") { value = Psy::PsyTdbFnHW(get("h_j_per_kg"), get("w_kg_per_kg")); unit = "degC"; }
            else if (function == "PsyWFnTdbH") { value = Psy::PsyWFnTdbH(*native.state, get("t_db_c"), get("h_j_per_kg")); unit = "kg/kg"; }
            else throw std::runtime_error("Function outside PSY-01");
            json row = call;
            row["resolved_inputs"] = resolved;
            row["value"] = encoded(value);
            row["value_class"] = classification(value);
            row["value_bits"] = bits(value);
            row["unit"] = unit;
            if (function == "PsyCpAirFnW" || function == "PsyCpAirFnW_fast") {
                if (!cache_observation.observed) throw std::runtime_error("Missing original Cp observer");
                row["cache_before"] = {{"dw_save", encoded(cache_observation.before_w)}, {"cpa_save", encoded(cache_observation.before_cp)},
                                       {"dw_save_bits", bits(cache_observation.before_w)}, {"cpa_save_bits", bits(cache_observation.before_cp)}};
                row["cache_after"] = {{"dw_save", encoded(cache_observation.after_w)}, {"cpa_save", encoded(cache_observation.after_cp)},
                                      {"dw_save_bits", bits(cache_observation.after_w)}, {"cpa_save_bits", bits(cache_observation.after_cp)}};
                row["cache_hit"] = cache_observation.hit;
            }
            result["calls"].push_back(row);
            previous.emplace(index, std::make_pair(function, value));
        }
        std::ofstream output(argv[2]);
        if (!output) throw std::runtime_error("Cannot write results");
        output << result.dump(2) << '\n';
        return 0;
    } catch (std::exception const &error) {
        std::cerr << error.what() << '\n';
        return 2;
    }
}
