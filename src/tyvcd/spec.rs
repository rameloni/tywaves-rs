use std::collections::HashMap;

use super::trace_pointer::{TraceFinder, TraceGetter};

type ScopeId = String;
/// Represents the TyVcd format.
#[derive(Debug, Clone, PartialEq)]
pub struct TyVcd {
    pub scopes: HashMap<ScopeId, Scope>,
}

impl TraceFinder for TyVcd {
    /// Return the element pointing to the trace path.
    fn find_trace(&self, full_path: &[String]) -> Option<&dyn TraceGetter> {
        // Get the root scope
        let root = self.scopes.get(full_path.first()?)?;

        if full_path.len() == 1 {
            // Exactly one scope -> the root
            Some(root)
        } else {
            // More than one scope specified
            let mut path = &full_path[1..];
            let mut curr_scope = root;

            // Get the last name in the path: it can point to a variable, a subscope or nothing
            while path.len() > 1 {
                curr_scope = curr_scope.find_subscope(&path[0])?;
                path = &path[1..];
            }

            // Return
            if let Some(variable) = curr_scope.find_variable(&path[0]) {
                Some(variable)
            } else {
                let subscope = curr_scope.find_subscope(&path[0])?;
                Some(subscope)
            }
        }
    }
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

    /// Find a subscope in the scope.
    fn find_subscope(&self, name: &str) -> Option<&Scope> {
        self.subscopes.iter().find(|s| s.get_trace_name() == name)
    }

    /// Find a variable in the scope.
    fn find_variable(&self, name: &str) -> Option<&Variable> {
        self.variables.iter().find_map(|v| v.find_var(name))
    }
}

impl TraceGetter for Scope {
    fn get_trace_name(&self) -> &String {
        &self._id_trace_name
    }

    fn get_trace_path(&self) -> Vec<&String> {
        // TODO: implement get_trace_path for Scope
        todo!()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
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

    /// Update the trace name of the variable.
    pub(in crate::tyvcd) fn update_trace_name(&mut self, trace_name: String) {
        self._id_trace_name = trace_name;
    }

    // Find a variable in the variable tree.
    fn find_var(&self, trace_name: &str) -> Option<&Self> {
        if trace_name == self._id_trace_name {
            Some(self)
        } else {
            match &self.kind {
                VariableKind::Struct { fields } | VariableKind::Vector { fields } => {
                    println!("trace_name: {:?}", trace_name);
                    fields.iter().find_map(|field| field.find_var(trace_name))
                }
                VariableKind::Ground | VariableKind::External => None,
            }
        }
    }
}

impl TraceGetter for Variable {
    fn get_trace_name(&self) -> &String {
        &self._id_trace_name
    }
    fn get_trace_path(&self) -> Vec<&String> {
        // TODO: implement get_trace_path for Variable
        todo!("Implement get_trace_path for Variable")
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Structure to store the type information of a variable.
#[derive(Debug, Clone, PartialEq)]
pub struct TypeInfo {
    /// The type name of the variable
    pub type_name: String,
    /// The parameters of the type (if any)
    pub params: Vec<ConstructorParams>,
}

impl TypeInfo {
    pub fn new(type_name: String, params: Vec<ConstructorParams>) -> Self {
        Self { type_name, params }
    }
}

/// The constructor parameters in a source language type
#[derive(Debug, Clone, PartialEq)]
pub struct ConstructorParams {
    /// The name of the parameter
    pub name: String,
    /// The type of the parameter
    pub tpe: String,
    /// The value of the parameter used (not always available)
    pub value: Option<String>,
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
