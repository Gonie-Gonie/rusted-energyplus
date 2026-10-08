// Direct genuine source member and manager calls, with explicit input-only state preparation.
#include "zon01_reference_fields.hh"
#include <EnergyPlus/DataHeatBalFanSys.hh>
#include <EnergyPlus/DataZoneControls.hh>
#include <EnergyPlus/DataZoneEnergyDemands.hh>
#include <EnergyPlus/DataZoneEquipment.hh>
#include <fstream>
#include <iostream>
#include <memory>
#include <new>
#include <stdexcept>

namespace {
using Zon01::json;
void require(bool value, std::string const &message)
{
    if (!value) throw std::runtime_error(message);
}
double own_input(json const &value, json const &token)
{
    require(value.is_number() && !value.is_boolean() && token.is_string(), "Finite input and authoritative bit string required");
    double const number = value.get<double>();
    require(std::isfinite(number) && Zon01::bits(number) == token.get<std::string>(), "Independently parsed binary64 input bits differ");
    return number;
}
void seed(EnergyPlus::ZoneTempPredictorCorrector::ZoneHeatBalanceData &zone, json const &fields, json const &tokens)
{
    require(fields.is_object() && tokens.is_object() && fields.size() == tokens.size(), "Seed field/bit objects differ");
    for (auto it = fields.begin(); it != fields.end(); ++it) {
        std::string const key = it.key();
        bool recognized = false;
#define SEED_SCALAR(name) if (key == #name) { zone.name = own_input(it.value(), tokens.at(key)); recognized = true; }
        ZON01_RETAINED_SCALARS(SEED_SCALAR)
        ZON01_WRITTEN_SCALARS(SEED_SCALAR)
#undef SEED_SCALAR
#define SEED_ARRAY(name) if (key == #name) { require(it.value().is_array() && it.value().size() == 4 && tokens.at(key).is_array() && tokens.at(key).size() == 4, "Four source slots required"); for (int j = 0; j < 4; ++j) zone.name[j] = own_input(it.value().at(j), tokens.at(key).at(j)); recognized = true; }
        ZON01_ARRAYS(SEED_ARRAY)
#undef SEED_ARRAY
        require(recognized, "Unknown selected source seed field: " + key);
    }
}
void original_zone_bulk_fragment(EnergyPlus::EnergyPlusData &state)
{
    using namespace EnergyPlus; // Supply the original enclosing namespace for the exact fragment.
    // Exact source2231-2239 includes constructor/currentW/TempTstatAir writes.
    // This is not the whole InitThermalAndFluxHistories or surface initialization.
#include ZON01_BULK_FRAGMENT
}
void prepare_guard_prerequisites(EnergyPlus::EnergyPlusData &state)
{
    auto &g = *state.dataGlobal;
    auto &owner = *state.dataZoneTempPredictorCorrector;
    auto &fan = *state.dataHeatBalFanSys;
    auto &demand = *state.dataZoneEnergyDemand;
    g.NumOfZones = 1;
    g.numSpaces = 0;
    g.BeginDayFlag = false;
    state.dataHeatBal->doSpaceHeatBalance = false;
    state.dataHeatBal->Zone.allocate(1);
    state.dataHeatBal->Zone(1).Name = "PREPARED-ONE-ZONE";
    // The concrete owner constructor was genuinely executed by allocation.
    // These arrays/flags bypass unrelated input/output registration; no parser claim.
    owner.InitZoneAirSetPointsOneTimeFlag = false;
    state.dataZoneCtrls->NumTempControlledZones = 0;
    state.dataZoneCtrls->NumComfortControlledZones = 0;
    state.dataZoneEquip->ZoneEquipInputsFilled = false;
    fan.zoneTstatSetpts.allocate(1);
    fan.LoadCorrectionFactor.dimension(1, 0.0);
    fan.TempControlType.dimension(1, EnergyPlus::HVAC::SetptType::Uncontrolled);
    fan.TempControlTypeRpt.dimension(1, 0);
    fan.TempTstatAir.dimension(1, EnergyPlus::DataHeatBalance::ZoneInitialTemp);
    fan.PreviousMeasuredZT1.dimension(1, 0.0);
    fan.PreviousMeasuredZT2.dimension(1, 0.0);
    fan.PreviousMeasuredZT3.dimension(1, 0.0);
    fan.PreviousMeasuredHumRat1.dimension(1, 0.0);
    fan.PreviousMeasuredHumRat2.dimension(1, 0.0);
    fan.PreviousMeasuredHumRat3.dimension(1, 0.0);
    demand.ZoneSysEnergyDemand.allocate(1);
    demand.ZoneSysMoistureDemand.allocate(1);
    demand.DeadBandOrSetback.dimension(1, false);
}
json evaluate(json const &item)
{
    auto state = std::make_unique<EnergyPlus::EnergyPlusData>();
    auto constructor = Zon01::snapshot(*state, "genuine-EnergyPlusData-constructor-no-zone-rows-allocated");
    state->init_constant_state(*state);
    state->dataZoneTempPredictorCorrector->zoneHeatBalance.allocate(1);
    auto allocated = Zon01::snapshot(*state, "genuine-allocated-concrete-zone-constructor-before-manager-prerequisites");
    prepare_guard_prerequisites(*state);
    auto prepared = Zon01::snapshot(*state, "declared-manager-prerequisites-no-initializer-call");
    auto &zone = state->dataZoneTempPredictorCorrector->zoneHeatBalance(1);
    json operations = json::array();
    json wrapper_invocations = {{"bare_begin_environment", 0}, {"guarded_init_zone_air_setpoints", 0},
                                {"bulk_reconstruct_and_current_w_seed", 0}};
    for (auto const &operation : item.at("operations")) {
        auto before_inputs = Zon01::snapshot(*state, "before-operation-input-writes");
        if (operation.contains("out_hum_rat")) {
            state->dataEnvrn->OutHumRat = own_input(operation.at("out_hum_rat"), operation.at("out_hum_rat_bits"));
        } else {
            require(!operation.contains("out_hum_rat_bits"), "Unbound OutHumRat bits");
        }
        if (operation.contains("begin_environment")) {
            require(operation.at("begin_environment").is_boolean(), "BeginEnvrnFlag input must be boolean");
            state->dataGlobal->BeginEnvrnFlag = operation.at("begin_environment").get<bool>();
        }
        auto before = Zon01::snapshot(*state, "after-declared-operation-inputs-before-original-call");
        std::string const name = operation.at("operation").get<std::string>();
        bool const eligible = state->dataZoneTempPredictorCorrector->MyEnvrnFlag && state->dataGlobal->BeginEnvrnFlag;
        if (name == "seed_zone_state") {
            seed(zone, operation.at("fields"), operation.at("input_field_bits"));
        } else if (name == "bare_begin_environment") {
            zone.beginEnvironmentInit(*state);
            wrapper_invocations[name] = wrapper_invocations.at(name).get<int>() + 1;
        } else if (name == "guarded_init_zone_air_setpoints") {
            EnergyPlus::ZoneTempPredictorCorrector::InitZoneAirSetPoints(*state);
            wrapper_invocations[name] = wrapper_invocations.at(name).get<int>() + 1;
        } else if (name == "bulk_reconstruct_and_current_w_seed") {
            original_zone_bulk_fragment(*state);
            wrapper_invocations[name] = wrapper_invocations.at(name).get<int>() + 1;
        } else {
            require(name == "snapshot", "Unknown original wrapper operation");
        }
        operations.push_back({{"operation_id", operation.at("operation_id")}, {"operation", name}, {"input", operation},
                              {"before_inputs", before_inputs}, {"before", before}, {"after", Zon01::snapshot(*state, "after-wrapper-operation")},
                              {"source_guard_eligibility_before_call", eligible}, {"member_call_count_observed", false},
                              {"wrapper_invocations", wrapper_invocations}});
    }
    return {{"sequence_id", item.at("sequence_id")}, {"kind", item.at("kind")}, {"input", item},
            {"constructor", constructor}, {"allocated_zone_constructor", allocated}, {"prepared_manager_state", prepared}, {"operations", operations},
            {"wrapper_invocations", wrapper_invocations}, {"status", "source_complete"},
            {"preparation", {{"zone_count", 1}, {"space_count", 0}, {"temperature_controller_count", 0},
                 {"comfort_controller_count", 0}, {"one_time_flag_explicitly_bypassed", true},
                 {"unrelated_caller_resets", "source-only-unpaired"}}},
            {"original_parser_admission_claimed", false}, {"physics_executed", false}};
}
} // namespace
int main(int argc, char const *argv[])
{
    try {
        require(argc == 2, "Usage: zon01_reference_helper <input-only-request.json>");
        std::ifstream stream(argv[1]);
        require(stream.is_open(), "Cannot read helper request");
        auto request = json::parse(stream);
        require(request.at("schema") == "zon01-helper-cases.v1" && request.at("expected_values_supplied") == false, "Wrong input-only request schema");
        json rows = json::array();
        for (auto const &item : request.at("sequences")) rows.push_back(evaluate(item));
        std::cout << json({{"schema", "zon01-helper-results.v1"}, {"sequences", rows},
                         {"expected_values_supplied", false}, {"original_parser_admission_claimed", false},
                         {"physics_executed", false}, {"gates_updated", false}}).dump() << '\n';
        return 0;
    } catch (std::exception const &error) {
        std::cerr << error.what() << '\n';
        return 2;
    }
}
