//! Literal parseStatLine5366–5453 and GetColumnUsingTabs6192–6240.
use super::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum LineType {
    Initialized,
    Statistics,
    Location,
    LatLong,
    Elevation,
    StdPressure,
    DataSource,
    WmoStation,
    DesignConditions,
    Heating,
    Cooling,
    StdHdd,
    StdCdd,
    MaxDryBulb,
    MinDryBulb,
    MaxDewPoint,
    MinDewPoint,
    WithHdd,
    WithCdd,
    MaxHourlyPrec,
    MonthlyPrec,
    Koppen,
    KoppenDes1,
    KoppenDes2,
    AshStd,
    AshStdDes1,
    AshStdDes2,
    AshStdDes3,
}
pub(super) fn has(line: &[u8], text: &[u8]) -> bool {
    index(line, text) != usize::MAX
}
pub(super) fn index(line: &[u8], text: &[u8]) -> usize {
    if text.is_empty() {
        return 0;
    }
    line.windows(text.len())
        .position(|value| value == text)
        .unwrap_or(usize::MAX)
}
pub(super) fn substr(line: &[u8], start: usize, count: usize) -> Result<&[u8], Error> {
    if start > line.len() {
        return Err(Error::Unavailable(
            "actual STAT std::string::substr out_of_range boundary",
        ));
    }
    Ok(&line[start..start + count.min(line.len() - start)])
}
pub(super) fn byte_at(line: &[u8], index: usize) -> Result<u8, Error> {
    if index == line.len() {
        // C++ std::string::operator[](size()) is its terminating zero cell.
        Ok(0)
    } else {
        line.get(index).copied().ok_or(Error::Unavailable(
            "actual STAT std::string subscript outside defined domain",
        ))
    }
}
pub(super) fn strip_spaces(line: &mut Vec<u8>) {
    // Objexx string.functions.cc82–95 strips ASCII spaces, not tabs/whitespace.
    if let Some(first) = line.iter().position(|value| *value != b' ') {
        let mut last = line.len() - 1;
        while line[last] == b' ' {
            last -= 1;
        }
        line.truncate(last + 1);
        line.drain(..first);
    } else {
        line.clear();
    }
}
pub(super) fn blank(line: &[u8]) -> bool {
    line.iter().all(|value| *value == b' ')
}
pub(super) fn column(line: &[u8], number: i32) -> &[u8] {
    let mut start = 0;
    let mut end = index(line, b"\t");
    if number == 1 {
        return if end == usize::MAX {
            line
        } else {
            &line[..end]
        };
    }
    if end == usize::MAX {
        return b"";
    }
    let mut columns = 1;
    while columns < number {
        start = end + 1;
        end = index(&line[start..], b"\t");
        if end != usize::MAX {
            end += start;
        }
        columns += 1;
        if end == usize::MAX {
            break;
        }
    }
    if number > columns {
        return b"";
    }
    if end == usize::MAX {
        end = line.len();
    }
    &line[start..end]
}
pub(super) fn parse_stat_line(
    line: &[u8],
    kind: &mut LineType,
    design_passed: &mut bool,
    heating_passed: &mut bool,
    cooling_passed: &mut bool,
    is_koppen: bool,
    inside_precipitation: &mut bool,
) {
    use LineType::*;
    if line.starts_with(b"Statistics") {
        *kind = Statistics;
    } else if line.starts_with(b"Location") {
        *kind = Location;
    } else if line.starts_with(b"{") {
        *kind = LatLong;
    } else if line.starts_with(b"Elevation") {
        *kind = Elevation;
    } else if line.starts_with(b"Standard Pressure") {
        *kind = StdPressure;
    } else if line.starts_with(b"Data Source") {
        *kind = DataSource;
    } else if line.starts_with(b"WMO Station") {
        *kind = WmoStation;
    } else if has(line, b"Design Conditions") {
        if !*design_passed {
            *design_passed = true;
            *kind = DesignConditions;
        }
    } else if line.starts_with(b"\tHeating") {
        if !*heating_passed {
            *heating_passed = true;
            *kind = Heating;
        }
    } else if line.starts_with(b"\tCooling") {
        if !*cooling_passed {
            *cooling_passed = true;
            *kind = Cooling;
        }
    } else if has(line, b"(standard) heating degree-days (18.3") {
        *kind = StdHdd;
    } else if has(line, b"(standard) cooling degree-days (10") {
        *kind = StdCdd;
    } else if has(line, b"Maximum Dry Bulb") {
        *kind = MaxDryBulb;
    } else if has(line, b"Minimum Dry Bulb") {
        *kind = MinDryBulb;
    } else if has(line, b"Maximum Dew Point") {
        *kind = MaxDewPoint;
    } else if has(line, b"Minimum Dew Point") {
        *kind = MinDewPoint;
    } else if has(line, b"(wthr file) heating degree-days (18")
        || has(line, b"heating degree-days (18")
    {
        *kind = WithHdd;
    } else if has(line, b"(wthr file) cooling degree-days (10")
        || has(line, b"cooling degree-days (10")
    {
        *kind = WithCdd;
    } else if has(line, b"Statistics for Liquid Precipitation") {
        *inside_precipitation = true;
    } else if *inside_precipitation && has(line, b"Total") {
        *kind = MonthlyPrec;
    } else if *inside_precipitation && has(line, b"Max Hourly") {
        *kind = MaxHourlyPrec;
        *inside_precipitation = false;
    }
    // These are sequential independent statements, not a mutually exclusive match.
    if *kind == KoppenDes1 && is_koppen {
        *kind = KoppenDes2;
    }
    if *kind == Koppen && is_koppen {
        *kind = KoppenDes1;
    }
    if has(line, b"ppen classification)") {
        *kind = Koppen;
    }
    if *kind == AshStdDes2 {
        *kind = AshStdDes3;
    }
    if *kind == AshStdDes1 {
        *kind = AshStdDes2;
    }
    if *kind == AshStd {
        *kind = AshStdDes1;
    }
    if has(line, b"ASHRAE Standard") {
        *kind = AshStd;
    }
}

