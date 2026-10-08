// Whole genuine cached-core weather calls. No copied scientific bodies or answers.
#ifndef WIN32_LEAN_AND_MEAN
#define WIN32_LEAN_AND_MEAN
#endif
#ifndef NOMINMAX
#define NOMINMAX
#endif
#include <windows.h>
#include <bcrypt.h>
#include "clk03_reference_fields.hh"
#include <EnergyPlus/DataReportingFlags.hh>
#include <EnergyPlus/DataStringGlobals.hh>
#include <EnergyPlus/InputProcessing/InputProcessor.hh>
#include <array>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <memory>
#include <vector>

namespace {
using Clk03::json;
using Clk03::require;
namespace fs = std::filesystem;
namespace Weather = EnergyPlus::Weather;

// Read-only Windows SHA plumbing; the weather engine never receives hashes/results.
class Sha256 {
    BCRYPT_ALG_HANDLE algorithm_ = nullptr;
    BCRYPT_HASH_HANDLE hash_ = nullptr;
    std::vector<unsigned char> object_;
public:
    Sha256()
    {
        require(BCryptOpenAlgorithmProvider(&algorithm_, BCRYPT_SHA256_ALGORITHM, nullptr, 0) >= 0, "Cannot open SHA256 provider");
        DWORD length = 0, copied = 0;
        require(BCryptGetProperty(algorithm_, BCRYPT_OBJECT_LENGTH, reinterpret_cast<PUCHAR>(&length), sizeof(length), &copied, 0) >= 0,
                "Cannot query SHA256 object size");
        object_.resize(length);
        require(BCryptCreateHash(algorithm_, &hash_, object_.data(), length, nullptr, 0, 0) >= 0, "Cannot create SHA256 hash");
    }
    Sha256(Sha256 const &) = delete;
    Sha256 &operator=(Sha256 const &) = delete;
    ~Sha256() { if (hash_ != nullptr) BCryptDestroyHash(hash_); if (algorithm_ != nullptr) BCryptCloseAlgorithmProvider(algorithm_, 0); }
    void append(void const *data, std::size_t size)
    {
        require(size <= MAXDWORD, "SHA256 chunk too large");
        require(BCryptHashData(hash_, const_cast<PUCHAR>(static_cast<unsigned char const *>(data)), static_cast<ULONG>(size), 0) >= 0,
                "Cannot hash input");
    }
    std::string finish()
    {
        std::array<unsigned char, 32> digest{};
        require(BCryptFinishHash(hash_, digest.data(), static_cast<ULONG>(digest.size()), 0) >= 0, "Cannot finish SHA256 hash");
        std::ostringstream out; out << std::hex << std::setfill('0');
        for (auto value : digest) out << std::setw(2) << static_cast<unsigned int>(value);
        return out.str();
    }
};
std::string file_sha(fs::path const &path)
{
    std::ifstream input(path, std::ios::binary); require(input.is_open(), "Cannot open artifact: " + path.string());
    Sha256 hash; std::array<char, 65536> block{};
    while (input.read(block.data(), static_cast<std::streamsize>(block.size())) || input.gcount() > 0)
        hash.append(block.data(), static_cast<std::size_t>(input.gcount()));
    require(input.eof(), "Cannot read complete artifact: " + path.string()); return hash.finish();
}
json file_ref(fs::path const &path)
{
    return {{"path", fs::absolute(path).lexically_normal().generic_string()}, {"sha256", file_sha(path)}, {"bytes", fs::file_size(path)}};
}
json read_json(fs::path const &path)
{
    std::ifstream input(path, std::ios::binary); require(input.is_open(), "Cannot open JSON input: " + path.string()); return json::parse(input);
}
void write_text(fs::path const &path, std::string const &value)
{
    require(!fs::exists(path), "Refusing to overwrite native output: " + path.string());
    std::ofstream output(path, std::ios::binary); require(output.is_open(), "Cannot create native output: " + path.string());
    output.write(value.data(), static_cast<std::streamsize>(value.size())); output.close();
    require(!output.fail(), "Cannot finish native output: " + path.string());
}
fs::path bound_path(fs::path const &root, json const &binding)
{
    require(binding.at("path").is_string() && binding.at("sha256").is_string(), "Input binding requires path/SHA256");
    fs::path const relative(binding.at("path").get<std::string>());
    require(relative.is_relative(), "Input binding must be repository-relative");
    auto const path = fs::weakly_canonical(root / relative);
    auto const inside = path.lexically_relative(root);
    require(!inside.empty() && *inside.begin() != ".." && fs::is_regular_file(path), "Input outside repository or not regular file");
    require(file_sha(path) == binding.at("sha256").get<std::string>(), "Frozen input hash differs"); return path;
}
template<class Function> json invoke(Function &&function)
{
    try { function(); return {{"status", "source_returned"}, {"source_fatal", false}, {"exception_message", nullptr}}; }
    catch (EnergyPlus::FatalError const &error) {
        return {{"status", "source_fatal"}, {"source_fatal", true}, {"exception_message", error.what()}};
    }
    // Every other exception is a wrapper/preparation failure, never a manufactured SourceFatal.
}
struct Factory {
    std::unique_ptr<EnergyPlus::EnergyPlusData> state = std::make_unique<EnergyPlus::EnergyPlusData>();
    std::ostringstream *error_text = nullptr;
    json messages = json::array(), constructor, after_constant_initialization, preparation = json::array();
    bool stopped = false, available = false, errors = false, print_environment_stamp = false;
    std::string stop_reason;
    Factory()
    {
        constructor = Clk03::snapshot(*state);
        state->init_constant_state(*state);
        after_constant_initialization = Clk03::snapshot(*state);
        auto errors_sink = std::make_unique<std::ostringstream>(); error_text = errors_sink.get();
        state->files.err_stream = std::move(errors_sink);
        state->files.eso.open_as_stringstream(); state->files.eio.open_as_stringstream();
        state->dataGlobal->errorCallback = [this](EnergyPlus::Error level, std::string const &message) {
            messages.push_back({{"level", static_cast<int>(level)}, {"message", message}});
        };
    }
    template<class Function> void prepare(std::string const &name, Function &&function)
    {
        if (stopped) return;
        auto before = Clk03::snapshot(*state); auto outcome = invoke(std::forward<Function>(function));
        preparation.push_back({{"kind", name}, {"before", before}, {"after", Clk03::snapshot(*state)}, {"call_outcome", outcome}});
        if (outcome.at("source_fatal").template get<bool>()) { stopped = true; stop_reason = "actual-source-fatal-during-preparation"; }
        else if (errors) { stopped = true; stop_reason = "actual-errors-during-genuine-preparation"; }
    }
    json diagnostics(fs::path const &output, std::string const &id)
    {
        auto const error = output / (id + ".err"), eso = output / (id + ".eso.txt"), eio = output / (id + ".eio.txt");
        write_text(error, error_text->str()); write_text(eso, state->files.eso.get_output()); write_text(eio, state->files.eio.get_output());
        return {{"messages", messages}, {"source_error_log", file_ref(error)}, {"source_optional_eso_log", file_ref(eso)},
                {"source_eio_log", file_ref(eio)}, {"actual_native_output_stringstream_sinks", true}, {"error_text_paired", false}};
    }
};
void check_identifier(json const &input)
{
    auto const id = input.at("id").get<std::string>();
    require(!id.empty() && std::all_of(id.begin(), id.end(), [](char c) {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9') || c == '-' || c == '_';
    }), "Safe distinct diagnostic identifier required");
}
json operation(Factory &factory, json const &input, bool weather_sequence)
{
    check_identifier(input); auto const kind = input.at("kind").get<std::string>(); auto &state = *factory.state;
    auto const before_caller = Clk03::snapshot(state);
    json result = {{"id", input.at("id")}, {"kind", kind}, {"requested_operation", input}, {"before_caller", before_caller}};
    if (factory.stopped) {
        result["before"] = before_caller; result["after"] = before_caller;
        result["call_outcome"] = {{"status", "not-invoked-after-prior-source-outcome"}, {"source_fatal", nullptr},
                                  {"skip_reason", factory.stop_reason}, {"exception_message", nullptr}};
        result["actual_source_invoked"] = false; return result;
    }
    if (weather_sequence) Clk03::seed(state, {{"global", input.at("caller")}});
    else Clk03::seed(state, input.at("before"));
    require(state.dataGlobal->TimeStepsInHour == 4, "Caller changed frozen four-step allocation");
    result["before"] = Clk03::snapshot(state);
    result["print_environment_stamp_before"] = factory.print_environment_stamp;
    bool returned = false;
    auto outcome = invoke([&] {
        if (kind == "UpdateWeatherData") Weather::UpdateWeatherData(state);
        else if (weather_sequence && kind == "GetNextEnvironment") {
            returned = Weather::GetNextEnvironment(state, factory.available, factory.errors);
        } else if (weather_sequence && kind == "InitializeWeather") {
            require(factory.available && !factory.errors && state.dataWeather->Envrn > 0, "No selected genuine environment");
            Weather::InitializeWeather(state, factory.print_environment_stamp);
        } else if (weather_sequence && kind == "ReadWeatherForDay") {
            require(factory.available && !factory.errors && state.dataWeather->Envrn > 0, "No selected genuine environment");
            auto const day = Clk03::integer(input.at("day_to_read"));
            require(day > 0, "Positive declared day_to_read required");
            Weather::ReadWeatherForDay(state, day, state.dataWeather->Envrn, Clk03::boolean(input.at("backspace_after_read")));
        } else throw std::runtime_error("Unknown declared whole-source weather call");
    });
    result["call_outcome"] = outcome; result["after"] = Clk03::snapshot(state); result["actual_source_invoked"] = true;
    result["print_environment_stamp_after"] = factory.print_environment_stamp;
    result["returned_bool"] = kind == "GetNextEnvironment" && !outcome.at("source_fatal").get<bool>() ? json(returned) : json(nullptr);
    result["Available"] = factory.available; result["ErrorsFound"] = factory.errors;
    if (outcome.at("source_fatal").get<bool>()) { factory.stopped = true; factory.stop_reason = "actual-source-fatal"; }
    else if (kind == "GetNextEnvironment" && !returned) {
        factory.stopped = true; factory.stop_reason = "actual-selected-environment-unavailable-or-errors";
    }
    return result;
}
json handoff_sequence(json const &input, fs::path const &output)
{
    check_identifier(input); require(Clk03::integer(input.at("time_steps_per_hour")) == 4, "Frozen four steps required");
    Factory factory; auto &state = *factory.state; state.dataGlobal->TimeStepsInHour = 4;
    factory.prepare("AllocateWeatherData", [&] { Weather::AllocateWeatherData(state); });
    auto const allocated_defaults = Clk03::snapshot(state);
    if (!factory.stopped) Clk03::seed(state, input.at("initial"));
    require(state.dataGlobal->TimeStepsInHour == 4, "Initial canary changed declared four-step allocation");
    auto const prepared = Clk03::snapshot(state); json operations = json::array();
    for (auto const &item : input.at("operations")) operations.push_back(operation(factory, item, false));
    return {{"id", input.at("id")}, {"lane", "whole-original-UpdateWeatherData-input-only-canaries"},
            {"constructor", factory.constructor}, {"after_constant_initialization", factory.after_constant_initialization},
            {"preparation_calls", factory.preparation}, {"allocated_defaults", allocated_defaults}, {"prepared", prepared},
            {"operations", operations}, {"final_state", Clk03::snapshot(state)},
            {"source_only_diagnostics", factory.diagnostics(output, input.at("id").get<std::string>())},
            {"carrier_generation_science_executed", false}, {"whole_source_constructor_observed", true}};
}
void check_preparation(json const &value)
{
    require(value.at("lane") == "selected-prepared-environment" && value.at("GetEnvironmentFirstCall") == false &&
            value.at("GetBranchInputOneTimeFlag") == false && value.at("WaterMainsParameterReport") == false &&
            value.at("BeginSimFlag") == false && value.at("DoWeathSim") == true && value.at("DoDesDaySim") == false &&
            value.at("Envrn_before_GetNextEnvironment") == "actual-native-TotDesDays" &&
            value.at("registered_whole_simulation_initialization_claimed") == false &&
            value.at("eio_sink") == "native-output-stringstream" && value.at("error_sink") == "native-output-stringstream",
            "Declared selected prepared lane differs");
    json const calls = {"InputProcessor::processInput", "OpenWeatherFile", "CloseWeatherFile", "ReadUserWeatherInput",
                        "AllocateWeatherData", "SetupInterpolationValues", "ResolveLocationInformation", "CheckLocationValidity"};
    require(value.at("genuine_calls") == calls && value.at("TimeStepFraction").at("bits") == "3fd0000000000000" &&
            value.at("TimeStepZone").at("bits") == "3fd0000000000000", "Declared input-only setup/call order differs");
}
json weather_sequence(json const &input, json const &declared, fs::path const &root, fs::path const &output)
{
    check_identifier(input); require(Clk03::integer(input.at("time_steps_per_hour")) == 4 && input.at("prepared_environment_lane") == true,
                                     "Frozen prepared weather lane required");
    auto const idf = bound_path(root, input.at("input")), epw = bound_path(root, input.at("weather"));
    Factory factory; auto &state = *factory.state; auto &owner = *state.dataWeather;
    state.dataStrGlobals->inputFilePath = idf; state.files.inputWeatherFilePath.filePath = epw;
    state.dataGlobal->TimeStepsInHour = 4; state.dataGlobal->TimeStepZone = Clk03::input_real(declared.at("TimeStepZone"));
    state.dataGlobal->BeginSimFlag = Clk03::boolean(declared.at("BeginSimFlag"));
    state.dataGlobal->DoWeathSim = Clk03::boolean(declared.at("DoWeathSim"));
    state.dataGlobal->DoDesDaySim = Clk03::boolean(declared.at("DoDesDaySim"));
    factory.prepare("InputProcessor::processInput", [&] { state.dataInputProcessing->inputProcessor->processInput(state); });
    factory.prepare("OpenWeatherFile", [&] { Weather::OpenWeatherFile(state, factory.errors); });
    factory.prepare("CloseWeatherFile", [&] { Weather::CloseWeatherFile(state); });
    factory.prepare("ReadUserWeatherInput", [&] { Weather::ReadUserWeatherInput(state); });
    factory.prepare("AllocateWeatherData", [&] { Weather::AllocateWeatherData(state); });
    auto const allocated_defaults = Clk03::snapshot(state);
    factory.prepare("SetupInterpolationValues", [&] { Weather::SetupInterpolationValues(state); });
    factory.prepare("ResolveLocationInformation", [&] { Weather::ResolveLocationInformation(state, factory.errors); });
    factory.prepare("CheckLocationValidity", [&] { Weather::CheckLocationValidity(state); });
    if (!factory.stopped) {
        require(owner.TotRunPers == 1 && owner.TotRunDesPers == 0 && owner.NumIntervalsPerHour == 1,
                "Original parsed input outside sole hourly nonactual RunPeriod domain");
        auto const &environment = owner.Environment(state.dataEnvrn->TotDesDays + 1);
        require(environment.KindOfEnvrn == EnergyPlus::Constant::KindOfSim::RunPeriodWeather && !environment.ActualWeather &&
                !environment.MatchYear && environment.StartYear == 2013 && environment.EndYear == 2013,
                "Original resolved environment outside frozen ordinary2013 weather scope");
        owner.GetEnvironmentFirstCall = Clk03::boolean(declared.at("GetEnvironmentFirstCall"));
        owner.GetBranchInputOneTimeFlag = Clk03::boolean(declared.at("GetBranchInputOneTimeFlag"));
        owner.WaterMainsParameterReport = Clk03::boolean(declared.at("WaterMainsParameterReport"));
        owner.TimeStepFraction = Clk03::input_real(declared.at("TimeStepFraction"));
        owner.Envrn = state.dataEnvrn->TotDesDays;
        // No Today/Tomorrow or calendar outcomes are fabricated here. These are
        // explicit caller controls for a selected, separately prepared source lane.
        if (factory.errors) { factory.stopped = true; factory.stop_reason = "actual-errors-during-genuine-preparation"; }
    }
    auto const prepared = Clk03::snapshot(state); json operations = json::array();
    for (auto const &item : input.at("operations")) operations.push_back(operation(factory, item, true));
    require(file_sha(idf) == input.at("input").at("sha256").get<std::string>() && file_sha(epw) == input.at("weather").at("sha256").get<std::string>(),
            "Original input bytes changed during genuine calls");
    return {{"id", input.at("id")}, {"lane", "selected-prepared-whole-original-environment-reader-lifecycle"},
            {"input", file_ref(idf)}, {"weather_input", file_ref(epw)}, {"declared_run_period_input", input.at("run_period")},
            {"constructor", factory.constructor}, {"after_constant_initialization", factory.after_constant_initialization},
            {"preparation_calls", factory.preparation}, {"allocated_defaults", allocated_defaults}, {"prepared", prepared},
            {"declared_native_preparation", declared}, {"operations", operations}, {"final_state", Clk03::snapshot(state)},
            {"source_only_diagnostics", factory.diagnostics(output, input.at("id").get<std::string>())},
            {"registered_whole_simulation_initialization_claimed", false}, {"native_processed_weather_storage_retained", true},
            {"processed_weather_numerical_fields_paired", false}, {"source_internal_record_index_observed", false},
            {"input_identity_mapping_policy", "actual-stream-byte-position; external-hashed-input-line-map-not-native-record-index"}};
}
} // namespace

