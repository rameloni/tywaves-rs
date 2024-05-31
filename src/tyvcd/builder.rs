use super::spec::*;
use super::trace_pointer::TraceGetter;
use std::collections::HashMap;

use crate::hgldd::spec as hgldd;

/// Trait for a generic TyVcd builder.
pub trait GenericBuilder {
    fn build(&mut self);
    fn get_ref(&self) -> Option<&TyVcd>;
    fn get_copy(&self) -> Option<TyVcd>;
}

/// A concrete builder for the TyVcd object.
pub struct TyVcdBuilder<T> {
    // The input list of objects from which the TyVcd object will be built
    origin_list: Vec<T>,
    // The target TyVcd object
    tyvcd: Option<TyVcd>,
}

impl GenericBuilder for TyVcdBuilder<hgldd::Hgldd> {
    /// Build a [TyVcd] from a list of [hgldd::Hgldd] objects.
    fn build(&mut self) {
        type ScopeId = String;

        // Store the scopes found in the hgldd objects
        let mut scopes: HashMap<ScopeId, Scope> = HashMap::new();

        // Iterates over the hgldd objects
        for hgldd in &self.origin_list {
            for obj in &hgldd.objects {
                match obj.kind {
                    // Ignore the struct
                    hgldd::ObjectKind::Struct => {}
                    hgldd::ObjectKind::Module => {
                        // Retrieve the information of the scope
                        let trace_name = if let Some(module_name) = &obj.module_name {
                            module_name
                        } else {
                            &obj.obj_name
                        };
                        let high_level_info =
                            Self::create_type_info_or(obj.source_lang_type_info.as_ref(), || {
                                obj.obj_name.clone()
                            });

                        // Create a scope from the module
                        let mut scope =
                            Scope::empty(trace_name.clone(), obj.obj_name.clone(), high_level_info);

                        // Check the children of this scope
                        if let Some(children) = &obj.children {
                            for inst in children {
                                let emptyscope = Self::create_empty_scope_from_instance(inst);
                                scope.subscopes.push(emptyscope);
                            }
                        }
                        // Check the port vars inside the module
                        for var in &obj.port_vars {
                            let variable = Self::create_variable(var, &hgldd.objects);
                            scope.variables.push(variable);
                        }

                        // Update the found scopes
                        scopes.insert(scope.get_trace_name().clone(), scope);
                    }
                }
            }
        }

        // Create the TyVcd object
        self.tyvcd = Some(TyVcd { scopes });

        let _ = self.fill_tyvcd_subscopes();
    }

    // Returns the TyVcd object. Hint: call it after the build method
    fn get_ref(&self) -> Option<&TyVcd> {
        self.tyvcd.as_ref()
    }

    // Returns a copy of the TyVcd object
    fn get_copy(&self) -> Option<TyVcd> {
        self.tyvcd.clone()
    }
}

impl TyVcdBuilder<hgldd::Hgldd> {
    /// Creates a new TyVcdBuilder object.
    pub fn init(hgldd_list: Vec<hgldd::Hgldd>) -> Self {
        Self {
            origin_list: hgldd_list,
            tyvcd: None,
        }
    }

    // Create an empty scope from an hgldd instace
    fn create_empty_scope_from_instance(hgldd_inst: &hgldd::Instance) -> Scope {
        let trace_name = if let Some(hdl_obj_name) = &hgldd_inst.hdl_obj_name {
            hdl_obj_name
        } else {
            &hgldd_inst.name
        };

        let name = if let Some(hgl_obj_name) = &hgldd_inst.obj_name {
            hgl_obj_name
        } else {
            &hgldd_inst.name
        };

        // Use the name of the instance here -> it'll be replaced by calling fill_tyvcd_subscopes() (if an actual type is present)
        let high_level_info = Self::create_type_info_or(None, || name.clone());

        // Create a new scope from an instance
        Scope::empty(trace_name.clone(), name.clone(), high_level_info)
    }

