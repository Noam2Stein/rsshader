use rsshader_macros::ConstEq;

use crate::ir::Type;

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum Function {
    UserDefined {
        entry_point_info: Option<EntryPointInfo>,
        parameters: &'static [Variable],
        return_type: Option<&'static Type>,
        stmts: &'static [usize],
        expr_bank: &'static [Expr],
        stmt_bank: &'static [Stmt],
    },
    BuiltIn(BuiltInFunction),
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum BuiltInFunction {
    Neg(&'static Type),
    Not(&'static Type),

    Add(&'static Type, &'static Type),
    Sub(&'static Type, &'static Type),
    Mul(&'static Type, &'static Type),
    Div(&'static Type, &'static Type),
    Rem(&'static Type, &'static Type),
    Shl(&'static Type, &'static Type),
    Shr(&'static Type, &'static Type),
    BitAnd(&'static Type, &'static Type),
    BitOr(&'static Type, &'static Type),
    BitXor(&'static Type, &'static Type),

    Eq(&'static Type),
    Ne(&'static Type),
    Lt(&'static Type),
    Gt(&'static Type),
    Le(&'static Type),
    Ge(&'static Type),

    And,
    Or,
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum EntryPointInfo {
    Vertex(VertexFunctionInfo),
    Fragment(FragmentFunctionInfo),
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub struct VertexFunctionInfo {
    pub input_attrs: &'static [&'static Type],
    pub output_attrs: &'static [&'static Type],
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub struct FragmentFunctionInfo {
    pub input_attrs: &'static [&'static Type],
}

#[derive(Debug, Clone, Copy, ConstEq)]
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

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum Expr {
    Literal(Literal),
    Variable(Variable),
    FunctionCall {
        function: &'static Function,
        args: &'static [ExprId],
    },
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum Place {
    Variable(Variable),
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum Stmt {
    VariableDecl(Variable),
    Assignment(Place, Expr),
    Return(Option<Expr>),
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub struct ExprId(pub usize);

#[derive(Debug, Clone, Copy, ConstEq)]
pub struct StmtId(pub usize);

impl Literal {
    pub const fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Literal::F32(a), Literal::F32(b)) => a.to_bits() == b.to_bits(),
            (Literal::I32(a), Literal::I32(b)) => *a == *b,
            (Literal::U32(a), Literal::U32(b)) => *a == *b,
            (Literal::Bool(a), Literal::Bool(b)) => *a == *b,
            _ => false,
        }
    }
}
