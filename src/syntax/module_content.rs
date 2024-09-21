use super::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ModContent {
    pub items: Vec<ModItem>
}
impl TypeDescribe for ModContent {
    fn type_desc() -> Description {
        Description::new("mod content")
    }
}
impl ParseTokens for ModContent {
    fn parse_tokens(parser: &mut impl TokenParser, errs: &mut Vec<Error>) -> Self {
        let mut items = Vec::new();
        while let Some(item) = parser.parse::<Option<ModItem>>(errs) {
            items.push(item);
        }

        Self {
            items,
        }
    }
}
impl DisplayWithSrc for ModContent {
    fn fmt_with_src(&self, f: &mut Formatter, srcfile: &SrcFile) -> fmt::Result {
        for item in &self.items {
            writeln!(f)?;
            item.fmt_with_src(f, srcfile)?;
            writeln!(f)?;
        }

        Ok(())    
    }
}
impl Syntax for ModContent {
    
}