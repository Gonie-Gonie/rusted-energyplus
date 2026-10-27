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
#[cfg(test)]
mod native_dfa;
mod native_finalize;
mod native_matcher;
#[cfg(test)]
mod native_onepass;
mod native_parser;
mod native_prefix;
#[cfg(test)]
mod native_program_snapshot;
mod unicode;
use std::sync::Arc;

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

/// Immutable compiled pattern, including an invalid constructor result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Re2 {
    pattern: String,
    status: Status,
    program: Option<Arc<native_finalize::ExecutableProgram>>,
    prefix: Vec<u8>,
    prefix_foldcase: bool,
}
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
                native_finalize::finalize_executable(front)
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
        let Some(head) = text.get(..self.prefix.len()) else {
            return false;
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
            return false;
        }
        self.program.as_ref().is_some_and(|program| {
            native_matcher::full_match(
                native_matcher::ProgramView {
                    instructions: &program.instructions,
                    start: program.start,
                    anchor_start: program.anchor_start,
                },
                text,
                self.prefix.len(),
            )
        })
    }
}
