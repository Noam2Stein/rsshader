pub fn parse_tokens_with(delimiter: Delimiter, parser: &mut impl TokenParser<'src>, errs: &mut Vec<Error<'src>>) -> Self {
    if let Some(token) = parser.next(errs) {
        if let TokenTree::Group(token) = token {
            if token.delimiter != delimiter {
                errs.push(Error::from_messages(token.srcslice, [
                    errm::expected_found(Self::type_desc().with(&delimiter.desc()), Self::type_desc().with(&token.delimiter.desc()))
                ]));
            }
            
            token
        }
        else {
            errs.push(Error::from_messages(token.srcslice(), [
                errm::expected_found(Self::type_desc(), token.token_type_desc())
            ]));

            Self::unwrap_tt(token.srcslice())
        }
    }
    else {
        errs.push(Error::from_messages(parser.end_srcslice(), [
            errm::unexpected_end_of_file(),
            errm::expected(Self::type_desc())
        ]));

        Self::unwrap_tt(&parser.end_srcslice()[0..0])
    }
}

macro_rules! group_with {
    ($ident:ident: $delimiter:expr) => {
        #[repr(transparent)]
        pub struct $ident<'src> {
            group: Group<'src>,
        }
        impl<'src> $ident<'src> {
            pub fn group(self) -> Group<'src> {
                self.group
            }
        }
        impl<'src> From<$ident<'src>> for Group<'src> {
            fn from(value: $ident<'src>) -> Self {
                value.group
            }
        }
        impl<'src> ParseTokens<'src> for $ident<'src> {
            fn parse_tokens(parser: &mut impl TokenParser<'src>, errs: &mut Vec<Error<'src>>) -> Self {
                unsafe {
                    mem::transmute(Group::parse_tokens_with($delimiter, parser, errs))
                }
            }
        }
    };
}

group_with!(GroupWithBraces: Delimiter::Brace);
group_with!(GroupWithBrackets: Delimiter::Bracket);
group_with!(GroupWithParenthesis: Delimiter::Parenthesis);