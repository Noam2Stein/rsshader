use rsshader_macros::ConstEq;

use crate::ir::{
    Bank, BitwisePrimitiveIr, Id, IntIr, LengthIr, LinkedShaderIr, NumIr, PrimitiveIr, SignedNumIr,
    TypeIr, VectorIr,
};

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
    ScalarNeg { ty: SignedNumIr },
    VectorNeg { n: LengthIr, t: SignedNumIr },

    ScalarNot { ty: BitwisePrimitiveIr },
    VectorNot { n: LengthIr, t: BitwisePrimitiveIr },

    ScalarAdd { ty: NumIr },
    VectorAdd { n: LengthIr, t: NumIr },

    ScalarSub { ty: NumIr },
    VectorSub { n: LengthIr, t: NumIr },

    ScalarMul { ty: NumIr },
    VectorMul { n: LengthIr, t: NumIr },

    ScalarDiv { ty: NumIr },
    VectorDiv { n: LengthIr, t: NumIr },

    ScalarRem { ty: NumIr },
    VectorRem { n: LengthIr, t: NumIr },

    ScalarShl { ty: IntIr },
    VectorShl { n: LengthIr, t: IntIr },

    ScalarShr { ty: IntIr },
    VectorShr { n: LengthIr, t: IntIr },

    ScalarBitAnd { ty: BitwisePrimitiveIr },
    VectorBitAnd { n: LengthIr, t: BitwisePrimitiveIr },

    ScalarBitOr { ty: BitwisePrimitiveIr },
    VectorBitOr { n: LengthIr, t: BitwisePrimitiveIr },

    ScalarBitXor { ty: BitwisePrimitiveIr },
    VectorBitXor { n: LengthIr, t: BitwisePrimitiveIr },

    ScalarEq { ty: PrimitiveIr },
    VectorEq { ty: VectorIr },

    ScalarNe { ty: PrimitiveIr },
    VectorNe { ty: VectorIr },

    ScalarLt { ty: NumIr },
    VectorLt { n: LengthIr, t: NumIr },

    ScalarGt { ty: NumIr },
    VectorGt { n: LengthIr, t: NumIr },

    ScalarLe { ty: NumIr },
    VectorLe { n: LengthIr, t: NumIr },

    ScalarGe { ty: NumIr },
    VectorGe { n: LengthIr, t: NumIr },
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
