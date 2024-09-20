use std::{mem, ops::Index, slice};

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SrcFile<'src> {
    s: &'src str,
}
impl<'src> SrcFile<'src> {
    #[inline(always)]
    pub const fn new(s: &'src str) -> Self {
        Self {
            s,
        }
    }
    #[inline(always)]
    pub const fn s(&self) -> &str {
        &self.s
    }

    pub const fn span(&self) -> Span {
        Span::sized(0, self.s.len())
    }
}
impl<'src> Index<Span> for SrcFile<'src> {
    type Output = SrcSlice;
    #[inline(always)]
    fn index(&self, index: Span) -> &Self::Output {
        unsafe {
            mem::transmute(&self.s[index.range()])
        }
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SrcSlice {
    s: str
}
impl SrcSlice {
    #[inline(always)]
    pub const fn s(&self) -> &str {
        &self.s
    }
    #[inline(always)]
    pub fn start_in_file(&self, srcfile: &SrcFile) -> usize {
        (&self.s.as_bytes()[0]) as *const u8 as usize - (&srcfile.s.as_bytes()[0]) as *const u8 as usize
    }
    pub const fn start(&self) -> &SrcSliceStart {
        unsafe {
            mem::transmute(&self.s.as_bytes()[0])
        }
    }
    #[inline(always)]
    pub const fn len(&self) -> usize {
        self.s.len()
    }
}
impl AsRef<str> for SrcSlice {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        &self.s
    }
}
impl Spanned for SrcSlice {
    #[inline(always)]
    fn span(&self, srcfile: &SrcFile) -> Span {
        Span::sized(
            self.start_in_file(srcfile),
            self.len()
        )
    }
}
impl<'src> Describe for SrcSlice {
    fn desc(&self) -> Description {
        Description::quote(&self.s)
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SrcSliceStart {
    inner: u8,
}
impl SrcSliceStart {
    pub const unsafe fn with_len(&self, len: usize) -> &SrcSlice {
        mem::transmute(slice::from_raw_parts(&self.inner, len))
    }
}