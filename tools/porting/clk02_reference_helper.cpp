// Test-only whole original EPW parser/header calls. No copied scientific bodies.
// Caller canaries and real files are explicit input preparation, not parsed state.
#ifndef WIN32_LEAN_AND_MEAN
#define WIN32_LEAN_AND_MEAN
#endif
#ifndef NOMINMAX
#define NOMINMAX
#endif
#include <windows.h>
#include <bcrypt.h>
#include "clk02_reference_fields.hh"
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/InputProcessing/InputProcessor.hh>
#include <array>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <memory>
#include <vector>

namespace {
using Clk02::json;
namespace fs = std::filesystem;

void require(bool condition, std::string const &message)
{
    if (!condition) throw std::runtime_error(message);
}

// Read-only Windows SHA plumbing reused from the previous native observer.
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
    ~Sha256()
    {
        if (hash_ != nullptr) BCryptDestroyHash(hash_);
        if (algorithm_ != nullptr) BCryptCloseAlgorithmProvider(algorithm_, 0);
    }
    void append(void const *data, std::size_t size)
    {
        require(size <= MAXDWORD, "SHA256 chunk too large");
        auto *input = const_cast<PUCHAR>(static_cast<unsigned char const *>(data));
        require(BCryptHashData(hash_, input, static_cast<ULONG>(size), 0) >= 0, "Cannot hash input");
    }
    std::string finish()
    {
        std::array<unsigned char, 32> digest{};
        require(BCryptFinishHash(hash_, digest.data(), static_cast<ULONG>(digest.size()), 0) >= 0, "Cannot finish SHA256 hash");
        std::ostringstream out;
        out << std::hex << std::setfill('0');
        for (auto value : digest) out << std::setw(2) << static_cast<unsigned int>(value);
        return out.str();
    }
};

std::string text_sha(std::string const &value)
{
    Sha256 hash;
    hash.append(value.data(), value.size());
    return hash.finish();
}

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
    return {{"path", fs::absolute(path).lexically_normal().generic_string()}, {"sha256", file_sha(path)}, {"bytes", fs::file_size(path)}};
}

json read_json(fs::path const &path)
{
    std::ifstream input(path, std::ios::binary);
    require(input.is_open(), "Cannot open JSON input: " + path.string());
    return json::parse(input);
}

void write_text(fs::path const &path, std::string const &value)
{
    require(!fs::exists(path), "Refusing to overwrite native output: " + path.string());
    std::ofstream output(path, std::ios::binary);
    require(output.is_open(), "Cannot create native output: " + path.string());
    output.write(value.data(), static_cast<std::streamsize>(value.size()));
    output.close();
    require(!output.fail(), "Cannot finish native output: " + path.string());
}

fs::path bound_path(fs::path const &root, json const &binding)
{
    require(binding.at("path").is_string() && binding.at("sha256").is_string(), "Input file binding needs path and SHA256");
    fs::path const path = root / binding.at("path").get<std::string>();
    require(file_sha(path) == binding.at("sha256").get<std::string>(), "Frozen input artifact hash differs");
    return path;
}

int integer(json const &value)
{
    require(value.is_number_integer() && !value.is_boolean(), "Typed integer input required");
    return value.get<int>();
}

double input_real(json const &value, json const &token)
{
    require(value.is_number() && !value.is_boolean() && token.is_string(), "Binary64 canary and authoritative bits required");
    double const number = value.get<double>();
    require(std::isfinite(number) && Clk02::bits(number) == token.get<std::string>(), "Caller canary input bits differ");
    return number;
}

void seed_outputs(Clk02::RawOutputs &outputs, json const &input)
{
    require(input.at("ErrorFound").is_boolean(), "Typed ErrorFound input required");
    outputs.ErrorFound = input.at("ErrorFound").get<bool>();
    require(input.at("dates").size() == 5 && input.at("mandatory_reals").size() == 20 &&
            input.at("mandatory_real_bits").size() == 20 && input.at("optional_reals").size() == 6 &&
            input.at("optional_real_bits").size() == 6 && input.at("weather_codes").size() == 9,
            "Frozen public output canary cardinality differs");
    std::size_t i = 0;
#define SEED_DATE(name) outputs.name = integer(input.at("dates").at(i++));
    CLK02_DATES(SEED_DATE)
#undef SEED_DATE
    i = 0;
#define SEED_MANDATORY(name) outputs.name = input_real(input.at("mandatory_reals").at(i), input.at("mandatory_real_bits").at(i)); ++i;
    CLK02_MANDATORY_REALS(SEED_MANDATORY)
#undef SEED_MANDATORY
    i = 0;
#define SEED_OPTIONAL(name) outputs.name = input_real(input.at("optional_reals").at(i), input.at("optional_real_bits").at(i)); ++i;
    CLK02_OPTIONAL_REALS(SEED_OPTIONAL)
#undef SEED_OPTIONAL
    outputs.WObs = integer(input.at("WObs"));
    for (i = 0; i < 9; ++i) outputs.WCodesArr(static_cast<int>(i) + 1) = integer(input.at("weather_codes").at(i));
}

