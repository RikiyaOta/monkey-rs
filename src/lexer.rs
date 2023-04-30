use crate::token::*;

#[derive(Debug, Default)]
pub struct Lexer {
    // 入力
    input: String,
    // 入力における現在の位置
    position: u32,
    // これから読み込む位置（現在の文字の次）
    read_position: u32,
    // 現在検査中の文字
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Self {
            input,
            ..Default::default()
        };

        lexer.read_char();

        lexer
    }

    pub fn next_token(&mut self) -> Token {
        let token = match self.ch {
            '=' => Self::new_token(TokenType::ASSIGN, self.ch),
            ';' => Self::new_token(TokenType::SEMICOLON, self.ch),
            '(' => Self::new_token(TokenType::LPAREN, self.ch),
            ')' => Self::new_token(TokenType::RPAREN, self.ch),
            ',' => Self::new_token(TokenType::COMMA, self.ch),
            '+' => Self::new_token(TokenType::PLUS, self.ch),
            '{' => Self::new_token(TokenType::LBRACE, self.ch),
            '}' => Self::new_token(TokenType::RBRACE, self.ch),
            '\0' => Self::new_token(TokenType::EOF, self.ch),
            _ => panic!("Unexpected token. To do later."),
        };

        self.read_char();

        token
    }

    pub fn new_token(token_type: TokenType, ch: char) -> Token {
        Token {
            r#type: token_type,
            literal: ch.to_string(),
        }
    }

    fn read_char(&mut self) {
        let len = self.input.len().try_into().unwrap();
        let read_position = self.read_position;
        if read_position >= len {
            self.ch = '\0';
        } else {
            self.ch = (&self.input)
                .chars()
                .nth(read_position.try_into().unwrap())
                .unwrap();
        }

        self.position += 1;
        self.read_position += 1;
    }
}

#[cfg(test)]
mod tests {

    use super::Lexer;
    use crate::token::*;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;".to_string();

        let test_tokens = [
            Token {
                r#type: TokenType::ASSIGN,
                literal: "=".to_string(),
            },
            Token {
                r#type: TokenType::PLUS,
                literal: "+".to_string(),
            },
            Token {
                r#type: TokenType::LPAREN,
                literal: "(".to_string(),
            },
            Token {
                r#type: TokenType::RPAREN,
                literal: ")".to_string(),
            },
            Token {
                r#type: TokenType::LBRACE,
                literal: "{".to_string(),
            },
            Token {
                r#type: TokenType::RBRACE,
                literal: "}".to_string(),
            },
            Token {
                r#type: TokenType::COMMA,
                literal: ",".to_string(),
            },
            Token {
                r#type: TokenType::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                r#type: TokenType::EOF,
                literal: "\0".to_string(),
            },
        ];

        let mut lexer = Lexer::new(input);

        for token in test_tokens.iter() {
            let tok = lexer.next_token();

            assert_eq!(tok.r#type, token.r#type);
            assert_eq!(tok.literal, token.literal);
        }
    }
}
