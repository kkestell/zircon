use std::collections::HashMap;
use std::vec::Vec;

use crate::bytecode::{Bytecode, Opcode, Value};

struct CallFrame {
    instruction_pointer: usize,
    function_index: usize,
    stack: Vec<Value>,
    locals: HashMap<usize, Value>,
}

impl CallFrame {
    fn new(func_index: usize) -> Self {
        CallFrame {
            instruction_pointer: 0,
            function_index: func_index,
            stack: Vec::new(),
            locals: HashMap::new(),
        }
    }

    fn advance_instruction_pointer(&mut self) {
        self.instruction_pointer += 1;
    }

    fn set_instruction_pointer(&mut self, ip: usize) {
        self.instruction_pointer = ip;
    }

    fn get_instruction_pointer(&self) -> usize {
        self.instruction_pointer
    }

    fn get_function_index(&self) -> usize {
        self.function_index
    }

    fn set_local(&mut self, index: usize, value: Value) {
        self.locals.insert(index, value);
    }

    fn get_local(&self, index: usize) -> Option<&Value> {
        self.locals.get(&index)
    }

    fn stack_push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn stack_pop(&mut self) -> Option<Value> {
        self.stack.pop()
    }

    fn is_stack_empty(&self) -> bool {
        self.stack.is_empty()
    }

    // fn debug_stack(&self) {
    //     for (i, value) in self.stack.iter().enumerate() {
    //         println!("Stack[{}]: {}", i, value);
    //     }
    // }
}

pub(crate) struct VirtualMachine<'a> {
    is_running: bool,
    bytecode: &'a Bytecode,
    frames: Vec<CallFrame>,
}

impl<'a> VirtualMachine<'a> {
    pub(crate) fn new(bytecode: &'a Bytecode) -> Self {
        VirtualMachine {
            is_running: true,
            bytecode,
            frames: Vec::new(),
        }
    }

    fn push_frame(&mut self, frame: CallFrame) {
        self.frames.push(frame);
    }

    fn pop_frame(&mut self) {
        if self.frames.is_empty() {
            panic!("Call stack underflow.");
        }
        self.frames.pop();
    }

    fn current_frame(&mut self) -> &mut CallFrame {
        self.frames.last_mut().expect("Call stack is empty.")
    }

    fn is_call_stack_empty(&self) -> bool {
        self.frames.is_empty()
    }

    fn is_operand_stack_empty(&self) -> bool {
        if let Some(current_frame) = self.frames.last() {
            current_frame.is_stack_empty()
        } else {
            true
        }
    }

    fn push_operand(&mut self, value: Value) {
        self.current_frame().stack_push(value);
    }

    fn pop_operand(&mut self) -> Value {
        self.current_frame().stack_pop().expect("Stack underflow.")
    }

    fn get_local(&mut self, index: usize) -> Value {
        self.current_frame()
            .get_local(index)
            .cloned()
            .expect("Local variable not found.")
    }

    fn set_local(&mut self, index: usize, value: Value) {
        self.current_frame().set_local(index, value);
    }

    fn unary_op(&mut self, opcode: Opcode) {
        let val = self.pop_operand();
        let result = match opcode {
            Opcode::Not => val.logical_not(),
            Opcode::Negate => val.negate(),
            _ => panic!("Invalid opcode for unary operation."),
        };
        self.push_operand(result);
    }

    fn binary_op(&mut self, opcode: Opcode) {
        let val2 = self.pop_operand();
        let val1 = self.pop_operand();
        let result = match opcode {
            Opcode::Add => val1.add(&val2),
            Opcode::Subtract => val1.subtract(&val2),
            Opcode::Multiply => val1.multiply(&val2),
            Opcode::Divide => val1.divide(&val2),
            Opcode::Modulo => val1.modulo(&val2),
            Opcode::And => val1.logical_and(&val2),
            Opcode::Or => val1.logical_or(&val2),
            _ => panic!("Invalid opcode for binary operation."),
        };
        self.push_operand(result);
    }

    fn handle_jump(&mut self, target: usize) {
        self.current_frame().set_instruction_pointer(target);
    }

    pub(crate) fn run(&mut self) {
        self.push_frame(CallFrame::new(0));

        while !self.is_call_stack_empty() && self.is_running {
            let function_index = self.current_frame().get_function_index();
            let current_function = self.bytecode.get_function(function_index);
            let current_frame = self.current_frame();
            let current_instruction_pointer = current_frame.get_instruction_pointer();
            let instruction = current_function.get_instruction(current_instruction_pointer);

            // println!("IP: {}", current_instruction_pointer);
            // current_frame.debug_stack();
            // println!("Instruction: {:?}", instruction.opcode());

            current_frame.advance_instruction_pointer();

            match instruction.opcode() {
                Opcode::PushConst => {
                    let constant = self
                        .bytecode
                        .get_constant(instruction.operand().into())
                        .expect("Constant index out of range.");
                    self.push_operand(constant.clone());
                }
                Opcode::Add
                | Opcode::Subtract
                | Opcode::Multiply
                | Opcode::Divide
                | Opcode::Modulo
                | Opcode::And
                | Opcode::Or => {
                    self.binary_op(instruction.opcode());
                }
                Opcode::Not | Opcode::Negate => {
                    self.unary_op(instruction.opcode());
                }
                Opcode::Equal => {
                    let val2 = self.pop_operand();
                    let val1 = self.pop_operand();
                    self.push_operand(Value::Boolean(val1 == val2));
                }
                Opcode::Jump => {
                    self.handle_jump(instruction.operand().into());
                }
                Opcode::JumpIfTrue => {
                    let val = self.pop_operand();
                    if let Value::Boolean(true) = val {
                        self.handle_jump(instruction.operand().into());
                    }
                }
                Opcode::JumpIfFalse => {
                    let val = self.pop_operand();
                    if let Value::Boolean(false) = val {
                        self.handle_jump(instruction.operand().into());
                    }
                }
                Opcode::Print => {
                    let val = self.pop_operand();
                    println!("{}", val);
                }
                Opcode::GetLocal => {
                    let val = self.get_local(instruction.operand().into());
                    self.push_operand(val);
                }
                Opcode::SetLocal => {
                    let val = self.pop_operand();
                    self.set_local(instruction.operand().into(), val);
                }
                Opcode::Call => {
                    let operand = instruction.operand();
                    let func_to_call = self.bytecode.get_function(operand.into());
                    let mut new_frame = CallFrame::new(operand.into());
                    for i in 0..func_to_call.num_args {
                        let arg = self.pop_operand();
                        new_frame.set_local(func_to_call.num_args - i - 1, arg);
                    }
                    self.push_frame(new_frame);
                }
                Opcode::Return => {
                    let return_value = if !self.is_operand_stack_empty() {
                        self.pop_operand()
                    } else {
                        Value::Boolean(false)
                    };
                    self.pop_frame();
                    if !self.is_call_stack_empty() {
                        self.push_operand(return_value);
                    }
                }
                Opcode::Halt => {
                    self.is_running = false;
                }
            }
        }
    }
}
