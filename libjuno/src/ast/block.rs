use super::{JunoSpan, Stmt};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Block {
    pub span: JunoSpan,
    pub stmts: Vec<Stmt>,
}
