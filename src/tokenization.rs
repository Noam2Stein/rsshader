use std::{write, fmt::{self, Display, Formatter}, str::FromStr};
use logos::Logos;

use crate::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct TokenStream {
    pub tts: Vec<TokenTree>
}
impl TokenStream {
    pub fn new(tts: Vec<TokenTree>) -> Self {
        Self {
            tts,
        }
    }
}
impl Display for TokenStream {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.tts.iter().map(|tt| tt.to_string()).collect::<Box<[String]>>().join(" "))
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum TokenTree {
    Keyword(Keyword),
    Ident(Ident),
    Punct(Punct),
    Literal(Literal),
    Group(Group),
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

macro_rules! keyword {
    (
        $(
            $value:ident
        ), *
        $(,)?
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum Keyword {
            $(
                $value { start: usize },
            )*
        }
        impl Keyword {
            pub fn parse(value: &str, start: usize) -> Option<Self> {
                $(
                    if value == stringify!($value).to_lowercase() {
                        Some(
                            Self::$value { start }
                        )
                    }
                    else
                )*
                {
                    None
                }
            }
        }
        impl Spanned for Keyword {
            fn span(&self) -> Span {
                match *self {
                    $(
                        Self::$value { start } => Span::new(start, start + stringify!($value).trim_matches('_').len()),
                    )*
                }
            }
        }
        impl Display for Keyword {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                match *self {
                    $(
                        Self::$value { start: _ } => write!(f, "{}", stringify!($value).to_lowercase()),
                    )*
                }
            }
        }
    };
}
keyword!(
    Pub,
    Use,
    Fn,
    Struct,
    Let,
    Loop,
    While,
    For,
    Impl,
    Mut,
    Self_,
    As,
    Const,
);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ident {
    pub str: String,
    pub start: usize,
}
impl Spanned for Ident {
    fn span(&self) -> Span {
        Span::new(self.start, self.start + self.str.len())
    }
}
impl Display for Ident {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.str)
    }
}

macro_rules! punct {
    (
        $(
            $punct:literal
        ), *
        $(,)?
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct Punct {
            pub value: u8,
            pub start: usize,
        }
        impl Punct {
            pub fn new(value: &str, start: usize) -> Option<Self> {
                [$($punct), *].iter().position(|&punct| value == punct).map(|value|
                    Self {
                        value: value as u8,
                        start,
                    }
                )
            }
        }
        impl Spanned for Punct {
            fn span(&self) -> Span {
                Span::new(self.start, self.start + [$($punct), *][self.value as usize].len())
            }
        }
        impl Display for Punct {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                write!(f, "{}", [$($punct), *][self.value as usize])
            }
        }
    };
}
punct!(
    r"`",
    r"~",
    r"!",
    r"@",
    r"#",
    r"$",
    r"%",
    r"^",
    r"&",
    r"*",
    r"(",
    r")",
    r"-",
    r"=",
    r"+",
    r"\",
    r"|",
    r";",
    r":",
    r"'",
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
);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Literal {
    Int(IntLiteral),
    Float(FloatLiteral),
}
impl Spanned for Literal {
    fn span(&self) -> Span {
        match self {
            Self::Int(literal) => literal.span(),
            Self::Float(literal) => literal.span(),
        }
    }
}
impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(literal) => literal.fmt(f),
            Self::Float(literal) => literal.fmt(f),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IntLiteral {
    pub value: String,
    pub suffix: Option<IntSuffix>,
    pub span: Span,
}
impl Spanned for IntLiteral {
    fn span(&self) -> Span {
        self.span
    }
}
impl Display for IntLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.suffix {
            Some(suffixication) => write!(f, "{}{}, ", self.value, suffixication),
            None => write!(f, "{}", self.value)
        }
    }
}
macro_rules! suffix {
    (
        $ident:ident {
            $(
                $value:ident
            ), *
            $(,)?
        }
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum $ident {
            $(
                $value,
            )*
        }
        impl Display for $ident {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                match *self {
                    $(
                        Self::$value => write!(f, "{}", stringify!($value).to_lowercase()),
                    )*
                }
            }
        }
        impl FromStr for $ident {
            type Err = ();
            fn from_str(value: &str) -> Result<Self, ()> {
                $(
                    if value == stringify!($value).to_lowercase() {
                        Ok(
                            Self::$value
                        )
                    }
                    else
                )*
                {
                    Err(())
                }
            }
        }
    };
}
suffix!(
    IntSuffix {
        U8,
        U16,
        U32,
        U64,
        U128,
        I8,
        I16,
        I32,
        I64,
        I128,
    }
);
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct FloatLiteral {
    pub value: String,
    pub suffix: Option<FloatSuffix>,
    pub span: Span,
}
impl Spanned for FloatLiteral {
    fn span(&self) -> Span {
        self.span
    }
}
impl Display for FloatLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.suffix {
            Some(suffixication) => write!(f, "{}{}, ", self.value, suffixication),
            None => write!(f, "{}", self.value)
        }
    }
}
suffix!(
    FloatSuffix {
        F16,
        F32,
        F64,
    }
);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Group {
    delimiter: Delimiter,
    tts: TokenStream,
    span: Span,
}
impl Group {
    pub fn open_span(&self) -> Span {
        Span::new(self.span.start, self.span.start + 1)
    }
    pub fn close_span(&self) -> Span {
        Span::new(self.span.end - 1, self.span.end)
    }
}
impl Spanned for Group {
    fn span(&self) -> Span {
        self.span
    }
}
impl Display for Group {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.delimiter.open_str(),
            self.tts,
            self.delimiter.close_str(),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Delimiter {
    Brace,
    Bracket,
    Parenthesis,
}
impl Delimiter {
    pub fn from_open_char(c: char) -> Option<Self> {
        match c {
            '{' => Some(Self::Brace),
            '[' => Some(Self::Bracket),
            '(' => Some(Self::Parenthesis),
            _ => None,
        }
    }
    pub fn from_close_char(c: char) -> Option<Self> {
        match c {
            '}' => Some(Self::Brace),
            ']' => Some(Self::Bracket),
            ')' => Some(Self::Parenthesis),
            _ => None,
        }
    }
    pub fn open_str(self) -> &'static str {
        match self {
            Self::Brace => "{",
            Self::Bracket => "[",
            Self::Parenthesis => "(",
        }
    }
    pub fn close_str(self) -> &'static str {
        match self {
            Self::Brace => "}",
            Self::Bracket => "]",
            Self::Parenthesis => ")",
        }
    }
    pub fn str(self) -> &'static str {
        match self {
            Self::Brace => "{}",
            Self::Bracket => "[]",
            Self::Parenthesis => "()",
        }
    }
    
    pub fn open_desc(self) -> &'static str {
        match self {
            Self::Brace => "'{'",
            Self::Bracket => "'['",
            Self::Parenthesis => "'('",
        }
    }
    pub fn close_desc(self) -> &'static str {
        match self {
            Self::Brace => "'}'",
            Self::Bracket => "']'",
            Self::Parenthesis => "')'",
        }
    }
}

