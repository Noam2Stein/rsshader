use std::fmt::{self, Display, Formatter};

use crate::{desc::*, error::*, span::*};
use super::{tt::*, SrcFile};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TokenStream<'src> {
    pub tokens: Vec<TokenTree<'src>>,
}
impl<'src> TokenStream<'src> {
    pub const fn new(tokens: Vec<TokenTree<'src>>) -> Self {
        Self {
            tokens,
        }
    }
}
impl<'src> Display for TokenStream<'src> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.tokens.iter().map(|tt| tt.to_string()).collect::<Box<[String]>>().join(" "))
    }
}
impl<'src> Spanned for TokenStream<'src> {
    fn span(&self) -> Span {
        if self.tokens.len() == 0 {
            Span::EMPTY
        }
        else {
            Span::connect(self.tokens[0].span(), self.tokens.last().unwrap().span())
        }
    }
}
impl<'src> Describe for TokenStream<'src> {
    fn desc(&self) -> Description {
        Description::new(
            format!("'{}'", self.tokens.iter().map(|tt| tt.to_string()).collect::<Box<[String]>>().join(" "))
        )
    }
}
impl<'src> TypeDescribe for TokenStream<'src> {
    fn type_desc() -> Description {
        Description::new("a token stream")
    }
}
impl<'src> From<Vec<TokenTree<'src>>> for TokenStream<'src> {
    fn from(value: Vec<TokenTree<'src>>) -> Self {
        Self::new(value)
    }
}