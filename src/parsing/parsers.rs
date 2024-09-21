use super::*;

trait TokenIter: fmt::Debug + Clone {
    fn next(&mut self, errs: &mut Vec<Error>) -> Option<TokenTree>;
}
impl<T: Iterator<Item = TokenTree> + fmt::Debug + Clone> TokenIter for T {
    #[inline(always)]
    fn next(&mut self, _errs: &mut Vec<Error>) -> Option<TokenTree> {
        self.next()
    }
}
impl<'src> TokenIter for Tokenizer<'src> {
    #[inline(always)]
    fn next(&mut self, errs: &mut Vec<Error>) -> Option<TokenTree> {
        self.next(errs)
    }
}

#[derive(Debug, Clone)]
struct TokenIterParser<T: TokenIter> {
    iter: T,
    end_span: Span,
}
impl<T: TokenIter> TokenIterParser<T> {
    const fn new(iter: T, end_span: Span) -> Self {
        Self {
            iter,
            end_span,
        }
    }
}
impl<T: TokenIter> TokenParser for TokenIterParser<T> {
    fn next(&mut self, errs: &mut Vec<Error>) -> Option<TokenTree> {
        self.iter.next(errs)
    }
    fn end_span(&self) -> Span {
        self.end_span
    }
}

impl<'src> IntoTokenParser for Tokenizer<'src> {
    fn into_parser(self) -> impl TokenParser {
        let end_span = self.srcfile().end_span();
        TokenIterParser::new(self, end_span)
    }
}

impl<'src> AsTokenParser for TokenStream<'src> {
    fn parser(&self) -> impl TokenParser {
        TokenIterParser::new(self.tts().iter().cloned(), self.srcfile().end_span())
    }
}
impl<'src> IntoTokenParser for TokenStream<'src> {
    fn into_parser(self) -> impl TokenParser {
        let end_span = self.srcfile().end_span();
        let iter = self.into_tts().into_iter();
        TokenIterParser::new(iter, end_span)
    }
}

impl AsTokenParser for Group {
    fn parser(&self) -> impl TokenParser {
        TokenIterParser::new(self.tts().iter().cloned(), self.close_span())
    }
}
impl IntoTokenParser for Group {
    fn into_parser(self) -> impl TokenParser {
        let end_span = self.close_span();
        let iter = self.into_tts().into_iter();
        TokenIterParser::new(iter, end_span)
    }
}