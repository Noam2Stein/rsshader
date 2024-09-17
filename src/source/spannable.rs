pub trait Spannable: Sized {
    type Spanned: SpannedSpannable<Unspanned = Self>;
}
pub trait SpannedSpannable: Spanned {
    type Unspanned: Spannable<Spanned = Self>;
    fn into_unspanned(self) -> Self::Unspanned;
}

impl<T: SpannedSpannable<Unspanned: TypeDescribe>> TypeDescribe for T {
    #[inline(always)]
    fn type_desc() -> Description {
        T::Unspanned::type_desc()
    }
}

pub trait WrapSpannable: Sized {
    type Wrapper: WrapSpanned<Inner = Self>;
}
pub trait WrapSpanned: Spanned {
    type Inner: WrapSpannable<Wrapper = Self>;
    fn inner(&self) -> &Self::Inner;
    fn into_inner(self) -> Self::Inner;
}

impl<T: WrapSpannable> Spannable for T {
    type Spanned = T::Wrapper;
}
impl<'src, T: WrapSpannable<Wrapper: ParseTokens<'src>>> ParseTokens<'src> for T {
    #[inline(always)]
    fn parse_tokens(tokens: impl Iterator<Item = TokenTree<'src>>, src: &'src SrcFile, errs: &mut Vec<Error>) -> Self {
        T::Wrapper::parse_tokens(tokens, src, errs).into_unspanned()
    }
}
impl<'src, T: WrapSpannable<Wrapper: FromSrc<'src>>> FromSrc<'src> for T {
    #[inline(always)]
    fn from_src(src: &'src SrcFile, span: Span) -> Option<Self> {
        T::Wrapper::from_src(src, span).map(|output| output.into_unspanned())
    }
}

impl<T: WrapSpanned> SpannedSpannable for T {
    type Unspanned = T::Inner;
    #[inline(always)]
    fn into_unspanned(self) -> Self::Unspanned {
        T::into_inner(self)
    }
}
impl<T: WrapSpanned<Inner: Describe>> Describe for T {
    #[inline(always)]
    fn desc(&self) -> Description {
        self.inner().desc()
    }
}