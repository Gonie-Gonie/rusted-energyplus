//! Actual predefined string entries and selected no-source CTF EIO reporting.

use super::ctf_public_initialization::ConstructionCtfPublicInitialization;
use super::surface_manager::ctf_public_storage::CtfPublicCoefficientStorage;
use ep_model::{
    ConstructionId, Material, MaterialDefinition, MaterialId, MaterialSurfaceRoughness,
    NormalizedName, TypedModel,
};
use std::io::{self, Write};

#[path = "ctf_reports_format.rs"]
pub(super) mod formatting;

/// Sole OutputReports.hh79-95 constructor state, retained across exporters.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceOutputReports {
    pub(crate) option_done: bool,
    pub(crate) last_option: String,
}

/// Honest scope/representation boundaries; none is an inferred Native guard.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CtfReportUnavailable {
    /// Original typed declaration, layer or loader-name custody is inconsistent.
    Association,
    /// A selected ordinary no-source construction is required.
    UnsupportedConstruction,
    /// The real public suffix has no returned owner.
    PublicOwnerUnavailable,
    /// Actual material ID whose selected loader projection is missing or unsupported.
    Material(MaterialId),
    /// A genuine source loader name has not been retained for this owner.
    LoaderNameUnavailable,
    /// Actual surface boundary is outside the ordinary Outdoors/Adiabatic report port.
    SurfaceBoundary(ep_model::SurfaceId),
    /// Public count cannot index the fixed19 owner; no clipping is performed.
    PublicCapacity(i32),
    /// Source's declared output counter cannot be represented as i32.
    DeclaredCounter,
    /// Allocated predefined array/count/size owners are not jointly representable.
    TableOwner,
    /// Source integer increment/doubling cannot be represented.
    TableCapacity,
    /// Actual layer column is absent; Native's unchecked access is not certified.
    MissingLayerColumn,
    /// Decimal text or formatting precision is not representable.
    DecimalRepresentation,
}

/// Source scope and actual Rust sink errors remain separate outcomes.
#[derive(Debug)]
pub enum CtfReportError {
    /// An honest selected scope or owner boundary.
    Unavailable(CtfReportUnavailable),
    /// The actual supplied EIO writer failed; no Native backend outcome is inferred.
    Io(io::Error),
}

impl From<CtfReportUnavailable> for CtfReportError {
    fn from(error: CtfReportUnavailable) -> Self {
        Self::Unavailable(error)
    }
}
impl From<io::Error> for CtfReportError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}
impl std::fmt::Display for CtfReportError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unavailable(error) => {
                write!(formatter, "selected CTF report unavailable: {error:?}")
            }
            Self::Io(error) => std::fmt::Display::fmt(error, formatter),
        }
    }
}
impl std::error::Error for CtfReportError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Unavailable(_) => None,
        }
    }
}

/// Full TableEntryType members, including fields untouched by string entry writes.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PredefinedTableEntry {
    /// Actual charEntry.
    pub char_entry: Vec<u8>,
    /// Actual objectName.
    pub object_name: String,
    /// Actual indexColumn, supplied by the real predefined column registry.
    pub index_column: i32,
    /// Actual subTableIndex, not assigned by the string overload.
    pub sub_table_index: i32,
    /// Actual uniqueObjName, not assigned by the string overload.
    pub unique_object_name: i32,
    /// Actual origRealEntry, not assigned by the string overload.
    pub original_real_entry: f64,
    /// Actual significantDigits, not assigned by the string overload.
    pub significant_digits: i32,
    /// Actual origEntryIsReal, not assigned by the string overload.
    pub original_entry_is_real: bool,
}

/// Retained real predefined array allocation, capacity and occupied count.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PredefinedStringTable {
    /// None is unallocated; Some retains every allocated slot, including unused ones.
    pub entries: Option<Vec<PredefinedTableEntry>>,
    /// Actual sizeTableEntry, independent of allocator Vec capacity.
    pub size_table_entry: i32,
    /// Actual numTableEntry, a one-based occupied count.
    pub num_table_entry: i32,
}

