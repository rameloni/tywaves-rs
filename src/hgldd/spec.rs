use core::fmt;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The HGLDD file root struct.
#[derive(Serialize, Deserialize, Clone)]
pub struct Hgldd {
    #[serde(rename = "HGLDD")]
    pub header: Header,
    #[serde(rename = "objects")]
    pub objects: Vec<Object>,
}

/// The header of an HGLDD file.
/// It contains generic information to access the source files and the version of the HGLDD.
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone)]
pub struct Header {
    /// The version of the HGLDD file
    #[serde(rename = "version")]
    pub version: String,
    /// The list of files referring to the HGLDD file
    #[serde(rename = "file_info")]
    pub file_info: Vec<String>,
    /// The index of the HDL file (i.e. `sv` file) in `file_info`
    #[serde(rename = "hdl_file_index")]
    pub hdl_file_index: Option<u32>,
}

/// An object in the HGLDD file. It can be a module or a struct (please see [ObjectKind]).
/// It represent a only a "type", the actual value is stored in the variables.
/// For example a struct will contain `port_vars` with the actual values of the struct.
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone)]
pub struct Object {
    /// The kind of the object
    #[serde(rename = "kind")]
    pub kind: ObjectKind,

    /// The HGL name (variable name in the source language, i.e. chisel)
    #[serde(rename = "obj_name")]
    pub hgl_obj_name: String,
    /// The HDL name of the module, if the object is an [ObjectKind::Module]
    /// (i.e. variable name the target language, i.e. verilog)
    #[serde(rename = "module_name")]
    pub hdl_module_name: Option<String>,
    /// Tells if the object is an external module imported from a different file in the source language.
    /// It should be a module implemented in the target language. Thus, the source language information
    /// may not be available
    #[serde(rename = "isExtModule")]
    pub is_ext_module: Option<u8>,

    /// The location of the object in the HGL file
    #[serde(rename = "hgl_loc")]
    pub hgl_loc: Option<Location>,
    /// The location of the object in the HDL file
    #[serde(rename = "hdl_loc")]
    pub hdl_loc: Option<Location>,

    /// Variables of the object (a module or a struct)
    #[serde(rename = "port_vars")]
    pub port_vars: Vec<Variable>,

    /// Children instances of the module
    #[serde(rename = "children")]
    pub children: Option<Vec<Instance>>,

    /// Optional source language type information for the object
    #[serde(rename = "source_lang_type_info")]
    pub source_lang_type_info: Option<SourceLangType>,
}

impl Object {
    /// Create a new object with the given name and kind.
    pub fn new(hgl_obj_name: String, kind: ObjectKind) -> Self {
        Self {
            kind,
            hgl_obj_name,
            hdl_module_name: None,
            is_ext_module: None,
            hgl_loc: None,
            hdl_loc: None,
            port_vars: Vec::new(),
            children: None,
            source_lang_type_info: None,
        }
    }

    pub fn with_children(mut self, children: Vec<Instance>) -> Self {
        self.children = Some(children);
        self
    }
}

/// Supported HGLDD object kinds.
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub enum ObjectKind {
    #[serde(rename = "module")]
    Module,
    #[serde(rename = "struct")]
    Struct,
}

/// A variable in the HGLDD file.
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone)]
pub struct Variable {
    /// The source language name of the variable
    #[serde(rename = "var_name")]
    pub var_name: String,

    #[serde(rename = "hgl_loc")]
    pub hgl_loc: Option<Location>,
    #[serde(rename = "hdl_loc")]
    pub hdl_loc: Option<Location>,

    /// The value of the variable
    #[serde(rename = "value")]
    pub value_expr: Option<Expression>,

    /// The type name in the target language (i.e. `logic` in verilog)
    #[serde(rename = "type_name")]
    pub type_name: Option<TypeName>,
    /// The dimensions range of a vector variable (i.e. `logic [7:0] x`)
    #[serde(rename = "packed_range")]
    pub packed_range: Option<PackedRange>,
    /// The dimensions range of a vector variable (i.e. `logic [7:0] x [1:0][3:0]`)
    #[serde(rename = "unpacked_range")]
    pub unpacked_range: Option<UnpackedRange>,

    /// The source lang type information
    #[serde(rename = "source_lang_type_info")]
    pub source_lang_type_info: Option<SourceLangType>,
}

/// The source language type information.
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone)]
pub struct SourceLangType {
    /// The source language type name
    #[serde(rename = "type_name")]
    pub type_name: Option<String>,
    /// Constructor parameters
    #[serde(rename = "params")]
    pub params: Option<Vec<ConstructorParams>>,
}

/// The constructor parameters in a source language type
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone)]
pub struct ConstructorParams {
    /// The name of the parameter
    #[serde(rename = "name")]
    pub name: String,
    /// The type of the parameter
    #[serde(rename = "type", alias = "typeName")]
    pub tpe: String,
    /// The value of the parameter used (not always available)
    #[serde(rename = "value")]
    pub value: Option<String>,
}

