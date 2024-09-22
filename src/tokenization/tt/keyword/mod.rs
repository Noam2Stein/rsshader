use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RawKeyword {
    id: u8,
}
impl RawKeyword {
    pub const STRS: &'static [&'static str] = &[
        "pub",
        "const",
        "fn",
        "struct",
        "use",
        "enum",
        "pipeline",
        "loop",
        "return",
        "break",
        "mod",
        "continue",
        "while",
        "for",
        "where",
        "as",
        "in",
    ];
    pub const fn from_str(s: &str) -> Option<Self> {
        // this fn could be faster when rust allows const hashmaps
        let mut position = 0;
        // while Self::STRS[position] != position {
        while !'eq: {
            if Self::STRS[position].len() == s.len() {
                let mut i = 0;
                while i < s.len() {
                    if Self::STRS[position].as_bytes()[i] != s.as_bytes()[i] {
                        break 'eq false;
                    }
                    i += 1;
                }

                true
            }
            else {
                false
            }
        } {    
            position += 1;
            if position >= Self::STRS.len() {
                return None;
            }
        }

        Some(
            Self {
                id: position as u8,
            }
        )
    }
    pub const fn str(self) -> &'static str {
        Self::STRS[self.id as usize]
    }
}
impl Display for RawKeyword {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.str().fmt(f)
    }
}
impl FromStr for RawKeyword {
    type Err = ErrorMessage;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(position) = Self::STRS.iter().position(|keyword| *keyword == s) {
            Ok(
                Self {
                    id: position as u8,
                }
            )
        }
        else {
            Err(errm::is_not(Description::quote(s), Self::type_desc()))
        }
    }
}
impl Describe for RawKeyword {
    fn desc(&self) -> Description {
        Description::quote(&self.str())
    }
}
impl TypeDescribe for RawKeyword {
    fn type_desc() -> Description {
        Description::quote("a keyword")
    }
}
impl UnwrapTokenTreeExpect for RawKeyword {
    type Output = Keyword;
    fn unwrap_tt_expect(self, tt: TokenTree, errs: &mut Vec<Error>) -> Self::Output {
        if let TokenTree::Keyword(tt) = tt {
            if tt.raw == self {
                tt
            }
            else {
                errs.push(Error::from_messages(tt.span(), [
                    errm::expected_found(self.desc(), tt.desc())
                ]));

                Keyword {
                    raw: self,
                    span_start: tt.span_start,
                }
            }
        }
        else {
            errs.push(Error::from_messages(tt.span(), [
                errm::expected_found(self.desc(), tt.token_type_desc())
            ]));

            unsafe {
                Keyword::tt_default(tt.span())
            }
        }   
    }
}

#[derive(Debug, Clone, Copy, Hash)]
pub struct Keyword {
    raw: RawKeyword,
    span_start: usize,
}
impl Keyword {
    pub const STRS: &'static [&'static str] = RawKeyword::STRS;

    #[inline(always)]
    pub const fn raw(&self) -> RawKeyword {
        self.raw
    }
    #[inline(always)]
    pub const fn str(&self) -> &'static str {
        self.raw.str()
    }

    #[inline(always)]
    pub(in crate::tokenization) fn new(srcslice: &SrcSlice, span: Span) -> Option<Self> {
        Self::STRS.iter().position(|keyword| srcslice.s() == *keyword).map(|position|
            Self {
                raw: RawKeyword {
                    id: position as u8
                },
                span_start: span.start(),
            }
        )
    }
}
impl PartialEq for Keyword {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.span_start.eq(&other.span_start)
    }
}
impl Eq for Keyword {
    
}
impl PartialOrd for Keyword {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.span_start.partial_cmp(&other.span_start)
    }
}
impl Ord for Keyword {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.span_start.cmp(&other.span_start)
    }
}
impl Display for Keyword {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.raw.fmt(f)
    }
}
impl Describe for Keyword {
    #[inline(always)]
    fn desc(&self) -> Description {
        RawKeyword::desc(&self.raw)
    }
}
impl TypeDescribe for Keyword {
    #[inline(always)]
    fn type_desc() -> Description {
        RawKeyword::type_desc()
    }
}
impl Spanned for Keyword {
    #[inline(always)]
    fn span(&self) -> Span {
        Span::sized(self.span_start, self.str().len())
    }
}
impl UnwrapTokenTree for Keyword {
    fn unwrap_tt(tt: TokenTree, errs: &mut Vec<Error>) -> Self {
        if let TokenTree::Keyword(tt) = tt {
            tt
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
impl TokenDefault for Keyword {
    #[inline(always)]
    unsafe fn tt_default(span: Span) -> Self {
        Self {
            raw: RawKeyword {
                id: 0,
            },
            span_start: span.start()
        }
    }
}
impl SubToken for Keyword {
    
}