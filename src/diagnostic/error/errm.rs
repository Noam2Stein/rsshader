use super::*;

pub fn unexpected_token() -> ErrorMessage {
    ErrorMessage::Problem(format!("unexpected token"))
}
pub fn expected(expected: Description) -> ErrorMessage {
    ErrorMessage::Problem(format!("expected {expected}"))
}
pub fn found(found: Description) -> ErrorMessage {
    ErrorMessage::Problem(format!("found {found}"))
}
pub fn expected_found(expected: Description, found: Description) -> ErrorMessage {
    ErrorMessage::Problem(format!("expected {expected}, found {found}"))
}
pub fn expected_is_not(expected: Description, found: Description) -> ErrorMessage {
    ErrorMessage::Problem(format!("expected {expected}, {found} is not {expected}"))
}
pub fn unexpected_end_of_file() -> ErrorMessage {
    ErrorMessage::Problem(format!("unexpected end of file"))
}
pub fn unexpected_end_of_tokens() -> ErrorMessage {
    ErrorMessage::Problem(format!("unexpected end of tokens"))
}
pub fn is_not(x: Description, y: Description) -> ErrorMessage {
    ErrorMessage::Problem(format!("{x} is not {y}"))
}
pub fn is_not_because(x: Description, y: Description, z: &str) -> ErrorMessage {
    ErrorMessage::Problem(format!("{x} is not {y} because {z}"))
}
pub fn unmatched_delimiter(delimiter: Description) -> ErrorMessage {
    ErrorMessage::Problem(format!("unmatched delimiter {delimiter}"))
}
pub fn valid_forms_are(valid_forms: impl IntoIterator<Item = Description>) -> ErrorMessage {
    ErrorMessage::Help(format!("valid forms are {}", Description::list(valid_forms)))
}
pub fn is_and_thus_cant_be_used_as(x: Description, y: Description, z: Description) -> ErrorMessage {
    ErrorMessage::Problem(format!("{x} is {y} and thus can't be used as {z}"))
}
pub fn is_too_large_for_the_literal_capacity(x: Description) -> ErrorMessage {
    ErrorMessage::Problem(format!("{x} is too large for the literal capacity ({})", u128::MAX))
}