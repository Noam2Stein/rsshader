use super::*;

pub const fn group_with(delimiter: Delimiter) -> GroupWith {
    GroupWith {
        delimiter,
    }
}

pub struct GroupWith {
    pub delimiter: Delimiter,
}
impl Describe for GroupWith {
    fn desc(&self) -> Description {
        Group::type_desc().with(&self.delimiter.desc())
    }
}
impl UnwrapTokenTreeExpect for GroupWith {
    type Output = Group;
    fn unwrap_tt_expect(self, tt: TokenTree, errs: &mut Vec<Error>) -> Self::Output {
        if let TokenTree::Group(mut tt) = tt {
            if tt.delimiter != self.delimiter {
                errs.push(Error::from_messages(tt.span(), [
                    errm::expected_found(self.desc(), group_with(tt.delimiter).desc())
                ]));
            }

            tt.delimiter = self.delimiter;
            tt
        }
        else {
            errs.push(Error::from_messages(tt.span(), [
                errm::expected_found(self.desc(), tt.token_type_desc())
            ]));

            unsafe {
                Group::tt_default(tt.span())
            }
        }   
    }
}