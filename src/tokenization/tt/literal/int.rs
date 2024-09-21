use super::*;

#[derive(Debug, Clone, Copy, Hash)]
pub struct IntLiteral {
    value: u128,
    suffix: u8,
    span: Span,
}
impl IntLiteral {
    pub fn value(&self) -> u128 {
        self.value
    }
    pub fn suffix(&self) -> Option<IntSuffix> {
        if self.suffix == 0 {
            None
        }
        else {
            Some(
                IntSuffix {
                    id: self.suffix - 1
                }
            )
        }
    }

    pub(in crate::tokenization) fn new(srcslice: &SrcSlice, span: Span, errs: &mut Vec<Error>) -> Self {
        let mid = srcslice.s().find(|c: char| c.is_alphabetic()).unwrap_or(srcslice.s().len());
        let (value_s, suffix_s) = srcslice.s().split_at(mid);
    
        let value = if let Ok(value) = u128::from_str(value_s) {
            value
        }
        else {
            errs.push(Error::from_messages(span, [
                errm::is_too_large_for_the_literal_capacity(Description::quote(value_s))
            ]));
    
            0
        };
    
        let suffix = match IntSuffix::option_from_str(suffix_s) {
            Ok(suffix) => suffix.map_or(0, |suffix| suffix.id + 1),
            Err(err) => {
                errs.push(Error::from_messages(span, [
                    err
                ]));
    
                0
            }
        };
    
        Self {
            value,
            suffix,
            span,
        }
    }
}
impl PartialEq for IntLiteral {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.span.eq(&other.span)
    }
}
impl Eq for IntLiteral {
    
}
impl PartialOrd for IntLiteral {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.span.partial_cmp(&other.span)
    }
}
impl Ord for IntLiteral {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.span.cmp(&other.span)
    }
}
impl Display for IntLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.suffix() {
            Some(suffix) => write!(f, "{}{}, ", self.value, suffix),
            None => write!(f, "{}", self.value)
        }
    }
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
impl Spanned for IntLiteral {
    fn span(&self) -> Span {
        self.span
    }
}
impl UnwrapTokenTree for IntLiteral {
    fn unwrap_tt(tt: TokenTree, errs: &mut Vec<Error>) -> Self {
        if let TokenTree::Literal(tt) = tt {
            if let Literal::Int(tt) = tt {
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
impl TokenDefault for IntLiteral {
    unsafe fn tt_default(span: Span) -> Self {
        Self {
            value: 0,
            suffix: 0,
            span,
        }
    }
}
impl SubToken for IntLiteral {
    
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