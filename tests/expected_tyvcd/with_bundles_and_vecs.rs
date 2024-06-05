use tywaves_rs::tyvcd::trace_pointer::TraceValue;

// External variable that will be captured.
use crate::tyvcd::spec::*;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::vec;

// Create the TyVcd for the [[tests/inputs/tyvcd/foo/foo.dd]] file.
pub fn create_with_bundles_and_vecs() -> TyVcd {
    let mut scopes = HashMap::new();
    // Main scope
    scopes.insert(
        String::from("WithBundlesAndVecs"),
        Rc::new(RefCell::new(Scope::empty(
            String::from("WithBundlesAndVecs"),
            String::from("WithBundlesAndVecs"),
            TypeInfo::new(
                "WithBundlesAndVecsModule".to_string(),
                vec![ConstructorParams {
                    name: "a_param".to_string(),
                    tpe: "bool".to_string(),
                    value: None,
                }],
            ),
        ))),
    );

    // clock
    scopes
        .get("WithBundlesAndVecs")
        .unwrap()
        .borrow_mut()
        .variables
        .push(Variable::new(
            TraceValue::RefTraceName("clock".to_string()),
            String::from("clock"),
            TypeInfo::new("logic".to_string(), Vec::new()),
            VariableKind::Ground(1),
        ));

    // reset
    scopes
        .get("WithBundlesAndVecs")
        .unwrap()
        .borrow_mut()
        .variables
        .push(Variable::new(
            TraceValue::RefTraceName("reset".to_string()),
            String::from("reset"),
            TypeInfo::new("logic".to_string(), Vec::new()),
            VariableKind::Ground(1),
        ));

    // io => struct
    scopes
        .get("WithBundlesAndVecs")
        .unwrap()
        .borrow_mut()
        .variables
        .push(Variable::new(
            TraceValue::RefTraceValues(vec![
                TraceValue::RefTraceName("io_a_0".to_string()),
                TraceValue::RefTraceValues(vec![
                    // b.a
                    TraceValue::RefTraceName("io_b_a_0".to_string()),
                    // b.b
                    TraceValue::RefTraceValues(vec![
                        // b.b.vec
                        TraceValue::RefTraceValues(vec![
                            TraceValue::RefTraceName("io_b_b_vec_0_0".to_string()), // b.b.vec[0]
                            TraceValue::RefTraceName("io_b_b_vec_1_0".to_string()), // b.b.vec[1]
                        ]),
                    ]),
                ]),
                TraceValue::RefTraceValues(vec![
                    // vec[0]
                    TraceValue::RefTraceValues(vec![
                        // vec[0].x
                        TraceValue::RefTraceName("io_vec_0_x_0".to_string()),
                        // vec[0].y
                        TraceValue::RefTraceValues(vec![
                            // vec[0].y.z
                            TraceValue::RefTraceName("io_vec_0_y_z_0".to_string()),
                        ]),
                    ]),
                    // vec[1]
                    TraceValue::RefTraceValues(vec![
                        // vec[1].x
                        TraceValue::RefTraceName("io_vec_1_x_0".to_string()),
                        // vec[1].y
                        TraceValue::RefTraceValues(vec![
                            // vec[1].y.z
                            TraceValue::RefTraceName("io_vec_1_y_z_0".to_string()),
                        ]),
                    ]),
                ]),
            ]),
            String::from("io"),
            TypeInfo::new("InterfaceIOBundle".to_string(), Vec::new()),
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
            TraceValue::RefTraceName("io_a_0".to_string()),
            String::from("a"),
            TypeInfo::new("UInt<32>".to_string(), Vec::new()),
            VariableKind::Ground(32),
        ),
        Variable::new(
            TraceValue::RefTraceValues(vec![
                // b.a
                TraceValue::RefTraceName("io_b_a_0".to_string()),
                // b.b
                TraceValue::RefTraceValues(vec![
                    // b.b.vec
                    TraceValue::RefTraceValues(vec![
                        TraceValue::RefTraceName("io_b_b_vec_0_0".to_string()), // b.b.vec[0]
                        TraceValue::RefTraceName("io_b_b_vec_1_0".to_string()), // b.b.vec[1]
                    ]),
                ]),
            ]),
            String::from("b"),
            TypeInfo::new("SubBundleB".to_string(), Vec::new()),
            VariableKind::Struct {
                fields: create_io_b_fields(),
            },
        ),
        Variable::new(
            TraceValue::RefTraceValues(vec![
                // vec[0]
                TraceValue::RefTraceValues(vec![
                    // vec[0].x
                    TraceValue::RefTraceName("io_vec_0_x_0".to_string()),
                    // vec[0].y
                    TraceValue::RefTraceValues(vec![
                        // vec[0].y.z
                        TraceValue::RefTraceName("io_vec_0_y_z_0".to_string()),
                    ]),
                ]),
                // vec[1]
                TraceValue::RefTraceValues(vec![
                    // vec[1].x
                    TraceValue::RefTraceName("io_vec_1_x_0".to_string()),
                    // vec[1].y
                    TraceValue::RefTraceValues(vec![
                        // vec[1].y.z
                        TraceValue::RefTraceName("io_vec_1_y_z_0".to_string()),
                    ]),
                ]),
            ]),
            String::from("vec"),
            TypeInfo::new("VecType".to_string(), Vec::new()),
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
            TraceValue::RefTraceName("io_b_a_0".to_string()),
            String::from("a"),
            TypeInfo::new("UInt<32>".to_string(), Vec::new()),
            VariableKind::Ground(32),
        ),
        Variable::new(
            TraceValue::RefTraceValues(vec![
                // b.b.vec
                TraceValue::RefTraceValues(vec![
                    TraceValue::RefTraceName("io_b_b_vec_0_0".to_string()), // b.b.vec[0]
                    TraceValue::RefTraceName("io_b_b_vec_1_0".to_string()), // b.b.vec[1]
                ]),
            ]),
            String::from("b"),
            TypeInfo::new("WithBundlesAndVecs_io_b_b".to_string(), Vec::new()),
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
            TraceValue::RefTraceName("io_b_b_vec_0_0".to_string()),
            String::from("0"),
            TypeInfo::new("logic".to_string(), Vec::new()),
            VariableKind::Ground(32),
        ),
        Variable::new(
            TraceValue::RefTraceName("io_b_b_vec_1_0".to_string()),
            String::from("1"),
            TypeInfo::new("logic".to_string(), Vec::new()),
            VariableKind::Ground(32),
        ),
    ];

    vec![Variable::new(
        // b.b.vec
        TraceValue::RefTraceValues(vec![
            TraceValue::RefTraceName("io_b_b_vec_0_0".to_string()), // b.b.vec[0]
            TraceValue::RefTraceName("io_b_b_vec_1_0".to_string()), // b.b.vec[1]
        ]),
        String::from("vec"),
        TypeInfo::new("logic".to_string(), Vec::new()),
        VariableKind::Vector { fields: fields },
    )]
}

