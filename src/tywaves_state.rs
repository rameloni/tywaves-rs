use std::path::Path;

use crate::{symbol_table::*, variable_finder::VariableFinder, vcd_rewrite::VcdRewriter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// The state of the Tywaves translator
pub struct TywaveState {
    /// List of the scopes
    pub scopes: Vec<Scope>,
}

impl VariableFinder for TywaveState {
    fn find_variable(&self, subpath: &[String]) -> Option<&Variable> {
        // Find the variable in the top scope
        self.scopes
            .iter()
            .find_map(|scope| scope.find_variable(subpath))
    }

    fn find_parent_variable(&self, subpath: &[String]) -> Option<&Variable> {
        // Find the variable in the top scope
        self.scopes
            .iter()
            .find_map(|scope| scope.find_parent_variable(subpath))
    }
}

impl TywaveState {
    /// Rewrite a vcd file in order to make it compatible with Surfer for TywaveState
    pub fn vcd_rewrite_surfer<'a>(&self, vcd: &'a Path) -> &'a Path {
        let mut vcd_rewriter = VcdRewriter::new(vcd, self.scopes.clone());

        // Rewrite the vcd file
        let _ = vcd_rewriter.rewrite();
        let final_file = vcd_rewriter.get_final_file();
        Path::new(final_file)
    }
}
