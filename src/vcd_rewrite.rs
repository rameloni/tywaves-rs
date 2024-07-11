use std::collections::vec_deque;
use std::{fs::File, io::*, path::Path};
use vcd::{Command, Header, IdCode, Parser, ReferenceIndex, Value, VarType, Writer};

use crate::tyvcd::spec::{self as tyvcd};
use crate::tyvcd::trace_pointer::TraceGetter;

type TyScope = tyvcd::Scope;
type TyVariable = tyvcd::Variable;
type TyVarKind = tyvcd::VariableKind;

type Result<T> = std::result::Result<T, VcdRewriteError>;

#[derive(Debug)]
pub enum VcdRewriteError {
    /// An IO error occurred
    IoError(std::io::Error),
    /// A VCD parsing error occurred
    VcdError(vcd::ParseError),
    /// Failed to update the value of a variable
    UpdateValueError {
        id_code: IdCode,
        len_diff: (usize, usize),
        values_diff: (String, String),
    },
    /// Another kind of error occurred. For example, while writing
    Other(String),
    /// The specified file name is not a valid vcd
    InvalidFileExtension(String),
}

impl From<std::io::Error> for VcdRewriteError {
    fn from(err: std::io::Error) -> Self {
        VcdRewriteError::IoError(err)
    }
}

pub struct VcdRewriter {
    /// The reader of the original VCD file
    reader: Parser<BufReader<File>>,
    /// The writer of the rewritten VCD file
    writer: Writer<BufWriter<File>>,
    /// The name of the rewritten VCD file
    output_vcd_name: String,
    /// The header of the original VCD file
    vcd_header: Header,

    /// The scopes of the tywaves state
    tywaves_scopes: Vec<TyScope>,

    /// A list of the variables that are going to be written in the rewritten VCD file
    rewrite_variables: Vec<VcdRewriteVariable>,
}

impl VcdRewriter {
    /// Get the full path of the rewritten VCD file
    pub fn get_final_file(&self) -> &String {
        &self.output_vcd_name
    }
    pub fn new(
        vcd_path: &Path,
        tywaves_scopes: Vec<TyScope>,
        out_vcd_name: String,
    ) -> Result<Self> {
        let vcd_file = File::open(vcd_path)?;

        // Raise an error if out_vcd_name does not end with `.vcd`
        if let Some(ext) = Path::new(&out_vcd_name).extension() {
            if ext != "vcd" {
                return Err(VcdRewriteError::InvalidFileExtension(out_vcd_name));
            }
        }
        let output_vcd = File::create(&out_vcd_name)?;

        let reader = Parser::new(BufReader::new(vcd_file));

        // Extract the information from the reader
        let writer = Writer::new(BufWriter::new(output_vcd));

        let vcd_rw = Self {
            reader,
            writer,
            output_vcd_name: out_vcd_name,
            tywaves_scopes,
            vcd_header: Header::default(),
            rewrite_variables: vec![],
        };
        Ok(vcd_rw)
    }

    /// Rewrite the VCD file
    pub fn rewrite(&mut self) -> Result<()> {
        self.rewrite_header()?;

        // Initialize the variables:
        // this will prevent some errors due to some missing variables in the original VCD file
        for v in self.rewrite_variables.iter() {
            self.writer
                .change_vector(v.get_id_code(), v.get_value().iter())?;
        }

        self.rewrite_commands()?;
        Ok(())
    }

    /// Rewrite the header of the VCD file with the tywaves scopes
    fn rewrite_header(&mut self) -> Result<()> {
        self.vcd_header = self.reader.parse_header()?;
        let vcd_header = &self.vcd_header;
        if let Some(date) = &vcd_header.date {
            self.writer.date(date)?;
        }
        if let Some(version) = &vcd_header.version {
            self.writer.version(version)?;
        }
        if let Some((ts, unit)) = vcd_header.timescale {
            self.writer.timescale(ts, unit)?;
        }

        // Parse the scopes of the tywave state
        for scope in &self.tywaves_scopes.clone() {
            self.add_scope_to_header(scope, &[])?;
        }

        // Finish the definitions of the header
        self.writer.enddefinitions()?;

        Ok(())
    }

