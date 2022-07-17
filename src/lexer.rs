use std::ops::Range;

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub line: usize,
    pub line_range: Range<usize>,
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
    for c in contents.chars() {
        match c {
            '{' => to_ret.push(Token {
                line,
                line_range: line_char..line_char + 1,
                ty: TokenType::BraceOpen,
            }),
            '}' => to_ret.push(Token {
                line,
                line_range: line_char..line_char + 1,
                ty: TokenType::BraceClose,
            }),
            _ => {
                if c.is_whitespace() {
                    if !current_token.is_empty() {
                        to_ret.push(Token {
                            line,
                            line_range: line_char - current_token.len()..line_char + 1,
                            ty: TokenType::Identifier(current_token.clone()),
                        });
                        current_token.clear();
                    }
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