struct Factory {
    json messages = json::array();
    std::unique_ptr<EnergyPlus::EnergyPlusData> state = std::make_unique<EnergyPlus::EnergyPlusData>();
    std::ostringstream *error_text = nullptr;
    json constructor;
    Factory()
    {
        constructor = Clk02::header_state(*state);
        state->init_constant_state(*state);
        auto errors = std::make_unique<std::ostringstream>();
        error_text = errors.get();
        state->files.err_stream = std::move(errors);
        // OpenEPlusWeatherFile passes ESO to ShowFatalError. Supply a real,
        // defined IO sink; an unopened optional output would assert in print().
        // This is declared error-IO preparation, not simulation output setup.
        state->files.eso.open_as_stringstream();
        state->dataGlobal->errorCallback = [this](EnergyPlus::Error level, std::string const &message) {
            messages.push_back({{"level", static_cast<int>(level)}, {"message", message}});
        };
    }
    Factory(Factory const &) = delete;
    Factory &operator=(Factory const &) = delete;
    json diagnostics(fs::path const &output, std::string const &id)
    {
        auto const path = output / (id + ".err");
        write_text(path, error_text->str());
        auto const optional = output / (id + ".optional-eso.txt");
        write_text(optional, state->files.eso.get_output());
        return {{"messages", messages}, {"source_error_log", file_ref(path)}, {"source_optional_eso_log", file_ref(optional)},
                {"declared_optional_eso_stringstream", true}, {"error_text_paired", false}};
    }
};

template<class Function> json invoke(Function &&function)
{
    // Only a genuine EnergyPlus fatal is a source outcome. Other exceptions
    // remain wrapper/preparation failures and abort orchestration honestly.
    try {
        function();
        return {{"status", "source_returned"}, {"source_fatal", false}, {"exception_message", nullptr}};
    } catch (EnergyPlus::FatalError const &error) {
        return {{"status", "source_fatal"}, {"source_fatal", true}, {"exception_message", error.what()}};
    }
}

json raw_call(Factory &factory, Clk02::RawOutputs &outputs, json const &canaries, std::string const &line)
{
    seed_outputs(outputs, canaries);
    auto const before = Clk02::raw_outputs(outputs);
    int const missed_before = factory.state->dataWeather->wvarsMissedCounts.WeathCodes;
    auto const outcome = invoke([&] {
        EnergyPlus::Weather::InterpretWeatherDataLine(*factory.state, line, outputs.ErrorFound,
#define PASS_DATE(name) outputs.name,
            CLK02_DATES(PASS_DATE)
#undef PASS_DATE
#define PASS_REAL(name) outputs.name,
            CLK02_MANDATORY_REALS(PASS_REAL)
#undef PASS_REAL
            outputs.WObs, outputs.WCodesArr, outputs.PrecipWater, outputs.AerosolOptDepth, outputs.SnowDepth,
            outputs.DaysSinceLastSnow, outputs.Albedo, outputs.LiquidPrecip);
    });
    return {{"before", before}, {"after", Clk02::raw_outputs(outputs)}, {"call_outcome", outcome},
            {"weather_code_missed_count_before", missed_before},
            {"weather_code_missed_count_after", factory.state->dataWeather->wvarsMissedCounts.WeathCodes},
            {"public_storage_defined", true}, {"individual_source_write_events_observed", false},
            {"fatal_output_policy", "actual-reference-arguments-retain-defined-caller-storage; source-local-write-completion-not-inferred"}};
}

