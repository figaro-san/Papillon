use clap::{Parser, Subcommand};
use colored::Colorize;
use std::process::exit;

mod calc;
use calc::CalcError;
mod patt;
use patt::{GenPattError, FindPattError};
mod readelf;
use readelf::{Elf64_Ehdr, Elf64ParseError};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Calculate expression or convert a number to Hexadecimal, Decimal or Binary.
    /// Both require a specific prefix to specify the number. (Excluding Decimal)
    /// Hex: 0x, Bin: 0b
    /// Example: calc '0xFF - 0b1101 + 256'
    #[clap(verbatim_doc_comment)]
    Calc {
        number: String,
    },

    /// Generate De Bruijn Sequence of a given length and subsequence length.
    Patt {
        /// Length of the sequence to generate
        len: usize,
        /// Length of the subsequences
        n: usize,
    },

    /// Calculate the offset of subsequence in de_bruijn sequence
    Find {
        /// Original sequence to calculate the offset of the location where subsequence exists
        de_bruijn_sequence: String,
        /// subsequence is subsequence in `de_bruijn_sequence`
        subsequence: String,
    },

    /// Read ELF format file information
    Readelf {
        /// target binary
        filepath: String,
    }
}

fn main() {
    print_banner();

    let cli = Cli::parse();
    match &cli.command {
        Commands::Calc {number} => {
            match calc::calc(number.as_str()) {
                Ok(num) => {
                    calc::display_number(num);
                },

                Err(CalcError::InvalidExpression) => {
                    eprintln!("[{}] Invalid Expression \"{}\"", "Error".red(), number.as_str());
                    exit(1);
                }
            }
        }

        Commands::Patt {len, n} => {
            match patt::gen_patt(*len, *n) {
                Ok(patt) => {
                    println!("[{}] Generated sequence: {}", "Patt".green(), patt);
                },

                Err(GenPattError::InvalidN) => {           
                    eprintln!("[{}] Invalid number of n", "Error".red());
                    eprintln!("[{}] The range of n is \"0 < n <= 26\"", "+".green());
                    exit(1);
                },

                Err(GenPattError::InvalidLen) => {
                    eprintln!("[{}] Invalid number of len", "Error".red());
                    eprintln!("[{}] The range of len is \"0 < len <= 26^n\"", "Error".red());
                    exit(1);           
                },
            };
        },

        Commands::Find {de_bruijn_sequence, subsequence} => {
            match patt::find_offset(de_bruijn_sequence.as_str(), subsequence.as_str()) {
                Ok(offset) => {
                    println!("[{}] Offset is: {}", "Find".green(), offset);
                },

                Err(FindPattError::PatternNotFound) => {
                    eprintln!("[{}]: Pattern not found", "Error".red());
                    exit(1);
                },
            }
        },

        Commands::Readelf { filepath } => {
            let data = std::fs::read(filepath).unwrap();
            let elf_header = match Elf64_Ehdr::parse_elf_header(&data) {
                Ok(eh) => eh,
                Err(err) => {
                    match err {
                        Elf64ParseError::NotElfFile => {
                            eprintln!("[{}] File is not ELF format", "Error".red());
                            exit(1);
                        },

                        Elf64ParseError::InvalidElfClass => {
                            eprintln!("[{}] Invalid ELF class found", "Error".red());
                            exit(1);
                        },

                        Elf64ParseError::InvalidEndian => {
                            eprintln!("[{}] Invalid endian found", "Error".red());
                            exit(1);
                        },

                        Elf64ParseError::InvalidElfVersion => {
                            eprintln!("[{}] Invalid ELF version found", "Error".red());
                            exit(1);
                        },

                        Elf64ParseError::InvalidObjectFileType => {
                            eprintln!("[{}] Invalid Object file type found", "Error".red());
                            exit(1);
                        },
                    }
                }
            };
            elf_header.print_elf_header();
        }
    }
}

fn print_banner() {
    let p = [
        ["┌─┐ "], 
        ["│─┘ "], 
        ["┴   "], 
    ];

    let a = [
        ["┌─┐ "],
        ["├─┤ "],
        ["┴ ┴ "],
    ];

    let i = [
        ["┬ "], 
        ["│ "], 
        ["┴ "]
    ];

    let l = [
        ["┬   "],
        ["│   "],
        ["┴─┘ "]
    ];

    let o = [
        ["┌─┐ "],
        ["│ │ "],
        ["└─┘ "],
    ];

    let n = [
        ["┌┐┌ "],
        ["│││ "],
        ["┘└┘ "],
    ];

    let banner = [p, a, p, i, l, l, o, n];
    let init_color = 91; // inc: 36
    let mut cnt = 0;

    for row in 0..=2 {
        let mut text_color = init_color;
        for char in banner {
            let color_code = format!("\x1b[38;5;{}m", text_color);
            print!("{}{}\x1b[m", color_code, char[row][0]);
            cnt += 1;
            if cnt % 2 == 0 {
                text_color += 36;
            }
        }
        println!("");
    }
}
