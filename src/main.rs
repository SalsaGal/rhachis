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
            Err(errs) => {
                for err in errs {
                    let line_number = err.line.to_string();

                    eprintln!("ERROR: {:?}", err.ty);
                    eprintln!(
                        "{} |{}",
                        line_number,
                        contents.split('\n').nth(err.line - 1).unwrap()
                    );
                    eprintln!(
                        "{} |{}{}",
                        " ".repeat(line_number.len()),
                        " ".repeat(err.line_range.start),
                        "^".repeat(err.line_range.end - err.line_range.start)
                    );
                }
            }
        }
    }
}
