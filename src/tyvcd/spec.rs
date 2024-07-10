use crate::hgldd::spec::EnumValMap;

use super::trace_pointer::{TraceFinder, TraceGetter, TraceValue};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

/// The identifier of a scope in the TyVcd format. In the hash map, the key is the scope id.
pub type ScopeId = String;
/// Represents the definition of a scope in the TyVcd format.
pub type Scope = ScopeDef;

/// Represents the TyVcd format.
#[derive(Debug, Clone)]
pub struct TyVcd {
    /// List of top level scopes in the TyVcd format, stored as a hash map of shared references.
    pub scopes: HashMap<ScopeId, Arc<RwLock<ScopeDef>>>,
}

// Implement manually the equality trait for TyVcd (derive not possible due to the RwLock)
impl PartialEq for TyVcd {
    fn eq(&self, other: &Self) -> bool {
        let mut subscopes_check = true;
        for (this_key, this_value) in &self.scopes {
            if let Some(other_value) = other.scopes.get(this_key) {
                subscopes_check = *this_value.read().unwrap() == *other_value.read().unwrap();
            } else {
                return false;
            }
            // Return immediately
            if !subscopes_check {
                return false;
            }
        }
        subscopes_check
    }
}

impl TraceFinder for TyVcd {
    /// Return the element pointing to the trace path.
    ///
    /// **Important**: The returned reference is a reference counted with interior mutability.
    /// This means that preventing to internal changes is left to the user.
    fn find_trace(&self, full_path: &[String]) -> Option<Arc<RwLock<dyn TraceGetter>>> {
        fn find_trace_impl(
            root: &Arc<RwLock<ScopeDef>>,
            full_path: &[String],
        ) -> Option<Arc<RwLock<dyn TraceGetter>>> {
            if full_path.len() == 1 {
                // Exactly one scope -> the root
                Some(root.clone())
            } else {
                // Initalize the pointer to the root scope
                let mut guess_scope_ptr = root.clone();
                // More than one scope specified
                let mut path = &full_path[1..];
                while path.len() > 1 {
                    // Explore the path and search for the actual guessed scope
                    guess_scope_ptr = ScopeDef::find_subscope(&guess_scope_ptr, &path[0])?;
                    path = &path[1..]; // Move to the next scope
                }
                if let Some(variable) = guess_scope_ptr
                    .clone()
                    .read()
                    .unwrap()
                    .find_variable(&path[0])
                {
                    Some(Arc::new(RwLock::new(variable.clone())))
                } else {
                    let subscope = ScopeDef::find_subscope(&guess_scope_ptr, &path[0])?;
                    Some(subscope)
                }
            }
        }
        // fn find_trace<'a>(&'a self, full_path: &[String]) -> Option<Ref<'a, dyn TraceGetter>> {
        // Get the root scope
        let root = self.scopes.get(full_path.first()?)?;
        find_trace_impl(root, full_path)
    }
}

/// Represent a scope (i.e. a module instance) in the TyVcd format.
#[derive(Debug, Clone)]
pub struct ScopeDef {
    /// The name of the scope in the trace
    _id_trace_value: TraceValue,

    /// The subscopes of this scope
    pub subscopes: HashMap<ScopeId, Arc<RwLock<ScopeDef>>>,
    /// The variables declared in this scope
    pub variables: Vec<Variable>,

    /// The original name of the scope in the HGLDD file. Definition name
    pub name: String,
    /// High level information of the scope
    pub high_level_info: TypeInfo,
}

impl PartialEq for ScopeDef {
    fn eq(&self, other: &Self) -> bool {
        for (this_key, this_value) in &self.subscopes {
            let subscopes_check = if let Some(other_value) = other.subscopes.get(this_key) {
                *this_value.read().unwrap() == *other_value.read().unwrap()
            } else {
                return false;
            };
            // Return immediately
            if !subscopes_check {
                return false;
            }
        }
        self._id_trace_value == other._id_trace_value
            && self.variables == other.variables
            && self.name == other.name
            && self.high_level_info == other.high_level_info
    }
}

