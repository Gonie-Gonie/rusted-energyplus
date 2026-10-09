// Proposed one-way SCH-01 packet admission. No schedule values or expected outcomes.
#ifndef SCH01_REFERENCE_ADMISSION_HH
#define SCH01_REFERENCE_ADMISSION_HH
namespace Sch01Admission {
struct Packet {
    json request, admission, contracts, requested_counts;
    fs::path projection_path;
};
inline Packet admit(fs::path const &root, fs::path const &request_path, fs::path const &admission_path)
{
    Packet packet; packet.request = read_json(request_path); packet.admission = read_json(admission_path);
    auto const &request = packet.request; auto const &admission = packet.admission;
    require(request.at("schema") == "sch01-helper-cases.v1" && admission.at("schema") == "sch01-native-admission-contract.v1" &&
            request.at("energyplus_commit") == "6f2e40d10250a105b49966baa24d843711e61048" &&
            admission.at("energyplus_commit") == request.at("energyplus_commit"), "Wrong original source/schema pin");
    for (auto const *value : {&request, &admission}) require(value->at("frozen_before_numerical_execution") == true &&
        value->at("expected_values_supplied") == false && value->at("expected_exits_supplied") == false &&
        value->at("scientific_execution_performed") == false, "Input-only pre-execution policy required");
    require(bound_path(root, admission.at("helper_request")) == fs::weakly_canonical(request_path) &&
            request.at("fixed_scope") == admission.at("fixed_scope") && request.at("all_45_guard_refs") == admission.at("all_45_guard_refs"),
            "One-way request/scope binding differs");
    for (auto const *key : {"actual_scope_amendment", "actual_scope_amendment_execution"}) {
        require(request.at(key) == admission.at(key), "Actual scope amendment binding differs"); bound_path(root, request.at(key));
    }
    auto const scope = read_json(bound_path(root, request.at("fixed_scope"))); json guards = json::array();
    for (auto const &row : scope.at("cases")) for (auto const *key : {"input", "weather", "metadata"}) guards.push_back(row.at(key));
    require(guards.size() == 45 && guards == request.at("all_45_guard_refs"), "Exact ordered 45 production input guards required");
    for (auto const &row : guards) bound_path(root, row);
    require(request.at("contracts").is_object() && request.at("contracts").size() == 2 &&
            admission.at("contracts").is_object() && admission.at("contracts").size() == 3 &&
            request.at("contracts").at("source") == admission.at("contracts").at("source") &&
            request.at("contracts").at("tolerances") == admission.at("contracts").at("tolerances"), "One-way source/tolerance DAG differs");
    packet.contracts = json::object();
    for (auto const *key : {"source", "cases", "tolerances"}) {
        auto const path = bound_path(root, admission.at("contracts").at(key)); auto const value = read_json(path);
        require(value.at("schema") == std::string("sch01-") + key + "-contract.v1" &&
                value.at("frozen_before_numerical_execution") == true, "Contract schema/frozen policy differs");
        if (std::string(key) == "source") {
            require(value.at("energyplus_commit") == request.at("energyplus_commit"), "Original source identity differs");
            auto const source_root = fs::weakly_canonical(root / ".reference/energyplus-src/26.1.0");
            for (auto const &source : value.at("source_files")) {
                json binding = source;
                binding["path"] = (fs::path(".reference/energyplus-src/26.1.0") / source.at("path").get<std::string>()).generic_string();
                auto const actual = bound_path(root, binding); auto const inside = actual.lexically_relative(source_root);
                require(!inside.empty() && *inside.begin() != "..", "Original source path escapes pinned source tree");
            }
        }
        if (std::string(key) == "cases") require(value.at("helper_request") == admission.at("helper_request") &&
            value.at("source") == admission.at("contracts").at("source") && value.at("tolerances") == admission.at("contracts").at("tolerances") &&
            value.at("fixed_CON_scope") == request.at("fixed_scope") && value.at("all_45_guard_refs") == guards, "Case contract DAG differs");
        packet.contracts[key] = file_ref(path);
    }
    require(admission.at("native_source_files").is_array() && admission.at("native_source_files").size() == 3,
            "Exact three held SCH helper sources required");
    std::set<std::string> native_names;
    for (auto const &row : admission.at("native_source_files"))
        require(native_names.insert(bound_path(root, row).filename().string()).second, "Duplicate native source filename");
    require(native_names == std::set<std::string>{"sch01_reference_helper.cpp", "sch01_reference_fields.hh", "sch01_reference_admission.hh"},
            "Actual three helper source names differ");
    packet.projection_path = bound_path(root, admission.at("projection")); auto const projection = read_json(packet.projection_path);
    require(projection.at("schema") == "sch01-observation-projection.v1" && projection.at("frozen_before_numerical_execution") == true &&
            projection.at("helper_request") == admission.at("helper_request") && projection.at("contracts") == admission.at("contracts") &&
            projection.at("native_observation_policy") == request.at("observation_policy"), "Observation DAG differs");
    auto const &models = request.at("model_cases");
    require(models.is_array() && !models.empty() && models.size() <= 256, "Bounded nonempty model case array required");
    packet.requested_counts = {{"model_cases", models.size()}, {"ProcessInput", models.size()},
                               {"ProcessScheduleInput", models.size()}, {"total_operations", 2 * models.size()}};
    std::set<std::string> case_ids;
    for (auto const &row : models) {
        require(row.is_object() && row.size() == 5, "Exactly id/lane/input/caller/operations model fields required");
        identifier(row); require(case_ids.insert(row.at("id").get<std::string>()).second, "Duplicate case ID");
        require(row.at("lane") == "production-CON" || row.at("lane") == "diagnostic-model", "Unknown model lane");
        auto const input = bound_path(root, row.at("input")); require(input.extension() == ".idf", "Actual IDF model input required");
        auto const &caller = row.at("caller");
        require(caller.is_object() && caller.size() == 5 && caller.at("isEpJSON") == false && caller.at("preserveIDFOrder") == true,
                "Only declared actual IDF parser and preserved order admitted");
        int const steps = integer(caller.at("TimeStepsInHour")), minutes = integer(caller.at("MinutesInTimeStep"));
        require(steps >= 1 && steps <= 60 && 60 % steps == 0 && minutes == 60 / steps, "Consistent caller timestep required before any engine call");
        auto const fraction = input_real(caller.at("TimeStepZone"));
        require(std::bit_cast<std::uint64_t>(fraction) == std::bit_cast<std::uint64_t>(1.0 / static_cast<double>(steps)),
                "Literal caller timestep fraction bits must equal 1.0 / TimeStepsInHour");
        if (row.at("lane") == "production-CON") {
            bool found = false;
            for (auto const &existing : scope.at("cases")) if (row.at("input").at("sha256") == existing.at("input").at("sha256") &&
                input == bound_path(root, existing.at("input"))) found = true;
            require(found, "Production model must be an unchanged admitted CON input");
        }
        auto const &operations = row.at("operations");
        require(operations.is_array() && operations.size() == 2 && operations.at(0).at("kind") == "ProcessInput" &&
                operations.at(1).at("kind") == "ProcessScheduleInput", "Exact genuine parser then scheduler call order required");
        std::set<std::string> operation_ids;
        for (auto const &operation : operations) {
            require(operation.is_object() && operation.size() == 2, "Only operation id/kind admitted"); identifier(operation);
            require(operation_ids.insert(operation.at("id").get<std::string>()).second, "Duplicate operation ID");
        }
    }
    require(request.at("requested_counts") == packet.requested_counts && admission.at("requested_counts") == packet.requested_counts,
            "Literal requested counts differ from actual input arrays");
    return packet;
}
} // namespace Sch01Admission
#endif
