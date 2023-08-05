use std::io;
use std::io::Write;
use std::num::ParseIntError;

use crate::vm::VM;
use crate::assembler::program_parser::Program;

pub struct Repl {
    command_buffer: Vec<String>,
    vm: VM,
}

impl Repl {
pub fn new() -> Repl {
    Repl { 
        command_buffer: vec![], 
        vm: VM::new()
    }
}
pub fn run(&mut self) {
    println!("Welcome to crusty burgers. Burgers for 99p. \r\n\r\n");
    loop {
        let mut buffer = String::new();
        let stdin = io::stdin();
        print!(">>> ");
        io::stdout().flush().expect("Unable to flush stdout. ");

        stdin.read_line(&mut buffer).expect("Unable to read line from user");
        let buffer = buffer.trim();
        self.command_buffer.push(buffer.to_string());
        match buffer {
            ".quit" => {
                std::process::exit(0);
            },
            ".program" => {
                println!("Program:");
                for instruction in &self.vm.program {
                    println!("{instruction}");
                }
            },
            ".registers" => {
                println!("Registers: ");
                for register in self.vm.registers {
                    println!("{register}");
                }
            },
            _ => {
                let parsed_program = Program(&buffer);
                match &parsed_program {
                    Ok(Result) => {
                        let (_, result) = &parsed_program.unwrap();
                        let mut bytecode = result.to_bytes();
                        self.vm.add_bytes(bytecode);
                        self.vm.run_once();
                    },
                    Err(e) => {
                        println!("{}", e);
                        continue;
                    }
                }
            }
        }
    }
}
fn parse_hex(&mut self, hex: &str) -> Result<Vec<u8>, ParseIntError> {
    let split = hex.split(" ").collect::<Vec<&str>>();
    let mut results: Vec<u8> = vec![];
    for hex_string in split {
        let byte = u8::from_str_radix(&hex_string, 16);
        match byte {
            Ok(result) => {
                results.push(result);
            },
            Err(e) => {
                return Err(e);
            }
        }
    }
    Ok(results)
}
}