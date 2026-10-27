//! Test-only actual finalized-program wire observation. No expected graph enters compilation.
#![cfg(test)]
use crate::native_ast::{Arena, Id, flags};
use crate::native_ast_snapshot::{Output, emit_nodes, hex};
use crate::{ErrorCode, Status};
use std::fmt::Write as _;
fn internal() -> Status {
    Status::new(ErrorCode::InternalError, "")
}
fn allocation() -> Status {
    Status::new(ErrorCode::PatternTooLarge, "")
}
macro_rules! emit {
    ($output:expr, $($argument:tt)*) => { write!($output, $($argument)*).map_err(|_| allocation())? };
}
fn emit_graph(output: &mut Output, arena: &Arena, roots: &[Option<Id>]) -> Result<(), Status> {
    emit_nodes(output, arena, roots, true)
}
use crate::native_compiler::{InstOp, NativeLayout};
use crate::native_finalize::{FinalizeLayout, ListsAndByteMap};

const INST_OPS: [InstOp; 8] = [
    InstOp::Alt,
    InstOp::AltMatch,
    InstOp::ByteRange,
    InstOp::Capture,
    InstOp::EmptyWidth,
    InstOp::Match,
    InstOp::Nop,
    InstOp::Fail,
];
// Public pinned prog.h Inst::kMaxInst, also used by the exact Rust Limits port.
const INST_MAX_ID: u32 = (1 << 28) - 1;

/// Explicit observed TARGET C++ ABI, never Rust struct sizes or guessed defaults.
#[derive(Clone, Copy, Debug)]
pub(crate) struct TargetAbi {
    pub prog_bytes: u64,
    pub inst_bytes: u64,
    pub uint16_bytes: u64,
    pub size_t_bytes: u8,
    pub pointer_bytes: u8,
}
impl TargetAbi {
    pub(crate) fn validate(self) -> Result<(), Status> {
        if self.prog_bytes == 0
            || self.inst_bytes != 8
            || self.uint16_bytes != 2
            || !matches!(self.size_t_bytes, 4 | 8)
            || self.pointer_bytes == 0
        {
            return Err(internal());
        }
        Ok(())
    }
    fn native(self) -> NativeLayout {
        NativeLayout {
            prog_bytes: self.prog_bytes,
            inst_bytes: self.inst_bytes,
        }
    }
    fn finalize(self) -> FinalizeLayout {
        FinalizeLayout {
            native: self.native(),
            list_head_bytes: self.uint16_bytes,
            size_t_bytes: self.size_t_bytes,
        }
    }
}

/// The wire record follows the Native schema. Rust compiler failures, if any,
/// remain separately labelled diagnostics, not invented Native error codes.
pub(crate) struct Observation {
    pub record: String,
    pub compiler_diagnostic: Option<String>,
}
struct CompilerFailure {
    phase: &'static str,
    status: Status,
}

fn compile(
    arena: &mut Arena,
    root: Id,
    abi: TargetAbi,
) -> Result<ListsAndByteMap, CompilerFailure> {
    let simplified = arena.simplify(root).map_err(|status| CompilerFailure {
        phase: "simplify",
        status,
    })?;
    let front =
        crate::native_compiler::compile_forward_simplified(arena, root, simplified, abi.native())
            .map_err(|status| CompilerFailure {
            phase: "frontend",
            status,
        })?;
    crate::native_finalize::finalize(front, abi.finalize()).map_err(|status| CompilerFailure {
        phase: "finalize",
        status,
    })
}

fn compiler_diagnostic(pattern: &str, failure: &CompilerFailure) -> Result<String, Status> {
    let mut output = Output::default();
    emit!(
        &mut output,
        "{{\"schema\":\"rust-program-adapter-diagnostic.v1\",\"pattern_hex\":\""
    );
    hex(&mut output, pattern.as_bytes())?;
    emit!(
        &mut output,
        "\",\"phase\":\"{}\",\"Rust_status\":{{\"code\":{},\"text_hex\":\"",
        failure.phase,
        failure.status.code as u8
    );
    hex(&mut output, failure.status.text().as_bytes())?;
    emit!(&mut output, "\",\"arg_hex\":\"");
    hex(&mut output, failure.status.arg.as_bytes())?;
    emit!(
        &mut output,
        "\"}},\"qualification\":\"Rust pipeline Result::Err; no Native error-code assertion\"}}\n"
    );
    Ok(output.0)
}

