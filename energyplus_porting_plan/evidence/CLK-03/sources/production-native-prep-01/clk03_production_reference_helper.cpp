// Supplemental matched caller history. All scientific bodies stay in cached core.
#define main clk03_unchanged_dispatcher_main
#include "../../../../tools/porting/clk03_reference_helper.cpp"
#undef main

namespace {
json matched_operation(Factory &factory, json const &input)
{
    auto const before_caller = Clk03::snapshot(*factory.state);
    auto delegated = input;
    auto &caller = delegated.at("caller");
    require(caller.is_object(), "Declared partial caller object required");
    bool const string_write = caller.contains("DayOfSimChr");
    if (string_write) {
        require(caller.at("DayOfSimChr").is_string(), "Declared caller day text required");
        auto const day = Clk03::integer(caller.at("DayOfSim"));
        require(day > 0 && caller.at("DayOfSimChr").get<std::string>() == std::to_string(day),
                "Declared nonwarmup day text must match caller day");
        if (!factory.stopped) factory.state->dataGlobal->DayOfSimChr = caller.at("DayOfSimChr").get<std::string>();
        caller.erase("DayOfSimChr");
    }
    // The old input-only seed deliberately has no string-field write. This
    // adapter adds only the declared caller string, before the unchanged call.
    auto result = operation(factory, delegated, true);
    result["before_caller"] = before_caller;
    result["requested_operation"] = input;
    result["DayOfSimChr_caller_transport"] = string_write ? "declared-input-only-string-before-unchanged-operation" : "no-declared-string-write";
    return result;
}

json matched_sequence(json const &input, fs::path const &root, fs::path const &output)
{
    check_identifier(input);
    require(Clk03::integer(input.at("time_steps_per_hour")) == 4 && input.at("prepared_environment_lane") == true,
            "Frozen four-step prepared CON lane required");
    auto const &declared = input.at("native_preparation");
    check_preparation(declared);
    auto const idf = bound_path(root, input.at("input")), epw = bound_path(root, input.at("weather"));
    Factory factory;
    auto &state = *factory.state;
    auto &owner = *state.dataWeather;
    // Unchanged preparation call plumbing from the preserved CLK03 dispatcher.
    // No body of a WeatherManager or other scientific function is copied here.
    state.dataStrGlobals->inputFilePath = idf;
    state.files.inputWeatherFilePath.filePath = epw;
    state.dataGlobal->TimeStepsInHour = 4;
    state.dataGlobal->TimeStepZone = Clk03::input_real(declared.at("TimeStepZone"));
    state.dataGlobal->BeginSimFlag = Clk03::boolean(declared.at("BeginSimFlag"));
    state.dataGlobal->DoWeathSim = Clk03::boolean(declared.at("DoWeathSim"));
    state.dataGlobal->DoDesDaySim = Clk03::boolean(declared.at("DoDesDaySim"));
    factory.prepare("InputProcessor::processInput", [&] { state.dataInputProcessing->inputProcessor->processInput(state); });
    factory.prepare("SetupInterpolationValues", [&] { Weather::SetupInterpolationValues(state); });
    if (!factory.stopped) owner.TimeStepFraction = Clk03::input_real(declared.at("TimeStepFraction"));
    factory.prepare("OpenWeatherFile", [&] { Weather::OpenWeatherFile(state, factory.errors); });
    factory.prepare("CloseWeatherFile", [&] { Weather::CloseWeatherFile(state); });
    factory.prepare("ReadUserWeatherInput", [&] { Weather::ReadUserWeatherInput(state); });
    factory.prepare("AllocateWeatherData", [&] { Weather::AllocateWeatherData(state); });
    auto const allocated_defaults = Clk03::snapshot(state);
    factory.prepare("ResolveLocationInformation", [&] { Weather::ResolveLocationInformation(state, factory.errors); });
    factory.prepare("CheckLocationValidity", [&] { Weather::CheckLocationValidity(state); });
    if (!factory.stopped) {
        require(owner.TotRunPers == 1 && owner.TotRunDesPers == 0 && owner.NumIntervalsPerHour == 1,
                "Original parsed input outside sole hourly nonactual RunPeriod domain");
        auto const &environment = owner.Environment(state.dataEnvrn->TotDesDays + 1);
        require(environment.KindOfEnvrn == EnergyPlus::Constant::KindOfSim::RunPeriodWeather && !environment.ActualWeather &&
                !environment.MatchYear && environment.StartYear == 2013 && environment.EndYear == 2013,
                "Original resolved environment outside fixed ordinary2013 scope");
        owner.GetEnvironmentFirstCall = Clk03::boolean(declared.at("GetEnvironmentFirstCall"));
        owner.GetBranchInputOneTimeFlag = Clk03::boolean(declared.at("GetBranchInputOneTimeFlag"));
        owner.WaterMainsParameterReport = Clk03::boolean(declared.at("WaterMainsParameterReport"));
        owner.Envrn = state.dataEnvrn->TotDesDays;
        if (factory.errors) { factory.stopped = true; factory.stop_reason = "actual-errors-during-genuine-preparation"; }
    }
    auto const prepared = Clk03::snapshot(state);
    json operations = json::array();
    std::size_t index = 0;
    for (auto const &item : input.at("operations")) {
        auto const kind = item.at("kind").get<std::string>();
        require(kind == (index == 0 ? "GetNextEnvironment" : "InitializeWeather"),
                "Exactly initial selection followed by whole matched InitializeWeather calls required");
        operations.push_back(matched_operation(factory, item));
        ++index;
    }
    require(index > 1, "Matched source caller history is empty");
    require(file_sha(idf) == input.at("input").at("sha256").get<std::string>() &&
            file_sha(epw) == input.at("weather").at("sha256").get<std::string>(), "Inputs changed during genuine calls");
    return {{"id", input.at("id")}, {"lane", "whole-original-weather-functions-with-declared-matched-production-caller-history"},
            {"input", file_ref(idf)}, {"weather_input", file_ref(epw)}, {"declared_run_period_input", input.at("run_period")},
            {"constructor", factory.constructor}, {"after_constant_initialization", factory.after_constant_initialization},
            {"preparation_calls", factory.preparation}, {"allocated_defaults", allocated_defaults}, {"prepared", prepared},
            {"declared_native_preparation", declared}, {"operations", operations}, {"final_state", Clk03::snapshot(state)},
            {"source_only_diagnostics", factory.diagnostics(output, input.at("id").get<std::string>())},
            {"processed_weather_numerical_fields_paired", false}, {"source_internal_record_index_observed", false},
            {"whole_simulation_initialization_claimed", false}, {"Rust_outputs_supplied_as_inputs", false}};
}
} // namespace

