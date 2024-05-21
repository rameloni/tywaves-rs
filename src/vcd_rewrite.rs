use std::{fs::File, io::*, path::Path, str::FromStr};
use vcd::{Command, Header, IdCode, Parser, ReferenceIndex, Value, VarType, Writer};

use crate::symbol_table;

// Alias of symbol_table::Scope
type TyScope = symbol_table::Scope;
type TyVariable = symbol_table::Variable;
type TyRealType = symbol_table::RealType;
type TyHwType = symbol_table::HwType;

pub struct VcdRewriter {
    /// The reader of the original VCD file
    reader: Parser<BufReader<File>>,
    /// The writer of the rewritten VCD file
    writer: Writer<BufWriter<File>>,
    /// The name of the rewritten VCD file
    output_vcd_name: &'static str,
    /// The header of the original VCD file
    vcd_header: Header,

    /// The scopes of the tywaves state
    tywaves_scopes: Vec<TyScope>,

    /// A list of the variables that are going to be written in the rewritten VCD file
    rewrite_variables: Vec<VcdRewriteVariable>,
}

impl VcdRewriter {
    /// Get the full path of the rewritten VCD file
    pub fn get_final_file(&self) -> &'static str {
        self.output_vcd_name
    }
    pub fn new(vcd: &Path, tywaves_scopes: Vec<TyScope>) -> Self {
        let vcd_file = File::open(vcd).unwrap();
        const RESULT_VCD: &str = "test.vcd";
        let output_vcd = File::create(RESULT_VCD).unwrap();

        let reader = Parser::new(BufReader::new(vcd_file));

        // Extract the information from the reader
        let writer = Writer::new(BufWriter::new(output_vcd));

        Self {
            reader,
            writer,
            output_vcd_name: RESULT_VCD,
            tywaves_scopes,
            vcd_header: Header::default(),
            rewrite_variables: vec![],
        }
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
        self.writer.add_module(&scope.name)?;

        // The scope of the children of the current scope
        let child_path_scope = &[path_scope, &[scope.name.clone()]].concat();

        // Add the variables to the header
        for variable in &scope.child_variables {
            self.add_variable_to_header(variable, child_path_scope)?;
        }

        // Add the child scopes to the header
        for child_scope in &scope.child_scopes {
            self.add_scope_to_header(child_scope, child_path_scope)?;
        }

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
        let var_type = match &ty_variable.hw_type {
            TyHwType::Reg => VarType::Reg,
            _ => VarType::Wire,
        };
        let reference = match &ty_variable.real_type {
            TyRealType::Ground {
                vcd_name: Some(vcd_name),
                ..
            }
            | TyRealType::Bundle {
                vcd_name: Some(vcd_name),
                ..
            } => vcd_name,
            TyRealType::Ground { vcd_name: None, .. }
            | TyRealType::Bundle { vcd_name: None, .. } => &ty_variable.name,
            TyRealType::Vec { .. } => todo!("Vec support"),
            TyRealType::Unknown => todo!("Unknown type"),
        };
        let width = ty_variable.real_type.find_width() as u32;
        let index = (width > 1).then_some(ReferenceIndex::Range(width as i32 - 1, 0));

        // Write the variable to the VCD file
        let new_id = self.writer.add_var(var_type, width, reference, index)?;

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
                if target_var.update_value(original_id, &value) {
                    writer
                        .change_vector(target_var.get_id_code(), target_var.get_value().iter())?;
                }
            }
            Ok(())
        };

        while let Some(command) = self.reader.next().transpose().unwrap() {
            match command {
                Command::ChangeScalar(original_id, value) => update_var(
                    &mut self.writer,
                    &original_id,
                    vcd::Vector::from(vec![value]),
                )?,
                Command::ChangeVector(original_id, value) => {
                    update_var(&mut self.writer, &original_id, value)?
                }
                Command::Timestamp(ts) => self.writer.timestamp(ts)?,
                _ => {}
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
        let mut source_id_codes = vec![];
        if vcd_header.find_scope(scope_path).is_none() {
            return Self {
                id_code,
                width,
                source_id_codes,
            };
        }
        // Collect first existing vcd_names
        for ty_ground_variable in ty_variable.collect_ground_variables() {
            match ty_ground_variable {
                TyRealType::Ground {
                    vcd_name,
                    width,
                    constant: _,
                } => {
                    // Get the actual path of the variable
                    if let Some(vcd_name) = vcd_name {
                        let path = &[scope_path, &[vcd_name.clone()]].concat();
                        // println!("PATH: {:?}", path);
                        // Find the variable in the original VCD file (if it exists)
                        if let Some(vcd_var) = vcd_header.find_var(path) {
                            // println!("FOUND: {:?}", vcd_var.reference);
                            // Prepend it
                            source_id_codes.insert(
                                0,
                                IdCodeWithShift::create(
                                    vcd_var.code,
                                    vcd::Vector::filled(Value::X, width.get() as usize),
                                ),
                            );
                        }
                    }
                }
                _ => panic!("Not implemented"),
            }
        }

        for ty_ground_variable in ty_variable.collect_ground_variables() {
            break;
            match ty_ground_variable {
                TyRealType::Ground {
                    vcd_name: _,
                    width,
                    constant,
                } => {
                    // Get the actual path of the variable
                    if let Some(constant_value) = constant {
                        let id_code: IdCode = if let Some(last) = source_id_codes.first() {
                            let a = last.id_code.next();
                            print!("Next id code: {:?}", a.to_string());
                            a
                        } else {
                            let a = IdCode::FIRST;
                            print!("FIRST: idCode: {:?}", a.to_string());
                            a
                        };

                        let value = vcd::Vector::from_str(&constant_value).unwrap();
                        source_id_codes.insert(0, IdCodeWithShift::create(id_code, value));
                    }
                }
                _ => panic!("Not implemented"),
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

        println!("\tVALUE: {:?}", value.len());
        println!("\tWIDTH: {:?}", self.width);
        println!("\tSTART_IDX: {:?}", start_idx);

        for id_code_with_shift in &self.source_id_codes {
            start_idx -= id_code_with_shift.get_value().len();
            println!("\tSTART_IDX: {:?}", start_idx);
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
    pub fn update_value(&mut self, source_id_code: &IdCode, value: &vcd::Vector) -> bool {
        // Find the source_id_code that needs to be updated
        for id_code_with_shift in &mut self.source_id_codes {
            if id_code_with_shift.id_code == *source_id_code {
                // Update the value
                id_code_with_shift.update_value(value.clone());
                println!("UPDATED: {:?}", id_code_with_shift.id_code.to_string());

                return true;
            }
        }
        false
    }

    /// Return the id code of this variable
    pub fn get_id_code(&self) -> IdCode {
        self.id_code
    }

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

    pub fn update_value(&mut self, value: vcd::Vector) {
        assert_eq!(value.len(), self.value.len(), "Failing to update the value. Update value has a different size from the original: new {} original {}", value.len(), self.value.len());
        self.value = value;
    }
}
