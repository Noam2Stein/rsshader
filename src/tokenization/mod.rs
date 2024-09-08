use std::{write, fmt::{self, Display, Formatter}, str::FromStr};
use errm::{expected, unexpected_end_of_file, unmatched_delimiter};
use logos::{Lexer, Logos};

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
        read(LogosToken::lexer(src), errs)
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

fn read<'a>(mut lexer: Lexer<'a, LogosToken<'a>>, errs: &mut Vec<Error>) -> TokenStream {
    let mut output_tokens = Vec::new();

    while let Some(token) = lexer.next() {
        if let Err(_) = token {
            continue;
        }

        let token_span = lexer.span();
        let token_span = Span::new(token_span.start, token_span.end);
        
        match token.unwrap() {
            LogosToken::Ident(str) => read_ident(&mut output_tokens, errs, str, token_span),
            LogosToken::UnsuffixedIntLiteral(str) => read_unsuffixed_int(&mut output_tokens, errs, str, token_span),
            LogosToken::SuffixedIntLiteral(str) => read_suffixed_int(&mut output_tokens, errs, str, token_span),
            LogosToken::UnsuffixedFloatLiteral(str) => read_unsuffixed_float(&mut output_tokens, errs, str, token_span),
            LogosToken::SuffixedFloatLiteral(str) => read_suffixed_float(&mut output_tokens, errs, str, token_span),
            LogosToken::Punct(str) => read_punct(&mut output_tokens, errs, str, token_span),
            LogosToken::GroupOpen(str) => {
                let (group, escaped_closer) = read_group(
                    Delimiter::from_open_str(str).unwrap(),
                    token_span,
                    &mut lexer,
                    errs
                );
                
                output_tokens.push(TokenTree::Group(group));

                if let Some((close_delimiter, _)) = escaped_closer {
                    errs.push(Error::from_messages(token_span, [
                        ErrorMessage::Problem(format!("closing delimiter without a group to close")),
                        errm::unmatched_delimiter(close_delimiter.open_desc()),
                        errm::expected(close_delimiter.close_desc()),
                    ]));
                }
            }
            LogosToken::GroupClose(str) => {
                let close_delimiter = Delimiter::from_close_str(str).unwrap();
                
                errs.push(Error::from_messages(token_span, [
                    ErrorMessage::Problem(format!("closing delimiter without a group to close")),
                    errm::unmatched_delimiter(close_delimiter.open_desc()),
                    errm::expected(close_delimiter.close_desc()),
                ]));
            }
            LogosToken::NotAToken(str) => {
                errs.push(Error::from_messages(token_span, [
                    errm::is_not(Description::quote(str), Description::new("a token"))
                ]))
            }
            LogosToken::Whitespace => {

            },
        }
    };
    
    TokenStream::new(output_tokens)
}
fn read_group<'a>(open_delimiter: Delimiter, open_span: Span, lexer: &mut Lexer<'a, LogosToken<'a>>, errs: &mut Vec<Error>) -> (Group, Option<(Delimiter, Span)>) {
    let mut output_tokens = Vec::new();

    while let Some(token) = lexer.next() {
        if let Err(_) = token {
            continue;
        }

        let token_span = lexer.span();
        let token_span = Span::new(token_span.start, token_span.end);
        
        match token.unwrap() {
            LogosToken::Ident(str) => read_ident(&mut output_tokens, errs, str, token_span),
            LogosToken::UnsuffixedIntLiteral(str) => read_unsuffixed_int(&mut output_tokens, errs, str, token_span),
            LogosToken::SuffixedIntLiteral(str) => read_suffixed_int(&mut output_tokens, errs, str, token_span),
            LogosToken::UnsuffixedFloatLiteral(str) => read_unsuffixed_float(&mut output_tokens, errs, str, token_span),
            LogosToken::SuffixedFloatLiteral(str) => read_suffixed_float(&mut output_tokens, errs, str, token_span),
            LogosToken::Punct(str) => read_punct(&mut output_tokens, errs, str, token_span),
            LogosToken::GroupOpen(str) => {
                let (group, escaped_closer) = read_group(
                    Delimiter::from_open_str(str).unwrap(),
                    token_span,
                    lexer,
                    errs
                );
                
                output_tokens.push(TokenTree::Group(group));

                if let Some((close_delimiter, close_span)) = escaped_closer {
                    let group_delimiter = open_delimiter;
                    let group_stream = TokenStream::new(output_tokens);
                    
                    let (group_span, excaped_closer) =
                    if close_delimiter == open_delimiter {
                        (
                            Span::connect(open_span, close_span),
                            None
                        )
                    }
                    else {
                        errs.push(Error::from_messages(open_span, [
                            errm::unmatched_delimiter(open_delimiter.open_desc()),
                            errm::expected_found(open_delimiter.close_desc(), close_delimiter.close_desc()),
                        ]));
    
                        (
                            Span::connect(open_span, group_stream.span),
                            Some(
                                (close_delimiter, token_span)
                            )
                        )
                    };
    
                    return (
                        Group {
                            delimiter: group_delimiter,
                            stream: group_stream,
                            span: group_span,
                        },
                        excaped_closer
                    );
                }
            }
            LogosToken::GroupClose(str) => {
                let close_delimiter = Delimiter::from_close_str(str).unwrap();

                let group_delimiter = open_delimiter;
                let group_stream = TokenStream::new(output_tokens);
                
                let (group_span, excaped_closer) =
                if close_delimiter == open_delimiter {
                    (
                        Span::connect(open_span, token_span),
                        None
                    )
                }
                else {
                    errs.push(Error::from_messages(open_span, [
                        errm::unmatched_delimiter(open_delimiter.open_desc()),
                        errm::expected_found(open_delimiter.close_desc(), close_delimiter.close_desc()),
                    ]));

                    (
                        Span::connect(open_span, group_stream.span),
                        Some(
                            (close_delimiter, token_span)
                        )
                    )
                };

                return (
                    Group {
                        delimiter: group_delimiter,
                        stream: group_stream,
                        span: group_span,
                    },
                    excaped_closer
                );
            }
            LogosToken::NotAToken(str) => {
                errs.push(Error::from_messages(token_span, [
                    errm::is_not(Description::quote(str), Description::new("a token"))
                ]))
            }
            LogosToken::Whitespace => {

            },
        }
    };
    
    let group_delimiter = open_delimiter;
    let group_stream = TokenStream::new(output_tokens);
    let group_span = Span::connect(open_span, group_stream.span);

    errs.push(
        Error::from_messages(group_span, [
            unmatched_delimiter(group_delimiter.open_desc()),
            unexpected_end_of_file(),
            expected(group_delimiter.close_desc())
        ])
    );

    (
        Group {
            delimiter: group_delimiter,
            stream: group_stream,
            span: group_span,
        },
        None
    )
}
fn read_ident(output: &mut Vec<TokenTree>, _errs: &mut Vec<Error>, str: &str, span: Span) {
    output.push(
        if let Some(keyword) = Keyword::parse(str, span.start()) {
            TokenTree::Keyword(keyword)
        }
        else {
            TokenTree::Ident(Ident {
                str: str.to_string(),
                start: span.start(),
            })
        }
    )
}
fn read_unsuffixed_int(output: &mut Vec<TokenTree>, _errs: &mut Vec<Error>, str: &str, span: Span) {
    output.push(TokenTree::Literal(Literal::Int(IntLiteral {
        value: str.to_string(),
        suffix: None,
        span: span,
    })));
}
fn read_suffixed_int(output: &mut Vec<TokenTree>, _errs: &mut Vec<Error>, str: &str, span: Span) {
    let (value, suffix_str) = str.split_at(str.find(|c: char| c.is_alphabetic()).unwrap());
    output.push(TokenTree::Literal(Literal::Int(IntLiteral {
        value: value.to_string(),
        suffix: IntSuffix::from_str(suffix_str).ok().or_else(|| {
            _errs.push(Error::from_messages(span, [
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
fn read_unsuffixed_float(output: &mut Vec<TokenTree>, _errs: &mut Vec<Error>, str: &str, span: Span) {
    output.push(TokenTree::Literal(Literal::Float(FloatLiteral {
        value: str.to_string(),
        suffix: None,
        span: span,
    })));
}
fn read_suffixed_float(output: &mut Vec<TokenTree>, _errs: &mut Vec<Error>, str: &str, span: Span) {
    let (value, suffix_str) = str.split_at(str.find(|c: char| c.is_alphabetic()).unwrap());
    output.push(TokenTree::Literal(Literal::Float(FloatLiteral {
        value: value.to_string(),
        suffix: FloatSuffix::from_str(suffix_str).ok().or_else(|| {
            _errs.push(Error::from_messages(span, [
                errm::expected_found(FloatSuffix::type_desc(), Description::quote(suffix_str)),
                errm::valid_forms_are(FloatSuffix::VALUES.map(|suffix| suffix.desc()))
            ]));

            Some(
                FloatSuffix::default()
            )
        }),
        span,
    })));
}
fn read_punct(output: &mut Vec<TokenTree>, _errs: &mut Vec<Error>, str: &str, span: Span) {
    output.push(TokenTree::Punct(Punct::parse(str, span.start()).unwrap()));
}