impl PredefinedStringTable {
    /// Literal incrementTableEntry then the string PreDefTableEntry assignments.
    /// The real column ID is retained; this helper creates no substitute registry.
    pub fn string_entry(
        &mut self,
        column: i32,
        object: &str,
        text: &str,
    ) -> Result<(), CtfReportUnavailable> {
        self.raw_byte_entry(column, object, text.as_bytes())
    }

    /// The same Native string overload with byte-preserving std::string storage.
    /// STAT input can contain legacy non-UTF8 bytes; no decoding or mirror is used.
    pub(crate) fn raw_byte_entry(
        &mut self,
        column: i32,
        object: &str,
        text: &[u8],
    ) -> Result<(), CtfReportUnavailable> {
        let index = self.next_table_entry()?;
        let entry = self
            .entries
            .as_mut()
            .and_then(|entries| entries.get_mut(index))
            .ok_or(CtfReportUnavailable::TableOwner)?;
        entry.char_entry = text.to_vec();
        entry.object_name = object.to_string();
        entry.index_column = column;
        Ok(())
    }

    /// Literal shared OutputReportPredefined incrementTableEntry owner.
    fn next_table_entry(&mut self) -> Result<usize, CtfReportUnavailable> {
        let index = if let Some(entries) = &mut self.entries {
            let size = usize::try_from(self.size_table_entry)
                .map_err(|_| CtfReportUnavailable::TableOwner)?;
            if size == 0
                || entries.len() != size
                || self.num_table_entry < 0
                || self.num_table_entry > self.size_table_entry
            {
                return Err(CtfReportUnavailable::TableOwner);
            }
            let next = self
                .num_table_entry
                .checked_add(1)
                .ok_or(CtfReportUnavailable::TableCapacity)?;
            if next > self.size_table_entry {
                let new_size = self
                    .size_table_entry
                    .checked_mul(2)
                    .ok_or(CtfReportUnavailable::TableCapacity)?;
                entries.resize(
                    usize::try_from(new_size).map_err(|_| CtfReportUnavailable::TableCapacity)?,
                    PredefinedTableEntry::default(),
                );
                self.size_table_entry = new_size;
            }
            self.num_table_entry = next;
            usize::try_from(next - 1).map_err(|_| CtfReportUnavailable::TableOwner)?
        } else {
            self.entries = Some(vec![PredefinedTableEntry::default(); 100]);
            self.size_table_entry = 100;
            self.num_table_entry = 1;
            0
        };
        Ok(index)
    }

    /// Genuine Integer PreDefTableEntry1877-1915. Formatting follows the actual
    /// counter/allocation prefix and preserves all unrelated retained fields.
    pub(crate) fn integer_entry(
        &mut self,
        column: i32,
        object: &str,
        value: i32,
    ) -> Result<(), CtfReportUnavailable> {
        let index = self.next_table_entry()?;
        let entry = self
            .entries
            .as_mut()
            .and_then(|entries| entries.get_mut(index))
            .ok_or(CtfReportUnavailable::TableOwner)?;
        entry.char_entry = format!("{value:12}").into_bytes();
        entry.object_name = object.to_string();
        entry.index_column = column;
        Ok(())
    }

    /// Real PreDefTableEntry selected fixed-notation branch, default precision 2.
    pub(crate) fn fixed_real_entry_2(
        &mut self,
        column: i32,
        object: &str,
        value: f64,
    ) -> Result<(), CtfReportUnavailable> {
        self.fixed_real_entry(column, object, value, 2)
    }

    /// Real PreDefTableEntry fixed-notation branch with explicit precision 3.
    pub(crate) fn fixed_real_entry_3(
        &mut self,
        column: i32,
        object: &str,
        value: f64,
    ) -> Result<(), CtfReportUnavailable> {
        self.fixed_real_entry(column, object, value, 3)
    }

    /// Same genuine fixed-notation writer with explicit precision0.
    pub(crate) fn fixed_real_entry_0(
        &mut self,
        column: i32,
        object: &str,
        value: f64,
    ) -> Result<(), CtfReportUnavailable> {
        self.fixed_real_entry(column, object, value, 0)
    }

