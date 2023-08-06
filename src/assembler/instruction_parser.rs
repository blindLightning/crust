use nom::branch::alt;

use super::Token;
use super::opcode_parser::opcode_load;
use super::operand_parser::integer_operand;
use super::register_parser::register;
use super::parse;

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    opcode: Token,
    operand1: Option<Token>,
    operand2: Option<Token>,
    operand3: Option<Token>,
}

pub fn instruction(input: parse::Input) -> parse::Result<AssemblerInstruction> {
    let result = alt((instruction_three, instruction_two, instruction_one, instruction_zero))(input)?;
    Ok(result)
}
fn instruction_zero(input: parse::Input) -> parse::Result<AssemblerInstruction> {
    let (inp, o) = opcode_load(input)?;
    let assembler_instruction = AssemblerInstruction {
        opcode: o,
        operand1: None,
        operand2: None,
        operand3: None
    };
    Ok((inp, assembler_instruction))
}

fn instruction_one(input: parse::Input) -> parse::Result<AssemblerInstruction> {
    let (inp, o) = opcode_load(input)?;
    let (inp, r) = register(inp)?;
    let tok = AssemblerInstruction{
        opcode: o,
        operand1: Some(r),
        operand2: None,
    operand3: None
    };
    Ok((inp, tok))
}
fn instruction_two(input: parse::Input) -> parse::Result<AssemblerInstruction> {
    let (inp, o) = opcode_load(input)?;
    let (inp, r) = register(inp)?;
    let (_, i) = integer_operand(inp)?;
    let assembler_instruction = AssemblerInstruction{
        opcode: o,
        operand1: Some(r),
        operand2: Some(i),
        operand3: None
    };
    Ok((inp, assembler_instruction))
}

fn instruction_three(input: parse::Input) -> parse::Result<AssemblerInstruction> {
    let (inp, o) = opcode_load(input)?;
    let (inp, r1) = register(inp)?;
    let (inp, r2) = register(inp)?;
    let (inp, r3) = register(inp)?;
    let tok = AssemblerInstruction{
        opcode: o,
        operand1: Some(r1),
        operand2: Some(r2),
        operand3: Some(r3)
    };
    Ok((inp, tok))
}
impl AssemblerInstruction {
pub fn to_bytes(&self) -> Vec<u8> {
    let mut results = vec![];
    match self.opcode {
        Token::Op { code } => match code {
            _ => {
                results.push(u8::from(code));
            }
        },
        _ => {
            println!("None code found in opcode field");
            std::process::exit(1);
        }
    };
    for operand in vec![&self.operand1, &self.operand2, &self.operand3] {
        match operand {
            Some(t) => AssemblerInstruction::extract_operand(t, &mut results),
            None => {}
        };
    }
    return results;
}
fn extract_operand(t: &Token, results: &mut Vec<u8>) {
    match t {
        Token::Register { reg_num }=> {
            results.push(*reg_num);
        },
        Token::IntegerOperand { value } => {
            let converted = *value as u16;
            let byte1 = converted;
            let byte2 = converted >> 8;
            results.push(byte2 as u8);
            results.push(byte1 as u8);
        },
        _ => {
            println!("opcode found in operand field");
            std::process::exit(1);
        }
    }
}
}