impl ScopeDef {
    /// Create a new empty scope without any subscopes or variables.
    pub fn empty(trace_name: String, name: String, high_level_info: TypeInfo) -> Self {
        Self {
            _id_trace_value: TraceValue::RefTraceName(trace_name),
            subscopes: HashMap::new(),
            variables: Vec::new(),
            name,
            high_level_info,
        }
    }

    /// Create a new scope definition from another with an updated trace name.
    /// *Pleas use the `clone` method directly for a full copy.*
    pub fn from_other(other: &ScopeDef, trace_name: String) -> Self {
        Self {
            _id_trace_value: TraceValue::RefTraceName(trace_name),
            subscopes: other.subscopes.clone(),
            variables: other.variables.clone(),
            name: other.name.clone(),
            high_level_info: other.high_level_info.clone(),
        }
    }

    // Find a subscope (child) in the scope definition of the current scope.
    fn find_subscope(parent: &Arc<RwLock<ScopeDef>>, name: &str) -> Option<Arc<RwLock<ScopeDef>>> {
        parent.read().unwrap().subscopes.get(name).cloned()
    }

    /// Find a variable in the scope definition.
    fn find_variable(&self, name: &str) -> Option<&Variable> {
        self.variables.iter().find_map(|v| v.find_var(name))
    }
}

impl TraceGetter for ScopeDef {
    fn get_trace_path(&self) -> Vec<&String> {
        // TODO: implement get_trace_path for ScopeDef
        todo!()
    }

    fn get_trace_value(&self) -> &TraceValue {
        &self._id_trace_value
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Represent a variable in the TyVcd format.
#[derive(Debug, Clone)]
pub struct Variable {
    /// The value of the variable in the trace.
    _trace_value: TraceValue,

    /// The name of the variable.
    pub name: String,
    /// The high level type information of the variable.
    pub high_level_info: TypeInfo,
    /// The kind of the variable.
    pub kind: VariableKind,
    /// The reference enum type if any.
    pub enum_val_map: Option<Arc<RwLock<EnumValMap>>>,
}

impl PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        if self.name != other.name
            || self.high_level_info != other.high_level_info
            || self.kind != other.kind
            || self._trace_value != other._trace_value
        {
            return false;
        }

        if let Some(enum_val_map) = &self.enum_val_map {
            if let Some(other_enum_val_map) = &other.enum_val_map {
                *enum_val_map.read().unwrap() == *other_enum_val_map.read().unwrap()
            } else {
                false
            }
        } else {
            self.enum_val_map.is_none() && other.enum_val_map.is_none()
        }
    }
}

impl Variable {
    pub fn new(
        trace_value: TraceValue,
        name: String,
        high_level_info: TypeInfo,
        kind: VariableKind,
    ) -> Self {
        Self {
            // _id_trace_name: trace_name,
            _trace_value: trace_value,
            name,
            high_level_info,
            kind,
            enum_val_map: None,
        }
    }

    pub fn with_enum_val_map(mut self, enum_val_map: EnumValMap) -> Self {
        if !enum_val_map.is_empty() {
            self.enum_val_map = Some(Arc::new(RwLock::new(enum_val_map)));
        }
        self
    }

    // /// Update the trace name of the variable.
    // pub(in crate::tyvcd) fn update_trace_name(&mut self, trace_name: String) {
    //     self._id_trace_name = trace_name;
    // }

    pub(in crate::tyvcd) fn update_trace_value(&mut self, trace_value: TraceValue) {
        self._trace_value = trace_value;
    }

    // Find a variable in the variable tree.
    fn find_var(&self, trace_name: &str) -> Option<&Self> {
        // Checkk if the trace name corresponds to the variable at the current hierarchy level
        if let Some(ref_trace_name) = self.get_trace_name() {
            if trace_name == ref_trace_name {
                return Some(self);
            }
        }
        // The variable should be in a struct or vector
        let subvar_opt = match &self.kind {
            VariableKind::Struct { fields } | VariableKind::Vector { fields } => {
                fields.iter().find_map(|field| field.find_var(trace_name))
            }
            VariableKind::Ground(_) | VariableKind::External => None,
        };

        // Check if the variable was found and return it
        if let Some(result) = subvar_opt {
            Some(result)
        } else if trace_name == self.name {
            // Otherwise check if the variable name corresponds to the current variable
            // TODO: this is a temporary fix, when vcd_rewrite is called the trace names are created from the variable names if the variable does not have a trace name
            Some(self)
        } else {
            None
        }
    }

