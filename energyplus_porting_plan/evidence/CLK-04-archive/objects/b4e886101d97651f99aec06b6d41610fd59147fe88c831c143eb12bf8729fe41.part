// Read-only declared caller outputs and defined original EPW owner fields.
// No weather/date/numeric parsing algorithm is implemented in this file.
#ifndef CLK02_REFERENCE_FIELDS_HH
#define CLK02_REFERENCE_FIELDS_HH
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataEnvironment.hh>
#include <EnergyPlus/IOFiles.hh>
#include <EnergyPlus/WeatherManager.hh>
#include <nlohmann/json.hpp>
#include <bit>
#include <cmath>
#include <cstdint>
#include <iomanip>
#include <sstream>
#include <stdexcept>

#if !defined(__MINGW32__) || !defined(_WIN64) || __GNUC__ != 13 || __GNUC_MINOR__ != 2
#error "Original EPW reference requires the verified GCC13.2 Windows ABI"
#endif
#if defined(_GLIBCXX_DEBUG) || defined(__FAST_MATH__) || defined(NDEBUG)
#error "Reference requires original assertions, nondebug containers and no fast math"
#endif
#if !defined(EP_psych_errors) || defined(EP_psych_stats)
#error "Reference definitions must match the genuine original core"
#endif

#define CLK02_DATES(X) X(WYear) X(WMonth) X(WDay) X(WHour) X(WMinute)
#define CLK02_MANDATORY_REALS(X) X(DryBulb) X(DewPoint) X(RelHum) X(AtmPress) X(ETHoriz) X(ETDirect) X(IRHoriz) X(GLBHoriz) X(DirectRad) X(DiffuseRad) X(GLBHorizIllum) X(DirectNrmIllum) X(DiffuseHorizIllum) X(ZenLum) X(WindDir) X(WindSpeed) X(TotalSkyCover) X(OpaqueSkyCover) X(Visibility) X(CeilHeight)
#define CLK02_OPTIONAL_REALS(X) X(PrecipWater) X(AerosolOptDepth) X(SnowDepth) X(DaysSinceLastSnow) X(Albedo) X(LiquidPrecip)

