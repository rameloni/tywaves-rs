use std::fmt::Display;

use crate::variable_finder::VariableFinder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// A scope in the state
pub struct Scope {
    /// Name of the scope
    pub name: String,
    /// List of variables in the scope
    pub child_variables: Vec<Variable>,
    /// The list of scopes in this scope
    pub child_scopes: Vec<Scope>,
}

impl VariableFinder for Scope {
    fn find_variable<'a>(&'a self, subpath: &Vec<String>) -> Option<&'a Variable> {
        // The head of the subpath should be the name of the scope
        let subpath = if let Some(head) = subpath.first() {
            if head != &self.name {
                return None;
            }
            subpath[1..].to_vec()
        } else {
            return None;
        };

        // If the path has one element or it is empty search for the ground variable in the children
        if subpath.len() <= 1 {
            // Return the first variable if it is found
            self.child_variables
                .iter()
                .find_map(|var| var.find_ground_variable(&subpath[0]))
        } else {
            // Explore the scopes recursively
            self.child_scopes
                .iter()
                .find_map(|scope| scope.find_variable(&subpath))
        }
    }
    fn find_parent_variable<'a>(&'a self, subpath: &Vec<String>) -> Option<&'a Variable> {
        // The head of the subpath should be the name of the scope
        let subpath = if let Some(head) = subpath.first() {
            if head != &self.name {
                return None;
            }
            subpath[1..].to_vec()
        } else {
            return None;
        };

        // If the path has one element or it is empty search for the ground variable in the children
        if subpath.len() <= 1 {
            // Return the first variable if it is found
            self.child_variables
                .iter()
                .find_map(|var| var.find_parent_variable(&subpath))
        } else {
            // Explore the scopes recursively
            self.child_scopes
                .iter()
                .find_map(|scope| scope.find_parent_variable(&subpath))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Variable {
    /// Name of the variable
    pub name: String,
    /// The type of the variable (high level type)
    pub type_name: String,
    pub hw_type: HwType,
    pub real_type: RealType,
}

impl VariableFinder for Variable {
    fn find_variable<'a>(&'a self, subpath: &Vec<String>) -> Option<&'a Variable> {
        if subpath.len() > 1 {
            return None;
        } else if let Some(vcd_name) = subpath.first() {
            self.find_ground_variable(vcd_name)
        } else {
            return None;
        }
    }
    /// This will find the first variable that will contain the ground vcd_name
    fn find_parent_variable<'a>(&'a self, subpath: &Vec<String>) -> Option<&'a Variable> {
        // If it can find a ground variable then this is a parent
        if let Some(_) = self.find_variable(subpath) {
            Some(self)
        } else {
            None
        }
    }
}

impl Variable {
    pub fn get_full_name(&self) -> String {
        format!("{}: {}", self.type_name, self.name)
    }

    pub fn new(name: &str, type_name: &str, hw_type: HwType, real_type: RealType) -> Variable {
        Variable {
            name: name.to_string(),
            type_name: type_name.to_string(),
            hw_type,
            real_type,
        }
    }

