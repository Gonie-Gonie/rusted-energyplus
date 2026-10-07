// Test-only dispatcher: calculations are linked original EnergyPlus bodies.
// The pure-only fallback includes exact pinned bodies for pure functions only.
// Private state is constructed by this executable's matching GNU static core;
// never cast an opaque state from the separately installed MSVC API DLL.
#include <EnergyPlus/General.hh>
#include <EnergyPlus/WeatherManager.hh>
#include <ObjexxFCL/Fmath.hh>
#include <nlohmann/json.hpp>

#ifdef CLK01_PURE_ONLY
#include "clk01_original_pure.inc"
#else
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataEnvironment.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataStringGlobals.hh>
#include <EnergyPlus/InputProcessing/InputProcessor.hh>
#include <EnergyPlus/IOFiles.hh>
#endif

#include <fstream>
#include <iostream>
#include <map>
#include <memory>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

using json = nlohmann::json;
using namespace EnergyPlus;

// The original external body is not declared in WeatherManager.hh.
namespace EnergyPlus::Weather {
bool validMonthDay(int month, int day, int leapYearAdd);
}

json resolve_inputs(json const &value, std::map<int, json> const &previous)
{
    if (value.is_object() && value.contains("from_call")) {
        if (value.size() != 1) throw std::runtime_error("Invalid from_call object");
        auto it = previous.find(value.at("from_call").get<int>());
        if (it == previous.end() || !(it->second.is_number_integer() || it->second.is_boolean())) {
            throw std::runtime_error("from_call must reference a preceding scalar original result");
        }
        return it->second;
    }
    if (value.is_object()) {
        json out = json::object();
        for (auto const &item : value.items()) out[item.key()] = resolve_inputs(item.value(), previous);
        return out;
    }
    if (value.is_array()) {
        json out = json::array();
        for (auto const &item : value) out.push_back(resolve_inputs(item, previous));
        return out;
    }
    return value;
}

#ifndef CLK01_PURE_ONLY
json array_values(ObjexxFCL::Array1D_int const &values)
{
    json out = json::array();
    for (int i = values.l(); i <= values.u(); ++i) out.push_back(values(i));
    return out;
}

json weather_input_state(EnergyPlus::EnergyPlusData const &state)
{
    return {{"day_of_week", state.dataEnvrn->DayOfWeek},
            {"leap_year_add", state.dataWeather->LeapYearAdd},
            {"end_day_of_month", array_values(state.dataWeather->EndDayOfMonth)},
            {"end_day_of_month_with_leap_day", array_values(state.dataWeather->EndDayOfMonthWithLeapDay)},
            {"environment_counter", state.dataWeather->Envrn},
            {"total_design_day_definitions", state.dataEnvrn->TotDesDays},
            {"total_weather_run_periods", state.dataWeather->TotRunPers},
            {"total_weather_sizing_periods", state.dataWeather->TotRunDesPers},
            {"weather_file_allows_leap_years", state.dataWeather->WFAllowsLeapYears}};
}
#endif

