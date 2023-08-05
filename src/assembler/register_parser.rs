use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use crate::assembler::Token;
use super::parse;


pub fn register(i: parse::Input) -> parse::Result<Token> {
    let (i, _) = parse::ws(i)?;
    let (i, _) = tag("$")(i)?;
    let (i, reg_num) = digit1(i)?;
    let reg_num = reg_num.parse::<u8>().unwrap();
    let tok = Token::Register { reg_num: reg_num };
    Ok((i, tok))
}
