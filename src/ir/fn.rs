use rsshader_macros::ConstEq;

use crate::ir::{Bank, Id, LengthIr, LinkedShaderIr, PrimitiveIr, TypeIr, VectorIr};

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum FnIr {
    UserDefined {
        params: &'static [VariableIr],
        ret_ty: Option<&'static TypeIr>,
        body: BodyIr,
    },
    BuiltIn(BuiltInFnIr),
}

#[derive(Debug, Clone, Copy, ConstEq)]

pub struct BodyIr {
    pub stmts: &'static [Id<StmtIr>],
    pub expr_bank: Bank<ExprIr>,
    pub stmt_bank: Bank<StmtIr>,
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub struct VariableIr {
    pub id: usize,
    pub ty: &'static TypeIr,
}

#[derive(Debug, Clone, Copy)]
pub enum LiteralIr {
    F32(f32),
    I32(i32),
    U32(u32),
    Bool(bool),
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum ExprIr {
    Literal(LiteralIr),
    Variable(VariableIr),
    FunctionCall {
        func: &'static FnIr,
        args: &'static [Id<ExprIr>],
    },
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum PlaceIr {
    Variable(VariableIr),
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum StmtIr {
    VariableDecl { var: VariableIr },
    Assignment { left: PlaceIr, right: ExprIr },
    Return { value: Option<ExprIr> },
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum BuiltInFnIr {
    Neg(NegFnIr),
    Not(NotFnIr),

    Add(AddFnIr),
    Sub(SubFnIr),
    Mul(MulFnIr),
    Div(DivFnIr),
    Rem(RemFnIr),
    Shl(ShlFnIr),
    Shr(ShrFnIr),
    BitAnd(BitAndFnIr),
    BitOr(BitOrFnIr),
    BitXor(BitXorFnIr),

    Eq(EqFnIr),
    Ne(EqFnIr),
    Lt(CmpFnIr),
    Gt(CmpFnIr),
    Le(CmpFnIr),
    Ge(CmpFnIr),
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum NegFnIr {
    F32,
    I32,
    U32,
    F32Vec { len: LengthIr },
    I32Vec { len: LengthIr },
    U32Vec { len: LengthIr },
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum NotFnIr {
    Bool,
    I32,
    U32,
    BoolVec { len: LengthIr },
    I32Vec { len: LengthIr },
    U32Vec { len: LengthIr },
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum AddFnIr {
    F32,
    I32,
    U32,
    F32Vec { len: LengthIr },
    I32Vec { len: LengthIr },
    U32Vec { len: LengthIr },
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum SubFnIr {
    F32,
    I32,
    U32,
    F32Vec { len: LengthIr },
    I32Vec { len: LengthIr },
    U32Vec { len: LengthIr },
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum MulFnIr {
    F32,
    I32,
    U32,
    F32Vec { len: LengthIr },
    I32Vec { len: LengthIr },
    U32Vec { len: LengthIr },
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum DivFnIr {
    F32,
    I32,
    U32,
    F32Vec { len: LengthIr },
    I32Vec { len: LengthIr },
    U32Vec { len: LengthIr },
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum RemFnIr {
    F32,
    I32,
    U32,
    F32Vec { len: LengthIr },
    I32Vec { len: LengthIr },
    U32Vec { len: LengthIr },
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum ShlFnIr {
    I32,
    U32,
    I32Vec { len: LengthIr },
    U32Vec { len: LengthIr },
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum ShrFnIr {
    I32,
    U32,
    I32Vec { len: LengthIr },
    U32Vec { len: LengthIr },
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum BitAndFnIr {
    Bool,
    I32,
    U32,
    BoolVec { len: LengthIr },
    I32Vec { len: LengthIr },
    U32Vec { len: LengthIr },
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum BitOrFnIr {
    Bool,
    I32,
    U32,
    BoolVec { len: LengthIr },
    I32Vec { len: LengthIr },
    U32Vec { len: LengthIr },
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum BitXorFnIr {
    Bool,
    I32,
    U32,
    BoolVec { len: LengthIr },
    I32Vec { len: LengthIr },
    U32Vec { len: LengthIr },
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum EqFnIr {
    Primitive(PrimitiveIr),
    Vector(VectorIr),
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum CmpFnIr {
    F32,
    I32,
    U32,
}

impl FnIr {
    pub const fn id(&self, shader: &LinkedShaderIr) -> usize {
        let mut i = 0;
        loop {
            if shader.fns[i].eq(self) {
                break i;
            }

            i += 1;
        }
    }
}

impl LiteralIr {
    pub const fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LiteralIr::F32(a), LiteralIr::F32(b)) => a.to_bits() == b.to_bits(),
            (LiteralIr::I32(a), LiteralIr::I32(b)) => *a == *b,
            (LiteralIr::U32(a), LiteralIr::U32(b)) => *a == *b,
            (LiteralIr::Bool(a), LiteralIr::Bool(b)) => *a == *b,
            _ => false,
        }
    }
}
