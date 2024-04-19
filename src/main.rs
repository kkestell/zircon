use bytecode::Bytecode;
use vm::VirtualMachine;
use std::env;

mod bytecode;
mod vm;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <bytecode_file>", args[0]);
        return;
    }

    let bytecode_filename = &args[1];
    let bytecode_result = Bytecode::from_file(bytecode_filename);
    match bytecode_result {
        Ok(bytecode) => {
            let mut vm = VirtualMachine::new(&bytecode);
            vm.run();
        }
        Err(e) => {
            eprintln!("Failed to load bytecode from '{}': {}", bytecode_filename, e);
        }
    }
}
