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
            '=' => Self::new_token(ASSIGN.to_string(), self.ch),
            ';' => Self::new_token(SEMICOLON.to_string(), self.ch),
            '(' => Self::new_token(LPAREN.to_string(), self.ch),
            ')' => Self::new_token(RPAREN.to_string(), self.ch),
            ',' => Self::new_token(COMMA.to_string(), self.ch),
            '+' => Self::new_token(PLUS.to_string(), self.ch),
            '{' => Self::new_token(LBRACE.to_string(), self.ch),
            '}' => Self::new_token(RBRACE.to_string(), self.ch),
            '\0' => Self::new_token(EOF.to_string(), self.ch),
            _ => panic!("Unexpected token."),
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
        // 入力を動的に受け付けるので、String の方が適している気がする。
        let input = "=+(){},;".to_string();

        let test_tokens = [
            Token {
                r#type: ASSIGN.to_string(),
                literal: "=".to_string(),
            },
            Token {
                r#type: PLUS.to_string(),
                literal: "+".to_string(),
            },
            Token {
                r#type: LPAREN.to_string(),
                literal: "(".to_string(),
            },
            Token {
                r#type: RPAREN.to_string(),
                literal: ")".to_string(),
            },
            Token {
                r#type: LBRACE.to_string(),
                literal: "{".to_string(),
            },
            Token {
                r#type: RBRACE.to_string(),
                literal: "}".to_string(),
            },
            Token {
                r#type: COMMA.to_string(),
                literal: ",".to_string(),
            },
            Token {
                r#type: SEMICOLON.to_string(),
                literal: ";".to_string(),
            },
            Token {
                r#type: EOF.to_string(),
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
