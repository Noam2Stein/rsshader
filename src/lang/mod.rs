mod formatter;
pub use formatter::*;

mod wgsl;
pub use wgsl::*;

#[macro_export]
macro_rules! shader {
    ($ir:expr => $fmt_fn:path) => {
        $crate::rsshader_macros::shader!($ir => $fmt_fn)
    };
}
