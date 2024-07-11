use crate::tyvcd::spec::*;
use crate::HashMap;
use crate::TraceValue;
use std::sync::{Arc, RwLock};

pub fn create_vecs_multi_dim() -> TyVcd {
    let mut scopes = HashMap::new();
    // Main scope
    scopes.insert(
        String::from("Issue16"),
        Arc::new(RwLock::new(Scope::empty(
            String::from("Issue16"),
            String::from("Issue16"),
            TypeInfo::new("Issue16".to_string(), vec![]),
            &vec![],
        ))),
    );

    // clock
    scopes
        .get("Issue16")
        .unwrap()
        .write()
        .unwrap()
        .variables
        .push(
            Variable::new(
                TraceValue::RefTraceName("clock".to_string()),
                String::from("clock"),
                TypeInfo::new("IO[Clock]".to_string(), Vec::new()),
                VariableKind::Ground(1),
            )
            .as_top(),
        );

    // reset
    scopes
        .get("Issue16")
        .unwrap()
        .write()
        .unwrap()
        .variables
        .push(
            Variable::new(
                TraceValue::RefTraceName("reset".to_string()),
                String::from("reset"),
                TypeInfo::new("IO[Bool]".to_string(), Vec::new()),
                VariableKind::Ground(1),
            )
            .as_top(),
        );

    // vec1d
    scopes
        .get("Issue16")
        .unwrap()
        .write()
        .unwrap()
        .variables
        .push(create_vec1d().as_top());
    // vec2d
    scopes
        .get("Issue16")
        .unwrap()
        .write()
        .unwrap()
        .variables
        .push(create_vec2d().as_top());
    // vec4
    scopes
        .get("Issue16")
        .unwrap()
        .write()
        .unwrap()
        .variables
        .push(create_vec4().as_top());

    TyVcd { scopes }
}

fn create_vec1d() -> Variable {
    let vec1d_0 = TraceValue::RefTraceName("vec1d_0_0".to_string());
    let vec1d_1 = TraceValue::RefTraceName("vec1d_1_0".to_string());
    let vec1d = TraceValue::RefTraceValues(vec![vec1d_0.clone(), vec1d_1.clone()]);

    Variable::new(
        vec1d,
        String::from("vec1d"),
        TypeInfo::new("IO[UInt<1>[2]]".to_string(), vec![]),
        VariableKind::Vector {
            fields: vec![
                // vec1d[0]
                Variable::new(
                    vec1d_0,
                    String::from("0"),
                    TypeInfo::new("IO[UInt<1>[2]]".to_string(), vec![]),
                    VariableKind::Ground(1),
                ),
                // vec1d[1]
                Variable::new(
                    vec1d_1,
                    String::from("1"),
                    TypeInfo::new("IO[UInt<1>[2]]".to_string(), vec![]),
                    VariableKind::Ground(1),
                ),
            ],
        },
    )
}

fn create_vec2d() -> Variable {
    let vec2d_0_0_0 = TraceValue::RefTraceName("vec2d_0_0_0".to_string());
    let vec2d_1_0_0 = TraceValue::RefTraceName("vec2d_1_0_0".to_string());
    let vec2d_0_0 = TraceValue::RefTraceValues(vec![vec2d_0_0_0.clone()]);
    let vec2d_1_0 = TraceValue::RefTraceValues(vec![vec2d_1_0_0.clone()]);
    let vec2d = TraceValue::RefTraceValues(vec![vec2d_0_0.clone(), vec2d_1_0.clone()]);

    Variable::new(
        vec2d,
        String::from("vec2d"),
        TypeInfo::new("IO[UInt<2>[1][2]]".to_string(), vec![]),
        VariableKind::Vector {
            fields: vec![
                // vec2d[0]
                Variable::new(
                    vec2d_0_0,
                    String::from("0"),
                    TypeInfo::new("IO[UInt<2>[1][2]]".to_string(), vec![]),
                    VariableKind::Vector {
                        fields: vec![Variable::new(
                            vec2d_0_0_0,
                            String::from("0"),
                            TypeInfo::new("IO[UInt<2>[1][2]]".to_string(), vec![]),
                            VariableKind::Ground(2),
                        )],
                    },
                ),
                // vec1d[1]
                Variable::new(
                    vec2d_1_0,
                    String::from("1"),
                    TypeInfo::new("IO[UInt<2>[1][2]]".to_string(), vec![]),
                    VariableKind::Vector {
                        fields: vec![Variable::new(
                            vec2d_1_0_0,
                            String::from("0"),
                            TypeInfo::new("IO[UInt<2>[1][2]]".to_string(), vec![]),
                            VariableKind::Ground(2),
                        )],
                    },
                ),
            ],
        },
    )
}

