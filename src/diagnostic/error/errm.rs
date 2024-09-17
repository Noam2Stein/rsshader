use crate::diagnostic::desc::Description;

use super::ErrorMessage;

pub fn expected(expected: Description) -> ErrorMessage {
    ErrorMessage::Problem(format!("expected {expected}"))
}
pub fn found(found: Description) -> ErrorMessage {
    ErrorMessage::Problem(format!("found {found}"))
}
pub fn expected_found(expected: Description, found: Description) -> ErrorMessage {
    ErrorMessage::Problem(format!("expected {expected}, found {found}"))
}
pub fn unexpected_end_of_file() -> ErrorMessage {
    ErrorMessage::Problem(format!("unexpected end of file"))
}
pub fn is_not(a: Description, b: Description) -> ErrorMessage {
    ErrorMessage::Problem(format!("{a} is not {b}"))
}
pub fn unmatched_delimiter(delimiter: Description) -> ErrorMessage {
    ErrorMessage::Problem(format!("unmatched delimiter {delimiter}"))
}
pub fn valid_forms_are(valid_forms: impl IntoIterator<Item = Description>) -> ErrorMessage {
    ErrorMessage::Help(format!("valid forms are {}", Description::list(valid_forms)))
}