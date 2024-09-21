use super::*;

pub trait TokenParser<'src>: Sized {
    fn next(&mut self, errs: &mut Vec<Error<'src>>) -> Option<TokenTree<'src>>;
    fn end_srcslice(&self) -> &'src SrcSlice;

    fn collect(mut self, errs: &mut Vec<Error<'src>>) -> Vec<TokenTree<'src>> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next(errs) {
            tokens.push(token);
        }

        tokens
    }

    #[inline(always)]
    fn parse<P: ParseTokens<'src>>(&mut self, errs: &mut Vec<Error<'src>>) -> P {
        P::parse_tokens(self, errs)
    }
}
pub trait ParseTokens<'src> {
    fn parse_tokens(parser: &mut impl TokenParser<'src>, errs: &mut Vec<Error<'src>>) -> Self;
}