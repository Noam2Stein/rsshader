use super::*;

mod delimiter;
pub use delimiter::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Group<'src> {
    delimiter: Delimiter,
    tts: Vec<TokenTree<'src>>,
    srcslice: &'src SrcSlice,
}
impl<'src> Group<'src> {
    pub fn parse_tokens_with(delimiter: Delimiter, parser: &mut impl TokenParser<'src>, errs: &mut Vec<Error<'src>>) -> Self {
        if let Some(token) = parser.next(errs) {
            if let TokenTree::Group(token) = token {
                if token.delimiter != delimiter {
                    errs.push(Error::from_messages(token.srcslice, [
                        errm::expected_found(Self::type_desc().with(&delimiter.desc()), Self::type_desc().with(&token.delimiter.desc()))
                    ]));
                }
                
                token
            }
            else {
                errs.push(Error::from_messages(token.srcslice(), [
                    errm::expected_found(Self::type_desc(), token.token_type_desc())
                ]));

                Self::default_token(token.srcslice())
            }
        }
        else {
            errs.push(Error::from_messages(parser.end_srcslice(), [
                errm::unexpected_end_of_file(),
                errm::expected(Self::type_desc())
            ]));

            Self::default_token(parser.end_srcslice().with_len(0))
        }
    }

    #[inline(always)]
    pub fn open_span(&self, srcfile: &'src SrcFile) -> Span {
        self.span(srcfile).first_byte()
    }
    #[inline(always)]
    pub fn close_span(&self, srcfile: &'src SrcFile) -> Span {
        self.span(srcfile).last_byte()
    }
}
impl<'src> Display for Group<'src> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.delimiter.open_str(),
            self.tts.iter().map(|tt| tt.to_string()).collect::<Box<[String]>>().join(" "),
            self.delimiter.close_str(),
        )
    }
}
impl<'src> GetSrcSlice<'src> for Group<'src> {
    fn srcslice(&self) -> &'src SrcSlice {
        self.srcslice
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
    fn default_token(srcslice: &'src SrcSlice) -> Self {
        Self {
            delimiter: Delimiter::Brace,
            tts: Vec::new(),
            srcslice,
        }
    }
}
impl<'src> ParseTokens<'src> for Group<'src> {
    fn parse_tokens(parser: &mut impl TokenParser<'src>, errs: &mut Vec<Error<'src>>) -> Self {
        if let Some(token) = parser.next(errs) {
            if let TokenTree::Group(token) = token {
                token
            }
            else {
                errs.push(Error::from_messages(token.srcslice(), [
                    errm::expected_found(Self::type_desc(), token.token_type_desc())
                ]));

                Self::default_token(token.srcslice())
            }
        }
        else {
            errs.push(Error::from_messages(parser.end_srcslice(), [
                errm::unexpected_end_of_file(),
                errm::expected(Self::type_desc())
            ]));

            Self::default_token(parser.end_srcslice().with_len(0))
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
            fn parse_tokens(parser: &mut impl TokenParser<'src>, errs: &mut Vec<Error<'src>>) -> Self {
                unsafe {
                    mem::transmute(Group::parse_tokens_with($delimiter, parser, errs))
                }
            }
        }
    };
}

group_with!(GroupWithBraces: Delimiter::Brace);
group_with!(GroupWithBrackets: Delimiter::Bracket);
group_with!(GroupWithParenthesis: Delimiter::Parenthesis);