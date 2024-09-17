use std::{fmt::Display, mem};

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Description {
    pub s: String,
}
impl Description {
    pub fn new(s: impl Into<String>) -> Self {
        Self {
            s: s.into()
        }
    }
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.s.fmt(f)
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