use std::collections::HashMap;

use super::trace_pointer::TracePointer;
use crate::hgldd::spec as hgldd;

#[derive(Debug, Clone)]
/// Represents the TyVcd format
pub struct TyVcd {
    pub scopes: HashMap<String, Scope>,
}

impl From<Vec<hgldd::Hgldd>> for TyVcd {
    fn from(hgldds: Vec<hgldd::Hgldd>) -> Self {
        todo!()
    }
}

#[derive(Debug, Clone)]
/// Represent a scope (i.e. a module instance) in the TyVcd format
pub struct Scope {
    /// The name of the scope in the trace.
    _id_trace_name: String,

    /// The subscopes of this scope.
    pub subscopes: Vec<Scope>,
    /// The variables declared in this scope.
    pub variables: Vec<Variable>,

    /// The original name of the scope in the HGLDD file.
    pub name: String,
    /// High level information of the scope.
    pub high_level_info: TypeInfo,
}

impl Scope {
    pub fn empty(trace_name: String, name: String, high_level_info: TypeInfo) -> Self {
        Self {
            _id_trace_name: trace_name,
            subscopes: Vec::new(),
            variables: Vec::new(),
            name,
            high_level_info,
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

#[derive(Debug, Clone)]
/// Represent a variable in the TyVcd format.
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

    pub fn update_trace(&mut self, trace_name: String) {
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

#[derive(Debug, Clone)]
/// Structure to store the type information of a variable.
pub struct TypeInfo {
    /// The type name of the variable.
    pub type_name: String,
    /// The parameters of the type (if any)
    pub params: Vec<String>,
}
impl TypeInfo {
    pub fn new(type_name: String, params: Vec<String>) -> Self {
        Self { type_name, params }
    }
}
#[derive(Debug, Clone)]
pub enum VariableKind {
    Struct { fields: Vec<Variable> },
    Vector { fields: Vec<Variable> },
    Ground,
    External,
}
