//! Genuine Integer overload prefix for reached unrequested registrations.
use super::{
    OrdinaryRadiationError, OrdinaryRadiationUnavailable, SourceOutputFileHost,
    SourceOutputPreScan, SourceOutputRegistration, SourceOutputStoreType, SourceOutputUnits,
    SourceRealOutputDeclaration, SourceReportVariableInput,
};

#[derive(Clone, Copy)]
pub(crate) struct SourceIntegerOutputDeclaration<'a> {
    pub(crate) name: &'a str,
    pub(crate) key: &'a str,
    pub(crate) units: SourceOutputUnits,
    pub(crate) store_type: SourceOutputStoreType,
}

impl SourceOutputRegistration {
    pub(crate) fn setup_unmetered_zone_integer(
        &mut self,
        input: &mut SourceReportVariableInput<'_>,
        prescan: &SourceOutputPreScan,
        declaration: SourceIntegerOutputDeclaration<'_>,
        minutes_in_time_step: i32,
        host: &mut impl SourceOutputFileHost,
        eio: &mut dyn std::io::Write,
    ) -> Result<(), OrdinaryRadiationError> {
        if !self.output_initialized {
            self.initialize_output(minutes_in_time_step, host)?;
        }
        self.get_report_variable_input(input, host, eio)?;
        let request_indexes = self.match_report_variables(declaration.name, declaration.key);
        // The shared DD arguments are metadata only; this genuine overload
        // passes Integer explicitly and never creates a Real dictionary row.
        self.add_typed_dictionary_variable(
            SourceRealOutputDeclaration {
                name: declaration.name,
                key: declaration.key,
                units: declaration.units,
                store_type: declaration.store_type,
            },
            true,
        );
        self.num_integer_variables_setup = self
            .num_integer_variables_setup
            .checked_add(1)
            .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
        self.num_total_integer_variables = self
            .num_total_integer_variables
            .checked_add(
                i32::try_from(request_indexes.len().max(1))
                    .map_err(|_| OrdinaryRadiationUnavailable::StorageDomain)?,
            )
            .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
        // Integer overload reaches FindItem after its dictionary/counters.
        if !prescan.find_item(declaration.key, declaration.name) {
            return Ok(());
        }
        // Stop before the unported positive Int keyed-variable branch.
        Err(OrdinaryRadiationUnavailable::InputBranch.into())
    }
}
