use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FloatLiteral<'src> {
    pub integral_value: u128,
    pub fractional_value: u128,
    pub suffix: Option<FloatSuffix>,
    srcslice: &'src SrcSlice,
}
impl<'src> Display for FloatLiteral<'src> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.suffix {
            Some(suffixication) => write!(f, "{}.{}{}, ", self.integral_value, self.fractional_value, suffixication),
            None => write!(f, "{}.{}", self.integral_value, self.fractional_value)
        }
    }
}
impl<'src> Describe for FloatLiteral<'src> {
    fn desc(&self) -> Description {
        Description::quote(&self.to_string())
    }
}
impl<'src> TypeDescribe for FloatLiteral<'src> {
    fn type_desc() -> Description {
        Description::new("a float literal")
    }
}
impl<'src> GetSrcSlice<'src> for FloatLiteral<'src> {
    fn srcslice(&self) -> &'src SrcSlice {   
        self.srcslice
    }
}
impl<'src> FromSrc<'src> for FloatLiteral<'src> {
    fn from_src(srcslice: &'src SrcSlice) -> Result<Self, ErrorMessage> {
        let mid = srcslice.s().find(|c: char| c.is_alphabetic()).unwrap_or(srcslice.s().len());
        let (value_s, suffix_s) = srcslice.s().split_at(mid);

        let split_value_s = value_s.split(".").collect::<Box<[&str]>>();
        if split_value_s.len() == 2 {
            let integral_value_s = split_value_s[0];
            let fractional_value_s = split_value_s[1];

            if integral_value_s.len() == 0 {
                Err(errm::expected_found(Description::a_whole_number(), Description::an_empty_str()))
            }
            else if integral_value_s.chars().any(|c| !c.is_ascii_digit()) {
                Err(errm::expected_is_not(Description::a_whole_number(), Description::quote(integral_value_s)))
            }
            else if let Ok(integral_value) = u128::from_str(integral_value_s) {
                if fractional_value_s.len() == 0 {
                    Err(errm::expected_found(Description::a_whole_number(), Description::an_empty_str()))
                }
                else if fractional_value_s.chars().any(|c| !c.is_ascii_digit()) {
                    Err(errm::expected_is_not(Description::a_whole_number(), Description::quote(fractional_value_s)))
                }
                else if let Ok(fractional_value) = u128::from_str(fractional_value_s) {
                    Ok(
                        Self {
                            integral_value,
                            fractional_value,
                            suffix: FloatSuffix::option_from_str(suffix_s)?,
                            srcslice,
                        }
                    )
                }
                else {
                    Err(errm::is_too_large_for_the_literal_capacity(Description::quote(fractional_value_s)))
                }
            }
            else {
                Err(errm::is_too_large_for_the_literal_capacity(Description::quote(integral_value_s)))
            }
        }
        else {
            Err(errm::expected_is_not(Description::a_decimal_number(), Description::quote(value_s)))
        }
    }
}
impl<'src> FromSrcUnchecked<'src> for FloatLiteral<'src> {
    unsafe fn from_src_unchecked(srcslice: &'src SrcSlice) -> Self {
        let mid = srcslice.s().find(|c: char| c.is_alphabetic()).unwrap_or(srcslice.s().len());
        let (value_s, suffix_s) = srcslice.s().split_at(mid);

        let split_value_s = value_s.split(".").collect::<Box<[&str]>>();
        let integral_value_s = split_value_s[0];
        let fractional_value_s = split_value_s[1];

        Self {
            integral_value: u128::from_str(integral_value_s).unwrap(),
            fractional_value: u128::from_str(fractional_value_s).unwrap(),
            suffix: FloatSuffix::option_from_str(suffix_s).unwrap(),
            srcslice,
        }
    }
}
impl<'src> DefaultToken<'src> for FloatLiteral<'src> {
    fn default_token(srcslice: &'src SrcSlice) -> Self {
        Self {
            integral_value: 0,
            fractional_value: 0,
            suffix: None,
            srcslice,
        }
    }
}
impl<'src> ParseTokens<'src> for FloatLiteral<'src> {
    fn parse_tokens(parser: &mut impl TokenParser<'src>, errs: &mut Vec<Error<'src>>) -> Self {
        if let Some(token) = parser.next(errs) {
            if let TokenTree::Literal(token) = token {
                if let Literal::Float(token) = token {
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
impl<'src> FromRawToken<'src> for FloatLiteral<'src> {
    fn from_raw_token(raw_token: RawToken<'src>, errs: &mut Vec<Error<'src>>) -> Self {
        let mid = raw_token.srcslice.s().find(|c: char| c.is_alphabetic()).unwrap_or(raw_token.srcslice.s().len());
        let (value_s, suffix_s) = raw_token.srcslice.s().split_at(mid);

        let (integral_value, fractional_value) = {
            let split_value_s = value_s.split(".").collect::<Box<[&str]>>();
            if split_value_s.len() == 2 {
                let integral_value_s = split_value_s[0];
                let fractional_value_s = split_value_s[1];
    
                (
                    if let Ok(integral_value) = u128::from_str(integral_value_s) {
                        integral_value
                    }
                    else {
                        errs.push(Error::from_messages(&raw_token.srcslice, [
                            errm::is_too_large_for_the_literal_capacity(Description::quote(integral_value_s))
                        ]));
            
                        0
                    },
                    if let Ok(fractional_value) = u128::from_str(fractional_value_s) {
                        fractional_value
                    }
                    else {
                        errs.push(Error::from_messages(&raw_token.srcslice, [
                            errm::is_too_large_for_the_literal_capacity(Description::quote(fractional_value_s))
                        ]));
            
                        0
                    }
                )
            }
            else {
                (0, 0)
            }
        };

        let suffix = match FloatSuffix::option_from_str(suffix_s) {
            Ok(suffix) => suffix,
            Err(err) => {
                errs.push(Error::from_messages(&raw_token.srcslice, [
                    err
                ]));

                None
            }
        };

        Self {
            integral_value,
            fractional_value,
            suffix,
            srcslice: raw_token.srcslice,
        }
    }
}
impl<'src> _ValidatedToken<'src> for FloatLiteral<'src> {
    
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FloatSuffix {
    id: u8,
}
impl FloatSuffix {
    pub const STRS: [&'static str; 4] = [
        "f16",
        "f32",
        "f64",
        "f128",
    ];
    pub const VALUES: [Self; Self::STRS.len()] = {
        // temporary solution until rust supports mut refs in const contexts
        [
            Self { id: 0 },
            Self { id: 1 },
            Self { id: 2 },
            Self { id: 3 },
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

    pub const fn str(&self) -> &'static str {
        Self::STRS[self.id as usize]
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
        match Self::STRS.into_iter().position(|keyword| s == keyword) {
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