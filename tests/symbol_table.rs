use tywaves::symbol_table::*;
use tywaves::variable_finder::VariableFinder;

fn create_bundle() -> Variable {
    Variable::new(
        "io",
        "AnonymousBundle",
        HwType::Wire,
        RealType::Bundle {
            vcd_name: Some("io".to_string()),
            fields: vec![
                Variable::new(
                    "a",
                    "Bool",
                    HwType::Port {
                        direction: Direction::Input,
                    },
                    RealType::Ground {
                        width: 1,
                        vcd_name: "io_a".to_string(),
                    },
                ),
                Variable::new(
                    "b",
                    "Bool",
                    HwType::Port {
                        direction: Direction::Input,
                    },
                    RealType::Ground {
                        width: 1,
                        vcd_name: "io_b".to_string(),
                    },
                ),
                Variable::new(
                    "out",
                    "UInt",
                    HwType::Port {
                        direction: Direction::Output,
                    },
                    RealType::Ground {
                        width: 1,
                        vcd_name: "io_out".to_string(),
                    },
                ),
            ],
        },
    )
}

#[test]
fn test_find_variable_bundle() {
    let bundle = create_bundle();
    let variable = bundle.find_variable(&vec!["io_a".to_string()]).unwrap();
    assert_eq!(
        variable.real_type,
        RealType::Ground {
            width: 1,
            vcd_name: "io_a".to_string(),
        }
    );
    println!("Variable: {:#?}", variable);
    let variable_none = bundle.find_variable(&vec!["a0aw2".to_string()]);
    assert_eq!(variable_none, None);
}

#[test]
fn test_find_variable_nested_bundle() {
    let mut bundle = create_bundle();
    let subbundle = create_bundle();
    bundle.real_type = RealType::Bundle {
        vcd_name: None,
        fields: vec![
            Variable::new(
                "a0aw2",
                "Bool",
                HwType::Port {
                    direction: Direction::Input,
                },
                RealType::Ground {
                    width: 1,
                    vcd_name: "a0aw2".to_string(),
                },
            ),
            subbundle,
        ],
    };
    let variable = bundle.find_variable(&vec!["io_a".to_string()]).unwrap();
    assert_eq!(
        variable.real_type,
        RealType::Ground {
            width: 1,
            vcd_name: "io_a".to_string(),
        }
    );
    println!("Variable: {:#?}", variable);
    let variable_none = bundle.find_variable(&vec!["unknown".to_string()]);
    assert_eq!(variable_none, None);
}

#[test]
fn test_find_variable_ground() {
    let ground = Variable::new(
        "aaa",
        "MyVariable",
        HwType::Reg,
        RealType::Ground {
            width: 1,
            vcd_name: "a0aw".to_string(),
        },
    );
    let variable = ground.find_variable(&vec!["a0aw".to_string()]).unwrap();
    assert_eq!(
        variable.real_type,
        RealType::Ground {
            width: 1,
            vcd_name: "a0aw".to_string(),
        }
    );

    let variable_none = ground.find_variable(&vec!["a0aw2".to_string()]);
    assert_eq!(variable_none, None);
}

#[test]
#[rustfmt::skip]
fn test_find_variable_scope() {
    let path = vec!["top".to_string(), "a0aw2".to_string()];
    let var = Variable::new(
        "a", "MyVariable",
        HwType::Reg,
        RealType::Ground { width: 1, vcd_name: "a0aw".to_string() },
    );

    let var2 = Variable::new(
        "b", "MyVariable",
        HwType::Reg,
        RealType::Ground { width: 1, vcd_name: "a0aw2".to_string() },
    );

    let subscope = Scope { name: "dut".to_string(), child_variables: vec![var2.clone(), var], child_scopes: vec![] };

    let scope = Scope { name: "top".to_string(), child_variables: vec![var2], child_scopes: vec![subscope] };

    let variable = scope.find_variable(&path).unwrap();

    assert_eq!(
        variable.real_type,
        RealType::Ground { width: 1, vcd_name: "a0aw2".to_string() }
    );
    let wrong_path = vec!["a0aw2".to_string()];
    let variable_none = scope.find_variable(&wrong_path);
    assert_eq!(variable_none, None);
}