namespace Clk02 {
using json = nlohmann::json;

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

// This is caller storage, not a WeatherManagerData record constructor.
// Every reference argument is defined before the unchanged source is invoked.
struct RawOutputs {
    bool ErrorFound = false;
#define INT_FIELD(name) int name = 0;
    CLK02_DATES(INT_FIELD)
#undef INT_FIELD
#define REAL_FIELD(name) double name = 0.0;
    CLK02_MANDATORY_REALS(REAL_FIELD)
    CLK02_OPTIONAL_REALS(REAL_FIELD)
#undef REAL_FIELD
    int WObs = 0;
    ObjexxFCL::Array1D_int WCodesArr = ObjexxFCL::Array1D_int(9, 0);
};

inline json raw_outputs(RawOutputs const &value)
{
    json dates = json::object(), mandatory = json::object(), optional = json::object(), codes = json::array();
#define READ_INT(name) dates[#name] = value.name;
    CLK02_DATES(READ_INT)
#undef READ_INT
#define READ_REAL(name) mandatory[#name] = scalar(value.name);
    CLK02_MANDATORY_REALS(READ_REAL)
#undef READ_REAL
#define READ_REAL(name) optional[#name] = scalar(value.name);
    CLK02_OPTIONAL_REALS(READ_REAL)
#undef READ_REAL
    for (int code : value.WCodesArr) codes.push_back(code);
    return {{"ErrorFound", value.ErrorFound}, {"date_fields", dates}, {"mandatory_reals", mandatory},
            {"WObs", value.WObs}, {"weather_codes", codes}, {"optional_reals", optional},
            {"private_RField21_observed", false}};
}

inline json dst(EnergyPlus::Weather::DSTPeriod const &value)
{
    return {{"StDateType", static_cast<int>(value.StDateType)}, {"StWeekDay", value.StWeekDay},
            {"StMon", value.StMon}, {"StDay", value.StDay}, {"EnDateType", static_cast<int>(value.EnDateType)},
            {"EnMon", value.EnMon}, {"EnDay", value.EnDay}, {"EnWeekDay", value.EnWeekDay}};
}

inline json header_state(EnergyPlus::EnergyPlusData const &state)
{
    auto const &owner = *state.dataWeather;
    json periods = json::array(), special = json::array(), typical = json::array(), ground = json::array(), month_ends = json::array();
    for (auto const &value : owner.DataPeriods) {
        json weekdays = json::array();
        for (int day : value.MonWeekDay) weekdays.push_back(day);
        periods.push_back({{"Name", value.Name}, {"DayOfWeek", value.DayOfWeek}, {"NumYearsData", value.NumYearsData},
                          {"WeekDay", value.WeekDay}, {"StMon", value.StMon}, {"StDay", value.StDay}, {"StYear", value.StYear},
                          {"EnMon", value.EnMon}, {"EnDay", value.EnDay}, {"EnYear", value.EnYear}, {"NumDays", value.NumDays},
                          {"MonWeekDay", weekdays}, {"DataStJDay", value.DataStJDay}, {"DataEnJDay", value.DataEnJDay},
                          {"HasYearData", value.HasYearData}});
    }
    for (auto const &value : owner.SpecialDays) {
        special.push_back({{"Name", value.Name}, {"dateType", static_cast<int>(value.dateType)}, {"Month", value.Month},
                          {"Day", value.Day}, {"WeekDay", value.WeekDay}, {"CompDate", value.CompDate}, {"WthrFile", value.WthrFile},
                          {"Duration", value.Duration}, {"DayType", value.DayType}, {"ActStMon", value.ActStMon},
                          {"ActStDay", value.ActStDay}, {"Used", value.Used}});
    }
    // Whole-file parsing really executes these branches. Their downstream CON
    // consumers are unpaired; these are copied defined source context fields.
    for (auto const &value : owner.TypicalExtremePeriods) {
        typical.push_back({{"Title", value.Title}, {"ShortTitle", value.ShortTitle}, {"MatchValue", value.MatchValue},
                           {"MatchValue1", value.MatchValue1}, {"MatchValue2", value.MatchValue2}, {"TEType", value.TEType},
                           {"TotalDays", value.TotalDays}, {"StartJDay", value.StartJDay}, {"StartMonth", value.StartMonth},
                           {"StartDay", value.StartDay}, {"EndMonth", value.EndMonth}, {"EndDay", value.EndDay}, {"EndJDay", value.EndJDay}});
    }
    for (double value : owner.GroundTempsFCFromEPWHeader) ground.push_back(scalar(value));
    for (int day : owner.EndDayOfMonth) month_ends.push_back(day);
    return {{"EPWHeaderTitle", owner.EPWHeaderTitle}, {"WeatherFileLocationTitle", state.dataEnvrn->WeatherFileLocationTitle},
            {"WeatherFileLatitude", scalar(owner.WeatherFileLatitude)}, {"WeatherFileLongitude", scalar(owner.WeatherFileLongitude)},
            {"WeatherFileTimeZone", scalar(owner.WeatherFileTimeZone)}, {"WeatherFileElevation", scalar(owner.WeatherFileElevation)},
            {"WFAllowsLeapYears", owner.WFAllowsLeapYears}, {"EPWDaylightSaving", owner.EPWDaylightSaving},
            {"EPWDST", dst(owner.EPWDST)}, {"DST", dst(owner.DST)}, {"NumSpecialDays", owner.NumSpecialDays},
            {"SpecialDays", special}, {"SpecialDays_allocated", owner.SpecialDays.allocated()},
            {"NumDataPeriods", owner.NumDataPeriods}, {"NumIntervalsPerHour", owner.NumIntervalsPerHour},
            {"DataPeriods", periods}, {"DataPeriods_allocated", owner.DataPeriods.allocated()},
            {"LeapYearAdd", owner.LeapYearAdd}, {"EndDayOfMonth", month_ends},
            {"wvarsMissedCounts.WeathCodes", owner.wvarsMissedCounts.WeathCodes},
            {"source_only_context", {{"NumEPWTypExtSets", owner.NumEPWTypExtSets}, {"TypicalExtremePeriods", typical},
                                     {"wthFCGroundTemps", owner.wthFCGroundTemps}, {"GroundTempsFCFromEPWHeader", ground}}}};
}

// position() dereferences a stream even if closed; tellg() on a failed stream
// can also change failbit. Only query a good opened regular-file stream.
inline json stream_state(EnergyPlus::InputFile const &file)
{
    bool const opened = file.is_open();
    auto const flags = file.rdstate();
    json position = nullptr;
    if (opened && flags == std::ios_base::goodbit) position = static_cast<std::int64_t>(static_cast<std::streamoff>(file.position()));
    if (flags != file.rdstate()) throw std::runtime_error("Read-only stream position observation changed rdstate");
    return {{"file_path", file.filePath.generic_string()}, {"is_open", opened}, {"good", file.good()},
            {"rdstate_bits", static_cast<int>(flags)}, {"eof", (flags & std::ios_base::eofbit) != 0},
            {"fail", (flags & std::ios_base::failbit) != 0}, {"bad", (flags & std::ios_base::badbit) != 0},
            {"position_byte", position}, {"position_available", !position.is_null()},
            {"position_unavailability", position.is_null() ? "closed-or-nongood-stream-position-not-queried" : "none"},
            {"source_error_state_text", file.error_state_to_string()}};
}
} // namespace Clk02
#endif
