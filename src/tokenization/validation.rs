use fmt::Debug;

use super::*;

pub trait ValidatedTokenType<'src>:
Debug +
Clone +
Eq +
Ord +
Hash +
Display +
FromSrc<'src> +
ParseTokens<'src> +
Spannable +
{

}

pub trait ValidatedSpannedTokenType<'src>:
Debug +
Clone +
Eq +
Ord +
Hash +
Display +
ParseTokens<'src> +
SpannedSpannable<Unspanned: ValidatedTokenType<'src, Spanned = Self>> +
{

}