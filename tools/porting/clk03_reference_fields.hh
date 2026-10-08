// Typed observations and input-only canary storage; no weather algorithms.
#ifndef CLK03_REFERENCE_FIELDS_HH
#define CLK03_REFERENCE_FIELDS_HH
#include "clk02_reference_fields.hh"
#include <EnergyPlus/DataGlobals.hh>
#include <algorithm>
#include <limits>

#define CLK03_DAY_INTS(X) X(DayOfYear) X(DayOfYear_Schedule) X(Year) X(Month) X(DayOfMonth) X(DayOfWeek) X(DaylightSavingIndex) X(HolidayIndex)
#define CLK03_DAY_REALS(X) X(SinSolarDeclinAngle) X(CosSolarDeclinAngle) X(EquationOfTime)
#define CLK03_WEATHER_BOOLS(X) X(IsRain) X(IsSnow)
#define CLK03_WEATHER_REALS(X) X(OutDryBulbTemp) X(OutDewPointTemp) X(OutBaroPress) X(OutRelHum) X(WindSpeed) X(WindDir) X(SkyTemp) X(HorizIRSky) X(BeamSolarRad) X(DifSolarRad) X(Albedo) X(WaterPrecip) X(LiquidPrecip) X(TotalSkyCover) X(OpaqueSkyCover)
#define CLK03_MISSING_REALS(X) X(Visibility) X(Ceiling) X(AerOptDepth) X(SnowDepth)
#define CLK03_COUNTS(X) X(OutDryBulbTemp) X(OutDewPointTemp) X(OutRelHum) X(OutBaroPress) X(WindDir) X(WindSpeed) X(BeamSolarRad) X(DifSolarRad) X(TotalSkyCover) X(OpaqueSkyCover) X(Visibility) X(Ceiling) X(LiquidPrecip) X(WaterPrecip) X(AerOptDepth) X(SnowDepth) X(DaysLastSnow) X(WeathCodes) X(Albedo)
#define CLK03_GLOBAL_BOOLS(X) X(BeginSimFlag) X(BeginEnvrnFlag) X(BeginDayFlag) X(BeginHourFlag) X(BeginTimeStepFlag) X(EndDayFlag) X(EndHourFlag) X(EndEnvrnFlag) X(EndDesignDayEnvrnsFlag) X(WarmupFlag)
#define CLK03_GLOBAL_INTS(X) X(DayOfSim) X(CalendarYear) X(PreviousHour) X(HourOfDay) X(NumOfDayInEnvrn) X(TimeStepsInHour) X(TimeStep)
#define CLK03_ENV_INTS(X) X(DayOfYear) X(DayOfYear_Schedule) X(Year) X(Month) X(DayOfMonth) X(DayOfWeek) X(HolidayIndex) X(DSTIndicator) X(YearTomorrow) X(MonthTomorrow) X(DayOfMonthTomorrow) X(DayOfWeekTomorrow) X(HolidayIndexTomorrow)