/// An instance of a module.
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Instance {
    /// The name of the instance in the source language (HGL). Use it as an identifier.
    #[serde(rename = "name")]
    pub name_id: String,
    /// The name of the instance in the target language (HDL if the instance name is different from the source language)
    #[serde(rename = "hdl_obj_name")]
    pub hdl_obj_name: Option<String>,
    /// The name of the module type of this instance in the source language (HGL)
    #[serde(rename = "obj_name")]
    pub hgl_module_name: Option<String>,
    /// The name of the module type of this instance in the target language (HDL)
    #[serde(rename = "module_name")]
    pub hdl_module_name: Option<String>,

    #[serde(rename = "hgl_loc")]
    pub hgl_loc: Option<Location>,
    #[serde(rename = "hdl_loc")]
    pub hdl_loc: Option<Location>,

    /// The variables of the instance
    #[serde(rename = "port_vars")]
    pub port_vars: Option<Vec<Variable>>,
    /// The children instances of the instance
    #[serde(rename = "children")]
    pub children: Option<Vec<Instance>>,
}

impl Instance {
    /// Create a new instance with the given name.
    pub fn new(
        hgl_name: String,
        hdl_obj_name: String,
        hgl_type_obj_name: String,
        hdl_type_obj_name: String,
    ) -> Self {
        Self {
            name_id: hgl_name,
            hdl_obj_name: Some(hdl_obj_name),
            hgl_module_name: Some(hgl_type_obj_name),
            hdl_module_name: Some(hdl_type_obj_name),
            // hdl_obj_name: None,
            // obj_name: None,
            // module_name: None,
            hgl_loc: None,
            hdl_loc: None,
            port_vars: None,
            children: None,
        }
    }
}

/// An emitted expression in HGLDD. An expression can refer to a signal in the target language,
/// to a constant value or to an operator (for example for aggregates).
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum Expression {
    // TODO: add more expression types
    /// A signal name: usually the variable name in the target language
    #[serde(rename = "sig_name")]
    SigName(String),
    /// A bit vector representing the value of the expression. The value contained is
    /// a binary constant value which can be converted into an integer.
    #[serde(rename = "bit_vector")]
    BitVector(String),
    /// An integer number
    #[serde(rename = "integer_num")]
    IntegerNum(u32),
    /// An operator with its operands. The operands are other expressions.
    #[serde(untagged)]
    Operator {
        #[serde(rename = "opcode")]
        opcode: Opcode,
        #[serde(rename = "operands")]
        operands: Vec<Expression>,
    },
}

/// The dimensions of a variable in the target language (i.e. verilog).
/// ```verilog
///                 // Dimensions
/// logic [7:0] x;  // PackedRange(7, 0)
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PackedRange(pub u32, pub u32);

impl From<&PackedRange> for u32 {
    fn from(range: &PackedRange) -> Self {
        range.0 - range.1 + 1
    }
}

impl From<&PackedRange> for u128 {
    fn from(range: &PackedRange) -> Self {
        u32::from(range).into()
    }
}

/// The dimensionality of a variable in the target language (i.e. verilog).
/// ```verilog
///                            // Dimensionality
/// logic       x [1:0][3:0];  // UnpackedRange([1, 0, 3, 0])
/// logic [7:0] y [0:0][2:0];  // UnpackedRange([0, 0, 2, 0])
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnpackedRange(pub Vec<u32>);

/// The type name of a variable in HGLDD.
#[derive(Serialize, Deserialize, Clone)]
pub enum TypeName {
    /// A verilog like logic type
    #[serde(rename = "logic")]
    Logic,
    /// A single bit type
    #[serde(rename = "bit")]
    Bit,
    /// A custom type name, when [TypeName::Logic] or [TypeName::Bit] are not enough.
    /// It is usually a pointer to a type defined in [Object].
    ///
    /// # Example
    ///
    /// In the example below the variable `io` is of type `BundleStruct_io`.
    /// So, it "points" to the object `BundleStruct_io`.
    ///
    /// ```json
    /// // ...
    ///  "kind": "struct",
    ///  "obj_name": "BundleStruct_io",
    ///  "port_vars": [
    ///    {
    ///      "hgl_loc": {
    ///        "begin_column": 7,
    ///        "begin_line": 74,
    ///        "end_column": 7,
    ///        "end_line": 74,
    ///        "file": 1
    ///      },
    ///      "packed_range": [
    ///        31,
    ///        0
    ///      ],
    ///      "type_name": "logic",
    ///      "var_name": "a"
    ///    },
    /// // ....
    /// {
    ///   "var_name": "io",
    ///   "hgl_loc": {
    ///     "begin_column": 14,
    ///     "begin_line": 75,
    ///     "end_column": 14,
    ///     "end_line": 75,
    ///     "file": 1
    ///   },
    ///   "value": {"opcode":"'{","operands":[{"sig_name":"io_a_0"}]},
    ///   "type_name": "BundleStruct_io"
    /// }
    /// ```
    #[serde(untagged)]
    Custom(String),
}

impl fmt::Display for TypeName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match self {
            TypeName::Logic => "logic",
            TypeName::Bit => "bit",
            TypeName::Custom(name) => name,
        };
        write!(f, "{}", output)
    }
}

/// The location of an object in a file.
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Location {
    /// The index of the file in the [Header::file_info] of the HGLDD file
    #[serde(rename = "file")]
    pub file_idx: u32,
    #[serde(rename = "begin_line")]
    pub begin_line: Option<u32>,
    #[serde(rename = "end_line")]
    pub end_line: Option<u32>,
    #[serde(rename = "begin_column")]
    pub begin_column: Option<u32>,
    #[serde(rename = "end_column")]
    pub end_column: Option<u32>,
}

/// Opcodes for the operators in the HGLDD [Expression].
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum Opcode {
    /// A struct operation. It links target language variable names to source language aggregate variable.
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
