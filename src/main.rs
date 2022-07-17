mod lexer;
mod parser;

use std::fs;

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Input files
    source: String,
    /// Print out tokens when they're lexed
    #[clap(short, long)]
    tokens: bool,
    /// Print out instructions when they're parsed
    #[clap(short, long)]
    instructions: bool,
}

fn main() {
    let args = Args::parse();
    if let Ok(contents) = fs::read_to_string(args.source) {
        let tokens = lexer::lex(contents);
        if args.tokens {
            dbg!(&tokens);
        }
        let instructions = parser::parse(tokens);
        if args.instructions {
            dbg!(&instructions);
        }
    }
}
