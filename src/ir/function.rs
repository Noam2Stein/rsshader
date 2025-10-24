use crate::ir::Type;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Function {
    pub entry_kind: Option<EntryKind>,
    pub parameters: &'static [Variable],
    pub return_type: Option<&'static Type>,
    pub stmts: &'static [usize],
    pub expr_bank: &'static [Expr],
    pub stmt_bank: &'static [Stmt],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryKind {
    Vertex,
    Fragment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Variable {
    pub id: usize,
    pub ty: &'static Type,
}

#[derive(Debug, Clone, Copy)]
pub enum Literal {
    F32(f32),
    I32(i32),
    U32(u32),
    Bool(bool),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Expr {
    Literal(Literal),
    Variable(Variable),
    FunctionCall {
        function: &'static Function,
        args: &'static [ExprId],
    },

    Neg(ExprId),
    Add(ExprId, ExprId),
    Sub(ExprId, ExprId),
    Mul(ExprId, ExprId),
    Div(ExprId, ExprId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Place {
    Variable(Variable),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stmt {
    VariableDecl(Variable),
    Assignment(Place, Expr),
    Return(Option<Expr>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExprId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StmtId(pub usize);

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Literal::F32(a), Literal::F32(b)) => a.to_bits() == b.to_bits(),
            (Literal::I32(a), Literal::I32(b)) => a == b,
            (Literal::U32(a), Literal::U32(b)) => a == b,
            (Literal::Bool(a), Literal::Bool(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for Literal {}