fn emit_program(
    output: &mut Output,
    image: &ListsAndByteMap,
    can_prefix_accel: bool,
) -> Result<(), Status> {
    emit!(
        output,
        "{{\"size\":{},\"start\":{},\"start_unanchored\":{},\"reversed\":{},\"anchor_start\":{},\"anchor_end\":{},\"list_count\":{},\"inst_count\":[",
        image.instructions.len(),
        image.start,
        image.start_unanchored,
        image.reversed,
        image.anchor_start,
        image.anchor_end,
        image.list_count
    );
    for (index, count) in image.inst_count.iter().enumerate() {
        if index != 0 {
            emit!(output, ",");
        }
        emit!(output, "{count}");
    }
    emit!(
        output,
        "],\"can_bit_state\":{},\"bit_state_text_max_size\":{},\"dfa_mem\":{},\"can_prefix_accel\":{},\"bytemap_range\":{},\"bytemap\":[",
        image.list_heads.is_some(),
        image.bit_state_text_max_size,
        image.dfa_mem,
        can_prefix_accel,
        image.bytemap_range
    );
    for (index, byte) in image.bytemap.iter().enumerate() {
        if index != 0 {
            emit!(output, ",");
        }
        emit!(output, "{byte}");
    }
    emit!(output, "],\"list_heads\":");
    if let Some(heads) = &image.list_heads {
        emit!(output, "[");
        for (index, head) in heads.iter().enumerate() {
            if index != 0 {
                emit!(output, ",");
            }
            emit!(output, "{head}");
        }
        emit!(output, "]");
    } else {
        emit!(output, "null");
    }
    emit!(output, ",\"instructions\":[");
    for (index, instruction) in image.instructions.iter().copied().enumerate() {
        let id = u32::try_from(index).map_err(|_| internal())?;
        if index != 0 {
            emit!(output, ",");
        }
        emit!(
            output,
            "{{\"id\":{id},\"opcode\":{},\"last\":{},\"out\":{},\"payload\":{{",
            instruction.opcode() as u8,
            u8::from(instruction.last()),
            instruction.out()
        );
        match instruction.opcode() {
            InstOp::Alt => emit!(output, "\"out1\":{}", instruction.payload),
            InstOp::AltMatch => emit!(
                output,
                "\"out1\":{},\"greedy\":{}",
                instruction.payload,
                image.alt_match_greedy(id)?
            ),
            InstOp::ByteRange => emit!(
                output,
                "\"lo\":{},\"hi\":{},\"foldcase\":{},\"hint\":{}",
                instruction.lo(),
                instruction.hi(),
                u8::from(instruction.foldcase()),
                instruction.hint()
            ),
            InstOp::Capture => emit!(output, "\"cap\":{}", instruction.payload as i32),
            InstOp::EmptyWidth => emit!(output, "\"empty\":{}", instruction.payload),
            InstOp::Match => emit!(output, "\"match_id\":{}", instruction.payload as i32),
            InstOp::Nop | InstOp::Fail => {}
        }
        emit!(output, "}}}}");
    }
    emit!(output, "]}}");
    Ok(())
}

