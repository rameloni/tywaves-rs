use crate::tyvcd::spec::*;
use crate::HashMap;
use crate::TraceValue;
use std::sync::{Arc, RwLock};

pub fn create_vecs() -> TyVcd {
    let mut scopes = HashMap::new();
    // Main scope
    scopes.insert(
        String::from("Issue10"),
        Arc::new(RwLock::new(Scope::empty(
            String::from("Issue10"),
            String::from("Issue10"),
            TypeInfo::new("Issue10".to_string(), vec![]),
        ))),
    );

    // clock
    scopes
        .get("Issue10")
        .unwrap()
        .write()
        .unwrap()
        .variables
        .push(Variable::new(
            TraceValue::RefTraceName("clock".to_string()),
            String::from("clock"),
            TypeInfo::new("IO[Clock]".to_string(), Vec::new()),
            VariableKind::Ground(1),
        ));

    // reset
    scopes
        .get("Issue10")
        .unwrap()
        .write()
        .unwrap()
        .variables
        .push(Variable::new(
            TraceValue::RefTraceName("reset".to_string()),
            String::from("reset"),
            TypeInfo::new("IO[Bool]".to_string(), Vec::new()),
            VariableKind::Ground(1),
        ));

    // vec1
    scopes
        .get("Issue10")
        .unwrap()
        .write()
        .unwrap()
        .variables
        .push(create_vec1());
    // vec2
    scopes
        .get("Issue10")
        .unwrap()
        .write()
        .unwrap()
        .variables
        .push(create_vec2());
    // vec3
    scopes
        .get("Issue10")
        .unwrap()
        .write()
        .unwrap()
        .variables
        .push(create_vec3());
    // vec4
    scopes
        .get("Issue10")
        .unwrap()
        .write()
        .unwrap()
        .variables
        .push(create_vec4());
    // vec5
    scopes
        .get("Issue10")
        .unwrap()
        .write()
        .unwrap()
        .variables
        .push(create_vec5());
    TyVcd { scopes }
}

fn create_vec1() -> Variable {
    let vec1_0_x = TraceValue::RefTraceName("vec1_0_x_0".to_string());
    let vec1_0 = TraceValue::RefTraceValues(vec![vec1_0_x.clone()]);
    let vec1_1_x = TraceValue::RefTraceName("vec1_1_x_0".to_string());
    let vec1_1 = TraceValue::RefTraceValues(vec![vec1_1_x.clone()]);
    let vec1 = TraceValue::RefTraceValues(vec![vec1_0.clone(), vec1_1.clone()]);

    Variable::new(
        vec1,
        String::from("vec1"),
        TypeInfo::new("IO[VecType1[2]]".to_string(), vec![]),
        VariableKind::Vector {
            fields: vec![
                // vec1[0]
                Variable::new(
                    vec1_0,
                    String::from("0"),
                    TypeInfo::new("IO[VecType1[2]]".to_string(), vec![]),
                    VariableKind::Struct {
                        fields: vec![Variable::new(
                            vec1_0_x,
                            String::from("x"),
                            TypeInfo::new("IO[UInt<1>]".to_string(), vec![]),
                            VariableKind::Ground(1),
                        )],
                    },
                ),
                // vec1[1]
                Variable::new(
                    vec1_1,
                    String::from("1"),
                    TypeInfo::new("IO[VecType1[2]]".to_string(), vec![]),
                    VariableKind::Struct {
                        fields: vec![Variable::new(
                            vec1_1_x,
                            String::from("x"),
                            TypeInfo::new("IO[UInt<1>]".to_string(), vec![]),
                            VariableKind::Ground(1),
                        )],
                    },
                ),
            ],
        },
    )
}

