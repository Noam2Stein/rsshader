use std::fmt::{self, Display, Formatter};

use crate::{desc::*, error::*, span::*, tokenization::{*, keyword::*}};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ident {
    str: String,
    span_start: usize,
}
impl Ident {
    pub fn parse(str: &str, span_start: usize) -> Result<Self, String> {        
        if KEYWORDS.contains(&str) {
            Err(format!("'{str}' is an invalid ident because it's a keyword"))
        }
        else if str.len() == 0 {
            Err(format!("an empty str is an invalid ident"))
        }
        else if str.chars().any(|c| !c.is_ascii_alphabetic() && !c.is_ascii_digit() && !['_'].contains(&c)) {
            Err(format!("'{str}' is an invalid ident because it contains invalid chars"))
        }
        else if str.chars().next().unwrap().is_ascii_digit() {
            Err(format!("'{str}' is an invalid ident because it starts with a digit"))
        }
        else {
            Ok(
                Self {
                    str: str.to_string(),
                    span_start,
                }
            )
        }
    }
    pub unsafe fn parse_unchecked(str: &str, span_start: usize) -> Self {
        Self {
            str: str.to_string(),
            span_start,
        }
    }

    pub fn str(&self) -> &str {
        &self.str
    }
}
impl Spanned for Ident {
    fn span(&self) -> Span {
        Span::new(self.span_start, self.span_start + self.str.len())
    }
}
impl Display for Ident {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.str)
    }
}
impl Describe for Ident {
    fn desc(&self) -> Description {
        Description::quote(&self.str)
    }
}
impl TypeDescribe for Ident {
    fn type_desc() -> Description {
        Description::new("an ident")
    }
}
impl<'a> FromTokens<'a> for Ident {
    fn from_tokens(stream: &mut TokenStreamIter) -> Result<Self, Error> {
        if let Some(token) = stream.next() {
            if let TokenTree::Ident(output) = token {
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