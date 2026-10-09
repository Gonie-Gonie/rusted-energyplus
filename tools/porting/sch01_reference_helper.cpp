// Genuine InputProcessor/ScheduleManager calls. No copied schedule algorithm or expected answer.
#ifndef WIN32_LEAN_AND_MEAN
#define WIN32_LEAN_AND_MEAN
#endif
#ifndef NOMINMAX
#define NOMINMAX
#endif
#include <windows.h>
#include <bcrypt.h>
#include "sch01_reference_fields.hh"
#include <EnergyPlus/DataStringGlobals.hh>
#include <array>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <memory>
#include <set>
#include <typeinfo>
#include <utility>

namespace {
using Sch01::json;
using Sch01::require;
namespace fs = std::filesystem;

// Byte authentication only; none of these hashes is an engine operand.
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
        require(size <= MAXDWORD && BCryptHashData(hash_, const_cast<PUCHAR>(static_cast<unsigned char const *>(data)),
                static_cast<ULONG>(size), 0) >= 0, "Cannot hash input");
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
fs::path bound_path(fs::path const &root, json const &binding)
{
    require(binding.at("path").is_string() && binding.at("sha256").is_string(), "Input binding requires path/SHA256");
    fs::path const relative(binding.at("path").get<std::string>());
    auto const digest = binding.at("sha256").get<std::string>();
    require(relative.is_relative() && digest.size() == 64 && std::all_of(digest.begin(), digest.end(), [](char c) {
        return (c >= '0' && c <= '9') || (c >= 'a' && c <= 'f'); }), "Relative input path and lowercase SHA256 required");
    auto const path = fs::weakly_canonical(root / relative); auto const inside = path.lexically_relative(root);
    DWORD const attributes = GetFileAttributesW(path.c_str());
    require(!inside.empty() && *inside.begin() != ".." && fs::is_regular_file(path) &&
            attributes != INVALID_FILE_ATTRIBUTES && !(attributes & FILE_ATTRIBUTE_REPARSE_POINT) && fs::file_size(path) <= 64000000,
            "Input outside repository, oversized, or not a regular non-reparse file");
    require(file_sha(path) == digest, "Frozen input hash differs");
    for (auto const *key : {"bytes", "size_bytes"}) if (binding.contains(key))
        require(binding.at(key).is_number_unsigned() || (binding.at(key).is_number_integer() && binding.at(key).get<std::int64_t>() >= 0),
                "Byte length must be a nonnegative integer");
    for (auto const *key : {"bytes", "size_bytes"}) if (binding.contains(key))
        require(binding.at(key).get<std::uint64_t>() == fs::file_size(path), "Frozen byte length differs");
    return path;
}
json read_json(fs::path const &path)
{
    require(fs::is_regular_file(path) && fs::file_size(path) <= 16000000, "Small metadata JSON required");
    std::ifstream input(path, std::ios::binary); require(input.is_open(), "Cannot open metadata JSON"); return json::parse(input);
}
void write_text(fs::path const &path, std::string const &value)
{
    require(!fs::exists(path), "Refusing to overwrite native output");
    std::ofstream output(path, std::ios::binary); require(output.is_open(), "Cannot create native output");
    output.write(value.data(), static_cast<std::streamsize>(value.size())); output.close();
    require(!output.fail(), "Cannot finish native output");
}
void identifier(json const &row)
{
    auto const id = row.at("id").get<std::string>();
    require(!id.empty() && id.size() <= 80 && std::all_of(id.begin(), id.end(), [](char c) {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9') || c == '-' || c == '_';
    }), "Bounded ASCII case/operation identifier required");
}
int integer(json const &value)
{
    require(value.is_number_integer() && !value.is_boolean(), "Typed integer required");
    auto const result = value.get<std::int64_t>();
    require(result >= 0 && result <= 1000000, "Caller integer outside bounded nonnegative domain");
    return static_cast<int>(result);
}
double input_real(json const &value)
{
    require(value.is_object() && value.size() == 1 && value.at("bits").is_string(), "Literal binary64 input bits required");
    auto const text = value.at("bits").get<std::string>();
    require(text.size() == 16 && std::all_of(text.begin(), text.end(), [](char c) {
        return (c >= '0' && c <= '9') || (c >= 'a' && c <= 'f'); }), "Sixteen lowercase hex digits required");
    auto const result = std::bit_cast<double>(static_cast<std::uint64_t>(std::stoull(text, nullptr, 16)));
    require(std::isfinite(result), "Finite literal caller input required"); return result;
}
template<class Function> json invoke(Function &&function)
{
    try { function(); return {{"status", "source_returned"}, {"source_fatal", false}, {"source_exception", false},
                             {"exception_type", nullptr}, {"exception_message", nullptr}}; }
    catch (EnergyPlus::FatalError const &error) {
        return {{"status", "source_fatal"}, {"source_fatal", true}, {"source_exception", false},
                {"exception_type", typeid(error).name()}, {"exception_message", error.what()}};
    }
    catch (std::exception const &error) {
        return {{"status", "source_exception"}, {"source_fatal", false}, {"source_exception", true},
                {"exception_type", typeid(error).name()}, {"exception_message", error.what()}};
    }
    // exception_type is the actual implementation-defined RTTI name, not an inferred source fatal.
}
struct Factory {
    std::unique_ptr<EnergyPlus::EnergyPlusData> state = std::make_unique<EnergyPlus::EnergyPlusData>();
    std::ostringstream *error_text = nullptr;
    json messages = json::array(), constructor, initialization;
    bool stopped = false;
    std::string stop_reason;
    Factory()
    {
        constructor = Sch01::snapshot(*state);
        auto sink = std::make_unique<std::ostringstream>(); error_text = sink.get(); state->files.err_stream = std::move(sink);
        state->files.audit.open_as_stringstream(); state->files.eso.open_as_stringstream(); state->files.eio.open_as_stringstream();
        state->dataGlobal->errorCallback = [this](EnergyPlus::Error level, std::string const &message) {
            messages.push_back({{"level", static_cast<int>(level)}, {"message", message}});
        };
        auto const outcome = invoke([&] { state->init_constant_state(*state); });
        initialization = {{"kind", "EnergyPlusData::init_constant_state"}, {"before", constructor},
                          {"after", Sch01::snapshot(*state)}, {"call_outcome", outcome}, {"actual_source_invoked", true}};
        stopped = outcome.at("status") != "source_returned";
        if (stopped) stop_reason = outcome.at("status").get<std::string>();
    }
    json diagnostics(fs::path const &output, std::string const &id)
    {
        json logs = json::object();
        for (auto const &[name, text] : std::array<std::pair<std::string, std::string>, 4>{
                 {{"err", error_text->str()}, {"audit.txt", state->files.audit.get_output()},
                  {"eso.txt", state->files.eso.get_output()}, {"eio.txt", state->files.eio.get_output()}}}) {
            auto const path = output / (id + "." + name); write_text(path, text); logs[name] = file_ref(path);
        }
        return {{"messages", messages}, {"logs", logs}, {"actual_native_stringstream_sinks", true}, {"diagnostic_text_numerically_paired", false}};
    }
};
void seed_caller(Factory &factory, fs::path const &idf, json const &caller)
{
    require(caller.is_object() && caller.size() == 5 && caller.at("isEpJSON") == false && caller.at("preserveIDFOrder") == true,
            "Only declared actual IDF parser and preserved input order admitted");
    int const steps = integer(caller.at("TimeStepsInHour")), minutes = integer(caller.at("MinutesInTimeStep"));
    require(steps >= 1 && steps <= 60 && 60 % steps == 0 && minutes == 60 / steps, "Consistent source timestep caller context required");
    auto const fraction = input_real(caller.at("TimeStepZone"));
    require(std::bit_cast<std::uint64_t>(fraction) == std::bit_cast<std::uint64_t>(1.0 / static_cast<double>(steps)),
            "Declared caller timestep fraction bits must equal 1.0 / TimeStepsInHour");
    auto &state = *factory.state;
    state.dataStrGlobals->inputFilePath = idf;
    state.dataGlobal->TimeStepsInHour = steps; state.dataGlobal->MinutesInTimeStep = minutes; state.dataGlobal->TimeStepZone = fraction;
    state.dataGlobal->isEpJSON = caller.at("isEpJSON").get<bool>();
    state.dataGlobal->preserveIDFOrder = caller.at("preserveIDFOrder").get<bool>();
    require(!state.dataGlobal->AnyEnergyManagementSystemInModel && !state.dataGlobal->outputEpJSONConversion &&
            !state.dataGlobal->outputEpJSONConversionOnly, "Unmodified default non-EMS/no-conversion caller context required");
}
void admitted_parsed_scope(EnergyPlus::EnergyPlusData const &state)
{
    for (auto const &item : state.dataInputProcessing->inputProcessor->epJSON.items()) {
        auto const &name = item.key();
        bool const schedule = name.starts_with("Schedule:");
        require(!schedule || name == "Schedule:Constant" || name == "Schedule:Compact", "Other schedule families not admitted");
        require(!name.starts_with("EnergyManagementSystem:") && name != "Output:Schedules", "EMS/output schedule registration not admitted");
    }
}
json model_case(json const &row, fs::path const &root, fs::path const &output)
{
    auto const input = bound_path(root, row.at("input")); Factory factory;
    auto const before_caller = Sch01::snapshot(*factory.state);
    if (!factory.stopped) seed_caller(factory, input, row.at("caller"));
    auto const after_caller = Sch01::snapshot(*factory.state); json operations = json::array();
    for (auto const &operation : row.at("operations")) {
        auto const kind = operation.at("kind").get<std::string>(); auto const before = Sch01::snapshot(*factory.state);
        auto const diagnostic_start = factory.messages.size(); bool const invoked = !factory.stopped;
        json outcome = {{"status", "source_not_invoked_after_stop"}, {"source_fatal", false}, {"source_exception", false},
                        {"exception_type", nullptr}, {"exception_message", nullptr}, {"prior_source_stop_reason", factory.stop_reason}};
        if (invoked) {
            // Admission is a wrapper guard. Do not misclassify it as an exception from an Original call.
            if (kind == "ProcessScheduleInput") admitted_parsed_scope(*factory.state);
            outcome = invoke([&] {
                if (kind == "ProcessInput") factory.state->dataInputProcessing->inputProcessor->processInput(*factory.state);
                else EnergyPlus::Sched::ProcessScheduleInput(*factory.state);
            });
            if (outcome.at("status") != "source_returned") {
                factory.stopped = true; factory.stop_reason = outcome.at("status").get<std::string>();
            }
        }
        operations.push_back({{"id", operation.at("id")}, {"kind", kind}, {"before", before}, {"after", Sch01::snapshot(*factory.state)},
                              {"actual_source_invoked", invoked}, {"call_outcome", outcome},
                              {"diagnostic_message_begin", diagnostic_start}, {"diagnostic_message_end", factory.messages.size()}});
    }
    bound_path(root, row.at("input"));
    return {{"id", row.at("id")}, {"lane", row.at("lane")}, {"input", file_ref(input)}, {"declared_caller", row.at("caller")},
            {"constructor", factory.constructor}, {"constant_initialization", factory.initialization},
            {"before_caller", before_caller}, {"after_caller", after_caller}, {"operations", operations},
            {"final_state", Sch01::snapshot(*factory.state)}, {"source_diagnostics", factory.diagnostics(output, row.at("id").get<std::string>())},
            {"whole_simulation_initialization_claimed", false}, {"SCH02_runtime_arithmetic_certified", false}};
}
} // namespace
#include "sch01_reference_admission.hh"

