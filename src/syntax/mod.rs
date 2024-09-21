use super::{source::*, diagnostic::*, tokenization::*, parsing::*, *};

mod module_content;
mod module_item;
mod module_import;
mod use_stmt;
pub use module_content::*;
pub use module_item::*;
pub use module_import::*;
pub use use_stmt::*;

pub type SyntaxTree = ModContent;

pub trait Syntax:
fmt::Debug +
Clone +
Eq +
Ord +
TypeDescribe +
ParseTokens +
DisplayWithSrc +
{
    
}