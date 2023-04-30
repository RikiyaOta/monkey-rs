use crate::lexer::Lexer;
use crate::token::Token;

#[derive(Debug, Default)]
pub struct Parser {
    lexer: Lexer,
    curToken: Token,
    peekToken: Token,
}

impl Parser {
    fn new(lexer: Lexer) -> Self {
        // TODO:
        // デフォルト値じゃだめか。lexer の保持してる情報に応じてセットした方が良いか？
        let p = Parser {
            lexer,
            ..Default::default()
        };
    }

    fn next_token(&mut self) {
        self.curToken = self.peekToken;
        self.peekToken = self.lexer.next_token();
    }
}