namespace Clk03 {
using Clk02::json;
using Clk02::scalar;
inline void require(bool value, std::string const &message) { if (!value) throw std::runtime_error(message); }
inline int integer(json const &value)
{
    require(value.is_number_integer() && !value.is_boolean(), "Typed integer required");
    auto const wide = value.get<std::int64_t>();
    require(wide >= std::numeric_limits<int>::min() && wide <= std::numeric_limits<int>::max(), "Integer outside native int");
    return static_cast<int>(wide);
}
inline bool boolean(json const &value) { require(value.is_boolean(), "Typed boolean required"); return value.get<bool>(); }
inline double input_real(json const &value)
{
    require(value.is_object() && value.size() == 1 && value.at("bits").is_string(), "Input-only binary64 bits required");
    auto const token = value.at("bits").get<std::string>();
    require(token.size() == 16 && std::all_of(token.begin(), token.end(), [](char c) {
        return (c >= '0' && c <= '9') || (c >= 'a' && c <= 'f'); }), "Sixteen lower-case IEEE hex digits required");
    auto const result = std::bit_cast<double>(static_cast<std::uint64_t>(std::stoull(token, nullptr, 16)));
    require(std::isfinite(result) && Clk02::bits(result) == token, "Finite exact input bits required");
    return result;
}

inline json day(EnergyPlus::Weather::DayWeatherVariables const &value)
{
    json result = json::object();
#define OBS_INT(name) result[#name] = value.name;
    CLK03_DAY_INTS(OBS_INT)
#undef OBS_INT
#define OBS_REAL(name) result[#name] = scalar(value.name);
    CLK03_DAY_REALS(OBS_REAL)
#undef OBS_REAL
    return result;
}
inline json weather(EnergyPlus::Weather::WeatherVars const &value)
{
    json result = json::object();
#define OBS_BOOL(name) result[#name] = value.name;
    CLK03_WEATHER_BOOLS(OBS_BOOL)
#undef OBS_BOOL
#define OBS_REAL(name) result[#name] = scalar(value.name);
    CLK03_WEATHER_REALS(OBS_REAL)
#undef OBS_REAL
    return result;
}
inline json missing(EnergyPlus::Weather::ExtWeatherVars const &value)
{
    json result = weather(value);
#define OBS_REAL(name) result[#name] = scalar(value.name);
    CLK03_MISSING_REALS(OBS_REAL)
#undef OBS_REAL
    result["DaysLastSnow"] = value.DaysLastSnow;
    return result;
}
inline json counts(EnergyPlus::Weather::WeatherVarCounts const &value)
{
    json result = json::object();
#define OBS_INT(name) result[#name] = value.name;
    CLK03_COUNTS(OBS_INT)
#undef OBS_INT
    return result;
}
template<class Array> json integers(Array const &value)
{
    json result = json::array();
    for (auto const &item : value) result.push_back(item);
    return result;
}
template<class Array> json reals(Array const &value)
{
    json result = json::array();
    for (auto const &item : value) result.push_back(scalar(item));
    return result;
}
inline json slots(ObjexxFCL::Array2D<EnergyPlus::Weather::WeatherVars> const &value)
{
    json result = json::array();
    if (value.allocated()) for (int hour = 1; hour <= value.size2(); ++hour) for (int step = 1; step <= value.size1(); ++step)
        result.push_back({{"hour", hour}, {"time_step", step}, {"value", weather(value(step, hour))}});
    return {{"allocated", value.allocated()}, {"time_steps", value.allocated() ? value.size1() : 0},
            {"hours", value.allocated() ? value.size2() : 0}, {"ordering", "hour-major,timestep-minor"}, {"slots", result}};
}
inline json source_environment(EnergyPlus::Weather::EnvironmentData const &value)
{
    json result = {{"Title", value.Title}, {"cKindOfEnvrn", value.cKindOfEnvrn}, {"KindOfEnvrn", static_cast<int>(value.KindOfEnvrn)},
                   {"skyTempModel", static_cast<int>(value.skyTempModel)}, {"MonWeekDay", integers(value.MonWeekDay)}};
#define OBS_INT(name) result[#name] = value.name;
    OBS_INT(DesignDayNum) OBS_INT(RunPeriodDesignNum) OBS_INT(SeedEnvrnNum) OBS_INT(HVACSizingIterationNum)
    OBS_INT(TotalDays) OBS_INT(StartJDay) OBS_INT(StartMonth) OBS_INT(StartDay) OBS_INT(StartYear) OBS_INT(StartDate)
    OBS_INT(EndMonth) OBS_INT(EndDay) OBS_INT(EndJDay) OBS_INT(EndYear) OBS_INT(EndDate) OBS_INT(DayOfWeek)
    OBS_INT(NumSimYears) OBS_INT(CurrentCycle) OBS_INT(WP_Type1) OBS_INT(CurrentYear) OBS_INT(RawSimDays)
#undef OBS_INT
#define OBS_BOOL(name) result[#name] = value.name;
    OBS_BOOL(UseDST) OBS_BOOL(UseHolidays) OBS_BOOL(ApplyWeekendRule) OBS_BOOL(UseRain) OBS_BOOL(UseSnow)
    OBS_BOOL(SetWeekDays) OBS_BOOL(UseWeatherFileHorizontalIR) OBS_BOOL(IsLeapYear) OBS_BOOL(RollDayTypeOnRepeat)
    OBS_BOOL(TreatYearsAsConsecutive) OBS_BOOL(MatchYear) OBS_BOOL(ActualWeather) OBS_BOOL(firstHrInterpUseHr1)
#undef OBS_BOOL
    return result;
}
inline json snapshot(EnergyPlus::EnergyPlusData const &state)
{
    auto const &global = *state.dataGlobal;
    auto const &environment = *state.dataEnvrn;
    auto const &owner = *state.dataWeather;
    json globals = json::object(), env = json::object(), weather_owner = json::object(), environments = json::array();
#define OBS_GBOOL(name) globals[#name] = global.name;
    CLK03_GLOBAL_BOOLS(OBS_GBOOL)
#undef OBS_GBOOL
#define OBS_GINT(name) globals[#name] = global.name;
    CLK03_GLOBAL_INTS(OBS_GINT)
#undef OBS_GINT
    globals["KindOfSim"] = static_cast<int>(global.KindOfSim);
    globals["DoWeathSim"] = global.DoWeathSim;
    globals["DoDesDaySim"] = global.DoDesDaySim;
    globals["DoOutputReporting"] = global.DoOutputReporting;
    globals["TimeStepZone"] = scalar(global.TimeStepZone);
    globals["DayOfSimChr"] = global.DayOfSimChr;
    globals["CalendarYearChr"] = global.CalendarYearChr;
#define OBS_EINT(name) env[#name] = environment.name;
    CLK03_ENV_INTS(OBS_EINT)
    OBS_EINT(TotDesDays) OBS_EINT(CurEnvirNum) OBS_EINT(DayOfYearStart) OBS_EINT(RunPeriodStartDayOfWeek)
#undef OBS_EINT
#define OBS_EREAL(name) env[#name] = scalar(environment.name);
    CLK03_DAY_REALS(OBS_EREAL)
    OBS_EREAL(Latitude) OBS_EREAL(Longitude) OBS_EREAL(Elevation) OBS_EREAL(StdBaroPress)
#undef OBS_EREAL
    env["EnvironmentName"] = environment.EnvironmentName;
    env["EndMonthFlag"] = environment.EndMonthFlag;
    env["EndYearFlag"] = environment.EndYearFlag;
#define OBS_WINT(name) weather_owner[#name] = owner.name;
    OBS_WINT(Envrn) OBS_WINT(NumOfEnvrn) OBS_WINT(TotRunPers) OBS_WINT(TotRunDesPers) OBS_WINT(NumDataPeriods)
    OBS_WINT(NumIntervalsPerHour) OBS_WINT(NumSpecialDays) OBS_WINT(LeapYearAdd) OBS_WINT(RptDayType)
    OBS_WINT(CurDayOfWeek) OBS_WINT(curSimDayForEndOfRunPeriod)
#undef OBS_WINT
#define OBS_WBOOL(name) weather_owner[#name] = owner.name;
    OBS_WBOOL(GetBranchInputOneTimeFlag) OBS_WBOOL(GetEnvironmentFirstCall) OBS_WBOOL(FirstCall)
    OBS_WBOOL(WaterMainsParameterReport) OBS_WBOOL(LastHourSet) OBS_WBOOL(WeatherFileExists)
    OBS_WBOOL(DatesShouldBeReset) OBS_WBOOL(StartDatesCycleShouldBeReset) OBS_WBOOL(Jan1DatesShouldBeReset)
    OBS_WBOOL(RPReadAllWeatherData) OBS_WBOOL(UseDaylightSaving) OBS_WBOOL(UseSpecialDays) OBS_WBOOL(DaylightSavingIsActive)
#undef OBS_WBOOL
    weather_owner["ReadEPlusWeatherCurTime"] = scalar(owner.ReadEPlusWeatherCurTime);
    weather_owner["TimeStepFraction"] = scalar(owner.TimeStepFraction);
    weather_owner["IsRainThreshold"] = scalar(owner.IsRainThreshold);
    if (owner.Environment.allocated()) for (auto const &value : owner.Environment) environments.push_back(source_environment(value));
    return {{"global", globals}, {"environment", env}, {"weather", weather_owner},
            {"today_variables", day(owner.TodayVariables)}, {"tomorrow_variables", day(owner.TomorrowVariables)},
            {"today_values", slots(owner.wvarsHrTsToday)}, {"tomorrow_values", slots(owner.wvarsHrTsTomorrow)},
            {"last_hour", weather(owner.wvarsLastHr)}, {"next_hour", weather(owner.wvarsNextHr)},
            {"missing_values", missing(owner.wvarsMissing)}, {"missed_counts", counts(owner.wvarsMissedCounts)},
            {"out_of_range_counts", counts(owner.wvarsOutOfRangeCounts)}, {"environments", environments},
            {"Environment_allocated", owner.Environment.allocated()}, {"header", Clk02::header_state(state)},
            {"stream", Clk02::stream_state(state.files.inputWeatherFile)}, {"DSTIndex", integers(owner.DSTIndex)},
            {"SpecialDayTypes", integers(owner.SpecialDayTypes)}, {"WeekDayTypes", integers(owner.WeekDayTypes)},
            {"Interpolation_allocated", owner.Interpolation.allocated()}, {"Interpolation", reals(owner.Interpolation)},
            {"SolarInterpolation_allocated", owner.SolarInterpolation.allocated()}, {"SolarInterpolation", reals(owner.SolarInterpolation)},
            {"native_internal_record_index_observed", false}, {"native_source_local_variables_observed", false}};
}

inline void seed_day(EnergyPlus::Weather::DayWeatherVariables &value, json const &input)
{
    require(input.is_object(), "Day canary must be object");
    std::size_t applied = 0;
#define SET_INT(name) if (input.contains(#name)) { value.name = integer(input.at(#name)); ++applied; }
    CLK03_DAY_INTS(SET_INT)
#undef SET_INT
#define SET_REAL(name) if (input.contains(#name)) { value.name = input_real(input.at(#name)); ++applied; }
    CLK03_DAY_REALS(SET_REAL)
#undef SET_REAL
    require(applied == input.size(), "Unknown day canary field");
}
inline void seed_weather(EnergyPlus::Weather::WeatherVars &value, json const &input, bool allow_missing = false)
{
    require(input.is_object(), "Weather canary must be object");
    std::size_t applied = 0;
#define SET_BOOL(name) if (input.contains(#name)) { value.name = boolean(input.at(#name)); ++applied; }
    CLK03_WEATHER_BOOLS(SET_BOOL)
#undef SET_BOOL
#define SET_REAL(name) if (input.contains(#name)) { value.name = input_real(input.at(#name)); ++applied; }
    CLK03_WEATHER_REALS(SET_REAL)
#undef SET_REAL
    if (allow_missing) for (char const *name : {"Visibility", "Ceiling", "AerOptDepth", "SnowDepth", "DaysLastSnow"})
        if (input.contains(name)) ++applied;
    require(applied == input.size(), "Unknown WeatherVars canary field");
}
inline void seed_counts(EnergyPlus::Weather::WeatherVarCounts &value, json const &input)
{
    require(input.is_object(), "Count canary must be object");
    std::size_t applied = 0;
#define SET_INT(name) if (input.contains(#name)) { value.name = integer(input.at(#name)); ++applied; }
    CLK03_COUNTS(SET_INT)
#undef SET_INT
    require(applied == input.size(), "Unknown count canary field");
}
inline void seed_slots(ObjexxFCL::Array2D<EnergyPlus::Weather::WeatherVars> &value, json const &input)
{
    require(value.allocated() && value.size1() == 4 && value.size2() == 24 && input.is_array() && input.size() == 96,
            "Declared whole4x24 input carrier required");
    for (int hour = 1; hour <= 24; ++hour) for (int step = 1; step <= 4; ++step)
        seed_weather(value(step, hour), input.at(static_cast<std::size_t>((hour - 1) * 4 + step - 1)));
}
inline void seed(EnergyPlus::EnergyPlusData &state, json const &input)
{
    require(input.is_object(), "Caller partial state must be object");
    auto &owner = *state.dataWeather;
    std::size_t groups = 0;
    if (input.contains("global")) {
        auto const &part = input.at("global"); auto &value = *state.dataGlobal; std::size_t applied = 0;
        require(part.is_object(), "Global canary object required");
#define SET_BOOL(name) if (part.contains(#name)) { value.name = boolean(part.at(#name)); ++applied; }
        CLK03_GLOBAL_BOOLS(SET_BOOL)
#undef SET_BOOL
#define SET_INT(name) if (part.contains(#name)) { value.name = integer(part.at(#name)); ++applied; }
        CLK03_GLOBAL_INTS(SET_INT)
#undef SET_INT
        require(applied == part.size(), "Unknown global caller field"); ++groups;
    }
    if (input.contains("environment")) {
        auto const &part = input.at("environment"); auto &value = *state.dataEnvrn; std::size_t applied = 0;
        require(part.is_object(), "Environment canary object required");
#define SET_INT(name) if (part.contains(#name)) { value.name = integer(part.at(#name)); ++applied; }
        CLK03_ENV_INTS(SET_INT)
#undef SET_INT
#define SET_REAL(name) if (part.contains(#name)) { value.name = input_real(part.at(#name)); ++applied; }
        CLK03_DAY_REALS(SET_REAL)
#undef SET_REAL
        require(applied == part.size(), "Unknown environment caller field"); ++groups;
    }
    if (input.contains("weather")) {
        auto const &part = input.at("weather"); std::size_t applied = 0;
        require(part.is_object(), "Weather owner canary object required");
#define SET_INT(name) if (part.contains(#name)) { owner.name = integer(part.at(#name)); ++applied; }
        SET_INT(RptDayType) SET_INT(CurDayOfWeek)
#undef SET_INT
        if (part.contains("LastHourSet")) { owner.LastHourSet = boolean(part.at("LastHourSet")); ++applied; }
        if (part.contains("ReadEPlusWeatherCurTime")) { owner.ReadEPlusWeatherCurTime = input_real(part.at("ReadEPlusWeatherCurTime")); ++applied; }
        require(applied == part.size(), "Unknown weather owner caller field"); ++groups;
    }
#define SET_GROUP(key, member, function) if (input.contains(key)) { function(owner.member, input.at(key)); ++groups; }
    SET_GROUP("today_variables", TodayVariables, seed_day) SET_GROUP("tomorrow_variables", TomorrowVariables, seed_day)
    SET_GROUP("today_values", wvarsHrTsToday, seed_slots) SET_GROUP("tomorrow_values", wvarsHrTsTomorrow, seed_slots)
    SET_GROUP("last_hour", wvarsLastHr, seed_weather) SET_GROUP("next_hour", wvarsNextHr, seed_weather)
    SET_GROUP("missed_counts", wvarsMissedCounts, seed_counts) SET_GROUP("out_of_range_counts", wvarsOutOfRangeCounts, seed_counts)
#undef SET_GROUP
    if (input.contains("missing_values")) {
        auto const &part = input.at("missing_values"); seed_weather(owner.wvarsMissing, part, true);
#define SET_REAL(name) if (part.contains(#name)) owner.wvarsMissing.name = input_real(part.at(#name));
        CLK03_MISSING_REALS(SET_REAL)
#undef SET_REAL
        if (part.contains("DaysLastSnow")) owner.wvarsMissing.DaysLastSnow = integer(part.at("DaysLastSnow"));
        ++groups;
    }
    require(groups == input.size(), "Unknown caller state group");
}
} // namespace Clk03
#endif
