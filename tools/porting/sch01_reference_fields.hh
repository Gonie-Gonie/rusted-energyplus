// Passive genuine ScheduleManager storage; no normalization or schedule arithmetic.
#ifndef SCH01_REFERENCE_FIELDS_HH
#define SCH01_REFERENCE_FIELDS_HH
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataErrorTracking.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/IOFiles.hh>
#include <EnergyPlus/InputProcessing/InputProcessor.hh>
#include <EnergyPlus/ScheduleManager.hh>
#include <nlohmann/json.hpp>
#include <algorithm>
#include <bit>
#include <cmath>
#include <cstdint>
#include <iomanip>
#include <iterator>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

#if !defined(__MINGW32__) || !defined(_WIN64) || __GNUC__ != 13 || __GNUC_MINOR__ != 2
#error "Schedule reference requires the verified GCC13.2 Windows ABI"
#endif
#if defined(_GLIBCXX_DEBUG) || defined(__FAST_MATH__) || defined(NDEBUG)
#error "Reference requires original assertions, nondebug containers and no fast math"
#endif
#if !defined(EP_psych_errors) || defined(EP_psych_stats)
#error "Reference definitions must match the genuine original core"
#endif

namespace Sch01 {
using json = nlohmann::json;
namespace Sched = EnergyPlus::Sched;

inline void require(bool value, std::string const &message)
{
    if (!value) throw std::runtime_error(message);
}
inline std::string bits(double value)
{
    std::ostringstream out;
    out << std::hex << std::setfill('0') << std::setw(16) << std::bit_cast<std::uint64_t>(value);
    return out.str();
}
inline json scalar(double value)
{
    char const *kind = std::isnan(value) ? "nan" : std::isinf(value) ? (value > 0 ? "positive_infinity" : "negative_infinity") :
                       value == 0 ? (std::signbit(value) ? "negative_zero" : "positive_zero") : "finite";
    return {{"value", std::isfinite(value) ? json(value) : json(nullptr)}, {"value_bits", bits(value)}, {"value_class", kind}};
}
inline json reals(std::vector<Real64> const &values)
{
    json result = json::array();
    for (auto value : values) result.push_back(scalar(value));
    return result;
}
inline json base(Sched::ScheduleBase const &value)
{
    return {{"Name", value.Name}, {"Num", value.Num}, {"isUsed", value.isUsed},
            {"minVal", scalar(value.minVal)}, {"maxVal", scalar(value.maxVal)}, {"isMinMaxSet", value.isMinMaxSet}};
}
template<class T> inline json pointer(T const *value, std::vector<T *> const &owners)
{
    if (value == nullptr) return nullptr;
    auto const found = std::find(owners.begin(), owners.end(), value);
    require(found != owners.end(), "Actual schedule pointer does not refer to its owned vector");
    return {{"Num", value->Num}, {"vector_index", std::distance(owners.begin(), found)}};
}
inline json parsed_schedule_objects(EnergyPlus::InputProcessor const &processor)
{
    json rows = json::array();
    for (auto const *family : {"ScheduleTypeLimits", "Schedule:Constant", "Schedule:Compact"}) {
        auto const objects = processor.epJSON.find(family);
        if (objects == processor.epJSON.end()) continue;
        require(objects->is_object(), "Actual parsed schedule family is not an object");
        int ordinal = 0;
        for (auto const &item : objects->items()) {
            rows.push_back({{"family", family}, {"parsed_instance_name", item.key()}, {"JSON_iteration_index", ordinal++},
                            {"idf_order", item.value().contains("idf_order") ? item.value().at("idf_order") : json(nullptr)}});
        }
    }
    return rows;
}
inline json snapshot(EnergyPlus::EnergyPlusData const &state)
{
    auto const &owner = *state.dataSched;
    auto const &global = *state.dataGlobal;
    json types = json::array(), schedules = json::array(), weeks = json::array(), days = json::array();
    for (auto const *value : owner.scheduleTypes) {
        if (value == nullptr) { types.push_back(nullptr); continue; }
        types.push_back({{"Name", value->Name}, {"Num", value->Num}, {"isLimited", value->isLimited},
                         {"minVal", scalar(value->minVal)}, {"maxVal", scalar(value->maxVal)},
                         {"isReal", value->isReal}, {"limitUnits", static_cast<int>(value->limitUnits)}});
    }
    for (auto const *value : owner.schedules) {
        if (value == nullptr) { schedules.push_back(nullptr); continue; }
        json row = base(*value);
        row.update({{"type", static_cast<int>(value->type)}, {"schedTypeNum", value->schedTypeNum},
                    {"currentVal", scalar(value->currentVal)}, {"EMSActuatedOn", value->EMSActuatedOn}, {"EMSVal", scalar(value->EMSVal)}});
        if (auto const *constant = dynamic_cast<Sched::ScheduleConstant const *>(value)) {
            row["storage_kind"] = "ScheduleConstant"; row["tsVals"] = reals(constant->tsVals);
        } else if (auto const *detailed = dynamic_cast<Sched::ScheduleDetailed const *>(value)) {
            row["storage_kind"] = "ScheduleDetailed";
            row["weekScheds_day_1_through_366"] = json::array();
            for (int day = 1; day <= 366; ++day) row["weekScheds_day_1_through_366"].push_back(pointer(detailed->weekScheds[day], owner.weekSchedules));
            row["UseDaylightSaving"] = detailed->UseDaylightSaving;
        } else require(false, "Actual top-level schedule storage is outside Constant/Compact scope");
        schedules.push_back(std::move(row));
    }
    for (auto const *value : owner.weekSchedules) {
        if (value == nullptr) { weeks.push_back(nullptr); continue; }
        json row = base(*value), references = json::array();
        for (int day = 1; day < static_cast<int>(Sched::DayType::Num); ++day) references.push_back(pointer(value->dayScheds[day], owner.daySchedules));
        row["dayScheds_real_day_types_1_through_12"] = references;
        row["unused_day_type_zero_pointer_is_null"] = value->dayScheds[0] == nullptr;
        weeks.push_back(std::move(row));
    }
    for (auto const *value : owner.daySchedules) {
        if (value == nullptr) { days.push_back(nullptr); continue; }
        json row = base(*value);
        row.update({{"schedTypeNum", value->schedTypeNum}, {"interpolation", static_cast<int>(value->interpolation)},
                    {"tsVals", reals(value->tsVals)}, {"sumTsVals", scalar(value->sumTsVals)}});
        days.push_back(std::move(row));
    }
    json day_types = json::array();
    for (int day = 1; day < static_cast<int>(Sched::DayType::Num); ++day) day_types.push_back({{"Num", day}, {"Name", std::string(Sched::dayTypeNames[day])}});
    return {{"caller", {{"TimeStepsInHour", global.TimeStepsInHour}, {"MinutesInTimeStep", global.MinutesInTimeStep},
                        {"TimeStepZone", scalar(global.TimeStepZone)}, {"isEpJSON", global.isEpJSON},
                        {"preserveIDFOrder", global.preserveIDFOrder}, {"AnyEnergyManagementSystemInModel", global.AnyEnergyManagementSystemInModel},
                        {"outputEpJSONConversion", global.outputEpJSONConversion}, {"outputEpJSONConversionOnly", global.outputEpJSONConversionOnly}}},
            {"ScheduleInputProcessed", owner.ScheduleInputProcessed}, {"scheduleTypes", types}, {"schedules", schedules},
            {"weekSchedules", weeks}, {"daySchedules", days}, {"scheduleTypeMap", owner.scheduleTypeMap},
            {"scheduleMap", owner.scheduleMap}, {"weekScheduleMap", owner.weekScheduleMap}, {"dayScheduleMap", owner.dayScheduleMap},
            {"real_day_type_enum", day_types}, {"parsed_schedule_objects", parsed_schedule_objects(*state.dataInputProcessing->inputProcessor)},
            {"TotalWarningErrors", state.dataErrTracking->TotalWarningErrors}, {"TotalSevereErrors", state.dataErrTracking->TotalSevereErrors},
            {"private_Through_Until_or_minute_buffers_observed", false}, {"state_mutating_getters_used_for_snapshot", false}};
}
} // namespace Sch01
#endif
