use either::Either;

use crate::lexer::{Token, TokenType};

#[derive(Debug)]
pub struct ParseError {
    pub ty: ParseErrorType,
    pub line: usize,
}

#[derive(Debug)]
pub enum ParseErrorType {
    UnexpectedToken,
    UnmatchedBrace,
}

#[derive(Clone, Debug)]
pub enum Instruction {
    Block {
        instructions: Vec<Instruction>,
    },
    Section {
        ident: String,
        instructions: Vec<Instruction>,
    },
}

pub fn parse(token: Vec<Token>) -> Result<Vec<Instruction>, ParseError> {
    let mut collection: Vec<Either<Token, Instruction>> =
        token.into_iter().map(Either::Left).collect();

    while collection.iter().any(Either::is_left) {
        for (index, item) in collection.iter().enumerate() {
            if let Either::Left(Token {
                ty: TokenType::BraceClose,
                line,
            }) = item {
                let brace_close = index;
                match find_last_brace(&collection, index) {
                    Some(brace_open) => {
                        let mut instructions = Vec::new();
                        for index in brace_open + 1..brace_close {
                            if let Either::Right(item) = &collection[index] {
                                instructions.push(item.clone());
                            } else {
                                return Err(ParseError {
                                    ty: ParseErrorType::UnexpectedToken,
                                    line: *line,
                                });
                            }
                        }

                        collection.drain(brace_open..brace_close + 1);
                        collection.insert(
                            brace_open,
                            Either::Right(Instruction::Block { instructions }),
                        );
                        break;
                    }
                    None => {
                        return Err(ParseError {
                            ty: ParseErrorType::UnmatchedBrace,
                            line: *line,
                        });
                    }
                }
            } else if let Either::Right(Instruction::Block { instructions }) = item {
                if let Some(Either::Left(Token {
                    ty: TokenType::Identifier(ident),
                    ..
                })) = collection.get(index - 1)
                {
                    let instructions = instructions.clone();
                    let ident = ident.clone();
                    collection.drain(index - 1..index + 1);
                    collection.insert(
                        index - 1,
                        Either::Right(Instruction::Section {
                            ident,
                            instructions,
                        }),
                    );
                    break;
                }
            }
        }
    }

    Ok(collection.into_iter().map(Either::unwrap_right).collect())
}

fn find_last_brace(collection: &[Either<Token, Instruction>], from: usize) -> Option<usize> {
    for i in (0..from).rev() {
        match &collection[i] {
            Either::Left(Token {
                ty: TokenType::BraceOpen,
                ..
            }) => {
                return Some(i);
            }
            _ => {}
        }
    }
    None
}
