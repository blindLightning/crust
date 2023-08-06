use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use crate::assembler::Token;
use super::parse;

pub fn integer_operand(i: parse::Input) -> parse::Result<Token> {
    let (i, _) = parse::ws(i)?;
    let (i, _) = tag("#")(i)?;
    let (i, operand) = digit1(i)?;
    let operand = operand.parse::<i32>().unwrap();
    let tok = Token::IntegerOperand { value: operand };
    Ok((i, tok))
}