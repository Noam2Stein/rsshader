use super::*;

mod int;
mod float;
pub use int::*;
pub use float::*;

#[derive(Debug, Clone, Copy, Hash)]
pub enum Literal {
    Int(IntLiteral),
    Float(FloatLiteral),
}
impl Literal {
    pub fn literal_type_desc(&self) -> Description {
        match self {
            Self::Int(_) => IntLiteral::type_desc(),
            Self::Float(_) => FloatLiteral::type_desc(),
        }
    }
}
impl PartialEq for Literal {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.span().eq(&other.span())
    }
}
impl Eq for Literal {
    
}
impl PartialOrd for Literal {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.span().partial_cmp(&other.span())
    }
}
impl Ord for Literal {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.span().cmp(&other.span())
    }
}
impl TypeDescribe for Literal {
    #[inline(always)]
    fn type_desc() -> Description {
        Description::new("a literal")
    }
}
impl Spanned for Literal {
    fn span(&self) -> Span {
        match self {
            Self::Int(literal) => literal.span(),
            Self::Float(literal) => literal.span(),
        }
    }
}
impl TokenDisplay for Literal {
    fn tt_to_string(&self, srcfile: &SrcFile) -> String {
        match self {
            Self::Int(literal) => literal.tt_to_string(srcfile),
            Self::Float(literal) => literal.tt_to_string(srcfile),
        } 
    }
}
impl UnwrapTokenTree for Literal {
    fn unwrap_tt(tt: TokenTree, errs: &mut Vec<Error>) -> Self {
        if let TokenTree::Literal(tt) = tt {
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
impl TokenDefault for Literal {
    #[inline(always)]
    fn tt_default(span: Span) -> Self {
        Self::Int(IntLiteral::tt_default(span))
    }
}
impl _ValidatedTokenTree for Literal {
    
}