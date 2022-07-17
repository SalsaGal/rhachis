#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub line: usize,
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
    for c in contents.chars() {
        match c {
            '{' => to_ret.push(Token {
                line,
                ty: TokenType::BraceOpen,
            }),
            '}' => to_ret.push(Token {
                line,
                ty: TokenType::BraceClose,
            }),
            _ => {
                if c.is_whitespace() {
                    if !current_token.is_empty() {
                        to_ret.push(Token {
                            line,
                            ty: TokenType::Identifier(current_token.clone()),
                        });
                        current_token.clear();
                    }
                    if c == '\n' {
                        line += 1;
                    }
                } else {
                    current_token.push(c);
                }
            }
        }
    }
    to_ret
}
