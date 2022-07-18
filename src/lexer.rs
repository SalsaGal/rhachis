use std::ops::Range;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CodePosition {
    pub line: usize,
    pub line_range: Range<usize>,
}

impl CodePosition {
    pub fn new(line: usize, line_range: Range<usize>) -> Self {
        Self { line, line_range }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub position: CodePosition,
    pub ty: TokenType,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    BraceOpen,
    BraceClose,
    Identifier(String),
}

pub fn lex(contents: String) -> Vec<Token> {
    let mut to_ret = Vec::new();
    let mut current_token = String::new();
    let mut line = 1;
    let mut line_char = 0;
    macro_rules! push_token {
        () => {
            if !current_token.is_empty() {
                to_ret.push(Token {
                    position: CodePosition::new(
                        line,
                        line_char - current_token.len()..line_char,
                    ),
                    ty: TokenType::Identifier(current_token.clone()),
                });
                current_token.clear();
            }
        }
    }

    for c in contents.chars() {
        match c {
            '{' => {
                push_token!();
                to_ret.push(Token {
                    position: CodePosition::new(line, line_char..line_char + 1),
                    ty: TokenType::BraceOpen,
                });
            },
            '}' => {
                push_token!();
                to_ret.push(Token {
                    position: CodePosition::new(line, line_char..line_char + 1),
                    ty: TokenType::BraceClose,
                });
            },
            _ => {
                if c.is_whitespace() {
                    push_token!();
                    if c == '\n' {
                        line += 1;
                        line_char = 0;
                        continue;
                    }
                } else {
                    current_token.push(c);
                }
            }
        }
        line_char += 1;
    }
    to_ret
}