/// Execute only the actual Rust compiler pipeline, then observe its raw result.
/// No Native process, baseline output, fixture lookup or matcher is reachable.
pub(crate) fn record(pattern: &str, abi: TargetAbi) -> Result<Observation, Status> {
    abi.validate()?;
    let mut parsed = crate::native_parser::parse(pattern);
    let success = Status::new(ErrorCode::Success, "");
    let status = match &parsed {
        Ok(_) => &success,
        Err(status) => status,
    };
    let mut output = Output::default();
    emit!(
        &mut output,
        "{{\"schema\":\"pinned-re2-native-program.v1\",\"pattern_hex\":\""
    );
    hex(&mut output, pattern.as_bytes())?;
    emit!(
        &mut output,
        "\",\"input_parse_flags\":{},\"default_max_mem\":{},\"compile_max_mem\":{},\"abi\":{{\"sizeof_prog\":{},\"sizeof_inst\":{},\"sizeof_uint16\":{},\"sizeof_size_t\":{},\"sizeof_pointer\":{},\"inst_opcode_count\":{},\"inst_max_id\":{}}},\"parse\":{{\"ok\":{},\"status\":{{\"ok\":{},\"code\":{},\"text_hex\":\"",
        flags::LIKE_PERL,
        crate::native_compiler::DEFAULT_MAX_MEM,
        crate::native_compiler::DEFAULT_FORWARD_MAX_MEM,
        abi.prog_bytes,
        abi.inst_bytes,
        abi.uint16_bytes,
        abi.size_t_bytes,
        abi.pointer_bytes,
        INST_OPS.len(),
        INST_MAX_ID,
        parsed.is_ok(),
        status.code == ErrorCode::Success,
        status.code as u8
    );
    hex(&mut output, status.text().as_bytes())?;
    emit!(&mut output, "\",\"error_arg_hex\":\"");
    hex(&mut output, status.arg.as_bytes())?;
    emit!(&mut output, "\"}}}},\"required_prefix\":{{");
    let mut diagnostic = None;
    match &mut parsed {
        Err(_) => {
            emit!(&mut output, "\"called\":false}},\"compile_input_graph\":");
            emit_graph(&mut output, &Arena::default(), &[None, None])?;
            emit!(
                &mut output,
                ",\"compile\":{{\"called\":false,\"present\":false,\"program\":null}}"
            );
        }
        Ok(ast) => {
            let original = ast.root;
            let prefix = ast.arena.required_prefix(original)?;
            let selected = if prefix.found {
                prefix.suffix.ok_or_else(internal)?
            } else {
                original
            };
            emit!(
                &mut output,
                "\"called\":true,\"found\":{},\"prefix_hex\":\"",
                prefix.found
            );
            hex(&mut output, &prefix.bytes)?;
            emit!(
                &mut output,
                "\",\"prefix_bytes\":{},\"foldcase\":{},\"returned_suffix_present\":{}}},\"compile_input_graph\":",
                prefix.bytes.len(),
                prefix.foldcase,
                prefix.suffix.is_some()
            );
            // This sole raw-AST emitter observes actual chosen input BEFORE
            // Simplify/frontend/finalizer can alter any phase-sensitive caches.
            emit_graph(&mut output, &ast.arena, &[Some(original), Some(selected)])?;
            let compiled = compile(&mut ast.arena, selected, abi);
            emit!(
                &mut output,
                ",\"compile\":{{\"called\":true,\"present\":{},\"program\":",
                compiled.is_ok()
            );
            match compiled {
                Err(failure) => {
                    emit!(&mut output, "null");
                    diagnostic = Some(compiler_diagnostic(pattern, &failure)?);
                }
                Ok(image) => {
                    // Source Finish analyzes the ORIGINAL CompileToProg argument,
                    // not the simplified or anchor-stripped root. Native public
                    // can_prefix_accel() means configured prefix_size_ !=0;
                    // FoldCase clips to at most9 bytes without changing nonempty.
                    // This observes only that source-defined scalar, not a cache.
                    let accel = ast
                        .arena
                        .required_prefix_for_accel(image.original_compile_input_root)?;
                    let can_prefix_accel = accel.found && !accel.bytes.is_empty();
                    emit_program(&mut output, &image, can_prefix_accel)?;
                }
            }
            emit!(&mut output, "}}");
        }
    }
    emit!(&mut output, "}}\n");
    Ok(Observation {
        record: output.0,
        compiler_diagnostic: diagnostic,
    })
}

/// Called from the existing AST table test; no extra test function or actor.
pub(crate) fn check_vectors() -> Result<(), String> {
    // Actual explicit GCC source-unit ABI. This is test metadata only; the
    // official constructor's separately observed cap is NOT inferred here.
    let abi = TargetAbi {
        prog_bytes: 432,
        inst_bytes: 8,
        uint16_bytes: 2,
        size_t_bytes: 8,
        pointer_bytes: 8,
    };
    let mut count = 0;
    for (ordinal, row) in include_str!("../tests/data/native-program.tsv")
        .lines()
        .enumerate()
    {
        let (encoded, expected) = row
            .split_once('\t')
            .ok_or("missing Native program fixture tab")?;
        if !encoded.len().is_multiple_of(2) {
            return Err("odd Native program fixture hex".into());
        }
        let bytes: Result<Vec<_>, _> = (0..encoded.len())
            .step_by(2)
            .map(|at| u8::from_str_radix(&encoded[at..at + 2], 16))
            .collect();
        let bytes = bytes.map_err(|error| error.to_string())?;
        let pattern = std::str::from_utf8(&bytes).map_err(|error| error.to_string())?;
        let observed = record(pattern, abi).map_err(|error| error.text())?;
        assert!(
            observed.compiler_diagnostic.is_none(),
            "Unexpected Rust compiler failure at Native program row {}",
            ordinal + 1
        );
        assert_eq!(
            observed.record.trim_end_matches('\n'),
            expected,
            "Native finalized program row {}",
            ordinal + 1
        );
        count += 1;
    }
    assert_eq!(count, 23);
    Ok(())
}