    /// Collect all the ground variable subtypes in the variable hierarchy tree.
    #[deprecated = "Should be removed"]
    pub(crate) fn collect_ground_variables(&self) -> Vec<&Self> {
        let mut ground_variables = Vec::new();
        match &self.kind {
            VariableKind::Ground(_) | VariableKind::External => ground_variables.push(self),
            VariableKind::Vector { fields } | VariableKind::Struct { fields } => {
                for field in fields {
                    ground_variables.append(&mut field.collect_ground_variables());
                }
            }
        }
        ground_variables
    }

    #[deprecated = "Should be removed. A better version from trace value should be used instead"]
    pub fn create_val_repr(
        &self,
        raw_val_vcd: &str,
        render_fn: &dyn Fn(u64, &str) -> String,
    ) -> String {
        // let size = self.kind.find_width() as usize;
        // if raw_val_vcd.len() < size {
        //     return String::from("---");
        // }

        match &self.kind {
            // If the variable is a ground type: use the raw value directly
            VariableKind::Ground(width) => {
                if let Some(enum_val_map) = &self.enum_val_map {
                    let enum_val_map = enum_val_map.read().unwrap();
                    if let Ok(intval) = i64::from_str_radix(raw_val_vcd, 2) {
                        let render = enum_val_map.get(&intval);
                        if let Some(render) = render {
                            return render.clone();
                        }
                    }
                }
                render_fn(*width as u64, raw_val_vcd)
            }
            // Otherwise, encode the fields recursively {x, {y, z}} or [x, y, z]
            VariableKind::Vector { fields } | VariableKind::Struct { fields } => {
                // Encode the fields recursively {x, {y, z}} or [x, y, z]
                let (lb, sep, rb) = match &self.kind {
                    VariableKind::Vector { .. } => ('[', ", ", ']'),
                    VariableKind::Struct { .. } => ('{', ", ", '}'),
                    _ => unreachable!(),
                };

                // Build the value of the aggregate type
                let mut value = lb.to_string();
                let mut start_idx = 0;

                for field in fields {
                    let end_idx = start_idx + field.kind.find_width() as usize;
                    let field_str = format!(
                        "{}: {}",
                        field.name,
                        field.create_val_repr(&raw_val_vcd[start_idx..end_idx], render_fn)
                    );

                    value.push_str(&field_str);
                    value.push_str(sep);
                    start_idx = end_idx;
                }
                value.pop();
                value.pop();
                value.push(rb);
                value
            }
            VariableKind::External => todo!("Unknown type not implemented"),
        }
    }
}

impl TraceGetter for Variable {
    fn get_trace_path(&self) -> Vec<&String> {
        // TODO: implement get_trace_path for Variable
        todo!("Implement get_trace_path for Variable")
    }

    fn get_trace_value(&self) -> &TraceValue {
        &self._trace_value
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
    /// A ground type with a defined range of width
    Ground(u128),
    /// A struct-like type
    Struct { fields: Vec<Variable> },
    /// A vector-like type
    Vector { fields: Vec<Variable> },
    /// An external type (from another source). Typically when it is not possible to infer the type
    External,
}

impl VariableKind {
    pub fn find_width(&self) -> u128 {
        match self {
            VariableKind::Ground(width) => *width,
            VariableKind::Struct { fields } => fields.iter().map(|f| f.kind.find_width()).sum(),
            VariableKind::Vector { fields } => {
                if let Some(field) = fields.first() {
                    fields.len() as u128 * field.kind.find_width()
                } else {
                    0 // Empty vector
                }
            }
            VariableKind::External => 0, // TODO: for now return 0, but should be handled differently
        }
    }
}
