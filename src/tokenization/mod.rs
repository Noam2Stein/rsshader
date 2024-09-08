use std::{write, fmt::{self, Display, Formatter}, str::FromStr};

use crate::{desc::*, error::*, span::*};

mod token;

use token::*;

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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct TokenStream {
    pub tokens: Vec<TokenTree>,
    pub span: Span,
}
impl TokenStream {
    pub fn new(tokens: Vec<TokenTree>) -> Self {
        Self {
            span: if tokens.len() > 0 {
                Span::connect(tokens[0].span(), tokens.last().unwrap().span())
            }
            else {
                Span::EMPTY
            },
            tokens: tokens,
        }
    }
    pub fn iter<'a>(&'a self) -> TokenStreamIter<'a> {
        TokenStreamIter {
            stream: self,
            iter: self.tokens.iter()
        }
    }

    pub fn parse(src: &str, errs: &mut Vec<Error>) -> Self {
        read(TokenIter::new(src), errs)
    }    
}
impl Display for TokenStream {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.tokens.iter().map(|tt| tt.to_string()).collect::<Box<[String]>>().join(" "))
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
            Err(Error::from_messages(stream.span().last_byte(), [
                errm::unexpected_end_of_file(),
                errm::expected(Self::type_desc())
            ]))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct GroupEdge {
    delimiter: Delimiter,
    span: Span,
}
struct ReadGroupOutput {
    group: Group,
    escaped_group_close: Option<GroupEdge>,
}

