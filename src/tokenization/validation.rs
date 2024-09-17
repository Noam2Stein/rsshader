use std::{fmt::{Debug, Display}, hash::Hash};

use super::*;

pub trait TokenTypeValidation<'src>:
Debug +
Clone +
Eq +
Ord +
Hash +
Display +
ParseTokens<'src> +
{

}

pub trait SpannedTokenTypeValidation<'src>:
TokenTypeValidation<'src> +
Spanned +
{

}