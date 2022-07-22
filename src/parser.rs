use either::Either;

use crate::lexer::{CodePosition, Token, TokenType};

#[derive(Debug)]
pub struct ParseError {
    pub ty: ParseErrorType,
    pub position: CodePosition,
}

#[derive(Debug)]
pub enum ParseErrorType {
    NestedSection,
    UnexpectedToken,
    UnmatchedBrace,
    UnmatchedParen,
}

#[derive(Clone, Debug)]
pub enum Instruction {
    Block {
        instructions: Vec<Instruction>,
    },
    Function {
        ident: String,
        captures: Captures,
        instructions: Vec<Instruction>,
    },
    FunctionCaptures {
        captures: Captures,
    },
    Section {
        ident: String,
        ident_position: CodePosition,
        instructions: Vec<Instruction>,
    },
}

#[derive(Clone, Debug)]
pub enum Captures {
    List(Vec<String>),
    Star,
}

fn collection_backcheck(
    collection: &[Either<Token, Instruction>],
    index: usize,
    offset: usize,
) -> Option<&Either<Token, Instruction>> {
    if offset > index {
        None
    } else {
        collection.get(index - offset)
    }
}

pub fn parse(token: Vec<Token>) -> Result<Vec<Instruction>, Vec<ParseError>> {
    let mut collection: Vec<Either<Token, Instruction>> =
        token.into_iter().map(Either::Left).collect();
    let mut errors = Vec::new();

    while collection.iter().any(Either::is_left) {
        dbg!(&collection);
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
                                    Either::Right(Instruction::Section {
                                        ident_position, ..
                                    }) => {
                                        errors.push(ParseError {
                                            ty: ParseErrorType::NestedSection,
                                            position: ident_position.clone(),
                                        });
                                    }
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
                Either::Left(Token {
                    ty: TokenType::ParenClose,
                    position,
                }) => {
                    let paren_close = index;
                    match find_last(&collection, index, TokenType::ParenOpen) {
                        Some(paren_open) => {
                            let captures = Captures::List(vec![]);
                            for item in collection.iter().take(paren_close).skip(paren_open + 1) {}
                            collection.drain(paren_open..paren_close + 1);
                            collection.insert(
                                paren_open,
                                Either::Right(Instruction::FunctionCaptures { captures }),
                            );
                            changed = true;
                            break;
                        }
                        None => {
                            errors.push(ParseError {
                                ty: ParseErrorType::UnmatchedParen,
                                position: position.clone(),
                            });
                        }
                    }
                }
                Either::Right(Instruction::Block { instructions }) => {
                    if let Some(Either::Left(Token {
                        ty: TokenType::Identifier(ident),
                        position,
                        ..
                    })) = collection_backcheck(&collection, index, 1)
                    {
                        // Section declaration
                        let ident = ident.clone();
                        let ident_position = position.clone();
                        let instructions = instructions.clone();
                        collection.drain(index - 1..index + 1);
                        collection.insert(
                            index - 1,
                            Either::Right(Instruction::Section {
                                ident,
                                ident_position,
                                instructions,
                            }),
                        );
                        changed = true;
                        break;
                    } else if let Some(Either::Right(Instruction::FunctionCaptures { captures })) =
                        collection_backcheck(&collection, index, 1)
                    {
                        if let Some(Either::Left(Token {
                            ty: TokenType::Identifier(ident),
                            ..
                        })) = collection_backcheck(&collection, index, 2)
                        {
                            if let Some(Either::Left(Token {
                                ty: TokenType::Function,
                                ..
                            })) = collection_backcheck(&collection, index, 3)
                            {
                                let ident = ident.clone();
                                let instructions = instructions.clone();
                                let captures = captures.clone();
                                collection.drain(index - 3..index + 1);
                                collection.insert(
                                    index - 3,
                                    Either::Right(Instruction::Function {
                                        ident,
                                        captures,
                                        instructions,
                                    }),
                                );
                                changed = true;
                                break;
                            }
                        }
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
        // Check for tokens
        match item {
            Either::Left(item) => errors.push(ParseError {
                ty: ParseErrorType::UnexpectedToken,
                position: item.position,
            }),
            Either::Right(item) => to_ret.push(item),
        }
    }
    if errors.is_empty() {
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