#[test]
#[rustfmt::skip]
fn test_find_variable_nested_scope() {
    let paths = vec![
        vec!["TOP".to_string(), "dut".to_string(), "subdut".to_string(), "aw2".to_string()],
        vec!["TOP".to_string(), "dut".to_string(), "aw2".to_string()],
        vec!["TOP".to_string(), "aw2".to_string()],
    ];
    let var_list = vec![("a", "aw0"), ("b", "aw1"), ("c", "aw2")];

    let var_list: Vec<Variable> = var_list.iter().map(|(name, vcd_name)| {
        Variable::new(
            name, "MyVariable", HwType::Reg,
            RealType::Ground { width: 1, vcd_name: vcd_name.to_string() },
        )
    }).collect();
    let subsubscope = Scope { name: "subdut".to_string(), child_variables: var_list.clone(), child_scopes: vec![] };
    let subscope = Scope { name: "dut".to_string(), child_variables: var_list.clone(), child_scopes: vec![subsubscope] };
    let scope = Scope { name: "TOP".to_string(), child_variables: var_list.clone(), child_scopes: vec![subscope] };

    for path in &paths {
        let variable = scope.find_variable(&path).unwrap();

        assert_eq!(
            variable.real_type,
            RealType::Ground { width: 1, vcd_name: "aw2".to_string() }
        );
    }

    let wrong_path = vec!["a0aw2".to_string()];
    let variable_none = scope.find_variable(&wrong_path);
    assert_eq!(variable_none, None);
}

#[test]
#[rustfmt::skip]
fn test_collect_ground_variables() {
    // Create a new variable
    let mut bundle = create_bundle();
    println!("Bundle: {:#?}", bundle);

    let nested_bundle = Variable::new("nested", "MyNestedBundle", HwType::Wire, RealType::Bundle {
        vcd_name: Some("nested".to_string()),
        fields: vec![
            Variable::new("a", "Bool", HwType::Port { direction: Direction::Input }, RealType::Ground { width: 10, vcd_name: "nested_a".to_string() }),
            Variable::new("b", "Bool", HwType::Port { direction: Direction::Input }, RealType::Ground { width: 23, vcd_name: "nested_b".to_string() }),
            Variable::new("out", "UInt", HwType::Port { direction: Direction::Output }, RealType::Ground { width: 56, vcd_name: "nested_out".to_string() }),
        ],
    });

    bundle.real_type = RealType::Bundle {
        vcd_name: Some("io".to_string()),
        fields: vec![
            Variable::new("a", "Bool", HwType::Port { direction: Direction::Input }, RealType::Ground { width: 1, vcd_name: "io_a".to_string() }),
            Variable::new("out", "UInt", HwType::Port { direction: Direction::Output }, RealType::Ground { width: 1, vcd_name: "io_out".to_string() }),
            Variable::new("b", "Bool", HwType::Port { direction: Direction::Input }, RealType::Ground { width: 1, vcd_name: "io_b".to_string() }),
            nested_bundle,
        ],
    };

    // Collect all ground variables
    let ground_variables = bundle.collect_ground_variables();

    // Check if the ground variables are correct
    let expect = vec![RealType::Ground { width: 1, vcd_name: "io_a".to_string() },
                      RealType::Ground { width: 1, vcd_name: "io_out".to_string() },
                      RealType::Ground { width: 1, vcd_name: "io_b".to_string() },
                      RealType::Ground { width: 10, vcd_name: "nested_a".to_string() },
                      RealType::Ground { width: 23, vcd_name: "nested_b".to_string() },
                      RealType::Ground { width: 56, vcd_name: "nested_out".to_string() }];
    assert_eq!(ground_variables, expect);
}
