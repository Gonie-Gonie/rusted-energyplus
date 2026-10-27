//! Test-only public OnePass/DFA wire observation, copied from the actual matched source unit.
#![cfg(test)]
use crate::native_ast_snapshot::Output;
use crate::{
    ErrorCode, Status, native_ast, native_compiler, native_dfa, native_finalize, native_onepass,
    native_parser,
};
use std::fmt::Write as _;
macro_rules! emit {
    ($out:expr, $($arg:tt)*) => { write!($out, $($arg)*).map_err(|_| "observer serialization allocation".to_owned())? };
}
fn phase_error(status: Status) -> String {
    format!("Rust component observation error: {}", status.text())
}
fn hex(out: &mut Output, bytes: &[u8]) -> Result<(), String> {
    crate::native_ast_snapshot::hex(out, bytes).map_err(phase_error)
}
#[derive(Clone, Copy)]
struct Layout {
    prog: u64,
    inst: u64,
    int: u64,
    uint32: u32,
    uint16: u64,
    size_t: u8,
    pointer: u8,
    one_state: u32,
    dfa: u64,
    dfa_state: u64,
    atomic_state_pointer: u64,
}
struct Query {
    context: Vec<u8>,
    begin: usize,
    end: usize,
}
fn record(pattern: &str, queries: &[Query], layout: Layout) -> Result<String, String> {
    let parsed = native_parser::parse(pattern);
    let parse_ok = parsed.is_ok();
    let status = parsed
        .as_ref()
        .err()
        .cloned()
        .unwrap_or_else(|| Status::new(ErrorCode::Success, ""));
    let mut prefix = None;
    let mut image = None;
    if let Ok(mut ast) = parsed {
        let selected = ast.arena.required_prefix(ast.root).map_err(phase_error)?;
        let suffix = if selected.found {
            selected
                .suffix
                .ok_or("found prefix without actual suffix")?
        } else {
            ast.root
        };
        prefix = Some(selected);
        let compilation = ast.arena.simplify(suffix).and_then(|simplified| {
            let front = native_compiler::compile_forward_with_budget(
                &mut ast.arena,
                suffix,
                simplified,
                native_compiler::NativeLayout {
                    prog_bytes: layout.prog,
                    inst_bytes: layout.inst,
                },
                native_compiler::DEFAULT_FORWARD_MAX_MEM,
            )?;
            native_finalize::finalize(
                front,
                native_finalize::FinalizeLayout {
                    native: native_compiler::NativeLayout {
                        prog_bytes: layout.prog,
                        inst_bytes: layout.inst,
                    },
                    list_head_bytes: layout.uint16,
                    size_t_bytes: layout.size_t,
                },
            )
        });
        match compilation {
            Ok(program) => image = Some(program),
            Err(status) if status.code == ErrorCode::PatternTooLarge => {}
            Err(status) => return Err(phase_error(status)),
        }
    }
    let mut out = Output::default();
    emit!(
        out,
        "{{\"schema\":\"pinned-re2-native-onepass-dfa.v1\",\"pattern_hex\":\""
    );
    hex(&mut out, pattern.as_bytes())?;
    emit!(
        out,
        "\",\"input_parse_flags\":{},\"default_max_mem\":{},\"compile_max_mem\":{},\"abi\":{{\"sizeof_prog\":{},\"sizeof_inst\":{},\"sizeof_int\":{},\"sizeof_uint32\":{},\"sizeof_uint16\":{},\"sizeof_size_t\":{},\"sizeof_pointer\":{}}},\"parse\":{{\"ok\":{},\"status\":{{\"ok\":{},\"code\":{},\"text_hex\":\"",
        native_ast::flags::LIKE_PERL,
        native_compiler::DEFAULT_MAX_MEM,
        native_compiler::DEFAULT_FORWARD_MAX_MEM,
        layout.prog,
        layout.inst,
        layout.int,
        layout.uint32,
        layout.uint16,
        layout.size_t,
        layout.pointer,
        parse_ok,
        status.code == ErrorCode::Success,
        status.code as u8
    );
    hex(&mut out, status.text().as_bytes())?;
    emit!(out, "\",\"error_arg_hex\":\"");
    hex(&mut out, status.arg.as_bytes())?;
    emit!(out, "\"}}}},\"required_prefix\":{{\"called\":{}", parse_ok);
    if let Some(prefix) = prefix {
        emit!(out, ",\"found\":{},\"prefix_hex\":\"", prefix.found);
        hex(&mut out, &prefix.bytes)?;
        emit!(out, "\",\"foldcase\":{}", prefix.foldcase);
    }
    emit!(
        out,
        "}},\"compile\":{{\"called\":{},\"present\":{}}},\"onepass\":{{\"called\":{}",
        parse_ok,
        image.is_some(),
        image.is_some()
    );
    let mut onepass_owner = native_onepass::OnePassState::default();
    if let Some(program) = image.as_mut() {
        let before = program.dfa_mem;
        let first = onepass_owner
            .get_or_analyze(
                native_onepass::ProgramView {
                    instructions: &program.instructions,
                    start: program.start,
                    inst_count: program.inst_count,
                    bytemap: &program.bytemap,
                    bytemap_range: program.bytemap_range,
                },
                &mut program.dfa_mem,
                native_onepass::OnePassLayout {
                    one_state_bytes: layout.one_state,
                    uint32_bytes: layout.uint32,
                },
            )
            .map_err(phase_error)?
            .is_one_pass;
        let after = program.dfa_mem;
        let repeat = onepass_owner
            .get_or_analyze(
                native_onepass::ProgramView {
                    instructions: &program.instructions,
                    start: program.start,
                    inst_count: program.inst_count,
                    bytemap: &program.bytemap,
                    bytemap_range: program.bytemap_range,
                },
                &mut program.dfa_mem,
                native_onepass::OnePassLayout {
                    one_state_bytes: layout.one_state,
                    uint32_bytes: layout.uint32,
                },
            )
            .map_err(phase_error)?
            .is_one_pass;
        emit!(
            out,
            ",\"initial\":{{\"result\":{},\"dfa_mem_before\":{},\"dfa_mem_after\":{}}},\"repeat\":{{\"result\":{},\"dfa_mem_before\":{},\"dfa_mem_after\":{}}}",
            first,
            before,
            after,
            repeat,
            after,
            program.dfa_mem
        );
    }
    emit!(out, "}},\"searches\":[");
    let mut dfa = None;
    for (ordinal, query) in queries.iter().enumerate() {
        if ordinal != 0 {
            emit!(out, ",");
        }
        emit!(
            out,
            "{{\"ordinal\":{},\"phase\":\"direct_anchored_full_SearchDFA\",\"context_hex\":\"",
            ordinal
        );
        hex(&mut out, &query.context)?;
        emit!(
            out,
            "\",\"begin\":{},\"end\":{},\"called\":{}",
            query.begin,
            query.end,
            image.is_some()
        );
        if let Some(program) = image.as_ref() {
            let before = program.dfa_mem;
            let rejected_anchor = program.anchor_start && query.begin != 0
                || program.anchor_end && query.end != query.context.len();
            let result = if rejected_anchor {
                native_dfa::SearchResult {
                    matched: false,
                    failed: false,
                    match_end: None,
                }
            } else {
                if dfa.is_none() {
                    dfa = Some(
                        native_dfa::LongestDfa::new(
                            native_dfa::ProgramView {
                                instructions: &program.instructions,
                                start: program.start,
                                start_unanchored: program.start_unanchored,
                                anchor_start: program.anchor_start,
                                anchor_end: program.anchor_end,
                                list_count: program.list_count,
                                inst_count: program.inst_count,
                                bytemap_range: program.bytemap_range,
                                bytemap: &program.bytemap,
                            },
                            program.dfa_mem,
                            native_dfa::DfaLayout {
                                dfa_bytes: layout.dfa,
                                state_bytes: layout.dfa_state,
                                atomic_state_pointer_bytes: layout.atomic_state_pointer,
                                int_bytes: layout.int,
                            },
                        )
                        .map_err(|error| format!("Rust DFA creation error: {error:?}"))?,
                    );
                }
                dfa.as_mut()
                    .ok_or("missing reached DFA owner")?
                    .search_full(&query.context, query.begin, query.end)
                    .map_err(|error| format!("Rust DFA observation error: {error:?}"))?
            };
            emit!(
                out,
                ",\"matched\":{},\"failed\":{},\"dfa_mem_before\":{},\"dfa_mem_after\":{},\"match0\":{{\"present\":{},\"begin\":",
                result.matched,
                result.failed,
                before,
                program.dfa_mem,
                result.matched
            );
            // Public Prog::SearchDFA writes initialized match0 only AFTER full success.
            if result.matched {
                let end = result
                    .match_end
                    .ok_or("successful DFA lacks actual endpoint")?;
                if end != query.end {
                    return Err("successful full DFA has a different endpoint".into());
                }
                emit!(
                    out,
                    "{},\"bytes\":{},\"hex\":\"",
                    query.begin,
                    end - query.begin
                );
                hex(&mut out, &query.context[query.begin..end])?;
            } else {
                emit!(out, "null,\"bytes\":0,\"hex\":\"");
            }
            emit!(out, "\"}},\"events\":[");
            let events = dfa
                .as_mut()
                .map(|value| value.take_observer_events())
                .unwrap_or_default();
            for (index, event) in events.iter().enumerate() {
                if index != 0 {
                    emit!(out, ",");
                }
                emit!(
                    out,
                    "{{\"kind\":\"cache_reset\",\"state_budget\":{},\"state_cache_size\":{}}}",
                    event.state_budget,
                    event.state_cache_size
                );
            }
            if result.failed {
                if !events.is_empty() {
                    emit!(out, ",");
                }
                // Source public Prog wrapper invokes this hook after DFA::Search returns.
                emit!(out, "{{\"kind\":\"search_failure\"}}");
            }
            emit!(out, "]");
        }
        emit!(out, "}}");
    }
    emit!(out, "]}}\n");
    Ok(out.0)
}

mod fixtures {
    include!("../tests/data/native-automata-operands.rs");
}
pub(crate) fn check_vectors() -> Result<(), String> {
    let expected = include_str!("../tests/data/native-automata.jsonl");
    assert_eq!(fixtures::OPERANDS.len(), 20);
    assert_eq!(expected.lines().count(), fixtures::OPERANDS.len());
    for (ordinal, ((pattern, operands), expected)) in
        fixtures::OPERANDS.iter().zip(expected.lines()).enumerate()
    {
        let queries = operands
            .iter()
            .map(|&(context, begin, end)| Query {
                context: context.to_vec(),
                begin,
                end,
            })
            .collect::<Vec<_>>();
        let actual = record(pattern, &queries, fixtures::LAYOUT)?;
        assert_eq!(
            actual.trim_end_matches('\n'),
            expected,
            "Native public automata group {}",
            ordinal + 1
        );
    }
    Ok(())
}
