use super::*;

pub fn tokenize<'src>(srcfile: &'src SrcFile) -> Tokenizer<'src> {
    Tokenizer::new(srcfile, srcfile.span())
}

#[repr(transparent)]
pub struct Tokenizer<'src> {
    raw: RawTokenizer<'src>,
}
impl<'src> Tokenizer<'src> {
    #[inline(always)]
    pub fn new(srcfile: &'src SrcFile, span: Span) -> Self {
        Self {
            raw: RawTokenizer::new(srcfile, span),
        }
    }
}
impl<'src> TokenParser<'src> for Tokenizer<'src> {
    fn next(&mut self, errs: &mut Vec<Error>) -> Option<TokenTree<'src>> {
        match read_raw_token(self.srcfile(), errs, &mut self.raw) {
            ReadRawTokenOutput::GroupClose(raw_token) => {
                let close_delimiter = Delimiter::from_close_str(raw_token.s()).unwrap();
                
                errs.push(Error::from_messages(raw_token.span(self.srcfile()), [
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
    fn srcfile(&self) -> &'src SrcFile {
        &self.raw.srcfile()
    }
}

enum ReadRawTokenOutput<'src> {
    GroupClose(&'src SrcSlice),
    TokenTree(TokenTree<'src>),
    None,
}

fn read_raw_token<'src>(srcfile: &'src SrcFile, errs: &mut Vec<Error>, raw_tokens: &mut RawTokenizer<'src>) -> ReadRawTokenOutput<'src> {
    if let Some(raw_token) = raw_tokens.next() {
        match raw_token.ty {
            RawTokenType::Ident =>
            if let Ok(keyword) = Keyword::from_src(raw_token.srcslice) {
                ReadRawTokenOutput::TokenTree(
                    TokenTree::Keyword(keyword)
                )
            }
            else {
                ReadRawTokenOutput::TokenTree(
                    TokenTree::Ident(unsafe { Ident::from_src_unchecked(raw_token.srcslice) })
                )
            },
            RawTokenType::IntLiteral => ReadRawTokenOutput::TokenTree(
                TokenTree::Literal(Literal::Int(
                    IntLiteral::from_raw_token(raw_token, errs)
                ))
            ),
            RawTokenType::FloatLiteral => ReadRawTokenOutput::TokenTree(
                TokenTree::Literal(Literal::Float(
                    FloatLiteral::from_raw_token(raw_token, errs)
                ))
            ),
            RawTokenType::Punct => ReadRawTokenOutput::TokenTree(
                TokenTree::Punct(Punct::from_raw_token(raw_token, errs))
            ),
            RawTokenType::GroupOpen => {
                let delimiter = Delimiter::from_open_str(raw_token.srcslice.s()).unwrap();
            
                let mut group_tts = Vec::new();
                loop {
                    match read_raw_token(srcfile, errs, raw_tokens) {
                        ReadRawTokenOutput::GroupClose(raw_token) => {
                            let srcslice = &srcfile[raw_token.span];
                            let close_delimiter = Delimiter::from_close_str(srcslice.s()).unwrap();

                            let stream = TokenStream::from(group_tts);
                            let srcslice = &srcfile[
                                match stream.span(srcfile) {
                                    Some(stream_span) => raw_token.span.connect(&stream_span),
                                    None => raw_token.span,
                                }
                            ];

                            if close_delimiter != delimiter {
                                errs.push(Error::from_messages(raw_token.span, [
                                    errm::unmatched_delimiter(delimiter.open_desc()),
                                    errm::expected_found(delimiter.close_desc(), close_delimiter.close_desc())
                                ]));
                            }

                            break ReadRawTokenOutput::TokenTree(TokenTree::Group(
                                Group {
                                    delimiter,
                                    srcslice,
                                    stream,
                                }
                            ));
                        }
                        ReadRawTokenOutput::TokenTree(tt) => {
                            group_tts.push(tt);
                        }
                        ReadRawTokenOutput::None => {

                        }
                    }
                }
            }
            RawTokenType::GroupClose => {
                ReadRawTokenOutput::GroupClose(raw_token)
            }
            RawTokenType::Invalid => {
                errs.push(Error::from_messages(raw_token.span, [
                    errm::is_not(Description::quote(srcfile[raw_token.span].s()), RawToken::type_desc())
                ]));
    
                read_raw_token(srcfile, errs, raw_tokens)
            }
        }
    }
    else {
        ReadRawTokenOutput::None
    }
}