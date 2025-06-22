mod parser;
mod writer;
pub use crate::parser::*;
pub use crate::writer::*;

use clap::{Parser as ClapParser, ValueEnum};

use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Format {
    /// 2-digit hex encoding, optionally separated by whitespace
    Hex,
    /// \xHH encoding, no separator
    Escaped,
    /// 0xHH encoding, values separated with commas and whitespace
    C,
    /// 0xHH encoding, with optional negative sign, values separated with commas and whitespace
    Java,
    /// 0bBBBBBBBB encoded binary, values separated with commas and whitespace
    Bin,
    /// Decimal
    Dec,
    /// Signed Decimal
    SDec,
    /// Raw bytes
    Raw,
}

#[derive(ClapParser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input format specifier
    #[arg(short, long)]
    from: Option<Format>,

    /// Output format specifier
    #[arg(short, long)]
    to: Option<Format>,

    /// Input file (default: stdin)
    #[arg(short, long)]
    input: Option<String>,

    /// Output file (default: stdout)
    #[arg(short, long)]
    output: Option<String>,
}

fn main() {
    let args = Args::parse();

    let data = if let Some(Format::Raw) = args.from {
        match args.input {
            Some(fname) => fs::read(fname).expect("Invalid input filename"), // Vec<u8>
            None => {
                let mut inbuf = Vec::new();
                let mut stdin = io::stdin();
                let _ = stdin.read(&mut inbuf);
                inbuf
            }
        }
    } else {
        let xer_parser = match args.from {
            Some(Format::Escaped) => hex_esc_seq,
            Some(Format::Hex) => hex_seq,
            Some(Format::C) => hex_0x_seq,
            Some(Format::Java) => hex_signed_seq,
            Some(Format::SDec) => dec_signed_seq,
            Some(Format::Dec) => dec_seq,
            Some(Format::Bin) => bin_0b_seq,
            _ => any_seq,
        };

        let orig_input: String = match args.input {
            Some(fname) => fs::read_to_string(fname).expect("Invalid input filename"),
            None => {
                let mut inbuf = Vec::new();
                let mut stdin = io::stdin();
                let _ = stdin.read_to_end(&mut inbuf);
                match str::from_utf8(&inbuf) {
                    Ok(v) => v.to_string(),
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                }
            }
        };
        let input = orig_input.to_lowercase();
        let Ok((_, data)) = xer_parser(&input) else {
            panic!("Couldn't process input!")
        };
        data
    };
    let printer = match args.to {
        Some(Format::Escaped) => write_esc_hex,
        Some(Format::Hex) => write_hex,
        Some(Format::Bin) => write_bin,
        Some(Format::Java) => write_signed_0x_hex,
        Some(Format::SDec) => write_signed_dec,
        Some(Format::Dec) => write_dec,
        Some(Format::Raw) => write_raw,
        _ => write_0x_hex,
    };

    // https://stackoverflow.com/questions/22355273/writing-to-a-file-or-stdout-in-rust
    let mut out_writer = match args.output {
        Some(fname) => Box::new(File::create(fname).unwrap()) as Box<dyn Write>,
        _ => Box::new(io::stdout()) as Box<dyn Write>,
    };
    printer(data, &mut out_writer);
}
