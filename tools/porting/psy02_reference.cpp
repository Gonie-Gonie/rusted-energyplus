// Test-only dispatch and read-only observations. All numerical routines below
// are unchanged linked original bodies or original inline header functions.
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/General.hh>
#include <EnergyPlus/IOFiles.hh>
#include <EnergyPlus/Psychrometrics.hh>
#include <nlohmann/json.hpp>
#include <bit>
#include <cmath>
#include <cstdint>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <limits>
#include <map>
#include <memory>
#include <sstream>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <vector>

using json = nlohmann::json;
namespace P = EnergyPlus::Psychrometrics;

std::string bits(double value)
{
    std::ostringstream out;
    out << std::hex << std::setfill('0') << std::setw(16) << std::bit_cast<std::uint64_t>(value);
    return out.str();
}
json scalar(double value)
{
    if (std::isnan(value)) return "NaN";
    if (std::isinf(value)) return value > 0 ? "+Infinity" : "-Infinity";
    return value;
}
double number(json const &value)
{
    if (value.is_object() && value.size()==1 && value.contains("ieee_bits")) {
        auto text=value.at("ieee_bits").get<std::string>();if(text.size()!=16||text.find_first_not_of("0123456789abcdef")!=std::string::npos)throw std::runtime_error("Invalid exact IEEE scalar");
        return std::bit_cast<double>(static_cast<std::uint64_t>(std::stoull(text,nullptr,16)));
    }
    if (value.is_number()) return value.get<double>();
    if (value == "NaN") return std::numeric_limits<double>::quiet_NaN();
    if (value == "+Infinity") return std::numeric_limits<double>::infinity();
    if (value == "-Infinity") return -std::numeric_limits<double>::infinity();
    throw std::runtime_error("Expected numerical scalar or exact IEEE string");
}
json state_values(EnergyPlus::EnergyPlusData const &state)
{
    auto const &s = *state.dataPsychrometrics;
    return {{"iconv_tol", scalar(s.iconvTol)}, {"iconv_tol_bits", bits(s.iconvTol)},
            {"last_patm", scalar(s.last_Patm)}, {"last_patm_bits", bits(s.last_Patm)},
            {"last_t_boil", scalar(s.last_tBoil)}, {"last_t_boil_bits", bits(s.last_tBoil)},
            {"press_save", scalar(s.Press_Save)}, {"press_save_bits", bits(s.Press_Save)},
            {"t_sat_save", scalar(s.tSat_Save)}, {"t_sat_save_bits", bits(s.tSat_Save)},
            {"use_interpolation", s.useInterpolationPsychTsatFnPb}, {"warmup", state.dataGlobal->WarmupFlag}};
}
json error_indices(EnergyPlus::EnergyPlusData const &state)
{
    return state.dataPsychrometrics->iPsyErrIndex;
}
json twb_row(EnergyPlus::cached_twb_t const &c, std::uint64_t index)
{
    return {{"kind", "Twb"}, {"index", index}, {"i_tdb", c.iTdb}, {"i_w", c.iW}, {"i_pb", c.iPb},
            {"value", scalar(c.Twb)}, {"value_bits", bits(c.Twb)}};
}
json psat_row(EnergyPlus::cached_psat_t const &c, std::uint64_t index)
{
    return {{"kind", "Psat"}, {"index", index}, {"i_tdb", c.iTdb}, {"value", scalar(c.Psat)}, {"value_bits", bits(c.Psat)}};
}
json tsat_row(EnergyPlus::cached_tsat_h_pb const &c, std::uint64_t index, std::string const &kind)
{
    return {{"kind", kind}, {"index", index}, {"i_h", c.iH}, {"i_pb", c.iPb}, {"value", scalar(c.Tsat)}, {"value_bits", bits(c.Tsat)}};
}
// Cache-key arithmetic is observation only. Never substitutes inputs or results.
json cache_slot(EnergyPlus::EnergyPlusData const &state, std::string const &function, json const &input)
{
    auto const &c = *state.dataPsychCache;
    if (function == "PsyPsatFnTemp") {
        auto tag = std::bit_cast<std::int64_t>(number(input.at("t_db_c"))) >> (52 - EnergyPlus::psatprecision_bits);
        auto index = static_cast<std::uint64_t>(tag & EnergyPlus::psatcache_mask);
        json out = psat_row(c.cached_Psat[index], index); out["requested_tags"] = {{"i_tdb", tag}}; return out;
    }
    if (function == "PsyTsatFnPb") {
        auto tag = std::bit_cast<std::int64_t>(number(input.at("p_pa"))) >> (52 - c.tsatprecision_bits);
        auto index = static_cast<std::uint64_t>(tag & EnergyPlus::tsatcache_mask);
        json out = tsat_row(c.cached_Tsat[index], index, "TsatPb"); out["requested_tags"] = {{"i_pb", tag}}; return out;
    }
    if (function == "PsyTsatFnHPb") {
        auto h = std::bit_cast<std::int64_t>(number(input.at("h_j_per_kg"))) >> (52 - EnergyPlus::tsat_hbp_precision_bits);
        auto p = std::bit_cast<std::int64_t>(number(input.at("p_pa"))) >> (52 - EnergyPlus::tsat_hbp_precision_bits);
        auto index = static_cast<std::uint64_t>((h ^ p) & (EnergyPlus::tsat_hbp_cache_size - 1));
        json out = tsat_row(c.cached_Tsat_HPb[index], index, "TsatHPb"); out["requested_tags"] = {{"i_h", h}, {"i_pb", p}}; return out;
    }
    if (function == "PsyTwbFnTdbWPb") {
        auto t = std::bit_cast<std::uint64_t>(number(input.at("t_db_c"))) >> (52 - EnergyPlus::twbprecision_bits);
        auto w = std::bit_cast<std::uint64_t>(number(input.at("w_kg_per_kg"))) >> (52 - EnergyPlus::twbprecision_bits);
        auto p = std::bit_cast<std::uint64_t>(number(input.at("p_pa"))) >> (52 - EnergyPlus::twbprecision_bits);
        auto index = (t ^ w ^ p) & (EnergyPlus::twbcache_size - 1);
        json out = twb_row(c.cached_Twb[index], index); out["requested_tags"] = {{"i_tdb", t}, {"i_w", w}, {"i_pb", p}}; return out;
    }
    return nullptr;
}
bool cache_hit(json const &slot)
{
    for (auto const &tag : slot.at("requested_tags").items()) if (slot.at(tag.key()) != tag.value()) return false;
    return true;
}
json final_caches(EnergyPlus::EnergyPlusData const &state)
{
    auto const &c = *state.dataPsychCache; json result = { {"Twb", json::array()}, {"Psat", json::array()}, {"TsatPb", json::array()}, {"TsatHPb", json::array()} };
    // Once per process only; exact nondefault occupied state, sorted by array index.
    for (std::size_t i = 0; i < c.cached_Twb.size(); ++i) {
        auto const &r = c.cached_Twb[i]; if (r.iTdb != 0 || r.iW != 0 || r.iPb != 0 || std::bit_cast<std::uint64_t>(r.Twb) != 0) result["Twb"].push_back(twb_row(r, i));
    }
    for (std::size_t i = 0; i < c.cached_Psat.size(); ++i) {
        auto const &r = c.cached_Psat[i]; if (r.iTdb != -1000 || std::bit_cast<std::uint64_t>(r.Psat) != 0) result["Psat"].push_back(psat_row(r, i));
    }
    for (std::size_t i = 0; i < c.cached_Tsat.size(); ++i) {
        auto const &r = c.cached_Tsat[i]; if (r.iH != 0 || r.iPb != 0 || std::bit_cast<std::uint64_t>(r.Tsat) != 0) result["TsatPb"].push_back(tsat_row(r, i, "TsatPb"));
    }
    for (std::size_t i = 0; i < c.cached_Tsat_HPb.size(); ++i) {
        auto const &r = c.cached_Tsat_HPb[i]; if (r.iH != 0 || r.iPb != 0 || std::bit_cast<std::uint64_t>(r.Tsat) != 0) result["TsatHPb"].push_back(tsat_row(r, i, "TsatHPb"));
    }
    return result;
}
json resolve(json const &value, std::map<int, json> const &previous)
{
    if (value.is_object() && value.contains("from_call")) {
        if (value.size() != 1) throw std::runtime_error("Invalid from_call");
        auto item = previous.find(value.at("from_call").get<int>());
        if (item == previous.end()) throw std::runtime_error("Unresolved/forward from_call");
        return item->second;
    }
    if (value.is_object()) { json out=json::object(); for (auto const &item:value.items()) out[item.key()]=resolve(item.value(),previous); return out; }
    return value;
}
std::pair<double,std::string> dispatch(EnergyPlus::EnergyPlusData &s, std::string const &f, json const &i, bool suppress)
{
    auto n=[&](char const *key){return number(i.at(key));};
    if (f=="PsyTsatFnHPb") return {P::PsyTsatFnHPb(s,n("h_j_per_kg"),n("p_pa"),"PSY-02 reference"),"degC"};
    if (f=="PsyTsatFnHPb_raw") return {P::PsyTsatFnHPb_raw(s,n("h_j_per_kg"),n("p_pa"),"PSY-02 reference"),"degC"};
    if (f=="PsyWFnTdbRhPb") return {P::PsyWFnTdbRhPb(s,n("t_db_c"),n("rh_fraction"),n("p_pa"),"PSY-02 reference"),"kg/kg"};
    if (f=="PsyWFnTdbH") return {P::PsyWFnTdbH(s,n("t_db_c"),n("h_j_per_kg"),"PSY-02 reference",suppress),"kg/kg"};
    if (f=="PsyTwbFnTdbWPb") return {P::PsyTwbFnTdbWPb(s,n("t_db_c"),n("w_kg_per_kg"),n("p_pa"),"PSY-02 reference"),"degC"};
    if (f=="PsyTwbFnTdbWPb_raw") return {P::PsyTwbFnTdbWPb_raw(s,n("t_db_c"),n("w_kg_per_kg"),n("p_pa"),"PSY-02 reference"),"degC"};
    if (f=="PsyPsatFnTemp") return {P::PsyPsatFnTemp(s,n("t_db_c"),"PSY-02 reference"),"Pa"};
    if (f=="PsyPsatFnTemp_raw") return {P::PsyPsatFnTemp_raw(s,n("t_db_c"),"PSY-02 reference"),"Pa"};
    if (f=="PsyTsatFnPb") return {P::PsyTsatFnPb(s,n("p_pa"),"PSY-02 reference"),"degC"};
    if (f=="PsyTsatFnPb_raw") return {P::PsyTsatFnPb_raw(s,n("p_pa"),"PSY-02 reference"),"degC"};
    if (f=="PsyWFnTdbTwbPb") return {P::PsyWFnTdbTwbPb(s,n("t_db_c"),n("t_wb_c"),n("p_pa"),"PSY-02 reference"),"kg/kg"};
    if (f=="PsyHFnTdbW") return {P::PsyHFnTdbW(n("t_db_c"),n("w_kg_per_kg")),"J/kg"};
    throw std::runtime_error("Unknown selected function: "+f);
}
json evaluate(EnergyPlus::EnergyPlusData &state, json &messages, json const &call, json const &input)
{
    auto *s=&state;json out=call;auto f=call.at("function").get<std::string>();
        if(call.contains("state_overrides")){
            auto const &o=call.at("state_overrides");for(auto const &item:o.items())if(item.key()!="warmup"&&item.key()!="iconv_tol")throw std::runtime_error("Unsupported state override");
            if(o.contains("warmup"))s->dataGlobal->WarmupFlag=o.at("warmup").get<bool>();
            if(o.contains("iconv_tol")){double t=number(o.at("iconv_tol"));if(!std::isfinite(t)||t<0)throw std::runtime_error("Invalid iconv_tol");s->dataPsychrometrics->iconvTol=t;}
        }
        auto count=messages.size();out["resolved_inputs"]=input;out["state_before"]=state_values(state);out["source_error_indices_before"]=error_indices(state);
        out["cache_before"]=cache_slot(state,f,input);out["cache_hit"]=out["cache_before"].is_null()?json(nullptr):json(cache_hit(out["cache_before"]));
        if(f=="General::Iterate"){
            double x=0,x1=number(input.at("x1")),y1=number(input.at("y1"));int converged=0;
            EnergyPlus::General::Iterate(x,number(input.at("tol")),number(input.at("x0")),number(input.at("y0")),x1,y1,input.at("iteration").get<int>(),converged);
            out["value"]={{"result_x",scalar(x)},{"result_x_bits",bits(x)},{"x1",scalar(x1)},{"x1_bits",bits(x1)},{"y1",scalar(y1)},{"y1_bits",bits(y1)},{"converged",converged}};out["unit"]="diagnostic scalar state";
        }else{
            auto [value,unit]=dispatch(state,f,input,call.value("suppress_warnings",false));out["value"]=scalar(value);out["value_bits"]=bits(value);out["value_class"]=std::isnan(value)?"nan":std::isinf(value)?(value>0?"positive_infinity":"negative_infinity"):value==0.0?(std::signbit(value)?"negative_zero":"positive_zero"):"finite";out["unit"]=unit;
        }
        out["state_after"]=state_values(state);out["source_error_indices_after"]=error_indices(state);out["cache_after"]=cache_slot(state,f,input);
        out["source_error_messages"]=json::array();for(std::size_t j=count;j<messages.size();++j)out["source_error_messages"].push_back(messages[j]);
    return out;
}
json root_input(std::string const &f, json const &row)
{
    static std::map<std::string,std::vector<std::string>> const keys={
        {"PsyTsatFnHPb",{"h_j_per_kg","p_pa"}},{"PsyWFnTdbRhPb",{"t_db_c","rh_fraction","p_pa"}},
        {"PsyWFnTdbH",{"t_db_c","h_j_per_kg"}},{"PsyTwbFnTdbWPb",{"t_db_c","w_kg_per_kg","p_pa"}},
        {"PsyPsatFnTemp",{"t_db_c"}},{"PsyTsatFnPb",{"p_pa"}},
        {"PsyTsatFnHPb_raw",{"h_j_per_kg","p_pa"}},
        {"PsyTwbFnTdbWPb_raw",{"t_db_c","w_kg_per_kg","p_pa"}},
        {"PsyPsatFnTemp_raw",{"t_db_c"}},{"PsyTsatFnPb_raw",{"p_pa"}},
        {"PsyWFnTdbTwbPb",{"t_db_c","t_wb_c","p_pa"}}};
    auto names=keys.find(f);if(names==keys.end())throw std::runtime_error("Unselected production root");
    auto const &values=row.at("input_bits");if(values.size()!=names->second.size())throw std::runtime_error("Invalid production input arity");
    json inputs=json::object();for(std::size_t i=0;i<values.size();++i){auto text=values[i].get<std::string>();if(text.size()!=16||text.find_first_not_of("0123456789abcdef")!=std::string::npos)throw std::runtime_error("Invalid IEEE input bits");inputs[names->second[i]]={{"ieee_bits",text}};}
    return inputs;
}
json run(json const &request)
{
    auto schema=request.at("schema");if(schema!="psy02-tuples.v1"&&schema!="psy02-calls.v1")throw std::runtime_error("Invalid request schema");
    auto state=std::make_unique<EnergyPlus::EnergyPlusData>();
    state->dataPsychrometrics->clear_state();state->dataPsychCache->clear_state();state->init_constant_state(*state);
    auto errors=std::make_unique<std::ostringstream>();state->files.err_stream=std::move(errors);
    json messages=json::array();state->dataGlobal->errorCallback=[&](EnergyPlus::Error e,std::string const &message){messages.push_back({{"level",static_cast<int>(e)},{"message",message}});};
    auto initial=state_values(*state);json output;
    if(schema=="psy02-tuples.v1"){
        std::map<int,json> previous;json calls=json::array();
        for(auto const &call:request.at("calls")){
            int id=call.at("call_index").get<int>();if(previous.count(id))throw std::runtime_error("Duplicate call_index");
            auto out=evaluate(*state,messages,call,resolve(call.at("inputs"),previous));previous.emplace(id,out.at("value"));calls.push_back(std::move(out));
        }
        output={{"schema","psy02-results.v1"},{"calls",calls}};
    }else{
        auto const &ids=request.at("ordered_ids");auto const &dictionary=request.at("dictionary");
        if(request.at("complete_on_collecting_thread")!=true||request.at("omitted_root_count")!=0||request.at("truncation_reason")!=nullptr||request.at("total_root_count")!=ids.size())throw std::runtime_error("Incomplete/truncated production roots");
        json results=json::array();json ordered=json::array();std::unordered_map<std::string,std::uint32_t> variants;
        for(auto const &id:ids){
            auto index=id.get<std::uint32_t>();if(index>=dictionary.size())throw std::runtime_error("Invalid ordered root dictionary index");
            auto const &row=dictionary[index];auto f=row.at("routine").get<std::string>();auto input=root_input(f,row);
            json call={{"function",f},{"inputs",input},{"input_dictionary_id",index},{"input_bits",row.at("input_bits")}};
            for(char const *field:{"phase","caller","context"})if(row.contains(field))call[field]=row.at(field);
            // Expected result, state and cache fields never enter the dispatcher.
            auto out=evaluate(*state,messages,call,input);auto key=out.dump();auto found=variants.find(key);
            std::uint32_t reference_id;if(found==variants.end()){reference_id=static_cast<std::uint32_t>(results.size());variants.emplace(std::move(key),reference_id);results.push_back(std::move(out));}else reference_id=found->second;
            ordered.push_back(reference_id);
        }
        output={{"schema","psy02-reference-roots.v1"},{"dictionary",results},{"ordered_ids",ordered},{"total_root_count",ids.size()},{"omitted_root_count",0},{"every_ordered_root_evaluated",true}};
    }
    output["initial_state"]=initial;output["final_state"]=state_values(*state);output["final_caches"]=final_caches(*state);output["source_error_indices_final"]=error_indices(*state);output["source_error_messages"]=messages;output["source_warning_lifecycle_is_paired"]=false;output["internal_local_iteration_count_observed"]=false;return output;
}
int main(int argc,char **argv)
{
    try{
        json request;if(argc==2){std::ifstream file(argv[1]);if(!file)throw std::runtime_error("Cannot open request");file>>request;}
        else if(argc==1)std::cin>>request;else throw std::runtime_error("Usage: psy02_reference [request.json]");
        std::cout<<run(request).dump()<<'\n';return 0;
    }catch(std::exception const &e){std::cerr<<e.what()<<'\n';return 2;}
}
