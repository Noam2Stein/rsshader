use std::{write, fmt::{self, Display, Formatter}, str::FromStr};
use logos::Logos;

use crate::{desc::*, error::*, span::*};

pub mod keyword;
pub mod ident;
pub mod punct;
pub mod literal;
pub mod group;

use keyword::Keyword;
use ident::Ident;
use punct::Punct;
use literal::{Literal, IntLiteral, IntSuffix, FloatLiteral, FloatSuffix};
use group::{Group, Delimiter};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TokenStream {
    pub tts: Vec<TokenTree>,
    pub span: Span,
}
impl TokenStream {
    pub fn new(tts: Vec<TokenTree>) -> Self {
        Self {
            span: if tts.len() > 0 {
                Span::connect(tts[0].span(), tts.last().unwrap().span())
            }
            else {
                Span::ZERO
            },
            tts,
        }
    }
    pub fn iter<'a>(&'a self) -> TokenStreamIter<'a> {
        TokenStreamIter {
            stream: self,
            iter: self.tts.iter()
        }
    }

    pub fn parse(source: &str, errs: &mut Vec<Error>) -> Self {
        let mut lexer = LogosToken::lexer(source);
    
        let mut layers = vec![Vec::new()];
    
        while let Some(token) = lexer.next() {
            if let Err(_) = token {
                continue;
            }
    
            let span = lexer.span();
            let span = Span::new(span.start, span.end);
            
            match token.unwrap() {
                LogosToken::Ident(ident) => layers.last_mut().unwrap().push(
                    if let Some(keyword) = Keyword::parse(ident, span.start) {
                        TokenTree::Keyword(keyword)
                    }
                    else {
                        TokenTree::Ident(Ident {
                            str: ident.to_string(),
                            start: span.start,
                        })
                    }
                ),
                LogosToken::UnsuffixedIntLiteral(str) => {
                    layers.last_mut().unwrap().push(TokenTree::Literal(Literal::Int(IntLiteral {
                        value: str.to_string(),
                        suffix: None,
                        span,
                    })));
                }
                LogosToken::SuffixedIntLiteral(str) => {
                    let (value, suffix_str) = str.split_at(str.find(|c: char| c.is_alphabetic()).unwrap());
                    layers.last_mut().unwrap().push(TokenTree::Literal(Literal::Int(IntLiteral {
                        value: value.to_string(),
                        suffix: IntSuffix::from_str(suffix_str).ok().or_else(|| {
                            errs.push(Error::from_messages(span, [
                                errm::expected_found(IntSuffix::type_desc(), Description::quote(suffix_str)),
                                errm::valid_forms_are(IntSuffix::VALUES.map(|suffix| suffix.desc()))
                            ]));
                            
                            Some(
                                IntSuffix::default()
                            )
                        }),
                        span,
                    })));
                }
                LogosToken::UnsuffixedFloatLiteral(str) => {
                    layers.last_mut().unwrap().push(TokenTree::Literal(Literal::Float(FloatLiteral {
                        value: str.to_string(),
                        suffix: None,
                        span,
                    })));
                }
                LogosToken::SuffixedFloatLiteral(str) => {
                    let (value, suffix_str) = str.split_at(str.find(|c: char| c.is_alphabetic()).unwrap());
                    layers.last_mut().unwrap().push(TokenTree::Literal(Literal::Float(FloatLiteral {
                        value: value.to_string(),
                        suffix: FloatSuffix::from_str(suffix_str).ok().or_else(|| {
                            errs.push(Error::from_messages(span, [
                                errm::expected_found(FloatSuffix::type_desc(), Description::quote(suffix_str)),
                                errm::valid_forms_are(FloatSuffix::VALUES.map(|suffix| suffix.desc()))
                            ]));

                            Some(
                                FloatSuffix::default()
                            )
                        }),
                        span,
                    })));
                },
                LogosToken::Punct(str) => {
                    layers.last_mut().unwrap().push(TokenTree::Punct(Punct::parse(str, span.start).unwrap()));
                },
                LogosToken::GroupOpen(str) => {
                    layers.last_mut().unwrap().push(TokenTree::Group(Group {
                        delimiter: Delimiter::from_open_char(str.chars().next().unwrap()).unwrap(),
                        stream: TokenStream::default(),
                        span: Span::new(span.start, source.len()),
                    }));
    
                    layers.push(Vec::new());
                }
                LogosToken::GroupClose(str) => {
                    let delimiter = Delimiter::from_close_char(str.chars().next().unwrap()).unwrap();
                    
                    loop {
                        if layers.len() > 1 {
                            let stream = TokenStream::new(layers.pop().unwrap());
                            if let TokenTree::Group(group) = layers.last_mut().unwrap().last_mut().unwrap() {
                                group.stream = stream;
                                
                                if delimiter == group.delimiter {
                                    group.span.end = span.end;
                                    break;
                                }
                                else {
                                    group.span.end = group.stream.span.end;
                                    
                                    errs.push(Error::from_messages(group.span, [
                                        errm::unmatched_delimiter(group.delimiter.open_desc()),
                                        errm::expected_found(group.delimiter.close_desc(), delimiter.close_desc()),
                                    ]))
                                }
                            }
                            else {
                                unreachable!()
                            };
                        }
                        else {
                            errs.push(Error::from_messages(span, [
                                errm::unmatched_delimiter(delimiter.close_desc()),
                                errm::expected(delimiter.open_desc()),
                            ]))
                        }
                    }
                }
                LogosToken::NotAToken(str) => {
                    errs.push(Error::from_messages(span, [
                        errm::is_not(Description::quote(str), Description::new("a token"))
                    ]))
                }
                LogosToken::Whitespace => {
    
                },
            }
        }
    
        while layers.len() > 1 {
            let tts = layers.pop().unwrap();
    
            if let TokenTree::Group(group) = layers.last_mut().unwrap().last_mut().unwrap() {
                group.stream = TokenStream::new(tts, );
                
                errs.push(Error::from_messages(group.span(), [
                    errm::unmatched_delimiter(group.delimiter.open_desc()),
                    errm::unexpected_end_of_file(),
                    errm::expected(group.delimiter.close_desc())
                ]))
            }
            else {
                unreachable!()
            }
        }
    
        Self::new(layers.into_iter().next().unwrap())
    }
}
impl Display for TokenStream {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.tts.iter().map(|tt| tt.to_string()).collect::<Box<[String]>>().join(" "))
    }
}
impl Describe for TokenStream {
    fn desc(&self) -> Description {
        Description::quote(self.to_string())
    }
}
impl TypeDescribe for TokenStream {
    fn type_desc() -> Description {
        Description::new("a token stream")
    }
}
impl From<Vec<TokenTree>> for TokenStream {
    fn from(value: Vec<TokenTree>) -> Self {
        Self::new(value)
    }
}
impl Default for TokenStream {
    fn default() -> Self {
        Self {
            tts: Vec::new(),
            span: Span::ZERO
        }
    }
}