json run_helpers(json const &request)
{
    if (request.at("schema") != "clk01-helper-tuples.v1") throw std::runtime_error("Invalid helper request schema");
    json results = json::array();
    std::map<int, json> previous;
#ifndef CLK01_PURE_ONLY
    EnergyPlus::EnergyPlusData state;
    state.init_constant_state(state);
#endif
    for (auto const &call : request.at("calls")) {
        int index = call.at("call_index").get<int>();
        if (previous.count(index)) throw std::runtime_error("Duplicate call_index");
        json out = call;
        auto inputs = resolve_inputs(call.at("inputs"), previous);
        out["resolved_inputs"] = inputs;
        auto routine = call.at("function").get<std::string>();
        json value;
        if (routine == "isLeapYear") {
            value = Weather::isLeapYear(inputs.at("year").get<int>());
        } else if (routine == "calculateDayOfYear") {
            value = Weather::calculateDayOfYear(inputs.at("month").get<int>(), inputs.at("day").get<int>(),
                                                inputs.at("leap_year").get<bool>());
        } else if (routine == "General::OrdinalDay") {
            value = General::OrdinalDay(inputs.at("month").get<int>(), inputs.at("day").get<int>(),
                                        inputs.at("leap_year_add").get<int>());
        } else if (routine == "computeJulianDate") {
            value = Weather::computeJulianDate(inputs.at("year").get<int>(), inputs.at("month").get<int>(),
                                               inputs.at("day").get<int>());
        } else if (routine == "computeGregorianDate") {
            auto date = Weather::computeGregorianDate(inputs.at("julian_date").get<int>());
            value = {{"year", date.year}, {"month", date.month}, {"day", date.day}};
        } else if (routine == "validMonthDay") {
            value = Weather::validMonthDay(inputs.at("month").get<int>(), inputs.at("day").get<int>(),
                                           inputs.at("leap_year_add").get<int>());
        } else if (routine == "calculateDayOfWeek") {
#ifdef CLK01_PURE_ONLY
            out["supported"] = false;
            out["reason"] = "Requires original EnergyPlusData; unavailable in pure fallback";
#else
            state.dataEnvrn->DayOfWeek = inputs.at("day_of_week_before").get<int>();
            out["state_before"] = {{"day_of_week", state.dataEnvrn->DayOfWeek}};
            value = static_cast<int>(Weather::calculateDayOfWeek(state, inputs.at("year").get<int>(),
                                                                 inputs.at("month").get<int>(), inputs.at("day").get<int>()));
            out["state_after"] = {{"day_of_week", state.dataEnvrn->DayOfWeek}};
#endif
        } else if (routine == "SetupWeekDaysByMonth") {
#ifdef CLK01_PURE_ONLY
            out["supported"] = false;
            out["reason"] = "Requires original EnergyPlusData; unavailable in pure fallback";
#else
            auto end_days = inputs.at("end_day_of_month").get<std::vector<int>>();
            auto before = inputs.at("week_days_before").get<std::vector<int>>();
            if (end_days.size() != 12 || before.size() != 12) throw std::runtime_error("Month arrays must have length 12");
            ObjexxFCL::Array1D_int week_days(12, 0);
            for (int month = 1; month <= 12; ++month) {
                state.dataWeather->EndDayOfMonth(month) = end_days[static_cast<std::size_t>(month - 1)];
                week_days(month) = before[static_cast<std::size_t>(month - 1)];
            }
            state.dataWeather->LeapYearAdd = inputs.at("leap_year_add").get<int>();
            out["state_before"] = {{"end_day_of_month", array_values(state.dataWeather->EndDayOfMonth)},
                                     {"leap_year_add", state.dataWeather->LeapYearAdd}, {"week_days", array_values(week_days)}};
            Weather::SetupWeekDaysByMonth(state, inputs.at("start_month").get<int>(), inputs.at("start_day").get<int>(),
                                          inputs.at("start_weekday").get<int>(), week_days);
            value = array_values(week_days);
            out["state_after"] = {{"end_day_of_month", array_values(state.dataWeather->EndDayOfMonth)},
                                    {"leap_year_add", state.dataWeather->LeapYearAdd}, {"week_days", value}};
#endif
        } else {
            throw std::runtime_error("Unknown helper routine: " + routine);
        }
        if (!out.contains("supported")) {
            out["supported"] = true;
            out["value"] = value;
        }
        previous[index] = value;
        results.push_back(out);
    }
    return {{"schema", "clk01-helper-results.v1"}, {"calls", results},
#ifdef CLK01_PURE_ONLY
            {"reference_kind", "pinned-original-pure-body-fallback"}};
#else
            {"reference_kind", "linked-original-source-with-genuine-state"}};
#endif
}

