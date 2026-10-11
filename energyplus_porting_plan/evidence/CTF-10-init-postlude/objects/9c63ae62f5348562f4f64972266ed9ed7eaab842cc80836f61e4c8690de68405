//! Retained predefined registry and the complete original registration sequence.
//!
//! `Default` corresponds to EnergyPlusData's value-initialized registry owner.
//! Import actual retained owners for a caller that already initialized it.

use super::ctf_reports::CtfReportUnavailable;
use std::collections::BTreeMap;

/// Actual reportNameType, including the mutable report visibility flag.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PredefinedReportName {
    /// Source report key.
    pub name: String,
    /// Source namewithspaces.
    pub name_with_spaces: String,
    /// Source abbreviation.
    pub abbreviation: String,
    /// Actual report selection owner; creation resets this member to false.
    pub show: bool,
}

/// Actual SubTableType; creation leaves an existing slot's footnote untouched.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PredefinedSubtable {
    /// Source subtable name.
    pub name: String,
    /// One-based source report index.
    pub report_index: i32,
    /// Actual retained footnote.
    pub footnote: String,
}

/// Actual ColumnTagType.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PredefinedColumn {
    /// Literal column heading.
    pub heading: String,
    /// One-based source subtable index.
    pub subtable_index: i32,
}

/// Actual allocated registry arrays, counters, named indices and layer vector.
///
/// Full allocated vectors include unoccupied slots. Each occupied index is
/// one-based. Table entries and unrelated report aggregates remain caller-owned.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PredefinedRegistry {
    /// Actual reportName allocation, including its unused tail.
    pub reports: Option<Vec<PredefinedReportName>>,
    /// Actual sizeReportName.
    pub report_capacity: i32,
    /// Actual numReportName.
    pub report_count: i32,
    /// Actual subTable allocation, including its unused tail.
    pub subtables: Option<Vec<PredefinedSubtable>>,
    /// Actual sizeSubTable.
    pub subtable_capacity: i32,
    /// Actual numSubTable.
    pub subtable_count: i32,
    /// Actual columnTag allocation, including its unused tail.
    pub columns: Option<Vec<PredefinedColumn>>,
    /// Actual sizeColumnTag.
    pub column_capacity: i32,
    /// Actual numColumnTag.
    pub column_count: i32,
    /// All 1,301 declared named IDs, using their exact Source member names.
    pub named_indices: BTreeMap<String, i32>,
    /// Actual retained pdchOpqConsLayCol; Set appends ten entries each visit.
    pub opaque_layer_columns: Vec<i32>,
}

include!("predefined_tables_names.rs");

impl Default for PredefinedRegistry {
    fn default() -> Self {
        Self {
            reports: None,
            report_capacity: 0,
            report_count: 0,
            subtables: None,
            subtable_capacity: 0,
            subtable_count: 0,
            columns: None,
            column_capacity: 0,
            column_count: 0,
            named_indices: NATIVE_INDEX_NAMES
                .iter()
                .map(|name| ((*name).to_owned(), 0))
                .collect(),
            opaque_layer_columns: Vec::new(),
        }
    }
}

// These three Source helpers share exactly this allocation/counter sequence.
// Rust errors represent unrepresentable supplied owners or undefined signed
// counter arithmetic; they do not invent a Native diagnostic or returned path.
fn next_slot<T: Clone + Default>(
    owner: &mut Option<Vec<T>>,
    capacity: &mut i32,
    count: &mut i32,
) -> Result<usize, CtfReportUnavailable> {
    if let Some(slots) = owner {
        let size = usize::try_from(*capacity).map_err(|_| CtfReportUnavailable::TableOwner)?;
        if slots.len() != size || *count < 0 || *count > *capacity {
            return Err(CtfReportUnavailable::TableOwner);
        }
        *count = count
            .checked_add(1)
            .ok_or(CtfReportUnavailable::TableCapacity)?;
        if *count > *capacity {
            *capacity = capacity
                .checked_mul(2)
                .ok_or(CtfReportUnavailable::TableCapacity)?;
            let new_size =
                usize::try_from(*capacity).map_err(|_| CtfReportUnavailable::TableCapacity)?;
            slots.resize(new_size, T::default());
        }
        let index = usize::try_from(*count - 1).map_err(|_| CtfReportUnavailable::TableOwner)?;
        if index >= slots.len() {
            return Err(CtfReportUnavailable::TableOwner);
        }
        Ok(index)
    } else {
        *owner = Some(vec![T::default(); 100]);
        *capacity = 100;
        *count = 1;
        Ok(0)
    }
}

impl PredefinedRegistry {
    /// Full SetPredefinedTables sequence with the caller's actual sizing flags.
    /// Existing arrays, named IDs and layer vector are retained across visits.
    pub fn set_predefined_tables(
        &mut self,
        do_zone_sizing: bool,
        do_system_sizing: bool,
    ) -> Result<(), CtfReportUnavailable> {
        if self.named_indices.len() != NATIVE_INDEX_NAMES.len()
            || NATIVE_INDEX_NAMES
                .iter()
                .any(|name| !self.named_indices.contains_key(*name))
        {
            return Err(CtfReportUnavailable::TableOwner);
        }
        self.register_catalog(do_zone_sizing, do_system_sizing)
    }