pub struct TokenStreamIter<'stream> {
    stream: &'stream TokenStream,
    iter: std::slice::Iter<'stream, TokenTree>,
}
impl<'stream> TokenStreamIter<'stream> {
    pub fn span(&self) -> Span {
        self.stream.span
    }
    pub fn read<'a, T: FromTokens<'a>>(&'a mut self) -> Result<T> {
        T::from_tokens(self)
    }
}
impl<'a> Iterator for TokenStreamIter<'a> {
    type Item = &'a TokenTree;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

pub trait FromTokens<'a>: Sized {
    fn from_tokens(stream: &mut TokenStreamIter) -> Result<Self>;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenTree {
    Keyword(Keyword),
    Ident(Ident),
    Punct(Punct),
    Literal(Literal),
    Group(Group),
}
impl TokenTree {
    pub fn token_type_desc(&self) -> Description {
        match self {
            Self::Keyword(_) => Keyword::type_desc(),
            Self::Ident(_) => Ident::type_desc(),
            Self::Punct(_) => Punct::type_desc(),
            Self::Literal(_) => Literal::type_desc(),
            Self::Group(_) => Group::type_desc(),
        }
    }
}
impl Spanned for TokenTree {
    fn span(&self) -> Span {
        match self {
            Self::Keyword(tt) => tt.span(),
            Self::Ident(tt) => tt.span(),
            Self::Punct(tt) => tt.span(),
            Self::Literal(tt) => tt.span(),
            Self::Group(tt) => tt.span(),
        }
    }
}
impl Display for TokenTree {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Keyword(tt) => tt.fmt(f),
            Self::Ident(tt) => tt.fmt(f),
            Self::Punct(tt) => tt.fmt(f),
            Self::Literal(tt) => tt.fmt(f),
            Self::Group(tt) => tt.fmt(f),
        }
    }
}
impl Describe for TokenTree {
    fn desc(&self) -> Description {
        match self {
            Self::Keyword(tt) => tt.desc(),
            Self::Ident(tt) => tt.desc(),
            Self::Punct(tt) => tt.desc(),
            Self::Literal(tt) => tt.desc(),
            Self::Group(tt) => tt.desc(),
        }
    }
}
impl TypeDescribe for TokenTree {
    fn type_desc() -> Description {
        Description::new("a token tree")
    }
}
impl<'a> FromTokens<'a> for TokenTree {
    fn from_tokens(stream: &mut TokenStreamIter) -> Result<Self> {
        if let Some(token) = stream.next() {
            Ok(
                token.clone()
            )
        }
        else {
            Err(Error::from_messages(stream.span().end(), [
                errm::unexpected_end_of_file(),
                errm::expected(Self::type_desc())
            ]))
        }
    }
}

#[derive(Logos, Debug, Clone, PartialEq, PartialOrd)]
enum LogosToken<'a> {
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", priority = 1)]
    Ident(&'a str),
    #[regex(r"[0-9]+", priority = 1)]
    UnsuffixedIntLiteral(&'a str),
    #[regex(r"[0-9]+[a-zA-Z_][a-zA-Z0-9_]*", priority = 1)]
    SuffixedIntLiteral(&'a str),
    #[regex(r"[0-9]*\.[0-9]+", priority = 1)]
    UnsuffixedFloatLiteral(&'a str),
    #[regex(r"[0-9]*\.[0-9]+[a-zA-Z_][a-zA-Z0-9_]*", priority = 1)]
    SuffixedFloatLiteral(&'a str),
    #[regex(r"->|<-|=>|<=|\+=|-=|\*=|/=|%=|!=|\^=|\|=|&=|==|\.\.|[`~!@#\$%\^&\*\-\+=\\\|;:',<\./\?]", priority = 1)]
    Punct(&'a str),
    #[regex(r"[\(\[\{]", priority = 1)]
    GroupOpen(&'a str),
    #[regex(r"[\)\]\}]", priority = 1)]
    GroupClose(&'a str),
    #[regex(r"\s+", logos::skip, priority = 1)]
    Whitespace,
    #[regex(r"[^\x00-\x7F]+")]
    NotAToken(&'a str),
}