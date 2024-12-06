use super::{GPUFieldDesc, GPULetDesc, GPUTypeDesc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GPUExprDesc {
    BoolLiteral(bool),
    F32Literal(&'static str),
    I32Literal(i32),
    U32Literal(u32),
    Array(&'static [GPUExprDesc]),
    Tuple(&'static [GPUExprDesc]),
    Struct(
        &'static GPUTypeDesc,
        &'static [(&'static GPUFieldDesc, GPUExprDesc)],
    ),
    Variable(&'static GPULetDesc),
    ArrayIndex(&'static GPUExprDesc, &'static GPUExprDesc),
    TupleField(&'static GPUExprDesc, usize),
    StructField(&'static GPUExprDesc, &'static GPUFieldDesc),
}

impl GPUExprDesc {
    pub const fn evaluation(&self) -> &'static GPUTypeDesc {
        match self {
            Self::BoolLiteral(_) => &GPUTypeDesc::Bool,
            Self::F32Literal(_) => &GPUTypeDesc::F32,
            Self::I32Literal(_) => &GPUTypeDesc::I32,
            Self::U32Literal(_) => &GPUTypeDesc::U32,
            Self::Array(expr) => &GPUTypeDesc::Array(
                match expr.first() {
                    Some(first_expr) => first_expr.evaluation(),
                    None => &GPUTypeDesc::Tuple(&[]),
                },
                expr.len(),
            ),
            Self::Tuple(items) => &GPUTypeDesc::Tuple(map_exprs_to_evaluations(items)),
            Self::Struct(ty, _) => ty,
            Self::Variable(let_stmt) => &let_stmt.value.evaluation(),
            Self::ArrayIndex(base_expr, _) => match base_expr.evaluation() {
                GPUTypeDesc::Array(item_type, _) => item_type,
                _ => panic!("GPUExprDesc::ArrayIndex: expected an array"),
            },
            Self::TupleField(base_expr, field_index) => match base_expr.evaluation() {
                GPUTypeDesc::Tuple(fields) => &fields[*field_index],
                _ => panic!("GPUExprDesc::ArrayIndex: expected an array"),
            },
            Self::StructField(_, field) => field.ty,
        }
    }
}

const fn map_exprs_to_evaluations(exprs: &'static [GPUExprDesc]) -> &'static [GPUTypeDesc] {}
