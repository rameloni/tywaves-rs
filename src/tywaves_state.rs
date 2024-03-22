use crate::{symbol_table::*, variable_finder::VariableFinder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// The state of the Tywaves translator
pub struct TywaveState {
    /// List of the scopes
    pub scopes: Vec<Scope>,
}

impl VariableFinder for TywaveState {
    fn find_variable(&self, subpath: &Vec<String>) -> Option<&Variable> {
        // Find the variable in the top scope
        self.scopes
            .iter()
            .find_map(|scope| scope.find_variable(subpath))
    }

    fn find_parent_variable(&self, subpath: &Vec<String>) -> Option<&Variable> {
        // Find the variable in the top scope
        self.scopes
            .iter()
            .find_map(|scope| scope.find_parent_variable(subpath))
    }
}
