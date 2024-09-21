use super::*;

impl<T: SubToken> ParseTokens for T {
    fn parse_tokens(parser: &mut impl TokenParser, errs: &mut Vec<Error>) -> Self {
        if let Some(tt) = parser.next(errs) {
            T::unwrap_tt(tt, errs)
        }
        else {
            errs.push(Error::from_messages(parser.end_span(), [
                errm::unexpected_end_of_file(),
                errm::expected(Self::type_desc())
            ]));

            unsafe {
                T::tt_default(parser.end_span())
            }
        }
    }
}
impl<E: Copy, T: SubToken + UnwrapTokenTreeExpect<E>> ParseTokensExpect<E> for T {
    fn parse_tokens_expect(parser: &mut impl TokenParser, expect: E, errs: &mut Vec<Error>) -> Self {
        if let Some(tt) = parser.next(errs) {
            T::unwrap_tt_expect(tt, expect, errs)
        }
        else {
            errs.push(Error::from_messages(parser.end_span(), [
                errm::unexpected_end_of_file(),
                errm::expected(T::expect_desc(expect))
            ]));

            unsafe {
                T::tt_default(parser.end_span())
            }
        }
    }
    fn expect_desc(expect: E) -> Description {
        T::expect_desc(expect)
    }
}