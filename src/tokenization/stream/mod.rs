use super::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TokenStream<'src> {
    pub tokens: Vec<TokenTree<'src>>,
}
impl<'src> TokenStream<'src> {
    pub const fn new(tokens: Vec<TokenTree<'src>>) -> Self {
        Self {
            tokens,
        }
    }

    pub fn span(&self, srcfile: &'src SrcFile) -> Option<Span> {
        self.tokens.first().map(|first|
            first.span(srcfile).connect(&self.tokens.last().unwrap().span(srcfile))
        )
    }

    pub fn into_iter(self, srcfile: &'src SrcFile<'src>) -> TokenStreamIter {
        TokenStreamIter {
            srcfile,
            tokens: self.tokens.into_iter(),
        }
    }
}
impl<'src> Display for TokenStream<'src> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.tokens.iter().map(|tt| tt.to_string()).collect::<Box<[String]>>().join(" "))
    }
}
impl<'src> Describe for TokenStream<'src> {
    fn desc(&self) -> Description {
        Description::new(
            format!("'{}'", self.tokens.iter().map(|tt| tt.to_string()).collect::<Box<[String]>>().join(" "))
        )
    }
}
impl<'src> TypeDescribe for TokenStream<'src> {
    fn type_desc() -> Description {
        Description::new("a token stream")
    }
}
impl<'src> From<Vec<TokenTree<'src>>> for TokenStream<'src> {
    fn from(value: Vec<TokenTree<'src>>) -> Self {
        Self::new(value)
    }
}

pub struct TokenStreamIter<'src> {
    srcfile: &'src SrcFile<'src>,
    tokens: <Vec<TokenTree<'src>> as IntoIterator>::IntoIter,
}
impl<'src> TokenIterator<'src> for TokenStreamIter<'src> {
    fn next(&mut self, _errs: &mut Vec<Error>) -> Option<TokenTree<'src>> {
        self.tokens.next()
    }
    fn srcfile(&self) -> &'src SrcFile<'src> {
        self.srcfile
    }
}