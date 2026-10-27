// Standalone, test-only observation of the pinned RE2 parser's actual DAG.
// No EnergyPlus runtime, Rust implementation or expected AST is linked.
#include <cstdio>
#include <exception>
#include <iostream>
#include <memory>
#include <sstream>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <vector>

#ifdef _WIN32
#include <fcntl.h>
#include <io.h>
#endif

#include "re2/re2.h"
#include "re2/regexp.h"
#include "re2/stringpiece.h"

namespace {
int nibble(unsigned char value) {
    if (value >= '0' && value <= '9') return value - '0';
    if (value >= 'a' && value <= 'f') return value - 'a' + 10;
    if (value >= 'A' && value <= 'F') return value - 'A' + 10;
    return -1;
}

bool decode_hex(const std::string& encoded, std::string& output) {
    if (encoded.size() % 2 != 0) return false;
    output.clear();
    output.reserve(encoded.size() / 2);
    for (std::size_t index = 0; index < encoded.size(); index += 2) {
        const int high = nibble(static_cast<unsigned char>(encoded[index]));
        const int low = nibble(static_cast<unsigned char>(encoded[index + 1]));
        if (high < 0 || low < 0) return false;
        output.push_back(static_cast<char>((high << 4) | low));
    }
    return true;
}

std::string encode_hex(const char* data, std::size_t size) {
    static const char alphabet[] = "0123456789abcdef";
    std::string output;
    output.reserve(size * 2);
    for (std::size_t index = 0; index < size; ++index) {
        const unsigned char value = static_cast<unsigned char>(data[index]);
        output.push_back(alphabet[value >> 4]);
        output.push_back(alphabet[value & 15]);
    }
    return output;
}

std::string encode_hex(const std::string& bytes) {
    return encode_hex(bytes.data(), bytes.size());
}

struct ReleaseRegexp {
    void operator()(re2::Regexp* value) const {
        if (value != nullptr) value->Decref();
    }
};

// IDs are assigned on the first preorder visit, descending children in their
// actual sub() order. Shared pointer identities receive one ID, not copies.
// Neither traversal nor JSON serialization recurses on the C++ call stack.
void emit_nodes(std::ostream& output, re2::Regexp* root) {
    std::vector<re2::Regexp*> nodes;
    std::unordered_map<re2::Regexp*, std::size_t> ids;
    std::vector<re2::Regexp*> pending;
    if (root != nullptr) pending.push_back(root);
    while (!pending.empty()) {
        re2::Regexp* node = pending.back();
        pending.pop_back();
        if (node == nullptr) throw std::runtime_error("null reachable AST child");
        if (ids.find(node) != ids.end()) continue;
        ids.emplace(node, nodes.size());
        nodes.push_back(node);
        for (int index = node->nsub(); index > 0; --index) {
            pending.push_back(node->sub()[index - 1]);
        }
    }

    std::unordered_map<const std::string*, std::size_t> name_ids;
    output << '[';
    for (std::size_t id = 0; id < nodes.size(); ++id) {
        re2::Regexp* node = nodes[id];
        if (id != 0) output << ',';
        output << "{\"id\":" << id
               << ",\"op\":" << static_cast<int>(node->op())
               << ",\"flags\":" << static_cast<int>(node->parse_flags())
               << ",\"simple\":" << (node->simple() ? "true" : "false")
               << ",\"children\":[";
        for (int index = 0; index < node->nsub(); ++index) {
            if (index != 0) output << ',';
            output << ids.at(node->sub()[index]);
        }
        output << "],\"payload\":{";
        switch (node->op()) {
            case re2::kRegexpLiteral:
                output << "\"rune\":" << node->rune();
                break;
            case re2::kRegexpLiteralString:
                output << "\"runes\":[";
                for (int index = 0; index < node->nrunes(); ++index) {
                    if (index != 0) output << ',';
                    output << node->runes()[index];
                }
                output << ']';
                break;
            case re2::kRegexpRepeat:
                output << "\"min\":" << node->min() << ",\"max\":" << node->max();
                break;
            case re2::kRegexpCapture: {
                output << "\"index\":" << node->cap() << ",\"name_hex\":";
                const std::string* name = node->name();
                if (name == nullptr) {
                    output << "null,\"name_id\":null";
                } else {
                    const auto identity = name_ids.emplace(name, name_ids.size());
                    output << '"' << encode_hex(*name) << '"'
                           << ",\"name_id\":" << identity.first->second;
                }
                break;
            }
            case re2::kRegexpCharClass: {
                re2::CharClass* cc = node->cc();
                if (cc == nullptr) throw std::runtime_error("unfinished reachable CharClass");
                output << "\"ranges\":[";
                bool first = true;
                for (re2::CharClass::iterator range = cc->begin(); range != cc->end(); ++range) {
                    if (!first) output << ',';
                    first = false;
                    output << '[' << range->lo << ',' << range->hi << ']';
                }
                output << "],\"rune_count\":" << cc->size()
                       << ",\"folds_ascii\":" << (cc->FoldsASCII() ? "true" : "false")
                       << ",\"empty\":" << (cc->empty() ? "true" : "false")
                       << ",\"full\":" << (cc->full() ? "true" : "false");
                break;
            }
            case re2::kRegexpHaveMatch:
                output << "\"match_id\":" << node->match_id();
                break;
            case re2::kRegexpNoMatch:
            case re2::kRegexpEmptyMatch:
            case re2::kRegexpConcat:
            case re2::kRegexpAlternate:
            case re2::kRegexpStar:
            case re2::kRegexpPlus:
            case re2::kRegexpQuest:
            case re2::kRegexpAnyChar:
            case re2::kRegexpAnyByte:
            case re2::kRegexpBeginLine:
            case re2::kRegexpEndLine:
            case re2::kRegexpWordBoundary:
            case re2::kRegexpNoWordBoundary:
            case re2::kRegexpBeginText:
            case re2::kRegexpEndText:
                break;
            default:
                throw std::runtime_error("unexpected reachable primary AST opcode");
        }
        output << "}}";
    }
    output << ']';
}

std::string observe(const std::string& pattern) {
    // This is the actual default-constructor flag calculation, not a copied
    // numeric mask. Parsing stops before RequiredPrefix, Simplify or lowering.
    const re2::RE2::Options options;
    const int flags = options.ParseFlags();
    re2::RegexpStatus status;
    const re2::StringPiece pattern_bytes(pattern.data(), pattern.size());
    std::unique_ptr<re2::Regexp, ReleaseRegexp> root(re2::Regexp::Parse(
        pattern_bytes, static_cast<re2::Regexp::ParseFlags>(flags), &status));
    const re2::StringPiece& argument = status.error_arg();
    std::ostringstream output;
    output << "{\"schema\":\"pinned-re2-native-ast.v1\",\"pattern_hex\":\""
           << encode_hex(pattern) << "\",\"input_parse_flags\":" << flags
           << ",\"ok\":" << (root != nullptr ? "true" : "false")
           << ",\"status\":{\"ok\":" << (status.ok() ? "true" : "false")
           << ",\"code\":" << static_cast<int>(status.code())
           << ",\"text_hex\":\"" << encode_hex(status.Text())
           << "\",\"error_arg_hex\":\"" << encode_hex(argument.data(), argument.size())
           << "\"},\"root\":";
    if (root == nullptr) output << "null";
    else output << '0';
    output << ",\"nodes\":";
    emit_nodes(output, root.get());
    output << "}\n";
    if (!output) throw std::runtime_error("AST serialization failed");
    return output.str();
}
}  // namespace

