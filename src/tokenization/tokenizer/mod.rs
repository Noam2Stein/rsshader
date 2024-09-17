use super::*;

pub fn tokenize<'src>(src: &'src SrcFile, errs: &mut Vec<Error>) -> impl Iterator<Item = TokenTree<'src>> {
    let mut tokens = RawTokenIter::new(src);

    let mut output = Vec::new();
    
    while let Some(token) = tokens.next() {
        match token.ty {
            RawTokenType::GroupOpen => {
                let ReadGroupOutput { group, escaped_group_close } = read_group(token, &mut tokens, errs);
                
                output.push(TokenTree::Group(group));

                if let Some(escaped_group_close) = escaped_group_close {
                    read_close(escaped_group_close, errs)
                }
            }
            RawTokenType::GroupClose => {
                let close_delimiter = Delimiter::from_close_str(token.str).unwrap();
                
                errs.push(Error::from_messages(token.span, [
                    ErrorMessage::Problem(format!("closing delimiter without a group to close")),
                    errm::unmatched_delimiter(close_delimiter.open_desc()),
                    errm::expected(close_delimiter.close_desc()),
                ]));
            }
            _ => {
                read_raw_token(token, &mut output, errs)
            }
        }
    };
    
    output.into()
}

pub trait TokenIter<'src>: Sized {
    fn next(&mut self, errs: &mut Vec<Error>) -> Option<TokenTree<'src>>;

    fn collect(mut self, errs: &mut Vec<Error>) -> TokenStream<'src> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next(errs) {
            tokens.push(token);
        }

        tokens.into()
    }
}

pub struct Tokenizer<'src> {
    raw_tokens: RawTokenIter<'src>,
}
impl<'src> Tokenizer<'src> {
    pub fn new(src: &'src SrcFile) -> Self {
        Self {
            raw_tokens: RawTokenIter::new(&src.s)
        }
    }
}
impl<'src> TokenIter<'src> for Tokenizer<'src> {
    fn next(&mut self, errs: &mut Vec<Error>) -> Option<TokenTree<'src>> {
        if let Some(raw_token) = self.raw_tokens.next() {
            match raw_token.ty {
                RawTokenType::GroupOpen => {
                    let output = read_group(raw_token, &mut self.raw_tokens, errs);
                    
                    if let Some(escaped_group_close) = output.escaped_group_close {
                        read_close(escaped_group_close, errs)
                    }

                    Some(
                        TokenTree::Group(output.group)
                    )
                }
                RawTokenType::GroupClose => {
                    let close_delimiter = Delimiter::from_close_str(&self.raw_tokens.src()[raw_token.span]).unwrap();
                    
                    errs.push(Error::from_messages(raw_token.span, [
                        ErrorMessage::Problem(format!("closing delimiter without a group to close")),
                        errm::unmatched_delimiter(close_delimiter.open_desc()),
                        errm::expected(close_delimiter.close_desc()),
                    ]));

                    None
                }
                _ => {
                    read_raw_token(raw_token, &mut output, errs)
                }
            }
        }
    }    
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct GroupEdge {
    delimiter: Delimiter,
    span: Span,
}
struct ReadGroupOutput<'src> {
    group: Group<'src>,
    escaped_group_close: Option<GroupEdge>,
}

#[inline(always)]
fn read_close(close: GroupEdge, errs: &mut Vec<Error>) {
    errs.push(Error::from_messages(close.span, [
        ErrorMessage::Problem(format!("closing delimiter without a group to close")),
        errm::unmatched_delimiter(close.delimiter.open_desc()),
        errm::expected(close.delimiter.close_desc()),
    ]));
}

