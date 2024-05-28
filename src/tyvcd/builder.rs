use super::spec::*;
use super::trace_pointer::TracePointer;
use std::collections::HashMap;

use crate::hgldd::spec::{self as hgldd};

pub fn from_hgldd(hgldd: &hgldd::Hgldd) -> TyVcd {
    // The scopes already found and their names
    let mut scopes: HashMap<String, Scope> = HashMap::new();
    // let mut scopes = Vec::new();

    // Iterates over the hgldd objects
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

                // Check the port vars inside the module
                for var in &obj.port_vars {
                    let variable = get_variable(var, &hgldd.objects);
                    scope.variables.push(variable);
                }

                // if let Some(child_instance) = obj.children {
                //     todo!()
                // }

                // Update the found scopes
                scopes.insert(scope.get_trace_name(), scope);
            }
            // Ignore the struct
            hgldd::ObjectKind::Struct => {}
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
    println!("Adjusting vector expressions: {:?}", expressions);

    // Find the fields of this dimension
    let mut fields = Vec::with_capacity(size as usize);
    for j in 0..size {
        let expr = expressions.get(j).cloned();
        let subexpressions = get_sub_expressions(&expr);
        let trace_name = get_trace_names(&expr).unwrap_or(String::from("default"));

        println!("Expression({}): {:?}", j, expressions.get(j));
        // Get subexpressions for the new dimension
        println!("subexpressions({}): {:?}", j, subexpressions);
        println!("trace_name({}): {:?}", j, trace_name);

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
    println!(
        "var: {:?},\n\t input value: {:?},\n\t trace_name: {:?}",
        hgldd_var.var_name, hgldd_var.value, trace_name
    );

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

            println!("obj: {:?}", obj.obj_name);
            let mut fields = Vec::with_capacity(obj.port_vars.len());
            for i in 0..obj.port_vars.len() {
                let mut var = obj.port_vars[i].clone();
                var.value = Some(expressions[i].clone());
                println!("\t subvar: {:?}", var.value);
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

pub fn get_instance(hgldd_var: &hgldd::Instance) {
    // Create a new variable

    todo!()
}
