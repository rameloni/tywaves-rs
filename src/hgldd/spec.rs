use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The HGLDD file root struct
#[derive(Serialize, Deserialize)]
pub struct Hgldd {
    #[serde(rename = "HGLDD")]
    pub hgldd: Header,
    pub objects: Vec<Object>,
}

/// The header of an hgldd file
#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct Header {
    /// The version of the HGLDD file
    pub version: String,
    /// The list of files referring to the HGLDD file
    pub file_info: Vec<String>,
    /// The index of the HDL file (i.e. sv file) in file_info
    pub hdl_file_index: Option<u32>,
}

/// An object in the HGLDD file. It can be a module or a struct.
#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Object {
    /// The kind of the object
    pub kind: ObjectKind,

    /// The HGL name (variable name in the source language, i.e. chisel)
    pub obj_name: String,
    /// The HDL name (i.e. variable name the target language, i.e. verilog)
    pub module_name: Option<String>,
    #[serde(rename = "isExtModule")]
    pub is_ext_module: Option<u8>,

    /// The location of the object in the HGL file
    pub hgl_loc: Option<Location>,
    /// The location of the object in the HDL file
    pub hdl_loc: Option<Location>,

    /// variables of the module
    pub port_vars: Vec<Variable>,

    /// Children instances of the module
    pub children: Option<Vec<Instance>>,
}

/// Supported HGLDD object kinds
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObjectKind {
    Module,
    Struct,
}

/// A variable in the HGLDD file
#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Variable {
    /// The source language name of the variable
    var_name: String,

    hgl_loc: Option<Location>,
    hdl_loc: Option<Location>,

    /// The value of the variable
    value: Option<Expression>,

    /// The type name in the target language (i.e. logic in verilog)
    type_name: Option<TypeName>,
    /// The dimensions range of the variable (i.e. [7:0])
    packed_range: Option<Dims>,
    /// The dimensions range of a variable (it can be more than one dimension, i.e. [7:0][3:0])
    unpacked_range: Option<Dims>,
}

/// An instance of a module
#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Instance {
    /// The name of the instance in the source language (HGL)
    pub name: String,
    /// The name of the instance in the target language (HDL if the instance name is different from the source language)
    pub hdl_obj_name: Option<String>,
    /// The name of the module type of this instance in the source language (HGL)
    pub obj_name: Option<String>,
    /// The name of the module type of this instance in the target language (HDL)
    pub module_name: Option<String>,

    pub hgl_loc: Option<Location>,
    pub hdl_loc: Option<Location>,

    /// The variables of the instance
    pub port_vars: Option<Vec<Variable>>,
    /// The children instances of the instance
    pub children: Option<Vec<Instance>>,
}

/// An emitted expression in HGLDD. An expression can refer to a signal in the target language,
/// to a constant value or to an operator (for example for aggregates).
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Expression {
    // TODO: add more expression types
    /// A signal name: usually the variable name in the target language
    SigName(String),
    /// A bit vector representing the value of the expression. The value contained is
    /// a binary constant value which can be converted into an integer.
    BitVector(String),
    /// An integer number
    IntegerNum(u32),
    /// An operator with its operands. The operands are other expressions.
    #[serde(untagged)]
    Operator {
        opcode: Opcode,
        operands: Vec<Expression>,
    },
}

/// The dimensions of a variable
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case", untagged)]
pub enum Dims {
    PackedRange(u32, u32),
    UnpackedRange(Vec<u32>),
}

/// The type name of a variable
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TypeName {
    Logic,
    Bit,
    /// A custom type name, when [TypeName::Logic] or [TypeName::Bit] are not enough
    #[serde(untagged)]
    Custom(String),
}

/// The location of an object in a file
#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Location {
    /// The index of the file in the [Header::file_info] of the HGLDD file
    pub file: u32,
    pub begin_line: Option<u32>,
    pub end_line: Option<u32>,
    pub begin_column: Option<u32>,
    pub end_column: Option<u32>,
}

/// Opcodes for the operators in the HGLDD [Expression]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Opcode {
    #[serde(rename = "'{")]
    Struct,
    #[serde(rename = "^")]
    UnaryOrXor,
    #[serde(rename = "&")]
    And,
    #[serde(rename = "|")]
    Or,
    // #[serde(rename = "^")]
    // Xor,
    #[serde(rename = "+")]
    Add,
    #[serde(rename = "-")]
    Sub,
    #[serde(rename = "*")]
    Mul,
    #[serde(rename = "/")]
    Div,
    #[serde(rename = "%")]
    Mod,
    #[serde(rename = "<<")]
    ShiftLeft,
    #[serde(rename = ">>")]
    ShiftRight,
    #[serde(rename = ">>>")]
    ShiftRightSigned,
    #[serde(rename = "==")]
    Eq,
    #[serde(rename = "!=")]
    NotEq,
    #[serde(rename = "===")]
    CEq,
    #[serde(rename = "!==")]
    CNotEq,
    #[serde(rename = "==?")]
    WEq,
    #[serde(rename = "!=?")]
    WNotEq,
    #[serde(rename = "<")]
    LessThan,
    #[serde(rename = ">")]
    GreaterThan,
    #[serde(rename = "<=")]
    LessEq,
    #[serde(rename = ">=")]
    GreaterEq,
    #[serde(rename = "{}")]
    Concat,
    #[serde(rename = "R{}")]
    Replicate,
    #[serde(rename = "[]")]
    Extract,
    #[serde(rename = "?:")]
    Mux,
}
