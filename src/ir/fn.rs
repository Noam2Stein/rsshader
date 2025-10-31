use rsshader_macros::ConstEq;

use crate::ir::{
    BitwisePrimitive, Int, Length, LinkedShaderIr, Numeric, Primitive, SignedNumeric, TypeIr,
};

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum FnIr {
    UserDefined {
        param_types: &'static [&'static TypeIr],
        ret_type: Option<&'static TypeIr>,
        body: BodyIr,
    },
    Builtin(BuiltinFn),
}

////////////////////////////////////////////////////////////////////////////////
// Function Body
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, ConstEq)]
pub struct BodyIr {
    pub stmts: &'static [StmtIr],
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum StmtIr {
    Call {
        func: &'static FnIr,
        args: &'static [ExprIr],
    },
    VariableDecl {
        id: usize,
        ty: &'static TypeIr,
    },
    Assignment {
        left: PlaceIr,
        right: ExprIr,
    },
    Return {
        value: Option<ExprIr>,
    },
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum ExprIr {
    Literal(Literal),
    Param {
        idx: usize,
        ty: &'static TypeIr,
    },
    Variable {
        id: usize,
        ty: &'static TypeIr,
    },
    Call {
        func: &'static FnIr,
        args: &'static [ExprIr],
    },
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum PlaceIr {
    Variable { id: usize, ty: &'static TypeIr },
    VectorElement { idx: usize, base: &'static PlaceIr },
    StructField { idx: usize, base: &'static PlaceIr },
}

#[derive(Debug, Clone, Copy)]
pub enum Literal {
    F32(f32),
    I32(i32),
    U32(u32),
    Bool(bool),
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

impl ExprIr {
    pub const fn ty(&self) -> TypeIr {
        match self {
            Self::Literal(Literal::F32(_)) => TypeIr::Primitive(Primitive::F32),
            Self::Literal(Literal::I32(_)) => TypeIr::Primitive(Primitive::I32),
            Self::Literal(Literal::U32(_)) => TypeIr::Primitive(Primitive::U32),
            Self::Literal(Literal::Bool(_)) => TypeIr::Primitive(Primitive::Bool),

            Self::Param { idx: _, ty } => **ty,
            Self::Variable { id: _, ty } => **ty,

            Self::Call { func, args: _ } => match func {
                FnIr::UserDefined {
                    param_types: _,
                    ret_type,
                    body: _,
                } => *ret_type.unwrap(),

                FnIr::Builtin(func) => func.ret_type().unwrap(),
            },
        }
    }
}

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

////////////////////////////////////////////////////////////////////////////////
// Builtin Functions
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum BuiltinFn {
    ScalarPrimitiveOp {
        op: PrimitiveOp,
        ty: Primitive,
    },
    ScalarNumericOp {
        op: NumericOp,
        ty: Numeric,
    },
    ScalarIntOp {
        op: IntOp,
        ty: Int,
    },
    ScalarSignedNumericOp {
        op: SignedNumericOp,
        ty: SignedNumeric,
    },
    ScalarBitwiseOp {
        op: BitwiseOp,
        ty: BitwisePrimitive,
    },

    VectorConstructor {
        op: VectorConstructor,
        t: Primitive,
    },
    VectorSplat {
        n: Length,
        t: Primitive,
    },
    VectorElement {
        i: usize,
        n: Length,
        t: Primitive,
    },

    VectorizedPrimitiveOp {
        op: PrimitiveOp,
        n: Length,
        t: Primitive,
    },
    VectorizedNumericOp {
        op: NumericOp,
        n: Length,
        t: Numeric,
    },
    VectorizedIntOp {
        op: IntOp,
        n: Length,
        t: Int,
    },
    VectorizedSignedNumericOp {
        op: SignedNumericOp,
        n: Length,
        t: SignedNumeric,
    },
    VectorizedBitwiseOp {
        op: BitwiseOp,
        n: Length,
        t: BitwisePrimitive,
    },

    VectorPrimitiveOp {
        op: VectorPrimitiveOp,
        n: Length,
        t: Primitive,
    },
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum PrimitiveOp {
    Eq,
    Ne,
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum NumericOp {
    Lt,
    Gt,
    Le,
    Ge,
    Add,
    Sub,
    Mul,
    Div,
    Rem,
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum IntOp {
    Shl,
    Shr,
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum SignedNumericOp {
    Neg,
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum BitwiseOp {
    Not,
    BitAnd,
    BitOr,
    BitXor,
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum VectorConstructor {
    Vec2From11,
    Vec3From111,
    Vec3From12,
    Vec3From21,
    Vec4From1111,
    Vec4From112,
    Vec4From121,
    Vec4From13,
    Vec4From211,
    Vec4From22,
    Vec4From31,
}

#[derive(Debug, Clone, Copy, ConstEq)]
pub enum VectorPrimitiveOp {
    Eq,
    Ne,
}

impl BuiltinFn {
    const fn ret_type(&self) -> Option<TypeIr> {
        match self {
            Self::ScalarPrimitiveOp { op, ty } => Some(TypeIr::Primitive(op.ret_type(*ty))),
            Self::ScalarNumericOp { op, ty } => Some(TypeIr::Primitive(op.ret_type(*ty))),
            Self::ScalarIntOp { op, ty } => Some(TypeIr::Primitive(op.ret_type(*ty))),
            Self::ScalarSignedNumericOp { op, ty } => Some(TypeIr::Primitive(op.ret_type(*ty))),
            Self::ScalarBitwiseOp { op, ty } => Some(TypeIr::Primitive(op.ret_type(*ty))),

            Self::VectorConstructor { op, t } => Some(TypeIr::Vector {
                n: op.ret_len(),
                t: *t,
            }),
            Self::VectorSplat { n, t } => Some(TypeIr::Vector { n: *n, t: *t }),
            Self::VectorElement { i: _, n: _, t } => Some(TypeIr::Primitive(*t)),

            Self::VectorizedPrimitiveOp { op, n, t } => Some(TypeIr::Vector {
                n: *n,
                t: op.ret_type(*t),
            }),
            Self::VectorizedNumericOp { op, n, t } => Some(TypeIr::Vector {
                n: *n,
                t: op.ret_type(*t),
            }),
            Self::VectorizedIntOp { op, n, t } => Some(TypeIr::Vector {
                n: *n,
                t: op.ret_type(*t),
            }),
            Self::VectorizedSignedNumericOp { op, n, t } => Some(TypeIr::Vector {
                n: *n,
                t: op.ret_type(*t),
            }),
            Self::VectorizedBitwiseOp { op, n, t } => Some(TypeIr::Vector {
                n: *n,
                t: op.ret_type(*t),
            }),

            Self::VectorPrimitiveOp { op, n, t } => Some(op.ret_type(*n, *t)),
        }
    }
}

impl PrimitiveOp {
    const fn ret_type(&self, _ty: Primitive) -> Primitive {
        match self {
            Self::Eq | Self::Ne => Primitive::Bool,
        }
    }
}

impl NumericOp {
    const fn ret_type(&self, ty: Numeric) -> Primitive {
        match self {
            Self::Lt | Self::Gt | Self::Le | Self::Ge => Primitive::Bool,
            Self::Add | Self::Sub | Self::Mul | Self::Div | Self::Rem => ty.as_primitive(),
        }
    }
}

impl IntOp {
    const fn ret_type(&self, ty: Int) -> Primitive {
        match self {
            Self::Shl | Self::Shr => ty.as_primitive(),
        }
    }
}

impl SignedNumericOp {
    const fn ret_type(&self, ty: SignedNumeric) -> Primitive {
        match self {
            Self::Neg => ty.as_primitive(),
        }
    }
}

impl BitwiseOp {
    const fn ret_type(&self, ty: BitwisePrimitive) -> Primitive {
        match self {
            Self::Not | Self::BitAnd | Self::BitOr | Self::BitXor => ty.as_primitive(),
        }
    }
}

impl VectorConstructor {
    const fn ret_len(&self) -> Length {
        match self {
            Self::Vec2From11 => Length::Two,

            Self::Vec3From111 | Self::Vec3From12 | Self::Vec3From21 => Length::Three,

            Self::Vec4From1111
            | Self::Vec4From112
            | Self::Vec4From121
            | Self::Vec4From13
            | Self::Vec4From211
            | Self::Vec4From22
            | Self::Vec4From31 => Length::Four,
        }
    }
}

impl VectorPrimitiveOp {
    const fn ret_type(&self, _n: Length, _t: Primitive) -> TypeIr {
        match self {
            Self::Eq | Self::Ne => TypeIr::Primitive(Primitive::Bool),
        }
    }
}
