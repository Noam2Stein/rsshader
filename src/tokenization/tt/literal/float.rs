use super::*;

#[derive(Debug, Clone, Copy, Hash)]
pub struct FloatLiteral {
    integral_value: u128,
    fractional_value: u128,
    suffix: u8,
    span: Span,
}
impl FloatLiteral {
    pub fn integral_value(&self) -> u128 {
        self.integral_value
    }
    pub fn fractional_value(&self) -> u128 {
        self.fractional_value
    }
    pub fn suffix(&self) -> Option<FloatSuffix> {
        if self.suffix == 0 {
            None
        }
        else {
            Some(
                FloatSuffix {
                    id: self.suffix - 1
                }
            )
        }
    }

    pub(in crate::tokenization) fn new(srcslice: &SrcSlice, span: Span, errs: &mut Vec<Error>) -> Self {
        let mid = srcslice.s().find(|c: char| c.is_alphabetic()).unwrap_or(srcslice.s().len());
        let (value_s, suffix_s) = srcslice.s().split_at(mid);

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
                        errs.push(Error::from_messages(span, [
                            errm::is_too_large_for_the_literal_capacity(Description::quote(integral_value_s))
                        ]));
            
                        0
                    },
                    if let Ok(fractional_value) = u128::from_str(fractional_value_s) {
                        fractional_value
                    }
                    else {
                        errs.push(Error::from_messages(span, [
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
            Ok(suffix) => suffix.map_or(0, |suffix| suffix.id + 1),
            Err(err) => {
                errs.push(Error::from_messages(span, [
                    err
                ]));

                0
            }
        };

        Self {
            integral_value,
            fractional_value,
            suffix,
            span,
        }
    }
}
impl PartialEq for FloatLiteral {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.span.eq(&other.span)
    }
}
impl Eq for FloatLiteral {
    
}
impl PartialOrd for FloatLiteral {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.span.partial_cmp(&other.span)
    }
}
impl Ord for FloatLiteral {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.span.cmp(&other.span)
    }
}
impl Display for FloatLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.suffix() {
            Some(suffix) => write!(f, "{}.{}{}, ", self.integral_value, self.fractional_value, suffix),
            None => write!(f, "{}.{}", self.integral_value, self.fractional_value)
        }
    }
}
impl Describe for FloatLiteral {
    fn desc(&self) -> Description {
        Description::quote(&self.to_string())
    }
}
impl TypeDescribe for FloatLiteral {
    fn type_desc() -> Description {
        Description::new("an int literal")
    }
}
impl Spanned for FloatLiteral {
    fn span(&self) -> Span {
        self.span
    }
}
impl UnwrapTokenTree for FloatLiteral {
    fn unwrap_tt(tt: TokenTree, errs: &mut Vec<Error>) -> Self {
        if let TokenTree::Literal(tt) = tt {
            if let Literal::Float(tt) = tt {
                tt
            }
            else {
                errs.push(Error::from_messages(tt.span(), [
                    errm::expected_found(Self::type_desc(), tt.literal_type_desc())
                ]));

                unsafe {
                    Self::tt_default(tt.span())
                }
            }
        }
        else {
            errs.push(Error::from_messages(tt.span(), [
                errm::expected_found(Self::type_desc(), tt.token_type_desc())
            ]));

            unsafe {
                Self::tt_default(tt.span())
            }
        }
    }
}
impl TokenDefault for FloatLiteral {
    unsafe fn tt_default(span: Span) -> Self {
        Self {
            integral_value: 0,
            fractional_value: 0,
            suffix: 0,
            span,
        }
    }
}
impl SubToken for FloatLiteral {
    
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