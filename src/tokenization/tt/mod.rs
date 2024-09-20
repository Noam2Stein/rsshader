pub mod keyword;
pub mod ident;
pub mod punct;
pub mod literal;
pub mod group;

pub use keyword::*;
pub use ident::*;
pub use punct::*;
pub use literal::*;
pub use group::*;

use super::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash, Ord)]
pub enum TokenTree<'src> {
    Keyword(Keyword<'src>),
    Ident(Ident<'src>),
    Punct(Punct<'src>),
    Literal(Literal<'src>),
    Group(Group<'src>),
}
impl<'src> TokenTree<'src> {
    pub fn token_type_desc(&self) -> Description {
        match self {
            Self::Keyword(_) => Keyword::type_desc(),
            Self::Ident(_) => Ident::type_desc(),
            Self::Punct(_) => Punct::type_desc(),
            Self::Literal(_) => Literal::type_desc(),
            Self::Group(_) => Group::type_desc(),
        }
    }
}
impl<'src> Display for TokenTree<'src> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Keyword(tt) => tt.fmt(f),
            Self::Ident(tt) => tt.fmt(f),
            Self::Punct(tt) => tt.fmt(f),
            Self::Literal(tt) => tt.fmt(f),
            Self::Group(tt) => tt.fmt(f),
        }
    }
}
impl<'src> GetSrcSlice<'src> for TokenTree<'src> {
    fn srcslice(&self) -> &'src SrcSlice {
        match self {
            Self::Keyword(tt) => tt.srcslice(),
            Self::Ident(tt) => tt.srcslice(),
            Self::Punct(tt) => tt.srcslice(),
            Self::Literal(tt) => tt.srcslice(),
            Self::Group(tt) => tt.srcslice(),
        }   
    }
}
impl<'src> Describe for TokenTree<'src> {
    fn desc(&self) -> Description {
        match self {
            Self::Keyword(tt) => tt.desc(),
            Self::Ident(tt) => tt.desc(),
            Self::Punct(tt) => tt.desc(),
            Self::Literal(tt) => tt.desc(),
            Self::Group(tt) => tt.desc(),
        }
    }
}
impl<'src> TypeDescribe for TokenTree<'src> {
    fn type_desc() -> Description {
        Description::new("a token tree")
    }
}
impl<'src> FromSrc<'src> for TokenTree<'src> {
    fn from_src(srcslice: &'src SrcSlice) -> Result<Self, ErrorMessage> {
        todo!()
    }
}
impl<'src> FromSrcUnchecked<'src> for TokenTree<'src> {
    unsafe fn from_src_unchecked(srcslice: &'src SrcSlice) -> Self {
        todo!()
    }
}
impl<'src> DefaultToken<'src> for TokenTree<'src> {
    fn default_token(srcslice: &'src SrcSlice) -> Self {
        Self::Ident(Ident::default_token(srcslice))
    }
}
impl<'src> ParseTokens<'src> for TokenTree<'src> {
    fn parse_tokens(tokens: &mut impl TokenParser<'src>, errs: &mut Vec<Error<'src>>) -> Self {
        if let Some(token) = tokens.next(errs) {
            token
        }
        else {
            errs.push(Error::from_messages(tokens.end_srcslice(), [
                errm::unexpected_end_of_file(),
                errm::expected(Self::type_desc())
            ]));

            Self::default_token(tokens.end_srcslice().with_len(0))
        }
    }
}
impl<'src> _ValidatedToken<'src> for TokenTree<'src> {

}

trait DefaultToken<'src> {
    fn default_token(srcslice: &'src SrcSlice) -> Self;
}

trait _ValidatedToken<'src>:
fmt::Debug +
Clone +
Eq +
Ord +
Hash +
Display +
Describe +
TypeDescribe +
GetSrcSlice<'src> +
FromSrc<'src> +
FromSrcUnchecked<'src> +
DefaultToken<'src> +
ParseTokens<'src> +
{

}