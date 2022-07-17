#[derive(Debug, PartialEq, Eq)]
pub struct Token {
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
    for c in contents.chars() {
        match c {
            '{' => to_ret.push(Token {
                ty: TokenType::BraceOpen,
            }),
            '}' => to_ret.push(Token {
                ty: TokenType::BraceClose,
            }),
            _ => {
                if c.is_whitespace() {
                    if !current_token.is_empty() {
                        to_ret.push(Token {
                            ty: TokenType::Identifier(current_token.clone()),
                        });
                        current_token.clear();
                    }
                } else {
                    current_token.push(c);
                }
            }
        }
    }
    to_ret
}
