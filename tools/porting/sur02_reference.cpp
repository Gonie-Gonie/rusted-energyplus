// Type/array adapter around an unmodified literal Original SUR-02 block.
// The checker inserts only the pinned block at ORIGINAL_BLOCK below.
#include <array>
#include <cstdint>
#include <cstdio>
#include <cstring>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
#ifdef _WIN32
#include <fcntl.h>
#include <io.h>
#endif

using Real64 = double;
namespace DataSurfaces {
enum class HeatTransferModel { CTF = 0, EMPD = 1, Other = 2 };
}
template <typename T> struct Array1 {
    std::vector<T> values;
    T &operator()(int i) { return values.at(static_cast<std::size_t>(i - 1)); }
    T const &operator()(int i) const { return values.at(static_cast<std::size_t>(i - 1)); }
};
struct Zone { std::vector<int> spaceIndexes; };
struct Space { int OpaqOrIntMassSurfaceFirst; int OpaqOrIntMassSurfaceLast; };
struct Surface { DataSurfaces::HeatTransferModel HeatTransferAlgorithm; int Construction; };
struct Construction {
    int NumCTFTerms;
    std::array<double, 19> CTFOutside, CTFCross, CTFInside, CTFFlux;
};
struct Globals { int NumOfZones; };
struct HeatBalance { Array1<struct Zone> Zone; Array1<Space> space; };
struct Surfaces { Array1<struct Surface> Surface; };
struct Constructions { Array1<Construction> Construct; };
struct HeatBalanceSurface {
    Array1<double> SurfCTFConstInPart, SurfCTFConstOutPart;
    Array1<Array1<double>> SurfOutsideTempHist, SurfInsideTempHist;
    Array1<Array1<double>> SurfOutsideFluxHist, SurfInsideFluxHist;
};
struct State {
    Globals *dataGlobal;
    HeatBalance *dataHeatBal;
    Surfaces *dataSurface;
    Constructions *dataConstruction;
    HeatBalanceSurface *dataHeatBalSurf;
};
static void invoke(State &state) {
/*__ORIGINAL_BLOCK__*/
}
static void require(bool value, char const *message) {
    if (!value) throw std::runtime_error(message);
}
static void token(std::istream &input, char const *expected) {
    std::string value;
    require(bool(input >> value) && value == expected, "input token");
}
static int integer(std::istream &input) {
    int value;
    require(bool(input >> value), "input integer");
    return value;
}
static std::string word(std::istream &input) {
    std::string value;
    require(bool(input >> value), "input word");
    return value;
}
static double real(std::istream &input) {
    auto text = word(input);
    require(text.size() == 16 && text.find_first_not_of("0123456789abcdef") == std::string::npos, "raw16 input");
    std::uint64_t bits = std::stoull(text, nullptr, 16);
    require((bits & UINT64_C(0x7ff0000000000000)) != UINT64_C(0x7ff0000000000000), "finite input");
    double value;
    std::memcpy(&value, &bits, sizeof value);
    return value;
}
static std::string hex(double value) {
    std::uint64_t bits;
    std::memcpy(&bits, &value, sizeof bits);
    std::ostringstream out;
    out << std::hex << std::setfill('0') << std::setw(16) << bits;
    return out.str();
}
static std::string outputs(HeatBalanceSurface const &h) {
    std::ostringstream out;
    for (std::size_t i = 0; i < h.SurfCTFConstInPart.values.size(); ++i)
        out << ' ' << hex(h.SurfCTFConstInPart.values[i]) << ' ' << hex(h.SurfCTFConstOutPart.values[i]);
    return out.str();
}
static std::string readset(State const &s, std::vector<int> const &typed, std::vector<int> const &history_ids) {
    std::ostringstream out;
    out << s.dataGlobal->NumOfZones;
    for (auto const &z : s.dataHeatBal->Zone.values) {
        out << ' ' << z.spaceIndexes.size();
        for (int n : z.spaceIndexes) out << ' ' << n;
    }
    out << ' ' << s.dataHeatBal->space.values.size();
    for (auto const &v : s.dataHeatBal->space.values)
        out << ' ' << v.OpaqOrIntMassSurfaceFirst << ' ' << v.OpaqOrIntMassSurfaceLast;
    out << ' ' << s.dataSurface->Surface.values.size();
    for (std::size_t i = 0; i < typed.size(); ++i) {
        auto const &v = s.dataSurface->Surface.values[i];
        out << ' ' << typed[i] << ' ' << int(v.HeatTransferAlgorithm) << ' ' << v.Construction;
    }
    out << ' ' << s.dataConstruction->Construct.values.size();
    for (auto const &c : s.dataConstruction->Construct.values) {
        out << ' ' << c.NumCTFTerms;
        for (auto const *a : {&c.CTFOutside, &c.CTFCross, &c.CTFInside, &c.CTFFlux})
            for (double x : *a) out << ' ' << hex(x);
    }
    out << ' ' << history_ids.size();
    for (int id : history_ids) out << ' ' << id;
    for (auto const *a : {&s.dataHeatBalSurf->SurfOutsideTempHist, &s.dataHeatBalSurf->SurfInsideTempHist,
                         &s.dataHeatBalSurf->SurfOutsideFluxHist, &s.dataHeatBalSurf->SurfInsideFluxHist})
        for (auto const &row : a->values)
            for (double x : row.values) out << ' ' << hex(x);
    return out.str();
}
static std::vector<void const *> addresses(State const &s) {
    std::vector<void const *> out = {s.dataGlobal, s.dataHeatBal, s.dataSurface, s.dataConstruction,
        s.dataHeatBalSurf, s.dataHeatBal->Zone.values.data(), s.dataHeatBal->space.values.data(),
        s.dataSurface->Surface.values.data(), s.dataConstruction->Construct.values.data(),
        s.dataHeatBalSurf->SurfCTFConstInPart.values.data(), s.dataHeatBalSurf->SurfCTFConstOutPart.values.data()};
    for (auto const *a : {&s.dataHeatBalSurf->SurfOutsideTempHist, &s.dataHeatBalSurf->SurfInsideTempHist,
                         &s.dataHeatBalSurf->SurfOutsideFluxHist, &s.dataHeatBalSurf->SurfInsideFluxHist})
        for (auto const &row : a->values) out.push_back(row.values.data());
    return out;
}
int main(int argc, char **argv) {
    try {
#ifdef _WIN32
        require(_setmode(_fileno(stdout), _O_BINARY) != -1, "binary stdout");
#endif
        require(argc == 2, "one input path");
        std::ifstream input(argv[1]);
        require(bool(input), "input open");
        token(input, "SUR02_INPUT_V1");
        token(input, "LIFETIMES");
        int lives = integer(input);
        require(lives > 0 && lives < 100, "bounded lifetimes");
        for (int life = 0; life < lives; ++life) {
            token(input, "LIFE");
            auto name = word(input);
            int nz = integer(input), nsp = integer(input), ns = integer(input), nc = integer(input), nh = integer(input), calls = integer(input);
            require(nz > 0 && nsp > 0 && ns > 0 && nc > 0 && nh == ns && ns <= 8 && calls > 0 && calls < 100, "bounded primary dimensions");
            Globals g{nz}; HeatBalance hb; Surfaces sf; Constructions cs; HeatBalanceSurface hs;
            hb.Zone.values.resize(nz); hb.space.values.resize(nsp); sf.Surface.values.resize(ns); cs.Construct.values.resize(nc);
            std::vector<int> typed, history_ids;
            for (auto &z : hb.Zone.values) {
                token(input, "ZONE"); int n = integer(input);
                require(n > 0 && n <= nsp, "zone space extent");
                for (int j = 0; j < n; ++j) z.spaceIndexes.push_back(integer(input));
            }
            for (auto &s : hb.space.values) { token(input, "SPACE"); s.OpaqOrIntMassSurfaceFirst = integer(input); s.OpaqOrIntMassSurfaceLast = integer(input); }
            for (auto &s : sf.Surface.values) {
                token(input, "SURFACE"); typed.push_back(integer(input)); int a = integer(input);
                require(a == 0 || a == 2, "CTF or non-EMPD skip selector");
                s.HeatTransferAlgorithm = DataSurfaces::HeatTransferModel(a); s.Construction = integer(input);
            }
            for (auto &c : cs.Construct.values) {
                token(input, "CONSTRUCTION"); c.NumCTFTerms = integer(input);
                require(c.NumCTFTerms >= 0 && c.NumCTFTerms <= 18, "fixed19 term domain");
                for (auto *a : {&c.CTFOutside, &c.CTFCross, &c.CTFInside, &c.CTFFlux}) for (double &x : *a) x = real(input);
            }
            token(input, "HISTORY_IDS");
            for (int j = 0; j < nh; ++j) history_ids.push_back(integer(input));
            for (auto *a : {&hs.SurfOutsideTempHist, &hs.SurfInsideTempHist, &hs.SurfOutsideFluxHist, &hs.SurfInsideFluxHist}) {
                a->values.resize(19); for (auto &row : a->values) row.values.resize(ns);
            }
            token(input, "INITIAL");
            for (int j = 0; j < ns; ++j) { hs.SurfCTFConstInPart.values.push_back(real(input)); hs.SurfCTFConstOutPart.values.push_back(real(input)); }
            State state{&g, &hb, &sf, &cs, &hs};
            auto initial_addresses = addresses(state);
            for (int call = 0; call < calls; ++call) {
                token(input, "CALL"); auto call_name = word(input);
                for (auto *a : {&hs.SurfOutsideTempHist, &hs.SurfInsideTempHist, &hs.SurfOutsideFluxHist, &hs.SurfInsideFluxHist})
                    for (auto &row : a->values) for (double &x : row.values) x = real(input);
                std::cout << "CASE " << name << " CALL " << call_name << " INDEX " << call << '\n';
                std::cout << "BEFORE" << outputs(hs) << '\n';
                auto before = readset(state, typed, history_ids);
                std::cout << "READSET_BEFORE " << before << '\n';
                invoke(state);
                std::cout << "AFTER" << outputs(hs) << '\n';
                auto after = readset(state, typed, history_ids);
                std::cout << "READSET_AFTER " << after << '\n';
                require(before == after, "Original mutated a read-only primary");
                require(initial_addresses == addresses(state), "Original adapter owner/storage moved");
                std::cout << "OWNER_STORAGE_STABLE 1\n";
            }
        }
        token(input, "END"); std::string extra; require(!(input >> extra), "no trailing input");
        return 0;
    } catch (std::exception const &e) { std::cerr << e.what() << '\n'; return 2; }
}