json record_sequence(json const &item, fs::path const &output)
{
    require(item.at("route").get<std::string>() == "whole-original-InterpretWeatherDataLine" &&
            item.at("fresh_owner") == true && item.at("reset_declared_output_canaries_before_each_call") == true,
            "Frozen record route/preparation differs");
    Factory factory;
    auto &owner = *factory.state->dataWeather;
    owner.wvarsMissedCounts.WeathCodes = integer(item.at("prepared_weather_code_missed_count"));
    require(item.at("prepared_EndDayOfMonth").size() == 12, "Twelve prepared month ends required");
    for (int i = 1; i <= 12; ++i) owner.EndDayOfMonth(i) = integer(item.at("prepared_EndDayOfMonth").at(i - 1));
    auto const prepared = Clk02::header_state(*factory.state);
    Clk02::RawOutputs outputs;
    json operations = json::array();
    for (auto const &operation : item.at("operations")) {
        auto const line = operation.at("line_utf8").get<std::string>();
        require(text_sha(line) == operation.at("line_sha256").get<std::string>() &&
                operation.at("expected_values_supplied") == false && operation.at("expected_exit_supplied") == false,
                "Frozen raw line identity/no-answer policy differs");
        auto row = raw_call(factory, outputs, item.at("initial_outputs"), line);
        row["record_id"] = operation.at("record_id");
        row["kind"] = operation.at("kind");
        row["input_line_utf8"] = line;
        row["input_line_sha256"] = text_sha(line);
        operations.push_back(std::move(row));
    }
    auto const id = item.at("sequence_id").get<std::string>();
    return {{"sequence_id", id}, {"route", item.at("route")}, {"constructor", factory.constructor},
            {"prepared", prepared}, {"operations", operations}, {"final_state", Clk02::header_state(*factory.state)},
            {"actual_wrapper_root_invocations", operations.size()}, {"source_only_diagnostics", factory.diagnostics(output, id)},
            {"physics_executed", false}};
}

EnergyPlus::Weather::EpwHeaderType header_type(std::string const &name)
{
    using Type = EnergyPlus::Weather::EpwHeaderType;
    if (name == "Location") return Type::Location;
    if (name == "HolidaysDST") return Type::HolidaysDST;
    if (name == "DataPeriods") return Type::DataPeriods;
    if (name == "Comments1") return Type::Comments1;
    if (name == "DesignConditions") return Type::DesignConditions;
    throw std::runtime_error("Unadmitted direct header enum: " + name);
}

void prepare_header(Factory &factory, json const &input)
{
    auto &owner = *factory.state->dataWeather;
#define PREPARE_REAL(name) require(input.at(#name).is_number() && !input.at(#name).is_boolean(), "Typed finite header canary required"); owner.name = input.at(#name).get<double>(); require(std::isfinite(owner.name), "Nonfinite header canary excluded");
    PREPARE_REAL(WeatherFileLatitude)
    PREPARE_REAL(WeatherFileLongitude)
    PREPARE_REAL(WeatherFileTimeZone)
    PREPARE_REAL(WeatherFileElevation)
#undef PREPARE_REAL
    owner.EPWHeaderTitle = input.at("EPWHeaderTitle").get<std::string>();
    owner.NumEPWTypExtSets = integer(input.at("NumEPWTypExtSets"));
    owner.LeapYearAdd = integer(input.at("LeapYearAdd"));
    require(owner.NumEPWTypExtSets == 0 && integer(input.at("InputProcessorSpecialDaysObjectCount")) == 0,
            "Prepared direct header count prerequisites differ");
    require(factory.state->dataInputProcessing->inputProcessor->getNumObjectsFound(*factory.state, "RunPeriodControl:SpecialDays") == 0,
            "Genuine empty-input SpecialDays prerequisite differs");
}

