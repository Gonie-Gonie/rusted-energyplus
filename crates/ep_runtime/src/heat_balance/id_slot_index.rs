//! Dense typed-ID lookup with a deterministic sparse-model fallback.

/// Maps a public typed-model ID to its arena slot without a linear scan.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum TypedIdSlotIndex {
    /// Compiler-produced arenas use `id == slot` and need no auxiliary storage.
    DenseIdentity,
    /// Manually assembled arenas may be sparse, reordered, or contain duplicate IDs.
    Sparse(Box<[(u32, usize)]>),
}

impl TypedIdSlotIndex {
    pub(crate) fn new(ids: impl IntoIterator<Item = u32>) -> Self {
        let mut candidates = ids
            .into_iter()
            .enumerate()
            .map(|(slot, id)| (id, slot))
            .collect::<Vec<_>>();
        if candidates
            .iter()
            .all(|(id, slot)| usize::try_from(*id).is_ok_and(|id| id == *slot))
        {
            return Self::DenseIdentity;
        }

        // The slot tie-break keeps the prior linear lookup's first-ID-wins behavior.
        candidates.sort_unstable_by_key(|(id, slot)| (*id, *slot));
        candidates.dedup_by_key(|(id, _slot)| *id);
        Self::Sparse(candidates.into_boxed_slice())
    }

    pub(crate) fn slot(&self, id: u32) -> Option<usize> {
        match self {
            Self::DenseIdentity => usize::try_from(id).ok(),
            Self::Sparse(entries) => entries
                .binary_search_by_key(&id, |(entry_id, _slot)| *entry_id)
                .ok()
                .map(|index| entries[index].1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TypedIdSlotIndex;

    #[test]
    fn dense_sparse_and_duplicate_ids_resolve_without_linear_lookup() {
        let dense = TypedIdSlotIndex::new([0, 1, 2]);
        assert_eq!(dense, TypedIdSlotIndex::DenseIdentity);
        assert_eq!(dense.slot(2), Some(2));

        let sparse = TypedIdSlotIndex::new([42, 7, 42]);
        assert_eq!(sparse.slot(7), Some(1));
        assert_eq!(sparse.slot(42), Some(0));
        assert_eq!(sparse.slot(99), None);
    }
}
