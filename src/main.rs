mod parser;
pub use crate::parser::hex_parser::*;

use clap::{
    Parser as ClapParser,
    ValueEnum
};

use std::fs;
use std::io;
use std::io::prelude::*;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Format{
    /// 2-digit hex encoding, optionally separated by whitespace
    Hex,
    /// \\xHH encoding, no separator
    Escaped,
    /// 0xHH encoding, values separated with commas
    C,
    /// Raw bytes
    Raw,
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
    input: Option<String>,

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
    let input :String = match args.input {
        Some(fname) => fs::read_to_string(fname).expect("Invalid input filename"),
        None => {
                let mut inbuf=Vec::new();
                let mut stdin=io::stdin();
                let _ = stdin.read_to_end(&mut inbuf);
                match str::from_utf8(&inbuf) {
                    Ok(v) => v.to_string(),
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                }    
            }
        };
    

    let Ok((_, data)) = hex_any_seq(&input) else {panic!("Couldn't process input!")};
    write_0x_hex(data);
}



