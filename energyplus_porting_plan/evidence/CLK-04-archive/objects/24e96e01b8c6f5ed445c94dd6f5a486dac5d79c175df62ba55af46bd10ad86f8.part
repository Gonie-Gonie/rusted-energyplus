// Read-only selected original zone-air fields. No numerical initializer here.
#ifndef ZON01_REFERENCE_FIELDS_HH
#define ZON01_REFERENCE_FIELDS_HH
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataEnvironment.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataHeatBalance.hh>
#include <EnergyPlus/DataHVACGlobals.hh>
#include <EnergyPlus/WeatherManager.hh>
#include <EnergyPlus/ZoneTempPredictorCorrector.hh>
#include <nlohmann/json.hpp>
#include <bit>
#include <cmath>
#include <cstdint>
#include <iomanip>
#include <sstream>

#if !defined(__MINGW32__) || !defined(_WIN64) || __GNUC__ != 13 || __GNUC_MINOR__ != 2
#error "Original private-state reference requires the verified GCC13.2 Windows ABI"
#endif
#if defined(_GLIBCXX_DEBUG) || defined(__FAST_MATH__) || defined(NDEBUG)
#error "Reference requires original assertions, nondebug containers and no fast math"
#endif
#if !defined(EP_psych_errors) || defined(EP_psych_stats)
#error "Reference definitions must match the genuine original core"
#endif

// The first group is preserved by the named member. The second is written.
#define ZON01_RETAINED_SCALARS(X) X(MAT) X(ZT) X(ZTAV) X(XMPT) X(TMX) X(TM2) X(airHumRat) X(airHumRatAvg)
#define ZON01_WRITTEN_SCALARS(X) X(WTimeMinusP) X(W1) X(WMX) X(WM2) X(airHumRatTemp) X(tempIndLoad) X(tempDepLoad) X(airRelHum) X(AirPowerCap) X(T1)
#define ZON01_ARRAYS(X) X(XMAT) X(DSXMAT) X(ZTM) X(WPrevZoneTS) X(DSWPrevZoneTS) X(WPrevZoneTSTemp)

namespace Zon01 {
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
inline json zone_fields(EnergyPlus::ZoneTempPredictorCorrector::ZoneSpaceHeatBalanceData const &zone)
{
    json result = json::object();
#define OBSERVE_SCALAR(name) result[#name] = scalar(zone.name);
    ZON01_RETAINED_SCALARS(OBSERVE_SCALAR)
    ZON01_WRITTEN_SCALARS(OBSERVE_SCALAR)
#undef OBSERVE_SCALAR
#define OBSERVE_ARRAY(name) result[#name] = json::array(); for (double v : zone.name) result[#name].push_back(scalar(v));
    ZON01_ARRAYS(OBSERVE_ARRAY)
#undef OBSERVE_ARRAY
    return result;
}
inline json flags(EnergyPlus::EnergyPlusData const &state)
{
    auto const &owner = *state.dataZoneTempPredictorCorrector;
    return {{"MyEnvrnFlag", owner.MyEnvrnFlag}, {"BeginEnvrnFlag", state.dataGlobal->BeginEnvrnFlag},
            {"InitZoneAirSetPointsOneTimeFlag", owner.InitZoneAirSetPointsOneTimeFlag},
            {"doSpaceHeatBalance", state.dataHeatBal->doSpaceHeatBalance}, {"MyDayFlag", owner.MyDayFlag},
            {"BeginDayFlag", state.dataGlobal->BeginDayFlag}, {"ErrorsFound", owner.ErrorsFound},
            {"ControlledZonesChecked", owner.ControlledZonesChecked}};
}
inline json context(EnergyPlus::EnergyPlusData const &state)
{
    auto const &g = *state.dataGlobal;
    auto const &h = *state.dataHVACGlobal;
    return {{"environment_index", state.dataWeather->Envrn}, {"kind_of_sim", static_cast<int>(g.KindOfSim)},
            {"warmup", g.WarmupFlag}, {"kickoff", g.KickOffSimulation}, {"begin_sim", g.BeginSimFlag},
            {"end_environment", g.EndEnvrnFlag}, {"day_of_sim", g.DayOfSim}, {"hour_of_day", g.HourOfDay},
            {"zone_timestep", g.TimeStep}, {"zone_steps_per_hour", g.TimeStepsInHour},
            {"zone_timestep_hours", scalar(g.TimeStepZone)}, {"system_timestep_hours", scalar(h.TimeStepSys)},
            {"system_time_elapsed_hours", scalar(h.SysTimeElapsed)}, {"raw_current_time_hours", scalar(g.CurrentTime)},
            {"first_system_timestep", h.FirstTimeStepSysFlag}, {"shorten_system_timestep", h.ShortenTimeStepSys},
            {"use_zone_timestep_history", h.UseZoneTimeStepHistory}, {"system_timestep_count", h.NumOfSysTimeSteps}};
}
inline json snapshot(EnergyPlus::EnergyPlusData const &state, std::string const &phase)
{
    json zones = json::array();
    auto const &owners = state.dataZoneTempPredictorCorrector->zoneHeatBalance;
    // Allocated concrete owners have fully initialized selected member fields.
    for (int i = 1; i <= static_cast<int>(owners.size()); ++i) {
        json name = state.dataHeatBal->Zone.allocated() && i <= static_cast<int>(state.dataHeatBal->Zone.size()) ?
                    json(state.dataHeatBal->Zone(i).Name) : json(nullptr);
        zones.push_back({{"id", i}, {"name", name}, {"fields", zone_fields(owners(i))}});
    }
    return {{"phase", phase}, {"zones", zones}, {"zone_owner_allocated", owners.allocated()},
            {"zone_owner_count", owners.size()}, {"flags", flags(state)}, {"context", context(state)},
            {"OutHumRat", scalar(state.dataEnvrn->OutHumRat)}};
}
} // namespace Zon01
#endif
