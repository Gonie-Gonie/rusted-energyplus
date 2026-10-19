//! Real empty Branch/Connector getters and early service constructors.
//! Consumed at SimulationManager223-231, using the sole actual input owner.

use super::error_backend::{SourceErrorBackend, SourceErrorBackendFailure};
use super::selected_input_processor::{
    SelectedInputProcessor, SelectedInputUnavailable, SourceArgumentCounts,
};

mod plugin_output;

#[derive(Debug)]
pub(crate) enum SelectedRunInputError {
    Input(SelectedInputUnavailable),
    Backend(SourceErrorBackendFailure),
    PositiveFamily(&'static str),
}
impl std::fmt::Display for SelectedRunInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Input(error) => std::fmt::Display::fmt(error, f),
            Self::Backend(error) => std::fmt::Display::fmt(error, f),
            Self::PositiveFamily(name) => write!(f, "actual {name} positive producer unavailable"),
        }
    }
}
impl std::error::Error for SelectedRunInputError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Input(error) => Some(error),
            Self::Backend(error) => Some(error),
            Self::PositiveFamily(_) => None,
        }
    }
}
impl From<SelectedInputUnavailable> for SelectedRunInputError {
    fn from(error: SelectedInputUnavailable) -> Self {
        Self::Input(error)
    }
}
impl From<SourceErrorBackendFailure> for SelectedRunInputError {
    fn from(error: SourceErrorBackendFailure) -> Self {
        Self::Backend(error)
    }
}