    /// Same genuine fixed-notation writer with explicit precision4.
    pub(crate) fn fixed_real_entry_4(
        &mut self,
        column: i32,
        object: &str,
        value: f64,
    ) -> Result<(), CtfReportUnavailable> {
        self.fixed_real_entry(column, object, value, 4)
    }

    fn fixed_real_entry(
        &mut self,
        column: i32,
        object: &str,
        value: f64,
        precision: i32,
    ) -> Result<(), CtfReportUnavailable> {
        if !value.is_finite() || value.abs() >= 1e8 {
            return Err(CtfReportUnavailable::DecimalRepresentation);
        }
        let mut text = formatting::fixed(value, 11, precision as usize);
        if text.len() > 12 {
            text = "  Too Big".to_string();
        }
        self.string_entry(column, object, &text)?;
        let index = usize::try_from(self.num_table_entry - 1)
            .map_err(|_| CtfReportUnavailable::TableOwner)?;
        let entry = self
            .entries
            .as_mut()
            .and_then(|rows| rows.get_mut(index))
            .ok_or(CtfReportUnavailable::TableOwner)?;
        entry.original_real_entry = value;
        entry.significant_digits = precision;
        entry.original_entry_is_real = true;
        Ok(())
    }
}

/// A selected actual material's report members, borrowed by original typed ID.
#[derive(Clone, Debug, PartialEq)]
pub struct CtfMaterialReport<'a> {
    /// Actual original material identity.
    pub material_id: MaterialId,
    /// Actual loader Name; regular spelling differs from getObjectItem names.
    pub name: &'a str,
    /// True only for actual Group::AirGap's different row format.
    pub air_gap: bool,
    /// Thickness, conductivity, density, specific heat and stored resistance.
    /// NoMass's first four entries are actual positive-zero MaterialBase defaults.
    pub values: [f64; 5],
}

fn loader_name<I: Copy + Eq>(names: &[(I, String)], id: I) -> Result<&str, CtfReportUnavailable> {
    let mut found = names.iter().filter(|(key, _)| *key == id);
    let value = found
        .next()
        .ok_or(CtfReportUnavailable::LoaderNameUnavailable)?;
    if found.next().is_some() {
        return Err(CtfReportUnavailable::Association);
    }
    Ok(&value.1)
}

fn material_report<'a>(
    model: &'a TypedModel,
    material: &'a Material,
) -> Result<CtfMaterialReport<'a>, CtfReportUnavailable> {
    let name = loader_name(&model.ctf_report_names.materials, material.id)?;
    if NormalizedName::new(name) != material.name {
        return Err(CtfReportUnavailable::Association);
    }
    let (air_gap, values) = match material.definition {
        MaterialDefinition::Regular(value) => (
            false,
            [
                value.thickness_m,
                value.conductivity_w_per_m_k,
                value.density_kg_per_m3,
                value.specific_heat_j_per_kg_k,
                value.thermal_resistance_m2_k_per_w,
            ],
        ),
        MaterialDefinition::NoMass(value) => (
            false,
            [0.0, 0.0, 0.0, 0.0, value.thermal_resistance_m2_k_per_w],
        ),
        MaterialDefinition::AirGap(value) => (
            true,
            [0.0, 0.0, 0.0, 0.0, value.thermal_resistance_m2_k_per_w],
        ),
        _ => return Err(CtfReportUnavailable::Material(material.id)),
    };
    Ok(CtfMaterialReport {
        material_id: material.id,
        name,
        air_gap,
        values,
    })
}

