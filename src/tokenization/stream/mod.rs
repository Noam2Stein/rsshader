use super::*;

#[inline(always)]
pub fn tokenize_collected<'src>(srcfile: &'src SrcFile, errs: &mut Vec<Error>) -> TokenStream<'src> {
    TokenStream::new(srcfile, errs)
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TokenStream<'src> {
    srcfile: &'src SrcFile,
    tts: Vec<TokenTree>,
}
impl<'src> TokenStream<'src> {
    #[inline(always)]
    pub fn new(srcfile: &'src SrcFile, errs: &mut Vec<Error>) -> Self {
        Self {
            srcfile,
            tts: tokenize(srcfile).collect(errs),
        }
    }

    #[inline(always)]
    pub const fn srcfile(&self) -> &'src SrcFile {
        self.srcfile
    }
    #[inline(always)]
    pub const fn tts(&self) -> &Vec<TokenTree> {
        &self.tts
    }
    #[inline(always)]
    pub fn into_tts(self) -> Vec<TokenTree> {
        self.tts
    }
}
impl<'src> Display for TokenStream<'src> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut tts = self.tts.iter();
        if let Some(tt) = tts.next() {
            tt.fmt_with_src(f, self.srcfile)?;
        }

        for tt in tts {
            write!(f, " ")?;
            tt.fmt_with_src(f, self.srcfile)?;
        }

        Ok(())
    }
}
impl<'src> Describe for TokenStream<'src> {
    fn desc(&self) -> Description {
        Description::quote(&self.to_string())
    }
}
impl<'src> TypeDescribe for TokenStream<'src> {
    fn type_desc() -> Description {
        Description::new("a token stream")
    }
}