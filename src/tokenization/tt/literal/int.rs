use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IntLiteral<'src> {
    pub value: u128,
    pub suffix: Option<IntSuffix>,
    srcslice: &'src SrcSlice,
}
impl<'src> Display for IntLiteral<'src> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.suffix {
            Some(suffixication) => write!(f, "{}{}, ", self.value, suffixication),
            None => write!(f, "{}", self.value)
        }
    }
}
impl<'src> Describe for IntLiteral<'src> {
    fn desc(&self) -> Description {
        Description::quote(&self.to_string())
    }
}
impl<'src> TypeDescribe for IntLiteral<'src> {
    fn type_desc() -> Description {
        Description::new("an int literal")
    }
}
impl<'src> GetSrcSlice<'src> for IntLiteral<'src> {
    fn srcslice(&self) -> &'src SrcSlice {
        self.srcslice
    }
}
impl<'src> FromSrc<'src> for IntLiteral<'src> {
    fn from_src(srcslice: &'src SrcSlice) -> Result<Self, ErrorMessage> {
        let mid = srcslice.s().find(|c: char| c.is_alphabetic()).unwrap_or(srcslice.s().len());
        let (value_s, suffix_s) = srcslice.s().split_at(mid);

        if value_s.len() == 0 {
            Err(errm::expected_found(Description::a_whole_number(), Description::an_empty_str()))
        }
        else if value_s.chars().any(|c| !c.is_ascii_digit()) {
            Err(errm::expected_is_not(Description::a_whole_number(), Description::quote(value_s)))
        }
        else if let Ok(value) = u128::from_str(value_s) {
            Ok(
                Self {
                    value,
                    suffix: IntSuffix::option_from_str(suffix_s)?,
                    srcslice,
                }
            )
        }
        else {
            Err(errm::is_too_large_for_the_literal_capacity(Description::quote(value_s)))
        }
    }
}
impl<'src> FromSrcUnchecked<'src> for IntLiteral<'src> {
    unsafe fn from_src_unchecked(srcslice: &'src SrcSlice) -> Self {
        let mid = srcslice.s().find(|c: char| c.is_alphabetic()).unwrap_or(srcslice.s().len());
        let (value_s, suffix_s) = srcslice.s().split_at(mid);

        Self {
            value: u128::from_str(value_s).unwrap(),
            suffix: IntSuffix::option_from_str(suffix_s).unwrap(),
            srcslice,
        }
    }
}
impl<'src> DefaultToken<'src> for IntLiteral<'src> {
    fn default_token(srcslice: &'src SrcSlice) -> Self {   
        Self {
            value: 0,
            suffix: None,
            srcslice,
        }
    }
}
impl<'src> ParseTokens<'src> for IntLiteral<'src> {
    fn parse_tokens(parser: &mut impl TokenParser<'src>, errs: &mut Vec<Error<'src>>) -> Self {
        if let Some(token) = parser.next(errs) {
            if let TokenTree::Literal(token) = token {
                if let Literal::Int(token) = token {
                    token
                }
                else {
                    errs.push(Error::from_messages(token.srcslice(), [
                        errm::expected_found(Self::type_desc(), token.literal_type_desc())
                    ]));

                    Self::default_token(token.srcslice())
                }
            }
            else {
                errs.push(Error::from_messages(token.srcslice(), [
                    errm::expected_found(Self::type_desc(), token.token_type_desc())
                ]));

                Self::default_token(token.srcslice())
            }
        }
        else {
            errs.push(Error::from_messages(parser.end_srcslice(), [
                errm::unexpected_end_of_file(),
                errm::expected(Self::type_desc())
            ]));

            Self::default_token(parser.end_srcslice().with_len(0))
        }
    }
}
impl<'src> FromRawToken<'src> for IntLiteral<'src> {
    fn from_raw_token(raw_token: RawToken<'src>, errs: &mut Vec<Error<'src>>) -> Self {
        let mid = raw_token.srcslice.s().find(|c: char| c.is_alphabetic()).unwrap_or(raw_token.srcslice.s().len());
        let (value_s, suffix_s) = raw_token.srcslice.s().split_at(mid);
    
        let value = if let Ok(value) = u128::from_str(value_s) {
            value
        }
        else {
            errs.push(Error::from_messages(&raw_token.srcslice, [
                errm::is_too_large_for_the_literal_capacity(Description::quote(value_s))
            ]));
    
            0
        };
    
        let suffix = match IntSuffix::option_from_str(suffix_s) {
            Ok(suffix) => suffix,
            Err(err) => {
                errs.push(Error::from_messages(&raw_token.srcslice, [
                    err
                ]));
    
                None
            }
        };
    
        Self {
            value,
            suffix,
            srcslice: raw_token.srcslice,
        }
    }
}
impl<'src> _ValidatedToken<'src> for IntLiteral<'src> {
    
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IntSuffix {
    id: u8,
}
impl IntSuffix {
    pub const STRS: [&'static str; 10] = [
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
    pub const VALUES: [Self; Self::STRS.len()] = {
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

    pub fn option_from_str(s: &str) -> Result<Option<Self>, ErrorMessage> {
        if s.len() > 0 {
            Ok(Some(Self::from_str(s)?))
        }
        else {
            Ok(None)
        }
    }

    pub const fn s(&self) -> &'static str {
        Self::STRS[self.id as usize]
    }
}
impl Display for IntSuffix {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.s().fmt(f)
    }
}
impl FromStr for IntSuffix {
    type Err = ErrorMessage;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match Self::STRS.into_iter().position(|keyword| s == keyword) {
            Some(position) => Ok(
                Self {
                    id: position as u8,
                }
            ),
            None => Err(
                errm::is_not(Description::quote(s), Self::type_desc())
            ),
        }
    }
}
impl Describe for IntSuffix {
    fn desc(&self) -> Description {
        Description::quote(self.s())
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