use crate::token::Token;

pub struct Lexer {
    // 入力
    input: String,
    // 入力における現在の位置
    position: u32,
    // これから読み込む位置（現在の文字の次）
    readPosition: u32,
    // 現在検査中の文字
    ch: char,
}

impl Lexer {
    fn new(_input: String) -> Self {
        unimplemented!()
    }

    fn next_token(&self) -> Token {
        unimplemented!()
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
