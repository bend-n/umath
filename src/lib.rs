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
#![warn(clippy::pedantic, clippy::dbg_macro, missing_docs)]
#![allow(clippy::return_self_not_must_use)]
use core::cmp::{Ordering, PartialEq, PartialOrd};
use core::ops::{
    Add as add, AddAssign as add_assign, Deref, DerefMut, Div as div, DivAssign as div_assign,
    Mul as mul, MulAssign as mul_assign, Neg, Rem as rem, RemAssign as rem_assign, Sub as sub,
    SubAssign as sub_assign,
};
#[cfg(doc)]
use std::f32::{INFINITY as INF, NAN};
use std::hash::Hash;

/// Type alias for <code>[FFloat]<[f32]></code>. (fast float 32 bits)
pub type FF32 = FFloat<f32>;
/// Type alias for <code>[FFloat]<[f64]></code>. (fast float 64 bits)
pub type FF64 = FFloat<f64>;

pub mod generic_float;
mod r#trait;
#[doc(inline)]
pub use generic_float::Float;
use r#trait::FastFloat;

/// Float wrapper that uses `ffast-math`. This float also implements [`Ord`], [`Hash`], and [`Eq`], as it is not allowed to be [`NAN`].
///
/// `FFloat<F>` is guaranteed to have the same memory layout and ABI as F.
/// ```
/// # use umath::FFloat;
/// # unsafe {
/// let result = FFloat::new(27.0) * 42109.0;
/// assert_eq!(*result, 1136943.0);
/// # }
/// ```
///
/// ## Safety Notice (for transmuters)
///
/// A [`FFloat`] is _never_ allowed to be [`NAN`] | [`INF`].
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq)]
pub struct FFloat<T>(T);

impl<T: FastFloat> core::fmt::Debug for FFloat<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl<T: FastFloat> core::fmt::Display for FFloat<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T: FastFloat> FFloat<T> {
    /// Create a new [`FFloat`] from your {[`f32`], [`f64`]}.
    #[doc = include_str!("ffloat_safety.md")]
    /// ```
    /// # use umath::FFloat;
    /// // SAFETY: i have verified that 7.0 is infact, not NAN or INF.
    /// let f = unsafe { FFloat::new(7.0) };
    /// ```
    pub unsafe fn new(from: T) -> Self {
        let new = Self(from);
        new.check();
        new
    }

    /// Checks if somebody else made a mistake, cause UB or panic if so.
    /// # Safety
    ///
    /// This can never cause UB unless someone else made a mistake, therefore ub has already occured.
    #[inline(always)]
    fn check(self) {
        if self.bad() {
            if cfg!(debug_assertions) {
                panic!("{self} is NAN | INF.");
            } else {
                unsafe { core::hint::unreachable_unchecked() };
            }
        }
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
                self.check();
                unsafe { Self::new(T::$name(self.0, rhs)) }
            }
        }

        impl<T: FastFloat> $name<&T> for FFloat<T> {
            type Output = FFloat<T>;

            fn $name(self, rhs: &T) -> Self::Output {
                self.check();
                unsafe { Self::new(T::$name(self.0, *rhs)) }
            }
        }

        impl<T: FastFloat> $name for FFloat<T> {
            type Output = FFloat<T>;
            fn $name(self, FFloat(rhs): FFloat<T>) -> Self::Output {
                self.check();
                unsafe { Self::new(T::$name(self.0, rhs)) }
            }
        }

        impl<T: FastFloat> $name<&FFloat<T>> for FFloat<T> {
            type Output = FFloat<T>;
            fn $name(self, FFloat(rhs): &FFloat<T>) -> Self::Output {
                self.check();
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

macro_rules! assign {
    ($name:ident, $op:ident) => {
        impl<T: FastFloat> $name<T> for FFloat<T> {
            fn $name(&mut self, rhs: T) {
                self.check();
                *self = unsafe { Self::new(T::$op(self.0, rhs)) };
            }
        }

        impl<T: FastFloat> $name<&T> for FFloat<T> {
            fn $name(&mut self, rhs: &T) {
                self.check();
                *self = unsafe { Self::new(T::$op(self.0, *rhs)) };
            }
        }

        impl<T: FastFloat> $name for FFloat<T> {
            fn $name(&mut self, FFloat(rhs): FFloat<T>) {
                self.check();
                *self = unsafe { Self::new(T::$op(self.0, rhs)) };
            }
        }

        impl<T: FastFloat> $name<&FFloat<T>> for FFloat<T> {
            fn $name(&mut self, FFloat(rhs): &FFloat<T>) {
                self.check();
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

// convenience
impl<T: FastFloat> Neg for FFloat<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self.check();
        unsafe { Self::new(-self.0) }
    }
}

impl<T: FastFloat> PartialEq<T> for FFloat<T> {
    fn eq(&self, other: &T) -> bool {
        self.check();
        self.0.eq(other)
    }
}
impl<T: FastFloat> Eq for FFloat<T> {}
impl<T: FastFloat> PartialOrd for FFloat<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<T: FastFloat> PartialOrd<T> for FFloat<T> {
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        self.check();
        self.0.partial_cmp(other)
    }
}
impl<T: FastFloat> Ord for FFloat<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.check();
        unsafe { self.0.partial_cmp(&other.0).unwrap_unchecked() }
    }
}

impl Hash for FFloat<f32> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.check();
        state.write_u32((self.0 + 0.0).to_bits());
    }
}

impl Hash for FFloat<f64> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.check();
        state.write_u64((self.0 + 0.0).to_bits());
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    #[test]
    fn it_works() {
        let result = unsafe { FFloat::new(2.0) + FFloat::new(2.0) };
        assert_eq!(*result, 4.0);
    }

    #[test]
    fn hashing() {
        let mut map = HashMap::new();
        map.insert(FFloat(2.0), "hi");
        map.insert(FFloat(7.0), "bye");
        map.insert(FFloat(-0.0), "edge");
        assert!(map[&FFloat(2.0)] == "hi");
        assert!(map[&FFloat(7.0)] == "bye");
        assert!(map[&FFloat(0.0)] == "edge");
    }
}