/// Borrowed actual construction/public/material report operands, without generation.
#[derive(Clone, Debug, PartialEq)]
pub struct CtfConstructionReport<'a> {
    /// Original typed identity, retained separately from the report counter.
    pub construction_id: ConstructionId,
    /// One-based original declared position; unused rows still occupy positions.
    pub declared_counter: i32,
    /// Actual loader construction Name.
    pub name: &'a str,
    /// Original outside-to-inside layers, without merge/reorder.
    pub layers: Vec<CtfMaterialReport<'a>>,
    /// Genuine selected public owner, including advanced caller dt/history metadata.
    pub public: &'a CtfPublicCoefficientStorage,
    /// Outside thermal, inside thermal, outside solar and inside solar absorptance.
    pub absorptance: [f64; 4],
    /// Actual selected outside-material roughness.
    pub outside_roughness: MaterialSurfaceRoughness,
}

/// Reuse actual compiled ordinary members and the same returned public row.
/// Used-row selection and completed-loop/fatal custody remain actual caller duties.
pub fn prepare_ctf_construction_report<'a>(
    model: &'a TypedModel,
    row: &'a ConstructionCtfPublicInitialization,
) -> Result<CtfConstructionReport<'a>, CtfReportUnavailable> {
    let construction = model
        .constructions
        .get(row.declared_index)
        .ok_or(CtfReportUnavailable::Association)?;
    if construction.id != row.construction_id
        || construction.effective_layers() != row.original_material_ids
        || !construction.is_ordinary_opaque()
        || construction.internal_heat_source.is_some()
    {
        return Err(CtfReportUnavailable::UnsupportedConstruction);
    }
    let name = loader_name(&model.ctf_report_names.constructions, construction.id)?;
    if NormalizedName::new(name) != construction.name {
        return Err(CtfReportUnavailable::Association);
    }
    let public = &row
        .result
        .as_ref()
        .map_err(|_| CtfReportUnavailable::PublicOwnerUnavailable)?
        .storage;
    let materials = row
        .original_material_ids
        .iter()
        .map(|id| {
            model
                .materials
                .iter()
                .find(|material| material.id == *id)
                .ok_or(CtfReportUnavailable::Material(*id))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let outside = materials.first().ok_or(CtfReportUnavailable::Association)?;
    let inside = materials.last().ok_or(CtfReportUnavailable::Association)?;
    let outside_properties = *outside
        .surface_properties()
        .ok_or(CtfReportUnavailable::Material(outside.id))?;
    let inside_properties = *inside
        .surface_properties()
        .ok_or(CtfReportUnavailable::Material(inside.id))?;
    let outside_roughness = outside
        .roughness()
        .ok_or(CtfReportUnavailable::Material(outside.id))?;
    let layers = materials
        .into_iter()
        .map(|material| material_report(model, material))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(CtfConstructionReport {
        construction_id: construction.id,
        declared_counter: i32::try_from(
            row.declared_index
                .checked_add(1)
                .ok_or(CtfReportUnavailable::DeclaredCounter)?,
        )
        .map_err(|_| CtfReportUnavailable::DeclaredCounter)?,
        name,
        layers,
        public,
        absorptance: [
            outside_properties.thermal_absorptance,
            inside_properties.thermal_absorptance,
            outside_properties.solar_absorptance,
            inside_properties.solar_absorptance,
        ],
        outside_roughness,
    })
}

/// Actual reportLayers writes; an empty registry returns without any entry writes.
/// Supplied columns must be genuine pdchOpqConsLayCol IDs from the real registry.
pub fn report_ctf_layers(
    report: &CtfConstructionReport<'_>,
    columns: &[i32],
    table: &mut PredefinedStringTable,
) -> Result<(), CtfReportUnavailable> {
    if columns.is_empty() {
        return Ok(());
    }
    for (index, material) in report.layers.iter().enumerate() {
        let column = columns
            .get(index)
            .ok_or(CtfReportUnavailable::MissingLayerColumn)?;
        table.string_entry(*column, report.name, material.name)?;
    }
    Ok(())
}

/// The four literal Init EIO headers, written to the actual supplied output sink.
pub fn write_ctf_report_headers(sink: &mut (impl Write + ?Sized)) -> io::Result<()> {
    for header in [
        "! <Construction CTF>,Construction Name,Index,#Layers,#CTFs,Time Step {hours},ThermalConductance {w/m2-K},OuterThermalAbsorptance,InnerThermalAbsorptance,OuterSolarAbsorptance,InnerSolarAbsorptance,Roughness\n",
        "! <Material CTF Summary>,Material Name,Thickness {m},Conductivity {w/m-K},Density {kg/m3},Specific Heat {J/kg-K},ThermalResistance {m2-K/w}\n",
        "! <Material:Air CTF Summary>,Material Name,ThermalResistance {m2-K/w}\n",
        "! <CTF>,Time,Outside,Cross,Inside,Flux (except final one)\n",
    ] {
        sink.write_all(header.as_bytes())?;
    }
    Ok(())
}

fn roughness_name(value: MaterialSurfaceRoughness) -> &'static str {
    match value {
        MaterialSurfaceRoughness::VeryRough => "VeryRough",
        MaterialSurfaceRoughness::Rough => "Rough",
        MaterialSurfaceRoughness::MediumRough => "MediumRough",
        MaterialSurfaceRoughness::MediumSmooth => "MediumSmooth",
        MaterialSurfaceRoughness::Smooth => "Smooth",
        MaterialSurfaceRoughness::VerySmooth => "VerySmooth",
    }
}

