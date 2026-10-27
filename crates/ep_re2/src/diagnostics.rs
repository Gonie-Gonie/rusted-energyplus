//! Pinned default RE2 LOG bytes, separate from UtilityRoutines error effects.
use crate::{Re2, Status};
use std::io::Write;

/// Actual target __FILE__ bytes and CRT LF translation; compatibility metadata,
/// never Rust file!(), a model seed, or a guessed default build location.
#[derive(Clone, Copy, Debug)]
pub struct TargetDiagnosticOrigin<'a> {
    /// Source file bytes recorded for the target build's LOG calls.
    pub source_file: &'a [u8],
    /// b"\n" for binary/LF output; b"\r\n" for observed Windows CRT text mode.
    /// Replacement applies to EVERY LF, including LF already preceded by CR.
    pub line_ending: &'a [u8],
}

/// Distinct pinned default diagnostic branches.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiagnosticKind {
    /// Constructor grammar failure.
    Parse,
    /// Constructor forward-program failure after parsing.
    Compile,
    /// FullMatch called on an invalid expression.
    InvalidFullMatch,
}

/// Actual diagnostic payload bytes, independent of target framing metadata.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Diagnostic {
    /// Reached source logging branch.
    pub kind: DiagnosticKind,
    /// Unframed message bytes, retaining embedded NUL and partial UTF8.
    pub payload: Vec<u8>,
}

#[derive(Clone, Copy)]
pub(crate) enum ConstructorFailureStage {
    Parse,
    Compile,
}

fn truncated(pattern: &[u8]) -> Vec<u8> {
    let mut bytes = pattern[..pattern.len().min(100)].to_vec();
    // Native trunc uses size<100, so EXACTLY100 bytes also gets an ellipsis.
    if pattern.len() >= 100 {
        bytes.extend_from_slice(b"...");
    }
    bytes
}

impl Diagnostic {
    /// LOG invocation line in the pinned re2.cc source.
    pub fn source_line(&self) -> u32 {
        match self.kind {
            DiagnosticKind::Parse => 205,
            DiagnosticKind::Compile => 226,
            DiagnosticKind::InvalidFullMatch => 885,
        }
    }

    /// Constructor parse-failure payload, including native byte truncation.
    pub fn parse_failure(pattern: &[u8], status: &Status) -> Self {
        let mut payload = b"Error parsing '".to_vec();
        payload.extend_from_slice(&truncated(pattern));
        payload.extend_from_slice(b"': ");
        payload.extend_from_slice(status.text().as_bytes());
        Self {
            kind: DiagnosticKind::Parse,
            payload,
        }
    }

    /// Constructor post-parse compile-failure payload.
    pub fn compile_failure(pattern: &[u8]) -> Self {
        let mut payload = b"Error compiling '".to_vec();
        payload.extend_from_slice(&truncated(pattern));
        payload.push(b'\'');
        Self {
            kind: DiagnosticKind::Compile,
            payload,
        }
    }

    /// Invalid DoMatch payload, before prefix checks.
    pub fn invalid_full_match(error: &[u8]) -> Self {
        let mut payload = b"Invalid RE2: ".to_vec();
        payload.extend_from_slice(error);
        Self {
            kind: DiagnosticKind::InvalidFullMatch,
            payload,
        }
    }

    /// Frame the payload and apply the specified CRT newline translation.
    pub fn bytes(&self, origin: TargetDiagnosticOrigin<'_>) -> Vec<u8> {
        let mut source = origin.source_file.to_vec();
        source.push(b':');
        source.extend_from_slice(self.source_line().to_string().as_bytes());
        source.extend_from_slice(b": ");
        source.extend_from_slice(&self.payload);
        source.push(b'\n'); // LogMessage::Flush appends one SOURCE newline.
        let mut target = Vec::new();
        for byte in source {
            if byte == b'\n' {
                target.extend_from_slice(origin.line_ending);
            } else {
                target.push(byte);
            }
        }
        target
    }

    /// One fwrite-like call; short writes/errors do not change constructor,
    /// matching, or the caller's subsequent utility Severe/Continue/Fatal flow.
    pub fn write_to<W: Write + ?Sized>(&self, origin: TargetDiagnosticOrigin<'_>, writer: &mut W) {
        let _ = writer.write(&self.bytes(origin));
    }
}

impl Re2 {
    fn construct_with_diagnostic(pattern: &str) -> (Self, Option<Diagnostic>) {
        let (regex, stage) = Self::new_with_failure_stage(pattern);
        let diagnostic = stage.map(|stage| match stage {
            ConstructorFailureStage::Parse => {
                Diagnostic::parse_failure(pattern.as_bytes(), regex.status())
            }
            ConstructorFailureStage::Compile => Diagnostic::compile_failure(pattern.as_bytes()),
        });
        (regex, diagnostic)
    }

    /// Construct with default logging to the supplied stderr owner.
    pub fn new_logged_to<W: Write + ?Sized>(
        pattern: &str,
        origin: TargetDiagnosticOrigin<'_>,
        writer: &mut W,
    ) -> Self {
        let (regex, diagnostic) = Self::construct_with_diagnostic(pattern);
        if let Some(diagnostic) = diagnostic {
            diagnostic.write_to(origin, writer);
        }
        regex
    }

    /// Construct with default logging to the actual process stderr.
    pub fn new_logged(pattern: &str, origin: TargetDiagnosticOrigin<'_>) -> Self {
        let (regex, diagnostic) = Self::construct_with_diagnostic(pattern);
        if let Some(diagnostic) = diagnostic {
            diagnostic.write_to(origin, &mut std::io::stderr().lock());
        }
        regex
    }

    /// Match with default invalid-expression logging to the supplied stderr.
    pub fn full_match_logged_to<W: Write + ?Sized>(
        &self,
        text: &[u8],
        origin: TargetDiagnosticOrigin<'_>,
        writer: &mut W,
    ) -> bool {
        // FullMatchN373 -> DoMatch884/885 guards invalid state BEFORE Match's
        // prefix logic. It returns here, so Match624 is not a second log.
        if !self.ok() {
            Diagnostic::invalid_full_match(self.error().as_bytes()).write_to(origin, writer);
            return false;
        }
        self.full_match(text)
    }

    /// Match with default invalid-expression logging to process stderr.
    pub fn full_match_logged(&self, text: &[u8], origin: TargetDiagnosticOrigin<'_>) -> bool {
        if !self.ok() {
            Diagnostic::invalid_full_match(self.error().as_bytes())
                .write_to(origin, &mut std::io::stderr().lock());
            return false;
        }
        self.full_match(text)
    }
}
