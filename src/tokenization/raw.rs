use logos::{Lexer, Logos};

use super::*;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RawToken {
    pub span: Span,
    pub ty: RawTokenType,
}
impl TypeDescribe for RawToken {
    fn type_desc() -> Description {
        Description::new("a token")
    }
}

#[derive(Debug, Clone)]
pub struct RawTokenizer<'src> {
    srcfile: &'src SrcFile,
    lexer: Lexer<'src, LogosToken>,
}
impl<'src> RawTokenizer<'src> {
    pub fn new(srcfile: &'src SrcFile) -> Self {
        Self {
            srcfile,
            lexer: LogosToken::lexer(srcfile.s() )
        }
    }

    pub fn srcfile(&self) -> &'src SrcFile {
        self.srcfile
    }
}
impl<'src> Iterator for RawTokenizer<'src> {
    type Item = RawToken;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(token) = self.lexer.next() {
                break Some(
                    RawToken {
                        span: self.lexer.span().into(),
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