use super::{Error, SourceDemandReportOwners, SourceTabularBody};
use crate::heat_balance::source_output_registration::{
    SourceEndUseCategory, SourceOutputRegistration,
};
pub(super) const END_USES: [&str; 14] = [
    "Heating",
    "Cooling",
    "Interior Lighting",
    "Exterior Lighting",
    "Interior Equipment",
    "Exterior Equipment",
    "Fans",
    "Pumps",
    "Heat Rejection",
    "Humidification",
    "Heat Recovery",
    "Water Systems",
    "Refrigeration",
    "Generators",
];
pub(super) const RESOURCE_NAMES: [&str; 13] = [
    "Electricity",
    "Natural Gas",
    "Gasoline",
    "Diesel",
    "Coal",
    "Fuel Oil No 1",
    "Fuel Oil No 2",
    "Propane",
    "Other Fuel 1",
    "Other Fuel 2",
    "District Cooling",
    "District Heating Water",
    "District Heating Steam",
];
pub(super) fn count(value: i32) -> Result<usize, Error> {
    usize::try_from(value).map_err(|_| Error::Unavailable("actual Demand signed count extent"))
}
pub(super) fn increment(value: &mut i32) -> Result<(), Error> {
    *value = value.checked_add(1).ok_or(Error::Unavailable(
        "actual Demand Native signed local increment",
    ))?;
    Ok(())
}
pub(super) fn category(
    registration: &SourceOutputRegistration,
    ordinal: usize,
) -> Result<&SourceEndUseCategory, Error> {
    registration
        .end_use_categories
        .as_ref()
        .and_then(|r| r.get(ordinal.wrapping_sub(1)))
        .ok_or(Error::Unavailable("actual Demand EndUseCategory row"))
}
pub(super) fn subcategory(
    registration: &SourceOutputRegistration,
    end_use: usize,
    ordinal: usize,
) -> Result<Vec<u8>, Error> {
    Ok(category(registration, end_use)?
        .subcategory_names
        .as_ref()
        .and_then(|r| r.get(ordinal.wrapping_sub(1)))
        .ok_or(Error::Unavailable("actual Demand SubcategoryName row"))?
        .as_bytes()
        .to_vec())
}
pub(super) fn real(formatted: bool, value: f64, digits: i32) -> Result<Vec<u8>, Error> {
    super::super::beps_report::source_report_real(formatted, value, digits)
}
pub(super) fn predef(
    o: &mut SourceDemandReportOwners<'_, '_>,
    column: i32,
    object: &[u8],
    value: f64,
) -> Result<(), Error> {
    let object = std::str::from_utf8(object)
        .map_err(|_| Error::Unavailable("actual Demand Predefined object nonUTF8 domain"))?;
    o.table.fixed_real_entry_2(column, object, value)?;
    Ok(())
}
pub(super) struct Table {
    pub(super) rows: Vec<Vec<u8>>,
    pub(super) cols: Vec<Vec<u8>>,
    pub(super) widths: Vec<i32>,
    pub(super) body: SourceTabularBody,
}
impl Table {
    pub(super) fn new(cols: usize, rows: usize) -> Result<Self, Error> {
        let rows = vec![Vec::new(); rows];
        let columns = vec![Vec::new(); cols];
        let widths = vec![10; cols];
        let body = SourceTabularBody::allocate(cols, rows.len())?;
        Ok(Self {
            rows,
            cols: columns,
            widths,
            body,
        })
    }
}
