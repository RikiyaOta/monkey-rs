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

// TODO: Impl.
#[derive(Debug, Clone)]
pub struct ParseError;

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

    // TODO: これだと、LetStatement しか解析できないなぁ。
    //fn parse_program(&self) -> Program<LetStatement<ExpressionStatement>> {
    fn parse_program(&self) -> Result<Program<LetStatement<ExpressionStatement>>, ParseError> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::ast::{ExpressionStatement, LetStatement, Node, Statement};
    use crate::lexer::Lexer;

    #[test]
    fn test_let_statements() {
        let input = r#"let x = 5;
        let y = 10;
        let foobar = 838383;
        "#
        .to_string();

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);
        let parse_result = parser.parse_program();

        assert!(parse_result.is_ok());

        let program = parse_result.unwrap();

        assert_eq!(program.statements.len(), 3);

        let expected_identifiers = ["x".to_string(), "y".to_string(), "foobar".to_string()];

        for (i, expected) in expected_identifiers.iter().enumerate() {
            assert!(test_let_statement(&program.statements[i], expected));
        }
    }

    // テストのヘルパー
    fn test_let_statement(
        let_statement: &LetStatement<ExpressionStatement>,
        name: &String,
    ) -> bool {
        if let_statement.token_literal() != "let" {
            return false;
        }

        if let_statement.name.value != *name {
            return false;
        }

        if let_statement.name.token_literal() != name {
            return false;
        }

        true
    }
}
