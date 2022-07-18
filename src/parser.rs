use either::Either;

use crate::lexer::{CodePosition, Token, TokenType};

#[derive(Debug)]
pub struct ParseError {
    pub ty: ParseErrorType,
    pub position: CodePosition,
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

pub fn parse(token: Vec<Token>) -> Result<Vec<Instruction>, Vec<ParseError>> {
    let mut collection: Vec<Either<Token, Instruction>> =
        token.into_iter().map(Either::Left).collect();
    let mut errors = Vec::new();

    while collection.iter().any(Either::is_left) {
        let mut changed = false;
        for (index, item) in collection.iter().enumerate() {
            match item {
                Either::Left(Token {
                    ty: TokenType::BraceClose,
                    position,
                }) => {
                    let brace_close = index;
                    match find_last(&collection, index, TokenType::BraceOpen) {
                        Some(brace_open) => {
                            let mut instructions = Vec::new();
                            for item in collection.iter().take(brace_close).skip(brace_open + 1) {
                                match item {
                                    Either::Right(item) => {
                                        instructions.push(item.clone());
                                    }
                                    Either::Left(token) => {
                                        errors.push(ParseError {
                                            ty: ParseErrorType::UnexpectedToken,
                                            position: token.position.clone(),
                                        });
                                    }
                                }
                            }

                            collection.drain(brace_open..brace_close + 1);
                            collection.insert(
                                brace_open,
                                Either::Right(Instruction::Block { instructions }),
                            );
                            changed = true;
                            break;
                        }
                        None => {
                            errors.push(ParseError {
                                ty: ParseErrorType::UnmatchedBrace,
                                position: position.clone(),
                            });
                        }
                    }
                }
                Either::Right(Instruction::Block { instructions }) => {
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
                        changed = true;
                        break;
                    }
                }
                _ => {}
            }
        }
        if !changed {
            break;
        }
    }

    let mut to_ret = Vec::new();
    for item in collection {
        match item {
            Either::Left(item) => errors.push(ParseError {
                ty: ParseErrorType::UnexpectedToken,
                position: item.position,
            }),
            Either::Right(item) => to_ret.push(item),
        }
    }
    if errors.is_empty() {
        // Check for tokens
        Ok(to_ret)
    } else {
        Err(errors)
    }
}

fn find_last(
    collection: &[Either<Token, Instruction>],
    from: usize,
    find: TokenType,
) -> Option<usize> {
    for i in (0..from).rev() {
        if let Either::Left(Token { ty, .. }) = &collection[i] {
            if *ty == find {
                return Some(i);
            }
        }
    }
    None
}
