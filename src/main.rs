mod parser;
pub use crate::parser::hex_parser::*;

use clap::{
    Parser as ClapParser,
    ValueEnum
};

use std::fs;


#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Format{
    /// 2-digit hex encoding, optionally separated by whitespace
    Hex,
    /// \\xHH encoding, no separator
    Escaped,
    /// 0xHH encoding, values separated with commas
    C
}

#[derive(ClapParser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input format specifier
    #[arg(value_enum)]
    from: Option<Format>,

    /// Output format specifier
    #[arg(value_enum)]
    to: Option<Format>,

    /// Input file
    #[arg(short, long)]
    input: String,

    // Output file
    //#[arg(short, long)]
    //output: Option<String>,


}

pub fn write_0x_hex(v: Vec<u8>){
    for b in v.iter() {
        println!("{} - ", b);
    }    
}

fn main() {
    let args = Args::parse();
    let input = fs::read_to_string(args.input).expect("Invalid input filename");

    let Ok((_, data)) = hex_any_seq(&input) else {panic!("Couldn't process input!")};
    write_0x_hex(data);
}