int main(int argc, char**) {
    if (argc != 1) {
        std::cerr << "usage: re2_native_ast_observer.exe < patterns.hex\n";
        return 2;
    }
#ifdef _WIN32
    if (_setmode(_fileno(stdin), _O_BINARY) == -1 ||
        _setmode(_fileno(stdout), _O_BINARY) == -1) {
        std::cerr << "transport: unable to select binary standard streams\n";
        return 2;
    }
#endif
    std::ios::sync_with_stdio(false);
    std::cin.tie(nullptr);
    std::size_t ordinal = 0;
    try {
        std::string line;
        while (std::getline(std::cin, line)) {
            ++ordinal;
            if (!line.empty() && line.back() == '\r') line.pop_back();
            std::string pattern;
            if (!decode_hex(line, pattern)) {
                std::cerr << "transport line " << ordinal << ": malformed pattern hex\n";
                return 2;
            }
            // Buffer one complete record, so a host failure cannot leave a
            // partial JSON line masquerading as a completed observation.
            std::cout << observe(pattern);
            if (!std::cout) {
                std::cerr << "transport: stdout write failed\n";
                return 2;
            }
        }
        if (!std::cin.eof()) {
            std::cerr << "transport: stdin read failed\n";
            return 2;
        }
    } catch (const std::exception& error) {
        std::cerr << "host exception at transport line " << ordinal << ": " << error.what() << '\n';
        return 3;
    }
    return 0;
}
