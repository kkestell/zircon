use std::fmt;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::Path;
use std::vec::Vec;

use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Opcode {
    PushConst = 0x01,
    Add = 0x10,
    Subtract = 0x11,
    Multiply = 0x12,
    Divide = 0x13,
    Modulo = 0x14,
    Negate = 0x15,
    And = 0x20,
    Or = 0x21,
    Not = 0x22,
    Equal = 0x30,
    Jump = 0x40,
    JumpIfTrue = 0x41,
    JumpIfFalse = 0x42,
    Print = 0x60,
    GetLocal = 0x70,
    SetLocal = 0x71,
    Call = 0x80,
    Return = 0x81,
    Halt = 0xFF,
}

impl Opcode {
    fn from_u8(value: u8) -> io::Result<Opcode> {
        match value {
            0x01 => Ok(Opcode::PushConst),
            0x10 => Ok(Opcode::Add),
            0x11 => Ok(Opcode::Subtract),
            0x12 => Ok(Opcode::Multiply),
            0x13 => Ok(Opcode::Divide),
            0x14 => Ok(Opcode::Modulo),
            0x15 => Ok(Opcode::Negate),
            0x20 => Ok(Opcode::And),
            0x21 => Ok(Opcode::Or),
            0x22 => Ok(Opcode::Not),
            0x30 => Ok(Opcode::Equal),
            0x40 => Ok(Opcode::Jump),
            0x41 => Ok(Opcode::JumpIfTrue),
            0x42 => Ok(Opcode::JumpIfFalse),
            0x60 => Ok(Opcode::Print),
            0x70 => Ok(Opcode::GetLocal),
            0x71 => Ok(Opcode::SetLocal),
            0x80 => Ok(Opcode::Call),
            0x81 => Ok(Opcode::Return),
            0xFF => Ok(Opcode::Halt),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Unknown opcode")),
        }
    }

    fn has_operand(self) -> bool {
        match self {
            Opcode::PushConst => true,
            Opcode::Add => false,
            Opcode::Subtract => false,
            Opcode::Multiply => false,
            Opcode::Divide => false,
            Opcode::Modulo => false,
            Opcode::Negate => false,
            Opcode::And => false,
            Opcode::Or => false,
            Opcode::Not => false,
            Opcode::Equal => false,
            Opcode::Jump => true,
            Opcode::JumpIfTrue => true,
            Opcode::JumpIfFalse => true,
            Opcode::Print => false,
            Opcode::GetLocal => true,
            Opcode::SetLocal => true,
            Opcode::Call => true,
            Opcode::Return => false,
            Opcode::Halt => false,
        }
    }
}

pub(crate) struct Instruction {
    opcode: Opcode,
    operand: Option<u16>,
}

impl Instruction {
    fn new(opcode: Opcode, operand: Option<u16>) -> Self {
        Instruction { opcode, operand }
    }

    pub(crate) fn opcode(&self) -> Opcode {
        self.opcode
    }

    // fn has_operand(&self) -> bool {
    //     self.operand.is_some()
    // }

    pub(crate) fn operand(&self) -> u16 {
        self.operand.expect("Instruction has no operand")
    }
}

#[derive(Clone, Debug)]
pub(crate) enum Value {
    Number(f64),
    Boolean(bool),
    Str(String),
}

impl Value {
    pub(crate) fn add(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
            _ => panic!("Invalid operand types for add."),
        }
    }

    pub(crate) fn subtract(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a - b),
            _ => panic!("Invalid operand types for subtract."),
        }
    }

    pub(crate) fn multiply(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a * b),
            _ => panic!("Invalid operand types for multiply."),
        }
    }

    pub(crate) fn divide(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a / b),
            _ => panic!("Invalid operand types for divide."),
        }
    }

    pub(crate) fn modulo(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a % b),
            _ => panic!("Invalid operand types for modulo."),
        }
    }

    pub(crate) fn negate(&self) -> Value {
        match self {
            Value::Number(a) => Value::Number(-a),
            _ => panic!("Invalid operand type for negate."),
        }
    }

    pub(crate) fn logical_and(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Boolean(a), Value::Boolean(b)) => Value::Boolean(*a && *b),
            _ => panic!("Invalid operand types for logical and."),
        }
    }

    pub(crate) fn logical_or(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Boolean(a), Value::Boolean(b)) => Value::Boolean(*a || *b),
            _ => panic!("Invalid operand types for logical or."),
        }
    }

    pub(crate) fn logical_not(&self) -> Value {
        match self {
            Value::Boolean(a) => Value::Boolean(!a),
            _ => panic!("Invalid operand type for logical not."),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Str(s) => write!(f, "{}", s),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Str(a), Value::Str(b)) => a == b,
            _ => false,
        }
    }
}

