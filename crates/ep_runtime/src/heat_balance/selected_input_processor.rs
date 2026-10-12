//! Selected real InputProcessor queries and its sole shared shortcut allocation.
//! Schema metadata is projected from the pinned IDD after ordinary raw admission.

use std::collections::{BTreeMap, BTreeSet};

/// Literal legacy_idd field counts; not an object-admission allowlist.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceObjectDefinition {
    /// Canonical schema family name.
    pub object_type: String,
    /// Count of non-extensible alpha fields, including Name where present.
    pub alpha_fields: usize,
    /// Count of non-extensible numeric fields.
    pub numeric_fields: usize,
    /// Alpha fields per actual extensible item.
    pub alpha_extensions: usize,
    /// Numeric fields per actual extensible item.
    pub numeric_extensions: usize,
    /// Actual legacy_idd extension key, or its absence.
    pub extension: Option<String>,
}

/// Actual JSON .size() values of the input fields, before schema queries mutate presence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceObjectShape {
    /// Exact original epJSON object-map key, including case and blank spelling.
    pub object_name: String,
    /// Actual input field names and their JSON .size() values.
    pub field_sizes: BTreeMap<String, usize>,
}

/// Distinguish an absent family from its real getObjectDefMaxArgs-created null.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SourceInputFamily {
    /// Real null inserted by a definition query on an absent family.
    Null,
    /// Actual input object map, including a present empty map.
    Objects(Vec<SourceObjectShape>),
}

/// Caller-owned selected raw/schema projection; positive admission is checked upstream.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectedInputProcessor {
    definitions: BTreeMap<String, SourceObjectDefinition>,
    families: BTreeMap<String, SourceInputFamily>,
    // Source initializeMaps/ObjectInfo lexicographic exact-case identity owner.
    unused_inputs: BTreeSet<(String, String)>,
}

/// Independent Original alpha/numeric argument counts.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SourceArgumentCounts {
    /// Maximum alpha argument count.
    pub alphas: usize,
    /// Maximum numeric argument count.
    pub numerics: usize,
}
impl SourceArgumentCounts {
    /// Sum the independent argument counts.
    ///
    /// # Errors
    /// Returns Size when the sum is not representable.
    pub fn total(self) -> Result<usize, SelectedInputUnavailable> {
        self.alphas
            .checked_add(self.numerics)
            .ok_or(SelectedInputUnavailable::Size)
    }
}

/// Explicit selected-schema or size boundary; never guessed query results.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SelectedInputUnavailable {
    /// The requested or present family has no represented schema definition.
    Definition(String),
    /// Empty or duplicate case-insensitive schema identity.
    Association,
    /// Actual extensible/count arithmetic exceeds addressable storage.
    Size,
}
impl std::fmt::Display for SelectedInputUnavailable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "selected InputProcessor unavailable: {self:?}")
    }
}
impl std::error::Error for SelectedInputUnavailable {}

impl SelectedInputProcessor {
    /// Retain complete actual family presence/shapes and the finite real schema closure.
    ///
    /// # Errors
    /// Rejects duplicate schema/object identities or an unrepresented present family.
    pub fn from_admitted_input(
        definitions: Vec<SourceObjectDefinition>,
        families: BTreeMap<String, SourceInputFamily>,
    ) -> Result<Self, SelectedInputUnavailable> {
        let mut by_name = BTreeMap::new();
        for definition in definitions {
            if definition.object_type.is_empty()
                || by_name
                    .keys()
                    .any(|key: &String| key.eq_ignore_ascii_case(&definition.object_type))
            {
                return Err(SelectedInputUnavailable::Association);
            }
            by_name.insert(definition.object_type.clone(), definition);
        }
        for name in families.keys() {
            if !by_name.contains_key(name) {
                return Err(SelectedInputUnavailable::Definition(name.clone()));
            }
        }
        let mut unused_inputs = BTreeSet::new();
        for (object_type, family) in &families {
            if let SourceInputFamily::Objects(objects) = family {
                for object in objects {
                    if !unused_inputs.insert((object_type.clone(), object.object_name.clone())) {
                        return Err(SelectedInputUnavailable::Association);
                    }
                }
            }
        }
        Ok(Self {
            definitions: by_name,
            families,
            unused_inputs,
        })
    }

    fn canonical_type(&self, object_type: &str) -> Result<String, SelectedInputUnavailable> {
        self.definitions
            .keys()
            .find(|key| key.eq_ignore_ascii_case(object_type))
            .cloned()
            .ok_or_else(|| SelectedInputUnavailable::Definition(object_type.into()))
    }

    /// getNumObjectsFound does not insert an absent family.
    pub fn get_num_objects_found(&self, object_type: &str) -> usize {
        match self.family(object_type) {
            Some(SourceInputFamily::Objects(objects)) => objects.len(),
            Some(SourceInputFamily::Null) | None => 0,
        }
    }

    /// Return the actual exact-case section size, or -1 for an absent section.
    /// Unlike object counts this does not canonicalize or insert a family.
    ///
    /// # Errors
    /// Returns Size when a present section count exceeds the Original signed int.
    pub fn get_num_sections_found(
        &self,
        section_word: &str,
    ) -> Result<i32, SelectedInputUnavailable> {
        match self.families.get(section_word) {
            None => Ok(-1),
            Some(SourceInputFamily::Null) => Ok(0),
            Some(SourceInputFamily::Objects(objects)) => {
                i32::try_from(objects.len()).map_err(|_| SelectedInputUnavailable::Size)
            }
        }
    }