// Type of WithBundlesAndVecs_io_vec
fn create_io_vec_fields() -> Vec<Variable> {
    let fields = vec![
        Variable::new(
            TraceValue::RefTraceName("io_vec_0_x_0".to_string()),
            String::from("x"),
            TypeInfo::new("SInt<32>".to_string(), Vec::new()),
            VariableKind::Ground(32),
        ),
        Variable::new(
            // vec[0].y
            TraceValue::RefTraceValues(vec![
                // vec[0].y.z
                TraceValue::RefTraceName("io_vec_0_y_z_0".to_string()),
            ]),
            String::from("y"),
            TypeInfo::new("AnonymousBundle".to_string(), Vec::new()),
            VariableKind::Struct {
                fields: create_io_vec_y_fields(),
            },
        ),
    ];

    vec![Variable::new(
        TraceValue::RefTraceValues(vec![
            // vec[0].x
            TraceValue::RefTraceName("io_vec_0_x_0".to_string()),
            // vec[0].y
            TraceValue::RefTraceValues(vec![
                // vec[0].y.z
                TraceValue::RefTraceName("io_vec_0_y_z_0".to_string()),
            ]),
        ]),
        String::from("0"),
        TypeInfo::new("VecType".to_string(), Vec::new()),
        VariableKind::Struct { fields: fields },
    )]
}

// Type of WithBundlesAndVecs_io_vec_y
fn create_io_vec_y_fields() -> Vec<Variable> {
    vec![Variable::new(
        TraceValue::RefTraceName("io_vec_0_y_z_0".to_string()),
        String::from("z"),
        TypeInfo::new("SInt<32>".to_string(), Vec::new()),
        VariableKind::Ground(32),
    )]
}

// Type of WithBundlesAndVecs_io_vec_0
fn create_io_vec_0_fields() -> Vec<Variable> {
    let fields = vec![
        Variable::new(
            TraceValue::RefTraceName("io_vec_1_x_0".to_string()),
            String::from("x"),
            TypeInfo::new("SInt<32>".to_string(), Vec::new()),
            VariableKind::Ground(32),
        ),
        Variable::new(
            // vec[0].y
            TraceValue::RefTraceValues(vec![
                // vec[0].y.z
                TraceValue::RefTraceName("io_vec_1_y_z_0".to_string()),
            ]),
            String::from("y"),
            TypeInfo::new("AnonymousBundle".to_string(), Vec::new()),
            VariableKind::Struct {
                fields: create_io_vec_y_0_fields(),
            },
        ),
    ];
    vec![Variable::new(
        TraceValue::RefTraceValues(vec![
            // vec[0].x
            TraceValue::RefTraceName("io_vec_1_x_0".to_string()),
            // vec[0].y
            TraceValue::RefTraceValues(vec![
                // vec[0].y.z
                TraceValue::RefTraceName("io_vec_1_y_z_0".to_string()),
            ]),
        ]),
        String::from("1"), // TODO: fix this name
        TypeInfo::new("VecType".to_string(), Vec::new()),
        VariableKind::Struct { fields: fields },
    )]
}

// Type of WithBundlesAndVecs_io_vec_0_y
fn create_io_vec_y_0_fields() -> Vec<Variable> {
    vec![Variable::new(
        TraceValue::RefTraceName("io_vec_1_y_z_0".to_string()),
        String::from("z"),
        TypeInfo::new("SInt<32>".to_string(), Vec::new()),
        VariableKind::Ground(32),
    )]
}
