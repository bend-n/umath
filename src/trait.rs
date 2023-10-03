use core::intrinsics::{
    fadd_fast as add, fdiv_fast as div, fmul_fast as mul, frem_fast as rem, fsub_fast as sub,
};
macro_rules! meth {
    ($($name:ident)|+) => {
        pub trait FastFloat: Copy + core::fmt::Display + core::fmt::Debug + core::ops::Neg<Output = Self> + core::cmp::PartialEq + core::cmp::PartialOrd {
            $(#[doc(hidden)] unsafe fn $name(a: Self, b: Self) -> Self;)+
            #[doc(hidden)]
            fn bad(self) -> bool;
        }

        impl FastFloat for f32 {
            $(#[inline(always)] unsafe fn $name(a: Self, b: Self) -> Self {
                $name(a, b)
            })+

            #[inline(always)]
            fn bad(self) -> bool { self.is_nan() || self.is_infinite() }
        }

        impl FastFloat for f64 {
            $(#[inline(always)] unsafe fn $name(a: Self, b: Self) -> Self {
                $name(a, b)
            })+

            #[inline(always)]
            fn bad(self) -> bool { self.is_nan() || self.is_infinite() }
        }
    };
}
meth!(add | sub | div | mul | rem);
