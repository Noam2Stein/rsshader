use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct FloatLiteral {
    pub integral_value: u128,
    pub fractional_value: u128,
    pub suffix: Option<FloatSuffix>,
}
impl FloatLiteral {
    pub unsafe fn from_str_unchecked(s: &str) -> Self {
        let (value_str, suffix_str) = s.split_at(s.find(|c: char| c.is_alphabetic()).unwrap_or(s.len()));
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
        }
    }
    pub fn from_str_unsuffixed(s: &str) -> Result<Self, ErrorMessage> {
        let split_str = s.split(".").collect::<Box<[&str]>>();

        if s.len() == 0 {
            Err(format!("an empty str is an invalid float"))
        }
        else if split_str.len() != 2 || split_str.iter().any(|str| str.chars().any(|c| !c.is_ascii_digit())) {
            Err(format!("expected float, found '{s}'"))
        }
        else if let Ok(integral_value) = u128::from_str(split_str[0]) {
            if let Ok(fractional_value) = u128::from_str(split_str[1]) {
                Ok(
                    Self {
                        integral_value,
                        fractional_value,
                        suffix: None,
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
    pub unsafe fn from_str_unsuffixed_unchecked(s: &str) -> Self {
        let split_str = s.split(".").collect::<Box<[&str]>>();

        Self {
            integral_value: u128::from_str(split_str[0]).unwrap(),
            fractional_value: u128::from_str(split_str[1]).unwrap(),
            suffix: None,
        }
    }
    pub fn from_str_suffixed(s: &str) -> Result<Self, ErrorMessage> {
        let literal = Self::from_str(s)?;
        if let Some(_) = literal.suffix {
            Ok(literal)
        }
        else {
            Err(format!("expected a suffixed float, found '{literal}'"))
        }
    }
    pub unsafe fn from_str_suffixed_unchecked(s: &str) -> Self {
        let (value_str, suffix_str) = s.split_at(s.find(|c: char| c.is_alphabetic()).unwrap());
        let split_value_str = value_str.split(".").collect::<Box<[&str]>>();

        Self {
            integral_value: u128::from_str(split_value_str[0]).unwrap(),
            fractional_value: u128::from_str(split_value_str[1]).unwrap(),
            suffix: Some(FloatSuffix::from_str(suffix_str).unwrap()),
        }
    }

    #[inline(always)]
    pub fn with_suffix(&self, suffix: Option<FloatSuffix>) -> Self {
        Self {
            integral_value: self.integral_value,
            fractional_value: self.fractional_value,
            suffix,
        }
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
impl FromStr for FloatLiteral {
    type Err = ErrorMessage;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (value_str, suffix_str) = s.split_at(s.find(|c: char| c.is_alphabetic()).unwrap_or(s.len()));
        let split_value_str = value_str.split(".").collect::<Box<[&str]>>();

        if value_str.len() == 0 {
            Err(errm::expected_found(Self::type_desc(), Description::quote("")))
        }
        else if split_value_str.len() != 2 || split_value_str.iter().any(|str| str.chars().any(|c| !c.is_ascii_digit())) {
            Err(errm::expected_is_not(Self::type_desc(), Description::quote(s)))
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
                    }
                )
            }
            else {
                Err(ErrorMessage::Problem(format!("'{}' is too large for the literal capacity: {}", split_value_str[1], u128::MAX)))
            }
        }
        else {
            Err(ErrorMessage::Problem(format!("'{}' is too large for the literal capacity: {}", split_value_str[0], u128::MAX)))
        }
    }
}
impl WrapSpannable for FloatLiteral {
    type Wrapper = SpannedFloatLiteral;
}
impl Describe for FloatLiteral {
    fn desc(&self) -> Description {
        Description::quote(&self.to_string())
    }
}
impl TypeDescribe for FloatLiteral {
    fn type_desc() -> Description {
        Description::new("a float literal")
    }
}
impl<'src> ValidatedTokenType<'src> for FloatLiteral {
    
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpannedFloatLiteral {
    inner: FloatLiteral,
    span: Span,
}
impl Display for SpannedFloatLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}
impl Spanned for SpannedFloatLiteral {
    fn span(&self) -> Span {
        self.span
    }    
}
impl WrapSpanned for SpannedFloatLiteral {
    type Inner = FloatLiteral;
    #[inline(always)]
    fn inner(&self) -> &Self::Inner {
        &self.inner
    }
    #[inline(always)]
    fn into_inner(self) -> Self::Inner {
        self.inner
    }
}
impl<'src> FromSrc<'src> for SpannedFloatLiteral {
    fn from_src(src: &'src SrcFile, span: Span, errs: &mut Vec<Error>) -> Self {
        match FloatLiteral::from_str(&src[span]) {
            Ok(inner) => Self {
                inner,
                span,
            },
            Err(err) => {
                errs.push(Error::from_messages(span, [
                    err
                ]));

                Self {
                    inner: FloatLiteral::default(),
                    span,
                }
            }
        }
    }
}
impl<'src> FromSrcUnchecked<'src> for SpannedFloatLiteral {
    unsafe fn from_src_unchecked(src: &'src SrcFile, span: Span, _errs: &mut Vec<Error>) -> Self {
        Self {
            inner: FloatLiteral::from_str_unchecked(&src[span]),
            span,
        }
    }
}
impl<'src> ParseTokens<'src> for SpannedFloatLiteral {
    fn parse_tokens(mut tokens: impl Iterator<Item = TokenTree<'src>>, src: &'src SrcFile, errs: &mut Vec<Error>) -> Self {
        if let Some(token) = tokens.next() {
            if let TokenTree::Literal(token) = token {
                if let SpannedLiteral::Float(token) = token {
                    token.clone()
                }
                else {
                    errs.push(Error::from_messages(token.span(), [
                        errm::expected_found(Self::type_desc(), token.literal_type_desc())
                    ]));

                    Self {
                        inner: FloatLiteral::default(),
                        span: token.span(),
                    }
                }
            }
            else {
                errs.push(Error::from_messages(token.span(), [
                    errm::expected_found(Self::type_desc(), token.token_type_desc())
                ]));

                Self {
                    inner: FloatLiteral::default(),
                    span: token.span(),
                }
            }
        }
        else {
            errs.push(Error::from_messages(src.span().last_byte(), [
                errm::unexpected_end_of_file(),
                errm::expected(Self::type_desc())
            ]));

            Self {
                inner: FloatLiteral::default(),
                span: Span::EMPTY,
            }
        }
    }
}
impl<'src> ValidatedSpannedTokenType<'src> for SpannedFloatLiteral {

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
impl Display for FloatSuffix {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.str().fmt(f)
    }
}
impl FromStr for FloatSuffix {
    type Err = ErrorMessage;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match FLOAT_SUFFIXES.into_iter().position(|keyword| s == keyword) {
            Some(position) => Ok(
                Self {
                    id: position as u8,
                }
            ),
            None => Err(errm::is_not(Description::quote(s), Self::type_desc())),
        }
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