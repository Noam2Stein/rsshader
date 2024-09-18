use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct IntLiteral {
    pub value: u128,
    pub suffix: Option<IntSuffix>,
}
impl IntLiteral {
    pub unsafe fn from_str_unchecked(str: &str) -> Self {
        let (value_str, suffix_str) = str.split_at(str.find(|c: char| c.is_alphabetic()).unwrap_or(str.len()));

        Self {
            value: u128::from_str(value_str).unwrap(),
            suffix: if suffix_str.len() > 0 {
                Some(IntSuffix::from_str(suffix_str).unwrap())
            }
            else {
                None
            },
        }
    }
    pub fn from_str_unsuffixed(str: &str) -> Result<Self, String> {
        let output = Self::from_str(str)?;
        if output.suffix == None {
            Ok(output)
        }
        else {
            Err(format!("expected an unsuffixed int literal, found '{output}'"))
        }
    }
    pub unsafe fn from_str_unsuffixed_unchecked(str: &str) -> Self {
        Self {
            value: u128::from_str(str).unwrap(),
            suffix: None,
        }
    }
    pub fn from_str_suffixed(str: &str) -> Result<Self, String> {
        let output = Self::from_str(str)?;
        if let Some(_) = output.suffix {
            Ok(output)
        }
        else {
            Err(format!("expected a suffixed int literal, found '{output}'"))
        }
    }
    pub unsafe fn from_str_suffixed_unchecked(str: &str) -> Self {
        let (value_str, suffix_str) = str.split_at(str.find(|c: char| c.is_alphabetic()).unwrap_or(str.len()));

        Self {
            value: u128::from_str(value_str).unwrap(),
            suffix: Some(IntSuffix::from_str(suffix_str).unwrap()),
        }
    }

    #[inline(always)]
    pub fn with_suffix(&self, suffix: Option<IntSuffix>) -> Self {
        Self {
            value: self.value,
            suffix,
        }
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
impl FromStr for IntLiteral {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (value_str, suffix_str) = s.split_at(s.find(|c: char| c.is_alphabetic()).unwrap_or(s.len()));

        if value_str.len() == 0 {
            Err(format!("'' is an invalid int"))
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
                }
            )
        }
        else {
            Err(format!("'{value_str}' is too large for the literal capacity: {}", u128::MAX))
        }
    }
}
impl WrapSpannable for IntLiteral {
    type Wrapper = SpannedIntLiteral;
}
impl Describe for IntLiteral {
    fn desc(&self) -> Description {
        Description::quote(&self.to_string())
    }
}
impl TypeDescribe for IntLiteral {
    fn type_desc() -> Description {
        Description::new("an int literal")
    }
}
impl<'src> ValidatedTokenType<'src> for IntLiteral {
    
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpannedIntLiteral {
    inner: IntLiteral,
    span: Span,
}
impl Display for SpannedIntLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}
impl Spanned for SpannedIntLiteral {
    fn span(&self) -> Span {
        self.span
    }
}
impl WrapSpanned for SpannedIntLiteral {
    type Inner = IntLiteral;
    fn inner(&self) -> &Self::Inner {
        &self.inner
    }
    fn into_inner(self) -> Self::Inner {
        self.inner
    }
}
impl<'src> ParseTokens<'src> for SpannedIntLiteral {
    fn parse_tokens(mut tokens: impl Iterator<Item = TokenTree<'src>>, src: &'src SrcFile, errs: &mut Vec<Error>) -> Self {
        if let Some(token) = tokens.next() {
            if let TokenTree::Literal(token) = token {
                if let SpannedLiteral::Int(token) = token {
                    token
                }
                else {
                    errs.push(Error::from_messages(token.span(), [
                        errm::expected_found(Self::type_desc(), token.literal_type_desc())
                    ]));

                    Self {
                        inner: IntLiteral::default(),
                        span: token.span(),
                    }
                }
            }
            else {
                errs.push(Error::from_messages(token.span(), [
                    errm::expected_found(Self::type_desc(), token.token_type_desc())
                ]));

                Self {
                    inner: IntLiteral::default(),
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
                inner: IntLiteral::default(),
                span: Span::EMPTY,
            }
        }   
    }
}
impl<'src> ValidatedTokenType<'src> for SpannedIntLiteral {
    
}
impl<'src> ValidatedSpannedTokenType<'src> for SpannedIntLiteral {

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
impl Display for IntSuffix {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.str().fmt(f)
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