#ifndef CLK01_PURE_ONLY
json run_calendar_case(json const &item)
{
    // processInput uses the original IDF parser, validation, maps and short-cut
    // buffers. No unit-test common object/default injection is used here.
    EnergyPlus::EnergyPlusData state;
    state.init_constant_state(state);
    auto error_stream = std::make_unique<std::ostringstream>();
    auto *error_text = error_stream.get();
    state.files.err_stream = std::move(error_stream);
    state.dataStrGlobals->inputFilePath = item.at("source_metadata").at("input").at("path").get<std::string>();
    state.dataInputProcessing->inputProcessor->processInput(state);
    auto &ip = state.dataInputProcessing->inputProcessor;
    int count = ip->getNumObjectsFound(state, "RunPeriod");
    if (count != 1) throw std::runtime_error("CLK-01 selected contract requires one RunPeriod");
    json before_input = weather_input_state(state);
    bool errors = false;
    Weather::GetRunPeriodData(state, count, errors);
    json after_input = weather_input_state(state);
    if (errors) throw std::runtime_error("Original GetRunPeriodData rejected selected input: " + error_text->str());
    auto const &rp = state.dataWeather->RunPeriodInput(1);
    if (rp.startYear != 2013 || rp.endYear != 2013 || rp.actualWeather) {
        throw std::runtime_error("Calendar production projection is restricted to frozen ordinary 2013 cases");
    }
    // Explicit unit-call prerequisites for the original SetupEnvironmentTypes.
    // Counts are obtained from the original parser; inactive design definitions
    // are counted but no sizing/day-weather payload is fabricated or executed.
    state.dataEnvrn->TotDesDays = ip->getNumObjectsFound(state, "SizingPeriod:DesignDay");
    state.dataWeather->TotRunPers = count;
    state.dataWeather->TotRunDesPers = ip->getNumObjectsFound(state, "SizingPeriod:WeatherFileDays") +
                                       ip->getNumObjectsFound(state, "SizingPeriod:WeatherFileConditionType");
    if (state.dataWeather->TotRunDesPers != 0) throw std::runtime_error("Weather sizing periods are outside frozen CON scope");
    state.dataWeather->WFAllowsLeapYears = item.at("weather_calendar").at("leap_year_observed").get<bool>();
    state.dataWeather->NumOfEnvrn = state.dataEnvrn->TotDesDays + count;
    state.dataWeather->Environment.allocate(state.dataWeather->NumOfEnvrn);
    json before_environment = weather_input_state(state);
    Weather::SetupEnvironmentTypes(state);
    json after_environment = weather_input_state(state);
    int selected = state.dataEnvrn->TotDesDays + 1;
    auto const &env = state.dataWeather->Environment(selected);
    json resolved = {{"run_period_name", rp.title}, {"start_year", rp.startYear}, {"start_month", rp.startMonth},
                     {"start_day_of_month", rp.startDay}, {"start_day_of_week", rp.dayOfWeek}, {"start_year_is_leap_year", rp.isLeapYear},
                     {"end_year", rp.endYear}, {"end_month", rp.endMonth}, {"end_day_of_month", rp.endDay},
                     {"end_year_is_leap_year", Weather::isLeapYear(rp.endYear)}, {"total_days", env.TotalDays},
                     {"start_julian_date", rp.startJulianDate}, {"end_julian_date", rp.endJulianDate},
                     {"monthly_start_weekdays", array_values(rp.monWeekDay)}};
    json environment = {{"selected_source_environment_ordinal", selected}, {"kind_of_sim", static_cast<int>(env.KindOfEnvrn)},
                        {"start_weather_ordinal", env.StartJDay}, {"end_weather_ordinal", env.EndJDay},
                        {"raw_sim_days", env.RawSimDays}, {"total_days", env.TotalDays}, {"current_year", env.CurrentYear},
                        {"weather_effective_year_is_leap_year", env.IsLeapYear}, {"start_day_of_week", env.DayOfWeek},
                        {"monthly_start_weekdays", array_values(env.MonWeekDay)}, {"use_dst", env.UseDST},
                        {"use_holidays", env.UseHolidays}, {"apply_weekend_rule", env.ApplyWeekendRule},
                        {"set_weekdays", env.SetWeekDays}, {"treat_years_as_consecutive", env.TreatYearsAsConsecutive},
                        {"actual_weather", env.ActualWeather}, {"match_year", env.MatchYear}};
    json days = json::array();
    // Each date and weekday is calculated by original linked helpers. This is
    // a consumed calendar projection, not the CLK-03 weather/prefetch producer.
    for (int jd = rp.startJulianDate; jd <= rp.endJulianDate; ++jd) {
        auto date = Weather::computeGregorianDate(jd);
        auto dow = Weather::calculateDayOfWeek(state, date.year, date.month, date.day);
        auto gregorian_leap = Weather::isLeapYear(date.year);
        days.push_back({{"year", date.year}, {"month", date.month}, {"day_of_month", date.day},
                        {"gregorian_day_of_year", Weather::calculateDayOfYear(date.month, date.day, gregorian_leap)},
                        {"weather_day_of_year", General::OrdinalDay(date.month, date.day, static_cast<int>(env.IsLeapYear))},
                        {"schedule_day_of_year", General::OrdinalDay(date.month, date.day, 1)},
                        {"gregorian_day_of_week", static_cast<int>(dow)}, {"day_of_week", static_cast<int>(dow)},
                        {"day_type", static_cast<int>(dow)}, {"day_type_name", Sched::dayTypeNames[static_cast<int>(dow)]},
                        {"gregorian_year_is_leap_year", gregorian_leap}, {"weather_effective_year_is_leap_year", env.IsLeapYear},
                        {"leap_year_add", static_cast<int>(env.IsLeapYear)}});
    }
    return {{"case_id", item.at("case_id")}, {"source_metadata", item.at("source_metadata")},
            {"resolved_calendar", resolved}, {"environment", environment}, {"days", days},
            {"state_before_get_run_period", before_input}, {"state_after_get_run_period", after_input},
            {"state_before_setup_environment", before_environment}, {"state_after_setup_environment", after_environment},
            {"original_messages", error_text->str()},
            {"calendar_projection_policy", "No weather reads, Today/Tomorrow, schedule values, warmup, SYS or reporting simulation is executed"}};
}