fn create_vec2() -> Variable {
    let vec2_0_x = TraceValue::RefTraceName("vec2_0_x_0".to_string());
    let vec2_0_y = TraceValue::RefTraceName("vec2_0_y_0".to_string());
    let vec2_0 = TraceValue::RefTraceValues(vec![vec2_0_x.clone(), vec2_0_y.clone()]);
    let vec2_1_x = TraceValue::RefTraceName("vec2_1_x_0".to_string());
    let vec2_1_y = TraceValue::RefTraceName("vec2_1_y_0".to_string());
    let vec2_1 = TraceValue::RefTraceValues(vec![vec2_1_x.clone(), vec2_1_y.clone()]);
    let vec2 = TraceValue::RefTraceValues(vec![vec2_0.clone(), vec2_1.clone()]);

    Variable::new(
        vec2,
        String::from("vec2"),
        TypeInfo::new("IO[VecType2[2]]".to_string(), vec![]),
        VariableKind::Vector {
            fields: vec![
                // vec2[0]
                Variable::new(
                    vec2_0,
                    String::from("0"),
                    TypeInfo::new("IO[VecType2[2]]".to_string(), vec![]),
                    VariableKind::Struct {
                        fields: vec![
                            Variable::new(
                                vec2_0_x,
                                String::from("x"),
                                TypeInfo::new("IO[UInt<1>]".to_string(), vec![]),
                                VariableKind::Ground(1),
                            ),
                            Variable::new(
                                vec2_0_y,
                                String::from("y"),
                                TypeInfo::new("IO[UInt<2>]".to_string(), vec![]),
                                VariableKind::Ground(2),
                            ),
                        ],
                    },
                ),
                // vec1[1]
                Variable::new(
                    vec2_1,
                    String::from("1"),
                    TypeInfo::new("IO[VecType2[2]]".to_string(), vec![]),
                    VariableKind::Struct {
                        fields: vec![
                            Variable::new(
                                vec2_1_x,
                                String::from("x"),
                                TypeInfo::new("IO[UInt<1>]".to_string(), vec![]),
                                VariableKind::Ground(1),
                            ),
                            Variable::new(
                                vec2_1_y,
                                String::from("y"),
                                TypeInfo::new("IO[UInt<2>]".to_string(), vec![]),
                                VariableKind::Ground(2),
                            ),
                        ],
                    },
                ),
            ],
        },
    )
}

fn create_vec3() -> Variable {
    let vec3_0_x = TraceValue::RefTraceName("vec3_0_x_0".to_string());
    let vec3_0_y = TraceValue::RefTraceName("vec3_0_y_0".to_string());
    let vec3_0_z = TraceValue::RefTraceName("vec3_0_z_0".to_string());
    let vec3_0 =
        TraceValue::RefTraceValues(vec![vec3_0_x.clone(), vec3_0_y.clone(), vec3_0_z.clone()]);
    let vec3_1_x = TraceValue::RefTraceName("vec3_1_x_0".to_string());
    let vec3_1_y = TraceValue::RefTraceName("vec3_1_y_0".to_string());
    let vec3_1_z = TraceValue::RefTraceName("vec3_1_z_0".to_string());
    let vec3_1 =
        TraceValue::RefTraceValues(vec![vec3_1_x.clone(), vec3_1_y.clone(), vec3_1_z.clone()]);
    let vec3 = TraceValue::RefTraceValues(vec![vec3_0.clone(), vec3_1.clone()]);

    Variable::new(
        vec3,
        String::from("vec3"),
        TypeInfo::new("IO[VecType3[2]]".to_string(), vec![]),
        VariableKind::Vector {
            fields: vec![
                // vec3[0]
                Variable::new(
                    vec3_0,
                    String::from("0"),
                    TypeInfo::new("IO[VecType3[2]]".to_string(), vec![]),
                    VariableKind::Struct {
                        fields: vec![
                            Variable::new(
                                vec3_0_x,
                                String::from("x"),
                                TypeInfo::new("IO[UInt<1>]".to_string(), vec![]),
                                VariableKind::Ground(1),
                            ),
                            Variable::new(
                                vec3_0_y,
                                String::from("y"),
                                TypeInfo::new("IO[UInt<2>]".to_string(), vec![]),
                                VariableKind::Ground(2),
                            ),
                            Variable::new(
                                vec3_0_z,
                                String::from("z"),
                                TypeInfo::new("IO[UInt<3>]".to_string(), vec![]),
                                VariableKind::Ground(3),
                            ),
                        ],
                    },
                ),
                // vec1[1]
                Variable::new(
                    vec3_1,
                    String::from("1"),
                    TypeInfo::new("IO[VecType3[2]]".to_string(), vec![]),
                    VariableKind::Struct {
                        fields: vec![
                            Variable::new(
                                vec3_1_x,
                                String::from("x"),
                                TypeInfo::new("IO[UInt<1>]".to_string(), vec![]),
                                VariableKind::Ground(1),
                            ),
                            Variable::new(
                                vec3_1_y,
                                String::from("y"),
                                TypeInfo::new("IO[UInt<2>]".to_string(), vec![]),
                                VariableKind::Ground(2),
                            ),
                            Variable::new(
                                vec3_1_z,
                                String::from("z"),
                                TypeInfo::new("IO[UInt<3>]".to_string(), vec![]),
                                VariableKind::Ground(3),
                            ),
                        ],
                    },
                ),
            ],
        },
    )
}

