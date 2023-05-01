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
pub struct LetStatement<T: Expression> {
    token: Token,
    pub name: Identifier,
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
pub struct Identifier {
    pub token: Token,
    pub value: String,
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
pub struct ExpressionStatement {
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
    pub statements: Box<[T]>,
}
