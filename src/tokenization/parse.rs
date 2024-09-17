use super::*;

pub trait ParseTokens<'src> {
    fn parse_tokens(tokens: impl Iterator<Item = TokenTree<'src>>, src: &'src SrcFile, errs: &mut Vec<Error>) -> Self;
}

pub trait ParseTokenIterExt<'src> {
    fn parse<P: ParseTokens<'src>>(&mut self, src: &'src SrcFile, errs: &mut Vec<Error>) -> P;
}
impl<'src, T: Iterator<Item = TokenTree<'src>>> ParseTokenIterExt<'src> for T {
    fn parse<P: ParseTokens<'src>>(&mut self, src: &'src SrcFile, errs: &mut Vec<Error>) -> P {
        P::parse_tokens(self, src, errs)
    }
}