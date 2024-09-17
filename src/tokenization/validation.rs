use fmt::Debug;

use super::*;

pub trait UnspannedTokenTypeValidation<'src>:
Debug +
Clone +
Eq +
Ord +
Hash +
Display +
ParseTokens<'src> +
FromSrc<'src> +
FromSrcUnchecked<'src> +
Spannable +
{

}

pub trait SpannedTokenTypeValidation<'src>:
Debug +
Clone +
Eq +
Ord +
Hash +
Display +
ParseTokens<'src> +
FromSrc<'src> +
FromSrcUnchecked<'src> +
SpannedSpannable +
{

}