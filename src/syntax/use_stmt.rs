use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UseStmt {
    pub path: Ident,
}
impl TypeDescribe for UseStmt {
    fn type_desc() -> Description {
        Description::new("a use stmt")
    }
}
impl ParseTokens for UseStmt {
    fn parse_tokens(parser: &mut impl TokenParser, errs: &mut Vec<Error>) -> Self {
        parser.parse_expect(keyword("ufse"), errs);
        let path = parser.parse(errs);
        parser.parse_expect(punct!(";"), errs);

        Self {
            path,
        }
    }
}
impl DisplayWithSrc for UseStmt {
    fn fmt_with_src(&self, f: &mut Formatter, srcfile: &SrcFile) -> fmt::Result {
        write!(f, "use {};", self.path.with_src(srcfile))
    }
}
impl Syntax for UseStmt {
    
}