pub(crate) struct Function {
    pub(crate) instructions: Vec<Instruction>,
    pub(crate) num_args: usize,
}

impl Function {
    fn new(instructions: Vec<Instruction>, num_args: usize) -> Self {
        Function {
            instructions,
            num_args,
        }
    }

    pub(crate) fn get_instruction(&self, index: usize) -> &Instruction {
        return self
            .instructions
            .get(index)
            .expect("Invalid instruction index");
    }
}

pub(crate) struct Bytecode {
    functions: Vec<Function>,
    constants: Vec<Value>,
}

impl Bytecode {
    // fn new() -> Self {
    //     Bytecode {
    //         functions: Vec::new(),
    //         constants: Vec::new(),
    //     }
    // }

    pub(crate) fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mut file = BufReader::new(File::open(path)?);
        let mut magic = [0u8; 4];
        file.read_exact(&mut magic)?;

        // Check magic number
        if magic != [b'Z', b'R', b'C', b'N'] {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid magic number",
            ));
        }

        let version = file.read_u8()?;
        if version != 1 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unsupported version",
            ));
        }

        let num_constants = file.read_u32::<LittleEndian>()?;

        let mut constants = Vec::with_capacity(num_constants as usize);
        for _ in 0..num_constants {
            constants.push(read_constant(&mut file)?);
        }

        let num_functions = file.read_u32::<LittleEndian>()?;

        let mut functions = Vec::with_capacity(num_functions as usize);
        for _ in 0..num_functions {
            functions.push(read_function(&mut file)?);
        }

        Ok(Bytecode {
            functions,
            constants,
        })
    }

    pub(crate) fn get_function(&self, index: usize) -> &Function {
        self.functions.get(index).expect("Invalid function index")
    }

    pub(crate) fn get_constant(&self, index: usize) -> Option<&Value> {
        self.constants.get(index)
    }

    // fn add_function(&mut self, function: Function) {
    //     self.functions.push(function);
    // }
    //
    // fn add_constant(&mut self, constant: Value) -> usize {
    //     self.constants.push(constant);
    //     self.constants.len() - 1
    // }
}

fn read_constant<R: Read>(reader: &mut R) -> io::Result<Value> {
    let type_id = reader.read_u8()?;
    match type_id {
        0x01 => Ok(Value::Number(reader.read_f64::<LittleEndian>()?)),
        0x02 => Ok(Value::Boolean(reader.read_u8()? != 0)),
        0x03 => {
            let len = reader.read_u16::<LittleEndian>()? as usize;
            let mut buffer = vec![0; len];
            reader.read_exact(&mut buffer)?;
            let string = String::from_utf8(buffer)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            Ok(Value::Str(string))
        }
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Unknown constant type",
        )),
    }
}

fn read_function<R: Read>(reader: &mut R) -> io::Result<Function> {
    let num_instructions = reader.read_u32::<LittleEndian>()?;
    let num_args = reader.read_u32::<LittleEndian>()? as usize;
    let mut instructions = Vec::with_capacity(num_instructions as usize);

    for _ in 0..num_instructions {
        let opcode = Opcode::from_u8(reader.read_u8()?)?;
        let has_operand = opcode.has_operand();
        let operand = if has_operand {
            Some(reader.read_u16::<LittleEndian>()?)
        } else {
            None
        };
        instructions.push(Instruction::new(opcode, operand));
    }

    Ok(Function::new(instructions, num_args))
}
