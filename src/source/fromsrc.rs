use super::*;

pub trait FromSrc<'src>: Sized {
    fn from_src(srcslice: &'src SrcSlice) -> Result<Self, ErrorMessage>;
}
pub trait FromSrcUnchecked<'src>: Sized {
    unsafe fn from_src_unchecked(srcslice: &'src SrcSlice) -> Self;
}