use super::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenTree<'src> {
    Keyword(SpannedKeyword),
    Ident(SpannedIdent<'src>),
    Punct(SpannedPunct),
    Literal(SpannedLiteral),
    Group(Group<'src>),
    InvalidAny(InvalidAny<'src>),
}
impl<'src> TokenTree<'src> {
    pub fn token_type_desc(&self) -> Description {
        match self {
            Self::Keyword(_) => Keyword::type_desc(),
            Self::Ident(_) => Ident::type_desc(),
            Self::Punct(_) => Punct::type_desc(),
            Self::Literal(_) => Literal::type_desc(),
            Self::Group(_) => Group::type_desc(),
            Self::InvalidAny(_) => InvalidAny::type_desc(),
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
            Self::InvalidAny(tt) => tt.fmt(f),
        }
    }
}
impl<'src> Spanned for TokenTree<'src> {
    fn span(&self) -> Span {
        match self {
            Self::Keyword(tt) => tt.span(),
            Self::Ident(tt) => tt.span(),
            Self::Punct(tt) => tt.span(),
            Self::Literal(tt) => tt.span(),
            Self::Group(tt) => tt.span(),
            Self::InvalidAny(tt) => tt.span(),
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
            Self::InvalidAny(tt) => tt.desc(),
        }
    }
}
impl<'src> TypeDescribe for TokenTree<'src> {
    fn type_desc() -> Description {
        Description::new("a token tree")
    }
}
impl<'stream, 'src> FromTokens<'stream, 'src> for TokenTree<'src> {
    fn from_tokens(tokens: &mut TokenStreamIter<'_, 'src>, errs: &mut Vec<Error>) -> Self {
        if let Some(token) = tokens.next() {
            token.clone()
        }
        else {
            errs.push(Error::from_messages(tokens.stream().span().last_byte(), [
                errm::unexpected_end_of_file(),
                errm::expected(Self::type_desc())
            ]));

            Self::InvalidAny(InvalidAny::empty())
        }
    }
}