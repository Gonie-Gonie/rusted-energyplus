//! Literal GetInputTabularStyle1249-1392, same local arrays/shared getter blanks.
use super::*;
impl SourceTabularReports {
    pub(super) fn get_source_tabular_style(
        &mut self,
        o: &mut SourceTabularInputOwners<'_, '_>,
    ) -> Result<(), Error> {
        const FAMILY: &str = "OutputControl:Table:Style";
        let counts = o
            .input
            .get_object_def_max_args(FAMILY)
            .map_err(|_| Error::Unavailable("actual Table:Style definition1269"))?;
        let mut alphas = vec![String::new(); counts.alphas];
        let _numbers = vec![0.0; counts.numerics];
        if alphas.len() < 3 {
            return Err(Error::Unavailable("actual Table:Style three-alpha schema"));
        }
        let number = o.input.get_num_objects_found(FAMILY);
        if number == 0 {
            alphas[0] = "COMMA".into();
            self.input_state.number_of_styles = 1;
            self.input_state.table_styles[0] = 0;
            self.input_state.delimiters[0] = ",".into();
            alphas[1] = "None".into();
            self.input_state.units_style_tabular = 0;
            self.input_state.format_reals_tabular = true;
        } else if number == 1 {
            let (values, num_alphas) = o.raw.style_values(o.input, o.shortcut)?;
            alphas[..3].clone_from_slice(&values);
            self.input_state.format_reals_tabular = alphas[2].eq_ignore_ascii_case("Yes");
            let styles: Option<&[(i32, &str)]> = match alphas[0].to_ascii_uppercase().as_str() {
                "COMMA" => Some(&[(0, ",")]),
                "TAB" => Some(&[(1, "\t")]),
                "FIXED" => Some(&[(2, " ")]),
                "HTML" => Some(&[(3, " ")]),
                "XML" => Some(&[(4, " ")]),
                "COMMAANDHTML" => Some(&[(0, ","), (3, " ")]),
                "COMMAANDXML" => Some(&[(0, ","), (4, " ")]),
                "TABANDHTML" => Some(&[(1, "\t"), (3, " ")]),
                "XMLANDHTML" => Some(&[(4, " "), (3, " ")]),
                "ALL" => Some(&[(0, ","), (1, "\t"), (2, " "), (3, " "), (4, " ")]),
                _ => None,
            };
            if let Some(styles) = styles {
                self.input_state.number_of_styles = i32::try_from(styles.len())
                    .map_err(|_| Error::Unavailable("actual number of TAB styles"))?;
                for (i, &(style, delimiter)) in styles.iter().enumerate() {
                    self.input_state.table_styles[i] = style;
                    self.input_state.delimiters[i] = delimiter.into();
                }
            } else {
                let field = o
                    .shortcut
                    .alpha_field_names
                    .first()
                    .ok_or(Error::Unavailable(
                        "actual Table:Style ColumnSeparator field name",
                    ))?;
                o.backend.show_warning_error(&format!(
                    "{FAMILY}: Invalid {field}=\"{}\". Commas will be used.",
                    alphas[0]
                ))?;
                self.input_state.number_of_styles = 1;
                self.input_state.table_styles[0] = 0;
                self.input_state.delimiters[0] = ",".into();
                alphas[0] = "COMMA".into();
            }
            if num_alphas >= 2 {
                self.input_state.units_style_tabular = match alphas[1].to_ascii_uppercase().as_str()
                {
                    "NONE" => 0,
                    "JTOKWH" => 1,
                    "JTOMJ" => 2,
                    "JTOGJ" => 3,
                    "INCHPOUND" => 4,
                    "INCHPOUNDEXCEPTELECTRICITY" => 5,
                    _ => 6,
                };
                if self.input_state.units_style_tabular == 6 {
                    let field = o
                        .shortcut
                        .alpha_field_names
                        .get(1)
                        .ok_or(Error::Unavailable(
                            "actual Table:Style UnitConversion field name",
                        ))?;
                    o.backend.show_warning_error(&format!("{FAMILY}: Invalid {field}=\"{}\". No unit conversion will be performed. Normal SI units will be shown.",alphas[1]))?;
                }
            } else {
                self.input_state.units_style_tabular = 0;
                alphas[1] = "None".into();
            }
        } else {
            o.backend.show_warning_error(&format!(
                "{FAMILY}: Only one instance of this object is allowed. Commas will be used."
            ))?;
            self.input_state.table_styles.fill(0);
            self.input_state.delimiters.fill(",".into());
            alphas[0] = "COMMA".into();
            self.input_state.units_style_tabular = 0;
            alphas[1] = "None".into();
            self.input_state.format_reals_tabular = alphas[2].eq_ignore_ascii_case("Yes");
            // Native >1 branch does not assign numStyles: preserve that state.
        }
        o.eio
            .write_all(b"! <Tabular Report>,Style,Unit Conversion, Format Reals\n")?;
        writeln!(
            o.eio,
            "Tabular Report,{},{},{}",
            alphas[0],
            alphas[1],
            if self.input_state.format_reals_tabular {
                "Yes"
            } else {
                "No"
            }
        )?;
        Ok(())
    }
}
