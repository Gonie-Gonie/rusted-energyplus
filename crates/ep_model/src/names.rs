//! EnergyPlus-compatible name normalization and name maps.

use std::collections::BTreeMap;

/// Normalized name used only during compile, diagnostics, and export.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NormalizedName(pub String);

impl NormalizedName {
    /// Applies the first EnergyPlus-compatible name normalization rule.
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self(name.trim().to_ascii_uppercase())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum NameNormalization {
    TrimmedAsciiUpper,
    SourceScheduleAsciiUpper,
}

/// Compile-time name map from EnergyPlus names to typed IDs.
/// Defaults preserve legacy trimming; explicitly selected Source namespaces keep
/// every authored whitespace byte while applying the same ASCII case folding.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NameMap<T> {
    by_name: BTreeMap<NormalizedName, T>,
    by_id: Vec<NormalizedName>,
    normalization: NameNormalization,
}

impl<T> Default for NameMap<T> {
    fn default() -> Self {
        Self {
            by_name: BTreeMap::new(),
            by_id: Vec::new(),
            normalization: NameNormalization::TrimmedAsciiUpper,
        }
    }
}

impl<T: Copy> NameMap<T> {
    /// Constructs the exact-name namespace for explicit Source schedule maps.
    /// This established API keeps the same name and reference behavior.
    #[must_use]
    pub fn source_schedule_namespace() -> Self {
        Self::source_exact_namespace()
    }

    /// Constructs an explicitly selected Source object namespace.
    /// Preserve every authored whitespace byte; legacy maps keep `Default`.
    #[must_use]
    pub fn source_exact_namespace() -> Self {
        Self {
            normalization: NameNormalization::SourceScheduleAsciiUpper,
            ..Self::default()
        }
    }

    /// Uses this map's identity policy for an arena name or reference.
    #[must_use]
    pub fn normalized_name(&self, name: &str) -> NormalizedName {
        match self.normalization {
            NameNormalization::TrimmedAsciiUpper => NormalizedName::new(name),
            NameNormalization::SourceScheduleAsciiUpper => {
                NormalizedName(name.to_ascii_uppercase())
            }
        }
    }

    /// Inserts a normalized name and returns the existing ID on duplicate.
    pub fn insert(&mut self, name: &str, id: T) -> Option<T> {
        let normalized = self.normalized_name(name);
        if let Some(existing) = self.by_name.get(&normalized) {
            return Some(*existing);
        }

        self.by_name.insert(normalized.clone(), id);
        self.by_id.push(normalized);
        None
    }

    /// Resolves a raw EnergyPlus name to a typed ID.
    #[must_use]
    pub fn resolve(&self, name: &str) -> Option<T> {
        self.by_name.get(&self.normalized_name(name)).copied()
    }

    /// Number of registered names.
    #[must_use]
    pub fn len(&self) -> usize {
        self.by_id.len()
    }

    /// Returns true when no names are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.by_id.is_empty()
    }

    /// Names in typed ID insertion order.
    #[must_use]
    pub fn names(&self) -> &[NormalizedName] {
        &self.by_id
    }
}
