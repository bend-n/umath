//! library providing a fast float wrapper.
//! ```
//! # use umath::FFloat;
//! # unsafe {
//! let mut f = FFloat::new(5.0);
//! f *= 7.0;
//! assert_eq!(*f, 35.0);
//! # }
//! ```
#![feature(core_intrinsics)]
#![warn(clippy::pedantic, clippy::dbg_macro, clippy::use_self, missing_docs)]
use std::ops::{
    Add as add, AddAssign as add_assign, Deref, DerefMut, Div as div, DivAssign as div_assign,
    Mul as mul, MulAssign as mul_assign, Neg, Rem as rem, RemAssign as rem_assign, Sub as sub,
    SubAssign as sub_assign,
};

mod r#trait;
use r#trait::FastFloat;

/// Float wrapper that uses `ffast-math`.
/// ```
/// # use umath::FFloat;
/// # unsafe {
/// let result = FFloat::new(27.0) * 42109.0;
/// assert_eq!(*result, 1136943.0);
/// # }
/// ```
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct FFloat<T>(T);

impl<T: FastFloat + std::fmt::Debug> std::fmt::Debug for FFloat<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl<T: FastFloat + std::fmt::Display> std::fmt::Display for FFloat<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T: FastFloat> FFloat<T> {
    /// Create a new [`FFloat`] from your {[`f32`], [`f64`]}.
    /// There is no checked new, because it needs to be `unsafe` so that i can make sure you will never do any funny.
    ///
    /// # Safety
    ///
    /// you must solemnly swear that your number is not [`NAN`](std::f32::NAN) | [`INF`](std::f32::INFINITY), and you will not make it [`NAN`](std::f32::NAN) | [`INF`](std::f32::INFINITY).
    /// ```
    /// # use umath::FFloat;
    /// // SAFETY: i have verified that 7.0 is infact, not NAN or INF.
    /// let f = unsafe { FFloat::new(7.0) };
    /// ```
    pub unsafe fn new(from: T) -> Self {
        debug_assert!(!from.bad());
        Self(from)
    }
}

impl<T> Deref for FFloat<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for FFloat<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

macro_rules! op {
    ($name:ident) => {
        impl<T: FastFloat> $name<T> for FFloat<T> {
            type Output = FFloat<T>;

            fn $name(self, rhs: T) -> Self::Output {
                unsafe { Self::new(T::$name(self.0, rhs)) }
            }
        }

        impl<T: FastFloat> $name<&T> for FFloat<T> {
            type Output = FFloat<T>;

            fn $name(self, rhs: &T) -> Self::Output {
                unsafe { Self::new(T::$name(self.0, *rhs)) }
            }
        }

        impl<T: FastFloat> $name for FFloat<T> {
            type Output = FFloat<T>;
            fn $name(self, FFloat(rhs): FFloat<T>) -> Self::Output {
                unsafe { Self::new(T::$name(self.0, rhs)) }
            }
        }

        impl<T: FastFloat> $name<&FFloat<T>> for FFloat<T> {
            type Output = FFloat<T>;
            fn $name(self, FFloat(rhs): &FFloat<T>) -> Self::Output {
                unsafe { Self::new(T::$name(self.0, *rhs)) }
            }
        }
    };
}

op!(add);
op!(div);
op!(mul);
op!(rem);
op!(sub);

impl<T: FastFloat + Neg<Output = T>> Neg for FFloat<T> {
    type Output = FFloat<T>;
    fn neg(self) -> Self::Output {
        unsafe { Self::new(-self.0) }
    }
}

macro_rules! assign {
    ($name:ident, $op:ident) => {
        impl<T: FastFloat> $name<T> for FFloat<T> {
            fn $name(&mut self, rhs: T) {
                *self = unsafe { Self::new(T::$op(self.0, rhs)) };
            }
        }

        impl<T: FastFloat> $name<&T> for FFloat<T> {
            fn $name(&mut self, rhs: &T) {
                *self = unsafe { Self::new(T::$op(self.0, *rhs)) };
            }
        }

        impl<T: FastFloat> $name for FFloat<T> {
            fn $name(&mut self, FFloat(rhs): FFloat<T>) {
                *self = unsafe { Self::new(T::$op(self.0, rhs)) };
            }
        }

        impl<T: FastFloat> $name<&FFloat<T>> for FFloat<T> {
            fn $name(&mut self, FFloat(rhs): &FFloat<T>) {
                *self = unsafe { Self::new(T::$op(self.0, *rhs)) };
            }
        }
    };
}
assign!(add_assign, add);
assign!(div_assign, div);
assign!(mul_assign, mul);
assign!(rem_assign, rem);
assign!(sub_assign, sub);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = unsafe { FFloat::new(2.0) + FFloat::new(2.0) };
        assert_eq!(*result, 4.0);
    }
}