fn create_vec4() -> Variable {
    let vec4_0_x = TraceValue::RefTraceName("vec4_0_x_0".to_string());
    let vec4_0_y = TraceValue::RefTraceName("vec4_0_y_0".to_string());
    let vec4_0_z = TraceValue::RefTraceName("vec4_0_z_0".to_string());
    let vec4_0_w = TraceValue::RefTraceName("vec4_0_w_0".to_string());
    let vec4_0 = TraceValue::RefTraceValues(vec![
        vec4_0_x.clone(),
        vec4_0_y.clone(),
        vec4_0_z.clone(),
        vec4_0_w.clone(),
    ]);
    let vec4_1_x = TraceValue::RefTraceName("vec4_1_x_0".to_string());
    let vec4_1_y = TraceValue::RefTraceName("vec4_1_y_0".to_string());
    let vec4_1_z = TraceValue::RefTraceName("vec4_1_z_0".to_string());
    let vec4_1_w = TraceValue::RefTraceName("vec4_1_w_0".to_string());
    let vec4_1 = TraceValue::RefTraceValues(vec![
        vec4_1_x.clone(),
        vec4_1_y.clone(),
        vec4_1_z.clone(),
        vec4_1_w.clone(),
    ]);
    let vec4 = TraceValue::RefTraceValues(vec![vec4_0.clone(), vec4_1.clone()]);

    Variable::new(
        vec4,
        String::from("vec4"),
        TypeInfo::new("IO[VecType4[2]]".to_string(), vec![]),
        VariableKind::Vector {
            fields: vec![
                // vec4[0]
                Variable::new(
                    vec4_0,
                    String::from("0"),
                    TypeInfo::new("IO[VecType4[2]]".to_string(), vec![]),
                    VariableKind::Struct {
                        fields: vec![
                            Variable::new(
                                vec4_0_x,
                                String::from("x"),
                                TypeInfo::new("IO[UInt<1>]".to_string(), vec![]),
                                VariableKind::Ground(1),
                            ),
                            Variable::new(
                                vec4_0_y,
                                String::from("y"),
                                TypeInfo::new("IO[UInt<2>]".to_string(), vec![]),
                                VariableKind::Ground(2),
                            ),
                            Variable::new(
                                vec4_0_z,
                                String::from("z"),
                                TypeInfo::new("IO[UInt<3>]".to_string(), vec![]),
                                VariableKind::Ground(3),
                            ),
                            Variable::new(
                                vec4_0_w,
                                String::from("w"),
                                TypeInfo::new("IO[UInt<4>]".to_string(), vec![]),
                                VariableKind::Ground(4),
                            ),
                        ],
                    },
                ),
                // vec1[1]
                Variable::new(
                    vec4_1,
                    String::from("1"),
                    TypeInfo::new("IO[VecType4[2]]".to_string(), vec![]),
                    VariableKind::Struct {
                        fields: vec![
                            Variable::new(
                                vec4_1_x,
                                String::from("x"),
                                TypeInfo::new("IO[UInt<1>]".to_string(), vec![]),
                                VariableKind::Ground(1),
                            ),
                            Variable::new(
                                vec4_1_y,
                                String::from("y"),
                                TypeInfo::new("IO[UInt<2>]".to_string(), vec![]),
                                VariableKind::Ground(2),
                            ),
                            Variable::new(
                                vec4_1_z,
                                String::from("z"),
                                TypeInfo::new("IO[UInt<3>]".to_string(), vec![]),
                                VariableKind::Ground(3),
                            ),
                            Variable::new(
                                vec4_1_w,
                                String::from("w"),
                                TypeInfo::new("IO[UInt<4>]".to_string(), vec![]),
                                VariableKind::Ground(4),
                            ),
                        ],
                    },
                ),
            ],
        },
    )
}

