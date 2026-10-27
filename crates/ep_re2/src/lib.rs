//! Pure Rust pinned RE2 grammar and boolean full matching for report requests.
//! AST fidelity, construction limits and production integration remain in progress.
mod byte_class;
mod parser;
mod program;
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
    program: Option<Arc<program::Program>>,
}
impl Re2 {
    /// Construct default UTF8/LikePerl with literal inline flags.
    #[must_use]
    pub fn new(pattern: &str) -> Self {
        let compiled = parser::parse(pattern).and_then(program::Program::compile);
        let (status, program) = match compiled {
            Ok(program) => (Status::new(ErrorCode::Success, ""), Some(Arc::new(program))),
            Err(status) => (status, None),
        };
        Self {
            pattern: pattern.into(),
            status,
            program,
        }
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
        self.program
            .as_ref()
            .is_some_and(|program| program.full_match(text))
    }
}

pub(crate) type NodeId = usize;
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Node {
    Empty,
    NoMatch,
    Literal(u32),
    AnyByte,
    Class(Vec<(u32, u32)>),
    Concat(Vec<NodeId>),
    Alternate(Vec<NodeId>),
    Repeat {
        child: NodeId,
        min: u32,
        max: Option<u32>,
    },
    Assert(Assertion),
    Capture {
        child: NodeId,
        index: u32,
    },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Assertion {
    BeginText,
    EndText,
    BeginLine,
    EndLine,
    WordBoundary,
    NotWordBoundary,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Parsed {
    pub nodes: Vec<Node>,
    pub root: NodeId,
    pub captures: u32,
}
