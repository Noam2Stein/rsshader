use rsshader_macros::ConstEq;

use crate::ir::{Bank, Id, LinkedShaderIr, TypeIr};

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
pub enum BuiltInFnIr {
    Neg {
        ty: &'static TypeIr,
    },
    Not {
        ty: &'static TypeIr,
    },

    Add {
        left: &'static TypeIr,
        right: &'static TypeIr,
    },
    Sub {
        left: &'static TypeIr,
        right: &'static TypeIr,
    },
    Mul {
        left: &'static TypeIr,
        right: &'static TypeIr,
    },
    Div {
        left: &'static TypeIr,
        right: &'static TypeIr,
    },
    Rem {
        left: &'static TypeIr,
        right: &'static TypeIr,
    },
    Shl {
        left: &'static TypeIr,
        right: &'static TypeIr,
    },
    Shr {
        left: &'static TypeIr,
        right: &'static TypeIr,
    },
    BitAnd {
        left: &'static TypeIr,
        right: &'static TypeIr,
    },
    BitOr {
        left: &'static TypeIr,
        right: &'static TypeIr,
    },
    BitXor {
        left: &'static TypeIr,
        right: &'static TypeIr,
    },

    Eq {
        ty: &'static TypeIr,
    },
    Ne {
        ty: &'static TypeIr,
    },
    Lt {
        ty: &'static TypeIr,
    },
    Le {
        ty: &'static TypeIr,
    },
    Ge {
        ty: &'static TypeIr,
    },

    And,
    Or,

    StructConstructor {
        ty: &'static TypeIr,
    },
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
