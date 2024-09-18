use logos::{Lexer, Logos};

use super::*;

pub trait FromRawToken<'src>: Sized {
    unsafe fn from_raw_token(src: &'src SrcFile, raw_token: RawToken, errs: &mut Vec<Error>) -> Self;
}
pub trait TryFromRawToken<'src>: Sized {
    unsafe fn try_from_raw_token(src: &'src SrcFile, raw_token: RawToken, errs: &mut Vec<Error>) -> Option<Self>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RawToken {
    pub span: Span,
    pub ty: RawTokenType,
}
impl<'src> TypeDescribe for RawToken {
    fn type_desc() -> Description {
        Description::new("a token")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RawTokenType {
    Ident,
    IntLiteral,
    FloatLiteral,
    Punct,
    GroupOpen,
    GroupClose,
    Invalid,
}

#[derive(Debug, Clone)]
pub struct RawTokenIter<'src> {
    lexer: Lexer<'src, LogosToken>,
}
impl<'src> RawTokenIter<'src> {
    pub fn new(src: &'src str) -> Self {
        Self {
            lexer: LogosToken::lexer(src)
        }
    }
    pub fn src(&self) -> &'src SrcFile {
        self.lexer.source().as_ref()
    }
}
impl<'src> Iterator for RawTokenIter<'src> {
    type Item = RawToken;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(token) = self.lexer.next() {
                let span_range = self.lexer.span();
                let span = Span::from(span_range);
                
                break Some(
                    RawToken {
                        span,
                        ty: if let Ok(token) = token {
                            match token {
                                LogosToken::Ident => RawTokenType::Ident,
                                LogosToken::IntLiteral => RawTokenType::IntLiteral,
                                LogosToken::FloatLiteral => RawTokenType::FloatLiteral,
                                LogosToken::Punct => RawTokenType::Punct,
                                LogosToken::GroupOpen => RawTokenType::GroupOpen,
                                LogosToken::GroupClose => RawTokenType::GroupClose,
                                LogosToken::NotAToken => RawTokenType::Invalid,
                                LogosToken::Whitespace => continue,
                            }
                        }
                        else {
                            RawTokenType::Invalid
                        }
                    }
                )
            }
            else {
                break None;
            }
        }
    }
}

#[derive(Logos, Debug, Clone, PartialEq, PartialOrd)]
enum LogosToken {
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", priority = 1)]
    Ident,
    #[regex(r"[0-9]+([a-zA-Z_][a-zA-Z0-9_]*)?", priority = 1)]
    IntLiteral,
    #[regex(r"[0-9]*\.[0-9]+([a-zA-Z_][a-zA-Z0-9_]*)?", priority = 1)]
    FloatLiteral,
    #[regex(r"->|<-|=>|<=|\+=|-=|\*=|/=|%=|!=|\^=|\|=|&=|==|\.\.|[`~!@#\$%\^&\*\-\+=\\\|;:',<\./\?]", priority = 1)]
    Punct,
    #[regex(r"[\(\[\{]", priority = 1)]
    GroupOpen,
    #[regex(r"[\)\]\}]", priority = 1)]
    GroupClose,
    #[regex(r"[^\x00-\x7F]+")]
    NotAToken,
    #[regex(r"\s+", logos::skip, priority = 1)]
    Whitespace,
}