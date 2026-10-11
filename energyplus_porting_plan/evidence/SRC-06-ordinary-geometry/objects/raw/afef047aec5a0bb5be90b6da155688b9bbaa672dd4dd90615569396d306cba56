"""Strict PE32+ maps for prospective debug-only native helper derivatives.

No engine, compiler, transformation or deletion is performed by this reader.
It preserves complete retained raw sections; no executable bytes are masked.
"""
from __future__ import annotations

import hashlib
import mmap
from pathlib import Path
import struct


def require(condition, message):
    if not condition:
        raise ValueError(message)


def file_sha(path):
    digest = hashlib.sha256()
    with Path(path).open("rb") as stream:
        while chunk := stream.read(16 * 1024 * 1024):
            digest.update(chunk)
    return digest.hexdigest()


def align(value, alignment):
    require(alignment > 0 and alignment & (alignment - 1) == 0, "Invalid PE alignment")
    return (value + alignment - 1) // alignment * alignment


def runtime_references(data, mapping):
    """Walk the observed GNU x64 helper's loader pointers; reject other layouts.

    Layout sources:
    https://learn.microsoft.com/en-us/windows/win32/debug/pe-format
    https://learn.microsoft.com/en-us/cpp/build/exception-handling-x64
    This is a bounded structural check, not a disassembler or execution proof.
    Handler-specific data is opaque here; its complete containing raw section
    must still be byte-identical in compare(). No complete pointer-walk claim.
    """
    sections, directories = mapping["sections"], mapping["data_directories"]
    base = mapping["image_base"]
    counts = {}
    targets = {}

    def resolve(rva, size, kind, raw=True, executable=False):
        require(size > 0, "Empty runtime pointer extent")
        matches = [row for row in sections if row["rva"] <= rva and rva + size <= row["rva"]
                   + (row["raw_size"] if raw else max(row["virtual_size"], row["raw_size"]))]
        require(len(matches) == 1 and not matches[0]["debug"], "Runtime pointer targets removed/unmapped section: " + kind)
        row = matches[0]
        require(not executable or row["characteristics"] & 0x20000000,
                "Runtime code pointer targets non-executable section: " + kind)
        counts[kind] = counts.get(kind, 0) + 1
        names = targets.setdefault(kind, set())
        names.add(row["name"])
        return row["raw_offset"] + rva - row["rva"]

    def unpack(fmt, rva, kind):
        return struct.unpack_from(fmt, data, resolve(rva, struct.calcsize(fmt), kind))

    def cstring(rva, kind):
        offset = resolve(rva, 1, kind)
        row = next(row for row in sections if row["rva"] <= rva < row["rva"] + row["raw_size"])
        end = data.find(b"\0", offset, row["raw_offset"] + row["raw_size"])
        require(end >= offset, "Unterminated runtime string: " + kind)
        return end - offset + 1

    # The first audited static helper has only imports, exceptions, relocations,
    # TLS and IAT. New nonzero directory kinds need their own reviewed walker.
    require(all(row["size"] == 0 for row in directories if row["index"] not in (1, 3, 5, 9, 12)),
            "Unsupported nonzero runtime directory for first derivative")
    resolve(mapping["entry_rva"], 1, "entry", executable=True)
    imports = directories[1]
    if imports["size"]:
        require(imports["size"] >= 20, "Short import directory")
        found_end = False
        for relative in range(0, imports["size"] - 19, 20):
            lookup, timestamp, chain, name, iat = unpack("<IIIII", imports["rva"] + relative, "import_descriptor")
            if (lookup, timestamp, chain, name, iat) == (0, 0, 0, 0, 0):
                found_end = True
                break
            require(timestamp == 0 and lookup and name and iat, "Bound/unsupported import descriptor")
            cstring(name, "import_dll")
            # Bound by available raw payload, not an arbitrary symbol count.
            row = next(row for row in sections if row["rva"] <= lookup < row["rva"] + row["raw_size"])
            limit = (row["rva"] + row["raw_size"] - lookup) // 8
            terminated = False
            for index in range(limit):
                value, = unpack("<Q", lookup + index * 8, "import_lookup")
                actual, = unpack("<Q", iat + index * 8, "import_iat")
                require(value == actual, "Prebound/unequal import lookup and IAT")
                require(directories[12]["rva"] <= iat + index * 8 and iat + (index + 1) * 8
                        <= directories[12]["rva"] + directories[12]["size"], "Import IAT outside IAT directory")
                if value == 0:
                    terminated = True
                    break
                if value & (1 << 63):
                    require(value & ~((1 << 63) | 0xffff) == 0, "Malformed ordinal import")
                else:
                    require(value <= 0xffffffff, "Invalid import-name RVA")
                    resolve(value, 2, "import_hint")
                    cstring(value + 2, "import_name")
            require(terminated, "Unterminated import lookup")
        require(found_end, "Unterminated import descriptors")

    exceptions = directories[3]
    seen_unwind = set()
    opaque_handlers = set()

    def function(begin, end, unwind):
        require(end > begin, "Invalid runtime-function extent")
        resolve(begin, end - begin, "runtime_function", executable=True)
        if unwind in seen_unwind:
            return
        seen_unwind.add(unwind)
        version_flags, prologue, codes, frame = unpack("<BBBB", unwind, "unwind_header")
        version, flags = version_flags & 7, version_flags >> 3
        require(version == 1 and flags & ~7 == 0 and not (flags & 4 and flags & 3),
                "Unsupported x64 unwind flags/version")
        extent = align(4 + codes * 2, 4)
        resolve(unwind, extent, "unwind_codes")
        if flags & 4:
            function(*unpack("<III", unwind + extent, "unwind_chain"))
        elif flags & 3:
            handler, = unpack("<I", unwind + extent, "unwind_handler_rva")
            resolve(handler, 1, "unwind_handler", executable=True)
            opaque_handlers.add(unwind)

    if exceptions["size"]:
        require(exceptions["size"] % 12 == 0, "Malformed x64 runtime-function directory")
        last = -1
        for relative in range(0, exceptions["size"], 12):
            begin, end, unwind = unpack("<III", exceptions["rva"] + relative, "exception_entry")
            require(begin >= last, "Unsorted runtime-function directory")
            last = begin
            function(begin, end, unwind)

    tls = directories[9]
    if tls["size"]:
        require(tls["size"] == 40, "Unexpected PE32+ TLS directory size")
        start, end, index, callbacks, zero_fill, flags = unpack("<QQQQII", tls["rva"], "tls_directory")
        require(base <= start < end, "Invalid TLS template VAs")
        resolve(start - base, end - start, "tls_template")
        require(index >= base, "Invalid TLS index VA")
        resolve(index - base, 4, "tls_index", raw=False)
        if callbacks:
            require(callbacks >= base, "Invalid TLS callback table VA")
            callback_rva = callbacks - base
            row = next(row for row in sections if row["rva"] <= callback_rva < row["rva"] + row["raw_size"])
            limit = (row["rva"] + row["raw_size"] - callback_rva) // 8
            terminated = False
            for index in range(limit):
                callback, = unpack("<Q", callback_rva + index * 8, "tls_callback_table")
                if callback == 0:
                    terminated = True
                    break
                require(callback >= base, "Invalid TLS callback VA")
                resolve(callback - base, 1, "tls_callback", executable=True)
            require(terminated, "Unterminated TLS callback table")

    relocations = directories[5]
    relative = 0
    if relocations["size"]:
        while relative < relocations["size"]:
            require(relative + 8 <= relocations["size"], "Truncated relocation block")
            page, size = unpack("<II", relocations["rva"] + relative, "relocation_block")
            require(size >= 8 and size % 2 == 0 and relative + size <= relocations["size"], "Invalid relocation block size")
            for offset in range(8, size, 2):
                item, = unpack("<H", relocations["rva"] + relative + offset, "relocation_entry")
                kind, displacement = item >> 12, item & 0xfff
                require(kind in (0, 10), "Unsupported x64 relocation kind")
                if kind:
                    value, = unpack("<Q", page + displacement, "relocation_slot")
                    if value == base:
                        # The GNU image-base anchor is an address, not a read of
                        # one byte from a section. Keep the exact VA/header base.
                        counts["relocated_image_base_anchor"] = counts.get("relocated_image_base_anchor", 0) + 1
                    elif base < value < base + mapping["image_size"]:
                        resolve(value - base, 1, "relocated_image_pointer", raw=False)
            relative += size
    return {"schema": "native-helper-loader-reference-walk.v1", "scope": "GNU-x64-unbound-imports-unwind-v1-TLS-DIR64",
            "counts": counts, "target_sections": {key: sorted(value) for key, value in targets.items()},
            "all_walked_targets_retained": True, "complete_runtime_pointer_walk": False,
            "opaque_language_specific_handler_records": len(opaque_handlers),
            "opaque_handler_data_preservation": "Exact full containing-section payload comparison; not interpreted",
            "disassembly_or_scientific_execution": False}


