use std::fmt::{self, Display, Formatter};

use crate::{desc::*, error::*, span::*, tokenization::*};

pub const KEYWORDS: &[&str] = &[
    "pub",
    "const",
    "fn",
    "struct",
    "use",
    "enum",
    "pipeline",
    "loop",
    "return",
    "break",
    "mod",
    "continue",
    "while",
    "for",
    "where",
    "as",
    "in",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Keyword {
    id: u8,
    span: usize,
}
impl Keyword {
    pub fn parse(value: &str, span_start: usize) -> Option<Self> {
        KEYWORDS.into_iter().position(|keyword| value == *keyword).map(|id|
            Self {
                id: id as u8,
                span: span_start,
            }
        )
    }
    
    pub fn str(&self) -> &'static str {
        KEYWORDS[self.id as usize]
    }
}
impl Spanned for Keyword {
    fn span(&self) -> Span {
        Span::sized(self.span, self.str().len())
    }
}
impl Display for Keyword {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.str().fmt(f)
    }
}
impl Describe for Keyword {
    fn desc(&self) -> Description {
        Description::quote(self.str())
    }
}
impl TypeDescribe for Keyword {
    fn type_desc() -> Description {
        Description::new("a keyword")
    }
}
impl<'a> FromTokens<'a> for Keyword {
    fn from_tokens(stream: &mut TokenStreamIter) -> Result<Self> {
        if let Some(token) = stream.next() {
            if let TokenTree::Keyword(output) = token {
                Ok(
                    *output
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