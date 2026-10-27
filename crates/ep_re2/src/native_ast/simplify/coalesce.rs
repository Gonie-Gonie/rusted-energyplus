// Copyright 2006 The RE2 Authors. All Rights Reserved.
// Rust port of pinned simplify.cc230-442; BSD-style license: ../../RE2-LICENSE.
//! CoalesceWalker postvisit and its exact adjacent-pair transformations.
use super::{Arena, Id, Op, Status, copy, flags, internal, push};

/// The source fields are signed int32. Do not manufacture wrapped results for
/// a domain where Native addition would overflow its signed count fields.
fn add_count(a: u32, b: u32) -> Result<u32, Status> {
    a.checked_add(b)
        .filter(|&sum| sum <= 2_147_483_647)
        .ok_or_else(internal)
}

fn repeat_child(op: &Op) -> Option<Id> {
    match op {
        Op::Star(child) | Op::Plus(child) | Op::Quest(child) | Op::Repeat { child, .. } => {
            Some(*child)
        }
        _ => None,
    }
}

fn repeat_bounds(op: &Op) -> Result<(u32, Option<u32>), Status> {
    let (min, max) = match op {
        Op::Star(_) => (0, None),
        Op::Plus(_) => (1, None),
        Op::Quest(_) => (0, Some(1)),
        Op::Repeat { min, max, .. } => (*min, *max),
        _ => return Err(internal()),
    };
    if min > 2_147_483_647 || max.is_some_and(|max| max > 2_147_483_647) {
        return Err(internal());
    }
    Ok((min, max))
}

impl Arena {
    pub(super) fn coalesce_post(
        &mut self,
        original: Id,
        mut children: Vec<Id>,
    ) -> Result<Id, Status> {
        let node = self.node(original)?;
        let bits = node.flags;
        let old = match &node.op {
            Op::Concat(ids) | Op::Alternate(ids) => copy(ids)?,
            op => op.unary_child().map_or_else(Vec::new, |id| vec![id]),
        };
        // Native returns a leaf unchanged, without consulting child_args.
        if old.is_empty() {
            return Ok(original);
        }
        if children.len() != old.len() {
            return Err(internal());
        }
        let concat = matches!(node.op, Op::Concat(_));
        if !concat {
            if children == old {
                return Ok(original);
            }
            return self.coalesce_rebuild(original, children);
        }

        let mut any = false;
        for pair in children.windows(2) {
            if self.can_coalesce(pair[0], pair[1])? {
                any = true;
                break;
            }
        }
        if !any {
            if children == old {
                return Ok(original);
            }
            // A raw new Concat, not the arity-normalizing Concat factory.
            return self.new_node(Op::Concat(children), bits);
        }

        // Native rechecks after each replacement, allowing a new right repeat
        // to coalesce with the next item on the next loop iteration.
        for index in 0..children.len() {
            if index + 1 < children.len()
                && self.can_coalesce(children[index], children[index + 1])?
            {
                let (left, right) = self.do_coalesce(children[index], children[index + 1])?;
                children[index] = left;
                children[index + 1] = right;
            }
        }
        let mut retained = Vec::new();
        retained
            .try_reserve(children.len())
            .map_err(|_| super::super::allocation())?;
        for child in children {
            if !matches!(self.node(child)?.op, Op::Empty) {
                push(&mut retained, child)?;
            }
        }
        // Native removes ALL empty children only on this coalescence path,
        // then keeps even a zero/one-child raw Concat with simple=false.
        self.new_node(Op::Concat(retained), bits)
    }

    fn coalesce_rebuild(&mut self, original: Id, children: Vec<Id>) -> Result<Id, Status> {
        let node = self.node(original)?;
        let bits = node.flags;
        let first = children.first().copied();
        let only = || first.ok_or_else(internal);
        let op = match &node.op {
            Op::Concat(_) => Op::Concat(children),
            Op::Alternate(_) => Op::Alternate(children),
            Op::Star(_) => Op::Star(only()?),
            Op::Plus(_) => Op::Plus(only()?),
            Op::Quest(_) => Op::Quest(only()?),
            Op::Repeat { min, max, .. } => Op::Repeat {
                child: only()?,
                min: *min,
                max: *max,
            },
            // Source copies cap_ but does NOT copy the optional name pointer.
            Op::Capture { index, .. } => Op::Capture {
                child: only()?,
                index: *index,
                name: None,
            },
            _ => return Err(internal()),
        };
        self.new_node(op, bits)
    }

