//! Pure Rust pinned RE2 grammar and boolean full matching for report requests.
//! Target construction limits and explicit default diagnostic effects are retained.
mod byte_class;
mod diagnostics;
pub use diagnostics::{Diagnostic, DiagnosticKind, TargetDiagnosticOrigin};
mod native_ast;
#[cfg(test)]
mod native_ast_snapshot;
#[cfg(test)]
mod native_automata_snapshot;
mod native_compiler;
mod native_dfa;
mod native_finalize;
mod native_matcher;
mod native_onepass;
mod native_parser;
mod native_prefix;
#[cfg(test)]
mod native_program_snapshot;
mod native_target;
mod unicode;
use std::sync::{Arc, Mutex};

/// Pinned parse/construction code, ordered as RE2::ErrorCode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ErrorCode {
    /// Successful construction.
    Success = 0,
    /// Unexpected parser failure.
    InternalError,
    /// Invalid escape sequence.
    BadEscape,
    /// Invalid character class.
    BadCharClass,
    /// Invalid character class range.
    BadCharRange,
    /// Missing closing bracket.
    MissingBracket,
    /// Missing closing parenthesis.
    MissingParen,
    /// Unexpected closing parenthesis.
    UnexpectedParen,
    /// Trailing backslash.
    TrailingBackslash,
    /// Missing repetition operand.
    RepeatArgument,
    /// Invalid repetition count.
    RepeatSize,
    /// Repeated repetition operator.
    RepeatOp,
    /// Invalid Perl operator.
    BadPerlOp,
    /// Invalid UTF8 pattern.
    BadUTF8,
    /// Invalid named capture group.
    BadNamedCapture,
    /// Program construction allocation failure.
    PatternTooLarge,
}

/// Retained constructor status and its exact error argument.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Status {
    /// Pinned code.
    pub code: ErrorCode,
    /// Exact selected pattern span.
    pub arg: String,
}
impl Status {
    pub(crate) fn new(code: ErrorCode, arg: impl Into<String>) -> Self {
        Self {
            code,
            arg: arg.into(),
        }
    }
    /// Pinned RegexpStatus::CodeText.
    #[must_use]
    pub fn code_text(&self) -> &'static str {
        match self.code {
            ErrorCode::Success => "no error",
            ErrorCode::InternalError => "unexpected error",
            ErrorCode::BadEscape => "invalid escape sequence",
            ErrorCode::BadCharClass => "invalid character class",
            ErrorCode::BadCharRange => "invalid character class range",
            ErrorCode::MissingBracket => "missing ]",
            ErrorCode::MissingParen => "missing )",
            ErrorCode::UnexpectedParen => "unexpected )",
            ErrorCode::TrailingBackslash => "trailing \\",
            ErrorCode::RepeatArgument => "no argument for repetition operator",
            ErrorCode::RepeatSize => "invalid repetition size",
            ErrorCode::RepeatOp => "bad repetition operator",
            ErrorCode::BadPerlOp => "invalid perl operator",
            ErrorCode::BadUTF8 => "invalid UTF-8",
            ErrorCode::BadNamedCapture => "invalid named capture group",
            ErrorCode::PatternTooLarge => "pattern too large - compile failed",
        }
    }
    /// Pinned RegexpStatus::Text plus the constructor compile-failure case.
    #[must_use]
    pub fn text(&self) -> String {
        if self.arg.is_empty() || self.code == ErrorCode::PatternTooLarge {
            self.code_text().into()
        } else {
            format!("{}: {}", self.code_text(), self.arg)
        }
    }
}

