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
        self.skip_whitespace();

        let token = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = ch.to_string() + &self.ch.to_string();
                    Token {
                        r#type: TokenType::EQ,
                        literal,
                    }
                } else {
                    Self::new_token(TokenType::ASSIGN, self.ch)
                }
            }
            ';' => Self::new_token(TokenType::SEMICOLON, self.ch),
            '(' => Self::new_token(TokenType::LPAREN, self.ch),
            ')' => Self::new_token(TokenType::RPAREN, self.ch),
            ',' => Self::new_token(TokenType::COMMA, self.ch),
            '+' => Self::new_token(TokenType::PLUS, self.ch),
            '-' => Self::new_token(TokenType::MINUS, self.ch),
            '!' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = ch.to_string() + &self.ch.to_string();
                    Token {
                        r#type: TokenType::NEQ,
                        literal,
                    }
                } else {
                    Self::new_token(TokenType::BANG, self.ch)
                }
            }
            '/' => Self::new_token(TokenType::SLASH, self.ch),
            '*' => Self::new_token(TokenType::ASTERISK, self.ch),
            '<' => Self::new_token(TokenType::LT, self.ch),
            '>' => Self::new_token(TokenType::GT, self.ch),
            '{' => Self::new_token(TokenType::LBRACE, self.ch),
            '}' => Self::new_token(TokenType::RBRACE, self.ch),
            '\0' => Self::new_token(TokenType::EOF, self.ch),
            _ => {
                if Self::is_letter(self.ch) {
                    let literal = self.read_identifier();
                    Token {
                        r#type: lookup_ident(&literal),
                        literal,
                    }
                } else if Self::is_digit(self.ch) {
                    Token {
                        r#type: TokenType::INT,
                        literal: self.read_number(),
                    }
                } else {
                    Self::new_token(TokenType::ILLEGAL, self.ch)
                }
            }
        };

        // ATTENTION:
        // 2文字以上のキーワードや数字については、すでに次の文字に進んでいるので、その場合は read_char() は実行しない。
        // TODO: もっと簡潔にこれを実現したい！
        if token.r#type != TokenType::INT
            && token.r#type != TokenType::IDENT
            && token.r#type != TokenType::FUNCTION
            && token.r#type != TokenType::LET
            && token.r#type != TokenType::TRUE
            && token.r#type != TokenType::FALSE
            && token.r#type != TokenType::IF
            && token.r#type != TokenType::ELSE
            && token.r#type != TokenType::RETURN
        {
            self.read_char();
        }

        token
    }

    fn read_identifier(&mut self) -> String {
        let mut char_store = vec![];

        while Self::is_letter(self.ch) {
            char_store.push(self.ch);
            self.read_char();
        }

        char_store.iter().collect()
    }

    fn read_number(&mut self) -> String {
        let mut digit_store = vec![];

        while Self::is_digit(self.ch) {
            digit_store.push(self.ch);
            self.read_char();
        }

        digit_store.iter().collect()
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }

    fn is_letter(ch: char) -> bool {
        ('a' <= ch && ch <= 'z') || ('A' <= ch && ch <= 'Z') || ch == '_'
    }

    fn is_digit(ch: char) -> bool {
        '0' <= ch && ch <= '9'
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

    // NOTE: read_char とほぼ同じ。
    // こちらは、覗き見（peek）をするだけ。いわゆる『先読み』する処理。
    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len().try_into().unwrap() {
            '\0'
        } else {
            self.input
                .chars()
                .nth(self.read_position.try_into().unwrap())
                .unwrap()
        }
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

    #[test]
    fn test_next_token_2() {
        let input = r#"let five = 5;
        let ten = 10;

        let add = fn(x, y) {
            x + y;
        };
        
        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;
        
        if (5 < 10) {
            return true;
        } else {
            return false;
        }
        
        10 == 10;
        10 != 9;
       "#
        .to_string();

        let test_tokens = [
            Token {
                r#type: TokenType::LET,
                literal: "let".to_string(),
            },
            Token {
                r#type: TokenType::IDENT,
                literal: "five".to_string(),
            },
            Token {
                r#type: TokenType::ASSIGN,
                literal: "=".to_string(),
            },
            Token {
                r#type: TokenType::INT,
                literal: "5".to_string(),
            },
            Token {
                r#type: TokenType::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                r#type: TokenType::LET,
                literal: "let".to_string(),
            },
            Token {
                r#type: TokenType::IDENT,
                literal: "ten".to_string(),
            },
            Token {
                r#type: TokenType::ASSIGN,
                literal: "=".to_string(),
            },
            Token {
                r#type: TokenType::INT,
                literal: "10".to_string(),
            },
            Token {
                r#type: TokenType::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                r#type: TokenType::LET,
                literal: "let".to_string(),
            },
            Token {
                r#type: TokenType::IDENT,
                literal: "add".to_string(),
            },
            Token {
                r#type: TokenType::ASSIGN,
                literal: "=".to_string(),
            },
            Token {
                r#type: TokenType::FUNCTION,
                literal: "fn".to_string(),
            },
            Token {
                r#type: TokenType::LPAREN,
                literal: "(".to_string(),
            },
            Token {
                r#type: TokenType::IDENT,
                literal: "x".to_string(),
            },
            Token {
                r#type: TokenType::COMMA,
                literal: ",".to_string(),
            },
            Token {
                r#type: TokenType::IDENT,
                literal: "y".to_string(),
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
                r#type: TokenType::IDENT,
                literal: "x".to_string(),
            },
            Token {
                r#type: TokenType::PLUS,
                literal: "+".to_string(),
            },
            Token {
                r#type: TokenType::IDENT,
                literal: "y".to_string(),
            },
            Token {
                r#type: TokenType::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                r#type: TokenType::RBRACE,
                literal: "}".to_string(),
            },
            Token {
                r#type: TokenType::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                r#type: TokenType::LET,
                literal: "let".to_string(),
            },
            Token {
                r#type: TokenType::IDENT,
                literal: "result".to_string(),
            },
            Token {
                r#type: TokenType::ASSIGN,
                literal: "=".to_string(),
            },
            Token {
                r#type: TokenType::IDENT,
                literal: "add".to_string(),
            },
            Token {
                r#type: TokenType::LPAREN,
                literal: "(".to_string(),
            },
            Token {
                r#type: TokenType::IDENT,
                literal: "five".to_string(),
            },
            Token {
                r#type: TokenType::COMMA,
                literal: ",".to_string(),
            },
            Token {
                r#type: TokenType::IDENT,
                literal: "ten".to_string(),
            },
            Token {
                r#type: TokenType::RPAREN,
                literal: ")".to_string(),
            },
            Token {
                r#type: TokenType::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                r#type: TokenType::BANG,
                literal: "!".to_string(),
            },
            Token {
                r#type: TokenType::MINUS,
                literal: "-".to_string(),
            },
            Token {
                r#type: TokenType::SLASH,
                literal: "/".to_string(),
            },
            Token {
                r#type: TokenType::ASTERISK,
                literal: "*".to_string(),
            },
            Token {
                r#type: TokenType::INT,
                literal: "5".to_string(),
            },
            Token {
                r#type: TokenType::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                r#type: TokenType::INT,
                literal: "5".to_string(),
            },
            Token {
                r#type: TokenType::LT,
                literal: "<".to_string(),
            },
            Token {
                r#type: TokenType::INT,
                literal: "10".to_string(),
            },
            Token {
                r#type: TokenType::GT,
                literal: ">".to_string(),
            },
            Token {
                r#type: TokenType::INT,
                literal: "5".to_string(),
            },
            Token {
                r#type: TokenType::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                r#type: TokenType::IF,
                literal: "if".to_string(),
            },
            Token {
                r#type: TokenType::LPAREN,
                literal: "(".to_string(),
            },
            Token {
                r#type: TokenType::INT,
                literal: "5".to_string(),
            },
            Token {
                r#type: TokenType::LT,
                literal: "<".to_string(),
            },
            Token {
                r#type: TokenType::INT,
                literal: "10".to_string(),
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
                r#type: TokenType::RETURN,
                literal: "return".to_string(),
            },
            Token {
                r#type: TokenType::TRUE,
                literal: "true".to_string(),
            },
            Token {
                r#type: TokenType::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                r#type: TokenType::RBRACE,
                literal: "}".to_string(),
            },
            Token {
                r#type: TokenType::ELSE,
                literal: "else".to_string(),
            },
            Token {
                r#type: TokenType::LBRACE,
                literal: "{".to_string(),
            },
            Token {
                r#type: TokenType::RETURN,
                literal: "return".to_string(),
            },
            Token {
                r#type: TokenType::FALSE,
                literal: "false".to_string(),
            },
            Token {
                r#type: TokenType::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                r#type: TokenType::RBRACE,
                literal: "}".to_string(),
            },
            Token {
                r#type: TokenType::INT,
                literal: "10".to_string(),
            },
            Token {
                r#type: TokenType::EQ,
                literal: "==".to_string(),
            },
            Token {
                r#type: TokenType::INT,
                literal: "10".to_string(),
            },
            Token {
                r#type: TokenType::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                r#type: TokenType::INT,
                literal: "10".to_string(),
            },
            Token {
                r#type: TokenType::NEQ,
                literal: "!=".to_string(),
            },
            Token {
                r#type: TokenType::INT,
                literal: "9".to_string(),
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

            println!("tok: {:?}, token: {:?}", tok, token);

            assert_eq!(tok.r#type, token.r#type);
            assert_eq!(tok.literal, token.literal);
        }
    }
}
