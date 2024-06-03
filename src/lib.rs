/// HGLDD file format rust api.
/// This module contains the data structures to read and write HGLDD file
pub mod hgldd;

#[deprecated = "Use the `tyvcd` module instead. `symbol_table` will be removed."]
pub mod symbol_table;

pub mod tywaves_state;

#[deprecated = "Use the `tyvcd::trace_pointer` module instead. `variable_finder` will be removed."]
pub mod variable_finder;
pub mod vcd_rewrite;

/// TyVcd IR format rust api.
/// This module contains methods and data structures to create and manipulate TyVcd IR format
/// and links variables/scopes to their trace paths (i.e. associating generic representation
/// to a value in a trace file).
pub mod tyvcd;