json run_calendar(json const &request)
{
    if (request.at("schema") != "clk01-calendar-cases.v1") throw std::runtime_error("Invalid calendar request schema");
    json results = json::array();
    for (auto const &item : request.at("cases")) results.push_back(run_calendar_case(item));
    return {{"schema", "clk01-calendar-results.v1"}, {"reference_kind", "linked-original-inputprocessor-and-genuine-state"}, {"cases", results}};
}
#endif

int main(int argc, char **argv)
{
    if (argc != 3) {
        std::cerr << "Usage: clk01_reference --helpers|--calendar INPUT.json\n";
        return 2;
    }
    try {
        std::ifstream input(argv[2]);
        if (!input) throw std::runtime_error("Cannot open request file");
        json request;
        input >> request;
        json results;
        std::ostringstream original_stdout;
        auto *saved = std::cout.rdbuf(original_stdout.rdbuf());
        try {
            if (std::string(argv[1]) == "--helpers") results = run_helpers(request);
            else if (std::string(argv[1]) == "--calendar") {
#ifdef CLK01_PURE_ONLY
                throw std::runtime_error("Calendar/state calls require the matching original-source GNU core");
#else
                results = run_calendar(request);
#endif
            } else throw std::runtime_error("Unknown request mode");
        } catch (...) {
            std::cout.rdbuf(saved);
            throw;
        }
        std::cout.rdbuf(saved);
        results["original_stdout"] = original_stdout.str();
        std::cout << results.dump(2) << '\n';
        return 0;
    } catch (std::exception const &error) {
        std::cerr << error.what() << '\n';
        return 2;
    }
}
