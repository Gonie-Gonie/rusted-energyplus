//! FillWeatherPredefinedEntries5455–6190, actual SI branch and local STAT stream.
//! IP conversion stops at its first LookupSItoIP. Input bytes are never decoded
//! into climate answers or replaced; the sole predefined charEntry owns bytes.
use super::{Error, SourceTabularReports};
use crate::RuntimeError;
use crate::heat_balance::ctf_initialization_context::{CtfInitializationReports, hb_input_error};
use crate::heat_balance::ctf_reports::PredefinedStringTable;
use crate::heat_balance::ordinary_source_session::OrdinarySourceSession;
use crate::heat_balance::predefined_tables::PredefinedRegistry;
use crate::heat_balance::source_initialization::SourceInitializationFiles;
use std::path::Path;
mod input;
mod parser;
use parser::{LineType, blank, byte_at, column, has, index, substr};

struct Entries<'a> {
    registry: &'a PredefinedRegistry,
    table: &'a mut PredefinedStringTable,
}
impl Entries<'_> {
    fn id(&self, name: &str) -> Result<i32, Error> {
        self.registry
            .named_indices
            .get(name)
            .copied()
            .ok_or(Error::Unavailable(
                "actual predefined weather column named owner",
            ))
    }
    fn weather(&mut self, object: &str, value: &[u8]) -> Result<(), Error> {
        let column = self.id("pdchWthrVal")?;
        self.table
            .raw_byte_entry(column, object, value)
            .map_err(Error::PredefinedReport)
    }
    fn leed(&mut self, object: &str, value: &[u8]) -> Result<(), Error> {
        let column = self.id("pdchLeedGenData")?;
        self.table
            .raw_byte_entry(column, object, value)
            .map_err(Error::PredefinedReport)
    }
    fn integer(&mut self, object: &str, value: i32) -> Result<(), Error> {
        let column = self.id("pdchWthrVal")?;
        self.table
            .integer_entry(column, object, value)
            .map_err(Error::PredefinedReport)
    }
}
fn lookup_ip_boundary(ip: bool, name: &str) -> Result<(), Error> {
    if ip {
        // Native assigns this local SI name immediately before LookupSItoIP.
        let _cur_name_with_si_units = name.to_owned();
        Err(Error::Unavailable(
            "actual weather STAT LookupSItoIP conversion child",
        ))
    } else {
        Ok(())
    }
}
fn with_degree(value: &[u8]) -> Vec<u8> {
    let mut text = value.to_vec();
    text.extend_from_slice("°".as_bytes());
    text
}
fn extreme(
    line: &[u8],
    units: &i32,
    name: &str,
    name_c: &str,
    occurs: &str,
    entries: &mut Entries<'_>,
) -> Result<(), Error> {
    let mut start = index(line, b"of");
    let mut end = index(line, b"C");
    start = start.wrapping_add(2); // Native size_t arithmetic, including npos.
    let degree = index(line, "°".as_bytes());
    if degree != usize::MAX {
        end = degree.wrapping_sub(1);
    } else {
        end = end.wrapping_sub(2);
    }
    if start != usize::MAX && end != usize::MAX {
        lookup_ip_boundary(*units == 4 || *units == 5, name_c)?;
        entries.weather(
            name_c,
            &with_degree(substr(
                line,
                start,
                end.wrapping_sub(start).wrapping_add(1),
            )?),
        )?;
    } else {
        entries.weather(name, b"not found")?;
    }
    start = index(line, b"on");
    start = start.wrapping_add(2);
    if start != usize::MAX {
        entries.weather(occurs, substr(line, start, usize::MAX)?)?;
    } else {
        entries.weather(occurs, b"not found")?;
    }
    Ok(())
}
impl SourceTabularReports {
    pub(crate) fn fill_source_weather_predefined_entries(
        &self,
        path: &Path,
        registry: &PredefinedRegistry,
        table: &mut PredefinedStringTable,
    ) -> Result<(), Error> {
        if !input::file_exists(path)? {
            return Ok(());
        }
        let mut interim = LineType::Initialized;
        let mut ashrae_year = Vec::new();
        let mut is_ashrae = false;
        let mut is_calculated = false;
        let mut is_koppen = false;
        let mut heating_passed = false;
        let mut cooling_passed = false;
        let mut design_passed = false;
        let mut inside_precipitation = false;
        let mut ashrae_hdd = Vec::new();
        let mut ashrae_cdd = Vec::new();
        let mut file = input::StatInput::open(path)?;
        let mut entries = Entries { registry, table };
        while file.good() {
            let mut kind = interim;
            let mut line = file.read_line()?;
            parser::strip_spaces(&mut line);
            parser::parse_stat_line(
                &line,
                &mut kind,
                &mut design_passed,
                &mut heating_passed,
                &mut cooling_passed,
                is_koppen,
                &mut inside_precipitation,
            );
            // ip() reads the actual retained unitsStyle_Tabular whenever Native
            // reaches that predicate; no CLI style or readiness value is copied.
            match kind {
                LineType::Statistics => {
                    entries.weather("Reference", substr(&line, 15, usize::MAX)?)?
                }
                LineType::Location => {
                    entries.weather("Site:Location", substr(&line, 11, usize::MAX)?)?
                }
                LineType::LatLong => {
                    for name in ["Latitude", "Longitude", "Time Zone"] {
                        let start = index(&line, b"{");
                        let end = index(&line, b"}");
                        if start != usize::MAX && end != usize::MAX {
                            entries.weather(
                                name,
                                substr(&line, start, end.wrapping_sub(start).wrapping_add(1))?,
                            )?;
                            line[start] = b'[';
                            line[end] = b']';
                        } else {
                            entries.weather(name, b"not found")?;
                        }
                    }
                }
                LineType::Elevation => {
                    let position = index(substr(&line, 12, usize::MAX)?, b"m");
                    if position != usize::MAX {
                        let mut name = b"Elevation (m) ".to_vec();
                        name.extend_from_slice(substr(&line, 12 + position + 2, usize::MAX)?);
                        let name = std::str::from_utf8(&name).map_err(|_| {
                            Error::Unavailable(
                                "actual non-UTF8 STAT Elevation local object-name construction",
                            )
                        })?;
                        lookup_ip_boundary(self.source_weather_stat_ip(), name)?;
                        entries.weather(name, substr(&line, 12, position)?)?;
                    } else {
                        entries.weather("Elevation", b"not found")?;
                    }
                }
                LineType::StdPressure => entries.weather(
                    "Standard Pressure at Elevation",
                    substr(&line, 34, usize::MAX)?,
                )?,
                LineType::DataSource => {
                    entries.weather("Data Source", substr(&line, 15, usize::MAX)?)?
                }
                LineType::WmoStation => {
                    entries.weather("WMO Station", substr(&line, 12, usize::MAX)?)?
                }
                LineType::DesignConditions => {
                    let position = index(&line, b"ASHRAE");
                    if position != usize::MAX {
                        is_ashrae = true;
                        is_calculated = true;
                        if position > 4 {
                            ashrae_year = substr(&line, position - 5, 5)?.to_vec();
                        } else {
                            ashrae_year.clear();
                        }
                        let mut text = b"Climate Design Data ".to_vec();
                        text.extend_from_slice(&ashrae_year);
                        text.extend_from_slice(b"ASHRAE Handbook");
                        entries.weather("Weather File Design Conditions", &text)?;
                    } else if has(&line, b"not calculated") || line.is_empty() {
                        is_calculated = false;
                        entries.weather(
                            "Weather File Design Conditions",
                            b"not calculated, Number of days < 1 year",
                        )?;
                    } else {
                        is_ashrae = false;
                        is_calculated = true;
                        entries.weather(
                            "Weather File Design Conditions",
                            b"Calculated from the weather file",
                        )?;
                    }
                }
                LineType::Heating => {
                    if is_calculated {
                        let (first, second) = if is_ashrae {
                            if ashrae_year == b"2001" {
                                (2, 3)
                            } else {
                                (4, 5)
                            }
                        } else if blank(column(&line, 5)) {
                            (3, 4)
                        } else {
                            (4, 5)
                        };
                        lookup_ip_boundary(
                            self.source_weather_stat_ip(),
                            "Heating Design Temperature 99.6% (C)",
                        )?;
                        entries.weather(
                            "Heating Design Temperature 99.6% (C)",
                            &with_degree(column(&line, first)),
                        )?;
                        entries.weather(
                            "Heating Design Temperature 99% (C)",
                            &with_degree(column(&line, second)),
                        )?;
                    }
                }
                LineType::Cooling => {
                    if is_calculated {
                        let (first, second, third) = if is_ashrae {
                            if ashrae_year == b"2001" {
                                (2, 4, 6)
                            } else {
                                (5, 7, 9)
                            }
                        } else if blank(column(&line, 6)) {
                            (3, 4, 5)
                        } else {
                            (4, 5, 6)
                        };
                        lookup_ip_boundary(
                            self.source_weather_stat_ip(),
                            "Cooling Design Temperature 0.4% (C)",
                        )?;
                        entries.weather(
                            "Cooling Design Temperature 0.4% (C)",
                            &with_degree(column(&line, first)),
                        )?;
                        entries.weather(
                            "Cooling Design Temperature 1% (C)",
                            &with_degree(column(&line, second)),
                        )?;
                        entries.weather(
                            "Cooling Design Temperature 2% (C)",
                            &with_degree(column(&line, third)),
                        )?;
                    }
                }
                LineType::StdHdd => ashrae_hdd = substr(&line, 2, 4)?.to_vec(),
                LineType::StdCdd => ashrae_cdd = substr(&line, 2, 4)?.to_vec(),
                LineType::MaxDryBulb => extreme(
                    &line,
                    &self.input_state.units_style_tabular,
                    "Maximum Dry Bulb Temperature",
                    "Maximum Dry Bulb Temperature (C)",
                    "Maximum Dry Bulb Occurs on",
                    &mut entries,
                )?,
                LineType::MinDryBulb => extreme(
                    &line,
                    &self.input_state.units_style_tabular,
                    "Minimum Dry Bulb Temperature",
                    "Minimum Dry Bulb Temperature (C)",
                    "Minimum Dry Bulb Occurs on",
                    &mut entries,
                )?,
                LineType::MaxDewPoint => extreme(
                    &line,
                    &self.input_state.units_style_tabular,
                    "Maximum Dew Point Temperature",
                    "Maximum Dew Point Temperature (C)",
                    "Maximum Dew Point Occurs on",
                    &mut entries,
                )?,
                LineType::MinDewPoint => extreme(
                    &line,
                    &self.input_state.units_style_tabular,
                    "Minimum Dew Point Temperature",
                    "Minimum Dew Point Temperature (C)",
                    "Minimum Dew Point Occurs on",
                    &mut entries,
                )?,
                LineType::MonthlyPrec => {
                    let columns = parser::precipitation_columns(&line)?;
                    let mut monthly = [None; 12]; // Native local cells remain unwritten until each stoi.
                    let mut annual = 0_i32;
                    for (i, cell) in monthly.iter_mut().enumerate() {
                        *cell = Some(parser::stoi(columns.get(i + 2).ok_or(
                            Error::Unavailable("actual STAT monthly result subscript"),
                        )?)?);
                        annual =
                            annual
                                .checked_add(cell.ok_or(Error::Unavailable(
                                    "actual STAT monthly local unwritten",
                                ))?)
                                .ok_or(Error::Unavailable(
                                    "actual STAT annual precipitation signed-overflow domain",
                                ))?;
                    }
                    entries.integer("Annual Total Precipitation [mm]", annual)?;
                }
                LineType::MaxHourlyPrec => {
                    let columns = parser::precipitation_columns(&line)?;
                    let mut monthly = [None; 12];
                    let mut maximum = 0;
                    let mut maximum_index = 0;
                    for (i, cell) in monthly.iter_mut().enumerate() {
                        *cell = Some(parser::stoi(columns.get(i + 2).ok_or(
                            Error::Unavailable("actual STAT max-hour result subscript"),
                        )?)?);
                        let value =
                            cell.ok_or(Error::Unavailable("actual STAT hourly local unwritten"))?;
                        if value > maximum {
                            maximum = value;
                            maximum_index = i;
                        }
                    }
                    const MONTHS: [&str; 12] = [
                        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct",
                        "Nov", "Dec",
                    ];
                    entries.integer("Max Hourly Precipitation [mm]", maximum)?;
                    entries.weather(
                        "Max Hourly Precipitation Occurs in",
                        MONTHS[maximum_index].as_bytes(),
                    )?;
                }
                LineType::WithHdd => {
                    if !ashrae_hdd.is_empty() {
                        lookup_ip_boundary(
                            self.source_weather_stat_ip(),
                            "ASHRAE Handbook 2009 Heating Degree-Days - base 65°(C)",
                        )?;
                        entries.weather(
                            "ASHRAE Handbook 2009 Heating Degree-Days (base 18.3°C)",
                            &ashrae_hdd,
                        )?;
                    } else if self.source_weather_stat_ip() {
                        entries.weather(
                            "ASHRAE Handbook 2009 Heating Degree-Days (base 65°F)",
                            b"not found",
                        )?;
                    } else {
                        entries.weather(
                            "ASHRAE Handbook 2009 Heating Degree-Days (base 18.3°C)",
                            b"not found",
                        )?;
                    }
                    lookup_ip_boundary(
                        self.source_weather_stat_ip(),
                        "Weather File Heating Degree-Days - base 65°(C)",
                    )?;
                    entries.weather(
                        "Weather File Heating Degree-Days (base 18°C)",
                        substr(&line, 2, 4)?,
                    )?;
                    entries.leed("Heating Degree Days", substr(&line, 2, 4)?)?;
                    entries.leed("HDD and CDD data source", b"Weather File Stat")?;
                }
                LineType::WithCdd => {
                    if !ashrae_cdd.is_empty() {
                        lookup_ip_boundary(
                            self.source_weather_stat_ip(),
                            "ASHRAE Handbook 2009  Cooling Degree-Days - base 50°(C)",
                        )?;
                        entries.weather(
                            "ASHRAE Handbook 2009  Cooling Degree-Days (base 10°C)",
                            &ashrae_cdd,
                        )?;
                    } else if self.source_weather_stat_ip() {
                        entries.weather(
                            "ASHRAE Handbook 2009  Cooling Degree-Days (base 50°F)",
                            b"not found",
                        )?;
                    } else {
                        entries.weather(
                            "ASHRAE Handbook 2009  Cooling Degree-Days (base 10°C)",
                            b"not found",
                        )?;
                    }
                    lookup_ip_boundary(
                        self.source_weather_stat_ip(),
                        "Weather File Cooling Degree-Days - base 50°(C)",
                    )?;
                    entries.weather(
                        "Weather File Cooling Degree-Days (base 10°C)",
                        substr(&line, 2, 4)?,
                    )?;
                    entries.leed("Cooling Degree Days", substr(&line, 2, 4)?)?;
                }
                LineType::Koppen => {
                    if !has(&line, b"not shown") {
                        is_koppen = true;
                        let count = if byte_at(&line, 18)? == b'"' { 2 } else { 3 };
                        entries.weather("Köppen Classification", substr(&line, 16, count)?)?;
                    } else {
                        is_koppen = false;
                        entries.weather("Köppen Recommendation", substr(&line, 2, usize::MAX)?)?;
                    }
                }
                LineType::KoppenDes1 => {
                    if is_koppen {
                        entries.weather("Köppen Description", substr(&line, 2, usize::MAX)?)?;
                    }
                }
                LineType::KoppenDes2 => {
                    if is_koppen {
                        if line.len() > 3 {
                            if substr(&line, 2, 2)? != b"**" {
                                entries.weather(
                                    "Köppen Recommendation",
                                    substr(&line, 2, usize::MAX)?,
                                )?;
                            } else {
                                entries.weather("Köppen Recommendation", b"")?;
                            }
                        } else {
                            entries.weather("Köppen Recommendation", b"")?;
                        }
                    }
                }
                LineType::AshStd
                | LineType::AshStdDes1
                | LineType::AshStdDes2
                | LineType::AshStdDes3 => {
                    if has(&line, b"Standard") {
                        let mut zone = substr(&line, 16, 2)?.to_vec();
                        if byte_at(&zone, 1)? == b'"' {
                            zone[1] = b' ';
                        }
                        entries.weather("ASHRAE Climate Zone", &zone)?;
                        entries.leed("Climate Zone", &zone)?;
                        let description: Option<&[u8]> = match zone.as_slice() {
                            b"1A" => Some(b"Very Hot-Humid"),
                            b"1B" => Some(b"Very Hot-Dry"),
                            b"2A" => Some(b"Hot-Humid"),
                            b"2B" => Some(b"Hot-Dry"),
                            b"3A" => Some(b"Warm-Humid"),
                            b"3B" => Some(b"Warm-Dry"),
                            b"3C" => Some(b"Warm-Marine"),
                            b"4A" => Some(b"Mixed-Humid"),
                            b"4B" => Some(b"Mixed-Dry"),
                            b"4C" => Some(b"Mixed-Marine"),
                            b"5A" => Some(b"Cool-Humid"),
                            b"5B" => Some(b"Cool-Dry"),
                            b"5C" => Some(b"Cool-Marine"),
                            b"6A" => Some(b"Cold-Humid"),
                            b"6B" => Some(b"Cold-Dry"),
                            b"7 " => Some(b"Very Cold"),
                            b"8 " => Some(b"Subarctic"),
                            _ => None,
                        };
                        if let Some(description) = description {
                            entries.weather("ASHRAE Description", description)?;
                        }
                    }
                }
                LineType::Initialized => {}
            }
            line.clear();
            interim = LineType::Initialized;
            // Native6168–6188 retains these seven independent ordered tests.
            if kind == LineType::AshStdDes3 {
                interim = LineType::Initialized;
            }
            if kind == LineType::AshStdDes2 {
                interim = LineType::AshStdDes2;
            }
            if kind == LineType::AshStdDes1 {
                interim = LineType::AshStdDes1;
            }
            if kind == LineType::AshStd {
                interim = LineType::AshStd;
            }
            if kind == LineType::KoppenDes2 {
                interim = LineType::Initialized;
            }
            if kind == LineType::KoppenDes1 {
                interim = LineType::KoppenDes1;
            }
            if kind == LineType::Koppen {
                interim = LineType::Koppen;
            }
        }
        Ok(())
    }
    fn source_weather_stat_ip(&self) -> bool {
        self.input_state.units_style_tabular == 4 || self.input_state.units_style_tabular == 5
    }
}
impl SourceInitializationFiles {
    /// Actual WriteTabularReports5212 child using the sole retained STAT path.
    /// # Errors
    /// Returns the real IO/table cause or the first unsupported Native operation.
    pub fn fill_source_weather_predefined_entries_on_session(
        &self,
        session: &Option<OrdinarySourceSession>,
        reports: &mut CtfInitializationReports,
    ) -> Result<(), RuntimeError> {
        let session = session.as_ref().ok_or_else(|| {
            RuntimeError::HeatBalanceInput(Box::new(Error::Unavailable(
                "same actual TAB owner before FillWeatherPredefinedEntries5212",
            )))
        })?;
        session
            .tabular
            .fill_source_weather_predefined_entries(
                self.source_stat_input_path(),
                &reports.registry,
                &mut reports.table,
            )
            .map_err(hb_input_error)
    }
}
