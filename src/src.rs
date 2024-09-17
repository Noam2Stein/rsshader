use std::ops::Index;

use crate::span::*;

pub struct SrcFile {
    pub s: Box<str>
}
impl SrcFile {
    pub fn new(s: impl Into<Box<str>>) -> Self {
        Self {
            s: s.into()
        }
    }
}
impl Index<Span> for SrcFile {
    type Output = str;
    fn index(&self, index: Span) -> &Self::Output {
        &self.s[index.into_range()]
    }
}

pub trait FromSrc<'src>: Sized {
    fn from_src(src: &'src SrcFile, span: Span) -> Option<Self>;
}