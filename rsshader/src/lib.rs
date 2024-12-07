mod blend;
mod gpu_fn;
mod gpu_item_info;
mod gpu_type;
mod render_pipeline;
use core::str;
use std::mem::MaybeUninit;

pub use blend::*;
pub use gpu_fn::*;
pub use gpu_item_info::*;
pub use gpu_type::*;
pub use render_pipeline::*;

pub use rsshader_proc_macros::gpu;

pub const fn __concat_strs_len__(strs: &[&str]) -> usize {
    let mut output = 0;

    let mut i = 0;
    while i < strs.len() {
        output += strs[i].len();

        i += 1;
    }

    output
}
pub const fn __concat_strs_init__<const LEN: usize>(strs: &[&str]) -> [u8; LEN] {
    assert!(LEN == __concat_strs_len__(strs));

    let mut output = unsafe { MaybeUninit::<[u8; LEN]>::uninit().assume_init() };

    {
        let mut output_index = 0;
        let mut str_index = 0;
        let mut byte_index = 0;

        while output_index < LEN {
            output[output_index] = strs[str_index].as_bytes()[byte_index];

            output_index += 1;
            if output_index < LEN {
                byte_index += 1;
                if byte_index == strs[str_index].len() {
                    str_index += 1;
                    byte_index = 0;
                }
            }
        }
    };

    output
}
pub const fn __concat_strs_into__<const LEN: usize>(init: &'static [u8; LEN]) -> &'static str {
    unsafe { str::from_utf8_unchecked(init) }
}
