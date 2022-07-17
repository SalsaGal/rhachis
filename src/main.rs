mod lexer;
mod parser;

use std::fs;

use clap::Parser;

#[derive(Parser)]
struct Args {
    source: String,
}

fn main() {
    let args = Args::parse();
    if let Ok(contents) = fs::read_to_string(args.source) {
        let tokens = lexer::lex(contents);
        dbg!(&tokens);
        let instructions = parser::parse(tokens);
        dbg!(&instructions);
    }
}
