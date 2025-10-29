use core::marker::PhantomData;

#[derive(Debug, Clone, Copy)]
pub struct Bank<T: 'static> {
    values: &'static [T],
}

#[derive(Debug, Clone, Copy)]
pub struct Id<T> {
    idx: usize,
    t: PhantomData<T>,
}

impl<T> Bank<T> {
    pub const fn raw(values: &'static [T]) -> Self {
        Self { values }
    }
}

impl<T> Id<T> {
    pub const fn raw(idx: usize) -> Self {
        Self {
            idx,
            t: PhantomData,
        }
    }

    pub const fn access(&self, bank: &Bank<T>) -> &'static T {
        &bank.values[self.idx]
    }

    pub const fn eq(&self, other: &Self) -> bool {
        self.idx == other.idx
    }
}

for_bank_types! {
    impl Bank<T> {
        pub const fn eq(&self, other: &Self) -> bool {
            if self.values.len() != other.values.len() {
                return false;
            }

            let mut i = 0;
            while i < self.values.len() {
                if !self.values[i].eq(&other.values[i]){
                    return false;
                }

                i += 1;
            }

            true
        }
    }
}

macro_rules! for_bank_types {
    ($impl:item) => {
        mod _mod0 {
            use super::*;
            type T = crate::ir::ExprIr;
            $impl
        }
        mod _mod1 {
            use super::*;
            type T = crate::ir::StmtIr;
            $impl
        }
    };
}

use for_bank_types;
