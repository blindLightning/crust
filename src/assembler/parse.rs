use nom;
use nom::character::complete::multispace0;
use nom::error::ParseError;
use nom::sequence::delimited;

pub type Input<'a> = &'a str;
pub type Result<'a, O, E = nom::error::VerboseError<Input<'a>>> = nom::IResult<Input<'a>, O, E>;


pub fn ws(i: Input) -> Result<&str> {
  let result = multispace0(i)?;
  Ok(result)
}