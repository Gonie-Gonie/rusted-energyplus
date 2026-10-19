//! Actual TestBranchIntegrity2535-2880 zero retained Branch/BranchList traversal.
use super::Error;
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::run_input_prefix::SelectedBranches;
use std::io::Write;

#[derive(Default)]
struct BranchUniqueNodes {
    #[allow(
        dead_code,
        reason = "BranchInputManager.cc TestBranchIntegrity retains its original BranchUniqueNodes scratch-row fields; the finite zero-branch domain does not read them."
    )]
    number_nodes: i32,
    #[allow(
        dead_code,
        reason = "BranchInputManager.cc TestBranchIntegrity retains its original BranchUniqueNodes scratch-row fields; the finite zero-branch domain does not read them."
    )]
    unique_node_names: Option<Vec<String>>,
}
impl SelectedBranches {
    /// Does not rerun ManageBranchInput or change any native input once-flag.
    pub(crate) fn test_source_branch_integrity(
        &self,
        err_found: &mut bool,
        bnd: &mut dyn Write,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        let branch_reported = vec![false; self.branches.as_ref().map_or(0, Vec::len)];
        backend.show_message("Testing Individual Branch Integrity")?;
        *err_found = false;
        let branch_nodes: Vec<BranchUniqueNodes> = (0..self.branches.as_ref().map_or(0, Vec::len))
            .map(|_| BranchUniqueNodes::default())
            .collect();
        writeln!(
            bnd,
            "! ==============================================================="
        )?;
        writeln!(bnd, "! <#Branch Lists>,<Number of Branch Lists>")?;
        writeln!(
            bnd,
            " #Branch Lists,{}",
            self.branch_lists.as_ref().map_or(0, Vec::len)
        )?;
        writeln!(
            bnd,
            "! <Branch List>,<Branch List Count>,<Branch List Name>,<Loop Name>,<Loop Type>,<Number of Branches>"
        )?;
        writeln!(
            bnd,
            "! <Branch>,<Branch Count>,<Branch Name>,<Loop Name>,<Loop Type>,<Branch Inlet Node Name>,<Branch Outlet Node Name>"
        )?;
        if !self.branch_lists.as_ref().is_none_or(Vec::is_empty) {
            return Err(Error::Unavailable(
                "actual TestBranchIntegrity2603-2609 first positive BranchList print arguments",
            ));
        }
        if !self.branches.as_ref().is_none_or(Vec::is_empty) {
            return Err(Error::Unavailable(
                "actual TestBranchIntegrity2773 Branch.NumOfComponents and UniqueNodeNames allocation",
            ));
        }
        // All three native branch-node traversal/deallocation loops are empty.
        drop(branch_nodes);
        let mut orphan_count = 0;
        for &reported in &branch_reported {
            if reported {
                continue;
            }
            orphan_count += 1;
        }
        if orphan_count > 0 {
            writeln!(
                bnd,
                "! <# Orphaned Branches>,<Number of Branches not on Branch Lists>"
            )?;
            writeln!(bnd, " #Orphaned Branches,{orphan_count}")?;
            backend.show_warning_error(
                "There are orphaned Branches in input. See .bnd file for details.",
            )?;
            return Err(Error::Unavailable(
                "actual TestBranchIntegrity2833 orphan Branch.Name warning",
            ));
        }
        if *err_found {
            backend.show_severe_error("Branch(es) did not pass integrity testing")?;
        } else {
            backend.show_message("All Branches passed integrity testing")?;
        }
        Ok(())
    }
}
