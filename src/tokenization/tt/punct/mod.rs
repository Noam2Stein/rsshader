use super::*;

#[derive(Debug, Clone, Copy, Hash)]
pub struct Punct {
    id: u8,
    span_start: usize,
}
impl Punct {
    pub const STRS: &'static [&'static str] = &[
        "`",
        "~",
        "!",
        "@",
        "#",
        "$",
        "%",
        "^",
        "&",
        "*",
        "(",
        ")",
        "-",
        "=",
        "+",
        "\\",
        "|",
        ";",
        ":",
        "'",
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
    ];

    #[inline(always)]
    pub const fn s(&self) -> &'static str {
        Self::STRS[self.id as usize]
    }

    #[inline(always)]
    pub(in crate::tokenization) fn new(srcslice: &SrcSlice, span: Span) -> Self {
        let position = Self::STRS.iter().position(|keyword| srcslice.s() == *keyword).unwrap();
        Self {
            id: position as u8,
            span_start: span.start(),
        }
    }
}
impl PartialEq for Punct {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.span_start.eq(&other.span_start)
    }
}
impl Eq for Punct {
    
}
impl PartialOrd for Punct {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.span_start.partial_cmp(&other.span_start)
    }
}
impl Ord for Punct {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.span_start.cmp(&other.span_start)
    }
}
impl Display for Punct {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.s().fmt(f)
    }
}
impl Describe for Punct {
    #[inline(always)]
    fn desc(&self) -> Description {
        Description::quote(self.s())
    }
}
impl TypeDescribe for Punct {
    #[inline(always)]
    fn type_desc() -> Description {
        Description::new("a punct")
    }
}
impl Spanned for Punct {
    #[inline(always)]
    fn span(&self) -> Span {
        Span::sized(self.span_start, self.s().len())
    }
}
impl UnwrapTokenTree for Punct {
    fn unwrap_tt(tt: TokenTree, errs: &mut Vec<Error>) -> Self {
        if let TokenTree::Punct(tt) = tt {
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
impl TokenDefault for Punct {
    #[inline(always)]
    unsafe fn tt_default(span: Span) -> Self {
        Self {
            id: 0,
            span_start: span.start()
        }
    }
}
impl UnwrapTokenTreeExpect<&str> for Punct {
    fn unwrap_tt_expect(tt: TokenTree, expect: &str, errs: &mut Vec<Error>) -> Self {
        if let TokenTree::Punct(tt) = tt {
            if tt.s() != expect {
                errs.push(Error::from_messages(tt.span(), [
                    errm::expected_found(Self::expect_desc(expect), tt.desc())
                ]));
            }

            Self {
                id: Self::STRS.iter().position(|item| item == &expect).unwrap() as u8,
                span_start: tt.span_start
            }
        }
        else {
            errs.push(Error::from_messages(tt.span(), [
                errm::expected_found(Self::expect_desc(expect), tt.token_type_desc())
            ]));

            unsafe {
                Self::tt_default(tt.span())
            }
        }
    }
    fn expect_desc(expect: &str) -> Description {
        Description::quote(expect)
    }
}
impl SubToken for Punct {
    
}