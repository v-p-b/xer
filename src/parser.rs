use nom::{
    IResult, Parser,
    bytes::complete::{tag, take},
};

use nom::character::complete::{char, multispace0};
use nom::multi::{many1, separated_list1};
use std::num::ParseIntError;
use std::vec::Vec;

use nom::branch::alt;

pub fn from_hex(input: &str) -> Result<u8, ParseIntError> {
    u8::from_str_radix(input, 16)
}

pub fn from_bin(input: &str) -> Result<u8, ParseIntError> {
    u8::from_str_radix(input, 2)
}

pub fn hex_byte(input: &str) -> IResult<&str, u8> {
    let (input, digits) = take(2usize)(input)?;
    let res = from_hex(digits);
    match res {
        Ok(res) => Ok((input, res)),
        Err(_) => Err(nom::Err::Error(nom::error::Error {
            input,
            code: nom::error::ErrorKind::HexDigit,
        })),
    }
}

pub fn bin_byte(input: &str) -> IResult<&str, u8> {
    let (input, digits) = take(8usize)(input)?;
    let res = from_bin(digits);
    match res {
        Ok(res) => Ok((input, res)),
        Err(_) => Err(nom::Err::Error(nom::error::Error {
            input,
            code: nom::error::ErrorKind::Digit,
        })),
    }
}

pub fn hex_0x_byte(input: &str) -> IResult<&str, u8> {
    let (input, _) = (tag("0x")).parse(input)?;
    let (input, res) = hex_byte(input)?;
    Ok((input, res))
}

pub fn bin_0b_byte(input: &str) -> IResult<&str, u8> {
    let (input, _) = (tag("0b")).parse(input)?;
    let (input, res) = bin_byte(input)?;
    Ok((input, res))
}


pub fn hex_esc_byte(input: &str) -> IResult<&str, u8> {
    let (input, _) = (tag("\\x")).parse(input)?;
    let (input, res) = hex_byte(input)?;
    Ok((input, res))
}

pub fn c_list_separator(input: &str) -> IResult<&str, (&str, char, &str)> {
    (multispace0, char(','), multispace0).parse(input)
}

pub fn hex_0x_seq(input: &str) -> IResult<&str, Vec<u8>> {
    separated_list1(c_list_separator, hex_0x_byte).parse(input)
}

pub fn bin_0b_seq(input: &str) -> IResult<&str, Vec<u8>> {
    separated_list1(c_list_separator, bin_0b_byte).parse(input)
}

pub fn hex_esc_seq(input: &str) -> IResult<&str, Vec<u8>> {
    many1(hex_esc_byte).parse(input)
}

pub fn hex_seq(input: &str) -> IResult<&str, Vec<u8>> {
    separated_list1(multispace0, hex_byte).parse(input)
}

pub fn any_seq(input: &str) -> IResult<&str, Vec<u8>> {
    alt((bin_0b_seq, hex_esc_seq, hex_0x_seq, hex_seq)).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_tests() {
        assert_eq!(hex_0x_byte("0xab"), Ok(("", 0xab)));
        assert_eq!(hex_esc_byte("\\xcd"), Ok(("", 0xcd)));
        assert_eq!(
            hex_0x_seq("0xde,0xad,0xbe,0xef"),
            Ok(("", vec![0xdeu8, 0xadu8, 0xbeu8, 0xefu8]))
        );
        assert_eq!(
            hex_0x_seq("0xde, 0xad ,0xbe , \n0xef"),
            Ok(("", vec![0xdeu8, 0xadu8, 0xbeu8, 0xefu8]))
        );
        assert_eq!(
            hex_esc_seq("\\xde\\xad\\xbe\\xef"),
            Ok(("", vec![0xdeu8, 0xadu8, 0xbeu8, 0xefu8]))
        );
        assert_eq!(
            hex_seq("dead  be ef "),
            Ok((" ", vec![0xdeu8, 0xadu8, 0xbeu8, 0xefu8]))
        );
        assert_eq!(
            bin_0b_seq("0b10101010 , 0b10111011,\r\n0b10101010 ,0b10111011"),
            Ok(("", vec![0xaau8, 0xbbu8, 0xaau8, 0xbbu8]))
        );

    }
}
