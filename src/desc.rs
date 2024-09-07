use std::{fmt::Display, mem};

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Description {
    pub str: String,
}
impl Description {
    pub fn new(str: impl Into<String>) -> Self {
        Self {
            str: str.into()
        }
    }
    pub fn quote(str: impl Into<String>) -> Self {
        Self {
            str: format!("'{}'", str.into())
        }
    }
    pub fn either(iter: impl IntoIterator<Item = Self>) -> Self {
        let descs = iter.into_iter().map(|desc| desc.str).collect::<Box<[String]>>();
        let (descs_but_last, descs_last) = descs.split_at(descs.len() - 1);
    
        Self {
            str: format!("either {}, or {}", descs_but_last.join(", "), descs_last[0])
        }
    }
    pub fn list(iter: impl IntoIterator<Item = Self>) -> Self {
        let descs = iter.into_iter().map(|desc| desc.str).collect::<Box<[String]>>();
        let (descs_but_last, descs_last) = descs.split_at(descs.len() - 1);
    
        Self {
            str: format!("{}, and {}", descs_but_last.join(", "), descs_last[0])
        }
    }

    pub fn with(&self, with: &Self) -> Self {
        Self::new(format!("{self} with {with}"))
    }
}
impl Display for Description {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.str.fmt(f)
    }
}
pub trait DescriptionArrayExt {
    fn as_strings(&self) -> &[String];
}
impl DescriptionArrayExt for [Description] {
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