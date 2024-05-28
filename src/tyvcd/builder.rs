use super::spec::*;
use super::trace_pointer::TracePointer;
use std::collections::HashMap;

use crate::hgldd::spec::{self as hgldd};

pub fn add_instance_links(tyvcd: &mut TyVcd) {
    let copy_scopes = tyvcd.scopes.clone();

    // For each Scope found in the list
    for (ref_scope, scope_def) in &mut tyvcd.scopes {
        // 1. Check if it has instances (subscopes),
        for subscope in &mut scope_def.subscopes {
            //      1.1 if yes, search for module definitions in the same list
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
}

pub fn from_hgldd(hgldd_list: &Vec<hgldd::Hgldd>) -> TyVcd {
    // The scopes already found and their names
    let mut scopes: HashMap<String, Scope> = HashMap::new();
    // let mut scopes = Vec::new();

    // Iterates over the hgldd objects
    for hgldd in hgldd_list {
        for obj in &hgldd.objects {
            match obj.kind {
                hgldd::ObjectKind::Module => {
                    // Retrieve the information of the scope
                    let high_level_info = TypeInfo::new("todo".to_string(), Vec::new());

                    let mut scope = if let Some(module_name) = obj.module_name.clone() {
                        Scope::empty(module_name, obj.obj_name.clone(), high_level_info)
                    } else {
                        Scope::empty(obj.obj_name.clone(), obj.obj_name.clone(), high_level_info)
                    };

                    // Check the children of this scope
                    if let Some(children) = &obj.children {
                        for inst in children {
                            let subscope = get_instance(inst);
                            scope.subscopes.push(subscope);
                        }
                    }

                    // Check the port vars inside the module
                    for var in &obj.port_vars {
                        let variable = get_variable(var, &hgldd.objects);
                        scope.variables.push(variable);
                    }

                    // Update the found scopes
                    scopes.insert(scope.get_trace_name(), scope);
                }
                // Ignore the struct
                hgldd::ObjectKind::Struct => {}
            }
        }
    }
    // An hgldd contains a list of objects, each o
    TyVcd { scopes }
}

#[inline]
fn get_trace_names(expression: &Option<hgldd::Expression>) -> Option<String> {
    if let Some(expression) = expression {
        match &expression {
            // This variable contains a value
            hgldd::Expression::SigName(s) => Some(s.clone()),
            hgldd::Expression::BitVector(s) => Some(s.clone()),
            hgldd::Expression::IntegerNum(i) => Some(i.to_string()),
            // This variable contains an operator, this means it contains the "values" of all its child variables (to be added in kind)
            hgldd::Expression::Operator { operands, .. } => None,
        }
    } else {
        None
    }
}

#[inline]
fn get_sub_expressions(expression: &Option<hgldd::Expression>) -> Vec<hgldd::Expression> {
    if let Some(expression) = expression {
        match &expression {
            // This variable contains a value
            hgldd::Expression::SigName(s) => vec![expression.clone()],
            hgldd::Expression::BitVector(s) => vec![expression.clone()],
            hgldd::Expression::IntegerNum(i) => vec![expression.clone()],
            // This variable contains an operator, this means it contains the "values" of all its child variables (to be added in kind)
            hgldd::Expression::Operator { opcode, operands } => match opcode {
                &hgldd::Opcode::Struct => {
                    // if operands.len() == 1 {
                    //     get_sub_expressions(&operands.first().cloned())
                    // } else {
                    operands.clone()
                    // }
                }

                //todo!("Operator: {:?}", opcode),
                _ => operands.clone(),
            },
        }
    } else {
        Vec::new()
    }
}

fn update_trace_name(kind: &mut VariableKind, expression: &Option<hgldd::Expression>) {
    let subexpressions = get_sub_expressions(expression);

    match kind {
        VariableKind::Vector { fields } | VariableKind::Struct { fields } => {
            // Update the trace name of this kind
            for i in 0..fields.len() {
                let trace_name = get_trace_names(&subexpressions.get(i).cloned())
                    .unwrap_or(String::from("default"));
                fields[i].update_trace(trace_name.clone());
                update_trace_name(&mut fields[i].kind, &subexpressions.get(i).cloned());
            }
        }
        _ => {}
    }
}

fn build_vector_fields(
    kind: &VariableKind,
    expressions: &Vec<hgldd::Expression>,
    dims: &[u32],
) -> Vec<Variable> {
    // No fields
    if dims.len() < 2 {
        return Vec::new();
    }

    // Get the range of the current dimension [a:b]
    let (a, b) = (dims[0], dims[1]);
    let size = (a - b + 1) as usize;

    // Find the fields of this dimension
    let mut fields = Vec::with_capacity(size as usize);
    for j in 0..size {
        let expr = expressions.get(j).cloned();
        let subexpressions = get_sub_expressions(&expr);
        let trace_name = get_trace_names(&expr).unwrap_or(String::from("default"));

        // Get subexpressions for the new dimension

        // Adjust the trace name of this kind
        let mut kind = kind.clone();
        update_trace_name(&mut kind, &expr);

        let kind = if dims.len() > 2 {
            VariableKind::Vector {
                fields: build_vector_fields(&kind, &subexpressions, &dims[2..]),
            }
        } else {
            kind.clone()
        };
        // Build the var
        let var = Variable::new(
            trace_name,
            format!("todo: {}", j),
            TypeInfo::new("todo".to_string(), Vec::new()),
            kind,
        );

        // let mut var = hgldd_var.clone();
        // var.value = Some(expressions[j].clone());
        fields.push(var);
    }

    fields
}

fn get_variable(hgldd_var: &hgldd::Variable, objects: &Vec<hgldd::Object>) -> Variable {
    let high_level_info: TypeInfo = TypeInfo::new("todo".to_string(), Vec::new());

    // let kind = match &hgldd_var.value {
    //     None => todo!(),
    //     Some(expression) => get_kind_from_expression(expression, objects),
    // };
    let expressions = get_sub_expressions(&hgldd_var.value);
    let trace_name = get_trace_names(&hgldd_var.value);
    let mut second_trace_name = None;

    // Build the kind of this type
    let kind = match &hgldd_var.type_name {
        // If type_name is None, this is an external variable
        None => VariableKind::External,
        Some(hgldd::TypeName::Logic) | Some(hgldd::TypeName::Bit) => {
            // Check the unpacked range:
            // TODO: move this outside since it is unrelated to the type, unpacked range means => I have multiple variables of type_name
            // if let Some(hgldd::UnpackedRange(dims)) = &hgldd_var.unpacked_range {
            //     VariableKind::Vector {
            //         fields: build_vector_fields(&expressions, dims),
            //     }
            // } else {
            //     VariableKind::Ground
            // }
            VariableKind::Ground
        }
        Some(hgldd::TypeName::Custom(t)) => {
            second_trace_name = Some(t.clone());
            // Find the custom typeName from the list of objects
            let obj = objects
                .iter()
                .find(|o| o.kind == hgldd::ObjectKind::Struct && &o.obj_name == t)
                .unwrap();

            let mut fields = Vec::with_capacity(obj.port_vars.len());
            for i in 0..obj.port_vars.len() {
                let mut var = obj.port_vars[i].clone();
                var.value = Some(expressions[i].clone());
                fields.push(get_variable(&var, objects));
            }

            VariableKind::Struct { fields }
        }
    };

    // Check if this type is in a vector or not
    let final_kind = if let Some(hgldd::UnpackedRange(dims)) = &hgldd_var.unpacked_range {
        VariableKind::Vector {
            fields: build_vector_fields(&kind, &expressions, dims),
        }
    } else {
        kind
    };

    // Create a new variable
    let name = hgldd_var.var_name.clone();
    let trace_name = if let Some(trace_name) = trace_name {
        trace_name
    } else {
        format!("todo ")
    };
    Variable::new(trace_name, name, high_level_info, final_kind)
}

fn get_instance(hgldd_inst: &hgldd::Instance) -> Scope {
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

    let high_level_info = TypeInfo::new("todo".to_string(), Vec::new());

    // Create a new scope from an instance
    Scope::empty(trace_name.clone(), name.clone(), high_level_info)
}
