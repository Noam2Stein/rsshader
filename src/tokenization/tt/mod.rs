pub mod keyword;
pub mod ident;
pub mod punct;
pub mod literal;
pub mod group;

pub use keyword::*;
pub use ident::*;
pub use punct::*;
pub use literal::*;
pub use group::*;

use super::*;

#[derive(Debug, Clone, Hash)]
pub enum TokenTree {
    Keyword(Keyword),
    Ident(Ident),
    Punct(Punct),
    Literal(Literal),
    Group(Group),
}
impl TokenTree {
    pub fn token_type_desc(&self) -> Description {
        match self {
            Self::Keyword(_) => Keyword::type_desc(),
            Self::Ident(_) => Ident::type_desc(),
            Self::Punct(_) => Punct::type_desc(),
            Self::Literal(_) => Literal::type_desc(),
            Self::Group(_) => Group::type_desc(),
        }
    }
}
impl PartialEq for TokenTree {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.span().eq(&other.span())
    }
}
impl Eq for TokenTree {
    
}
impl PartialOrd for TokenTree {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.span().partial_cmp(&other.span())
    }
}
impl Ord for TokenTree {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.span().cmp(&other.span())
    }
}
impl Spanned for TokenTree {
    fn span(&self) -> Span {
        match self {
            Self::Keyword(tt) => tt.span(),
            Self::Ident(tt) => tt.span(),
            Self::Punct(tt) => tt.span(),
            Self::Literal(tt) => tt.span(),
            Self::Group(tt) => tt.span(),
        }
    }
}
impl TypeDescribe for TokenTree {
    fn type_desc() -> Description {
        Description::new("a token tree")
    }
}
impl TokenDisplay for TokenTree {
    fn tt_to_string(&self, srcfile: &SrcFile) -> String {
        match self {
            Self::Keyword(tt) => tt.tt_to_string(srcfile),
            Self::Ident(tt) => tt.tt_to_string(srcfile),
            Self::Punct(tt) => tt.tt_to_string(srcfile),
            Self::Literal(tt) => tt.tt_to_string(srcfile),
            Self::Group(tt) => tt.tt_to_string(srcfile),
        }
    }
}
impl UnwrapTokenTree for TokenTree {
    fn unwrap_tt(tt: TokenTree, _errs: &mut Vec<Error>) -> Self {
        tt
    }
}
impl TokenDefault for TokenTree {
    fn tt_default(span: Span) -> Self {
        Self::Ident(Ident::tt_default(span))
    }
}
impl _ValidatedTokenTree for TokenTree {

}

pub trait TokenDisplay {
    fn tt_to_string(&self, srcfile: &SrcFile) -> String;
}
impl<T: Display> TokenDisplay for T {
    fn tt_to_string(&self, _srcfile: &SrcFile) -> String {
        self.to_string()
    }
}

pub trait UnwrapTokenTree {
    fn unwrap_tt(tt: TokenTree, errs: &mut Vec<Error>) -> Self;
}

trait TokenDefault {
    fn tt_default(span: Span) -> Self;
}

trait _ValidatedTokenTree:
fmt::Debug +
Clone +
Eq +
Ord +
Hash +
TypeDescribe +
Spanned +
TokenDisplay +
UnwrapTokenTree +
TokenDefault +
{

}