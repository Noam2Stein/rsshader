use std::{fmt::{self, Display, Formatter}, mem};

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Description {
    pub s: String,
}
impl Description {
    #[inline(always)]
    pub fn an_empty_str() -> Self {
        Self::new("an empty string")
    }
    #[inline(always)]
    pub fn a_whole_number() -> Self {
        Self::new("a whole number")
    }
    #[inline(always)]
    pub fn a_decimal_number() -> Self {
        Self::new("a decimal number")
    }

    #[inline(always)]
    pub fn new(s: impl Into<String>) -> Self {
        Self {
            s: s.into()
        }
    }
    #[inline(always)]
    pub fn quote(s: &str) -> Self {
        Self {
            s: format!("'{s}'")
        }
    }
    pub fn either(iter: impl IntoIterator<Item = Self>) -> Self {
        let descs = iter.into_iter().map(|desc| desc.s).collect::<Box<[String]>>();
        let (descs_but_last, descs_last) = descs.split_at(descs.len() - 1);
    
        Self {
            s: format!("either {}, or {}", descs_but_last.join(", "), descs_last[0])
        }
    }
    pub fn list(iter: impl IntoIterator<Item = Self>) -> Self {
        let descs = iter.into_iter().map(|desc| desc.s).collect::<Box<[String]>>();
        let (descs_but_last, descs_last) = descs.split_at(descs.len() - 1);
    
        Self {
            s: format!("{}, and {}", descs_but_last.join(", "), descs_last[0])
        }
    }

    pub fn with(&self, with: &Self) -> Self {
        Self::new(format!("{self} with {with}"))
    }
}
impl Display for Description {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.s.fmt(f)
    }
}
pub trait DescriptionArrayExt {
    fn as_strings(&self) -> &[String];
}
impl DescriptionArrayExt for [Description] {
    #[inline(always)]
    fn as_strings(&self) -> &[String] {
        unsafe {
            mem::transmute(self)
        }
    }
}

pub trait Describe {
    fn desc(&self) -> Description;
}
pub trait TypeDescribe {
    fn type_desc() -> Description;
}