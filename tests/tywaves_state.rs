use tywaves_rs::variable_finder::VariableFinder;
use tywaves_rs::{symbol_table::*, tywaves_state::*};

#[rustfmt::skip]
fn create_io_nested_sample() -> Variable {
    Variable::new(
        "io",
        "AnonymousBundle",
        HwType::Wire,
        RealType::Bundle {
            vcd_name: Some("io".to_string()),
            fields: vec![
                Variable::new(
                    "a", "Bool",
                    HwType::Port { direction: Direction::Input },
                    RealType::Ground { width: Width::Single(1), vcd_name: Some("io_a".to_string()), constant: None },
                ),
                Variable::new(
                    "b", "Bool",
                    HwType::Port { direction: Direction::Input },
                    RealType::Ground { width: Width::Single(1), vcd_name: Some("io_b".to_string()), constant: None },
                ),
                Variable::new(
                    "out", "UInt",
                    HwType::Port { direction: Direction::Output },
                    RealType::Ground { width: Width::Single(1), vcd_name: Some("io_out".to_string()), constant: None },
                ),
                Variable::new(
                    "io", "MyNestedBundleType",
                    HwType::Port { direction: Direction::Input },
                    RealType::Bundle {
                        vcd_name: Some("io_io".to_string()),
                        fields: vec![
                            Variable::new(
                                "a", "UInt_2",
                                HwType::Port { direction: Direction::Input },
                                RealType::Ground { width: Width::Single(1), vcd_name: Some("io_io_a".to_string()), constant: None },
                            ),
                            Variable::new(
                                "b", "UInt_2",
                                HwType::Port { direction: Direction::Input },
                                RealType::Ground { width: Width::Single(1), vcd_name: Some("io_io_b".to_string()), constant: None },
                            ),
                            Variable::new(
                                "out", "UInt_2",
                                HwType::Port { direction: Direction::Output },
                                RealType::Ground { width: Width::Single(1), vcd_name: Some("io_io_out".to_string()), constant: None },
                            ),
                        ],
                    },
                ),
            ],
        },
    )
}

#[allow(unused)]
#[rustfmt::skip]
fn create_io_sample() -> Variable {
    Variable::new(
        "io",
        "AnonymousBundle",
        HwType::Wire,
        RealType::Bundle {
            vcd_name: Some("io".to_string()),
            fields: vec![
                Variable::new(
                    "a", "Bool",
                    HwType::Port { direction: Direction::Input },
                    RealType::Ground { width: Width::Single(1), vcd_name: Some("io_a".to_string()), constant: None },
                ),
                Variable::new(
                    "b", "Bool",
                    HwType::Port { direction: Direction::Input },
                    RealType::Ground { width: Width::Single(1), vcd_name: Some("io_b".to_string()), constant: None },
                ),
                Variable::new(
                    "out", "UInt",
                    HwType::Port { direction: Direction::Output },
                    RealType::Ground { width: Width::Single(1), vcd_name: Some("io_out".to_string()), constant: None },
                ),
            ],
        },
    )
}

#[rustfmt::skip]
fn sample_state() -> TywaveState {
    let io = create_io_nested_sample();
    let svsim_childs = vec![
        Variable::new(
            "clock", "Clock",
            HwType::Port { direction: Direction::Input },
            RealType::Ground { width: Width::Single(1), vcd_name: Some("clock".to_string()), constant: None },
        ),
        Variable::new(
            "reset", "Bool",
            HwType::Port { direction: Direction::Input },
            RealType::Ground { width: Width::Single(1), vcd_name: Some("reset".to_string()), constant: None },
        ),
        io,
    ];

    let mut dut_childs = svsim_childs.clone();

    dut_childs.push(Variable::new(
        "wire", "SInt",
        HwType::Wire,
        RealType::Ground { width: Width::Single(1), vcd_name: Some("wire_0".to_string()), constant: None },
    ));

    TywaveState {
        scopes: vec![Scope {
            name: "TOP".to_string(),
            child_variables: Vec::new(),
            child_scopes: vec![Scope {
                name: "svsimTestbench".to_string(),
                child_variables: svsim_childs,
                child_scopes: vec![Scope { name: "dut".to_string(), child_variables: dut_childs, child_scopes: Vec::new() }],
            }],
        }],
    }
}

#[test]
fn dump_json_sample() {
    let json = serde_json::to_string_pretty(&sample_state()).unwrap();
    println!("{}", json);

    // Write a file with the json
    use std::fs::File;
    use std::io::Write;
    let mut file = File::create("trace.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

#[test]
fn test_find_variable_from_scope_path() {
    #[rustfmt::skip]
        let path_vecs = vec![
        (None, vec!["TOP".to_string()]),
        (None, vec!["TOP".to_string(), "svsimTestbench".to_string()]),
        (None, vec!["TOP".to_string(), "svsimTestbench".to_string(), "dut".to_string()]),
        (Some(String::from("clock")), vec!["TOP".to_string(), "svsimTestbench".to_string(), "clock".to_string()]),
        (Some(String::from("io_a")), vec!["TOP".to_string(), "svsimTestbench".to_string(), "io_a".to_string()]),
        (Some(String::from("io_b")), vec!["TOP".to_string(), "svsimTestbench".to_string(), "io_b".to_string()]),
        (Some(String::from("reset")), vec!["TOP".to_string(), "svsimTestbench".to_string(), "reset".to_string()]),
        (Some(String::from("clock")), vec!["TOP".to_string(), "svsimTestbench".to_string(), "clock".to_string()]),
        (Some(String::from("clock")), vec!["TOP".to_string(), "svsimTestbench".to_string(), "dut".to_string(), "clock".to_string()]),
        (Some(String::from("io_a")), vec!["TOP".to_string(), "svsimTestbench".to_string(), "dut".to_string(), "io_a".to_string()]),
        (Some(String::from("io_b")), vec!["TOP".to_string(), "svsimTestbench".to_string(), "dut".to_string(), "io_b".to_string()]),
        (Some(String::from("reset")), vec!["TOP".to_string(), "svsimTestbench".to_string(), "dut".to_string(), "reset".to_string()]),
        (Some(String::from("clock")), vec!["TOP".to_string(), "svsimTestbench".to_string(), "dut".to_string(), "clock".to_string()]),
    ];
    let state = sample_state();
    println!("State: {:#?}", state);
    for (expected, path) in path_vecs {
        let variable = state.find_variable(&path);
        println!("Path: {:?}, Variable: {:?}", path, variable);
        let mut x: Option<String> = None;
        variable.map(|v| match &v.real_type {
            RealType::Ground { vcd_name, .. } => x = vcd_name.clone(),
            _ => x = None,
        });
        println!("Found: {:?}", x);
        assert_eq!(x, expected);
    }
}
