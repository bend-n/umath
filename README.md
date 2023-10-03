# umath: ffast-math, for rust.

[![MSRV](https://img.shields.io/badge/msrv-nightly-blue?style=for-the-badge&logo=rust)](#nightlyness)
[![DOCS](https://img.shields.io/badge/docs.rs-umath-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs")](https://docs.rs/umath)

Want to make your math *faster*? [<sup>*t&c apply</sup>](https://simonbyrne.github.io/notes/fastmath)

Want to *order* a float?

You can do all of that, with `umath`!

```rs
use umath::FFloat;
// wrap a non NAN and non INF f32/f64 (we will also *never* make this number nan).
let mut f = unsafe { FFloat::new(4.0f32) };
f *= 3; // multiply by 3
// this check will be removed by the optimizer!
assert!(!f.is_nan());
# use std::collections::BinaryHeap;
// use a ORD type! this is allowed, as FFloat is not allowed to be NAN | INF.
let mut b = BinaryHeap::new();
b.push(unsafe { FFloat::new(2.0) });
b.push(unsafe { FFloat::new(1.0) });
b.push(unsafe { FFloat::new(3.0) });
b.push(f);
assert_eq!(b.pop(), Some(unsafe { FFloat::new(24.0) }));
```

## A note on safety

When you make your first [`FFLoat`](https://docs.rs/umath/latest/umath/struct.FFloat.html), you must promise that you will never create a [`NAN`](https://doc.rust-lang.org/nightly/std/primitive.f32.html#associatedconstant.NAN) | [`INF`](https://doc.rust-lang.org/nightly/std/primitive.f32.html#associatedconstant.INFINITY) [`FFLoat`](https://docs.rs/umath/latest/umath/struct.FFloat.html). Hence, `*f = NAN` is (delayed) UB.

### Nightlyness

`umath` is nightly because it makes use of core intrinsics, like [`fadd_fast()`](https://doc.rust-lang.org/nightly/core/intrinsics/fn.fadd_fast.html), which require the [`core_intrinsics`](https://doc.rust-lang.org/nightly/unstable-book/library-features/core-intrinsics.html) feature to use.