int main(int argc, char **argv)
{
    try {
        require(argc == 5, "Usage: sch01_reference_helper <root> <frozen-request> <native-admission-contract> <fresh-output>");
        auto const root = fs::weakly_canonical(fs::absolute(argv[1]));
        auto const request_path = fs::absolute(argv[2]), admission_path = fs::absolute(argv[3]), output = fs::absolute(argv[4]);
        auto const inside = fs::weakly_canonical(output).lexically_relative(root / ".runtime/porting/SCH-01");
        require(!inside.empty() && *inside.begin() != ".." && !fs::exists(output), "Fresh SCH-01 output directory required");
        auto const packet = Sch01Admission::admit(root, request_path, admission_path); fs::create_directories(output);
        json rows = json::array(), calls = json::object(), statuses = json::object(); int invoked = 0, skipped = 0;
        for (auto const &row : packet.request.at("model_cases")) rows.push_back(model_case(row, root, output));
        for (auto const &row : rows) for (auto const &operation : row.at("operations")) {
            auto const kind = operation.at("kind").get<std::string>(), status = operation.at("call_outcome").at("status").get<std::string>();
            statuses[status] = (statuses.contains(status) ? statuses.at(status).get<int>() : 0) + 1;
            if (operation.at("actual_source_invoked").get<bool>()) { ++invoked; calls[kind] = (calls.contains(kind) ? calls.at(kind).get<int>() : 0) + 1; }
            else ++skipped;
        }
        auto const after = Sch01Admission::admit(root, request_path, admission_path);
        require(after.request == packet.request && after.admission == packet.admission && after.contracts == packet.contracts,
                "Input/source admission changed during actual calls");
        json const result = {{"schema", "sch01-helper-results.v1"}, {"energyplus_commit", packet.request.at("energyplus_commit")},
            {"actual_request", file_ref(request_path)}, {"native_admission_contract", file_ref(admission_path)},
            {"actual_scope_amendment", packet.request.at("actual_scope_amendment")},
            {"actual_scope_amendment_execution", packet.request.at("actual_scope_amendment_execution")},
            {"observation_projection", file_ref(packet.projection_path)}, {"contracts", packet.contracts}, {"actual_binary", file_ref(fs::absolute(argv[0]))},
            {"model_cases", rows}, {"requested_counts", packet.requested_counts}, {"actual_operation_invocations", invoked},
            {"actual_operations_skipped", skipped}, {"actual_source_function_counts", calls}, {"actual_source_status_counts", statuses},
            {"complete_observation_record", true}, {"scientific_comparison_performed", false}, {"expected_values_supplied", false},
            {"expected_exits_supplied", false}, {"whole_simulation_or_selected_scope_PASS_claimed", false},
            {"private_Through_Until_or_minute_buffers_observed", false}, {"SCH02_runtime_arithmetic_certified", false}, {"gates_updated", false}};
        write_text(output / "sch01-helper-results.json", result.dump(2) + "\n"); return 0;
    } catch (std::exception const &error) { std::cerr << error.what() << '\n'; return 1; }
}