json header_case(json const &item, fs::path const &root, fs::path const &output)
{
    Factory factory;
    require(item.at("initial_ErrorsFound").is_boolean(), "Typed ErrorsFound canary required");
    bool errors = item.at("initial_ErrorsFound").get<bool>();
    std::string const route = item.at("route").get<std::string>();
    bool const direct = route == "direct-original-ProcessEPWHeader";
    require(direct || route == "whole-original-OpenEPlusWeatherFile", "Unadmitted header route");
    auto const binding = item.at(direct ? "stream" : "file");
    auto const path = bound_path(root, binding);
    std::string line;
    if (direct) {
        prepare_header(factory, item.at("prepared_state"));
        line = item.at("initial_Line").get<std::string>();
        require(text_sha(line) == item.at("initial_Line_sha256").get<std::string>(), "Direct header Line identity differs");
        factory.state->files.inputWeatherFile.filePath = path;
        factory.state->files.inputWeatherFile.open();
        require(factory.state->files.inputWeatherFile.good(), "Cannot open real header continuation stream");
    } else {
        factory.state->dataWeather->LeapYearAdd = integer(item.at("prepared_LeapYearAdd"));
        factory.state->files.inputWeatherFilePath.filePath = path;
        require(item.at("ProcessHeader") == true, "Whole header processing must be enabled");
    }
    auto const prepared = Clk02::header_state(*factory.state);
    auto const stream_before = Clk02::stream_state(factory.state->files.inputWeatherFile);
    bool const errors_before = errors;
    std::string const line_before = line;
    auto const outcome = invoke([&] {
        if (direct) EnergyPlus::Weather::ProcessEPWHeader(*factory.state, header_type(item.at("header_type").get<std::string>()), line, errors);
        else EnergyPlus::Weather::OpenEPlusWeatherFile(*factory.state, errors, true);
    });
    auto const id = item.at("case_id").get<std::string>();
    return {{"case_id", id}, {"route", route}, {"input_declared_binding", binding}, {"actual_input_file", file_ref(path)},
            {"constructor", factory.constructor}, {"prepared", prepared}, {"before", prepared},
            {"after", Clk02::header_state(*factory.state)}, {"ErrorsFound_before", errors_before}, {"ErrorsFound_after", errors},
            {"Line_observed", direct}, {"Line_before", direct ? json(line_before) : json(nullptr)},
            {"Line_after", direct ? json(line) : json(nullptr)},
            {"Line_before_sha256", direct ? json(text_sha(line_before)) : json(nullptr)},
            {"Line_after_sha256", direct ? json(text_sha(line)) : json(nullptr)},
            {"Line_unavailability", direct ? "none" : "whole-OpenEPlusWeatherFile-local-Line-not-exported"},
            {"stream_before", stream_before}, {"stream_after", Clk02::stream_state(factory.state->files.inputWeatherFile)},
            {"call_outcome", outcome}, {"actual_wrapper_root_invocations", 1}, {"internal_dispatch_or_read_call_counts_observed", false},
            {"source_only_diagnostics", factory.diagnostics(output, id)}, {"physics_executed", false}};
}

json fixed_epw(json const &item, fs::path const &root, fs::path const &output)
{
    Factory factory;
    auto const path = bound_path(root, item.at("file"));
    auto &file = factory.state->files.inputWeatherFile;
    factory.state->files.inputWeatherFilePath.filePath = path;
    require(item.at("initial_ErrorsFound").is_boolean() && item.at("ProcessHeader") == true &&
            item.at("same_owner_for_all_raw_records") == true && item.at("reset_declared_output_canaries_before_each_call") == true,
            "Fixed EPW input-only lifecycle policy differs");
    bool errors = item.at("initial_ErrorsFound").get<bool>();
    auto const prepared = Clk02::header_state(*factory.state);
    auto const stream_before = Clk02::stream_state(file);
    auto const open_outcome = invoke([&] { EnergyPlus::Weather::OpenEPlusWeatherFile(*factory.state, errors, true); });
    auto const header_after = Clk02::header_state(*factory.state);
    auto const stream_after_header = Clk02::stream_state(file);
    json records = json::array();
    Clk02::RawOutputs outputs;
    std::size_t read_attempts = 0;
    bool all_records_returned = !open_outcome.at("source_fatal").get<bool>();
    json terminal_read = nullptr;
    if (!open_outcome.at("source_fatal").get<bool>()) {
        while (true) {
            auto const before_read = Clk02::stream_state(file);
            auto const read = file.readLine();
            ++read_attempts;
            auto const after_read = Clk02::stream_state(file);
            if (!read.good) {
                terminal_read = {{"read_ordinal", read_attempts}, {"data", read.data}, {"eof", read.eof}, {"good", read.good},
                                 {"stream_before", before_read}, {"stream_after", after_read}};
                break;
            }
            auto row = raw_call(factory, outputs, item.at("initial_outputs"), read.data);
            row["record_index"] = records.size();
            row["read_ordinal"] = read_attempts;
            row["input_line_utf8"] = read.data;
            row["input_line_sha256"] = text_sha(read.data);
            row["read_eof"] = read.eof;
            row["read_good"] = read.good;
            row["stream_before_read"] = before_read;
            row["stream_after_read"] = after_read;
            if (row.at("call_outcome").at("source_fatal").get<bool>()) all_records_returned = false;
            records.push_back(std::move(row));
        }
    }
    auto const id = item.at("case_id").get<std::string>();
    return {{"case_id", id}, {"route", item.at("route")}, {"input_declared_binding", item.at("file")}, {"actual_input_file", file_ref(path)},
            {"constructor", factory.constructor}, {"prepared", prepared}, {"header_after", header_after},
            {"ErrorsFound_before", item.at("initial_ErrorsFound")}, {"ErrorsFound_after", errors}, {"open_call_outcome", open_outcome},
            {"stream_before_open", stream_before}, {"stream_after_header", stream_after_header},
            {"record_count", records.size()}, {"records", records}, {"terminal_read", terminal_read},
            {"actual_wrapper_readLine_invocations_after_open", read_attempts}, {"actual_wrapper_raw_parser_invocations", records.size()},
            {"actual_wrapper_open_invocations", 1}, {"all_records_returned_normally", all_records_returned},
            {"final_state", Clk02::header_state(*factory.state)}, {"final_stream", Clk02::stream_state(file)},
            {"source_only_diagnostics", factory.diagnostics(output, id)}, {"physics_executed", false},
            {"civil_calendar_supplied_to_raw_parser", false}, {"whole_simulation_weather_admission_claimed", false}};
}

} // namespace

