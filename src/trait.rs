use std::intrinsics::{
    fadd_fast as add, fdiv_fast as div, fmul_fast as mul, frem_fast as rem, fsub_fast as sub,
};
macro_rules! meth {
    ($($name:ident)|+) => {
        pub trait FastFloat: Copy {
            $(#[doc(hidden)] unsafe fn $name(a: Self, b: Self) -> Self;)+
            #[doc(hidden)]
            fn bad(self) -> bool;
        }

        impl FastFloat for f32 {
            $(unsafe fn $name(a: Self, b: Self) -> Self {
                $name(a, b)
            })+


            fn bad(self) -> bool { self.is_nan() || self.is_infinite() }
        }

        impl FastFloat for f64 {
            $(unsafe fn $name(a: Self, b: Self) -> Self {
                $name(a, b)
            })+

            fn bad(self) -> bool { self.is_nan() || self.is_infinite() }
        }
    };
}
meth!(add | sub | div | mul | rem);