use super::*;

pub trait TokenIterator<'src>: Sized {
    fn next(&mut self, errs: &mut Vec<Error>) -> Option<TokenTree<'src>>;
    fn srcfile(&self) -> &'src SrcFile<'src>;

    fn collect(mut self, errs: &mut Vec<Error>) -> TokenStream<'src> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next(errs) {
            tokens.push(token);
        }

        tokens.into()
    }

    #[inline(always)]
    fn parse<P: ParseTokens<'src>>(&mut self, errs: &mut Vec<Error>) -> P {
        P::parse_tokens(self, errs)
    }
}
pub trait ParseTokens<'src> {
    fn parse_tokens(tokens: &mut impl TokenIterator<'src>, errs: &mut Vec<Error>) -> Self;
}