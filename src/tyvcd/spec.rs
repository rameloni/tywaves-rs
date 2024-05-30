use std::collections::HashMap;

use super::trace_pointer::TracePointer;
use crate::hgldd::spec as hgldd;

/// Represents the TyVcd format.
#[derive(Debug, Clone, PartialEq)]
pub struct TyVcd {
    pub scopes: HashMap<String, Scope>,
}

/// Represent a scope (i.e. a module instance) in the TyVcd format.
#[derive(Debug, Clone, PartialEq)]
pub struct Scope {
    /// The name of the scope in the trace
    _id_trace_name: String,

    /// The subscopes of this scope
    pub subscopes: Vec<Scope>,
    /// The variables declared in this scope
    pub variables: Vec<Variable>,

    /// The original name of the scope in the HGLDD file
    pub name: String,
    /// High level information of the scope
    pub high_level_info: TypeInfo,
}

impl Scope {
    /// Create a new empty scope without any subscopes or variables.
    pub fn empty(trace_name: String, name: String, high_level_info: TypeInfo) -> Self {
        Self {
            _id_trace_name: trace_name,
            subscopes: Vec::new(),
            variables: Vec::new(),
            name,
            high_level_info,
        }
    }

    /// Create a new scope from another scope with an updated trace name.
    /// *Pleas use the `clone` method directly for a full copy.*
    pub fn from_other(other: &Scope, trace_name: String) -> Self {
        Self {
            _id_trace_name: trace_name,
            subscopes: other.subscopes.clone(),
            variables: other.variables.clone(),
            name: other.name.clone(),
            high_level_info: other.high_level_info.clone(),
        }
    }
}

impl TracePointer for Scope {
    fn get_trace_name(&self) -> String {
        self._id_trace_name.clone()
    }
    fn get_trace_path(&self) -> String {
        todo!()
    }
}

/// Represent a variable in the TyVcd format.
#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    /// The name of the variable in the trace.
    _id_trace_name: String,

    /// The name of the variable.
    pub name: String,
    /// The high level type information of the variable.
    pub high_level_info: TypeInfo,
    /// The kind of the variable.
    pub kind: VariableKind,
}
impl Variable {
    pub fn new(
        trace_name: String,
        name: String,
        high_level_info: TypeInfo,
        kind: VariableKind,
    ) -> Self {
        Self {
            _id_trace_name: trace_name,
            name,
            high_level_info,
            kind,
        }
    }

    pub(in crate::tyvcd) fn update_trace_name(&mut self, trace_name: String) {
        self._id_trace_name = trace_name;
    }
}

impl TracePointer for Variable {
    fn get_trace_name(&self) -> String {
        self._id_trace_name.clone()
    }
    fn get_trace_path(&self) -> String {
        todo!()
    }
}

/// Structure to store the type information of a variable.
#[derive(Debug, Clone, PartialEq)]
pub struct TypeInfo {
    /// The type name of the variable
    pub type_name: String,
    /// The parameters of the type (if any)
    pub params: Vec<String>,
}

impl TypeInfo {
    pub fn new(type_name: String, params: Vec<String>) -> Self {
        Self { type_name, params }
    }
}
/// Represents the kind of a variable in the TyVcd format.
#[derive(Debug, Clone, PartialEq)]
pub enum VariableKind {
    /// A ground type
    Ground,
    /// A struct-like type
    Struct { fields: Vec<Variable> },
    /// A vector-like type
    Vector { fields: Vec<Variable> },
    /// An external type (from another source). Typically when it is not possible to infer the type
    External,
}
