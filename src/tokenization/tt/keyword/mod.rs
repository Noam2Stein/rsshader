use super::*;

#[derive(Debug, Clone, Copy, Hash)]
pub struct Keyword {
    id: u8,
    span_start: usize,
}
impl Keyword {
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

    #[inline(always)]
    pub const fn s(&self) -> &'static str {
        Self::STRS[self.id as usize]
    }

    #[inline(always)]
    pub(in crate::tokenization) fn new(srcslice: &SrcSlice, span: Span) -> Option<Self> {
        Self::STRS.iter().position(|keyword| srcslice.s() == *keyword).map(|position|
            Self {
                id: position as u8,
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
        self.s().fmt(f)
    }
}
impl Describe for Keyword {
    #[inline(always)]
    fn desc(&self) -> Description {
        Description::quote(self.s())
    }
}
impl TypeDescribe for Keyword {
    #[inline(always)]
    fn type_desc() -> Description {
        Description::new("a keyword")
    }
}
impl Spanned for Keyword {
    #[inline(always)]
    fn span(&self) -> Span {
        Span::sized(self.span_start, self.s().len())
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

            Self::tt_default(tt.span())
        }
    }
}
impl TokenDefault for Keyword {
    #[inline(always)]
    fn tt_default(span: Span) -> Self {
        Self {
            id: 0,
            span_start: span.start()
        }
    }
}
impl _ValidatedTokenTree for Keyword {
    
}