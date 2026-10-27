//! Test-only flat JSONL observation of the actual Rust NativeAst result.
//! Match the sealed C++ observer schema; never reconstruct an expected AST.
//! Included only with cfg(test); never part of production matching.
#![cfg(test)]

#[test]
fn pinned_native_parse_ast_vectors() -> Result<(), String> {
    let mut rows = 0;
    for (ordinal, expected) in include_str!("../tests/data/native-parse-ast.jsonl")
        .lines()
        .enumerate()
    {
        let encoded = expected
            .strip_prefix("{\"schema\":\"pinned-re2-native-ast.v1\",\"pattern_hex\":\"")
            .and_then(|tail| tail.split_once('"').map(|(pattern, _)| pattern))
            .ok_or_else(|| format!("invalid AST fixture row {}", ordinal + 1))?;
        if !encoded.len().is_multiple_of(2) {
            return Err("odd AST fixture hex".into());
        }
        let bytes: Result<Vec<_>, _> = (0..encoded.len())
            .step_by(2)
            .map(|at| u8::from_str_radix(&encoded[at..at + 2], 16))
            .collect();
        let bytes = bytes.map_err(|error| error.to_string())?;
        let pattern = std::str::from_utf8(&bytes).map_err(|error| error.to_string())?;
        let parsed = crate::native_parser::parse(pattern);
        let actual = record(pattern, &parsed).map_err(|error| error.text())?;
        assert_eq!(
            actual.trim_end_matches('\n'),
            expected,
            "Native parse AST row {}",
            ordinal + 1
        );
        rows += 1;
    }
    assert_eq!(rows, 1_494);

    // One compact table retains actual Native outputs for both later phases.
    // Keep this in the same test so the raw payload writer has one fixture lane.
    let mut rewrite_rows = 0;
    let mut seen_phases = [false; 2];
    for (ordinal, row) in include_str!("../tests/data/native-rewrite.tsv")
        .lines()
        .enumerate()
    {
        let mut columns = row.splitn(3, '\t');
        let phase = columns
            .next()
            .ok_or_else(|| format!("missing rewrite phase at row {}", ordinal + 1))?;
        match phase {
            "required_prefix" => seen_phases[0] = true,
            "simplify" => seen_phases[1] = true,
            _ => return Err(format!("invalid rewrite phase at row {}", ordinal + 1)),
        }
        let encoded = columns
            .next()
            .ok_or_else(|| format!("missing rewrite pattern at row {}", ordinal + 1))?;
        let expected = columns
            .next()
            .ok_or_else(|| format!("missing rewrite observation at row {}", ordinal + 1))?;
        if !encoded.len().is_multiple_of(2) {
            return Err("odd rewrite fixture hex".into());
        }
        let bytes: Result<Vec<_>, _> = (0..encoded.len())
            .step_by(2)
            .map(|at| u8::from_str_radix(&encoded[at..at + 2], 16))
            .collect();
        let bytes = bytes.map_err(|error| error.to_string())?;
        let pattern = std::str::from_utf8(&bytes).map_err(|error| error.to_string())?;
        let actual = phase_record(pattern, phase).map_err(|error| error.text())?;
        assert_eq!(
            actual.trim_end_matches('\n'),
            expected,
            "Native {} rewrite row {}",
            phase,
            ordinal + 1
        );
        rewrite_rows += 1;
    }
    assert!(
        rewrite_rows > 0,
        "Native rewrite fixture must contain observations"
    );
    assert!(
        seen_phases.into_iter().all(|seen| seen),
        "Native rewrite fixture must cover both phases"
    );
    crate::native_program_snapshot::check_vectors()?;
    crate::native_automata_snapshot::check_vectors()?;
    Ok(())
}

use crate::native_ast::{Arena, Ast, Id, MAX_RUNE, Op, flags};
use crate::{ErrorCode, Status};
use std::collections::HashMap;
use std::fmt::{self, Write as _};
use std::sync::Arc;

fn internal() -> Status {
    Status::new(ErrorCode::InternalError, "")
}
fn allocation() -> Status {
    Status::new(ErrorCode::PatternTooLarge, "")
}

