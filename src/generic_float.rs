//! provides generic float traits.
//! this is the best way to make a function possibly take a [`FFloat`].
//! ```
//! # use umath::*;
//! /// this function can take anything that implements Float, and "works with" a f32: it can be added to a f32, it can be created from a f32, etc.
//! /// with no external implementations, this can take either f32 or FFloat<f32>.
//! fn takes_float<F: Float<f32>>(f: F) {}
//! ```
use crate::{FFloat, FastFloat};
use core::ops::{
    Add, AddAssign, Deref, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};
#[cfg(doc)]
use std::f32::{INFINITY as INF, NAN};

macro_rules! simp {
    ($doc:literal trait $trat:ident with $($name:ident$(($arg:ident))?),+) => {
        #[doc = $doc]
        pub trait $trat {
            $(
                #[doc = concat!("Refer to [`f32::", stringify!($name), "`]")]
                fn $name(self $(, $arg: Self)?) -> Self;
            )+
        }

        impl $trat for f32 { $(fn $name(self $(, $arg: Self)?) -> Self { self.$name($($arg)?) })+ }
        impl $trat for f64 { $(fn $name(self $(, $arg: Self)?) -> Self { self.$name($($arg)?) })+ }
        impl<T: FastFloat + Trig + Rounding + Log> $trat for FFloat<T> {
            $(
                #[doc = include_str!("ffloat_safety_notice.md")]
                fn $name(self $(, $arg: Self)?) -> Self { unsafe { FFloat::new(self.deref().$name($(*$arg)?)) } }
            )+
        }
    };
}

simp!["Trigonometry functions" trait Trig with sin, asin, sinh, asinh, cos, acos, cosh, acosh, tan, atan, atan2(other), tanh, atanh];
simp!["Rounding functions" trait Rounding with floor, ceil, round];
simp!["Logarithm functions" trait Log with log(base), log2, log10, ln];

macro_rules! ctor {
    ($for:ty) => {
        impl Constructors for $for {
            /// Returns 0. This function is safe to call.
            unsafe fn zero() -> $for {
                0.0
            }

            /// Returns 1. This function is safe to call.
            unsafe fn one() -> $for {
                1.0
            }

            #[doc = concat!("Returns [`", stringify!($for), "::MIN`]. This function is safe to call")]
            unsafe fn min() -> $for {
                <$for>::MIN
            }

            #[doc = concat!("Returns [`", stringify!($for), "::MAX`]. This function is safe to call")]
            unsafe fn max() -> $for {
                <$for>::MAX
            }
        }
    };
}

ctor!(f32);
ctor!(f64);

/// Float constructors.
pub trait Constructors {
    /// Returns 0.
    #[doc = include_str!("refer.md")]
    unsafe fn zero() -> Self;

    /// Returns 1.
    #[doc = include_str!("refer.md")]
    unsafe fn one() -> Self;

    /// Returns the minimum value for this float.
    #[doc = include_str!("refer.md")]
    unsafe fn min() -> Self;

    /// Returns the maximum value for this float.
    #[doc = include_str!("refer.md")]
    unsafe fn max() -> Self;
}

/// Methods on a float.
/// If there is a method you would like to see on this trait, please open a issue.
///
/// Do note that the implementations of these functions are provided by std.
/// These functions are not likely to be faster than the std counterparts, unless the implementation is software provided and can benefit from fast math.
pub trait FloatMethods: Trig + Rounding + Log {
    /// Refer to [`f32::trunc`]
    fn trunc(self) -> Self;

    /// Refer to [`f32::fract`]
    fn fract(self) -> Self;

    /// Refer to [`f32::abs`]
    fn abs(self) -> Self;

    /// Refer to [`f32::powi`]
    fn powi(self, n: i32) -> Self;

    /// Refer to [`f32::powf`]
    fn powf(self, n: Self) -> Self;

    /// Refer to [`f32::sqrt`]
    fn sqrt(self) -> Self;