int main(int argc, char **argv)
{
    try {
        require(argc == 4, "Usage: clk03_reference_helper <repo_root> <input-only-request.json> <fresh_output_dir>");
        auto const root = fs::weakly_canonical(fs::absolute(argv[1]));
        auto const request_path = fs::absolute(argv[2]).lexically_normal(), output = fs::absolute(argv[3]).lexically_normal();
        require(!fs::exists(output), "Native output directory must be fresh"); auto const request = read_json(request_path);
        require(request.at("schema") == "clk03-helper-cases.v1" && request.at("expected_values_supplied") == false &&
                request.at("expected_exits_supplied") == false && request.at("scientific_execution_performed") == false,
                "Frozen input-only helper schema/policy required");
        check_preparation(request.at("native_preparation")); bound_path(root, request.at("fixed_scope"));
        json contracts = json::object(), frozen_counts;
        for (char const *name : {"source", "cases", "tolerances"}) {
            auto const path = root / "energyplus_porting_plan/contracts" / (std::string("CLK-03-") + name + ".json");
            auto const contract = read_json(path);
            require(contract.at("status") == "frozen-before-numerical-execution" && contract.at("frozen_before_numerical_execution") == true,
                    "Unfrozen original contract");
            if (std::string(name) == "cases") {
                require(file_sha(request_path) == contract.at("helper_request").at("sha256").get<std::string>(), "Request differs from frozen cases");
                frozen_counts = contract.at("counts");
            }
            if (std::string(name) == "source") {
                require(contract.at("energyplus_commit") == "6f2e40d10250a105b49966baa24d843711e61048", "Wrong original source pin");
                for (auto const &source : contract.at("source_files")) require(file_sha(root / ".reference/energyplus-src/26.1.0" /
                    source.at("path").get<std::string>()) == source.at("sha256").get<std::string>(), "Pinned original source changed");
            }
            contracts[name] = file_ref(path);
        }
        require(request.at("handoff_sequences").is_array() && request.at("weather_sequences").is_array(), "Declared root arrays required");
        fs::create_directories(output); json handoffs = json::array(), weather = json::array();
        for (auto const &item : request.at("handoff_sequences")) handoffs.push_back(handoff_sequence(item, output));
        for (auto const &item : request.at("weather_sequences")) weather.push_back(weather_sequence(item, request.at("native_preparation"), root, output));
        std::size_t handoff_requested = 0, weather_requested = 0, invoked = 0, skipped = 0;
        json actual_calls = json::object();
        for (auto const &sequence : handoffs) handoff_requested += sequence.at("operations").size();
        for (auto const &sequence : weather) weather_requested += sequence.at("operations").size();
        for (auto const *group : {&handoffs, &weather}) for (auto const &sequence : *group) for (auto const &item : sequence.at("operations")) {
            if (item.at("actual_source_invoked").get<bool>()) {
                ++invoked; auto const key = item.at("kind").get<std::string>(); actual_calls[key] = actual_calls.value(key, 0) + 1;
            } else ++skipped;
        }
        json const counts = {{"handoff_sequences", handoffs.size()}, {"weather_sequences", weather.size()},
                             {"handoff_operations", handoff_requested}, {"weather_operations", weather_requested},
                             {"total_sequences", handoffs.size() + weather.size()}, {"total_operations", handoff_requested + weather_requested}};
        require(counts == frozen_counts, "Frozen requested root cardinalities differ");
        json const result = {{"schema", "clk03-helper-results.v1"}, {"source_commit", "6f2e40d10250a105b49966baa24d843711e61048"},
                             {"contracts", contracts}, {"actual_request", file_ref(request_path)}, {"actual_binary", file_ref(fs::absolute(argv[0]))},
                             {"handoff_sequences", handoffs}, {"weather_sequences", weather}, {"requested_counts", counts},
                             {"actual_operation_counts", actual_calls}, {"actual_operation_invocations", invoked}, {"actual_operations_skipped", skipped},
                             {"complete", true}, {"expected_answers_supplied", false}, {"gates_updated", false},
                             {"original_error_text_peer_parity_claimed", false}, {"native_raw_record_index_claimed", false},
                             {"processed_weather_physics_retained_unpaired", true}, {"pure_body_fallback_used", false}};
        write_text(output / "results.json", result.dump() + "\n");
        std::cout << json({{"status", "wrapper-complete-source-outcomes-preserved"}, {"result", file_ref(output / "results.json")},
                          {"requested_counts", counts}, {"actual_operation_counts", actual_calls},
                          {"actual_operation_invocations", invoked}, {"actual_operations_skipped", skipped}}).dump() << '\n';
        return 0;
    } catch (std::exception const &error) { std::cerr << error.what() << '\n'; return 2; }
}
