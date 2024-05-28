use tywaves_rs::tyvcd::trace_pointer::TracePointer;
use vcd::Var;

// External variable that will be captured.
use crate::tyvcd::builder::*;
use crate::tyvcd::spec::*;

use super::*;
use std::collections::HashMap;
use std::vec;

// Create the TyVcd for the [[tests/inputs/tyvcd/foo/foo.dd]] file.
pub fn create_with_bundles_and_vecs() -> TyVcd {
    let mut scopes = HashMap::new();
    // Main scope
    scopes.insert(
        String::from("WithBundlesAndVecs"),
        Scope::empty(
            String::from("WithBundlesAndVecs"),
            String::from("WithBundlesAndVecs"),
            TypeInfo::new("todo".to_string(), Vec::new()),
        ),
    );

    // clock
    scopes
        .get_mut("WithBundlesAndVecs")
        .unwrap()
        .variables
        .push(Variable::new(
            String::from("clock"),
            String::from("clock"),
            TypeInfo::new("todo".to_string(), Vec::new()),
            VariableKind::Ground,
        ));

    // reset
    scopes
        .get_mut("WithBundlesAndVecs")
        .unwrap()
        .variables
        .push(Variable::new(
            String::from("reset"),
            String::from("reset"),
            TypeInfo::new("todo".to_string(), Vec::new()),
            VariableKind::Ground,
        ));

    // io => struct
    scopes
        .get_mut("WithBundlesAndVecs")
        .unwrap()
        .variables
        .push(Variable::new(
            String::from(""),
            String::from("io"),
            TypeInfo::new("todo".to_string(), Vec::new()),
            VariableKind::Struct {
                fields: create_io_fields(),
            },
        ));

    TyVcd { scopes }
}

// Type of WithBundlesAndVecs_io
fn create_io_fields() -> Vec<Variable> {
    vec![
        Variable::new(
            String::from("io_a_0"),
            String::from("a"),
            TypeInfo::new("todo".to_string(), Vec::new()),
            VariableKind::Ground,
        ),
        Variable::new(
            String::from(""),
            String::from("b"),
            TypeInfo::new("todo".to_string(), Vec::new()),
            VariableKind::Struct {
                fields: create_io_b_fields(),
            },
        ),
        Variable::new(
            String::from(""),
            String::from("vec"),
            TypeInfo::new("todo".to_string(), Vec::new()),
            VariableKind::Vector {
                fields: vec![create_io_vec_fields(), create_io_vec_0_fields()].concat(),
            },
        ),
    ]
}

// Type of WithBundlesAndVecs_io_b
fn create_io_b_fields() -> Vec<Variable> {
    vec![
        Variable::new(
            String::from("io_b_a_0"),
            String::from("a"),
            TypeInfo::new("todo".to_string(), Vec::new()),
            VariableKind::Ground,
        ),
        Variable::new(
            String::from(""),
            String::from("b"),
            TypeInfo::new("todo".to_string(), Vec::new()),
            VariableKind::Struct {
                fields: create_io_b_b_fields(),
            },
        ),
    ]
}

// Type of WithBundlesAndVecs_io_b_b
fn create_io_b_b_fields() -> Vec<Variable> {
    let fields: Vec<Variable> = vec![
        Variable::new(
            String::from("io_b_b_vec_0_0"), // todo: check
            String::from("0"),
            TypeInfo::new("todo".to_string(), Vec::new()),
            VariableKind::Ground,
        ),
        Variable::new(
            String::from("io_b_b_vec_1_0"),
            String::from("1"),
            TypeInfo::new("todo".to_string(), Vec::new()),
            VariableKind::Ground,
        ),
    ];

    vec![Variable::new(
        String::from(""),
        String::from("vec"),
        TypeInfo::new("todo".to_string(), Vec::new()),
        VariableKind::Vector { fields: fields },
    )]
}

// Type of WithBundlesAndVecs_io_vec
fn create_io_vec_fields() -> Vec<Variable> {
    let fields = vec![
        Variable::new(
            String::from("io_vec_0_x_0"),
            String::from("x"),
            TypeInfo::new("todo".to_string(), Vec::new()),
            VariableKind::Ground,
        ),
        Variable::new(
            String::from(""),
            String::from("y"),
            TypeInfo::new("todo".to_string(), Vec::new()),
            VariableKind::Struct {
                fields: create_io_vec_y_fields(),
            },
        ),
    ];

    vec![Variable::new(
        String::from(""),
        String::from("0"),
        TypeInfo::new("todo".to_string(), Vec::new()),
        VariableKind::Struct { fields: fields },
    )]
}

// Type of WithBundlesAndVecs_io_vec_y
fn create_io_vec_y_fields() -> Vec<Variable> {
    vec![Variable::new(
        String::from("io_vec_0_y_z_0"),
        String::from("z"),
        TypeInfo::new("todo".to_string(), Vec::new()),
        VariableKind::Ground,
    )]
}

// Type of WithBundlesAndVecs_io_vec_0
fn create_io_vec_0_fields() -> Vec<Variable> {
    let fields = vec![
        Variable::new(
            String::from("io_vec_1_x_0"),
            String::from("x"),
            TypeInfo::new("todo".to_string(), Vec::new()),
            VariableKind::Ground,
        ),
        Variable::new(
            String::from(""),
            String::from("y"),
            TypeInfo::new("todo".to_string(), Vec::new()),
            VariableKind::Struct {
                fields: create_io_vec_y_0_fields(),
            },
        ),
    ];
    vec![Variable::new(
        String::from(""),
        String::from("1"),
        TypeInfo::new("todo".to_string(), Vec::new()),
        VariableKind::Struct { fields: fields },
    )]
}

// Type of WithBundlesAndVecs_io_vec_0_y
fn create_io_vec_y_0_fields() -> Vec<Variable> {
    vec![Variable::new(
        String::from("io_vec_1_y_z_0"),
        String::from("z"),
        TypeInfo::new("todo".to_string(), Vec::new()),
        VariableKind::Ground,
    )]
}
