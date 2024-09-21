use super::*;

#[inline(always)]
pub fn tokenize<'src>(srcfile: &'src SrcFile) -> Tokenizer<'src> {
    Tokenizer::new(srcfile)
}

#[derive(Debug, Clone)]
pub struct Tokenizer<'src> {
    raw: RawTokenizer<'src>,
}
impl<'src> Tokenizer<'src> {
    #[inline(always)]
    pub fn new(srcfile: &'src SrcFile) -> Self {
        Self {
            raw: RawTokenizer::new(srcfile),
        }
    }

    pub fn srcfile(&self) -> &'src SrcFile {
        &self.raw.srcfile()
    }
    pub fn next(&mut self, errs: &mut Vec<Error>) -> Option<TokenTree> {
        match read_raw_token(self.raw.srcfile(), errs, &mut self.raw) {
            ReadRawTokenOutput::GroupClose(span) => {
                let close_delimiter = Delimiter::from_close_str(self.srcfile()[span].s()).unwrap();
                
                errs.push(Error::from_messages(span, [
                    ErrorMessage::Problem(format!("closing delimiter without a group to close")),
                    errm::unmatched_delimiter(close_delimiter.open_desc()),
                    errm::expected(close_delimiter.close_desc()),
                ]));

                self.next(errs)
            },
            ReadRawTokenOutput::TokenTree(tt) => {
                Some(tt)
            },
            ReadRawTokenOutput::None => {
                None
            }
        }
    }
    pub fn collect(mut self, errs: &mut Vec<Error>) -> Vec<TokenTree> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next(errs) {
            tokens.push(token);
        }

        tokens
    }
}

enum ReadRawTokenOutput {
    GroupClose(Span),
    TokenTree(TokenTree),
    None,
}

fn read_raw_token<'src>(srcfile: &'src SrcFile, errs: &mut Vec<Error>, raw_tokens: &mut RawTokenizer<'src>) -> ReadRawTokenOutput {
    if let Some(RawToken { span, ty }) = raw_tokens.next() {
        let srcslice = &srcfile[span];
        match ty {
            RawTokenType::Ident => ReadRawTokenOutput::TokenTree(
                Keyword::new(srcslice, span).map_or_else(
                    || TokenTree::Ident(Ident::new(span)),
                    |keyword| TokenTree::Keyword(keyword)
                )
            ),
            RawTokenType::IntLiteral => ReadRawTokenOutput::TokenTree(
                TokenTree::Literal(Literal::Int(IntLiteral::new(srcslice, span, errs)))
            ),
            RawTokenType::FloatLiteral => ReadRawTokenOutput::TokenTree(
                TokenTree::Literal(Literal::Float(FloatLiteral::new(srcslice, span, errs)))
            ),
            RawTokenType::Punct => ReadRawTokenOutput::TokenTree(
                TokenTree::Punct(Punct::new(srcslice, span))
            ),
            RawTokenType::GroupOpen => {
                let delimiter = Delimiter::from_open_str(srcslice.s()).unwrap();
            
                let mut group_tts = Vec::<TokenTree>::new();
                loop {
                    match read_raw_token(srcfile, errs, raw_tokens) {
                        ReadRawTokenOutput::GroupClose(close_span) => {
                            let close_srcslice = &srcfile[close_span];
                            let close_delimiter = Delimiter::from_close_str(close_srcslice.s()).unwrap();
                            
                            let group_srcslice = if close_delimiter == delimiter {
                                span.connect(&close_span)
                            }
                            else {
                                errs.push(Error::from_messages(close_span, [
                                    errm::unmatched_delimiter(delimiter.open_desc()),
                                    errm::expected_found(delimiter.close_desc(), close_delimiter.close_desc())
                                ]));

                                group_tts.last().map_or(span, |last_tt| span.connect(&last_tt.span()))
                            };

                            break ReadRawTokenOutput::TokenTree(TokenTree::Group(
                                Group::new(delimiter, group_tts, group_srcslice)
                            ));
                        }
                        ReadRawTokenOutput::TokenTree(tt) => {
                            group_tts.push(tt);
                        }
                        ReadRawTokenOutput::None => {
                            errs.push(Error::from_messages(srcfile.end_span(), [
                                errm::unmatched_delimiter(delimiter.open_desc()),
                                errm::unexpected_end_of_file(),
                                errm::expected(delimiter.close_desc())
                            ]));

                            break ReadRawTokenOutput::TokenTree(TokenTree::Group(
                                Group::new(delimiter, group_tts, span.connect(&srcfile.end_span()))
                            ));
                        }
                    }
                }
            }
            RawTokenType::GroupClose => {
                ReadRawTokenOutput::GroupClose(span)
            }
            RawTokenType::Invalid => {
                errs.push(Error::from_messages(span, [
                    errm::is_not(Description::quote(srcslice.s()), RawToken::type_desc())
                ]));
    
                read_raw_token(srcfile, errs, raw_tokens)
            }
        }
    }
    else {
        ReadRawTokenOutput::None
    }
}