/// Immutable compiled pattern with a shared persistent DFA cache.
/// Clones share the cache; expression equality does not inspect mutable state.
#[derive(Clone)]
pub struct Re2 {
    pattern: String,
    status: Status,
    program: Option<Arc<native_finalize::ExecutableProgram>>,
    prefix: Vec<u8>,
    prefix_foldcase: bool,
    dfa: Arc<Mutex<Option<native_dfa::LongestDfa>>>,
}
impl std::fmt::Debug for Re2 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Re2")
            .field("pattern", &self.pattern)
            .field("status", &self.status)
            .field("program", &self.program)
            .field("prefix", &self.prefix)
            .field("prefix_foldcase", &self.prefix_foldcase)
            .finish_non_exhaustive()
    }
}
impl PartialEq for Re2 {
    fn eq(&self, other: &Self) -> bool {
        self.pattern == other.pattern
            && self.status == other.status
            && self.program == other.program
            && self.prefix == other.prefix
            && self.prefix_foldcase == other.prefix_foldcase
    }
}
impl Eq for Re2 {}
impl Re2 {
    /// Construct UTF8/LikePerl with literal inline flags and quiet diagnostics.
    /// Reporting owners use `new_logged` for the default RE2 stderr effects.
    #[must_use]
    pub fn new(pattern: &str) -> Self {
        Self::new_with_failure_stage(pattern).0
    }

    fn new_with_failure_stage(
        pattern: &str,
    ) -> (Self, Option<diagnostics::ConstructorFailureStage>) {
        let mut prefix = Vec::new();
        let mut prefix_foldcase = false;
        let parsed = native_parser::parse(pattern);
        let parse_failure = parsed
            .is_err()
            .then_some(diagnostics::ConstructorFailureStage::Parse);
        let compiled = parsed.and_then(|mut ast| {
            // Grammar failures retain their actual parser status. Construction
            // failures after Parse use the native constructor's empty-arg
            // ErrorPatternTooLarge result, never an internal port guard status.
            let constructed = (|| {
                let selected = ast.arena.required_prefix(ast.root)?;
                prefix = selected.bytes;
                prefix_foldcase = selected.foldcase;
                let suffix = if selected.found {
                    selected
                        .suffix
                        .ok_or_else(|| Status::new(ErrorCode::InternalError, ""))?
                } else {
                    ast.root
                };
                let simplified = ast.arena.simplify(suffix)?;
                let front = native_compiler::compile_forward_with_instruction_cap(
                    &mut ast.arena,
                    suffix,
                    simplified,
                    native_compiler::OFFICIAL_DEFAULT_FORWARD_INSTRUCTION_CAP,
                )?;
                let mut program = native_finalize::finalize_executable(front)?;
                // RE2::Init eagerly calls Prog::IsOnePass after CompileToProg.
                // Keep both true/false outcomes and actual image/charge on the
                // immutable compiled Arc; the lazy DFA subtracts that charge once.
                program.onepass = Some(native_onepass::analyze_default(
                    native_onepass::ProgramView {
                        instructions: &program.instructions,
                        start: program.start,
                        inst_count: program.inst_count,
                        bytemap: &program.bytemap,
                        bytemap_range: program.bytemap_range,
                    },
                    program.finalized_instructions,
                )?);
                Ok::<_, Status>(program)
            })();
            constructed.map_err(|_| Status::new(ErrorCode::PatternTooLarge, ""))
        });
        let (status, program, failure_stage) = match compiled {
            Ok(program) => (
                Status::new(ErrorCode::Success, ""),
                Some(Arc::new(program)),
                None,
            ),
            Err(status) => (
                status,
                None,
                parse_failure.or(Some(diagnostics::ConstructorFailureStage::Compile)),
            ),
        };
        (
            Self {
                pattern: pattern.into(),
                status,
                program,
                prefix,
                prefix_foldcase,
                dfa: Arc::new(Mutex::new(None)),
            },
            failure_stage,
        )
    }
    /// Whether construction succeeded.
    #[must_use]
    pub fn ok(&self) -> bool {
        self.program.is_some()
    }
    /// Original pattern bytes.
    #[must_use]
    pub fn pattern(&self) -> &str {
        &self.pattern
    }
    /// Retained constructor status.
    #[must_use]
    pub fn status(&self) -> &Status {
        &self.status
    }
    /// Native error is empty after successful construction.
    #[must_use]
    pub fn error(&self) -> String {
        if self.ok() {
            String::new()
        } else {
            self.status.text()
        }
    }
    /// Boolean FullMatch with explicit byte lengths and no captures.
    #[must_use]
    pub fn full_match(&self, text: &[u8]) -> bool {
        self.full_match_with_diagnostic(text, None)
    }

