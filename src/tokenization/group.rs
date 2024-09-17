use std::{fmt::{self, Display, Formatter}, mem};

use crate::{desc::*, error::*, span::*, tokenization::*};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Group<'src> {
    pub delimiter: Delimiter,
    pub stream: TokenStream<'src>,
    pub span: Span,
}
impl<'src> Group<'src> {
    pub fn from_tokens_with(delimiter: Delimiter, stream: &mut TokenStreamIter) -> Result<Self, Error> {
        if let Some(token) = stream.next() {
            if let TokenTree::Group(output) = token {
                if output.delimiter == delimiter {
                    Ok(
                        output.clone()
                    )
                }
                else {
                    Err(Error::from_messages(token.span(), [
                        errm::expected_found(Self::type_desc().with(&delimiter.desc()), Self::type_desc().with(&output.delimiter.desc()))
                    ]))
                }
            }
            else {
                Err(Error::from_messages(token.span(), [
                    errm::expected_found(Self::type_desc().with(&delimiter.desc()), token.token_type_desc())
                ]))
            }
        }
        else {
            Err(Error::from_messages( stream.span().last_byte(), [
                errm::unexpected_end_of_file(),
                errm::expected(Self::type_desc())
            ]))
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
impl<'src> FromTokens for Group<'src> {
    fn from_tokens(stream: &mut TokenStreamIter) -> Result<Self, Error> {
        if let Some(token) = stream.next() {
            if let TokenTree::Group(output) = token {
                Ok(
                    output.clone()
                )
            }
            else {
                Err(Error::from_messages(token.span(), [
                    errm::expected_found(Self::type_desc(), token.token_type_desc())
                ]))
            }
        }
        else {
            Err(Error::from_messages(stream.span().last_byte(), [
                errm::unexpected_end_of_file(),
                errm::expected(Self::type_desc())
            ]))
        }
    }
}

#[repr(transparent)]
pub struct GroupWithBraces<'src> {
    pub group: Group<'src>,
}
impl<'src> From<GroupWithBraces<'src>> for Group<'src> {
    fn from(value: GroupWithBraces) -> Self {
        value.group
    }
}
impl<'src> FromTokens for GroupWithBraces<'src> {
    fn from_tokens(stream: &mut TokenStreamIter) -> Result<Self, Error> {
        unsafe {
            mem::transmute(Group::from_tokens_with(Delimiter::Brace, stream))
        }
    }
}

#[repr(transparent)]
pub struct GroupWithBrackets<'src> {
    pub group: Group<'src>,
}
impl<'src> From<GroupWithBrackets<'src>> for Group<'src> {
    fn from(value: GroupWithBrackets) -> Self {
        value.group
    }
}
impl<'src> FromTokens for GroupWithBrackets<'src> {
    fn from_tokens(stream: &mut TokenStreamIter) -> Result<Self, Error> {
        unsafe {
            mem::transmute(Group::from_tokens_with(Delimiter::Bracket, stream))
        }
    }
}

#[repr(transparent)]
pub struct GroupWithParenthesis<'src> {
    pub group: Group<'src>,
}
impl<'src> From<GroupWithParenthesis<'src>> for Group<'src> {
    fn from(value: GroupWithParenthesis) -> Self {
        value.group
    }
}
impl<'src> FromTokens for GroupWithParenthesis<'src> {
    fn from_tokens(stream: &mut TokenStreamIter) -> Result<Self, Error> {
        unsafe {
            mem::transmute(Group::from_tokens_with(Delimiter::Parenthesis, stream))
        }
    }
}