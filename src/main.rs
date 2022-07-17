mod lexer;
mod parser;

use std::fs;

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Input files
    source: String,
}

fn main() {
    let args = Args::parse();
    if let Ok(contents) = fs::read_to_string(args.source) {
        let tokens = lexer::lex(contents);
        let instructions = parser::parse(tokens);
        match instructions {
            Ok(instructions) => {
                dbg!(instructions);
            }
            Err(err) => {
                dbg!(err);
            }
        }
    }
}