// These are real getter locals, allocated from its actual definition query.
// They are destroyed at the same selected deallocation boundary; no retained
// duplicate shortcut or synthetic getter-complete flag is introduced.
type LocalArguments = (
    Vec<String>,
    Vec<f64>,
    Vec<String>,
    Vec<String>,
    Vec<bool>,
    Vec<bool>,
);
fn allocate_arguments(
    counts: SourceArgumentCounts,
) -> Result<LocalArguments, SelectedRunInputError> {
    counts.total()?;
    Ok((
        vec![String::new(); counts.alphas],
        vec![0.0; counts.numerics],
        vec![String::new(); counts.alphas],
        vec![String::new(); counts.numerics],
        vec![true; counts.alphas],
        vec![true; counts.numerics],
    ))
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SelectedBranches {
    pub(crate) get_branch_input: bool,
    pub(crate) get_branch_list_input: bool,
    pub(crate) get_splitter_input: bool,
    pub(crate) get_mixer_input: bool,
    pub(crate) get_connector_list_input: bool,
    pub(crate) invalid_branch_definitions: bool,
    pub(crate) get_branch_input_one_time: bool,
    pub(crate) branches: Option<Vec<()>>,
    pub(crate) branch_lists: Option<Vec<()>>,
    pub(crate) splitters: Option<Vec<()>>,
    pub(crate) mixers: Option<Vec<()>>,
    pub(crate) connector_lists: Option<Vec<()>>,
}
impl Default for SelectedBranches {
    fn default() -> Self {
        Self {
            get_branch_input: true,
            get_branch_list_input: true,
            get_splitter_input: true,
            get_mixer_input: true,
            get_connector_list_input: true,
            invalid_branch_definitions: false,
            get_branch_input_one_time: true,
            branches: None,
            branch_lists: None,
            splitters: None,
            mixers: None,
            connector_lists: None,
        }
    }
}
impl SelectedBranches {
    fn get_branches(
        &mut self,
        input: &SelectedInputProcessor,
    ) -> Result<(), SelectedRunInputError> {
        if self.get_branch_input_one_time && input.get_num_objects_found("Branch") > 0 {
            return Err(SelectedRunInputError::PositiveFamily("Branch"));
        }
        // Original1044-1094: the one-time flag is cleared ONLY inside >0.
        // Actual zero input retains both the unallocated Branch and true flag.
        Ok(())
    }

    fn get_branch_lists(
        &mut self,
        input: &mut SelectedInputProcessor,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), SelectedRunInputError> {
        if input.get_num_objects_found("BranchList") > 0 {
            return Err(SelectedRunInputError::PositiveFamily("BranchList"));
        }
        self.branch_lists = Some(Vec::new()); // Original1310, including count0.
        let counts = input.get_object_def_max_args("BranchList")?;
        let locals = allocate_arguments(counts)?;
        if counts.numerics > 0 {
            backend.show_severe_error("GetBranchListInput: BranchList Object definition contains numbers, cannot be decoded by GetBranchListInput routine.")?;
            backend.show_severe_error("GetBranchListInput:  Invalid Input -- preceding condition(s) will likely cause termination.")?;
        }
        // Actual count0 input/repeated-name loops have zero iterations. ErrFound
        // triggers a second severe message, not a fabricated fatal/invalid flag.
        drop(locals); // Original1404-1410 local deallocation suffix.
        Ok(())
    }

    pub(crate) fn manage_branches(
        &mut self,
        input: &mut SelectedInputProcessor,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), SelectedRunInputError> {
        if self.get_branch_input {
            self.get_branches(input)?;
            if self.get_branch_list_input {
                self.get_branch_list_input = false; // Original109 before getter.
                self.get_branch_lists(input, backend)?;
            }
            // AuditBranches(false), Original2482-2535: the retained Branch
            // owner is unallocated/size0, so there are no dangling warnings.
            self.get_branch_input = false; // Original113 after actual Audit.
        }
        Ok(())
    }

    fn get_splitters(
        &mut self,
        input: &mut SelectedInputProcessor,
    ) -> Result<(), SelectedRunInputError> {
        if !self.get_splitter_input {
            return Ok(());
        }
        if input.get_num_objects_found("Connector:Splitter") > 0 {
            return Err(SelectedRunInputError::PositiveFamily("Connector:Splitter"));
        }
        self.splitters = Some(Vec::new());
        let locals = allocate_arguments(input.get_object_def_max_args("Connector:Splitter")?)?;
        self.get_splitter_input = false; // Original1783 before deallocation.
        drop(locals);
        if !self.get_branch_input {
            // Literal original condition, not reversed.
            self.get_branches(input)?;
            self.get_branch_input = false;
        }
        Ok(()) // Zero retained splitters skip all validation/fatal bodies.
    }

    fn get_mixers(
        &mut self,
        input: &mut SelectedInputProcessor,
    ) -> Result<(), SelectedRunInputError> {
        if !self.get_mixer_input {
            return Ok(());
        }
        if input.get_num_objects_found("Connector:Mixer") > 0 {
            return Err(SelectedRunInputError::PositiveFamily("Connector:Mixer"));
        }
        self.mixers = Some(Vec::new());
        let locals = allocate_arguments(input.get_object_def_max_args("Connector:Mixer")?)?;
        self.get_mixer_input = false; // Original2045 before deallocation.
        drop(locals);
        if !self.get_branch_input {
            self.get_branches(input)?;
            self.get_branch_input = false;
        }
        Ok(())
    }

    fn get_connector_lists(
        &mut self,
        input: &mut SelectedInputProcessor,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), SelectedRunInputError> {
        if !self.get_connector_list_input {
            return Ok(());
        }
        if input.get_num_objects_found("ConnectorList") > 0 {
            return Err(SelectedRunInputError::PositiveFamily("ConnectorList"));
        }
        self.connector_lists = Some(Vec::new());
        let counts = input.get_object_def_max_args("ConnectorList")?;
        if counts.alphas != 5 || counts.numerics != 0 {
            backend.show_warning_error("GetConnectorList: Illegal \"extension\" to ConnectorList object. Internal code does not support > 2 connectors (Connector:Splitter and Connector:Mixer)")?;
        }
        let locals = allocate_arguments(counts)?;
        self.get_connector_list_input = false; // Original1531 before deallocation.
        drop(locals);
        if self.get_splitter_input {
            self.get_splitters(input)?;
            self.get_splitter_input = false;
        }
        if self.get_mixer_input {
            self.get_mixers(input)?;
            self.get_mixer_input = false;
        }
        Ok(())
    }

    pub(crate) fn manage_connectors(
        &mut self,
        input: &mut SelectedInputProcessor,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), SelectedRunInputError> {
        let mut has_splitter_or_mixer = false;
        if self.get_splitter_input {
            has_splitter_or_mixer = true;
            self.get_splitters(input)?;
        }
        if self.get_mixer_input {
            has_splitter_or_mixer = true;
            self.get_mixers(input)?;
        }
        if has_splitter_or_mixer && self.get_connector_list_input {
            self.get_connector_lists(input, backend)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SelectedCoilSelectionReport {
    pub(crate) number_of_coils_reported: i32,
    pub(crate) coil_selection_data: Vec<()>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SelectedPluginManager {
    pub(crate) maximum_global_variable_index: i32,
    pub(crate) maximum_trend_variable_index: i32,
    pub(crate) eplus_running_via_python_api: bool,
}
impl SelectedPluginManager {
    pub(crate) fn new(
        input: &SelectedInputProcessor,
        eplus_running_via_python_api: bool,
    ) -> Result<Self, SelectedRunInputError> {
        // PluginManager490-496 returns before all interpreter/search-path work.
        if input.get_num_objects_found("PythonPlugin:Instance") == 0 {
            return Ok(Self {
                maximum_global_variable_index: -1,
                maximum_trend_variable_index: -1,
                eplus_running_via_python_api,
            });
        }
        Err(SelectedRunInputError::PositiveFamily(
            "PythonPlugin:Instance",
        ))
    }
}

mod coil_summary;
