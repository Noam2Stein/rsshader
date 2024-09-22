use super::*;

impl<T: UnwrapTokenTree + TokenDefault + TypeDescribe> ParseTokens for T {
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
impl<T: UnwrapTokenTreeExpect> ParseTokensExpect for T {
    type Output = T::Output;
    fn parse_tokens_expect(self, parser: &mut impl TokenParser, errs: &mut Vec<Error>) -> Self::Output {
        if let Some(tt) = parser.next(errs) {
            self.unwrap_tt_expect(tt, errs)
        }
        else {
            errs.push(Error::from_messages(parser.end_span(), [
                errm::unexpected_end_of_file(),
                errm::expected(self.desc())
            ]));

            unsafe {
                T::Output::tt_default(parser.end_span())
            }
        }
    }
}