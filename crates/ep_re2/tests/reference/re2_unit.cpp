// Standalone, test-only facade over the exact pinned RE2 library sources.
// No EnergyPlus runtime, DLL, actuator, output fixture or Rust code is linked.
#include <cstdio>
#include <exception>
#include <iostream>
#include <string>

#ifdef _WIN32
#include <fcntl.h>
#include <io.h>
#endif

#include "re2/re2.h"
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

std::string encode_hex(const std::string& bytes) {
    static const char alphabet[] = "0123456789abcdef";
    std::string output;
    output.reserve(bytes.size() * 2);
    for (unsigned char value : bytes) {
        output.push_back(alphabet[value >> 4]);
        output.push_back(alphabet[value & 15]);
    }
    return output;
}
}  // namespace

int main(int argc, char**) {
    if (argc != 1) {
        std::cerr << "usage: re2_native_unit.exe < hex-pattern-and-text.tsv\n";
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
            // CR belongs only to CRLF framing; actual CR bytes are encoded 0d.
            if (!line.empty() && line.back() == '\r') line.pop_back();
            const std::size_t separator = line.find('\t');
            if (separator == std::string::npos ||
                line.find('\t', separator + 1) != std::string::npos) {
                std::cerr << "transport line " << ordinal << ": expected exactly one tab\n";
                return 2;
            }
            std::string pattern;
            std::string text;
            // Validate both byte strings before constructing an RE2 object.
            if (!decode_hex(line.substr(0, separator), pattern) ||
                !decode_hex(line.substr(separator + 1), text)) {
                std::cerr << "transport line " << ordinal << ": malformed hex\n";
                return 2;
            }
            const re2::StringPiece pattern_bytes(pattern.data(), pattern.size());
            const re2::StringPiece text_bytes(text.data(), text.size());
            // Quiet differs from DefaultOptions only in constructor error logging.
            // UTF8, case sensitivity, grammar and max_mem retain Native defaults.
            const re2::RE2::Options options(re2::RE2::Quiet);
            const re2::RE2 expression(pattern_bytes, options);
            // Call the actual API even for invalid expressions: no invented false.
            const bool matched = re2::RE2::FullMatch(text_bytes, expression);
            std::cout << (expression.ok() ? 1 : 0) << '\t'
                      << static_cast<int>(expression.error_code()) << '\t'
                      << encode_hex(expression.error()) << '\t'
                      << encode_hex(expression.error_arg()) << '\t'
                      << (matched ? 1 : 0) << '\n';
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
        std::cerr << "host exception at transport line " << ordinal << ": "
                  << error.what() << '\n';
        return 3;
    }
    return 0;
}
