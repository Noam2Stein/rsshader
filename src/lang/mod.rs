mod formatter;
pub use formatter::*;

mod wgsl;
pub use wgsl::*;

#[macro_export]
macro_rules! shader {
    ($($entry_points:path),* $(,)? => $fmt_fn:path) => {
        $crate::rsshader_macros::shader!($($entry_points),* => $fmt_fn)
    };
}