int main(int argc, char const *argv[])
{
    try {
        require(argc == 4, "Usage: clk02_reference_helper <repo_root> <input-only-request.json> <fresh_output_dir>");
        fs::path const root = fs::absolute(argv[1]).lexically_normal();
        fs::path const request_path = fs::absolute(argv[2]).lexically_normal();
        fs::path const output = fs::absolute(argv[3]).lexically_normal();
        require(!fs::exists(output), "Native output directory must be fresh");
        auto const request = read_json(request_path);
        require(request.at("schema").get<std::string>() == "clk02-helper-cases.v1" &&
                request.at("expected_values_supplied") == false && request.at("expected_exits_supplied") == false,
                "Wrong input-only helper schema/policy");
        json contracts = json::object();
        for (char const *name : {"source", "cases", "tolerances"}) {
            auto const path = root / "energyplus_porting_plan/contracts" / (std::string("CLK-02-") + name + ".json");
            auto const contract = read_json(path);
            require(contract.at("status").get<std::string>() == "frozen-before-numerical-execution" &&
                    contract.at("frozen_before_numerical_execution") == true, "Unfrozen contract");
            if (std::string(name) == "cases") require(file_sha(request_path) == contract.at("helper_request").at("sha256").get<std::string>(),
                                                     "Input request differs from frozen cases");
            if (std::string(name) == "source") {
                require(contract.at("energyplus_commit").get<std::string>() == "6f2e40d10250a105b49966baa24d843711e61048", "Wrong original source pin");
                for (auto const &file : contract.at("source_files"))
                    require(file_sha(root / ".reference/energyplus-src/26.1.0" / file.at("path").get<std::string>()) ==
                            file.at("sha256").get<std::string>(), "Pinned original source file changed");
            }
            contracts[name] = file_ref(path);
        }
        require(request.at("record_sequences").size() == 40 && request.at("header_cases").size() == 22,
                "Frozen root cardinalities differ");
        fs::create_directories(output);
        json sequences = json::array(), headers = json::array();
        std::size_t diagnostic_raw_calls = 0;
        for (auto const &item : request.at("record_sequences")) {
            auto row = record_sequence(item, output);
            diagnostic_raw_calls += row.at("operations").size();
            sequences.push_back(std::move(row));
        }
        for (auto const &item : request.at("header_cases")) headers.push_back(header_case(item, root, output));
        auto fixed = fixed_epw(request.at("fixed_epw"), root, output);
        json result = {{"schema", "clk02-helper-results.v1"}, {"source_commit", "6f2e40d10250a105b49966baa24d843711e61048"},
                       {"contracts", contracts}, {"actual_request", file_ref(request_path)}, {"actual_binary", file_ref(fs::absolute(argv[0]))},
                       {"record_sequences", sequences}, {"header_cases", headers}, {"fixed_epw", fixed},
                       {"actual_wrapper_counts", {{"diagnostic_raw_calls", diagnostic_raw_calls}, {"diagnostic_header_roots", headers.size()},
                                                  {"fixed_epw_raw_calls", fixed.at("record_count")}, {"fixed_epw_open_roots", 1}}},
                       {"complete", true}, {"physics_executed", false}, {"expected_answers_supplied", false}, {"gates_updated", false},
                       {"source_locals_observed", false}, {"original_warning_error_IO_peer_parity_claimed", false},
                       {"direct_enum_and_whole_open_routes_distinct", true}, {"unpaired_downstream_weather_science", true}};
        write_text(output / "results.json", result.dump() + "\n");
        std::cout << json({{"status", "wrapper-complete-source-outcomes-preserved"}, {"result", file_ref(output / "results.json")},
                          {"actual_wrapper_counts", result.at("actual_wrapper_counts")}}).dump() << '\n';
        return 0;
    } catch (std::exception const &error) {
        std::cerr << error.what() << '\n';
        return 2;
    }
}
