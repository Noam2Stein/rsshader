use super::*;

mod delimiter;
pub use delimiter::*;

mod with;
pub use with::*;

#[derive(Debug, Clone, Hash)]
pub struct Group {
    delimiter: Delimiter,
    tts: Vec<TokenTree>,
    span: Span,
}
impl Group {
    #[inline(always)]
    pub const fn delimiter(&self) -> Delimiter {
        self.delimiter
    }
    #[inline(always)]
    pub const fn tts(&self) -> &Vec<TokenTree> {
        &self.tts
    }
    #[inline(always)]
    pub fn into_tts(self) -> Vec<TokenTree> {
        self.tts
    }

    #[inline(always)]
    pub fn open_span(&self) -> Span {
        self.span.first_byte()
    }
    #[inline(always)]
    pub fn close_span(&self) -> Span {
        self.span.last_byte()
    }

    pub(in crate::tokenization) fn new(delimiter: Delimiter, tts: Vec<TokenTree>, span: Span) -> Self {
        Self {
            delimiter,
            tts,
            span,
        }
    }
}
impl PartialEq for Group {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.span.eq(&other.span)
    }
}
impl Eq for Group {
    
}
impl PartialOrd for Group {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.span.partial_cmp(&other.span)
    }
}
impl Ord for Group {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.span.cmp(&other.span)
    }
}
impl Spanned for Group {
    fn span(&self) -> Span {
        self.span
    }
}
impl TypeDescribe for Group {
    fn type_desc() -> Description {
        Description::new("a group")
    }
}
impl DisplayWithSrc for Group {
    fn fmt_with_src(&self, f: &mut Formatter, srcfile: &SrcFile) -> fmt::Result {
        write!(f, "{} ", self.delimiter.open_str())?;
        for tt in &self.tts {
            tt.fmt_with_src(f, srcfile)?;
            write!(f, " ")?;
        }
        write!(f, "{}", self.delimiter.close_str())
    }
}
impl UnwrapTokenTree for Group {
    fn unwrap_tt(tt: TokenTree, errs: &mut Vec<Error>) -> Self {
        if let TokenTree::Group(tt) = tt {
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
impl TokenDefault for Group {
    unsafe fn tt_default(span: Span) -> Self {
        Self {
            delimiter: Delimiter::Brace,
            tts: Vec::new(),
            span,
        }
    }
}
impl SubToken for Group {

}