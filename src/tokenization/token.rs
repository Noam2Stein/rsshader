use logos::{Lexer, Logos};

use crate::{desc::*, span::*};

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


pub struct Token<'src> {
    pub str: &'src str,
    pub span: Span,
    pub ty: TokenType,
}
impl<'src> Describe for Token<'src> {
    fn desc(&self) -> Description {
        Description::quote(self.str)
    }
}
impl<'src> TypeDescribe for Token<'src> {
    fn type_desc() -> Description {
        Description::new("a token")
    }
}
pub enum TokenType {
    Ident,
    IntLiteral,
    FloatLiteral,
    Punct,
    GroupOpen,
    GroupClose,
    Invalid,
}

pub struct TokenIter<'src> {
    lexer: Lexer<'src, LogosToken>,
}
impl<'src> TokenIter<'src> {
    pub fn new(src: &'src str) -> Self {
        Self {
            lexer: LogosToken::lexer(src)
        }
    }
}
impl<'src> Iterator for TokenIter<'src> {
    type Item = Token<'src>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(token) = self.lexer.next() {
                let span_range = self.lexer.span();
                let str = &self.lexer.source()[span_range.clone()];
                let span = Span::from(span_range);

                if let Ok(token) = token {
                    match token {
                        LogosToken::Ident => break Some(Token { str, span, ty: TokenType::Ident, }),
                        LogosToken::IntLiteral => break Some(Token { str, span, ty: TokenType::IntLiteral, }),
                        LogosToken::FloatLiteral => break Some(Token { str, span, ty: TokenType::FloatLiteral, }),
                        LogosToken::Punct => break Some(Token { str, span, ty: TokenType::Punct, }),
                        LogosToken::GroupOpen => break Some(Token { str, span, ty: TokenType::GroupOpen, }),
                        LogosToken::GroupClose => break Some(Token { str, span, ty: TokenType::GroupClose, }),
                        LogosToken::NotAToken => break Some(Token { str, span, ty: TokenType::Invalid, }),
                        LogosToken::Whitespace => continue,
                    }
                }
                else {
                    break Some(
                        Token {
                            str,
                            span,
                            ty: TokenType::Invalid,
                        }
                    )
                }
            }
            else {
                break None;
            }
        }
    }
}