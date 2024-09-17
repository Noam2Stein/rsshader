use std::fmt::{self, Display, Formatter};

use crate::{desc::*, error::*, span::*};
use super::tt::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct TokenStream<'src> {
    pub tokens: Vec<TokenTree<'src>>,
}
impl<'src> TokenStream<'src> {
    pub fn new(tokens: Vec<TokenTree<'src>>) -> Self {
        Self {
            tokens,
        }
    }
    pub fn iter<'a>(&'a self) -> TokenStreamIter<'a, 'src> {
        TokenStreamIter {
            stream: self,
            iter: self.tokens.iter()
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

#[derive(Debug, Clone)]
pub struct TokenStreamIter<'stream, 'src> {
    stream: &'stream TokenStream<'src>,
    iter: std::slice::Iter<'stream, TokenTree<'src>>,
}
pub trait FromTokens<'stream, 'src>: Sized {
    fn from_tokens(tokens: &mut TokenStreamIter<'stream, 'src>, errs: &mut Vec<Error>) -> Self;
}
impl<'stream, 'src> TokenStreamIter<'stream, 'src> {
    pub fn stream(&self) -> &'stream TokenStream<'src> {
        self.stream
    }
    pub fn read<T: FromTokens<'stream, 'src>>(&mut self, errs: &mut Vec<Error>) -> T {
        T::from_tokens(self, errs)
    }
}
impl<'stream, 'src> Iterator for TokenStreamIter<'stream, 'src> {
    type Item = &'stream TokenTree<'src>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}