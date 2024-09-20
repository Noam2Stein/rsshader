use std::fmt::{self, Display, Formatter};

use super::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Error<'src> {
    pub srcslice: &'src SrcSlice,
    pub problems: Box<[String]>,
    pub help: Box<[String]>,
}
impl<'src> Error<'src> {
    pub fn from_messages(srcslice: &'src SrcSlice, messages: impl IntoIterator<Item = ErrorMessage>) -> Self {
        let mut problems = Vec::new();
        let mut help = Vec::new();
        
        for message in messages {
            match message {
                ErrorMessage::Problem(str) => problems.push(str),
                ErrorMessage::Help(str) => help.push(str),
            }
        }

        Self {
            srcslice,
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
impl<'src> Display for Error<'src> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.problems.join(". "))
    }
}
impl<'src> std::error::Error for Error<'src> {

}

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