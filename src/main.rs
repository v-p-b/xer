use nom::{
  IResult,
  Parser,
  bytes::complete::{tag, take},
};

use std::num::ParseIntError;

fn from_hex(input: &str) -> Result<u8, ParseIntError> {
  u8::from_str_radix(input, 16)
}

fn hex_byte(input: &str) -> IResult<&str, u8> {
    let (input, digits) = take(2usize)(input)?;
    let res = from_hex(digits);
    match res {
        Ok(res) => Ok((input,res)),
        Err(_) => Err(nom::Err::Error(nom::error::Error{input, code: nom::error::ErrorKind::HexDigit}))
    }
}

fn hex_0x_byte(input: &str) -> IResult<&str, u8> {
    let (input, _) = (tag("0x")).parse(input)?;
    let (input, res) = hex_byte(input)?;
    Ok((input, res))
}

fn main() {
    assert_eq!(hex_0x_byte("0xab"), Ok(("", 0xab)));
}
