use std::{fmt::{self, Display, Formatter}, str::FromStr};

use crate::{desc::*, error::*, span::*, tokenization::*};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Literal {
    Int(IntLiteral),
    Float(FloatLiteral),
}
impl Literal {
    pub fn literal_type_desc(&self) -> Description {
        match self {
            Self::Int(_) => IntLiteral::type_desc(),
            Self::Float(_) => FloatLiteral::type_desc(),
        }
    }
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
impl Describe for Literal {
    fn desc(&self) -> Description {
        match self {
            Self::Int(literal) => literal.desc(),
            Self::Float(literal) => literal.desc(),
        }
    }
}
impl TypeDescribe for Literal {
    fn type_desc() -> Description {
        Description::new("a literal")
    }
}
impl<'a> FromTokens<'a> for Literal {
    fn from_tokens(stream: &mut TokenStreamIter) -> Result<Self> {
        if let Some(token) = stream.next() {
            if let TokenTree::Literal(output) = token {
                Ok(
                    output.clone()
                )
            }
            else {
                Err(Error::from_messages(token.span(), [
                    errm::expected_found(Self::type_desc(), token.token_type_desc())
                ]))
            }
        }
        else {
            Err(Error::from_messages(stream.span().end(), [
                errm::unexpected_end_of_file(),
                errm::expected(Self::type_desc())
            ]))
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
impl Describe for IntLiteral {
    fn desc(&self) -> Description {
        Description::quote(self.to_string())
    }
}
impl TypeDescribe for IntLiteral {
    fn type_desc() -> Description {
        Description::new("an int literal")
    }
}
impl<'a> FromTokens<'a> for IntLiteral {
    fn from_tokens(stream: &mut TokenStreamIter) -> Result<Self> {
        if let Some(token) = stream.next() {
            if let TokenTree::Literal(literal) = token {
                if let Literal::Int(output) = literal {
                    Ok(
                        output.clone()
                    )
                }
                else {
                    Err(Error::from_messages(token.span(), [
                        errm::expected_found(Self::type_desc(), literal.literal_type_desc())
                    ]))    
                }
            }
            else {
                Err(Error::from_messages(token.span(), [
                    errm::expected_found(Self::type_desc(), token.token_type_desc())
                ]))
            }
        }
        else {
            Err(Error::from_messages(stream.span().end(), [
                errm::unexpected_end_of_file(),
                errm::expected(Self::type_desc())
            ]))
        }
    }
}

const INT_SUFFIXES: [&str; 10] = [
    "u8",
    "u16",
    "u32",
    "u64",
    "u128",
    "i8",
    "i16",
    "i32",
    "i64",
    "i128",
];
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IntSuffix {
    id: u8,
}
impl IntSuffix {    
    pub const VALUES: [Self; INT_SUFFIXES.len()] = {
        // temporary solution until rust supports mut refs in const contexts
        [
            Self { id: 0 },
            Self { id: 1 },
            Self { id: 2 },
            Self { id: 3 },
            Self { id: 4 },
            Self { id: 5 },
            Self { id: 6 },
            Self { id: 7 },
            Self { id: 8 },
            Self { id: 9 },
        ]
    };

    pub const fn str(&self) -> &'static str {
        INT_SUFFIXES[self.id as usize]
    }
}
impl FromStr for IntSuffix {
    type Err = String;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match INT_SUFFIXES.into_iter().position(|keyword| s == keyword) {
            Some(position) => Ok(
                Self {
                    id: position as u8,
                }
            ),
            None => Err(format!("'{s}' is not {}", Self::type_desc())),
        }
    }
}
impl Display for IntSuffix {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.str().fmt(f)
    }
}
impl Describe for IntSuffix {
    fn desc(&self) -> Description {
        Description::quote(self.str())
    }
}
impl TypeDescribe for IntSuffix {
    fn type_desc() -> Description {
        Description::new("an int suffix")
    }
}
impl Default for IntSuffix {
    fn default() -> Self {
        Self::from_str("i32").unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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
impl Describe for FloatLiteral {
    fn desc(&self) -> Description {
        Description::quote(self.to_string())
    }
}
impl TypeDescribe for FloatLiteral {
    fn type_desc() -> Description {
        Description::new("a float literal")
    }
}
impl<'a> FromTokens<'a> for FloatLiteral {
    fn from_tokens(stream: &mut TokenStreamIter) -> Result<Self> {
        if let Some(token) = stream.next() {
            if let TokenTree::Literal(literal) = token {
                if let Literal::Float(output) = literal {
                    Ok(
                        output.clone()
                    )
                }
                else {
                    Err(Error::from_messages(token.span(), [
                        errm::expected_found(Self::type_desc(), literal.literal_type_desc())
                    ]))    
                }
            }
            else {
                Err(Error::from_messages(token.span(), [
                    errm::expected_found(Self::type_desc(), token.token_type_desc())
                ]))
            }
        }
        else {
            Err(Error::from_messages(stream.span().end(), [
                errm::unexpected_end_of_file(),
                errm::expected(Self::type_desc())
            ]))
        }
    }
}

const FLOAT_SUFFIXES: [&str; 4] = [
    "f16",
    "f32",
    "f64",
    "f128",
];
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FloatSuffix {
    id: u8,
}
impl FloatSuffix {
    pub const VALUES: [Self; FLOAT_SUFFIXES.len()] = {
        // temporary solution until rust supports mut refs in const contexts
        [
            Self { id: 0 },
            Self { id: 1 },
            Self { id: 2 },
            Self { id: 3 },
        ]
    };

    pub const fn str(&self) -> &'static str {
        FLOAT_SUFFIXES[self.id as usize]
    }
}
impl FromStr for FloatSuffix {
    type Err = String;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match FLOAT_SUFFIXES.into_iter().position(|keyword| s == keyword) {
            Some(position) => Ok(
                Self {
                    id: position as u8,
                }
            ),
            None => Err(format!("'{s}' is not {}", Self::type_desc())),
        }
    }
}
impl Display for FloatSuffix {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.str().fmt(f)
    }
}
impl Describe for FloatSuffix {
    fn desc(&self) -> Description {
        Description::quote(self.str())
    }
}
impl TypeDescribe for FloatSuffix {
    fn type_desc() -> Description {
        Description::new("a float suffix")
    }
}
impl Default for FloatSuffix {
    fn default() -> Self {
        Self::from_str("f32").unwrap()
    }
}