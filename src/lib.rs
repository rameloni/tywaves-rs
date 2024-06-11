/// HGLDD file format rust api.
/// This module contains the data structures to read and write HGLDD file
pub mod hgldd;

pub mod vcd_rewrite;

/// TyVcd IR format rust api.
/// This module contains methods and data structures to create and manipulate TyVcd IR format
/// and links variables/scopes to their trace paths (i.e. associating generic representation
/// to a value in a trace file).
pub mod tyvcd;