    fn can_coalesce(&self, left: Id, right: Id) -> Result<bool, Status> {
        let r1 = self.node(left)?;
        let r2 = self.node(right)?;
        let Some(atom) = repeat_child(&r1.op) else {
            return Ok(false);
        };
        let atom_node = self.node(atom)?;
        if !matches!(
            atom_node.op,
            Op::Literal(_) | Op::Class(_) | Op::AnyChar | Op::AnyByte
        ) {
            return Ok(false);
        }

        if let Some(other) = repeat_child(&r2.op)
            && self.equal(atom, other)?
            && (r1.flags & flags::NON_GREEDY) == (r2.flags & flags::NON_GREEDY)
        {
            return Ok(true);
        }
        if self.equal(atom, right)? {
            return Ok(true);
        }
        if let (Op::Literal(rune), Op::LiteralString(runes)) = (&atom_node.op, &r2.op) {
            return Ok(runes.first().is_some_and(|first| first == rune)
                && (atom_node.flags & flags::FOLD_CASE) == (r2.flags & flags::FOLD_CASE));
        }
        Ok(false)
    }

    fn do_coalesce(&mut self, left: Id, right: Id) -> Result<(Id, Id), Status> {
        let r1 = self.node(left)?;
        let atom = repeat_child(&r1.op).ok_or_else(internal)?;
        let bits = r1.flags;
        // Source constructs Repeat(0,0) before filling either operand's bounds.
        let result = self.native_repeat(atom, 0, Some(0), bits)?;
        let (mut min, mut max) = repeat_bounds(&self.node(left)?.op)?;
        let r2 = self.node(right)?;
        let mut remainder: Option<(Vec<u32>, u16)> = None;
        match &r2.op {
            Op::Star(_) => max = None,
            Op::Plus(_) => {
                min = add_count(min, 1)?;
                max = None;
            }
            Op::Quest(_) => {
                if let Some(value) = max {
                    max = Some(add_count(value, 1)?);
                }
            }
            Op::Repeat { .. } => {
                let (second_min, second_max) = repeat_bounds(&r2.op)?;
                min = add_count(min, second_min)?;
                max = match (max, second_max) {
                    (_, None) | (None, _) => None,
                    (Some(a), Some(b)) => Some(add_count(a, b)?),
                };
            }
            Op::Literal(_) | Op::Class(_) | Op::AnyChar | Op::AnyByte => {
                min = add_count(min, 1)?;
                if let Some(value) = max {
                    max = Some(add_count(value, 1)?);
                }
            }
            Op::LiteralString(runes) => {
                let Op::Literal(rune) = self.node(atom)?.op else {
                    return Err(internal());
                };
                if runes.first() != Some(&rune) {
                    return Err(internal());
                }
                let mut count = 1;
                while count < runes.len() && runes[count] == rune {
                    count += 1;
                }
                let increment = u32::try_from(count).map_err(|_| internal())?;
                min = add_count(min, increment)?;
                if let Some(value) = max {
                    max = Some(add_count(value, increment)?);
                }
                if count != runes.len() {
                    remainder = Some((copy(&runes[count..])?, r2.flags));
                }
            }
            _ => return Err(internal()),
        }
        self.node_mut(result)?.op = Op::Repeat {
            child: atom,
            min,
            max,
        };
        if let Some((runes, flags)) = remainder {
            // Partial literal-string consumption leaves repeat on the LEFT.
            let tail = self.literal_string(&runes, flags)?;
            Ok((result, tail))
        } else {
            // All other paths leave a freshly constructed unflagged empty LEFT.
            let empty = self.new_node(Op::Empty, 0)?;
            Ok((empty, result))
        }
    }
}
