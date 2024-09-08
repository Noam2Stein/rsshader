use std::{fmt::{self, Display, Formatter}, str::FromStr};

use crate::{desc::*, error::*, span::*, tokenization::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Literal {
    Int(IntLiteral),
    Float(FloatLiteral),
}
impl Literal {
    pub fn parse(str: &str, span_start: usize) -> Result<Self, String> {
        if str.contains(".") {
            FloatLiteral::parse(str, span_start).map(|literal| Self::Float(literal))
        }
        else {
            IntLiteral::parse(str, span_start).map(|literal| Self::Int(literal))
        }
    }
    pub unsafe fn parse_unchecked(str: &str, span_start: usize) -> Self {
        if str.contains(".") {
            Self::Float(FloatLiteral::parse_unchecked(str, span_start))
        }
        else {
            Self::Int(IntLiteral::parse_unchecked(str, span_start))
        }
    }
    pub fn parse_unsuffixed(str: &str, span_start: usize) -> Result<Self, String> {
        if str.contains(".") {
            FloatLiteral::parse_unsuffixed(str, span_start).map(|literal| Self::Float(literal))
        }
        else {
            IntLiteral::parse_unsuffixed(str, span_start).map(|literal| Self::Int(literal))
        }
    }
    pub unsafe fn parse_unsuffixed_unchecked(str: &str, span_start: usize) -> Self {
        if str.contains(".") {
            Self::Float(FloatLiteral::parse_unsuffixed_unchecked(str, span_start))
        }
        else {
            Self::Int(IntLiteral::parse_unsuffixed_unchecked(str, span_start))
        }
    }
    pub fn parse_suffixed(str: &str, span_start: usize) -> Result<Self, String> {
        if str.contains(".") {
            FloatLiteral::parse_suffixed(str, span_start).map(|literal| Self::Float(literal))
        }
        else {
            IntLiteral::parse_suffixed(str, span_start).map(|literal| Self::Int(literal))
        }
    }
    pub unsafe fn parse_suffixed_unchecked(str: &str, span_start: usize) -> Self {
        if str.contains(".") {
            Self::Float(FloatLiteral::parse_suffixed_unchecked(str, span_start))
        }
        else {
            Self::Int(IntLiteral::parse_suffixed_unchecked(str, span_start))
        }
    }

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
    fn from_tokens(stream: &mut TokenStreamIter) -> Result<Self, Error> {
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
            Err(Error::from_messages(stream.span().last_byte(), [
                errm::unexpected_end_of_file(),
                errm::expected(Self::type_desc())
            ]))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct IntLiteral {
    pub value: u128,
    pub suffix: Option<IntSuffix>,
    pub span: Span,
}
impl IntLiteral {
    pub fn parse(str: &str, span_start: usize) -> Result<Self, String> {
        let (value_str, suffix_str) = str.split_at(str.find(|c: char| c.is_alphabetic()).unwrap_or(str.len()));

        if value_str.len() == 0 {
            Err(format!("an empty str is an invalid int"))
        }
        else if value_str.chars().any(|c| !c.is_ascii_digit()) {
            Err(format!("'{value_str}' is an invalid int because it contains non digit chars"))
        }
        else if let Ok(value) = u128::from_str(value_str) {
            Ok(
                Self {
                    value,
                    suffix: if suffix_str.len() > 0 {
                        Some(IntSuffix::from_str(suffix_str)?)
                    }
                    else {
                        None
                    },
                    span: Span::sized(span_start, str.len())
                }
            )
        }
        else {
            Err(format!("'{value_str}' is too large for the literal capacity: {}", u128::MAX))
        }
    }
    pub unsafe fn parse_unchecked(str: &str, span_start: usize) -> Self {
        let (value_str, suffix_str) = str.split_at(str.find(|c: char| c.is_alphabetic()).unwrap_or(str.len()));

        Self {
            value: u128::from_str(value_str).unwrap(),
            suffix: if suffix_str.len() > 0 {
                Some(IntSuffix::from_str(suffix_str).unwrap())
            }
            else {
                None
            },
            span: Span::sized(span_start, str.len())
        }
    }
    pub fn parse_unsuffixed(str: &str, span_start: usize) -> Result<Self, String> {
        let literal = Self::parse(str, span_start)?;
        if literal.suffix == None {
            Ok(literal)
        }
        else {
            Err(format!("expected an unsuffixed int, found '{literal}'"))
        }
    }
    pub unsafe fn parse_unsuffixed_unchecked(str: &str, span_start: usize) -> Self {
        Self {
            value: u128::from_str(str).unwrap(),
            suffix: None,
            span: Span::sized(span_start, str.len())
        }
    }
    pub fn parse_suffixed(str: &str, span_start: usize) -> Result<Self, String> {
        let literal = Self::parse(str, span_start)?;
        if let Some(_) = literal.suffix {
            Ok(literal)
        }
        else {
            Err(format!("expected a suffixed int, found '{literal}'"))
        }
    }
    pub unsafe fn parse_suffixed_unchecked(str: &str, span_start: usize) -> Self {
        let (value_str, suffix_str) = str.split_at(str.find(|c: char| c.is_alphabetic()).unwrap_or(str.len()));

        Self {
            value: u128::from_str(value_str).unwrap(),
            suffix: Some(IntSuffix::from_str(suffix_str).unwrap()),
            span: Span::sized(span_start, str.len())
        }
    }
    pub fn parse_with_suffix(str: &str, span_start: usize, suffix: IntSuffix) -> Result<Self, String> {
        let mut literal = Self::parse_unsuffixed(str, span_start)?;
        literal.suffix = Some(suffix);

        Ok(literal)
    }
    pub unsafe fn parse_with_suffix_unchecked(str: &str, span_start: usize, suffix: IntSuffix) -> Self {
        Self {
            value: u128::from_str(str).unwrap(),
            suffix: Some(suffix),
            span: Span::sized(span_start, str.len())
        }
    }
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
    fn from_tokens(stream: &mut TokenStreamIter) -> Result<Self, Error> {
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
            Err(Error::from_messages(stream.span().last_byte(), [
                errm::unexpected_end_of_file(),
                errm::expected(Self::type_desc())
            ]))
        }
    }
}

pub const INT_SUFFIXES: [&str; 10] = [
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct FloatLiteral {
    pub integral_value: u128,
    pub fractional_value: u128,
    pub suffix: Option<FloatSuffix>,
    pub span: Span,
}
impl FloatLiteral {
    pub fn parse(str: &str, span_start: usize) -> Result<Self, String> {
        let (value_str, suffix_str) = str.split_at(str.find(|c: char| c.is_alphabetic()).unwrap_or(str.len()));
        let split_value_str = value_str.split(".").collect::<Box<[&str]>>();

        if value_str.len() == 0 {
            Err(format!("an empty str is an invalid float"))
        }
        else if split_value_str.len() != 2 || split_value_str.iter().any(|str| str.chars().any(|c| !c.is_ascii_digit())) {
            Err(format!("expected float, found '{value_str}'"))
        }
        else if let Ok(integral_value) = u128::from_str(split_value_str[0]) {
            if let Ok(fractional_value) = u128::from_str(split_value_str[1]) {
                Ok(
                    Self {
                        integral_value,
                        fractional_value,
                        suffix: if suffix_str.len() > 0 {
                            Some(FloatSuffix::from_str(suffix_str)?)
                        }
                        else {
                            None
                        },
                        span: Span::sized(span_start, str.len())
                    }
                )
            }
            else {
                Err(format!("'{}' is too large for the literal capacity: {}", split_value_str[1], u128::MAX))
            }
        }
        else {
            Err(format!("'{}' is too large for the literal capacity: {}", split_value_str[0], u128::MAX))
        }
    }
    pub unsafe fn parse_unchecked(str: &str, span_start: usize) -> Self {
        let (value_str, suffix_str) = str.split_at(str.find(|c: char| c.is_alphabetic()).unwrap_or(str.len()));
        let split_value_str = value_str.split(".").collect::<Box<[&str]>>();

        Self {
            integral_value: u128::from_str(split_value_str[0]).unwrap(),
            fractional_value: u128::from_str(split_value_str[1]).unwrap(),
            suffix: if suffix_str.len() > 0 {
                Some(FloatSuffix::from_str(suffix_str).unwrap())
            }
            else {
                None
            },
            span: Span::sized(span_start, str.len())
        }
    }
    pub fn parse_unsuffixed(str: &str, span_start: usize) -> Result<Self, String> {
        let split_str = str.split(".").collect::<Box<[&str]>>();

        if str.len() == 0 {
            Err(format!("an empty str is an invalid float"))
        }
        else if split_str.len() != 2 || split_str.iter().any(|str| str.chars().any(|c| !c.is_ascii_digit())) {
            Err(format!("expected float, found '{str}'"))
        }
        else if let Ok(integral_value) = u128::from_str(split_str[0]) {
            if let Ok(fractional_value) = u128::from_str(split_str[1]) {
                Ok(
                    Self {
                        integral_value,
                        fractional_value,
                        suffix: None,
                        span: Span::sized(span_start, str.len())
                    }
                )
            }
            else {
                Err(format!("'{}' is too large for the literal capacity: {}", split_str[1], u128::MAX))
            }
        }
        else {
            Err(format!("'{}' is too large for the literal capacity: {}", split_str[0], u128::MAX))
        }
    }
    pub unsafe fn parse_unsuffixed_unchecked(str: &str, span_start: usize) -> Self {
        let split_str = str.split(".").collect::<Box<[&str]>>();

        Self {
            integral_value: u128::from_str(split_str[0]).unwrap(),
            fractional_value: u128::from_str(split_str[1]).unwrap(),
            suffix: None,
            span: Span::sized(span_start, str.len())
        }
    }
    pub fn parse_suffixed(str: &str, span_start: usize) -> Result<Self, String> {
        let literal = Self::parse(str, span_start)?;
        if let Some(_) = literal.suffix {
            Ok(literal)
        }
        else {
            Err(format!("expected a suffixed float, found '{literal}'"))
        }
    }
    pub unsafe fn parse_suffixed_unchecked(str: &str, span_start: usize) -> Self {
        let (value_str, suffix_str) = str.split_at(str.find(|c: char| c.is_alphabetic()).unwrap());
        let split_value_str = value_str.split(".").collect::<Box<[&str]>>();

        Self {
            integral_value: u128::from_str(split_value_str[0]).unwrap(),
            fractional_value: u128::from_str(split_value_str[1]).unwrap(),
            suffix: Some(FloatSuffix::from_str(suffix_str).unwrap()),
            span: Span::sized(span_start, str.len())
        }
    }
    pub fn parse_with_suffix(str: &str, span_start: usize, suffix: FloatSuffix) -> Result<Self, String> {
        let mut literal = Self::parse_unsuffixed(str, span_start)?;
        literal.suffix = Some(suffix);

        Ok(literal)
    }
    pub unsafe fn parse_with_suffix_unchecked(str: &str, span_start: usize, suffix: FloatSuffix) -> Self {
        let split_str = str.split(".").collect::<Box<[&str]>>();

        Self {
            integral_value: u128::from_str(split_str[0]).unwrap(),
            fractional_value: u128::from_str(split_str[1]).unwrap(),
            suffix: Some(suffix),
            span: Span::sized(span_start, str.len())
        }
    }
}
impl Spanned for FloatLiteral {
    fn span(&self) -> Span {
        self.span
    }
}
impl Display for FloatLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.suffix {
            Some(suffixication) => write!(f, "{}.{}{}, ", self.integral_value, self.fractional_value, suffixication),
            None => write!(f, "{}.{}", self.integral_value, self.fractional_value)
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
    fn from_tokens(stream: &mut TokenStreamIter) -> Result<Self, Error> {
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
            Err(Error::from_messages(stream.span().last_byte(), [
                errm::unexpected_end_of_file(),
                errm::expected(Self::type_desc())
            ]))
        }
    }
}

pub const FLOAT_SUFFIXES: [&str; 4] = [
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