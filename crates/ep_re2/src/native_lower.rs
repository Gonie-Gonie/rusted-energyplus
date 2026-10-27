//! Boolean matcher lowering after the retained Native AST's ordered rewrites.
//! This does not establish Native compiler allocation limits or graph identity.
use crate::native_ast::{Ast, Id, Op, flags};
use crate::{Assertion, ErrorCode, Node, NodeId, Parsed, Status};

fn internal() -> Status {
    Status::new(ErrorCode::InternalError, "")
}
fn allocation() -> Status {
    Status::new(ErrorCode::PatternTooLarge, "")
}
fn append<T>(values: &mut Vec<T>, value: T) -> Result<usize, Status> {
    values.try_reserve(1).map_err(|_| allocation())?;
    let id = values.len();
    values.push(value);
    Ok(id)
}
fn literal(nodes: &mut Vec<Node>, rune: u32, bits: u16) -> Result<NodeId, Status> {
    // compile.cc813-830 forwards FoldCase only to a single ASCII ByteRange.
    // Non-ASCII literals use exact runetochar bytes. Never close Unicode here.
    if bits & flags::FOLD_CASE != 0 && matches!(rune, 65..=90 | 97..=122) {
        let lower = (rune as u8).to_ascii_lowercase() as u32;
        append(
            nodes,
            Node::Class(vec![(lower - 32, lower - 32), (lower, lower)]),
        )
    } else {
        append(nodes, Node::Literal(rune))
    }
}

pub(crate) fn lower(ast: &Ast, root: Id) -> Result<Parsed, Status> {
    let mut nodes = Vec::new();
    let mut mapped = Vec::new();
    mapped
        .try_reserve(ast.arena.nodes.len())
        .map_err(|_| allocation())?;
    mapped.resize(ast.arena.nodes.len(), None);
    // Retained arenas can contain replacements/Swaps. Traverse actual edges,
    // rather than assuming allocation IDs form a postorder.
    let mut work = vec![(root, false)];
    while let Some((id, visited)) = work.pop() {
        if mapped.get(id).ok_or_else(internal)?.is_some() {
            continue;
        }
        let native = ast.arena.node(id)?;
        if !visited {
            append(&mut work, (id, true))?;
            match &native.op {
                Op::Concat(children) | Op::Alternate(children) => {
                    for &child in children.iter().rev() {
                        append(&mut work, (child, false))?;
                    }
                }
                Op::Star(child)
                | Op::Plus(child)
                | Op::Quest(child)
                | Op::Repeat { child, .. }
                | Op::Capture { child, .. } => {
                    append(&mut work, (*child, false))?;
                }
                _ => {}
            }
            continue;
        }
        let child = |id: Id| mapped.get(id).copied().flatten().ok_or_else(internal);
        let children = |ids: &[Id]| -> Result<Vec<NodeId>, Status> {
            let mut result = Vec::new();
            result.try_reserve(ids.len()).map_err(|_| allocation())?;
            for &id in ids {
                result.push(child(id)?);
            }
            Ok(result)
        };
        let lowered = match &native.op {
            Op::Literal(rune) => {
                mapped[id] = Some(literal(&mut nodes, *rune, native.flags)?);
                continue;
            }
            Op::LiteralString(runes) => {
                let mut ids = Vec::new();
                ids.try_reserve(runes.len()).map_err(|_| allocation())?;
                for &rune in runes {
                    ids.push(literal(&mut nodes, rune, native.flags)?);
                }
                Node::Concat(ids)
            }
            Op::Empty => Node::Empty,
            Op::NoMatch => Node::NoMatch,
            Op::Class(ranges) => {
                if ranges.is_empty() {
                    Node::NoMatch
                } else {
                    Node::Class(ranges.clone())
                }
            }
            Op::AnyChar => Node::Class(vec![(0, 0x10ffff)]),
            Op::AnyByte => Node::AnyByte,
            Op::Concat(ids) => Node::Concat(children(ids)?),
            Op::Alternate(ids) => Node::Alternate(children(ids)?),
            Op::Star(id) => Node::Repeat {
                child: child(*id)?,
                min: 0,
                max: None,
            },
            Op::Plus(id) => Node::Repeat {
                child: child(*id)?,
                min: 1,
                max: None,
            },
            Op::Quest(id) => Node::Repeat {
                child: child(*id)?,
                min: 0,
                max: Some(1),
            },
            Op::Repeat {
                child: id,
                min,
                max,
            } => Node::Repeat {
                child: child(*id)?,
                min: *min,
                max: *max,
            },
            Op::Capture {
                child: id, index, ..
            } => Node::Capture {
                child: child(*id)?,
                index: *index,
            },
            Op::BeginLine => Node::Assert(Assertion::BeginLine),
            Op::EndLine => Node::Assert(Assertion::EndLine),
            Op::BeginText => Node::Assert(Assertion::BeginText),
            Op::EndText => Node::Assert(Assertion::EndText),
            Op::WordBoundary => Node::Assert(Assertion::WordBoundary),
            Op::NotWordBoundary => Node::Assert(Assertion::NotWordBoundary),
            Op::ClassBuilder(_) | Op::HaveMatch(_) => return Err(internal()),
        };
        mapped[id] = Some(append(&mut nodes, lowered)?);
    }
    Ok(Parsed {
        root: mapped[root].ok_or_else(internal)?,
        nodes,
        captures: ast.captures,
    })
}
