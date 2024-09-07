use std::fmt::{self, Display, Formatter};

use crate::span::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Error {
    pub span: Span,
    pub problems: Box<[String]>,
    pub help: Box<[String]>,
}
impl Error {
    pub fn from_messages(span: Span, messages: impl IntoIterator<Item = ErrorMessage>) -> Self {
        let mut problems = Vec::new();
        let mut help = Vec::new();
        
        for message in messages {
            match message {
                ErrorMessage::Problem(str) => problems.push(str),
                ErrorMessage::Help(str) => help.push(str),
            }
        }

        Self {
            span,
            problems: problems.into_boxed_slice(),
            help: help.into_boxed_slice(),
        }
    }

    pub fn fmt_multiline(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.problems.join("\n"))?;
        if self.help.len() > 0 {
            write!(f, "\n* {}", self.help.join("\n* "))?;
        }

        Ok(())
    }
    pub fn to_string_multiline(&self) -> String {
        let mut output = format!("{}", self.problems.join("\n"));
        if self.help.len() > 0 {
            output += &format!("\n* {}", self.help.join("\n* "));
        }

        output
    }
}
impl Spanned for Error {
    fn span(&self) -> Span {
        self.span
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.problems.join(". "))
    }
}
impl std::error::Error for Error {

}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ErrorMessage {
    Problem(String),
    Help(String)
}
impl Display for ErrorMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Problem(str) => str.fmt(f),
            Self::Help(str) => write!(f, "* {str}")
        }
    }
}

pub mod errm {
    use crate::{desc::*, error::*};

    pub fn expected(expected: Description) -> ErrorMessage {
        ErrorMessage::Problem(format!("expected {expected}"))
    }
    pub fn found(found: Description) -> ErrorMessage {
        ErrorMessage::Problem(format!("found {found}"))
    }
    pub fn expected_found(expected: Description, found: Description) -> ErrorMessage {
        ErrorMessage::Problem(format!("expected {expected}, found {found}"))
    }
    pub fn unexpected_end_of_file() -> ErrorMessage {
        ErrorMessage::Problem(format!("unexpected end of file"))
    }
    pub fn is_not(a: Description, b: Description) -> ErrorMessage {
        ErrorMessage::Problem(format!("{a} is not {b}"))
    }
    pub fn unmatched_delimiter(delimiter: Description) -> ErrorMessage {
        ErrorMessage::Problem(format!("unmatched delimiter {delimiter}"))
    }
    pub fn valid_forms_are(valid_forms: impl IntoIterator<Item = Description>) -> ErrorMessage {
        ErrorMessage::Help(format!("valid forms are {}", Description::list(valid_forms)))
    }
}