fn create_vec4() -> Variable {
    // vec4d[0][0][0][0]
    let vec4d_0_0_0_0_x = TraceValue::RefTraceName("vec4d_0_0_0_0_x_0".to_string());
    let vec4d_0_0_0_0_y = TraceValue::RefTraceName("vec4d_0_0_0_0_y_0".to_string());
    let vec4d_0_0_0_0_z = TraceValue::RefTraceName("vec4d_0_0_0_0_z_0".to_string());
    let vec4d_0_0_0_0_w = TraceValue::RefTraceName("vec4d_0_0_0_0_w_0".to_string());
    let vec4d_0_0_0_0 = TraceValue::RefTraceValues(vec![
        vec4d_0_0_0_0_x.clone(),
        vec4d_0_0_0_0_y.clone(),
        vec4d_0_0_0_0_z.clone(),
        vec4d_0_0_0_0_w.clone(),
    ]);
    // Vec4d[0][0][1][0]
    let vec4d_0_0_1_0_x = TraceValue::RefTraceName("vec4d_0_0_1_0_x_0".to_string());
    let vec4d_0_0_1_0_y = TraceValue::RefTraceName("vec4d_0_0_1_0_y_0".to_string());
    let vec4d_0_0_1_0_z = TraceValue::RefTraceName("vec4d_0_0_1_0_z_0".to_string());
    let vec4d_0_0_1_0_w = TraceValue::RefTraceName("vec4d_0_0_1_0_w_0".to_string());
    let vec4d_0_0_1_0 = TraceValue::RefTraceValues(vec![
        vec4d_0_0_1_0_x.clone(),
        vec4d_0_0_1_0_y.clone(),
        vec4d_0_0_1_0_z.clone(),
        vec4d_0_0_1_0_w.clone(),
    ]);
    // Vec4d[0][0][2][0]
    let vec4d_0_0_2_0_x = TraceValue::RefTraceName("vec4d_0_0_2_0_x_0".to_string());
    let vec4d_0_0_2_0_y = TraceValue::RefTraceName("vec4d_0_0_2_0_y_0".to_string());
    let vec4d_0_0_2_0_z = TraceValue::RefTraceName("vec4d_0_0_2_0_z_0".to_string());
    let vec4d_0_0_2_0_w = TraceValue::RefTraceName("vec4d_0_0_2_0_w_0".to_string());
    let vec4d_0_0_2_0 = TraceValue::RefTraceValues(vec![
        vec4d_0_0_2_0_x.clone(),
        vec4d_0_0_2_0_y.clone(),
        vec4d_0_0_2_0_z.clone(),
        vec4d_0_0_2_0_w.clone(),
    ]);

    // Vec4d[1][0][0][0]
    let vec4d_1_0_0_0_x = TraceValue::RefTraceName("vec4d_1_0_0_0_x_0".to_string());
    let vec4d_1_0_0_0_y = TraceValue::RefTraceName("vec4d_1_0_0_0_y_0".to_string());
    let vec4d_1_0_0_0_z = TraceValue::RefTraceName("vec4d_1_0_0_0_z_0".to_string());
    let vec4d_1_0_0_0_w = TraceValue::RefTraceName("vec4d_1_0_0_0_w_0".to_string());
    let vec4d_1_0_0_0 = TraceValue::RefTraceValues(vec![
        vec4d_1_0_0_0_x.clone(),
        vec4d_1_0_0_0_y.clone(),
        vec4d_1_0_0_0_z.clone(),
        vec4d_1_0_0_0_w.clone(),
    ]);
    // Vec4d[1][0][1][0]
    let vec4d_1_0_1_0_x = TraceValue::RefTraceName("vec4d_1_0_1_0_x_0".to_string());
    let vec4d_1_0_1_0_y = TraceValue::RefTraceName("vec4d_1_0_1_0_y_0".to_string());
    let vec4d_1_0_1_0_z = TraceValue::RefTraceName("vec4d_1_0_1_0_z_0".to_string());
    let vec4d_1_0_1_0_w = TraceValue::RefTraceName("vec4d_1_0_1_0_w_0".to_string());
    let vec4d_1_0_1_0 = TraceValue::RefTraceValues(vec![
        vec4d_1_0_1_0_x.clone(),
        vec4d_1_0_1_0_y.clone(),
        vec4d_1_0_1_0_z.clone(),
        vec4d_1_0_1_0_w.clone(),
    ]);
    // Vec4d[1][0][2][0]
    let vec4d_1_0_2_0_x = TraceValue::RefTraceName("vec4d_1_0_2_0_x_0".to_string());
    let vec4d_1_0_2_0_y = TraceValue::RefTraceName("vec4d_1_0_2_0_y_0".to_string());
    let vec4d_1_0_2_0_z = TraceValue::RefTraceName("vec4d_1_0_2_0_z_0".to_string());
    let vec4d_1_0_2_0_w = TraceValue::RefTraceName("vec4d_1_0_2_0_w_0".to_string());
    let vec4d_1_0_2_0 = TraceValue::RefTraceValues(vec![
        vec4d_1_0_2_0_x.clone(),
        vec4d_1_0_2_0_y.clone(),
        vec4d_1_0_2_0_z.clone(),
        vec4d_1_0_2_0_w.clone(),
    ]);

    let vec4d_0_0_0 = TraceValue::RefTraceValues(vec![vec4d_0_0_0_0.clone()]); // vec4d[0][0][0]
    let vec4d_0_0_1 = TraceValue::RefTraceValues(vec![vec4d_0_0_1_0.clone()]); // vec4d[0][0][1]
    let vec4d_0_0_2 = TraceValue::RefTraceValues(vec![vec4d_0_0_2_0.clone()]); // vec4d[0][0][2]
    let vec4d_1_0_0 = TraceValue::RefTraceValues(vec![vec4d_1_0_0_0.clone()]); // vec4d[1][0][0]
    let vec4d_1_0_1 = TraceValue::RefTraceValues(vec![vec4d_1_0_1_0.clone()]); // vec4d[1][0][1]
    let vec4d_1_0_2 = TraceValue::RefTraceValues(vec![vec4d_1_0_2_0.clone()]); // vec4d[1][0][2]

    let vec4d_0_0 = TraceValue::RefTraceValues(vec![
        vec4d_0_0_0.clone(),
        vec4d_0_0_1.clone(),
        vec4d_0_0_2.clone(),
    ]); // vec4d[0][0]
    let vec4d_1_0 = TraceValue::RefTraceValues(vec![
        vec4d_1_0_0.clone(),
        vec4d_1_0_1.clone(),
        vec4d_1_0_2.clone(),
    ]); // vec4d[1][0]

    let vec4d_0 = TraceValue::RefTraceValues(vec![vec4d_0_0.clone()]); // vec4d[0]
    let vec4d_1 = TraceValue::RefTraceValues(vec![vec4d_1_0.clone()]); // vec4d[1]

    let vec4d = TraceValue::RefTraceValues(vec![vec4d_0.clone(), vec4d_1.clone()]);

    Variable::new(
        vec4d,
        String::from("vec4d"),
        TypeInfo::new("IO[VecType4[1][3][1][2]]".to_string(), vec![]),
        VariableKind::Vector {
            fields: vec![
                // vec4d[0]
                Variable::new(
                    vec4d_0,
                    String::from("0"),
                    TypeInfo::new("IO[VecType4[1][3][1][2]]".to_string(), vec![]),
                    VariableKind::Vector {
                        fields: vec![Variable::new(
                            vec4d_0_0,
                            String::from("0"),
                            TypeInfo::new("IO[VecType4[1][3][1][2]]".to_string(), vec![]),
                            VariableKind::Vector {
                                fields: vec![
                                    Variable::new(
                                        vec4d_0_0_0,
                                        String::from("0"),
                                        TypeInfo::new(
                                            "IO[VecType4[1][3][1][2]]".to_string(),
                                            vec![],
                                        ),
                                        VariableKind::Vector {
                                            fields: vec![Variable::new(
                                                vec4d_0_0_0_0,
                                                String::from("0"),
                                                TypeInfo::new(
                                                    "IO[VecType4[1][3][1][2]]".to_string(),
                                                    vec![],
                                                ),
                                                VariableKind::Struct {
                                                    fields: vec![
                                                        Variable::new(
                                                            vec4d_0_0_0_0_x,
                                                            String::from("x"),
                                                            TypeInfo::new(
                                                                "IO[UInt<1>]".to_string(),
                                                                vec![],
                                                            ),
                                                            VariableKind::Ground(1),
                                                        ),
                                                        Variable::new(
                                                            vec4d_0_0_0_0_y,
                                                            String::from("y"),
                                                            TypeInfo::new(
                                                                "IO[UInt<2>]".to_string(),
                                                                vec![],
                                                            ),
                                                            VariableKind::Ground(2),
                                                        ),
                                                        Variable::new(
                                                            vec4d_0_0_0_0_z,
                                                            String::from("z"),
                                                            TypeInfo::new(
                                                                "IO[UInt<3>]".to_string(),
                                                                vec![],
                                                            ),
                                                            VariableKind::Ground(3),
                                                        ),
                                                        Variable::new(
                                                            vec4d_0_0_0_0_w,
                                                            String::from("w"),
                                                            TypeInfo::new(
                                                                "IO[UInt<4>]".to_string(),
                                                                vec![],
                                                            ),
                                                            VariableKind::Ground(4),
                                                        ),
                                                    ],
                                                },
                                            )],
                                        },
                                    ),
                                    Variable::new(
                                        vec4d_0_0_1,
                                        String::from("1"),
                                        TypeInfo::new(
                                            "IO[VecType4[1][3][1][2]]".to_string(),
                                            vec![],
                                        ),
                                        VariableKind::Vector {
                                            fields: vec![Variable::new(
                                                vec4d_0_0_1_0,
                                                String::from("0"),
                                                TypeInfo::new(
                                                    "IO[VecType4[1][3][1][2]]".to_string(),
                                                    vec![],
                                                ),
                                                VariableKind::Struct {
                                                    fields: vec![
                                                        Variable::new(
                                                            vec4d_0_0_1_0_x,
                                                            String::from("x"),
                                                            TypeInfo::new(
                                                                "IO[UInt<1>]".to_string(),
                                                                vec![],
                                                            ),
                                                            VariableKind::Ground(1),
                                                        ),
                                                        Variable::new(
                                                            vec4d_0_0_1_0_y,
                                                            String::from("y"),
                                                            TypeInfo::new(
                                                                "IO[UInt<2>]".to_string(),
                                                                vec![],
                                                            ),
                                                            VariableKind::Ground(2),
                                                        ),
                                                        Variable::new(
                                                            vec4d_0_0_1_0_z,
                                                            String::from("z"),
                                                            TypeInfo::new(
                                                                "IO[UInt<3>]".to_string(),
                                                                vec![],
                                                            ),
                                                            VariableKind::Ground(3),
                                                        ),
                                                        Variable::new(
                                                            vec4d_0_0_1_0_w,
                                                            String::from("w"),
                                                            TypeInfo::new(
                                                                "IO[UInt<4>]".to_string(),
                                                                vec![],
                                                            ),
                                                            VariableKind::Ground(4),
                                                        ),
                                                    ],
                                                },
                                            )],
                                        },
                                    ),
                                    Variable::new(
                                        vec4d_0_0_2,
                                        String::from("2"),
                                        TypeInfo::new(
                                            "IO[VecType4[1][3][1][2]]".to_string(),
                                            vec![],
                                        ),
                                        VariableKind::Vector {
                                            fields: vec![Variable::new(
                                                vec4d_0_0_2_0,
                                                String::from("0"),
                                                TypeInfo::new(
                                                    "IO[VecType4[1][3][1][2]]".to_string(),
                                                    vec![],
                                                ),
                                                VariableKind::Struct {
                                                    fields: vec![
                                                        Variable::new(
                                                            vec4d_0_0_2_0_x,
                                                            String::from("x"),
                                                            TypeInfo::new(
                                                                "IO[UInt<1>]".to_string(),
                                                                vec![],
                                                            ),
                                                            VariableKind::Ground(1),
                                                        ),
                                                        Variable::new(
                                                            vec4d_0_0_2_0_y,
                                                            String::from("y"),
                                                            TypeInfo::new(
                                                                "IO[UInt<2>]".to_string(),
                                                                vec![],
                                                            ),
                                                            VariableKind::Ground(2),
                                                        ),
                                                        Variable::new(
                                                            vec4d_0_0_2_0_z,
                                                            String::from("z"),
                                                            TypeInfo::new(
                                                                "IO[UInt<3>]".to_string(),
                                                                vec![],
                                                            ),
                                                            VariableKind::Ground(3),
                                                        ),
                                                        Variable::new(
                                                            vec4d_0_0_2_0_w,
                                                            String::from("w"),
                                                            TypeInfo::new(
                                                                "IO[UInt<4>]".to_string(),
                                                                vec![],
                                                            ),
                                                            VariableKind::Ground(4),
                                                        ),
                                                    ],
                                                },
                                            )],
                                        },
                                    ),
                                ],
                            },
                        )],
                    },
                ),
                // vec1d[1]
                Variable::new(
                    vec4d_1,
                    String::from("1"),
                    TypeInfo::new("IO[VecType4[1][3][1][2]]".to_string(), vec![]),
                    VariableKind::Vector {
                        fields: vec![Variable::new(
                            vec4d_1_0,
                            String::from("0"),
                            TypeInfo::new("IO[VecType4[1][3][1][2]]".to_string(), vec![]),
                            VariableKind::Vector {
                                fields: vec![
                                    Variable::new(
                                        vec4d_1_0_0,
                                        String::from("0"),
                                        TypeInfo::new(
                                            "IO[VecType4[1][3][1][2]]".to_string(),
                                            vec![],
                                        ),
                                        VariableKind::Vector {
                                            fields: vec![Variable::new(
                                                vec4d_1_0_0_0,
                                                String::from("0"),
                                                TypeInfo::new(
                                                    "IO[VecType4[1][3][1][2]]".to_string(),
                                                    vec![],
                                                ),
                                                VariableKind::Struct {
                                                    fields: vec![
                                                        Variable::new(
                                                            vec4d_1_0_0_0_x,
                                                            String::from("x"),
                                                            TypeInfo::new(
                                                                "IO[UInt<1>]".to_string(),
                                                                vec![],
                                                            ),
                                                            VariableKind::Ground(1),
                                                        ),
                                                        Variable::new(
                                                            vec4d_1_0_0_0_y,
                                                            String::from("y"),
                                                            TypeInfo::new(
                                                                "IO[UInt<2>]".to_string(),
                                                                vec![],
                                                            ),
                                                            VariableKind::Ground(2),
                                                        ),
                                                        Variable::new(
                                                            vec4d_1_0_0_0_z,
                                                            String::from("z"),
                                                            TypeInfo::new(
                                                                "IO[UInt<3>]".to_string(),
                                                                vec![],
                                                            ),
                                                            VariableKind::Ground(3),
                                                        ),
                                                        Variable::new(
                                                            vec4d_1_0_0_0_w,
                                                            String::from("w"),
                                                            TypeInfo::new(
                                                                "IO[UInt<4>]".to_string(),
                                                                vec![],
                                                            ),
                                                            VariableKind::Ground(4),
                                                        ),
                                                    ],
                                                },
                                            )],
                                        },
                                    ),
                                    Variable::new(
                                        vec4d_1_0_1,
                                        String::from("1"),
                                        TypeInfo::new(
                                            "IO[VecType4[1][3][1][2]]".to_string(),
                                            vec![],
                                        ),
                                        VariableKind::Vector {
                                            fields: vec![Variable::new(
                                                vec4d_1_0_1_0,
                                                String::from("0"),
                                                TypeInfo::new(
                                                    "IO[VecType4[1][3][1][2]]".to_string(),
                                                    vec![],
                                                ),
                                                VariableKind::Struct {
                                                    fields: vec![
                                                        Variable::new(
                                                            vec4d_1_0_1_0_x,
                                                            String::from("x"),
                                                            TypeInfo::new(
                                                                "IO[UInt<1>]".to_string(),
                                                                vec![],
                                                            ),
                                                            VariableKind::Ground(1),
                                                        ),
                                                        Variable::new(
                                                            vec4d_1_0_1_0_y,
                                                            String::from("y"),
                                                            TypeInfo::new(
                                                                "IO[UInt<2>]".to_string(),
                                                                vec![],
                                                            ),
                                                            VariableKind::Ground(2),
                                                        ),
                                                        Variable::new(
                                                            vec4d_1_0_1_0_z,
                                                            String::from("z"),
                                                            TypeInfo::new(
                                                                "IO[UInt<3>]".to_string(),
                                                                vec![],
                                                            ),
                                                            VariableKind::Ground(3),
                                                        ),
                                                        Variable::new(
                                                            vec4d_1_0_1_0_w,
                                                            String::from("w"),
                                                            TypeInfo::new(
                                                                "IO[UInt<4>]".to_string(),
                                                                vec![],
                                                            ),
                                                            VariableKind::Ground(4),
                                                        ),
                                                    ],
                                                },
                                            )],
                                        },
                                    ),
                                    Variable::new(
                                        vec4d_1_0_2,
                                        String::from("2"),
                                        TypeInfo::new(
                                            "IO[VecType4[1][3][1][2]]".to_string(),
                                            vec![],
                                        ),
                                        VariableKind::Vector {
                                            fields: vec![Variable::new(
                                                vec4d_1_0_2_0,
                                                String::from("0"),
                                                TypeInfo::new(
                                                    "IO[VecType4[1][3][1][2]]".to_string(),
                                                    vec![],
                                                ),
                                                VariableKind::Struct {
                                                    fields: vec![
                                                        Variable::new(
                                                            vec4d_1_0_2_0_x,
                                                            String::from("x"),
                                                            TypeInfo::new(
                                                                "IO[UInt<1>]".to_string(),
                                                                vec![],
                                                            ),
                                                            VariableKind::Ground(1),
                                                        ),
                                                        Variable::new(
                                                            vec4d_1_0_2_0_y,
                                                            String::from("y"),
                                                            TypeInfo::new(
                                                                "IO[UInt<2>]".to_string(),
                                                                vec![],
                                                            ),
                                                            VariableKind::Ground(2),
                                                        ),
                                                        Variable::new(
                                                            vec4d_1_0_2_0_z,
                                                            String::from("z"),
                                                            TypeInfo::new(
                                                                "IO[UInt<3>]".to_string(),
                                                                vec![],
                                                            ),
                                                            VariableKind::Ground(3),
                                                        ),
                                                        Variable::new(
                                                            vec4d_1_0_2_0_w,
                                                            String::from("w"),
                                                            TypeInfo::new(
                                                                "IO[UInt<4>]".to_string(),
                                                                vec![],
                                                            ),
                                                            VariableKind::Ground(4),
                                                        ),
                                                    ],
                                                },
                                            )],
                                        },
                                    ),
                                ],
                            },
                        )],
                    },
                ),
            ],
        },
    )
}
