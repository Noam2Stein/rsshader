use crate::GPUItemInfo;

pub unsafe trait GPUType {
    const GPU_TYPE_INFO: GPUTypeInfo;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GPUTypeInfo {
    pub item_info: GPUItemInfo,
    pub wgsl_reference: &'static str,
}

macro_rules! impl_array_gpu_type {
    ([$t:ty; $($n:literal), * $(,)?]) => {$(
        unsafe impl GPUType for [$t; $n] {
            const GPU_TYPE_INFO: GPUTypeInfo = GPUTypeInfo {
                item_info: GPUItemInfo {
                    id: 0,
                    dependencies: &[],
                    wgsl_declaration: "",
                },
                wgsl_reference: {
                    const STRS: &[&str] = &[
                        "array<",
                        <$t as GPUType>::GPU_TYPE_INFO.wgsl_reference,
                        ", ",
                        stringify!($n),
                        ">"
                    ];
                    crate::__concat_strs_into__(&crate::__concat_strs_init__::<{ crate::__concat_strs_len__(STRS) }>(STRS))
                },
            };
        }
    )*};
}
macro_rules! impl_gpu_type {
    ($ty:ty = wgsl($($wgsl_reference_tt:tt)*)) => {
        unsafe impl GPUType for $ty {
            const GPU_TYPE_INFO: GPUTypeInfo = GPUTypeInfo {
                item_info: GPUItemInfo {
                    id: 0,
                    dependencies: &[],
                    wgsl_declaration: "",
                },
                wgsl_reference: stringify!($($wgsl_reference_tt)*),
            };
        }

        impl_array_gpu_type!([$ty;
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
            11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
            21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
            31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
            41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
            51, 52, 53, 54, 55, 56, 57, 58, 59, 60,
            61, 62, 63, 64, 65, 66, 67, 68, 69, 70,
            71, 72, 73, 74, 75, 76, 77, 78, 79, 80,
            81, 82, 83, 84, 85, 86, 87, 88, 89, 90,
            91, 92, 93, 94, 95, 96, 97, 98, 99, 100,
        ]);
    };
}

impl_gpu_type!(bool = wgsl(bool));
impl_gpu_type!(f32 = wgsl(f32));
impl_gpu_type!(i32 = wgsl(i32));
impl_gpu_type!(u32 = wgsl(u32));
