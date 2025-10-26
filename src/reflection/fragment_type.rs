use crate::reflection::ShaderType;

pub trait FragmentType: ShaderType {
    #[doc(hidden)]
    const _ASSERT: ();
}

impl FragmentType for f32 {
    const _ASSERT: () = ();
}

impl FragmentType for i32 {
    const _ASSERT: () = ();
}

impl FragmentType for u32 {
    const _ASSERT: () = ();
}