/// The only fmt::Write failure is allocation; a failed record is never returned.
#[derive(Default)]
pub(crate) struct Output(pub(crate) String);
impl fmt::Write for Output {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        self.0.try_reserve(text.len()).map_err(|_| fmt::Error)?;
        self.0.push_str(text);
        Ok(())
    }
}
macro_rules! emit {
    ($output:expr, $($argument:tt)*) => {
        write!($output, $($argument)*).map_err(|_| allocation())?
    };
}

pub(crate) fn hex(output: &mut Output, bytes: &[u8]) -> Result<(), Status> {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let extra = bytes.len().checked_mul(2).ok_or_else(allocation)?;
    output.0.try_reserve(extra).map_err(|_| allocation())?;
    for &byte in bytes {
        output.0.push(char::from(DIGITS[usize::from(byte >> 4)]));
        output.0.push(char::from(DIGITS[usize::from(byte & 15)]));
    }
    Ok(())
}

fn children(op: &Op) -> &[Id] {
    match op {
        Op::Concat(children) | Op::Alternate(children) => children,
        Op::Star(child)
        | Op::Plus(child)
        | Op::Quest(child)
        | Op::Repeat { child, .. }
        | Op::Capture { child, .. } => std::slice::from_ref(child),
        _ => &[],
    }
}

fn push<T>(values: &mut Vec<T>, value: T) -> Result<(), Status> {
    values.try_reserve(1).map_err(|_| allocation())?;
    values.push(value);
    Ok(())
}

fn emit_class(output: &mut Output, ranges: &[(u32, u32)]) -> Result<(), Status> {
    let mut rune_count = 0_u64;
    let mut previous: Option<u32> = None;
    emit!(output, "\"ranges\":[");
    for (index, &(lo, hi)) in ranges.iter().enumerate() {
        // Observe the stored finished set unchanged. A malformed arena is a
        // snapshot error, never normalized or turned into a parser outcome.
        if lo > hi || hi > MAX_RUNE || previous.is_some_and(|end| lo <= end + 1) {
            return Err(internal());
        }
        previous = Some(hi);
        rune_count += u64::from(hi) - u64::from(lo) + 1;
        if index != 0 {
            emit!(output, ",");
        }
        emit!(output, "[{lo},{hi}]");
    }
    // NativeAst stores the finished intervals, rather than redundant cached
    // CharClass cardinality/ASCII bitmaps. Derive these metadata predicates
    // from that actual set; do not add any fold-equivalent rune to the set.
    let contains = |rune: u32| ranges.iter().any(|&(lo, hi)| lo <= rune && rune <= hi);
    let folds_ascii =
        (u32::from(b'A')..=u32::from(b'Z')).all(|upper| contains(upper) == contains(upper + 32));
    emit!(
        output,
        "],\"rune_count\":{rune_count},\"folds_ascii\":{folds_ascii},\"empty\":{},\"full\":{}",
        rune_count == 0,
        rune_count == u64::from(MAX_RUNE) + 1
    );
    Ok(())
}

