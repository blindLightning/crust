

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Opcode {
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    HLT,
    JMP,
    JMPF,
    JMPB,
    JEQ,
    EQ,
    NEQ,
    GT,
    LT,
    GTQ,
    LTQ,
    IGL
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction { 
            opcode: opcode 
        }
    }
}


impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 => return Opcode::HLT,
            1 => return Opcode::LOAD,
            2 => return Opcode::JMP,
            3 => return Opcode::JMPF,
            4 => return Opcode::JMPB,
            5 => return Opcode::ADD,
            6 => return Opcode::SUB,
            7 => return Opcode::MUL,
            8 => return Opcode::DIV,
            9 => return Opcode::JEQ,
            10 => return Opcode::EQ,
            11 => return Opcode::NEQ,
            12 => return Opcode::GT,
            13 => return Opcode::LT,
            14 => return Opcode::GTQ,
            15 => return Opcode::LTQ,
            16 => return Opcode::IGL,
            _ => return Opcode::IGL
        }
    }
}

impl From<&str> for  Opcode{
fn from(v: &str) -> Self {
    match v.to_lowercase().as_str() {
        "hlt" => Opcode::HLT,
        "load" => Opcode::LOAD,
        "add" => Opcode::ADD,
        "sub" => Opcode::SUB,
        "mul" => Opcode::MUL,
        "div" => Opcode::DIV,
        "jmp" => Opcode::JMP,
        "jmpf" => Opcode::JMPF,
        "jmpb" => Opcode::JMPB,
        "jeq" => Opcode::JEQ,
        "eq" => Opcode::EQ,
        "neq" => Opcode::NEQ,
        "lt" => Opcode::LT,
        "ltq" => Opcode::LTQ,
        "gt" => Opcode::GT,
        "gtq" => Opcode::GTQ,
        _ => Opcode::IGL
    }
}
}

impl From<Opcode> for u8 {
fn from(value: Opcode) -> Self {
    match value {
        Opcode::HLT => 0,
        Opcode::LOAD => 1,
        Opcode::JMP => 2,
        Opcode::JMPF => 3,
        Opcode::JMPB => 4,
        Opcode::ADD => 5,
        Opcode::SUB => 6,
        Opcode::MUL => 7,
        Opcode::DIV => 8,
        Opcode::JEQ => 9,
        Opcode::EQ => 10,
        Opcode::NEQ => 11,
        Opcode::GT => 12,
        Opcode::LT => 13,
        Opcode::GTQ => 14,
        Opcode::LTQ => 15,
        Opcode::IGL => 16,
    }
}
}