mod lexer;
mod parser;

use std::fs;
use std::io::Write;

use clap::Parser;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(Parser)]
struct Args {
    /// Input files
    source: String,
    /// Whether to show color or not, can be auto, none, or always
    #[clap(short, long)]
    color: Option<String>,
}

fn main() {
    let args = Args::parse();
    let mut stderr =
        StandardStream::stderr(match &*args.color.unwrap_or_else(|| "auto".to_owned()) {
            "always" => ColorChoice::Always,
            "auto" => ColorChoice::Auto,
            "never" => ColorChoice::Never,
            _ => panic!("Invalid color option"),
        });
    if let Ok(contents) = fs::read_to_string(args.source) {
        let tokens = lexer::lex(contents.clone());
        dbg!(&tokens);
        let instructions = parser::parse(tokens);
        match instructions {
            Ok(instructions) => {
                dbg!(instructions);
            }
            Err(errs) => {
                for err in errs {
                    let line_number = err.position.line.to_string();

                    stderr
                        .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
                        .unwrap();
                    write!(&mut stderr, "ERROR").unwrap();
                    stderr.reset().unwrap();
                    eprintln!(": {:?}", err.ty);
                    eprintln!(
                        "{} |{}",
                        line_number,
                        contents.split('\n').nth(err.position.line - 1).unwrap()
                    );
                    stderr
                        .set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))
                        .unwrap();
                    eprintln!(
                        "{}  {}{}",
                        " ".repeat(line_number.len()),
                        " ".repeat(err.position.line_range.start),
                        "^".repeat(err.position.line_range.end - err.position.line_range.start)
                    );
                    stderr.reset().unwrap();
                }
            }
        }
    }
}