/// Literal selected no-source reportTransferFunction rows; no coefficient is
/// reevaluated. Actual Rust writer failures retain the already written prefix.
pub fn report_ctf_transfer_function(
    report: &CtfConstructionReport<'_>,
    sink: &mut (impl Write + ?Sized),
) -> Result<(), CtfReportError> {
    use formatting::{fixed as f, general as g};
    let p = report.public;
    let terms = usize::try_from(p.num_ctf_terms)
        .map_err(|_| CtfReportUnavailable::PublicCapacity(p.num_ctf_terms))?;
    if terms >= p.outside.len() {
        return Err(CtfReportUnavailable::PublicCapacity(p.num_ctf_terms).into());
    }
    let row = format!(
        " Construction CTF,{},{:4},{:4},{:4},{},{},{},{},{},{},{}\n",
        report.name,
        report.declared_counter,
        report.layers.len(),
        p.num_ctf_terms,
        f(p.ctf_time_step, 8, 3),
        g(p.u_value, 15, 4)?,
        f(report.absorptance[0], 8, 3),
        f(report.absorptance[1], 8, 3),
        f(report.absorptance[2], 8, 3),
        f(report.absorptance[3], 8, 3),
        roughness_name(report.outside_roughness)
    );
    sink.write_all(row.as_bytes())?;
    for material in &report.layers {
        let v = material.values;
        let row = if material.air_gap {
            format!(
                " Material:Air CTF Summary,{},{}\n",
                material.name,
                g(v[4], 12, 4)?
            )
        } else {
            format!(
                " Material CTF Summary,{},{},{},{},{},{}\n",
                material.name,
                f(v[0], 8, 4),
                f(v[1], 14, 3),
                f(v[2], 11, 3),
                f(v[3], 13, 3),
                g(v[4], 12, 4)?
            )
        };
        sink.write_all(row.as_bytes())?;
    }
    for index in (0..=terms).rev() {
        let row = if index != 0 {
            format!(
                " CTF,{index:4},{},{},{},{}\n",
                g(p.outside[index], 20, 8)?,
                g(p.cross[index], 20, 8)?,
                g(p.inside[index], 20, 8)?,
                g(p.flux[index], 20, 8)?
            )
        } else {
            format!(
                " CTF,{index:4},{},{},{}\n",
                g(p.outside[index], 20, 8)?,
                g(p.cross[index], 20, 8)?,
                g(p.inside[index], 20, 8)?
            )
        };
        sink.write_all(row.as_bytes())?;
    }
    Ok(())
}

#[cfg(test)]
#[path = "ctf_reports_tests.rs"]
mod tests;

// Public typed leaf for generic predefined reporting; all scientific bodies above unchanged.
impl std::fmt::Display for CtfReportUnavailable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "predefined report unavailable: {self:?}")
    }
}
impl std::error::Error for CtfReportUnavailable {}
