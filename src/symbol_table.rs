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

    pub fn collect_ground_variables(&self) -> Vec<RealType> {
        let mut ground_variables: Vec<RealType> = Vec::new();

        match &self.real_type {
            RealType::Ground { .. } => ground_variables.push(self.real_type.clone()),
            RealType::Vec { .. } => {}
            RealType::Bundle { fields, .. } => {
                for field in fields {
                    ground_variables.append(&mut field.collect_ground_variables());
                }
            }
            RealType::Unknown => {}
        }
        ground_variables
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
