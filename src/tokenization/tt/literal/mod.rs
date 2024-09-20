use super::*;

mod int;
mod float;
pub use int::*;
pub use float::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Literal<'src> {
    Int(IntLiteral<'src>),
    Float(FloatLiteral<'src>),
}
impl<'src> Literal<'src> {
    pub fn literal_type_desc(&self) -> Description {
        match self {
            Self::Int(_) => IntLiteral::type_desc(),
            Self::Float(_) => FloatLiteral::type_desc(),
        }
    }
}
impl<'src> Display for Literal<'src> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(literal) => literal.fmt(f),
            Self::Float(literal) => literal.fmt(f),
        }
    }
}
impl<'src> Describe for Literal<'src> {
    fn desc(&self) -> Description {
        match self {
            Self::Int(literal) => literal.desc(),
            Self::Float(literal) => literal.desc(),
        }
    }
}
impl<'src> TypeDescribe for Literal<'src> {
    fn type_desc() -> Description {
        Description::new("a literal")
    }
}
impl<'src> Spanned for Literal<'src> {
    fn span(&self, srcfile: &SrcFile) -> Span {
        match self {
            Self::Int(literal) => literal.span(srcfile),
            Self::Float(literal) => literal.span(srcfile),
        }
    }
}
impl<'src> FromSrc<'src> for Literal<'src> {
    fn from_src(srcslice: &'src SrcSlice) -> Result<Self, ErrorMessage> {
        if srcslice.s().contains(".") {
            FloatLiteral::from_src(srcslice).map(|literal| Self::Float(literal))
        }
        else {
            IntLiteral::from_src(srcslice).map(|literal| Self::Int(literal))
        }
    }
}
impl<'src> FromSrcUnchecked<'src> for Literal<'src> {
    unsafe fn from_src_unchecked(srcslice: &'src SrcSlice) -> Self {
        if srcslice.s().contains(".") {
            Self::Float(FloatLiteral::from_src_unchecked(srcslice))
        }
        else {
            Self::Int(IntLiteral::from_src_unchecked(srcslice))
        }
    }
}
impl<'src> DefaultToken<'src> for Literal<'src> {
    fn default_token(srcfile: &'src SrcFile, span: Span) -> Self {
        Self::Int(IntLiteral::default_token(srcfile, span))
    }
}
impl<'src> ParseTokens<'src> for Literal<'src> {
    fn parse_tokens(tokens: &mut impl TokenIterator<'src>, errs: &mut Vec<Error>) -> Self {
        if let Some(token) = tokens.next(errs) {
            if let TokenTree::Literal(tokens) = token {
                tokens
            }
            else {
                let srcfile = tokens.srcfile();

                errs.push(Error::from_messages(token.span(srcfile), [
                    errm::expected_found(Self::type_desc(), token.token_type_desc())
                ]));

                Self::default_token(srcfile, token.span(srcfile))
            }
        }
        else {
            let srcfile = tokens.srcfile();

            errs.push(Error::from_messages(srcfile.span().end_span(), [
                errm::unexpected_end_of_file(),
                errm::expected(Self::type_desc())
            ]));
            
            Self::default_token(srcfile, srcfile.span().end_span())
        }
    }
}
impl<'src> _ValidatedToken<'src> for Literal<'src> {
    
}