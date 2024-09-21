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
        if let TokenTree::Keyword(keyword) = parser.clone().parse(errs) {
            match parser.clone().parse::<Keyword>(errs).s() {
                "mod" => Self::SubMod(parser.parse(errs)),
                _ => {
                    errs.push(Error::from_messages(parser.end_span(), [
                        errm::unexpected_token()
                    ]));
    
    
                }
            }
        }
        else {
            parser.parse(errs)
        }
    }
}
impl DisplayWithSrc for ModItem {
    fn fmt_with_src(&self, f: &mut Formatter, srcfile: &SrcFile) -> fmt::Result {
        match self {
            Self::SubMod(s) => s.fmt_with_src(f, srcfile)
        }
    }
}
impl Syntax for ModItem {
    
}

impl ParseTokens for Option<ModItem> {
    fn parse_tokens(parser: &mut impl TokenParser, errs: &mut Vec<Error>) -> Self {
        parser.next(errs).map_or(None, |first_tt| {
            if let TokenTree::Keyword(keyword) = first_tt {
                match keyword.s() {
                    "mod" => {
                        let ident = parser.parse(errs);
                        parser.parse_expect::<&str, Punct>(";", errs);
            
                        Some(
                            ModItem::SubMod(SubMod {
                                ident,
                            })
                        )
                    }
                    _ => {
                        errs.push(Error::from_messages(first_tt.span(), [
                            errm::unexpected_token()
                        ]));

                        parser.parse(errs)
                    }
                }
            }
            else {
                errs.push(Error::from_messages(first_tt.span(), [
                    errm::expected_found(Keyword::type_desc(), first_tt.token_type_desc())
                ]));

                parser.parse(errs)
            }
        })
    }
}