/// One raw node/payload serializer, optionally wrapped as a multi-root graph.
/// Single-root record() requests the original bare array wire representation.
pub(crate) fn emit_nodes(
    output: &mut Output,
    arena: &Arena,
    roots: &[Option<Id>],
    graph: bool,
) -> Result<(), Status> {
    let mut normalized: HashMap<Id, usize> = HashMap::new();
    let mut ordered: Vec<Id> = Vec::new();
    let mut pending: Vec<Id> = Vec::new();
    for &root in roots.iter().rev() {
        if let Some(root) = root {
            push(&mut pending, root)?;
        }
    }
    while let Some(id) = pending.pop() {
        if normalized.contains_key(&id) {
            continue;
        }
        let node = arena.node(id)?;
        normalized.try_reserve(1).map_err(|_| allocation())?;
        normalized.insert(id, ordered.len());
        push(&mut ordered, id)?;
        for &child in children(&node.op).iter().rev() {
            push(&mut pending, child)?;
        }
    }

    // Data addresses are internal hash keys only, never serialized. Explicit
    // Arc::ptr_eq verifies the name identity rule of the sealed Native schema.
    let mut names: HashMap<*const u8, (usize, &Arc<str>)> = HashMap::new();
    if graph {
        emit!(output, "{{\"roots\":[");
        for (index, root) in roots.iter().enumerate() {
            if index != 0 {
                emit!(output, ",");
            }
            if let Some(root) = root {
                emit!(
                    output,
                    "{}",
                    normalized.get(root).copied().ok_or_else(internal)?
                );
            } else {
                emit!(output, "null");
            }
        }
        emit!(output, "],\"nodes\":");
    }
    emit!(output, "[");
    for (identity, &id) in ordered.iter().enumerate() {
        let node = arena.node(id)?;
        if identity != 0 {
            emit!(output, ",");
        }
        emit!(
            output,
            "{{\"id\":{identity},\"op\":{},\"flags\":{},\"simple\":{},\"children\":[",
            node.op.opcode(),
            node.flags,
            node.simple
        );
        for (index, child) in children(&node.op).iter().enumerate() {
            if index != 0 {
                emit!(output, ",");
            }
            emit!(
                output,
                "{}",
                normalized.get(child).copied().ok_or_else(internal)?
            );
        }
        emit!(output, "],\"payload\":{{");
        match &node.op {
            Op::Literal(rune) => emit!(output, "\"rune\":{rune}"),
            Op::LiteralString(runes) => {
                emit!(output, "\"runes\":[");
                for (index, rune) in runes.iter().enumerate() {
                    if index != 0 {
                        emit!(output, ",");
                    }
                    emit!(output, "{rune}");
                }
                emit!(output, "]");
            }
            Op::Repeat { min, max, .. } => {
                emit!(output, "\"min\":{min},\"max\":");
                if let Some(max) = max {
                    emit!(output, "{max}");
                } else {
                    emit!(output, "-1");
                }
            }
            Op::Capture { index, name, .. } => {
                emit!(output, "\"index\":{index},\"name_hex\":");
                if let Some(name) = name {
                    let pointer = Arc::as_ptr(name).cast::<u8>();
                    let name_id = if let Some(&(identity, previous)) = names.get(&pointer) {
                        if !Arc::ptr_eq(name, previous) {
                            return Err(internal());
                        }
                        identity
                    } else {
                        let identity = names.len();
                        names.try_reserve(1).map_err(|_| allocation())?;
                        names.insert(pointer, (identity, name));
                        identity
                    };
                    emit!(output, "\"");
                    hex(output, name.as_bytes())?;
                    emit!(output, "\",\"name_id\":{name_id}");
                } else {
                    emit!(output, "null,\"name_id\":null");
                }
            }
            Op::Class(ranges) => emit_class(output, ranges)?,
            Op::ClassBuilder(_) => return Err(internal()),
            Op::HaveMatch(id) => emit!(output, "\"match_id\":{id}"),
            Op::NoMatch
            | Op::Empty
            | Op::Concat(_)
            | Op::Alternate(_)
            | Op::Star(_)
            | Op::Plus(_)
            | Op::Quest(_)
            | Op::AnyChar
            | Op::AnyByte
            | Op::BeginLine
            | Op::EndLine
            | Op::WordBoundary
            | Op::NotWordBoundary
            | Op::BeginText
            | Op::EndText => {}
        }
        emit!(output, "}}}}");
    }
    emit!(output, "]");
    if graph {
        emit!(output, "}}");
    }
    Ok(())
}

