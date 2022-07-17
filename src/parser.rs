use either::Either;

use crate::lexer::Token;

#[derive(Debug)]
pub enum ParseError {
    UnmatchedBrace,
}

#[derive(Debug)]
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
            if matches!(item, Either::Left(Token::BraceClose)) {
                if let Some(brace_open) = find_last_brace(&collection, index) {
                    let brace_close = index;
                    collection.drain(brace_open..brace_close + 1);
                    collection.insert(
                        brace_open,
                        Either::Right(Instruction::Block {
                            instructions: vec![],
                        }),
                    );
                    dbg!(&collection);
                    break;
                } else {
                    return Err(ParseError::UnmatchedBrace);
                }
            }
        }
    }

    Ok(collection.into_iter().map(Either::unwrap_right).collect())
}

fn find_last_brace(collection: &[Either<Token, Instruction>], from: usize) -> Option<usize> {
    for i in (0..from).rev() {
        if matches!(collection[i], Either::Left(Token::BraceOpen)) {
            return Some(i);
        }
    }
    None
}
