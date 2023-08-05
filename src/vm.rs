use crate::instruction::Opcode;

#[derive(Debug)]
pub struct VM {
    pub registers: [i32; 32],
    pc: usize,
    pub program: Vec<u8>,
    remainder: u32,
    EqualFlag: bool,
}

impl VM {
pub fn new() -> VM {
    VM {
        registers: [0; 32],
        pc: 0,
        program: vec![],
        remainder: 0,
        EqualFlag: false,
    }
}

pub fn run(&mut self) {
    let mut is_done = false;
    while !is_done {
        is_done = self.execute_instruction();
    }
}
pub fn run_once(&mut self) {
    self.execute_instruction();
}
pub fn add_byte(&mut self, byte: u8) {
    self.program.push(byte);
}
pub fn add_bytes(&mut self, mut bytes: Vec<u8>) {
    self.program.append(&mut bytes);
}
fn execute_instruction(&mut self) -> bool {
    if self.pc >= self.program.len() {
        return false;
    }
    match self.decode_opcode() {
        Opcode::LOAD => {
            let register = self.next_8_bits() as usize;
            let number = self.next_16_bits() as u32;
            self.registers[register] = number as i32;
        },
        Opcode::ADD => {
            let register1 = self.registers[self.next_8_bits() as usize];
            let register2 = self.registers[self.next_8_bits() as usize];
            self.registers[self.next_8_bits() as usize] = register1 + register2;
        },
        Opcode::SUB => {
            let register1 = self.registers[self.next_8_bits() as usize];
            let register2 = self.registers[self.next_8_bits() as usize];
            self.registers[self.next_8_bits() as usize] = register1 - register2;
        },
        Opcode::MUL => {
            let register1 = self.registers[self.next_8_bits() as usize];
            let register2 = self.registers[self.next_8_bits() as usize];
            self.registers[self.next_8_bits() as usize] = register1 * register2;
        },
        Opcode::DIV => {
            let register1 = self.registers[self.next_8_bits() as usize];
            let register2 = self.registers[self.next_8_bits() as usize];
            self.registers[self.next_8_bits() as usize] = register1 / register2;
            self.remainder = (register1  % register2) as u32;
        }
        Opcode::HLT => {
            println!("HLT encountered");
            return false;
        },
        Opcode::JMP => {
            let target = self.registers[self.next_8_bits() as usize];
            self.pc = target as usize;
        },
        Opcode::JMPF => {
            let value = self.registers[self.next_8_bits() as usize];
            self.pc += value as usize;
        },
        Opcode::JMPB => {
            let value = self.registers[self.next_8_bits() as usize];
            self.pc -= value as usize;
        },
        Opcode::EQ => {
            let register1 = self.registers[self.next_8_bits() as usize];
            let register2 = self.registers[self.next_8_bits() as usize];
            if register1 == register2 {
                self.EqualFlag=true;
            } else {
                self.EqualFlag=false;
            }
            self.next_8_bits();
        },
        Opcode::NEQ => {
            let register1 = self.registers[self.next_8_bits() as usize];
            let register2 = self.registers[self.next_8_bits() as usize];
            if register1 != register2 {
                self.EqualFlag=true;
            } else {
                self.EqualFlag=false;
            }
            self.next_8_bits();
        },
        Opcode::GT => {
            let register1 = self.registers[self.next_8_bits() as usize];
            let register2 = self.registers[self.next_8_bits() as usize];
            if register1 > register2 {
                self.EqualFlag=true;
            } else {
                self.EqualFlag=false;
            }
            self.next_8_bits();
        },
        Opcode::GTQ => {
            let register1 = self.registers[self.next_8_bits() as usize];
            let register2 = self.registers[self.next_8_bits() as usize];
            if register1 >= register2 {
                self.EqualFlag=true;
            } else {
                self.EqualFlag=false;
            }
            self.next_8_bits();
        },
        Opcode::LT => {
            let register1 = self.registers[self.next_8_bits() as usize];
            let register2 = self.registers[self.next_8_bits() as usize];
            if register1 < register2 {
                self.EqualFlag=true;
            } else {
                self.EqualFlag=false;
            }
            self.next_8_bits();
        },
        Opcode::LTQ => {
            let register1 = self.registers[self.next_8_bits() as usize];
            let register2 = self.registers[self.next_8_bits() as usize];
            if register1 <= register2 {
                self.EqualFlag=true;
            } else {
                self.EqualFlag=false;
            }
            self.next_8_bits();
        },
        Opcode::JEQ => {
            let register = self.next_8_bits() as usize;
            let target = self.registers[register];
            if self.EqualFlag {
                self.pc = target as usize;
            }
        },
        Opcode::IGL => {
            println!("invalid Opcode");
            return false;
        }
    }
    true
}
fn decode_opcode(&mut self) -> Opcode {
    let opcode = Opcode::from(self.program[self.pc]);
    self.pc+=1;
    return opcode;
}
fn next_8_bits(&mut self) -> u8 {
    let result = self.program[self.pc];
    dbg!(self.pc);
    self.pc+=1;
    return result;
}
fn next_16_bits(&mut self) -> u16 {
    let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc+1] as u16;
    self.pc+=2;
    return result;
}
}