pub(super) fn stoi(text: &[u8]) -> Result<i32, Error> {
    // The selected C-locale stoi integer prefix, including sign/trailing text.
    // Non-ASCII classification and exceptions are explicit first boundaries.
    let mut position = 0;
    while let Some(&byte) = text.get(position) {
        if byte >= 128 {
            return Err(Error::Unavailable(
                "actual STAT stoi non-ASCII locale classification",
            ));
        }
        if !matches!(byte, b' ' | b'\t' | b'\n' | b'\r' | 11 | 12) {
            break;
        }
        position += 1;
    }
    let negative = text.get(position) == Some(&b'-');
    if negative || text.get(position) == Some(&b'+') {
        position += 1;
    }
    let first = position;
    let mut value = 0_i64;
    while let Some(&byte) = text.get(position) {
        if !byte.is_ascii_digit() {
            break;
        }
        value = value
            .checked_mul(10)
            .and_then(|v| v.checked_add(i64::from(byte - b'0')))
            .ok_or(Error::Unavailable(
                "actual STAT stoi out_of_range exception",
            ))?;
        position += 1;
    }
    if position == first {
        return Err(Error::Unavailable(
            "actual STAT stoi invalid_argument exception",
        ));
    }
    i32::try_from(if negative { -value } else { value })
        .map_err(|_| Error::Unavailable("actual STAT stoi out_of_range exception"))
}
pub(super) fn precipitation_columns(line: &[u8]) -> Result<Vec<Vec<u8>>, Error> {
    // stringstream.good()/getline with '\t' also visits the last empty field.
    line.split(|byte| *byte == b'\t')
        .map(|field| {
            if field.iter().any(|byte| *byte >= 128) {
                return Err(Error::Unavailable(
                    "actual STAT isspace signed-char locale domain",
                ));
            }
            Ok(field
                .iter()
                .copied()
                .filter(|byte| !matches!(*byte, b' ' | b'\t' | b'\n' | b'\r' | 11 | 12))
                .collect())
        })
        .collect()
}
