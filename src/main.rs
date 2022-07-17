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
        let tokens = lexer::lex(contents.clone());
        let instructions = parser::parse(tokens);
        match instructions {
            Ok(instructions) => {
                dbg!(instructions);
            }
            Err(err) => {
                eprintln!("ERROR: {:?}", err.ty);
                eprintln!(
                    "{}  |{}",
                    err.line,
                    contents.split('\n').nth(err.line - 1).unwrap()
                );
                eprintln!(
                    "   |{}{}",
                    " ".repeat(err.line_range.start),
                    "^".repeat(err.line_range.end - err.line_range.start)
                );
            }
        }
    }
}