    fn find_ground_variable<'a>(&'a self, vcd_name: &String) -> Option<&'a Variable> {
        match &self.real_type {
            RealType::Ground { vcd_name: name, .. } => {
                if name == vcd_name {
                    return Some(self);
                }
            }
            RealType::Vec { fields, .. } => {
                for field in fields {
                    if let Some(v) = field.find_ground_variable(vcd_name) {
                        return Some(v);
                    }
                }
            }
            RealType::Bundle {
                fields,
                vcd_name: bundle_vcd_name,
            } => {
                if let Some(bundle_vcd_name) = bundle_vcd_name {
                    if bundle_vcd_name == vcd_name {
                        return Some(self);
                    }
                }

                for field in fields {
                    if let Some(v) = field.find_ground_variable(vcd_name) {
                        return Some(v);
                    }
                }
            }
            RealType::Unknown => {}
        }
        None
    }

    pub fn create_val_repr(&self, raw_val_vcd: &str) -> String {
        let size = self.real_type.find_width() as usize;
        println!(
            "Size of {} {}: \n{}, raw_val_vcd size: {}",
            self.name,
            self.real_type,
            size,
            raw_val_vcd.len()
        );
        if raw_val_vcd.len() < size {
            return String::from("---");
        }
        let value = match &self.real_type {
            RealType::Ground { width, .. } => match width {
                0 | 1 => raw_val_vcd.to_string(),
                _ => format!("{} {}: {raw_val_vcd}", &self.type_name, &self.name),
            },
            RealType::Vec { .. } => todo!("Vec type not implemented"),
            RealType::Bundle { fields, .. } => {
                // Encode the fields recursively {x, {y, z}}
                let mut value = format!("{} {}: {{", &self.type_name, &self.name);

                let mut start_idx = 0;
                for field in fields {
                    let end_idx = start_idx + field.real_type.find_width() as usize;
                    println!("start_idx: {}, end_idx: {}", start_idx, end_idx);
                    value.push_str(&field.create_val_repr(&raw_val_vcd[start_idx..end_idx]));
                    value.push_str(", ");
                    start_idx = end_idx;
                }
                value.pop();
                value.pop();
                value.push_str("}");
                value
                // todo!("Bundle type not implemented")
            }
            RealType::Unknown => todo!("Unknown type not implemented"),
        };
        format!("{value}")
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HwType {
    Wire,
    Reg,
    Port { direction: Direction },
    Mem,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    Input,
    Output,
    Inout,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RealType {
    Ground {
        width: u64,
        vcd_name: String,
    },
    Vec {
        size: u64,
        fields: Vec<Variable>,
    },
    Bundle {
        fields: Vec<Variable>,
        vcd_name: Option<String>,
    },
    Unknown,
}
impl Display for RealType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RealType::Ground { .. } => write!(f, "Ground"),
            RealType::Vec { size, fields } => {
                let fields = fields
                    .iter()
                    .map(|f| format!("{}", f.real_type))
                    .collect::<Vec<_>>();
                write!(f, "Vec[{}]: [{:?}]", size, fields)
            }
            RealType::Bundle { fields, .. } => {
                let fields = fields
                    .iter()
                    .map(|f| format!("{}", f.real_type))
                    .collect::<Vec<_>>();
                write!(f, "Bundle: [{:?}]", fields)
            }
            RealType::Unknown => write!(f, "Unknown"),
        }
    }
}

impl RealType {
    pub fn find_width(&self) -> u128 {
        match self {
            RealType::Ground { width, .. } => *width as u128,
            RealType::Vec { size, fields } => {
                if let Some(field) = fields.first() {
                    *size as u128 * field.real_type.find_width()
                } else {
                    0
                }
            }
            RealType::Bundle { fields, .. } => {
                fields.iter().map(|f| f.real_type.find_width()).sum()
            }
            RealType::Unknown => 0,
        }
    }
}

#[cfg(test)]
mod test {
    use std::env::var;

    use super::*;

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
        let variable = bundle.find_ground_variable(&"io_a".to_string()).unwrap();
        assert_eq!(
            variable.real_type,
            RealType::Ground {
                width: 1,
                vcd_name: "io_a".to_string()
            }
        );
        println!("Variable: {:#?}", variable);
        let variable_none = bundle.find_ground_variable(&"a0aw2".to_string());
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
        let variable = bundle.find_ground_variable(&"io_a".to_string()).unwrap();
        assert_eq!(
            variable.real_type,
            RealType::Ground {
                width: 1,
                vcd_name: "io_a".to_string()
            }
        );
        println!("Variable: {:#?}", variable);
        let variable_none = bundle.find_ground_variable(&"unknown".to_string());
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
        let variable = ground.find_ground_variable(&"a0aw".to_string()).unwrap();
        assert_eq!(
            variable.real_type,
            RealType::Ground {
                width: 1,
                vcd_name: "a0aw".to_string()
            }
        );

        let variable_none = ground.find_ground_variable(&"a0aw2".to_string());
        assert_eq!(variable_none, None);
    }

    #[test]
    #[rustfmt::skip]
    fn test_find_variable_scope() {
        let path = vec!["top".to_string(), "a0aw2".to_string()];
        let var = Variable::new(
            "a", "MyVariable",
            HwType::Reg,
            RealType::Ground { width: 1, vcd_name: "a0aw".to_string(), },
        );

        let var2 = Variable::new(
            "b", "MyVariable",
            HwType::Reg,
            RealType::Ground { width: 1, vcd_name: "a0aw2".to_string(), },
        );

        let subscope = Scope { name: "dut".to_string(), child_variables: vec![var2.clone(), var], child_scopes: vec![], };

        let scope = Scope { name: "top".to_string(), child_variables: vec![var2], child_scopes: vec![subscope], };

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
                RealType::Ground { width: 1, vcd_name: vcd_name.to_string(), },
            )
        }).collect();
        let subsubscope = Scope { name: "subdut".to_string(), child_variables: var_list.clone(), child_scopes: vec![], };
        let subscope = Scope { name: "dut".to_string(), child_variables: var_list.clone(), child_scopes: vec![subsubscope], };
        let scope = Scope { name: "TOP".to_string(), child_variables: var_list.clone(), child_scopes: vec![subscope], };
        
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
}
