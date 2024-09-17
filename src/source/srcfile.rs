use std::{mem, ops::Index};

#[repr(transparent)]
pub struct SrcFile {
    pub s: str
}
impl AsRef<SrcFile> for str {
    fn as_ref(&self) -> &SrcFile {
        unsafe {
            mem::transmute(self)
        }
    }
}
impl Index<Span> for SrcFile {
    type Output = str;
    fn index(&self, index: Span) -> &Self::Output {
        &self.s[index.into_range()]
    }
}
impl Spanned for SrcFile {
    fn span(&self) -> Span {
        Span::new(0, self.s.len())
    }
}