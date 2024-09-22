use super::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ModItem {
    SubMod(SubMod),
    UseStmt(UseStmt),
}
impl TypeDescribe for ModItem {
    fn type_desc() -> Description {
        Description::new("a mod item")
    }
}
impl ParseTokens for ModItem {
    fn parse_tokens(parser: &mut impl TokenParser, errs: &mut Vec<Error>) -> Self {
        match parser.clone().parse::<Keyword>(errs).str() {
            "mod" => Self::SubMod(parser.parse(errs)),
            "use" => Self::UseStmt(parser.parse(errs)),
            _ => {
                errs.push(Error::from_messages(parser.end_span(), [
                    errm::unexpected_token()
                ]));

                parser.parse(errs)
            }
        }
    }
}
impl DisplayWithSrc for ModItem {
    fn fmt_with_src(&self, f: &mut Formatter, srcfile: &SrcFile) -> fmt::Result {
        match self {
            Self::SubMod(s) => s.fmt_with_src(f, srcfile),
            Self::UseStmt(s) => s.fmt_with_src(f, srcfile),
        }
    }
}
impl Syntax for ModItem {
    
}

impl ParseTokens for Option<ModItem> {
    fn parse_tokens(parser: &mut impl TokenParser, errs: &mut Vec<Error>) -> Self {
        if parser.clone().next(errs).is_some() {
            Some(
                parser.parse(errs)
            )
        }
        else {
            None
        }
    }
}