fn read_group<'src>(open: RawToken, tokens: &mut RawTokenIter, errs: &mut Vec<Error>) -> ReadGroupOutput<'src> {
    let open = GroupEdge {
        delimiter: Delimiter::from_open_str(open.str).unwrap(),
        span: open.span,
    };

    let mut output = Vec::new();

    while let Some(token) = tokens.next() {        
        match token.ty {
            RawTokenType::GroupOpen => {
                let ReadGroupOutput { group, escaped_group_close } = read_group(token, tokens, errs);
                
                output.push(TokenTree::Group(group));

                if let Some(escaped_group_close) = escaped_group_close {
                    return read_group_close(open, escaped_group_close, output.into(), errs)
                }
            }
            RawTokenType::GroupClose => {
                let close = GroupEdge {
                    delimiter: Delimiter::from_close_str(token.str).unwrap(),
                    span: token.span,
                };

                return read_group_close(open, close, output.into(), errs)
            }
            _ => {
                read_raw_token(token, &mut output, errs)
            }
        }
    };
    
    '_group_unclosed: {
        let stream = TokenStream::from(output);
        let span = open.span.connect(stream.span);
    
        errs.push(
            Error::from_messages(span, [
                errm::unmatched_delimiter(open.delimiter.open_desc()),
                errm::unexpected_end_of_file(),
                errm::expected(open.delimiter.close_desc())
            ])
        );

        ReadGroupOutput {
            group: Group {
                delimiter: open.delimiter,
                stream,
                span,
            },
            escaped_group_close: None,
        }
    }
}

#[inline(always)]
fn read_group_close<'src>(open: GroupEdge, close: GroupEdge, stream: TokenStream, errs: &mut Vec<Error>) -> ReadGroupOutput<'src> {
    if close.delimiter == open.delimiter {
        ReadGroupOutput {
            group: Group {
                delimiter: open.delimiter,
                stream: stream,
                span: open.span.connect(close.span),
            },
            escaped_group_close: None,
        }
    }
    else {
        errs.push(Error::from_messages(open.span, [
            errm::unmatched_delimiter(open.delimiter.open_desc()),
            errm::expected_found(open.delimiter.close_desc(), close.delimiter.close_desc()),
        ]));

        ReadGroupOutput {
            group: Group {
                delimiter: open.delimiter,
                span: open.span.connect(stream.span),
                stream: stream,
            },
            escaped_group_close: Some(close),
        }
    }
}

#[inline(always)]
fn read_raw_token<'src>(src: &'src SrcFile, raw_token: RawToken, errs: &mut Vec<Error>) -> TokenTree<'src> {
    match raw_token.ty {
        RawTokenType::Ident =>
        if let Some(keyword) = SpannedKeyword::from_src(src, raw_token.span) {
            TokenTree::Keyword(keyword)
        }
        else {
            TokenTree::Ident(
                unsafe { SpannedIdent::parse_unchecked(raw_token.str, raw_token.span.start()) }
            )
        },
        RawTokenType::IntLiteral => output.push(
            TokenTree::Literal(Literal::Int(
                IntLiteral::parse_unsuffixed(raw_token.str, raw_token.span.start()).unwrap_or_else(|err| {
                    errs.push(Error::from_messages(raw_token.span, [
                        ErrorMessage::Problem(err)
                    ]));
                    IntLiteral::default()
                })
            ))
        ),
        RawTokenType::FloatLiteral => output.push(
            TokenTree::Literal(Literal::Float(
                FloatLiteral::parse_unsuffixed(raw_token.str, raw_token.span.start()).unwrap_or_else(|err| {
                    errs.push(Error::from_messages(raw_token.span, [
                        ErrorMessage::Problem(err)
                    ]));
                    FloatLiteral::default()
                })
            ))
        ),
        RawTokenType::Punct => output.push(
            TokenTree::Punct(Punct::parse(raw_token.str, raw_token.span.start()).unwrap())
        ),
        RawTokenType::GroupOpen => {
            unreachable!()
        }
        RawTokenType::GroupClose => {
            unreachable!()
        }
        RawTokenType::Invalid => errs.push(Error::from_messages(raw_token.span, [
            errm::is_not(Description::quote(raw_token.str), RawToken::type_desc())
        ]))
    }
}