def inspect(path):
    path = Path(path).resolve()
    before = path.stat()
    digest = file_sha(path)
    with path.open("rb") as stream, mmap.mmap(stream.fileno(), 0, access=mmap.ACCESS_READ) as data:
        def bounded(offset, size):
            require(offset >= 0 and size >= 0 and offset + size <= len(data), "PE extent outside file")
            return data[offset:offset + size]

        def number(fmt, offset):
            return struct.unpack(fmt, bounded(offset, struct.calcsize(fmt)))[0]

        require(bounded(0, 2) == b"MZ", "Missing DOS signature")
        pe = number("<I", 0x3c)
        require(bounded(pe, 4) == b"PE\0\0", "Missing PE signature")
        coff = pe + 4
        machine, count, stamp, symbols, symbol_count, opt_size, flags = struct.unpack(
            "<HHIIIHH", bounded(coff, 20))
        opt = coff + 20
        require(machine == 0x8664 and count > 0 and opt_size == 240, "Only fixed PE32+ x86-64 layout supported")
        require(number("<H", opt) == 0x20b and number("<I", opt + 108) == 16,
                "Unexpected PE32+ optional header")
        header_size = number("<I", opt + 60)
        require(opt + opt_size + count * 40 <= header_size <= len(data), "Invalid PE header extent")
        string_start = symbols + symbol_count * 18 if symbols else 0
        string_size = number("<I", string_start) if string_start else 0
        if string_start:
            require(string_size >= 4, "Invalid COFF string table")
            bounded(string_start, string_size)
        sections = []
        for index in range(count):
            row = opt + opt_size + index * 40
            name = bounded(row, 8).rstrip(b"\0").decode("ascii")
            if name.startswith("/"):
                relative = int(name[1:])
                require(string_start and 4 <= relative < string_size, "Invalid long PE section name")
                start = string_start + relative
                end = data.find(b"\0", start, string_start + string_size)
                require(end >= start, "Unterminated long PE section name")
                name = bounded(start, end - start).decode("ascii")
            virtual_size, rva, raw_size, raw_offset, reloc, line_numbers, reloc_count, line_count, characteristics = struct.unpack(
                "<IIIIIIHHI", bounded(row + 8, 32))
            require(reloc == line_numbers == reloc_count == line_count == 0,
                    "Unexpected linked section relocations/line-number table")
            payload = bounded(raw_offset, raw_size) if raw_size else b""
            debug = name.startswith(".debug")
            if debug:
                require(not characteristics & (0x20000000 | 0x80000000), "Executable/writable debug section")
            sections.append({"name": name, "rva": rva, "virtual_size": virtual_size,
                "raw_size": raw_size, "raw_offset": raw_offset, "characteristics": characteristics,
                "debug": debug, "raw_sha256": hashlib.sha256(payload).hexdigest()})
        require(len({row["name"] for row in sections}) == len(sections), "Duplicate PE section names")
        section_alignment = number("<I", opt + 32)
        file_alignment = number("<I", opt + 36)
        align(0, section_alignment)
        align(0, file_alignment)
        for row in sections:
            require(row["rva"] % section_alignment == 0, "Misaligned section RVA")
            if row["raw_size"]:
                require(row["raw_offset"] >= header_size and row["raw_offset"] % file_alignment == 0
                        and row["raw_size"] % file_alignment == 0, "Invalid aligned raw section extent")
        for previous, current in zip(sections, sections[1:]):
            require(previous["rva"] + max(previous["virtual_size"], previous["raw_size"]) <= current["rva"],
                    "Overlapping/reordered PE virtual sections")
        raw_rows = sorted((row for row in sections if row["raw_size"]), key=lambda row: row["raw_offset"])
        for previous, current in zip(raw_rows, raw_rows[1:]):
            require(previous["raw_offset"] + previous["raw_size"] <= current["raw_offset"],
                    "Overlapping PE raw sections")

        directories = []
        for index in range(16):
            rva, size = struct.unpack("<II", bounded(opt + 112 + index * 8, 8))
            require((rva == 0) == (size == 0), "Partially empty PE directory")
            require(index not in (4, 6) or rva == size == 0,
                    "Signed/nonzero-debug-directory PE is outside first derivative scope")
            item = {"index": index, "rva": rva, "size": size, "section": None,
                    "section_relative_offset": None, "payload_sha256": None}
            if rva:
                matches = [row for row in sections if row["rva"] <= rva
                           and rva + size <= row["rva"] + row["raw_size"]]
                require(len(matches) == 1 and not matches[0]["debug"], "Runtime directory points outside retained raw data")
                row = matches[0]
                relative = rva - row["rva"]
                item.update(section=row["name"], section_relative_offset=relative,
                            payload_sha256=hashlib.sha256(bounded(row["raw_offset"] + relative, size)).hexdigest())
            directories.append(item)
        result = {"sha256": digest, "size_bytes": before.st_size, "pe_offset": pe,
            "dos_prefix_sha256": hashlib.sha256(bounded(0, pe)).hexdigest(),
            "machine": machine, "section_count": count, "timestamp": stamp,
            "symbol_table_offset": symbols, "symbol_count": symbol_count,
            "optional_header_size": opt_size, "coff_characteristics": flags,
            "complete_header_hex": bounded(0, header_size).hex(),
            "optional_header_hex": bounded(opt, opt_size).hex(),
            "entry_rva": number("<I", opt + 16), "image_base": number("<Q", opt + 24),
            "section_alignment": section_alignment, "file_alignment": file_alignment,
            "code_size": number("<I", opt + 4), "initialized_data_size": number("<I", opt + 8),
            "uninitialized_data_size": number("<I", opt + 12),
            "image_size": number("<I", opt + 56), "header_size": header_size,
            "checksum": number("<I", opt + 64), "sections": sections, "data_directories": directories}
        result["runtime_references"] = runtime_references(data, result)
    after = path.stat()
    require((before.st_size, before.st_mtime_ns) == (after.st_size, after.st_mtime_ns)
            and file_sha(path) == digest, "PE input changed while mapped")
    return result


