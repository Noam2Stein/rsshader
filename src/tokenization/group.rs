use std::{fmt::{self, Display, Formatter}, mem};

use crate::{desc::*, error::*, span::*, tokenization::*};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Group<'src> {
    pub delimiter: Delimiter,
    pub stream: TokenStream<'src>,
    pub span: Span,
}
impl<'src> Group<'src> {
    pub fn parse_tokens_with(delimiter: Delimiter, mut tokens: impl Iterator<Item = TokenTree<'src>>, src: &'src SrcFile, errs: &mut Vec<Error>) -> Self {
        if let Some(token) = tokens.next() {
            if let TokenTree::Group(token) = token {
                if token.delimiter == delimiter {
                    token
                }
                else {
                    errs.push(Error::from_messages(token.span(), [
                        errm::expected_found(Self::type_desc().with(&delimiter.desc()), Self::type_desc().with(&token.delimiter.desc()))
                    ]));

                    Self {
                        delimiter,
                        stream: token.stream,
                        span: token.span,
                    }
                }
            }
            else {
                errs.push(Error::from_messages(token.span(), [
                    errm::expected_found(Self::type_desc().with(&delimiter.desc()), token.token_type_desc())
                ]));

                Self {
                    delimiter: Delimiter::InvalidAny,
                    stream: TokenStream::default(),
                    span: token.span(),
                }
            }
        }
        else {
            errs.push(Error::from_messages( src.span().last_byte(), [
                errm::unexpected_end_of_file(),
                errm::expected(Self::type_desc().with(&delimiter.desc()))
            ]));

            Self {
                delimiter: Delimiter::InvalidAny,
                stream: TokenStream::default(),
                span: Span::EMPTY,
            }
        }
    }

    #[inline(always)]
    pub fn open_span(&self) -> Span {
        self.span.first_byte()
    }
    #[inline(always)]
    pub fn close_span(&self) -> Span {
        self.span.last_byte()
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
    fn span(&self) -> Span {
        self.span
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
impl<'src> ParseTokens<'src> for Group<'src> {
    fn parse_tokens(mut tokens: impl Iterator<Item = TokenTree<'src>>, src: &'src SrcFile, errs: &mut Vec<Error>) -> Self {
        if let Some(token) = tokens.next() {
            if let TokenTree::Group(token) = token {
                token
            }
            else {
                errs.push(Error::from_messages(token.span(), [
                    errm::expected_found(Self::type_desc(), token.token_type_desc())
                ]));

                Self {
                    delimiter: Delimiter::InvalidAny,
                    stream: TokenStream::default(),
                    span: token.span(),
                }
            }
        }
        else {
            errs.push(Error::from_messages( src.span().last_byte(), [
                errm::unexpected_end_of_file(),
                errm::expected(Self::type_desc())
            ]));

            Self {
                delimiter: Delimiter::InvalidAny,
                stream: TokenStream::default(),
                span: Span::EMPTY,
            }
        }   
    }
}
impl<'src> TokenTypeValidation<'src> for Group<'src> {
    
}
impl<'src> SpannedTokenTypeValidation<'src> for Group<'src> {

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
            fn parse_tokens(tokens: impl Iterator<Item = TokenTree<'src>>, src: &'src SrcFile, errs: &mut Vec<Error>) -> Self {
                unsafe {
                    mem::transmute(Group::parse_tokens_with($delimiter, tokens, src, errs))
                }
            }
        }
    };
}

group_with!(GroupWithBraces: Delimiter::Brace);
group_with!(GroupWithBrackets: Delimiter::Bracket);
group_with!(GroupWithParenthesis: Delimiter::Parenthesis);