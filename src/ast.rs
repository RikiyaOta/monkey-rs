use crate::token::Token;

pub trait Node {
    fn token_literal(&self) -> &str;
}

pub trait Statement: Node {
    fn statement_node();
}

pub trait Expression: Node {
    fn expression_node();
}

#[derive(Debug)]
struct LetStatement<T: Expression> {
    token: Token,
    name: Identifier,
    value: T,
}

impl Node for LetStatement<ExpressionStatement> {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl Statement for LetStatement<ExpressionStatement> {
    fn statement_node() {}
}

#[derive(Debug)]
struct Identifier {
    token: Token,
    value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl Expression for Identifier {
    fn expression_node() {}
}

// Expression trait を実装しようと思った型
// ただし、Expression は他の Expression を内包するので、同じく Expression trait を実装する型パラメータが必要になる。
// →つまり再帰的なデータ構造が必要になる！
// 素直に再帰的なデータ構造を定義しようとすると、コンパイル時にサイズが決定できないので、コンパイルできない。
// しかし、参照（ポインタ）ならサイズが確定する。
// これが Box<> の1つの使い所！The Rust Programming Language にも書いてある！
struct ExpressionStatement {
    token: Token,
    expression: Box<ExpressionStatement>,
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl Expression for ExpressionStatement {
    fn expression_node() {}
}

#[derive(Debug)]
pub struct Program<T: Statement> {
    statements: [T],
}

//impl Node for Program<T: Statement> {
//   fn token_literal(&self) -> String {
//       if self.statements.len() > 0 {
//            self.statements[0].token_literal()
//       } else {
//            "".to_string();
//       }
//   }
//}