    /// Refer to [`f32::cbrt`]
    fn cbrt(self) -> Self;

    /// Refer to [`f32::hypot`]
    fn hypot(self, other: Self) -> Self;

    /// Refer to [`f32::exp2`]
    fn exp2(self) -> Self;

    /// Refer to [`f32::min`]
    fn min(self, other: Self) -> Self;

    /// Refer to [`f32::max`]
    fn max(self, other: Self) -> Self;
}

/// Completely stand-alone [`Float`].
/// This is comparable to something like [num_traits::Float](https://docs.rs/num-traits/latest/num_traits/float/trait.Float.html).
pub trait FloatAlone:
    PartialEq
    + PartialOrd
    + Copy
    + Constructors
    + FloatMethods
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Rem<Self, Output = Self>
    + Div<Self, Output = Self>
    + Neg<Output = Self>
    + AddAssign<Self>
    + SubAssign<Self>
    + MulAssign<Self>
    + DivAssign<Self>
    + RemAssign<Self>
{
}

impl<
        T: PartialEq
            + PartialOrd
            + Copy
            + Constructors
            + FloatMethods
            + Add<T, Output = T>
            + Sub<T, Output = T>
            + Mul<T, Output = T>
            + Rem<T, Output = T>
            + Div<T, Output = T>
            + Neg<Output = T>
            + AddAssign<T>
            + SubAssign<T>
            + MulAssign<T>
            + DivAssign<T>
            + RemAssign<T>,
    > FloatAlone for T
{
}

/// Generic float trait, implemented by {[`FFloat`], [`f32`], [`f64`]}. Takes a "base" argument, intended to be set to {[`f32`], [`f64`]}.
/// The main purpose of this is to be taken (generically) by optionally fast functions.
///
///
/// # Safety
///
/// Please note that calling these functions on a [`FFloat`] _may_ incur UB.
/// These functions are not marked `unsafe`, as the entire [`FFloat`] type is essentially unsafe.
/// Calling these functions on a [`f32`] is perfectly safe, even the `unsafe` marked functions (although theres not much point in doing so).
pub trait Float<F>:
    PartialEq
    + PartialOrd
    + PartialOrd<F>
    + Copy
    + Constructors
    + FloatMethods
    + Add<Self, Output = Self>
    + Add<F, Output = Self>
    + Sub<Self, Output = Self>
    + Sub<F, Output = Self>
    + Mul<Self, Output = Self>
    + Mul<F, Output = Self>
    + Rem<Self, Output = Self>
    + Rem<F, Output = Self>
    + Div<Self, Output = Self>
    + Neg<Output = Self>
    + Div<F, Output = Self>
    + AddAssign<Self>
    + AddAssign<F>
    + SubAssign<Self>
    + SubAssign<F>
    + MulAssign<Self>
    + MulAssign<F>
    + DivAssign<Self>
    + DivAssign<F>
    + RemAssign<Self>
    + RemAssign<F>
where
    Self: Sized,
{
    /// Returns a new [`Self`] from the float.
    #[doc = include_str!("refer.md")]
    unsafe fn new(from: F) -> Self;

    /// Returns this float
    fn take(self) -> F;
}

macro_rules! impf {
    ($for:ty) => {
        impl Float<$for> for $for {
            /// Returns the input value. This function is safe to call.
            unsafe fn new(from: $for) -> $for {
                from
            }
            fn take(self) -> $for {
                self
            }
        }
        impl FloatMethods for $for {
            fn trunc(self) -> $for {
                self.trunc()
            }
            fn fract(self) -> $for {
                self.fract()
            }
            fn abs(self) -> $for {
                self.abs()
            }
            fn powi(self, n: i32) -> $for {
                self.powi(n)
            }
            fn powf(self, n: $for) -> $for {
                self.powf(n)
            }
            fn sqrt(self) -> $for {
                self.sqrt()
            }
            fn cbrt(self) -> $for {
                self.cbrt()
            }
            fn hypot(self, other: Self) -> $for {
                self.hypot(other)
            }
            fn exp2(self) -> $for {
                self.exp2()
            }
            fn min(self, other: Self) -> Self {
                self.min(other)
            }
            fn max(self, other: Self) -> Self {
                self.max(other)
            }
        }
    };
}

