use crate::constructs::*;

pub trait Element: Type {

}
macro_rules! element_ty {
    ($ty:ty) => {
        impl Type for $ty {

        }
        impl Element for $ty {

        }       
    };
}
element_ty!(f32);
element_ty!(f64);
element_ty!(u8);
element_ty!(u16);
element_ty!(u32);
element_ty!(u64);
element_ty!(u128);
element_ty!(i8);
element_ty!(i16);
element_ty!(i32);
element_ty!(i64);
element_ty!(i128);

macro_rules! vec_ty {
    ($ident:ident($($component:ident), +)) => {
        pub struct $ident<T: Element = f32> {
            $(
                pub $component: T,
            )+
        }
        impl<T: Element> Type for $ident<T> {
            
        }
    };
}
vec_ty!(Vec2(x, y));
vec_ty!(Vec3(x, y, z));
vec_ty!(Vec4(x, y, z, w));