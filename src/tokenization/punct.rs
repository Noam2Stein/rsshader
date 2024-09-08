use std::fmt::{self, Display, Formatter};

use crate::{desc::*, error::*, span::*, tokenization::*};

pub const PUNCTS: &[&str] = &[
    "`",
    "~",
    "!",
    "@",
    "#",
    "$",
    "%",
    "^",
    "&",
    "*",
    "(",
    ")",
    "-",
    "=",
    "+",
    "\\",
    "|",
    ";",
    ":",
    "'",
    "\"",
    r",",
    r"<",
    r".",
    r">",
    r"/",
    r"?",
    "!=",
    "%=",
    "^=",
    "&=",
    "*=",
    "-=",
    "+=",
    "==",
    "|=",
    "/=",
    "->",
    "<-",
    "=>",
    "<=",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Punct {
    id: u8,
    span: usize,
}
impl Punct {
    pub fn parse(value: &str, span: usize) -> Option<Self> {
        PUNCTS.into_iter().position(|punct| value == *punct).map(|position|
            Self {
                id: position as u8,
                span,
            }
        )
    }
    
    pub fn str(&self) -> &'static str {
        PUNCTS[self.id as usize]
    }
}
impl Spanned for Punct {
    fn span(&self) -> Span {
        Span::sized(self.span, self.str().len())
    }
}
impl Display for Punct {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.str().fmt(f)
    }
}
impl Describe for Punct {
    fn desc(&self) -> Description {
        Description::quote(self.str())
    }
}
impl TypeDescribe for Punct {
    fn type_desc() -> Description {
        Description::new("a punct")
    }
}
impl<'a> FromTokens<'a> for Punct {
    fn from_tokens(stream: &mut TokenStreamIter) -> Result<Self> {
        if let Some(token) = stream.next() {
            if let TokenTree::Punct(output) = token {
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