    /// Genuine newPreDefReport allocation, record writes and returned index.
    pub fn new_report(
        &mut self,
        name: &str,
        abbreviation: &str,
        name_with_spaces: &str,
    ) -> Result<i32, CtfReportUnavailable> {
        let index = next_slot(
            &mut self.reports,
            &mut self.report_capacity,
            &mut self.report_count,
        )?;
        let slots = self
            .reports
            .as_mut()
            .ok_or(CtfReportUnavailable::TableOwner)?;
        let record = &mut slots[index];
        record.name = name.to_owned();
        record.abbreviation = abbreviation.to_owned();
        record.name_with_spaces = name_with_spaces.to_owned();
        record.show = false;
        Ok(self.report_count)
    }

    /// Genuine newPreDefSubTable; unrelated retained slot fields are not reset.
    pub fn new_subtable(
        &mut self,
        report_index: i32,
        name: &str,
    ) -> Result<i32, CtfReportUnavailable> {
        let index = next_slot(
            &mut self.subtables,
            &mut self.subtable_capacity,
            &mut self.subtable_count,
        )?;
        let slots = self
            .subtables
            .as_mut()
            .ok_or(CtfReportUnavailable::TableOwner)?;
        slots[index].name = name.to_owned();
        slots[index].report_index = report_index;
        Ok(self.subtable_count)
    }

    /// Genuine newPreDefColumn; the caller supplies the actual subtable index.
    pub fn new_column(
        &mut self,
        subtable_index: i32,
        heading: &str,
    ) -> Result<i32, CtfReportUnavailable> {
        let index = next_slot(
            &mut self.columns,
            &mut self.column_capacity,
            &mut self.column_count,
        )?;
        let slots = self
            .columns
            .as_mut()
            .ok_or(CtfReportUnavailable::TableOwner)?;
        slots[index].heading = heading.to_owned();
        slots[index].subtable_index = subtable_index;
        Ok(self.column_count)
    }

    /// Source range guard and write. Source index zero dereferences outside the
    /// one-based array; this Rust representation returns unavailable there.
    pub fn add_footnote(
        &mut self,
        subtable_index: i32,
        text: &str,
    ) -> Result<(), CtfReportUnavailable> {
        if subtable_index < 0 || subtable_index > self.subtable_count {
            return Ok(());
        }
        let index =
            usize::try_from(subtable_index - 1).map_err(|_| CtfReportUnavailable::TableOwner)?;
        let record = self
            .subtables
            .as_mut()
            .and_then(|slots| slots.get_mut(index))
            .ok_or(CtfReportUnavailable::TableOwner)?;
        record.footnote = text.to_owned();
        Ok(())
    }

    /// Borrows the genuine retained layer-column IDs for reportLayers.
    pub fn opaque_layer_columns(&self) -> &[i32] {
        &self.opaque_layer_columns
    }

    fn named_index(&self, binding: &str) -> Result<i32, CtfReportUnavailable> {
        self.named_indices
            .get(binding)
            .copied()
            .ok_or(CtfReportUnavailable::TableOwner)
    }
    fn register_report(
        &mut self,
        binding: &str,
        name: &str,
        abbreviation: &str,
        pretty: &str,
    ) -> Result<(), CtfReportUnavailable> {
        let index = self.new_report(name, abbreviation, pretty)?;
        self.named_indices.insert(binding.to_owned(), index);
        Ok(())
    }
    fn register_subtable(
        &mut self,
        binding: &str,
        parent: &str,
        name: &str,
    ) -> Result<(), CtfReportUnavailable> {
        let index = self.new_subtable(self.named_index(parent)?, name)?;
        self.named_indices.insert(binding.to_owned(), index);
        Ok(())
    }
    fn register_column(
        &mut self,
        binding: &str,
        parent: &str,
        heading: &str,
    ) -> Result<(), CtfReportUnavailable> {
        let index = self.new_column(self.named_index(parent)?, heading)?;
        self.named_indices.insert(binding.to_owned(), index);
        Ok(())
    }
    fn register_layer_column(
        &mut self,
        parent: &str,
        heading: &str,
    ) -> Result<(), CtfReportUnavailable> {
        let index = self.new_column(self.named_index(parent)?, heading)?;
        self.opaque_layer_columns.push(index);
        Ok(())
    }
    fn register_footnote(&mut self, parent: &str, text: &str) -> Result<(), CtfReportUnavailable> {
        self.add_footnote(self.named_index(parent)?, text)
    }
}

include!("predefined_tables_catalog.rs");

#[cfg(test)]
#[path = "predefined_tables_tests.rs"]
mod tests;
