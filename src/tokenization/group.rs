use std::{fmt::{self, Display, Formatter}, mem};

use crate::{desc::*, error::*, span::*, tokenization::*};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Group {
    pub delimiter: Delimiter,
    pub stream: TokenStream,
    pub span: Span,
}
impl Group {
    #[inline(always)]
    pub fn open_span(&self) -> Span {
        self.span.first_byte()
    }
    #[inline(always)]
    pub fn close_span(&self) -> Span {
        self.span.last_byte()
    }

    pub fn from_tokens_with(delimiter: Delimiter, stream: &mut TokenStreamIter) -> Result<Self> {
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
}
impl Spanned for Group {
    fn span(&self) -> Span {
        self.span
    }
}
impl Display for Group {
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
impl Describe for Group {
    fn desc(&self) -> Description {
        Description::quote(self.to_string())
    }
}
impl TypeDescribe for Group {
    fn type_desc() -> Description {
        Description::new("a group")
    }
}
impl<'a> FromTokens<'a> for Group {
    fn from_tokens(stream: &mut TokenStreamIter) -> Result<Self> {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Delimiter {
    Brace,
    Bracket,
    Parenthesis,
}
impl Delimiter {
    pub fn from_open_char(c: char) -> Option<Self> {
        match c {
            '{' => Some(Self::Brace),
            '[' => Some(Self::Bracket),
            '(' => Some(Self::Parenthesis),
            _ => None,
        }
    }
    pub fn from_close_char(c: char) -> Option<Self> {
        match c {
            '}' => Some(Self::Brace),
            ']' => Some(Self::Bracket),
            ')' => Some(Self::Parenthesis),
            _ => None,
        }
    }
    pub fn from_open_str(str: &str) -> Option<Self> {
        if str.len() == 1 {
            Self::from_open_char(str.chars().next().unwrap())
        }
        else {
            None
        }
    }
    pub fn from_close_str(str: &str) -> Option<Self> {
        if str.len() == 1 {
            Self::from_close_char(str.chars().next().unwrap())
        }
        else {
            None
        }
    }

    pub fn open_str(self) -> &'static str {
        match self {
            Self::Brace => "{",
            Self::Bracket => "[",
            Self::Parenthesis => "(",
        }
    }
    pub fn close_str(self) -> &'static str {
        match self {
            Self::Brace => "}",
            Self::Bracket => "]",
            Self::Parenthesis => ")",
        }
    }
    pub fn str(self) -> &'static str {
        match self {
            Self::Brace => "{}",
            Self::Bracket => "[]",
            Self::Parenthesis => "()",
        }
    }
    
    pub fn open_desc(self) -> Description {
        Description::quote(self.open_str())
    }
    pub fn close_desc(self) -> Description {
        Description::quote(self.close_str())
    }
}
impl Describe for Delimiter {
    fn desc(&self) -> Description {
        Description::new(
            match self {
                Self::Brace => "braces",
                Self::Bracket => "brackets",
                Self::Parenthesis => "parenthesis",
            }
        )
    }
}
impl TypeDescribe for Delimiter {
    fn type_desc() -> Description {
        Description::new("a delimiter")
    }
}

#[repr(transparent)]
pub struct GroupWithBraces {
    group: Group,
}
impl From<GroupWithBraces> for Group {
    fn from(value: GroupWithBraces) -> Self {
        value.group
    }
}
impl<'a> FromTokens<'a> for GroupWithBraces {
    fn from_tokens(stream: &mut TokenStreamIter) -> Result<Self> {
        unsafe {
            mem::transmute(Group::from_tokens_with(Delimiter::Brace, stream))
        }
    }
}

#[repr(transparent)]
pub struct GroupWithBrackets {
    group: Group,
}
impl From<GroupWithBrackets> for Group {
    fn from(value: GroupWithBrackets) -> Self {
        value.group
    }
}
impl<'a> FromTokens<'a> for GroupWithBrackets {
    fn from_tokens(stream: &mut TokenStreamIter) -> Result<Self> {
        unsafe {
            mem::transmute(Group::from_tokens_with(Delimiter::Bracket, stream))
        }
    }
}

#[repr(transparent)]
pub struct GroupWithParenthesis {
    group: Group,
}
impl From<GroupWithParenthesis> for Group {
    fn from(value: GroupWithParenthesis) -> Self {
        value.group
    }
}
impl<'a> FromTokens<'a> for GroupWithParenthesis {
    fn from_tokens(stream: &mut TokenStreamIter) -> Result<Self> {
        unsafe {
            mem::transmute(Group::from_tokens_with(Delimiter::Parenthesis, stream))
        }
    }
}