impf!(f32);
impf!(f64);

impl<F: FastFloat + Constructors> Constructors for FFloat<F> {
    /// Create a new [`FFloat`] representing `0.0`.
    #[doc = include_str!("ffloat_safety_noconstr.md")]
    unsafe fn zero() -> Self {
        Self::new(F::zero())
    }
    /// Create a new [`FFloat`] representing `1.0`.
    #[doc = include_str!("ffloat_safety_noconstr.md")]
    unsafe fn one() -> Self {
        Self::new(F::one())
    }
    /// Create a new [`FFloat`] representing the minimum value for the inner float..
    #[doc = include_str!("ffloat_safety_noconstr.md")]
    unsafe fn min() -> Self {
        Self::new(F::min())
    }
    /// Create a new [`FFloat`] representing the maximum value for the inner float..
    #[doc = include_str!("ffloat_safety_noconstr.md")]
    unsafe fn max() -> Self {
        Self::new(F::max())
    }
}

macro_rules! reuse {
    (fn $name:ident) => {
        #[doc = concat!("Refer to [`f32::", stringify!($name), "`]")]
        #[doc = include_str!("ffloat_safety_notice.md")]
        fn $name(self) -> Self {
            self.check();
            unsafe { Self::new(self.0.$name()) }
        }
    };
}

impl<F: FastFloat + Float<F>> Float<F> for FFloat<F> {
    /// Create a new [`FFloat`] from your {[`f32`], [`f64`]}
    #[doc = include_str!("ffloat_safety.md")]
    unsafe fn new(from: F) -> Self {
        Self::new(from)
    }

    fn take(self) -> F {
        self.0
    }
}

impl<F: FloatMethods + FastFloat + Float<F>> FloatMethods for FFloat<F> {
    reuse!(fn trunc);
    reuse!(fn fract);
    reuse!(fn abs);

    /// Refer to [`f32::powi`]
    #[doc = include_str!("ffloat_safety_notice.md")]
    fn powi(self, n: i32) -> Self {
        unsafe { Self::new(self.0.powi(n)) }
    }

    /// Refer to [`f32::powf`]
    #[doc = include_str!("ffloat_safety_notice.md")]
    fn powf(self, n: Self) -> Self {
        self.check();
        unsafe { Self::new(self.0.powf(*n)) }
    }

    reuse!(fn sqrt);
    reuse!(fn cbrt);
    /// Refer to [`f32::hypot`]
    #[doc = include_str!("ffloat_safety_notice.md")]
    fn hypot(self, other: Self) -> Self {
        self.check();
        unsafe { Self::new(self.0.hypot(*other)) }
    }
    reuse!(fn exp2);

    /// Refer to [`f32::min`]
    #[doc = include_str!("ffloat_safety_notice.md")]
    fn min(self, other: Self) -> Self {
        self.check();
        unsafe { Self::new(self.0.min(*other)) }
    }

    /// Refer to [`f32::max`]
    #[doc = include_str!("ffloat_safety_notice.md")]
    fn max(self, other: Self) -> Self {
        self.check();
        unsafe { Self::new(self.0.max(*other)) }
    }
}

#[test]
fn usable() {
    fn cos<F: Float<f32>>(x: F) -> F {
        let mut y = x * (1.0 / 6.283);
        y -= (y + 0.25).floor() + 0.25;
        y *= (y.abs() - 0.5) * 16.0;
        return y;
    }
    assert!((0.995..0.996).contains(&cos(0.1)));
    assert!((0.995..0.996).contains(&*cos(unsafe { FFloat::new(0.1) })));
}