    fn full_match_with_diagnostic(
        &self,
        text: &[u8],
        mut emit: Option<&mut dyn FnMut(Diagnostic)>,
    ) -> bool {
        match self.try_full_match(text, &mut emit) {
            Ok(matched) => matched,
            Err(error) => fatal_dfa_error(error),
        }
    }

    fn try_full_match(
        &self,
        text: &[u8],
        emit: &mut Option<&mut dyn FnMut(Diagnostic)>,
    ) -> Result<bool, native_dfa::DfaError> {
        // FullMatchN -> DoMatch rejects an invalid expression before Match's
        // prefix checks. Quiet matching takes the same branch without LOG.
        let Some(program) = self.program.as_deref() else {
            if let Some(emit) = emit.as_mut() {
                emit(Diagnostic::invalid_full_match(self.error().as_bytes()));
            }
            return Ok(false);
        };
        let Some(head) = text.get(..self.prefix.len()) else {
            return Ok(false);
        };
        // re2.cc599-610 folds only the text operand; the parser's prefix is
        // already ASCII lowercase. Explicit lengths retain embedded NUL.
        let equal = if self.prefix_foldcase {
            self.prefix
                .iter()
                .zip(head)
                .all(|(&a, &b)| a == b.to_ascii_lowercase())
        } else {
            self.prefix == head
        };
        if !equal {
            return Ok(false);
        }
        let onepass = program
            .onepass
            .as_ref()
            .ok_or(native_dfa::DfaError::SourceInvariant)?;
        // FullMatchN requests no captures (ncap=0). Native's OnePass gate
        // therefore reduces to ORIGINAL text.size()<=16. BitState's pre-DFA
        // skip requires ncap>1 and cannot apply to this API.
        if onepass.is_one_pass && text.len() <= 16 {
            return Ok(self.nfa_full_match(program, text));
        }
        let begin = self.prefix.len();
        let end = text.len();
        // Prog::SearchDFA checks anchors before GetDFA; a suffix BeginText
        // after a stripped literal prefix must retain its original context.
        if (program.anchor_start && begin != 0) || (program.anchor_end && end != text.len()) {
            return Ok(false);
        }
        let result = {
            // Writers and fallback run after releasing this cache lock.
            let mut owner = match self.dfa.lock() {
                Ok(owner) => owner,
                Err(poisoned) => poisoned.into_inner(),
            };
            let view = Self::dfa_view(program);
            if owner.is_none() {
                *owner = Some(native_dfa::LongestDfa::new(
                    view,
                    native_target::post_onepass_dfa_mem(program)?,
                    native_target::DFA_LAYOUT,
                )?);
            }
            owner
                .as_mut()
                .ok_or(native_dfa::DfaError::SourceInvariant)?
                .search_full(view, text, begin, end)?
        };
        if result.failed {
            // re2.cc811 LOG precedes the acceptance fallback.
            if let Some(emit) = emit.as_mut() {
                emit(Diagnostic::dfa_out_of_memory(
                    self.pattern.len(),
                    program.finalized_instructions,
                    program.list_count,
                    program.bytemap_range,
                ));
            }
            return Ok(self.nfa_full_match(program, text));
        }
        if result.matched && result.match_end != Some(end) {
            return Err(native_dfa::DfaError::SourceInvariant);
        }
        Ok(result.matched)
    }

    fn dfa_view(program: &native_finalize::ExecutableProgram) -> native_dfa::ProgramView<'_> {
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
        }
    }

    fn nfa_full_match(&self, program: &native_finalize::ExecutableProgram, text: &[u8]) -> bool {
        native_matcher::full_match(
            native_matcher::ProgramView {
                instructions: &program.instructions,
                start: program.start,
                anchor_start: program.anchor_start,
            },
            text,
            self.prefix.len(),
        )
    }
}

// Public bool matching has no host/invariant error channel. Do not translate
// these typed failures to Native no-match, budget failure or LOG811. This is a
// port/host fatal boundary, not a claim about Native exception or exit codes.
fn fatal_dfa_error(error: native_dfa::DfaError) -> ! {
    use std::io::Write as _;
    let _ = writeln!(
        std::io::stderr().lock(),
        "ep_re2 host/invariant DFA failure: {error:?}"
    );
    std::process::abort()
}