/// Serialize one actual default-parser result as the sealed Native schema.
/// The returned string includes its LF terminator. Errors from this writer are
/// transport/arena failures, not replacement statuses for the parse result.
pub(crate) fn record(pattern: &str, result: &Result<Ast, Status>) -> Result<String, Status> {
    let success = Status::new(ErrorCode::Success, "");
    let (ast, status) = match result {
        Ok(ast) => (Some(ast), &success),
        Err(status) => (None, status),
    };
    let mut output = Output::default();
    emit!(
        &mut output,
        "{{\"schema\":\"pinned-re2-native-ast.v1\",\"pattern_hex\":\""
    );
    hex(&mut output, pattern.as_bytes())?;
    emit!(
        &mut output,
        "\",\"input_parse_flags\":{},\"ok\":{},\"status\":{{\"ok\":{},\"code\":{},\"text_hex\":\"",
        flags::LIKE_PERL,
        ast.is_some(),
        status.code == ErrorCode::Success,
        status.code as u8
    );
    hex(&mut output, status.text().as_bytes())?;
    emit!(&mut output, "\",\"error_arg_hex\":\"");
    hex(&mut output, status.arg.as_bytes())?;
    emit!(&mut output, "\"}},\"root\":");
    if ast.is_some() {
        emit!(&mut output, "0");
    } else {
        emit!(&mut output, "null");
    }
    emit!(&mut output, ",\"nodes\":");
    if let Some(ast) = ast {
        emit_nodes(&mut output, &ast.arena, &[Some(ast.root)], false)?;
    } else {
        emit!(&mut output, "[]");
    }
    emit!(&mut output, "}}\n");
    Ok(output.0)
}

/// Serialize a fresh parse and exactly one selected Native rewrite operation.
/// Both pre-phase and combined post-phase graphs use emit_nodes' sole payload
/// switch. Result/original sharing is observed within the combined graph only.
pub(crate) fn phase_record(pattern: &str, phase: &str) -> Result<String, Status> {
    if !matches!(phase, "required_prefix" | "simplify") {
        return Err(internal());
    }
    let mut parsed = crate::native_parser::parse(pattern);
    let success = Status::new(ErrorCode::Success, "");
    let status = match &parsed {
        Ok(_) => &success,
        Err(status) => status,
    };
    let mut output = Output::default();
    emit!(
        &mut output,
        "{{\"schema\":\"pinned-re2-native-phase.v1\",\"pattern_hex\":\""
    );
    hex(&mut output, pattern.as_bytes())?;
    emit!(
        &mut output,
        "\",\"input_parse_flags\":{},\"phase\":\"{}\",\"parse\":{{\"ok\":{},\"status\":{{\"ok\":{},\"code\":{},\"text_hex\":\"",
        flags::LIKE_PERL,
        phase,
        parsed.is_ok(),
        status.code == ErrorCode::Success,
        status.code as u8
    );
    hex(&mut output, status.text().as_bytes())?;
    emit!(&mut output, "\",\"error_arg_hex\":\"");
    hex(&mut output, status.arg.as_bytes())?;
    emit!(&mut output, "\"}},\"graph\":");
    match &parsed {
        Ok(ast) => emit_nodes(&mut output, &ast.arena, &[Some(ast.root)], true)?,
        Err(_) => emit_nodes(&mut output, &Arena::default(), &[None], true)?,
    }
    emit!(&mut output, "}},\"stage\":{{");
    match &mut parsed {
        Err(_) => emit!(&mut output, "\"called\":false"),
        Ok(ast) => {
            let original = ast.root;
            let result = if phase == "required_prefix" {
                let prefix = ast.arena.required_prefix(original)?;
                emit!(
                    &mut output,
                    "\"called\":true,\"found\":{},\"prefix_hex\":\"",
                    prefix.found
                );
                hex(&mut output, &prefix.bytes)?;
                emit!(
                    &mut output,
                    "\",\"prefix_bytes\":{},\"foldcase\":{},",
                    prefix.bytes.len(),
                    prefix.foldcase
                );
                prefix.suffix
            } else {
                // This branch is Simplify on a fresh Parse, never a suffix
                // selected by RequiredPrefix in the same record.
                let result = ast.arena.simplify(original)?;
                emit!(&mut output, "\"called\":true,");
                Some(result)
            };
            emit!(
                &mut output,
                "\"result_present\":{},\"result_same_as_original\":{},\"graph\":",
                result.is_some(),
                result == Some(original)
            );
            emit_nodes(&mut output, &ast.arena, &[Some(original), result], true)?;
        }
    }
    emit!(&mut output, "}}}}\n");
    Ok(output.0)
}
