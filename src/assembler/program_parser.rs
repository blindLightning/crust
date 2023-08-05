use nom::multi::many1;
use super::instruction_parser::*;
use super::parse;


#[derive(Debug, PartialEq)]
pub struct Program {
    instructions: Vec<AssemblerInstruction>
}




pub fn Program(i: parse::Input) -> parse::Result<Program> {
    let (i, instructions) = many1(instruction)(i)?;
    let prog = Program{
        instructions: instructions
    };
    Ok((i, prog))
}


impl Program {
pub fn to_bytes(&self) -> Vec<u8> {
    let mut results = vec![];
    for instruction in &self.instructions {
        let mut bytes = instruction.to_bytes();
        results.append(&mut bytes);
    }
    return results;
}
}
