use std::{mem, ops::{Index, Range}, slice};

use super::*;

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SrcFile {
    s: str,
}
impl SrcFile {
    #[inline(always)]
    pub const fn new(s: &str) -> &Self {
        unsafe {
            mem::transmute(s)
        }
    }
    #[inline(always)]
    pub const fn s(&self) -> &str {
        &self.s
    }

    #[inline(always)]
    pub const fn span(&self) -> Span {
        Span::sized(0, self.s.len())
    }
    #[inline(always)]
    pub const fn srcslice(&self) -> &SrcSlice {
        unsafe { mem::transmute(self) }
    }
    #[inline(always)]
    pub fn end_srcslice(&self) -> &SrcSlice {
        unsafe { mem::transmute(&self.s[self.s.len()..self.s.len()]) }
    }
}
impl Index<Span> for SrcFile {
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
    pub const fn start(&self) -> &SrcSliceStart {
        unsafe {
            mem::transmute(&self.s.as_bytes()[0])
        }
    }
    #[inline(always)]
    pub const fn len(&self) -> usize {
        self.s.len()
    }


    #[inline(always)]
    pub fn with_len(&self, len: usize) -> &SrcSlice {
        unsafe {
            mem::transmute(&self.s[..len])
        }
    }

    #[inline(always)]
    pub fn span_start(&self, srcfile: &SrcFile) -> usize {
        (&self.s.as_bytes()[0]) as *const u8 as usize - (&srcfile.s.as_bytes()[0]) as *const u8 as usize
    }
    #[inline(always)]
    pub fn span(&self, srcfile: &SrcFile) -> Span {
        Span::sized(
            self.span_start(srcfile),
            self.len()
        )
    }
}
impl AsRef<str> for SrcSlice {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        &self.s
    }
}
impl Index<Range<usize>> for SrcSlice {
    type Output = Self;
    fn index(&self, index: Range<usize>) -> &Self::Output {
        unsafe { mem::transmute(&self.s()[index]) }
    }
}
impl<'src> Describe for SrcSlice {
    fn desc(&self) -> Description {
        Description::quote(&self.s)
    }
}

pub trait GetSrcSlice<'src> {
    fn srcslice(&self) -> &'src SrcSlice;

    #[inline(always)]
    fn span(&self, srcfile: &SrcFile) -> Span {
        self.srcslice().span(srcfile)
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

mod tests {
    #[test]
    fn srcslice_span_conversion() {
        use super::*;

        let srcfile = SrcFile::new("smg1.5");
        let span = Span::sized(1, 3);
        let srcslice = &srcfile[span];

        assert_eq!(srcslice.s(), "mg1");
        assert_eq!(srcslice.span(srcfile), span)
    }
}