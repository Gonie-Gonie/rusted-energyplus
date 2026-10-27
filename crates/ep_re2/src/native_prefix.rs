// Copyright 2006 The RE2 Authors. All Rights Reserved.
// Rust port of pinned regexp.cc658-742. BSD license: RE2-LICENSE.
//! Native required-prefix selection on the retained AST, before compilation.
use crate::native_ast::{Arena, Id, Op, flags};
use crate::{ErrorCode, Status};

fn allocation() -> Status {
    Status::new(ErrorCode::PatternTooLarge, "")
}

/// Native RequiredPrefix's complete cleared-or-found output state.
/// A suffix ID belongs to the same arena as the supplied root; it is neither
/// detached nor recursively owned. The original root remains available.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct RequiredPrefix {
    pub found: bool,
    pub bytes: Vec<u8>,
    /// Native's ASCII byte-folding flag, never a Unicode closure request.
    pub foldcase: bool,
    pub suffix: Option<Id>,
}

/// Native RequiredPrefixForAccel has no suffix output or arena allocation.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct AccelPrefix {
    pub found: bool,
    pub bytes: Vec<u8>,
    pub foldcase: bool,
}

/// ConvertRunesToBytes, regexp.cc658-672. Preserve bytes rather than constructing
/// a Rust String: runetochar permits surrogate Rune encodings and embedded NUL.
pub(crate) fn convert_runes_to_bytes(latin1: bool, runes: &[u32]) -> Result<Vec<u8>, Status> {
    let capacity = if latin1 {
        runes.len()
    } else {
        runes.len().checked_mul(4).ok_or_else(allocation)?
    };
    let mut bytes = Vec::new();
    bytes
        .try_reserve_exact(capacity)
        .map_err(|_| allocation())?;
    if latin1 {
        // Native static_cast<char>(Rune) retains the low byte in the pinned
        // Windows/GCC implementation. No decoding or scalar validation occurs.
        for &rune in runes {
            bytes.push(rune as u8);
        }
    } else {
        for &rune in runes {
            let (encoded, width) = crate::byte_class::encode_rune(rune);
            bytes.extend_from_slice(&encoded[..width]);
        }
        // Like primary shrink_to_fit, this is only a capacity request. It has
        // no bearing on retained bytes or Native program-memory accounting.
        bytes.shrink_to_fit();
    }
    Ok(bytes)
}

fn atom_bytes(op: &Op, bits: u16) -> Result<Vec<u8>, Status> {
    let runes: &[u32] = match op {
        Op::Literal(rune) => std::slice::from_ref(rune),
        Op::LiteralString(runes) => runes,
        _ => return Err(Status::new(ErrorCode::InternalError, "")),
    };
    convert_runes_to_bytes(bits & flags::LATIN1 != 0, runes)
}

impl Arena {
    /// Exact RequiredPrefix shape: top-level Concat, one or more BeginText
    /// children, then one Literal or LiteralString, then the optional suffix.
    /// Does not see through captures, descend nested concats or glue atoms.
    pub(crate) fn required_prefix(&mut self, root: Id) -> Result<RequiredPrefix, Status> {
        let cleared = RequiredPrefix::default();
        let node = self.node(root)?;
        let Op::Concat(children) = &node.op else {
            return Ok(cleared);
        };
        let mut index = 0;
        while let Some(&child) = children.get(index) {
            if !matches!(self.node(child)?.op, Op::BeginText) {
                break;
            }
            index += 1;
        }
        if index == 0 || index >= children.len() {
            return Ok(cleared);
        }
        let atom = children[index];
        if !matches!(self.node(atom)?.op, Op::Literal(_) | Op::LiteralString(_)) {
            return Ok(cleared);
        }
        index += 1;
        let root_flags = node.flags;
        // Native increments these child references and calls its Concat
        // factory BEFORE converting bytes. Arena IDs retain the same children
        // and their cache/flags without mutation or recursive ownership.
        let suffix = if index < children.len() {
            let remaining = &children[index..];
            let mut ids = Vec::new();
            ids.try_reserve(remaining.len()).map_err(|_| allocation())?;
            ids.extend_from_slice(remaining);
            self.concat(ids, root_flags)?
        } else {
            // Constructor cache remains simple=false. Do not call PushRegexp,
            // refresh_simple, simplify, or the parser's empty normalization.
            self.new_node(Op::Empty, root_flags)?
        };
        let atom = self.node(atom)?;
        let bytes = atom_bytes(&atom.op, atom.flags)?;
        Ok(RequiredPrefix {
            found: true,
            bytes,
            foldcase: atom.flags & flags::FOLD_CASE != 0,
            suffix: Some(suffix),
        })
    }

    /// Exact RequiredPrefixForAccel selection. Inspect the initial concat's
    /// first child, then look through each Capture and at most one immediate
    /// first-concat child after that Capture. Never join literal fragments.
    #[expect(
        dead_code,
        reason = "Retained compiler acceleration API; exact compiler integration is pending"
    )]
    pub(crate) fn required_prefix_for_accel(&self, root: Id) -> Result<AccelPrefix, Status> {
        let cleared = AccelPrefix::default();
        let mut atom = match &self.node(root)?.op {
            Op::Concat(children) if !children.is_empty() => children[0],
            _ => root,
        };
        while let Op::Capture { child, .. } = &self.node(atom)?.op {
            atom = *child;
            if let Op::Concat(children) = &self.node(atom)?.op
                && let Some(&first) = children.first()
            {
                atom = first;
            }
        }
        let atom = self.node(atom)?;
        if !matches!(atom.op, Op::Literal(_) | Op::LiteralString(_)) {
            return Ok(cleared);
        }
        Ok(AccelPrefix {
            found: true,
            bytes: atom_bytes(&atom.op, atom.flags)?,
            foldcase: atom.flags & flags::FOLD_CASE != 0,
        })
    }
}
