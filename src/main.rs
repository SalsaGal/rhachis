mod lexer;
mod parser;

use std::fs;
use std::io::Write;

use clap::Parser;
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};

#[derive(Parser)]
struct Args {
    /// Input files
    source: String,
}

fn main() {
    let args = Args::parse();
    let mut stderr = StandardStream::stderr(termcolor::ColorChoice::Always);
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

                    stderr
                        .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
                        .unwrap();
                    write!(&mut stderr, "ERROR").unwrap();
                    stderr.reset().unwrap();
                    eprintln!(": {:?}", err.ty);
                    eprintln!(
                        "{} |{}",
                        line_number,
                        contents.split('\n').nth(err.line - 1).unwrap()
                    );
                    eprintln!(
                        "{} |{}{}\n",
                        " ".repeat(line_number.len()),
                        " ".repeat(err.line_range.start),
                        "^".repeat(err.line_range.end - err.line_range.start)
                    );
                }
            }
        }
    }
}