fn read(mut tokens: TokenIter, errs: &mut Vec<Error>) -> TokenStream {
    fn read_token(token: Token, output: &mut Vec<TokenTree>, errs: &mut Vec<Error>) {
        match token.ty {
            TokenType::Ident => output.push(
                if let Some(keyword) = Keyword::parse(token.str, token.span.start()) {
                    TokenTree::Keyword(keyword)
                }
                else {
                    TokenTree::Ident(Ident {
                        str: token.str.to_string(),
                        span_start: token.span.start(),
                    })
                }
            ),
            TokenType::UnsuffixedIntLiteral => output.push(
                TokenTree::Literal(Literal::Int(IntLiteral {
                    value: token.str.to_string(),
                    suffix: None,
                    span: token.span,
                }))
            ),
            TokenType::SuffixedIntLiteral => {
                let (value_str, suffix_str) = token.str.split_at(token.str.find(|c: char| c.is_alphabetic()).unwrap());
                output.push(
                    TokenTree::Literal(Literal::Int(IntLiteral {
                        value: value_str.to_string(),
                        suffix: IntSuffix::from_str(suffix_str).ok().or_else(|| {
                            errs.push(Error::from_messages(token.span, [
                                errm::expected_found(IntSuffix::type_desc(), Description::quote(suffix_str)),
                                errm::valid_forms_are(IntSuffix::VALUES.map(|suffix| suffix.desc()))
                            ]));
                        
                            Some(
                                IntSuffix::default()
                            )
                        }),
                        span: token.span,
                    }))
                );
            },
            TokenType::UnsuffixedFloatLiteral => output.push(
                TokenTree::Literal(Literal::Float(FloatLiteral {
                    value: token.str.to_string(),
                    suffix: None,
                    span: token.span,
                }))
            ),
            TokenType::SuffixedFloatLiteral => {
                let (value_str, suffix_str) = token.str.split_at(token.str.find(|c: char| c.is_alphabetic()).unwrap());
                output.push(
                    TokenTree::Literal(Literal::Float(FloatLiteral {
                        value: value_str.to_string(),
                        suffix: FloatSuffix::from_str(suffix_str).ok().or_else(|| {
                            errs.push(Error::from_messages(token.span, [
                                errm::expected_found(FloatSuffix::type_desc(), Description::quote(suffix_str)),
                                errm::valid_forms_are(FloatSuffix::VALUES.map(|suffix| suffix.desc()))
                            ]));
            
                            Some(
                                FloatSuffix::default()
                            )
                        }),
                        span: token.span,
                    }))
                );
            }
            TokenType::Punct => output.push(
                TokenTree::Punct(Punct::parse(token.str, token.span.start()).unwrap())
            ),
            TokenType::GroupOpen => {
                unreachable!()
            }
            TokenType::GroupClose => {
                unreachable!()
            }
            TokenType::Invalid => errs.push(Error::from_messages(token.span, [
                errm::is_not(Description::quote(token.str), Token::type_desc())
            ]))
        }
    }
    fn read_group<'a>(open: Token, tokens: &mut TokenIter, errs: &mut Vec<Error>) -> ReadGroupOutput {
        #[inline(always)]
        fn read_group_close(open: GroupEdge, close: GroupEdge, stream: TokenStream, errs: &mut Vec<Error>) -> ReadGroupOutput {
            if close.delimiter == open.delimiter {
                ReadGroupOutput {
                    group: Group {
                        delimiter: open.delimiter,
                        stream: stream,
                        span: open.span.connect(close.span),
                    },
                    escaped_group_close: None,
                }
            }
            else {
                errs.push(Error::from_messages(open.span, [
                    errm::unmatched_delimiter(open.delimiter.open_desc()),
                    errm::expected_found(open.delimiter.close_desc(), close.delimiter.close_desc()),
                ]));
    
                ReadGroupOutput {
                    group: Group {
                        delimiter: open.delimiter,
                        span: open.span.connect(stream.span),
                        stream: stream,
                    },
                    escaped_group_close: Some(close),
                }
            }
        }
    
        let open = GroupEdge {
            delimiter: Delimiter::from_open_str(open.str).unwrap(),
            span: open.span,
        };
    
        let mut output = Vec::new();
    
        while let Some(token) = tokens.next() {        
            match token.ty {
                TokenType::GroupOpen => {
                    let ReadGroupOutput { group, escaped_group_close } = read_group(token, tokens, errs);
                    
                    output.push(TokenTree::Group(group));
    
                    if let Some(escaped_group_close) = escaped_group_close {
                        return read_group_close(open, escaped_group_close, output.into(), errs)
                    }
                }
                TokenType::GroupClose => {
                    let close = GroupEdge {
                        delimiter: Delimiter::from_close_str(token.str).unwrap(),
                        span: token.span,
                    };
    
                    return read_group_close(open, close, output.into(), errs)
                }
                _ => {
                    read_token(token, &mut output, errs)
                }
            }
        };
        
        '_group_unclosed: {
            let stream = TokenStream::from(output);
            let span = open.span.connect(stream.span);
        
            errs.push(
                Error::from_messages(span, [
                    errm::unmatched_delimiter(open.delimiter.open_desc()),
                    errm::unexpected_end_of_file(),
                    errm::expected(open.delimiter.close_desc())
                ])
            );
    
            ReadGroupOutput {
                group: Group {
                    delimiter: open.delimiter,
                    stream,
                    span,
                },
                escaped_group_close: None,
            }
        }
    }
    #[inline(always)]
    fn read_close(close: GroupEdge, errs: &mut Vec<Error>) {
        errs.push(Error::from_messages(close.span, [
            ErrorMessage::Problem(format!("closing delimiter without a group to close")),
            errm::unmatched_delimiter(close.delimiter.open_desc()),
            errm::expected(close.delimiter.close_desc()),
        ]));
    }

    let mut output = Vec::new();

    while let Some(token) = tokens.next() {
        match token.ty {
            TokenType::GroupOpen => {
                let ReadGroupOutput { group, escaped_group_close } = read_group(token, &mut tokens, errs);
                
                output.push(TokenTree::Group(group));

                if let Some(escaped_group_close) = escaped_group_close {
                    read_close(escaped_group_close, errs)
                }
            }
            TokenType::GroupClose => {
                let close_delimiter = Delimiter::from_close_str(token.str).unwrap();
                
                errs.push(Error::from_messages(token.span, [
                    ErrorMessage::Problem(format!("closing delimiter without a group to close")),
                    errm::unmatched_delimiter(close_delimiter.open_desc()),
                    errm::expected(close_delimiter.close_desc()),
                ]));
            }
            _ => {
                read_token(token, &mut output, errs)
            }
        }
    };
    
    output.into()
}