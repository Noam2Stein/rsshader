use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubMod {
    pub ident: Ident,
}
impl TypeDescribe for SubMod {
    fn type_desc() -> Description {
        Description::new("a mod import")
    }
}
impl ParseTokens for SubMod {
    fn parse_tokens(parser: &mut impl TokenParser, errs: &mut Vec<Error>) -> Self {
        parser.parse_expect(const {  }, errs);
        let ident = parser.parse(errs);
        parser.parse_expect(punct!(";"), errs);

        Self {
            ident,
        }
    }
}
impl DisplayWithSrc for SubMod {
    fn fmt_with_src(&self, f: &mut Formatter, srcfile: &SrcFile) -> fmt::Result {
        write!(f, "mod {};", self.ident.with_src(srcfile))
    }
}
impl Syntax for SubMod {
    
}