use super::{spec::*, trace_pointer::TraceGetter};
use crate::hgldd::spec as hgldd;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

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
        // Store the scopes found in the hgldd objects
        let mut scopes: HashMap<ScopeId, Arc<RwLock<Scope>>> = HashMap::new();

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
                                // scope.subscopes.push(emptyscope);
                                scope.subscopes.insert(
                                    emptyscope.get_trace_name().unwrap().clone(),
                                    Arc::new(RwLock::new(emptyscope)),
                                );
                            }
                        }
                        // Check the port vars inside the module
                        for var in &obj.port_vars {
                            let variable = Self::create_variable(var, &hgldd.objects);
                            scope.variables.push(variable);
                        }

                        // Update the found scopes, the key is the name trace_name of the scope
                        scopes.insert(scope.name.clone(), Arc::new(RwLock::new(scope)));
                    }
                }
            }
        }

        // Create the TyVcd object
        self.tyvcd = Some(TyVcd { scopes });

        let _ = self.fill_tyvcd_subscopes();
    }

    // Returns the TyVcd object. Hint: call it after the build method
    fn get_ref<'a>(&'a self) -> Option<&'a TyVcd> {
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

    /// Add extra scopes to the TyVcd object.
    pub fn with_extra_artifact_scopes(
        self,
        extra_scopes: Vec<String>,
        top_module_name: &String,
    ) -> Self {
        let mut _self = self;
        _self.origin_list = crate::hgldd::reader::add_extra_modules(
            _self.origin_list,
            extra_scopes,
            top_module_name,
        );
        _self
    }

    // Create an empty scope from an hgldd instace
    fn create_empty_scope_from_instance(hgldd_inst: &hgldd::Instance) -> Scope {
        // Identify among the traces
        let trace_name = if let Some(hdl_obj_name) = &hgldd_inst.hdl_obj_name {
            // hdl_obj_name // TODO: this wasn't working, no idea why
            &hgldd_inst.name_id
        } else {
            &hgldd_inst.name_id
        };

        // Identify the definition of the instance from HGL
        let name = if let Some(hgl_module_name) = &hgldd_inst.hgl_module_name {
            hgl_module_name
        } else {
            &hgldd_inst.name_id
        };

        // Use the name of the instance here -> it'll be replaced by calling fill_tyvcd_subscopes() (if an actual type is present)
        let high_level_info = Self::create_type_info_or(None, || name.clone());

        // Create a new scope from an instance
        let s = Scope::empty(trace_name.clone(), name.clone(), high_level_info);
        s
    }

    // Create a variable from an hgldd variable
    fn create_variable(hgldd_var: &hgldd::Variable, objects: &Vec<hgldd::Object>) -> Variable {
        let trace_value =
            helper::get_trace_value_from_expression(hgldd_var.value.as_ref()).unwrap(); // TODO: check this unwrap()

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
            Some(hgldd::TypeName::Logic) => {
                if let Some(packed_range) = &hgldd_var.packed_range {
                    VariableKind::Ground(u128::from(packed_range))
                } else {
                    VariableKind::Ground(1) // Default to 1 bit
                }
            }
            Some(hgldd::TypeName::Bit) => VariableKind::Ground(1),
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

        Variable::new(trace_value, name.clone(), high_level_info, final_kind)
    }

    // Build the fields of a vector variable.
    fn create_vector_fields(
        kind: &VariableKind, // the kind of the vector elements
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
        for (idx, expr) in expressions.iter().enumerate() {
            // let expr = expressions.get(i);
            let expr = Some(expr);
            let subexpressions = helper::get_sub_expressions(expr);
            let trace_value = helper::get_trace_value_from_expression(expr).unwrap(); // TODO: check this unwrap()

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
                trace_value,
                idx.to_string(),
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
                    // if let Some(trace_name) = helper::get_trace_name_from_expression(expr) {
                    //     fields[i].update_trace_name(trace_name);
                    // }
                    if let Some(trace_value) = helper::get_trace_value_from_expression(expr) {
                        fields[i].update_trace_value(trace_value);
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

        // Get a copy of the scopes
        let original_scope_map = tyvcd.scopes.clone();

        // For each Scope found in the list
        for (_, scope_def) in tyvcd.scopes.iter_mut() {
            // Get the scope definition
            let mut scope_def = scope_def.write().unwrap();

            // 1. Check if it has instances (subscopes)
            for (_, subscope_def) in (*scope_def).subscopes.iter_mut() {
                let mut subscope_def = subscope_def.write().unwrap();
                // Get the real module definition from the original list
                if let Some(module_def) = original_scope_map.get(&subscope_def.name) {
                    let module_def = module_def.read().unwrap();
                    // SubScope contains the actual trace_name
                    subscope_def.high_level_info = module_def.high_level_info.clone();
                    subscope_def.name = module_def.name.clone();
                    subscope_def.variables = module_def.variables.clone();
                    subscope_def.subscopes = module_def.subscopes.clone();
                }
            }
        }

        // Keep only the top scopes
        let mut top_scopes = HashMap::new();
        for (def_name, scope) in &tyvcd.scopes {
            if !tyvcd.scopes.values().any(|s| {
                let s = s.read().unwrap();
                // s.subscopes.get(trace_name).is_some()
                s.subscopes
                    .iter()
                    .any(|(_, ss)| &ss.read().unwrap().name == def_name)
            }) {
                top_scopes.insert(def_name.clone(), scope.clone());
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
    use crate::{
        hgldd::spec as hgldd,
        tyvcd::trace_pointer::{ConstValue, TraceValue},
    };

    use super::ConstructorParams;

    /// Get the trace names from an hgldd expression.
    #[inline]
    pub(in crate::tyvcd) fn get_trace_value_from_expression(
        expression: Option<&hgldd::Expression>,
    ) -> Option<TraceValue> {
        if let Some(expression) = expression {
            match &expression {
                // This variable contains a value
                hgldd::Expression::SigName(s) => Some(TraceValue::RefTraceName(s.clone())),
                hgldd::Expression::BitVector(bv) => Some(TraceValue::Constant(
                    ConstValue::FourValue(bv.as_bytes().to_vec(), bv.len() as u32),
                )),
                hgldd::Expression::IntegerNum(i) => {
                    let bv = format!("{:b}", i);
                    let bv = bv.as_bytes();
                    Some(TraceValue::Constant(ConstValue::FourValue(
                        bv.to_vec(),
                        bv.len() as u32,
                    )))
                }
                // This variable contains an operator, this means it contains the "values" of all its child variables (to be added in kind)
                hgldd::Expression::Operator { opcode, operands } => {
                    // TODO: check how to use the opcode here to calculate the right value
                    let mut v = Vec::with_capacity(operands.len());
                    for o in operands {
                        if let Some(x) = get_trace_value_from_expression(Some(o)) {
                            v.push(x);
                        }
                    }
                    Some(TraceValue::RefTraceValues(v))
                }
            }
        } else {
            Some(TraceValue::RefTraceName("todo: no expression".to_string())) // TODO: check this default
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

#[cfg(test)]
mod test {
    use crate::hgldd;
    use crate::tyvcd::builder::GenericBuilder;
    use crate::tyvcd::trace_pointer::TraceGetter;
    #[test]
    fn test_sub_sub_scopes() {
        let hgldd_str = r#"
        { "HGLDD": { "version": "1.0", "file_info": [] },
            "objects": [{ "kind": "module", "obj_name": "D", "module_name": "D", "source_lang_type_info": { "type_name": "D" },
                "port_vars": [
                  { "var_name": "clock", "value": {"sig_name":"clock"}, "type_name": "logic", "source_lang_type_info": { "type_name": "IO[Clock]" }},
                  { "var_name": "reset", "value": {"sig_name":"reset"}, "type_name": "logic", "source_lang_type_info": { "type_name": "IO[Bool]" }},
                  { "var_name": "i",     "value": {"sig_name":"i"},     "type_name": "logic", "source_lang_type_info": { "type_name": "IO[Bool]" }}
                ],
                "children": [{ "name": "c1", "obj_name": "C", "module_name": "C" }]
              }]
        } 
        { "HGLDD": { "version": "1.0", "file_info": [] },
            "objects": [{
                "kind": "module", "obj_name": "C", "module_name": "C", "source_lang_type_info": { "type_name": "C" },
                "port_vars": [
                  { "var_name": "clock", "value": {"sig_name":"clock"}, "type_name": "logic", "source_lang_type_info": { "type_name": "IO[Clock]" }},
                  { "var_name": "reset", "value": {"sig_name":"reset"}, "type_name": "logic", "source_lang_type_info": { "type_name": "IO[Reset]" }},
                  { "var_name": "i",     "value": {"sig_name":"i"},     "type_name": "logic", "source_lang_type_info": { "type_name": "IO[Bool]" }}
                ],
                "children": [
                  { "name": "B_0", "hdl_obj_name": "B", "obj_name": "B", "module_name": "B" },
                  { "name": "B_1",                      "obj_name": "B", "module_name": "B" }
                ]
            }]
        }
        { "HGLDD": { "version": "1.0", "file_info": [] },
            "objects": [
              {
                "kind": "module", "obj_name": "B", "module_name": "B", "source_lang_type_info": { "type_name": "B" },
                "port_vars": [{ "var_name": "i", "value": {"sig_name":"i"}, "type_name": "logic", "source_lang_type_info": { "type_name": "IO[Bool]" }}],
                "children": [
                  { "name": "A_0", "hdl_obj_name": "A", "obj_name": "A", "module_name": "A" },
                  { "name": "A_1",                      "obj_name": "A", "module_name": "A" }
                ]
              }]
        }
        { "HGLDD": { "version": "1.0", "file_info": [] },
            "objects": [
              { 
                "kind": "module", "obj_name": "A", "module_name": "A", "source_lang_type_info": { "type_name": "A"},
                "port_vars": [{ "var_name": "i", "value": {"sig_name":"i"}, "type_name": "logic", "source_lang_type_info": { "type_name": "IO[Bool]" }}],
                "children": []
              }
            ]
        }
        "#;

        let hgldds = hgldd::reader::parse_hgldds_pub(hgldd_str);
        let mut builder = super::TyVcdBuilder::init(hgldds);
        builder.build();
        let tyvcd = builder.get_ref();

        // Check it is created
        assert!(tyvcd.is_some());
        let tyvcd = tyvcd.unwrap();
        // tyvcd contain the top scope D
        assert!(tyvcd.scopes.get("D").is_some());
        // Check the hierarchy
        // D
        // |_ c1: C
        //   |_ B_0: B
        //     |_ A_0: A
        //     |_ A_1: A
        //   |_ B_1: B
        //     |_ A_0: A
        //     |_ A_1: A

        // D has c1 only as subscope
        let d = tyvcd.scopes.get("D").unwrap().read().unwrap();
        assert_eq!(d.subscopes.len(), 1);
        let c1 = d.subscopes.get("c1").unwrap().read().unwrap();
        assert_eq!(c1.get_trace_name().unwrap(), "c1");
        // c1 has B_0 and B_1 as subscopes
        let (b0, b1) = (
            c1.subscopes.get("B_0").unwrap().read().unwrap(),
            c1.subscopes.get("B_1").unwrap().read().unwrap(),
        );
        assert_eq!(b0.get_trace_name().unwrap(), "B_0");
        assert_eq!(b1.get_trace_name().unwrap(), "B_1");
        // B_0 and B_1 have A_0 and A_1 as subscopes
        let (a0_b0, a1_b0) = (
            b0.subscopes.get("A_0").unwrap().read().unwrap(),
            b0.subscopes.get("A_1").unwrap().read().unwrap(),
        );
        let (a0_b1, a1_b1) = (
            b1.subscopes.get("A_0").unwrap().read().unwrap(),
            b1.subscopes.get("A_1").unwrap().read().unwrap(),
        );
        assert_eq!(a0_b0.get_trace_name().unwrap(), "A_0");
        assert_eq!(a1_b0.get_trace_name().unwrap(), "A_1");
        assert_eq!(a0_b1.get_trace_name().unwrap(), "A_0");
        assert_eq!(a1_b1.get_trace_name().unwrap(), "A_1");

        // Check the variables in the hierarchy
        assert_eq!(d.variables.len(), 3); // i, clock, reset
        assert_eq!(c1.variables.len(), 3); // i, clock, reset
        assert_eq!(b0.variables.len(), 1); // i
        assert_eq!(b1.variables.len(), 1); // i
        assert_eq!(a0_b0.variables.len(), 1); // i
        assert_eq!(a1_b0.variables.len(), 1); // i
        assert_eq!(a0_b1.variables.len(), 1); // i
        assert_eq!(a1_b1.variables.len(), 1); // i

        // tyvcd contain only D in its definitions
        assert_eq!(tyvcd.scopes.len(), 1)
    }

    #[test]
    fn test_with_extra_artifacts() {
        let extra_modules = vec!["TOP_TB".to_string(), "dut".to_string()];
        let top_module_name = "A".to_string();
        let hgldd = r#"
        { 
            "HGLDD": { "version": "1.0", "file_info": [] },
            "objects": [
                { 
                    "kind": "module", "obj_name": "A", "module_name": "A", "source_lang_type_info": { "type_name": "A"},
                    "port_vars": [{ "var_name": "i", "value": {"sig_name":"i"}, "type_name": "logic", "source_lang_type_info": { "type_name": "IO[Bool]" }}],
                    "children": []
                }
            ]
        }"#;

        let hgldds = hgldd::reader::parse_hgldds_pub(hgldd);
        // Add extra modules
        let mut builder = super::TyVcdBuilder::init(hgldds)
            .with_extra_artifact_scopes(extra_modules, &top_module_name);

        builder.build();

        let tyvcd = builder.get_ref().unwrap();
        // Old hierachy was just A
        // New hierarchy is:
        // TOP_TB
        // |_ dut: A
        assert!(tyvcd.scopes.get("TOP_TB").is_some());
        let top_tb = tyvcd.scopes.get("TOP_TB").unwrap().read().unwrap();
        assert_eq!(top_tb.subscopes.len(), 1);
        let dut = top_tb.subscopes.get("dut").unwrap().read().unwrap();
        assert_eq!(dut.get_trace_name().unwrap(), "dut");
        assert_eq!(dut.name, "A");
        assert_eq!(dut.variables.len(), 1);

        // Check that only there is only one top scope
        assert_eq!(tyvcd.scopes.len(), 1);
    }
}
