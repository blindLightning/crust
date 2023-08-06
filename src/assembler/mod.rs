use crate::instruction::Opcode;
mod opcode_parser;
mod register_parser;
mod operand_parser;
mod instruction_parser;
mod parse;
pub mod program_parser;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op{code: Opcode},
    Register{reg_num: u8},
    IntegerOperand{value: i32},
}
