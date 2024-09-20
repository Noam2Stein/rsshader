use super::*;

mod delimiter;
pub use delimiter::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Group<'src> {
    pub delimiter: Delimiter,
    pub stream: TokenStream<'src>,
    pub srcslice: &'src SrcSlice,
}
impl<'src> Group<'src> {
    pub fn parse_tokens_with(delimiter: Delimiter, tokens: &mut impl TokenIterator<'src>, errs: &mut Vec<Error>) -> Self {
        if let Some(token) = tokens.next(errs) {
            if let TokenTree::Group(token) = token {
                if token.delimiter == delimiter {
                    token
                }
                else {
                    let srcfile = tokens.srcfile();

                    errs.push(Error::from_messages(token.span(srcfile), [
                        errm::expected_found(Self::type_desc().with(&delimiter.desc()), Self::type_desc().with(&token.delimiter.desc()))
                    ]));

                    Self::default_token(srcfile, token.span(srcfile))
                }
            }
            else {
                let srcfile = tokens.srcfile();

                errs.push(Error::from_messages(token.span(srcfile), [
                    errm::expected_found(Self::type_desc().with(&delimiter.desc()), token.token_type_desc())
                ]));

                Self::default_token(srcfile, token.span(srcfile))
            }
        }
        else {
            let srcfile = tokens.srcfile();

            errs.push(Error::from_messages(srcfile.span().end_span(), [
                errm::unexpected_end_of_file(),
                errm::expected(Self::type_desc().with(&delimiter.desc()))
            ]));

            Self::default_token(srcfile, srcfile.span().end_span())
        }
    }

    #[inline(always)]
    pub fn open_span(&self, srcfile: &'src SrcFile<'src>) -> Span {
        self.span(srcfile).first_byte()
    }
    #[inline(always)]
    pub fn close_span(&self, srcfile: &'src SrcFile<'src>) -> Span {
        self.span(srcfile).last_byte()
    }
}
impl<'src> Display for Group<'src> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.delimiter.open_str(),
            self.stream,
            self.delimiter.close_str(),
        )
    }
}
impl<'src> Spanned for Group<'src> {
    fn span(&self, srcfile: &SrcFile) -> Span {
        self.srcslice.span(srcfile)
    }
}
impl<'src> Describe for Group<'src> {
    fn desc(&self) -> Description {
        Description::quote(&self.to_string())
    }
}
impl<'src> TypeDescribe for Group<'src> {
    fn type_desc() -> Description {
        Description::new("a group")
    }
}
impl<'src> FromSrc<'src> for Group<'src> {
    fn from_src(srcslice: &'src SrcSlice) -> Result<Self, ErrorMessage> {
        todo!()
    }
}
impl<'src> FromSrcUnchecked<'src> for Group<'src> {
    unsafe fn from_src_unchecked(srcslice: &'src SrcSlice) -> Self {
        todo!()
    }
}
impl<'src> DefaultToken<'src> for Group<'src> {
    fn default_token(srcfile: &'src SrcFile, span: Span) -> Self {
        Self {
            delimiter: Delimiter::Brace,
            stream: TokenStream::default(),
            srcslice: &srcfile[span],
        }
    }
}
impl<'src> ParseTokens<'src> for Group<'src> {
    fn parse_tokens(tokens: &mut impl TokenIterator<'src>, errs: &mut Vec<Error>) -> Self {
        if let Some(token) = tokens.next(errs) {
            if let TokenTree::Group(token) = token {
                token
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
impl<'src> _ValidatedToken<'src> for Group<'src> {

}

macro_rules! group_with {
    ($ident:ident: $delimiter:expr) => {
        #[repr(transparent)]
        pub struct $ident<'src> {
            group: Group<'src>,
        }
        impl<'src> $ident<'src> {
            pub fn group(self) -> Group<'src> {
                self.group
            }
        }
        impl<'src> From<$ident<'src>> for Group<'src> {
            fn from(value: $ident<'src>) -> Self {
                value.group
            }
        }
        impl<'src> ParseTokens<'src> for $ident<'src> {
            fn parse_tokens(tokens: &mut impl TokenIterator<'src>, errs: &mut Vec<Error>) -> Self {
                unsafe {
                    mem::transmute(Group::parse_tokens_with($delimiter, tokens, errs))
                }
            }
        }
    };
}

group_with!(GroupWithBraces: Delimiter::Brace);
group_with!(GroupWithBrackets: Delimiter::Bracket);
group_with!(GroupWithParenthesis: Delimiter::Parenthesis);