pub fn tokenize(source: &str, errs: &mut Vec<Error>) -> TokenStream {
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
                        errs.push(err!(span,
                            expected("an int suffix"),
                            found(format!("'{suffix_str}'"))
                        ));
                        
                        Some(
                            IntSuffix::I32
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
                        errs.push(err!(span,
                            expected("a float suffix"),
                            found(format!("'{suffix_str}'"))
                        ));
                        Some(
                            FloatSuffix::F32
                        )
                    }),
                    span,
                })));
            },
            LogosToken::Punct(str) => {
                layers.last_mut().unwrap().push(TokenTree::Punct(Punct::new(str, span.start).unwrap()));
            },
            LogosToken::GroupOpen(str) => {
                layers.last_mut().unwrap().push(TokenTree::Group(Group {
                    delimiter: Delimiter::from_open_char(str.chars().next().unwrap()).unwrap(),
                    tts: TokenStream::default(),
                    span: Span::new(span.start, source.len()),
                }));

                layers.push(Vec::new());
            }
            LogosToken::GroupClose(str) => {
                let delimiter = Delimiter::from_close_char(str.chars().next().unwrap()).unwrap();
                
                if layers.len() < 2 {
                    errs.push(err!(span,
                        unmatched_delimiter(delimiter.close_desc()),
                        expected(delimiter.open_desc()),
                    ))
                }
                else {
                    let tts = layers.pop().unwrap();

                    if let TokenTree::Group(group) = layers.last_mut().unwrap().last_mut().unwrap() {
                        if delimiter != group.delimiter {
                            errs.push(err!(span,
                                unmatched_delimiter(group.delimiter.open_desc()),
                                expected(group.delimiter.close_desc()),
                                found(delimiter.close_desc())
                            ))
                        }
    
                        group.tts = TokenStream::new(tts);
                        group.span.end = span.end;
                    }
                    else {
                        unreachable!()
                    }
                }
            }
            LogosToken::NotAToken(str) => {
                errs.push(err!(span,
                    not_a_token(format!("'{str}'"))
                ))
            }
            LogosToken::Whitespace => {

            },
        }
    }

    while layers.len() > 1 {
        let tts = layers.pop().unwrap();

        if let TokenTree::Group(group) = layers.last_mut().unwrap().last_mut().unwrap() {
            group.tts = TokenStream::new(tts);
            
            errs.push(err!(group.span(),
                unmatched_delimiter(group.delimiter.open_desc()),
                unexpected_end_of_file(),
                expected(group.delimiter.close_desc())
            ))
        }
        else {
            unreachable!()
        }
    }

    TokenStream::new(layers.into_iter().next().unwrap())

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

fn not_a_token(desc: impl Into<String>) -> String {
    format!("{} is not a token", desc.into())
}
fn unmatched_delimiter(desc: impl Into<String>) -> String {
    format!("unmatched delimiter {}", desc.into())
}