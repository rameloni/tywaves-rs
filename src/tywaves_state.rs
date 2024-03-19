use crate::{symbol_table::*, variable_finder::VariableFinder};
use eyre::{eyre, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// The state of the Tywaves translator
pub struct TywaveState {
    /// List of the scopes
    pub scopes: Vec<Scope>,
}

impl VariableFinder for TywaveState {
    fn find_variable(&self, subpath: &Vec<String>) -> Option<&Variable> {
        // Find the variable in the top scope
        self.scopes
            .iter()
            .find_map(|scope| scope.find_variable(subpath))
    }

    fn find_parent_variable(&self, subpath: &Vec<String>) -> Option<&Variable> {
        // Find the variable in the top scope
        self.scopes
            .iter()
            .find_map(|scope| scope.find_parent_variable(subpath))
    }
}

#[cfg(test)]
mod test {

    use std::path;

    use super::*;
    #[rustfmt::skip]
    fn create_io_sample() -> Variable {
        Variable::new(
            "io",
            "AnonymousBundle",
            HwType::Wire,
            RealType::Bundle {
                fields: vec![
                    Variable::new(
                        "a", "Bool",
                        HwType::Port { direction: Direction::Input, },
                        RealType::Ground { width: 1, vcd_name: "io_a".to_string(), },
                    ),
                    Variable::new(
                        "b", "Bool",
                        HwType::Port { direction: Direction::Input, },
                        RealType::Ground { width: 1, vcd_name: "io_b".to_string(), },
                    ),
                    Variable::new(
                        "out", "UInt",
                        HwType::Port { direction: Direction::Output, },
                        RealType::Ground { width: 1, vcd_name: "io_out".to_string(), },
                    ),
                ],
            },
        )
    }

    #[rustfmt::skip]
    fn sample_state() -> TywaveState {
        let io = create_io_sample();
        let svsim_childs = vec![
            Variable::new(
                "clock", "Clock",
                HwType::Port { direction: Direction::Input, },
                RealType::Ground { width: 1, vcd_name: "clock".to_string(), },
            ),
            Variable::new(
                "reset", "Bool",
                HwType::Port { direction: Direction::Input, },
                RealType::Ground { width: 1, vcd_name: "reset".to_string(), },
            ),
            io,
        ];

        let mut dut_childs = svsim_childs.clone();

        dut_childs.push(Variable::new(
            "wire", "SInt",
            HwType::Wire,
            RealType::Ground { width: 1, vcd_name: "wire_0".to_string(), },
        ));

        TywaveState {
            scopes: vec![Scope {
                name: "TOP".to_string(),
                child_variables: Vec::new(),
                child_scopes: vec![Scope {
                    name: "svsimTestbench".to_string(),
                    child_variables: svsim_childs,
                    child_scopes: vec![Scope { name: "dut".to_string(), child_variables: dut_childs, child_scopes: Vec::new(), }],
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
            (Some(String::from("clock")), vec!["TOP".to_string(), "svsimTestbench".to_string(), "dut".to_string(), "clock".to_string()])                        
        ];
        let state = sample_state();
        println!("State: {:#?}", state);
        for (expected, path) in path_vecs {
            let variable = state.find_variable(&path);
            println!("Path: {:?}, Variable: {:?}", path, variable);
            let x = variable.map(|v| match &v.real_type {
                RealType::Ground { vcd_name, width: _ } => vcd_name.clone(),
                _ => "unknown".to_string(),
            });
            println!("Found: {:?}", x);
            assert_eq!(x, expected);
        }
    }
}
