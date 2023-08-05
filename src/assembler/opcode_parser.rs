use nom::bytes::complete::tag;
use nom::character::complete::alpha1;

use crate::assembler::Token;
use crate::instruction::Opcode;
use super::parse;


pub fn opcode_load(i: parse::Input) -> parse::Result<Token> {
    let (i, opcode) = alpha1(i)?;
    let tok = Token::Op { code: Opcode::from(opcode) };
    Ok((i, tok))
}