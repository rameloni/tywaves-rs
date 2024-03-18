use crate::symbol_table::Variable;
pub trait VariableFinder {
    fn find_variable(&self, subpath: &Vec<String>) -> Option<&Variable>;
    fn find_parent_variable(&self, subpath: &Vec<String>) -> Option<&Variable>;
}
