// Copyright 2007 The RE2 Authors. All Rights Reserved.
// Pinned compile.cc985-1074, bounded IsAnchorStart/End. BSD: ../RE2-LICENSE.
use super::{Arena, Id, Op, Status, allocation, internal};

/// The Native depth guard is EXACTLY4, even if a deeper assertion would be
/// semantically required. Rebuild only the successful path, without refreshing
/// simple caches. Capture() factory deliberately drops the original name.
pub(super) fn remove(arena: &mut Arena, original: Id, end: bool) -> Result<(bool, Id), Status> {
    let mut path: [Option<(Id, usize)>; 4] = [None; 4];
    let mut depth = 0;
    let mut current = original;
    let mut replacement;
    loop {
        if depth >= 4 {
            return Ok((false, original));
        }
        let node = arena.node(current)?;
        match &node.op {
            Op::Concat(children) if !children.is_empty() => {
                let at = if end { children.len() - 1 } else { 0 };
                path[depth] = Some((current, at));
                current = children[at];
                depth += 1;
            }
            Op::Capture { child, .. } => {
                path[depth] = Some((current, 0));
                current = *child;
                depth += 1;
            }
            Op::BeginText if !end => {
                replacement = arena.new_node(Op::Empty, node.flags)?;
                break;
            }
            Op::EndText if end => {
                replacement = arena.new_node(Op::Empty, node.flags)?;
                break;
            }
            _ => return Ok((false, original)),
        }
    }
    while depth > 0 {
        depth -= 1;
        let (id, child_index) = path[depth].ok_or_else(internal)?;
        let node = arena.node(id)?;
        let bits = node.flags;
        replacement = match &node.op {
            Op::Concat(children) => {
                let mut copied = Vec::new();
                copied
                    .try_reserve(children.len())
                    .map_err(|_| allocation())?;
                copied.extend_from_slice(children);
                *copied.get_mut(child_index).ok_or_else(internal)? = replacement;
                arena.concat(copied, bits)?
            }
            Op::Capture { index, .. } => {
                let index = *index;
                arena.new_node(
                    Op::Capture {
                        child: replacement,
                        index,
                        name: None,
                    },
                    bits,
                )?
            }
            _ => return Err(internal()),
        };
    }
    Ok((true, replacement))
}