def compare(before, after):
    """Require exact runtime sections, directories and narrowly explained headers."""
    for key in ("pe_offset", "dos_prefix_sha256", "machine", "timestamp", "optional_header_size"):
        require(before[key] == after[key], "Unexpected PE header change: " + key)
    require((before["coff_characteristics"] ^ after["coff_characteristics"]) & ~0x020c == 0,
            "Unexpected executable/relocation COFF flag change")
    old_runtime = [row for row in before["sections"] if not row["debug"]]
    new_runtime = [row for row in after["sections"] if not row["debug"]]
    removed = [row for row in before["sections"] if row["debug"]]
    require(removed and not any(row["debug"] for row in after["sections"]),
            "Expected removal of all existing debug sections only")
    require(len(old_runtime) == len(new_runtime), "Retained section count changed")
    moved = []
    for old, new in zip(old_runtime, new_runtime):
        require({key: value for key, value in old.items() if key != "raw_offset"}
                == {key: value for key, value in new.items() if key != "raw_offset"},
                "Retained PE section changed: " + old["name"])
        if old["raw_offset"] != new["raw_offset"]:
            moved.append({"name": old["name"], "before": old["raw_offset"], "after": new["raw_offset"]})
    require(before["data_directories"] == after["data_directories"], "Runtime PE directory changed")
    require(before["runtime_references"] == after["runtime_references"], "Runtime PE references changed")
    old_optional = bytearray.fromhex(before["optional_header_hex"])
    new_optional = bytearray.fromhex(after["optional_header_hex"])
    for start in (8, 56, 60, 64):
        old_optional[start:start + 4] = new_optional[start:start + 4] = bytes(4)
    require(old_optional == new_optional, "Unknown optional-header change")
    debug_initialized = sum(row["raw_size"] for row in removed if row["characteristics"] & 0x40)
    retained_initialized = sum(row["raw_size"] for row in old_runtime if row["characteristics"] & 0x40)
    require(before["initialized_data_size"] in (retained_initialized, retained_initialized + debug_initialized)
            and after["initialized_data_size"] == retained_initialized,
            "Initialized-data aggregate is not exact retained-only/all-section accounting")
    last_end = max(row["rva"] + max(row["virtual_size"], row["raw_size"]) for row in old_runtime)
    retained_image_size = align(last_end, before["section_alignment"])
    require(after["image_size"] in (before["image_size"], retained_image_size)
            and after["image_size"] >= retained_image_size, "Unexplained/inadequate image size")
    required_headers = align(before["pe_offset"] + 24 + before["optional_header_size"]
                             + after["section_count"] * 40, before["file_alignment"])
    require(after["header_size"] in (before["header_size"], required_headers), "Unexplained PE header size")
    require(after["size_bytes"] < before["size_bytes"], "Derivative did not reduce size")
    return {"status": "pass-exact-retained-runtime-sections-and-directories",
        "retained_section_count": len(old_runtime), "removed_debug_sections": removed,
        "removed_debug_raw_bytes": sum(row["raw_size"] for row in removed),
        "initialized_data_accounting": {"retained_initialized_raw_bytes": retained_initialized,
            "removed_debug_initialized_raw_bytes": debug_initialized,
            "original_included_debug": before["initialized_data_size"] != retained_initialized},
        "raw_file_offset_changes": moved,
        "explained_header_changes": {key: {"before": before[key], "after": after[key]}
            for key in ("section_count", "symbol_table_offset", "symbol_count", "coff_characteristics",
                        "initialized_data_size", "image_size", "header_size", "checksum") if before[key] != after[key]},
        "code_or_data_masked": False, "scientific_execution_certified": False}