    /// Inspect permanent exact-case raw identity, including an already used object.
    /// This does not consult or mutate the distinct unusedInputs owner.
    pub fn has_object(&self, object_type: &str, object_name: &str) -> bool {
        match self.families.get(object_type) {
            Some(SourceInputFamily::Objects(objects)) => objects
                .iter()
                .any(|object| object.object_name == object_name),
            Some(SourceInputFamily::Null) | None => false,
        }
    }

    /// Actual markObjectAsUsed exact-case erase; absent/already-used is a no-op.
    pub fn mark_object_as_used(&mut self, object_type: &str, object_name: &str) {
        self.unused_inputs
            .remove(&(object_type.into(), object_name.into()));
    }

    /// Inspect the actual remaining exact-case unused object identity.
    /// False for an absent or already-consumed object, not a readiness decision.
    pub fn is_object_unused(&self, object_type: &str, object_name: &str) -> bool {
        self.unused_inputs
            .contains(&(object_type.into(), object_name.into()))
    }

    /// Actual presence, including null insertion, is observable by subsequent getters.
    pub fn family(&self, object_type: &str) -> Option<&SourceInputFamily> {
        self.families.get(object_type).or_else(|| {
            self.families
                .iter()
                .find(|(key, _)| key.eq_ignore_ascii_case(object_type))
                .map(|(_, family)| family)
        })
    }

    fn extension_size(family: &SourceInputFamily, key: &str) -> usize {
        match family {
            SourceInputFamily::Null => 0,
            SourceInputFamily::Objects(objects) => objects
                .iter()
                .filter_map(|object| object.field_sizes.get(key))
                .copied()
                .max()
                .unwrap_or(0),
        }
    }

    fn counts(
        definition: &SourceObjectDefinition,
        extensions: usize,
    ) -> Result<SourceArgumentCounts, SelectedInputUnavailable> {
        let alphas = definition
            .alpha_extensions
            .checked_mul(extensions)
            .and_then(|count| count.checked_add(definition.alpha_fields))
            .ok_or(SelectedInputUnavailable::Size)?;
        let numerics = definition
            .numeric_extensions
            .checked_mul(extensions)
            .and_then(|count| count.checked_add(definition.numeric_fields))
            .ok_or(SelectedInputUnavailable::Size)?;
        let counts = SourceArgumentCounts { alphas, numerics };
        counts.total()?;
        Ok(counts)
    }

    /// Literal getMaxSchemaArgs: lexical family traversal, independent maxima and
    /// the Original extension_key lifetime spanning the complete traversal.
    ///
    /// # Errors
    /// Rejects an unrepresented definition or unrepresentable actual array size.
    pub fn get_max_schema_args(&self) -> Result<SourceArgumentCounts, SelectedInputUnavailable> {
        let mut result = SourceArgumentCounts::default();
        let mut extension_key = "";
        for (name, family) in &self.families {
            let definition = self
                .definitions
                .get(name)
                .ok_or_else(|| SelectedInputUnavailable::Definition(name.clone()))?;
            if let Some(extension) = &definition.extension {
                extension_key = extension;
            }
            let counts = Self::counts(definition, Self::extension_size(family, extension_key))?;
            result.alphas = result.alphas.max(counts.alphas);
            result.numerics = result.numerics.max(counts.numerics);
        }
        result.total()?;
        Ok(result)
    }

    /// Literal getObjectDefMaxArgs: absent known family becomes null, with a
    /// fresh local extension key for this one definition query.
    ///
    /// # Errors
    /// Rejects a definition outside the finite schema closure or count overflow.
    pub fn get_object_def_max_args(
        &mut self,
        object_type: &str,
    ) -> Result<SourceArgumentCounts, SelectedInputUnavailable> {
        let canonical = self.canonical_type(object_type)?;
        let definition = self
            .definitions
            .get(&canonical)
            .ok_or_else(|| SelectedInputUnavailable::Definition(canonical.clone()))?;
        let family = self
            .families
            .entry(canonical)
            .or_insert(SourceInputFamily::Null);
        Self::counts(
            definition,
            Self::extension_size(family, definition.extension.as_deref().unwrap_or("")),
        )
    }
}

/// Sole dataIPShortCut member set, borrowed by the actual reached getters.
/// InputProcessor.cc315–320 initializes blank flags false, not getter-local true.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SourceInputProcessorShortCut {
    /// Current getter's actual object-family name.
    pub current_module_object: String,
    /// Shared alpha arguments, initially empty strings.
    pub alpha_args: Vec<String>,
    /// Shared numeric arguments, initially positive zero.
    pub numeric_args: Vec<f64>,
    /// Shared alpha field names, initially empty strings.
    pub alpha_field_names: Vec<String>,
    /// Shared numeric field names, initially empty strings.
    pub numeric_field_names: Vec<String>,
    /// Shared alpha blank flags, initially false.
    pub alpha_field_blanks: Vec<bool>,
    /// Shared numeric blank flags, initially false.
    pub numeric_field_blanks: Vec<bool>,
}

impl SourceInputProcessorShortCut {
    /// Actual initializeMaps/getMaxSchemaArgs-dependent shared array allocation.
    ///
    /// # Errors
    /// Returns an explicit unavailable definition or size boundary before writes.
    pub fn allocate_from_input(
        &mut self,
        input: &SelectedInputProcessor,
    ) -> Result<(), SelectedInputUnavailable> {
        let counts = input.get_max_schema_args()?;
        self.alpha_field_names = vec![String::new(); counts.alphas];
        self.alpha_args = vec![String::new(); counts.alphas];
        self.alpha_field_blanks = vec![false; counts.alphas];
        self.numeric_field_names = vec![String::new(); counts.numerics];
        self.numeric_args = vec![0.0; counts.numerics];
        self.numeric_field_blanks = vec![false; counts.numerics];
        Ok(())
    }
}
