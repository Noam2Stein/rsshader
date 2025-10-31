use core::marker::PhantomData;

use crate::ir::TypeIr;

#[derive(Debug, Clone, Copy)]
pub struct Iter<T>(pub(in crate::ir) Inner, pub(in crate::ir) PhantomData<T>);

#[derive(Debug, Clone, Copy)]
pub(in crate::ir) enum Inner {
    Attributes { ty: &'static TypeIr, idx: usize },
}

impl Iter<TypeIr> {
    pub const fn next(&mut self) -> Option<&TypeIr> {
        match &mut self.0 {
            Inner::Attributes { ty, idx } => {
                const fn peek(ty: &TypeIr, idx: usize) -> Option<&TypeIr> {
                    const fn count(ty: &TypeIr) -> usize {
                        match ty {
                            TypeIr::Primitive(_) | TypeIr::Vector { .. } => 1,

                            TypeIr::Struct { fields } => {
                                let mut sum = 0;
                                let mut i = 0;
                                while i < fields.len() {
                                    sum += count(&fields[i]);
                                    i += 1;
                                }

                                sum
                            }
                        }
                    }

                    match ty {
                        TypeIr::Primitive(_) | TypeIr::Vector { .. } => {
                            if idx == 0 {
                                Some(ty)
                            } else {
                                None
                            }
                        }

                        TypeIr::Struct { fields } => {
                            let mut field_idx = 0;
                            let mut idx_in_field = idx;
                            loop {
                                if field_idx < fields.len() {
                                    let field_attr_count = count(&fields[field_idx]);
                                    if idx_in_field < field_attr_count {
                                        break Some(peek(&fields[field_idx], idx_in_field).unwrap());
                                    } else {
                                        field_idx += 1;
                                        idx_in_field -= field_attr_count;
                                    }
                                } else {
                                    break None;
                                }
                            }
                        }
                    }
                }

                let output = peek(*ty, *idx);
                *idx += 1;

                output
            }
        }
    }
}
