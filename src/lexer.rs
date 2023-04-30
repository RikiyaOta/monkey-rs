use crate::token::Token;

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
    fn new(input: String) -> Self {
        let mut lexer = Self {
            input,
            ..Default::default()
        };

        lexer.read_char();

        lexer
    }

    fn next_token(&self) -> Token {
        unimplemented!()
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

        let tests = [
            Token {
                r#type: ASSIGN,
                literal: "=",
            },
            Token {
                r#type: PLUS,
                literal: "+",
            },
            Token {
                r#type: LPAREN,
                literal: "(",
            },
            Token {
                r#type: RPAREN,
                literal: ")",
            },
            Token {
                r#type: LBRACE,
                literal: "{",
            },
            Token {
                r#type: RBRACE,
                literal: "}",
            },
            Token {
                r#type: COMMA,
                literal: ",",
            },
            Token {
                r#type: SEMICOLON,
                literal: ";",
            },
            Token {
                r#type: EOF,
                literal: "",
            },
        ];

        let lexer = Lexer::new(input);

        for (i, token) in tests.iter().enumerate() {
            let tok = lexer.next_token();

            assert_eq!(tok.r#type, token.r#type);
            assert_eq!(tok.literal, token.literal);
        }
    }
}