fn create_vec5() -> Variable {
    let vec5_0_x = TraceValue::RefTraceName("vec5_0_x_0".to_string());
    let vec5_0_y_a_0 = TraceValue::RefTraceName("vec5_0_y_a_0".to_string());
    let vec5_0_y_b_0 = TraceValue::RefTraceName("vec5_0_y_b_0".to_string());
    let vec5_0_y_c_0 = TraceValue::RefTraceName("vec5_0_y_c_0".to_string());
    let vec5_0_y = TraceValue::RefTraceValues(vec![
        vec5_0_y_a_0.clone(),
        vec5_0_y_b_0.clone(),
        vec5_0_y_c_0.clone(),
    ]);
    let vec5_0 = TraceValue::RefTraceValues(vec![vec5_0_x.clone(), vec5_0_y.clone()]);

    let vec5_1_x = TraceValue::RefTraceName("vec5_1_x_0".to_string());
    let vec5_1_y_a_0 = TraceValue::RefTraceName("vec5_1_y_a_0".to_string());
    let vec5_1_y_b_0 = TraceValue::RefTraceName("vec5_1_y_b_0".to_string());
    let vec5_1_y_c_0 = TraceValue::RefTraceName("vec5_1_y_c_0".to_string());
    let vec5_1_y = TraceValue::RefTraceValues(vec![
        vec5_1_y_a_0.clone(),
        vec5_1_y_b_0.clone(),
        vec5_1_y_c_0.clone(),
    ]);
    let vec5_1 = TraceValue::RefTraceValues(vec![vec5_1_x.clone(), vec5_1_y.clone()]);

    let vec5 = TraceValue::RefTraceValues(vec![vec5_0.clone(), vec5_1.clone()]);

    Variable::new(
        vec5,
        String::from("vec5"),
        TypeInfo::new("IO[VecType5[2]]".to_string(), vec![]),
        VariableKind::Vector {
            fields: vec![
                // vec5[0]
                Variable::new(
                    vec5_0,
                    String::from("0"),
                    TypeInfo::new("IO[VecType5[2]]".to_string(), vec![]),
                    VariableKind::Struct {
                        fields: vec![
                            Variable::new(
                                vec5_0_x,
                                String::from("x"),
                                TypeInfo::new("IO[UInt<1>]".to_string(), vec![]),
                                VariableKind::Ground(1),
                            ),
                            Variable::new(
                                vec5_0_y,
                                String::from("y"),
                                TypeInfo::new("IO[AnonymousBundle]".to_string(), vec![]),
                                VariableKind::Struct {
                                    fields: vec![
                                        Variable::new(
                                            vec5_0_y_a_0,
                                            String::from("a"),
                                            TypeInfo::new("IO[UInt<32>]".to_string(), vec![]),
                                            VariableKind::Ground(32),
                                        ),
                                        Variable::new(
                                            vec5_0_y_b_0,
                                            String::from("b"),
                                            TypeInfo::new("IO[UInt<2>]".to_string(), vec![]),
                                            VariableKind::Ground(2),
                                        ),
                                        Variable::new(
                                            vec5_0_y_c_0,
                                            String::from("c"),
                                            TypeInfo::new("IO[UInt<2>]".to_string(), vec![]),
                                            VariableKind::Ground(2),
                                        ),
                                    ],
                                },
                            ),
                        ],
                    },
                ),
                // vec1[1]
                Variable::new(
                    vec5_1,
                    String::from("1"),
                    TypeInfo::new("IO[VecType5[2]]".to_string(), vec![]),
                    VariableKind::Struct {
                        fields: vec![
                            Variable::new(
                                vec5_1_x,
                                String::from("x"),
                                TypeInfo::new("IO[UInt<1>]".to_string(), vec![]),
                                VariableKind::Ground(1),
                            ),
                            Variable::new(
                                vec5_1_y,
                                String::from("y"),
                                TypeInfo::new("IO[AnonymousBundle]".to_string(), vec![]),
                                VariableKind::Struct {
                                    fields: vec![
                                        Variable::new(
                                            vec5_1_y_a_0,
                                            String::from("a"),
                                            TypeInfo::new("IO[UInt<32>]".to_string(), vec![]),
                                            VariableKind::Ground(32),
                                        ),
                                        Variable::new(
                                            vec5_1_y_b_0,
                                            String::from("b"),
                                            TypeInfo::new("IO[UInt<2>]".to_string(), vec![]),
                                            VariableKind::Ground(2),
                                        ),
                                        Variable::new(
                                            vec5_1_y_c_0,
                                            String::from("c"),
                                            TypeInfo::new("IO[UInt<2>]".to_string(), vec![]),
                                            VariableKind::Ground(2),
                                        ),
                                    ],
                                },
                            ),
                        ],
                    },
                ),
            ],
        },
    )
}