    /// Add one [TyScope] to the header of the rewritten VCD file.
    /// Search for child [TyVariable] and [TyScope]s and add them to the header.
    fn add_scope_to_header(&mut self, scope: &TyScope, path_scope: &[String]) -> Result<()> {
        // Add the scopes to the header
        let scope_name = scope.get_trace_name().ok_or_else(|| {
            VcdRewriteError::Other(format!("Failed to get the scope from {}", scope.name))
        })?;

        self.writer.add_module(scope_name)?;

        // The scope of the children of the current scope
        let child_path_scope = &[path_scope, &[scope_name.clone()]].concat();

        // Add the variables to the header
        let mut created_vars = vec_deque::VecDeque::new();
        for variable in &scope.variables {
            self.add_variable_to_header(variable, child_path_scope)?;
            created_vars.push_back(variable.name.clone());
        }

        // Add uncovered variables
        // let _scope_path: Vec<&str> = scope.get_trace_path().iter().map(|s| s.as_str()).collect();
        // let mut new_vars = vec_deque::VecDeque::new();
        // if let Some(original_scope) = self.vcd_header.find_scope(_scope_path.as_slice()) {
        //     for item in &original_scope.items {
        //         match item {
        //             vcd::ScopeItem::Var(var) => {
        //                 // Create a variable if this var is not present in the scope.variables
        //                 let start_size = created_vars.len();
        //                 created_vars.retain(|v| v != &var.reference);
        //                 if start_size == created_vars.len() {
        //                     // The var was not created originally
        //                     let new_var = TyVariable::new(
        //                         TraceValue::RefTraceName(var.reference.clone()),
        //                         var.reference.clone(),
        //                         TypeInfo::new(var.var_type.to_string(), vec![]),
        //                         TyVarKind::Ground(var.size as u128),
        //                     );
        //                     new_vars.push_back(new_var);
        //                 }
        //             }
        //             _ => {}
        //         }
        //     }
        // }
        // for var in &new_vars {
        //     // Adding variable to header:
        //     println!("Adding additional to VCD header {:?}", var);
        //     self.add_variable_to_header(var, child_path_scope)?;
        // }
        // drop(new_vars);

        // Add the child scopes to the header
        for child_scope in scope.subscopes.values() {
            let child_scope = child_scope.read().unwrap();
            self.add_scope_to_header(&child_scope, child_path_scope)?;
        }

        // Close the scope
        self.writer.upscope()?;

        Ok(())
    }

    /// Add one [TyVariable] to the header of the rewritten VCD file.
    fn add_variable_to_header(
        &mut self,
        ty_variable: &TyVariable,
        path_scope: &[String],
    ) -> Result<()> {
        // Get the information for the variable
        static VAR_TYPE: VarType = VarType::Wire;

        let reference_name = if let Some(trace_name) = ty_variable.get_trace_name() {
            trace_name
        } else {
            &ty_variable.name
        };

        let width = ty_variable.kind.find_width() as u32;
        let index = (width > 1).then_some(ReferenceIndex::Range(width as i32 - 1, 0));

        // Write the variable to the VCD file
        let new_id = self
            .writer
            .add_var(VAR_TYPE, width, reference_name, index)?;

        // Update the rewrite_variables list
        self.rewrite_variables.push(VcdRewriteVariable::create(
            new_id,
            width,
            ty_variable,
            path_scope,
            &self.vcd_header,
        ));

        Ok(())
    }

    fn rewrite_commands(&mut self) -> Result<()> {
        // Create a lambda function to handle the commands
        let mut update_var = |writer: &mut Writer<BufWriter<File>>,
                              original_id: &IdCode,
                              value: vcd::Vector|
         -> Result<()> {
            // Find a variable in inside the rewrite variables
            for target_var in &mut self.rewrite_variables {
                target_var.update_value(original_id, &value)?;
                writer.change_vector(target_var.get_id_code(), target_var.get_value().iter())?;
            }
            Ok(())
        };

        // while let Some(command) = self.reader.next() {
        for command in self.reader.by_ref() {
            match command? {
                Command::ChangeScalar(original_id, value) => {
                    update_var(&mut self.writer, &original_id, vcd::Vector::from([value]))?
                }
                Command::ChangeVector(original_id, value) => {
                    update_var(&mut self.writer, &original_id, value)?
                }
                Command::Timestamp(ts) => self.writer.timestamp(ts)?,
                Command::ChangeString(_original_id, _value) => { /* TODO: implement ChangeString update */
                }
                Command::ChangeReal(_original_id, _value) => { /* TODO: implement ChangeReal update */
                }
                _ => {} // ignore the other commands
            }
        }

        Ok(())
    }
}

/// Represent a variable of the new VCD file that is going to be written.
///
/// A variable in the rewritten VCD file is a combination of multiple variables from the original VCD file.
/// If a variable is a compound type, it is represented as an ***ordered*** concatenation of the variables that compose it.
#[derive(Debug)]
pub struct VcdRewriteVariable {
    /// The id code of the new variable that is going to be written
    id_code: IdCode,
    /// The width of this variable
    width: u32,
    /// The path of the variable
    source_id_codes: Vec<IdCodeWithShift>,
}

