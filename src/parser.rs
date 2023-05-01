use crate::ast::{ExpressionStatement, LetStatement, Program};
use crate::lexer::Lexer;
use crate::token::Token;

#[derive(Debug, Default)]
pub struct Parser {
    // 字句解析器（参照の方がいいのか・・・？）
    lexer: Lexer,

    // 現在のトークン
    cur_token: Token,

    // 次のトークン
    peek_token: Token,
}

impl Parser {
    fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            ..Default::default()
        };

        parser.next_token();
        parser.next_token();

        parser
    }

    fn next_token(&mut self) {
        // やりたかったイメージはこう：
        // self.cur_token = self.peek_token;
        // self.peek_token = self.lexer.next_token();
        std::mem::swap(&mut self.cur_token, &mut self.peek_token);
        self.peek_token = self.lexer.next_token();
    }

    fn parse_program(&self) -> Program<LetStatement<ExpressionStatement>> {
        unimplemented!()
    }
}