    // Create a variable from an hgldd variable
    fn create_variable(hgldd_var: &hgldd::Variable, objects: &Vec<hgldd::Object>) -> Variable {
        let trace_name = if let Some(trace_name) =
            helper::get_trace_name_from_expression(hgldd_var.value.as_ref())
        {
            trace_name
        } else {
            // TODO: check how to handle this case
            String::from("todo ")
        };
        let name = &hgldd_var.var_name;
        let high_level_info = Self::create_type_info_or(
            hgldd_var.source_lang_type_info.as_ref(),
            || match &hgldd_var.type_name {
                None => "na".to_string(), // TODO: ensure this na is fine
                Some(e) => e.to_string(),
            },
        );

        // Get the expressions of this variable
        let expressions = helper::get_sub_expressions(hgldd_var.value.as_ref());

        // Build the kind of this type
        let kind = match &hgldd_var.type_name {
            // If type_name is None, this is an external variable
            None => VariableKind::External,
            Some(hgldd::TypeName::Logic) | Some(hgldd::TypeName::Bit) => VariableKind::Ground,
            Some(hgldd::TypeName::Custom(custom_type_name)) => {
                // Find the custom typeName from the list of objects
                let obj = objects
                    .iter()
                    .find(|o| {
                        o.kind == hgldd::ObjectKind::Struct && &o.obj_name == custom_type_name
                    })
                    .unwrap();

                // Build the fields of the struct
                let mut fields: Vec<Variable> = Vec::with_capacity(obj.port_vars.len());
                #[allow(clippy::needless_range_loop)]
                for i in 0..obj.port_vars.len() {
                    let mut var = obj.port_vars[i].clone();
                    var.value = Some(expressions[i].clone());
                    fields.push(Self::create_variable(&var, objects));
                }

                VariableKind::Struct { fields }
            }
        };

        // Check if this type is in a vector or not
        let final_kind: VariableKind =
            if let Some(hgldd::UnpackedRange(dims)) = &hgldd_var.unpacked_range {
                VariableKind::Vector {
                    fields: Self::create_vector_fields(&kind, expressions, dims, &high_level_info),
                }
            } else {
                kind
            };

        Variable::new(trace_name, name.clone(), high_level_info, final_kind)
    }

    // Build the fields of a vector variable.
    fn create_vector_fields(
        kind: &VariableKind,
        expressions: &[hgldd::Expression],
        dims: &[u32],
        high_level_info: &TypeInfo,
    ) -> Vec<Variable> {
        // No fields: empty vector
        if dims.len() < 2 {
            return Vec::new();
        }

        // Get the range of the current dimension [a:b]
        let (a, b) = (dims[0], dims[1]);
        let size = (a - b + 1) as usize;

        // Find the fields of this dimension
        let mut fields = Vec::with_capacity(size);
        // for i in 0..size {
        for expr in expressions {
            // let expr = expressions.get(i);
            let expr = Some(expr);
            let subexpressions = helper::get_sub_expressions(expr);
            let trace_name = helper::get_trace_name_from_expression(expr)
                .unwrap_or_else(|| String::from("default from create_vector_fields")); // TODO: update this default

            // Adjust the trace name of this kind
            let mut kind = kind.clone(); // TODO: remove this clone
            Self::update_fields_trace_name(&mut kind, subexpressions);

            let kind = if dims.len() > 2 {
                // If there are more dimensions, this is a vector of vectors
                VariableKind::Vector {
                    fields: Self::create_vector_fields(
                        &kind,
                        subexpressions,
                        &dims[2..],
                        high_level_info,
                    ),
                }
            } else {
                kind
            };

            // Build the var
            let var = Variable::new(
                trace_name,
                "todo from build_vector_fields".to_string(), // TODO: update this todo from build_vector_fields
                high_level_info.clone(),
                // TypeInfo::new("type_name".to_string(), Vec::new()),
                kind,
            );

            // let mut var = hgldd_var.clone();
            // var.value = Some(expressions[j].clone());
            fields.push(var);
        }

        fields
    }

    // Recursively update the trace names of the fields of a vector or struct variable.
    fn update_fields_trace_name(kind: &mut VariableKind, expressions: &[hgldd::Expression]) {
        // let subexpressions = helper::get_sub_expressions(expression);
        let subexpressions = expressions;
        match kind {
            VariableKind::Vector { fields } | VariableKind::Struct { fields } => {
                // Update the trace name of this kind
                #[allow(clippy::needless_range_loop)]
                for i in 0..fields.len() {
                    let expr = subexpressions.get(i);
                    if let Some(trace_name) = helper::get_trace_name_from_expression(expr) {
                        fields[i].update_trace_name(trace_name);
                    }
                    Self::update_fields_trace_name(
                        &mut fields[i].kind,
                        helper::get_sub_expressions(expr),
                    );
                }
            }
            _ => {}
        }
    }

