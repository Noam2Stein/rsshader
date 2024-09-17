use crate::desc::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Delimiter {
    Brace,
    Bracket,
    Parenthesis,
}
impl Delimiter {
    pub const fn from_open_char(c: char) -> Option<Self> {
        match c {
            '{' => Some(Self::Brace),
            '[' => Some(Self::Bracket),
            '(' => Some(Self::Parenthesis),
            _ => None,
        }
    }
    pub const fn from_close_char(c: char) -> Option<Self> {
        match c {
            '}' => Some(Self::Brace),
            ']' => Some(Self::Bracket),
            ')' => Some(Self::Parenthesis),
            _ => None,
        }
    }
    pub fn from_open_str(str: &str) -> Option<Self> {
        if str.len() == 1 {
            Self::from_open_char(str.chars().next().unwrap())
        }
        else {
            None
        }
    }
    pub fn from_close_str(str: &str) -> Option<Self> {
        if str.len() == 1 {
            Self::from_close_char(str.chars().next().unwrap())
        }
        else {
            None
        }
    }

    pub const fn open_str(self) -> &'static str {
        match self {
            Self::Brace => "{",
            Self::Bracket => "[",
            Self::Parenthesis => "(",
        }
    }
    pub const fn close_str(self) -> &'static str {
        match self {
            Self::Brace => "}",
            Self::Bracket => "]",
            Self::Parenthesis => ")",
        }
    }
    pub const fn str(self) -> &'static str {
        match self {
            Self::Brace => "{}",
            Self::Bracket => "[]",
            Self::Parenthesis => "()",
        }
    }
    
    pub fn open_desc(self) -> Description {
        Description::quote(self.open_str())
    }
    pub fn close_desc(self) -> Description {
        Description::quote(self.close_str())
    }
}
impl Describe for Delimiter {
    fn desc(&self) -> Description {
        Description::new(
            match self {
                Self::Brace => "braces",
                Self::Bracket => "brackets",
                Self::Parenthesis => "parenthesis",
            }
        )
    }
}
impl TypeDescribe for Delimiter {
    fn type_desc() -> Description {
        Description::new("a delimiter")
    }
}