impl VcdRewriteVariable {
    pub fn new(id_code: IdCode, width: u32, source_id_codes: Vec<IdCodeWithShift>) -> Self {
        Self {
            id_code,
            width,
            source_id_codes,
        }
    }

    /// Create a new VcdRewriteVariable from a TyVariable.
    ///
    /// This function inspects the hierarchy of the TyVariable and for each ground variable found, it creates a new IdCodeWithShift.
    pub fn create(
        id_code: IdCode,
        width: u32,
        ty_variable: &TyVariable,
        scope_path: &[String],
        vcd_header: &Header,
    ) -> Self {
        if vcd_header.find_scope(scope_path).is_none() {
            return Self {
                id_code,
                width,
                source_id_codes: Vec::new(),
            };
        }
        let mut source_id_codes = Vec::with_capacity(ty_variable.kind.find_width() as usize);
        // Collect first existing vcd_names: all the ground variables
        for ty_ground_variable in ty_variable.collect_ground_variables() {
            match ty_ground_variable.kind {
                TyVarKind::Ground(width) => {
                    if width < 1 {
                        continue;
                    }
                    // Get the actual path of the variable
                    if let Some(vcd_name) = ty_ground_variable.get_trace_name() {
                        let path = &[scope_path, &[vcd_name.clone()]].concat();
                        // Find the variable in the original VCD file (if it exists)
                        if let Some(vcd_var) = vcd_header.find_var(path) {
                            // Prepend it
                            source_id_codes.insert(
                                0,
                                IdCodeWithShift::create(
                                    vcd_var.code,
                                    vcd::Vector::filled(Value::X, width as usize),
                                ),
                            );
                        }
                    }
                }
                TyVarKind::External => {} // Ignore external variables
                _ => unreachable!(
                    "Var \"{}\" Should be unreachable. Kind: {:?}",
                    ty_ground_variable.name, ty_ground_variable.kind
                ),
            }
        }

        Self {
            id_code,
            width,
            source_id_codes,
        }
    }

    /// Return the current value of the variable.
    /// The concatenation of the values of the source_id_codes shifted to the left.
    pub fn get_value(&self) -> vcd::Vector {
        // Calculate the value from its original variables
        let mut value: Vec<Value> = vec![Value::V0; self.width as usize];
        let mut start_idx = self.width as usize;

        for id_code_with_shift in &self.source_id_codes {
            start_idx -= id_code_with_shift.get_value().len();
            for (i, v) in id_code_with_shift.get_value().iter().enumerate() {
                let idx = start_idx + i;
                if idx < value.len() {
                    value[start_idx + i] = v;
                }
            }
        }

        value.into()
    }

    /// Update the value of the variable when an original variable is updated.
    pub fn update_value(&mut self, source_id_code: &IdCode, value: &vcd::Vector) -> Result<()> {
        // Find the source_id_code that needs to be updated
        for id_code_with_shift in &mut self.source_id_codes {
            if id_code_with_shift.id_code == *source_id_code {
                // Update the value
                return id_code_with_shift.update_value(value.clone());
            }
        }

        Ok(())
    }

    /// Return the id code of this variable
    #[inline]
    pub fn get_id_code(&self) -> IdCode {
        self.id_code
    }
    #[inline]
    pub fn get_width(&self) -> u32 {
        self.width
    }
}

/// It represents a variable from the original VCD file that is going to be used in the rewritten VCD file.
#[derive(Debug)]
pub struct IdCodeWithShift {
    /// The id code of the original variable
    id_code: IdCode,
    /// The number of bits to shift left the value of the original variable in the new variable
    // shift_left: u64,
    /// The "non-shifted" value of the original variable, the one read from the original VCD file
    value: vcd::Vector,
}

impl IdCodeWithShift {
    pub fn new(id_code: IdCode, value: vcd::Vector) -> Self {
        Self { id_code, value }
    }

    pub fn create(id_code: IdCode, value: vcd::Vector) -> Self {
        Self { id_code, value }
    }

    pub fn get_value(&self) -> &vcd::Vector {
        &self.value
    }

    pub fn update_value(&mut self, value: vcd::Vector) -> Result<()> {
        if value.len() != self.value.len() {
            return Err(VcdRewriteError::UpdateValueError {
                id_code: self.id_code,
                len_diff: (value.len(), self.value.len()),
                values_diff: (value.to_string(), self.value.to_string()),
            });
        }
        self.value = value;

        Ok(())
    }
}
