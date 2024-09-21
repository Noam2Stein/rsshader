use super::{source::*, diagnostic::*, tokenization::*, *};

mod parsers;
mod tokens;

pub trait TokenParser: Sized + Clone {
    fn next(&mut self, errs: &mut Vec<Error>) -> Option<TokenTree>;
    fn end_span(&self) -> Span;

    fn collect(mut self, errs: &mut Vec<Error>) -> Vec<TokenTree> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next(errs) {
            tokens.push(token);
        }

        tokens
    }

    #[inline(always)]
    fn parse<P: ParseTokens>(&mut self, errs: &mut Vec<Error>) -> P {
        P::parse_tokens(self, errs)
    }
    fn parse_expect<E: Copy, P: ParseTokensExpect<E>>(&mut self, expect: E, errs: &mut Vec<Error>) -> P {
        P::parse_tokens_expect(self, expect, errs)
    }
}
pub trait ParseTokens {
    fn parse_tokens(parser: &mut impl TokenParser, errs: &mut Vec<Error>) -> Self;
}
pub trait ParseTokensExpect<E: Copy> {
    fn parse_tokens_expect(parser: &mut impl TokenParser, expect: E, errs: &mut Vec<Error>) -> Self;
    fn expect_desc(expect: E) -> Description;
}

pub trait AsTokenParser {
    fn parser(&self) -> impl TokenParser;
}
pub trait IntoTokenParser {
    fn into_parser(self) -> impl TokenParser;
}