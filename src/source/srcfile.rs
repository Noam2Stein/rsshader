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
    pub const fn end_span(&self) -> Span {
        Span::sized(self.s.len(), 0)
    }

    #[inline(always)]
    pub const fn srcslice(&self) -> &SrcSlice {
        unsafe { mem::transmute(self) }
    }
}
impl Display for SrcFile {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.s.fmt(f)
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
impl Describe for SrcFile {
    #[inline(always)]
    fn desc(&self) -> Description {
        Description::quote(&self.s)
    }
}
impl TypeDescribe for SrcFile {
    #[inline(always)]
    fn type_desc() -> Description {
        Description::new("a srcfile")
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

    pub fn span(&self, srcfile: &SrcFile) -> Span {
        let (srcfile_start, srcfile_len): (usize, usize) = unsafe { mem::transmute(srcfile) };
        let (start, len): (usize, usize) = unsafe { mem::transmute(self) };

        assert!(start >= srcfile_start && start + len <= srcfile_start + srcfile_len, "srcslice out of srcfile range");

        Span::sized(start - srcfile_start, len)
    }
}
impl Display for SrcSlice {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.s.fmt(f)
    }
}
impl<'src> Describe for SrcSlice {
    #[inline(always)]
    fn desc(&self) -> Description {
        Description::quote(&self.s)
    }
}
impl TypeDescribe for SrcSlice {
    #[inline(always)]
    fn type_desc() -> Description {
        Description::new("a srcslice")
    }
}