//! Selected real InputProcessor queries and its sole shared shortcut allocation.
//! Schema metadata is projected from the pinned IDD after ordinary raw admission.

use std::collections::{BTreeMap, BTreeSet};
mod input_statistics;
mod version;
pub use input_statistics::{
    SourceInputStatisticsFields, SourceInputStatisticsInput, SourceInputStatisticsValue,
};
pub use version::SourceVersionCheckError;

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
    // Same original epJSON operands, not computed statistics or a readiness flag.
    statistics_input: Option<SourceInputStatisticsInput>,
    source_direct_json_no_mass_fields: bool,
    // Canonical AirGap cache owner, distinct from retained raw/statistics/unused spelling.
    source_direct_json_air_gap_family: Option<String>,
    original_idf_fixed_fields: BTreeMap<String, Vec<(String, bool, String)>>,
    original_idf_extension_fields: BTreeMap<String, Vec<(String, bool, String)>>,
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
            statistics_input: None,
            source_direct_json_no_mass_fields: false,
            source_direct_json_air_gap_family: None,
            original_idf_fixed_fields: BTreeMap::new(),
            original_idf_extension_fields: BTreeMap::new(),
        })
    }

    /// Build the real Source direct-JSON cache while retaining its AirGap raw key.
    /// Other families and the ordinary/Original constructor keep exact ownership.
    ///
    /// # Errors
    /// Rejects uncoalesced AirGap roots, unknown families or inconsistent schema.
    pub fn from_admitted_source_direct_json_input(
        definitions: Vec<SourceObjectDefinition>,
        mut families: BTreeMap<String, SourceInputFamily>,
    ) -> Result<Self, SelectedInputUnavailable> {
        const FAMILY: &str = "Material:AirGap";
        let mut aliases = families
            .keys()
            .filter(|name| {
                name.as_str() != FAMILY && ep_model::source_json_c_string_prefix(name) == FAMILY
            })
            .cloned()
            .collect::<Vec<_>>();
        if aliases.len() > 1 || (!aliases.is_empty() && families.contains_key(FAMILY)) {
            return Err(SelectedInputUnavailable::Association);
        }
        let retained = aliases.pop();
        if let Some(name) = &retained {
            let objects = families
                .remove(name)
                .ok_or(SelectedInputUnavailable::Association)?;
            families.insert(FAMILY.into(), objects);
        }
        let mut input = Self::from_admitted_input(definitions, families)?;
        if let Some(name) = retained {
            let definition = input.source_object_definition(FAMILY)?;
            if (
                definition.alpha_fields,
                definition.numeric_fields,
                definition.alpha_extensions,
                definition.numeric_extensions,
            ) != (1, 1, 0, 0)
                || definition.extension.is_some()
            {
                return Err(SelectedInputUnavailable::Association);
            }
            let Some(SourceInputFamily::Objects(objects)) = input.families.get(FAMILY) else {
                return Err(SelectedInputUnavailable::Association);
            };
            for object in objects {
                input
                    .unused_inputs
                    .remove(&(FAMILY.into(), object.object_name.clone()));
                input
                    .unused_inputs
                    .insert((name.clone(), object.object_name.clone()));
            }
            input.source_direct_json_air_gap_family = Some(name);
        }
        Ok(input)
    }

    fn source_query_family<'a>(&self, family: &'a str) -> &'a str {
        if self.source_direct_json_air_gap_family.is_some()
            && ep_model::source_json_c_string_prefix(family) == "Material:AirGap"
        {
            "Material:AirGap"
        } else {
            family
        }
    }

    fn source_statistics_family<'a>(&'a self, family: &'a str) -> &'a str {
        if self.source_query_family(family) == "Material:AirGap" {
            self.source_direct_json_air_gap_family
                .as_deref()
                .unwrap_or(family)
        } else {
            family
        }
    }

    /// Retain genuine Source original-IDF fixed/extension field metadata from ingress.
    ///
    /// # Errors
    /// Rejects duplicate layouts or a layout inconsistent with the existing schema capacities.
    pub fn retain_source_original_idf_fields(
        &mut self,
        family: &str,
        fields: Vec<(String, bool, String)>,
        extensions: Vec<(String, bool, String)>,
    ) -> Result<(), SelectedInputUnavailable> {
        let definition = self.source_object_definition(family)?;
        let alphas = fields.iter().filter(|(_, alpha, _)| *alpha).count();
        let numerics = fields.len().saturating_sub(alphas);
        let ext_alphas = extensions.iter().filter(|(_, alpha, _)| *alpha).count();
        let ext_numerics = extensions.len().saturating_sub(ext_alphas);
        if (!fields.is_empty()
            && fields
                .first()
                .is_none_or(|(name, alpha, label)| name != "name" || !*alpha || label != "Name"))
            || (fields.is_empty()
                && (definition.alpha_fields != 0
                    || definition.numeric_fields != 0
                    || extensions.is_empty()))
            || fields
                .iter()
                .chain(&extensions)
                .any(|(name, _, label)| name.is_empty() || label.is_empty())
            || fields
                .iter()
                .map(|(name, _, _)| name)
                .collect::<BTreeSet<_>>()
                .len()
                != fields.len()
            || extensions
                .iter()
                .map(|(name, _, _)| name)
                .collect::<BTreeSet<_>>()
                .len()
                != extensions.len()
            || alphas != definition.alpha_fields
            || numerics != definition.numeric_fields
            || ext_alphas != definition.alpha_extensions
            || ext_numerics != definition.numeric_extensions
            || definition.extension.is_some() == extensions.is_empty()
            || self.original_idf_fixed_fields.contains_key(family)
        {
            return Err(SelectedInputUnavailable::Association);
        }
        self.original_idf_fixed_fields.insert(family.into(), fields);
        self.original_idf_extension_fields
            .insert(family.into(), extensions);
        Ok(())
    }

    pub(crate) fn source_original_idf_fixed_fields(
        &self,
        family: &str,
    ) -> Option<&[(String, bool, String)]> {
        self.original_idf_fixed_fields
            .get(family)
            .map(Vec::as_slice)
    }

    pub(crate) fn source_original_idf_argument_counts(
        &self,
        family: &str,
        maximum: usize,
        maximum_extensions: Option<usize>,
        retained_rows: usize,
    ) -> Result<SourceArgumentCounts, SelectedInputUnavailable> {
        let fields = self
            .original_idf_fixed_fields
            .get(family)
            .ok_or_else(|| SelectedInputUnavailable::Definition(family.into()))?;
        let extensions = self
            .original_idf_extension_fields
            .get(family)
            .ok_or_else(|| SelectedInputUnavailable::Definition(family.into()))?;
        if extensions.is_empty() && retained_rows != 0
            || !extensions.is_empty() && maximum_extensions.is_none()
        {
            return Err(SelectedInputUnavailable::Association);
        }
        let extra = extensions
            .len()
            .checked_mul(retained_rows)
            .ok_or(SelectedInputUnavailable::Size)?;
        let _total_fields = fields
            .len()
            .checked_add(extra)
            .ok_or(SelectedInputUnavailable::Size)?;
        let (mut alpha, mut numeric) = (0, 0);
        let mut counts = SourceArgumentCounts::default();
        for (position, (_, is_alpha, _)) in fields.iter().enumerate() {
            if *is_alpha {
                alpha += 1;
                if position == 0 || position < maximum {
                    counts.alphas = alpha;
                }
            } else {
                numeric += 1;
                if position < maximum {
                    counts.numerics = numeric;
                }
            }
        }
        for (position, (_, is_alpha, _)) in extensions.iter().cycle().take(extra).enumerate() {
            if *is_alpha {
                alpha += 1;
                if position < maximum_extensions.unwrap_or(0) {
                    counts.alphas = alpha;
                }
            } else {
                numeric += 1;
                if position < maximum_extensions.unwrap_or(0) {
                    counts.numerics = numeric;
                }
            }
        }
        counts.total()?;
        Ok(counts)
    }

    pub(crate) fn source_original_idf_field_names(
        &self,
        family: &str,
        retained_rows: usize,
        alpha_names: &mut [String],
        numeric_names: &mut [String],
    ) -> Result<(), SelectedInputUnavailable> {
        let fields = self
            .original_idf_fixed_fields
            .get(family)
            .ok_or_else(|| SelectedInputUnavailable::Definition(family.into()))?;
        let extensions = self
            .original_idf_extension_fields
            .get(family)
            .ok_or_else(|| SelectedInputUnavailable::Definition(family.into()))?;
        let extra = extensions
            .len()
            .checked_mul(retained_rows)
            .ok_or(SelectedInputUnavailable::Size)?;
        let (mut alpha, mut numeric) = (0, 0);
        for (_, is_alpha, label) in fields.iter().chain(extensions.iter().cycle().take(extra)) {
            let cell = if *is_alpha {
                let cell = alpha_names
                    .get_mut(alpha)
                    .ok_or(SelectedInputUnavailable::Size)?;
                alpha += 1;
                cell
            } else {
                let cell = numeric_names
                    .get_mut(numeric)
                    .ok_or(SelectedInputUnavailable::Size)?;
                numeric += 1;
                cell
            };
            *cell = label.clone();
        }
        Ok(())
    }

    fn canonical_type(&self, object_type: &str) -> Result<String, SelectedInputUnavailable> {
        let object_type = self.source_query_family(object_type);
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
        match self.families.get(self.source_query_family(section_word)) {
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
        match self.families.get(self.source_query_family(object_type)) {
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

    /// Literal epJSON.find identity: exact case, no insertion or count query.
    /// Distinguishes absent, an earlier definition-created null, and an empty map.
    pub(crate) fn find_exact_family(&self, object_type: &str) -> Option<&SourceInputFamily> {
        self.families.get(self.source_query_family(object_type))
    }

    /// Borrow the actual selected schema without getObjectDefMaxArgs null insertion.
    /// This is the bounded getObjectSchemaProps lookup used by direct JSON getters.
    pub(crate) fn source_object_definition(
        &self,
        object_type: &str,
    ) -> Result<&SourceObjectDefinition, SelectedInputUnavailable> {
        self.definitions
            .get(object_type)
            .ok_or_else(|| SelectedInputUnavailable::Definition(object_type.into()))
    }

    /// Actual presence, including null insertion, is observable by subsequent getters.
    pub fn family(&self, object_type: &str) -> Option<&SourceInputFamily> {
        let object_type = self.source_query_family(object_type);
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

impl SelectedInputProcessor {
    /// Same ordered unusedInputs set after every actual getter/markObjectAsUsed.
    pub(crate) fn source_unused_objects(&self) -> impl Iterator<Item = (&str, &str)> {
        self.unused_inputs
            .iter()
            .map(|(kind, name)| (kind.as_str(), name.as_str()))
    }
}
