use std::fmt::{self, Display, Formatter};

use crate::{desc::*, error::*, span::*, tokenization::*};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ident {
    pub str: String,
    pub span_start: usize,
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
    fn from_tokens(stream: &mut TokenStreamIter) -> Result<Self> {
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