    /// This method checks for subscope definitions among the `tyvcd.scopes` and
    /// applies the correct definitions to the respective subscopes.
    /// Indeed, after the first parsing all the `subscopes` do not contain their
    /// actual definitions (i.e. the variables and subscopes they contain)
    /// if there are scopes in
    /// `tyvcd` that should be instead subscopes of other scopes.
    fn fill_tyvcd_subscopes(&mut self) -> Result<(), &'static str> {
        let tyvcd = self.tyvcd.as_mut().ok_or("TyVcd not initialized")?;

        let copy_scopes = tyvcd.scopes.clone();

        // For each Scope found in the list
        for scope_def in tyvcd.scopes.values_mut() {
            // 1. Check if it has instances (subscopes)
            for subscope in &mut scope_def.subscopes {
                // 1.1 if yes, search for module definitions in the same list
                let subscope_name = &subscope.name;

                // Get the module definition
                if let Some(module_def) = copy_scopes.get(subscope_name) {
                    // SubScope contains the actual trace_name
                    subscope.high_level_info = module_def.high_level_info.clone();
                    subscope.name = module_def.name.clone();
                    subscope.variables = module_def.variables.clone();
                    subscope.subscopes = module_def.subscopes.clone();
                }
            }
        }

        // Keep only the top scopes
        let mut top_scopes = HashMap::new();
        for (name, scope) in &tyvcd.scopes {
            if !tyvcd
                .scopes
                .values()
                .any(|s| s.subscopes.iter().any(|ss| &ss.name == name))
            {
                top_scopes.insert(name.clone(), scope.clone());
            }
        }
        tyvcd.scopes = top_scopes;
        Ok(())
    }

    /// Create type info from source language type in hgldd
    fn create_type_info_or<F: Fn() -> String>(
        src_lang_tp: Option<&hgldd::SourceLangType>,
        default_type_name: F,
    ) -> TypeInfo {
        if let Some(x) = src_lang_tp {
            let params = if let Some(x) = &x.params {
                x.iter()
                    .map(|x| Into::<crate::tyvcd::spec::ConstructorParams>::into(x.clone()))
                    .collect()
            } else {
                Vec::new()
            };
            let type_name = if let Some(x) = &x.type_name {
                x.clone()
            } else {
                default_type_name()
            };
            TypeInfo::new(type_name, params)
        } else {
            // TypeInfo::new("todo: missing all source lang info".to_string(), Vec::new())
            TypeInfo::new(default_type_name(), Vec::new())
        }
    }
}

mod helper {
    use crate::hgldd::spec as hgldd;

    use super::ConstructorParams;

    /// Get the trace names from an hgldd expression.
    #[inline]
    pub(in crate::tyvcd) fn get_trace_name_from_expression(
        expression: Option<&hgldd::Expression>,
    ) -> Option<String> {
        if let Some(expression) = expression {
            match &expression {
                // This variable contains a value
                hgldd::Expression::SigName(s) => Some(s.clone()),
                hgldd::Expression::BitVector(bv) => Some(bv.clone()),
                hgldd::Expression::IntegerNum(i) => Some(i.to_string()),
                // This variable contains an operator, this means it contains the "values" of all its child variables (to be added in kind)
                hgldd::Expression::Operator { .. } => None,
            }
        } else {
            None
        }
    }

    #[inline]
    pub(in crate::tyvcd) fn get_sub_expressions(
        expression: Option<&hgldd::Expression>,
    ) -> &[hgldd::Expression] {
        if let Some(expression) = expression {
            match expression {
                // This variable contains a value
                hgldd::Expression::SigName(_)
                | hgldd::Expression::BitVector(_)
                | hgldd::Expression::IntegerNum(_) => std::slice::from_ref(expression),
                // This variable contains an operator, this means it contains the "values" of all its child variables (to be added in kind)
                hgldd::Expression::Operator { opcode, operands } => match opcode {
                    &hgldd::Opcode::Struct => {
                        // if operands.len() == 1 {
                        //     get_sub_expressions(&operands.first().cloned())
                        // } else {
                        operands.as_slice()
                        // }
                    }
                    _ => operands.as_slice(),
                },
            }
        } else {
            &[]
        }
    }

    impl From<hgldd::ConstructorParams> for ConstructorParams {
        fn from(val: hgldd::ConstructorParams) -> Self {
            ConstructorParams {
                name: val.name,
                tpe: val.tpe,
                value: val.value,
            }
        }
    }
}