int main(int argc, char **argv)
{
    try {
        require(argc == 5, "Usage: clk03_production_reference_helper <repo_root> <matched-input-only-request> <matched-native-contract> <fresh_output>");
        auto const root = fs::weakly_canonical(fs::absolute(argv[1]));
        auto const request_path = fs::absolute(argv[2]).lexically_normal();
        auto const contract_path = fs::absolute(argv[3]).lexically_normal();
        auto const output = fs::absolute(argv[4]).lexically_normal();
        require(!fs::exists(output), "Fresh native output directory required");
        auto const request = read_json(request_path);
        require(request.at("schema") == "clk03-production-helper-cases.v1" && request.at("expected_values_supplied") == false &&
                request.at("expected_exits_supplied") == false && request.at("scientific_execution_performed") == false &&
                request.at("source_commit") == "6f2e40d10250a105b49966baa24d843711e61048", "Frozen input-only source caller request required");
        for (auto const &binding : request.at("contracts").items()) bound_path(root, binding.value());
        bound_path(root, request.at("scope"));
        auto const contract = read_json(contract_path);
        require(contract.at("schema") == "clk03-matched-production-native-contract.v1" &&
                contract.at("expected_values_supplied") == false && contract.at("expected_exits_supplied") == false &&
                contract.at("scientific_execution_performed") == false && contract.at("source_commit") == request.at("source_commit") &&
                contract.at("contracts") == request.at("contracts") && contract.at("fixed_scope") == request.at("scope") &&
                contract.at("requested_counts") == request.at("requested_counts"), "Separate matched contract/request identity differs");
        require(bound_path(root, contract.at("request")) == fs::weakly_canonical(request_path), "Actual matched contract request bytes differ");
        require(request.at("cases").is_array() && request.at("cases").size() == 3, "Exactly three fixed CON production cases required");
        std::array<std::string, 3> const ids = {"A-24H", "A-72H", "B-BOTH-24H"};
        fs::create_directories(output);
        json cases = json::array();
        std::size_t invoked = 0, skipped = 0;
        json kinds = json::object(), outcomes = json::object();
        for (std::size_t i = 0; i < ids.size(); ++i) {
            auto const &input = request.at("cases").at(i);
            require(input.at("id") == ids[i], "Fixed case order required");
            auto observed = matched_sequence(input, root, output);
            for (auto const &item : observed.at("operations")) {
                auto const kind = item.at("kind").get<std::string>();
                auto const status = item.at("call_outcome").at("status").get<std::string>();
                outcomes[status] = outcomes.value(status, std::size_t{0}) + 1;
                if (item.at("actual_source_invoked").get<bool>()) {
                    ++invoked;
                    kinds[kind] = kinds.value(kind, std::size_t{0}) + 1;
                } else ++skipped;
            }
            cases.push_back(std::move(observed));
        }
        json result = {{"schema", "clk03-production-helper-results.v1"}, {"complete", true}, {"cases", cases},
                       {"source_commit", request.at("source_commit")}, {"actual_request", file_ref(request_path)},
                       {"actual_contract", file_ref(contract_path)}, {"actual_binary", file_ref(fs::absolute(argv[0]))},
                       {"contracts", request.at("contracts")}, {"scope", request.at("scope")},
                       {"requested_counts", request.at("requested_counts")},
                       {"actual_operation_invocations", invoked}, {"actual_operations_skipped", skipped}, {"actual_operation_counts", kinds},
                       {"actual_source_invocations", invoked}, {"actual_source_operations_skipped", skipped},
                       {"actual_source_function_counts", kinds}, {"actual_source_status_counts", outcomes},
                       {"expected_values_supplied", false}, {"expected_exits_supplied", false}, {"Rust_outputs_supplied_as_inputs", false},
                       {"expected_answers_supplied", false}, {"native_raw_record_index_claimed", false},
                       {"processed_weather_physics_retained_unpaired", true}, {"pure_body_fallback_used", false},
                       {"scientific_comparison_executed", false}, {"gates_updated", false}};
        write_text(output / "results.json", result.dump() + "\n");
        std::cout << "Completed whole original calls with declared matched source caller history.\n";
        return 0;
    } catch (std::exception const &error) {
        std::cerr << error.what() << '\n';
        return 1;
    }
}
