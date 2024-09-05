use std::fmt::{self, Debug, Display, Formatter};

use crate::*;

pub use crate::err;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Error {
    pub span: Span,
    pub message: String,
}
impl Spanned for Error {
    fn span(&self) -> Span {
        self.span
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl std::error::Error for Error {

}

#[macro_export(local_inner_macros)]
macro_rules! err {
    ($span:expr, $($message:expr), + $(,)?) => {
        Error {
            span: $span,
            message: [$($message), +].join(". "),
        }
    };
}

pub fn expected(expected: impl Into<String>) -> String {
    format!("expected {}", expected.into())
}
pub fn found(found: impl Into<String>) -> String {
    format!("found {}", found.into())
}
pub fn unexpected_end_of_file() -> String {
    format!("unexpected end of file")
}