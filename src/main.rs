use clap::{Parser, Subcommand};
use colored::Colorize;
use std::process::exit;

mod calc;
use calc::{CalcError};
mod patt;
use patt::{GenPattError, FindPattError};

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
        de_bruijn_sequence: String,
        subsequence: String,
    },
}

fn main() {
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

                Err(FindPattError::NegativeOffset) => {
                    eprintln!("[{}]: Offset can not be negative", "Error".red());
                    exit(1);
                },
            }
        }
    }
}
