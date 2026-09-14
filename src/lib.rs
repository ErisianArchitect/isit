#![cfg_attr(feature = "no-std", no_std)]
//! `isit` is a compile time checking library. It has a multitude of functions for performing compile time checks to
//! ensure that your program will work as expected before you ever run it.
//! 
//! # Examples
//! 
//! ### `const_assert`
//! ```rust
//! #[repr(C)]
//! struct Fred {
//!     foo: u32,
//!     bar: u64,
//!     baz: bool,
//! }
//! const _: () = isit::const_assert(std::mem::offset_of!(Fred, bar) == 8);
//! ```
//! 
//! ### `const_assert_all`
//! ```rust
//! #[repr(C)]
//! struct Fred {
//!     foo: u32,
//!     bar: u64,
//!     baz: bool,
//! }
//! const _: () = isit::const_assert_all([
//!     std::mem::size_of::<Fred>() == 24,
//!     std::mem::offset_of!(Fred, foo) == 0,
//!     std::mem::offset_of!(Fred, bar) == 8,
//!     std::mem::offset_of!(Fred, baz) == 16,
//! ]);
//! ```
//! 
//! ### `const_assert_any`
//! ```rust
//! #[repr(C)]
//! struct Fred {
//!     foo: u32,
//!     bar: usize,
//!     baz: bool,
//! }
//! const _: () = isit::const_assert_any([
//!     // 64-bit systems
//!     std::mem::size_of::<Fred>() == 24,
//!     // 32-bit systems
//!     std::mem::size_of::<Fred>() == 12,
//!     // 16-bit systems
//!     std::mem::size_of::<Fred>() == 8,
//! ]);
//! ```
//! 
//! ### `const_assert_none`
//! ```rust
//! #[repr(C)]
//! struct Fred {
//!     foo: u32,
//!     bar: u64,
//!     baz: bool,
//! }
//! const _: () = isit::const_assert_none([
//!     // align is 8, so none of these will be true.
//!     std::mem::align_of::<Fred>() == 1,
//!     std::mem::align_of::<Fred>() == 2,
//!     std::mem::align_of::<Fred>() == 4,
//! ]);
//! ```
//! 
//! ### `assert_size`
//! ```rust
//! #[repr(C)]
//! struct Fred {
//!     foo: u32,
//!     bar: u64,
//!     baz: bool,
//! }
//! const _: () = isit::assert_size::<Fred, 24>();
//! ```
//! 
//! ### `assert_not_size`
//! ```rust
//! #[repr(C)]
//! struct Fred {
//!     foo: u32,
//!     bar: u64,
//!     baz: bool,
//! }
//! // If the type were not `C` layout, and were instead `Rust`
//! // layout, the size would end up being 16 instead of 24, and
//! // this check would fail.
//! const _: () = isit::assert_not_size::<Fred, 16>();
//! ```
//! 
//! ### `assert_same_size`
//! ```rust
//! #[repr(C)]
//! struct Fred {
//!     foo: u32,
//!     bar: u64,
//!     baz: bool,
//! }
//! const _: () = isit::assert_same_size::<Fred, [u64; 3]>();
//! ```
//! 
//! ### `assert_different_size`
//! ```rust
//! #[repr(C)]
//! struct Fred {
//!     foo: u32,
//!     bar: u64,
//!     baz: bool,
//! }
//! /// 
//! 
//! #[repr(C)]
//! struct NotFred {
//!     bar: u64,
//!     foo: u32,
//!     baz: bool,
//! }
//! const _: () = isit::assert_different_size::<Fred, NotFred>();
//! ```
//! 
//! ### `assert_compatible_size`
//! ```rust
//! #[repr(C)]
//! struct Fred {
//!     foo: u32,
//!     bar: u64,
//!     baz: bool,
//! }
//! const _: () = isit::assert_compatible_size::<[u64; 1], Fred>();
//! const _: () = isit::assert_compatible_size::<[u64; 2], Fred>();
//! const _: () = isit::assert_compatible_size::<[u64; 3], Fred>();
//! ```
//! 
//! ### `assert_incompatible_size`
//! ```rust
//! #[repr(C)]
//! struct Fred {
//!     foo: u32,
//!     bar: u64,
//!     baz: bool,
//! }
//! const _: () = isit::assert_incompatible_size::<Fred, [u64; 2]>();
//! ```
//! 
//! ### `assert_align`
//! ```rust
//! #[repr(C)]
//! struct Fred {
//!     foo: u32,
//!     bar: u64,
//!     baz: bool,
//! }
//! const _: () = isit::assert_align::<Fred, 8>();
//! ```
//! 
//! ### `assert_not_align`
//! ```rust
//! #[repr(C)]
//! struct Fred {
//!     foo: u32,
//!     bar: u64,
//!     baz: bool,
//! }
//! const _: () = isit::assert_not_align::<Fred, 4>();
//! const _: () = isit::assert_not_align::<Fred, 16>();
//! ```
//! 
//! ### `assert_same_align`
//! ```rust
//! #[repr(C)]
//! struct Fred {
//!     foo: u32,
//!     bar: u64,
//!     baz: bool,
//! }
//! const _: () = isit::assert_same_align::<Fred, u64>();
//! ```
//! 
//! ### `assert_different_align`
//! ```rust
//! #[repr(C)]
//! struct Fred {
//!     foo: u32,
//!     bar: u64,
//!     baz: bool,
//! }
//! const _: () = isit::assert_different_align::<Fred, u32>();
//! const _: () = isit::assert_different_align::<Fred, bool>();
//! ```
//! 
//! ### `assert_compatible_align`
//! ```rust
//! #[repr(C)]
//! struct Fred {
//!     foo: u32,
//!     bar: u64,
//!     baz: bool,
//! }
//! const _: () = isit::assert_compatible_align::<Fred, u64>();
//! ```
//! 
//! ### `assert_incompatible_align`
//! ```rust
//! #[repr(C)]
//! struct Fred {
//!     foo: u32,
//!     bar: u64,
//!     baz: bool,
//! }
//! const _: () = isit::assert_incompatible_align::<Fred, u32>();
//! const _: () = isit::assert_incompatible_align::<Fred, bool>();
//! ```
//! 
//! ### `assert_size_align`
//! ```rust
//! #[repr(C)]
//! struct Fred {
//!     foo: u32,
//!     bar: u64,
//!     baz: bool,
//! }
//! const _: () = isit::assert_size_align::<Fred, 24, 8>();
//! ```
//! 
//! ### `assert_not_size_align`
//! ```rust
//! #[repr(C)]
//! struct Fred {
//!     foo: u32,
//!     bar: u64,
//!     baz: bool,
//! }
//! const _: () = isit::assert_not_size_align::<Fred, 16, 4>();
//! ```
//! 
//! ### `assert_same_size_align`
//! ```rust
//! #[repr(C)]
//! struct Fred {
//!     foo: u32,
//!     bar: u64,
//!     baz: bool,
//! }
//! const _: () = isit::assert_same_size_align::<Fred, [u64; 3]>();
//! ```
//! 
//! ### `assert_different_size_align`
//! ```rust
//! #[repr(C)]
//! struct Fred {
//!     foo: u32,
//!     bar: u64,
//!     baz: bool,
//! }
//! const _: () = isit::assert_different_size_align::<Fred, [u32; 4]>();
//! ```
//! 
//! ### `assert_compatible_size_align`
//! ```rust
//! #[repr(C)]
//! struct Fred {
//!     foo: u32,
//!     bar: u64,
//!     baz: bool,
//! }
//! const _: () = isit::assert_compatible_size_align::<[u32; 3], Fred>();
//! ```
//! 
//! ### `assert_incompatible_size_algin`
//! ```rust
//! #[repr(C)]
//! struct Fred {
//!     foo: u32,
//!     bar: u64,
//!     baz: bool,
//! }
//! const _: () = isit::assert_incompatible_size_align::<Fred, u32>();
//! ```
//! 
//! ### `assert_pointer_size`
//! ```rust
//! use std::ptr::NonNull;
//! #[repr(transparent)]
//! struct Foo {
//!     ptr: NonNull<u8>,
//! }
//! const _: () = isit::assert_pointer_size::<Foo>();
//! ```
//! 
//! ### `assert_not_pointer_size`
//! ```rust
//! #[repr(C)]
//! struct Foo {
//!     a: bool,
//!     b: u16,
//!     c: bool,
//!     d: u16,
//!     e: bool,
//! }
//! const _: () = isit::assert_not_pointer_size::<Foo>();
//! ```
//! 
//! ### `assert_pointer_align`
//! ```rust
//! #[cfg_attr(target_pointer_width = "64", repr(C, align(8)))]
//! #[cfg_attr(target_pointer_width = "32", repr(C, align(4)))]
//! #[cfg_attr(target_pointer_width = "16", repr(C, align(2)))]
//! struct Foo(usize);
//! const _: () = isit::assert_pointer_align::<Foo>();
//! ```
//! 
//! ### `assert_not_pointer_align`
//! ```rust
//! #[repr(transparent)]
//! struct Foo {
//!     inner: u8,
//! }
//! const _: () = isit::assert_not_pointer_align::<Foo>();
//! ```
//! 
//! ### `assert_pointer_size_align`
//! ```rust
//! use std::ptr::NonNull;
//! /// 
//! 
//! #[repr(transparent)]
//! struct Foo(NonNull<Foo>);
//! const _: () = isit::assert_pointer_size_align::<Foo>();
//! ```
//! 
//! ### `assert_not_pointer_size_align`
//! ```rust
//! #[repr(transparent)]
//! struct Foo(u8);
//! const _: () = isit::assert_not_pointer_size_align::<Foo>();
//! ```
//! 
//! ### `assert_pointer_compatible_size_align`
//! ```rust
//! #[repr(transparent)]
//! struct Foo(u8);
//! const _: () = isit::assert_pointer_compatible_size_align::<Foo>();
//! ```
//! 
//! ### `assert_pointer_incompatible_size_align`
//! ```rust
//! #[repr(C, align(16))]
//! struct Fred {
//!     foo: u32,
//!     bar: u64,
//!     baz: bool,
//! }
//! const _: () = isit::assert_pointer_incompatible_size_align::<Fred>();
//! ```
//! 
//! ### `assert_t_niche`
//! ```rust
//! use std::ptr::NonNull;
//! /// 
//! 
//! #[repr(transparent)]
//! struct Foo {
//!     ptr: NonNull<Foo>,
//! }
//! const _: () = isit::assert_t_niche::<Foo, NonNull<Foo>>();
//! ```
//! 
//! ### `assert_no_t_niche`
//! ```rust
//! #[repr(transparent)]
//! struct Foo {
//!     foo: u32,
//! }
//! const _: () = isit::assert_no_t_niche::<Foo, u32>();
//! ```
//! 
//! ### `assert_t_niche_compatible`
//! ```rust
//! use std::num::NonZero;
//! #[repr(transparent)]
//! struct Foo(NonZero<u8>);
//! const _: () = isit::assert_t_niche_compatible::<Foo, NonZero<u8>>();
//! const _: () = isit::assert_t_niche_compatible::<Foo, NonZero<u16>>();
//! ```
//! 
//! ### `assert_niche`
//! ```rust
//! use std::ptr::NonNull;
//! /// 
//! 
//! #[repr(transparent)]
//! struct Foo(NonNull<Foo>);
//! const _: () = isit::assert_niche::<Foo>();
//! ```
//! 
//! ### `assert_no_niche`
//! ```rust
//! #[repr(transparent)]
//! struct Foo(u64);
//! const _: () = isit::assert_no_niche::<Foo>();
//! ```
//! 
//! ### `assert_single_niche`
//! ```rust
//! use std::ptr::NonNull;
//! /// 
//! 
//! #[repr(transparent)]
//! struct Foo(NonNull<Foo>);
//! const _: () = isit::assert_single_niche::<Foo>();
//! ```
//! ### `assert_pointer_niche`
//! ```rust
//! use std::ptr::NonNull;
//! /// 
//! 
//! #[repr(transparent)]
//! struct Foo(NonNull<Foo>);
//! const _: () = isit::assert_pointer_niche::<Foo>();
//! ```
//! 
//! ### `assert_no_pointer_niche`
//! ```rust
//! #[repr(transparent)]
//! struct Foo(usize);
//! const _: () = isit::assert_no_pointer_niche::<Foo>();
//! ```
//! 
//! ### `assert_pointer_niche_compatible`
//! ```rust
//! use std::num::NonZero;
//! #[repr(transparent)]
//! struct Foo(NonZero<u128>);
//! const _: () = isit::assert_pointer_niche_compatible::<Foo>();
//! ```
//! 
//! ### `assert_u8_niche`
//! ```rust
//! #[repr(u8)]
//! enum Fred {
//!     Foo,
//!     Bar,
//!     Baz,
//! }
//! const _: () = isit::assert_u8_niche::<Fred>();
//! ```
//! 
//! ### `assert_u8_size_align`
//! ```rust
//! #[repr(transparent)]
//! struct Foo(u8);
//! const _: () = isit::assert_u8_size_align::<Foo>();
//! ```
//! 
//! ### `assert_u8_compatible`
//! ```rust
//! struct Zst;
//! #[repr(transparent)]
//! struct U8(u8);
//! const _: () = isit::assert_u8_compatible::<Zst>();
//! const _: () = isit::assert_u8_compatible::<U8>();
//! ```
//! 
//! ### `assert_u8_incompatible`
//! ```rust
//! #[repr(transparent)]
//! struct Foo(u16);
//! const _: () = isit::assert_u8_incompatible::<Foo>();
//! ```
//! 
//! ### `assert_zst`
//! ```rust
//! struct Zst;
//! const _: () = isit::assert_zst::<Zst>();
//! const _: () = isit::assert_zst::<()>();
//! const _: () = isit::assert_zst::<[[u64; u16::MAX as usize]; 0]>();
//! ```
//! 
//! ### `assert_not_zst`
//! ```rust
//! struct NonZst([u64; u16::MAX as usize]);
//! const _: () = isit::assert_not_zst::<NonZst>();
//! ```
//! 
//! ### `assert_uninhabited_zst`
//! ```rust
//! enum Uninhabited {}
//! struct Holder<T>(T);
//! const _: () = isit::assert_uninhabited_zst::<Uninhabited>();
//! const _: () = isit::assert_uninhabited_zst::<(Uninhabited, Uninhabited)>();
//! const _: () = isit::assert_uninhabited_zst::<[Uninhabited; usize::MAX]>();
//! const _: () = isit::assert_uninhabited_zst::<Holder<Uninhabited>>();
//! const _: () = isit::assert_uninhabited_zst::<Holder<[Uninhabited; usize::MAX]>>();
//! const _: () = isit::assert_uninhabited_zst::<Holder<[[Uninhabited; usize::MAX]; usize::MAX]>>();
//! const _: () = isit::assert_uninhabited_zst::<Holder<[[[Uninhabited; usize::MAX]; usize::MAX]; usize::MAX]>>();
//! const _: () = isit::assert_uninhabited_zst::<Holder<(
//!     Holder<[[[Uninhabited; usize::MAX]; usize::MAX]; usize::MAX]>,
//!     Holder<[[[Uninhabited; usize::MAX]; usize::MAX]; usize::MAX]>,
//! )>>();
//! ```
//! 
//! ### `assert_not_uninhabited_zst`
//! ```rust
//! struct Foo(u8);
//! const _: () = isit::assert_not_uninhabited_zst::<Foo>();
//! ```

mod meta;

#[cfg(feature = "macros")]
pub use isit_macros::*;

use core::{num::NonZero, ptr::NonNull};
use meta::{Niche255, UninhabitedZst};

/// Assert that a condition is `true` at compile time.
/// 
/// # Usage
/// ```rust
/// #[repr(C)]
/// struct Fred {
///     foo: u32,
///     bar: u64,
///     baz: bool,
/// }
/// const _: () = isit::const_assert(std::mem::offset_of!(Fred, bar) == 8);
/// ```
#[track_caller]
#[inline(always)]
pub const fn const_assert(assert_condition: bool) {
    _ = assert_condition || panic!("Assertion failed");
}
const _: () = const_assert(true);

/// Check that all conditions are true at compile time.
#[must_use]
#[inline(always)]
pub const fn all_condition<const SIZE: usize>(conditions: [bool; SIZE]) -> bool {
    let mut index = 0;
    while index < SIZE {
        _ = conditions[index] || return false;
        index += 1;
    }
    true
}

/// Assert that all conditions are `true` at compile time.
/// 
/// # Usage
/// ```rust
/// #[repr(C)]
/// struct Fred {
///     foo: u32,
///     bar: u64,
///     baz: bool,
/// }
/// const _: () = isit::const_assert_all([
///     std::mem::size_of::<Fred>() == 24,
///     std::mem::offset_of!(Fred, foo) == 0,
///     std::mem::offset_of!(Fred, bar) == 8,
///     std::mem::offset_of!(Fred, baz) == 16,
/// ]);
/// ```
#[track_caller]
#[inline(always)]
pub const fn const_assert_all<const SIZE: usize>(assertions: [bool; SIZE]) {
    _ = all_condition(assertions) || panic!("Assertion failed");
}
const _: () = const_assert_all([
    true,
    true,
]);

/// Check that any condition is `true` at compile time.
#[must_use]
#[inline(always)]
pub const fn any_condition<const SIZE: usize>(conditions: [bool; SIZE]) -> bool {
    let mut index = 0;
    while index < SIZE {
        _ = conditions[index] && return true;
        index += 1;
    }
    false
}

/// Assert that any condition is `true` at compile time.
/// 
/// # Usage
/// ```rust
/// #[repr(C)]
/// struct Fred {
///     foo: u32,
///     bar: usize,
///     baz: bool,
/// }
/// const _: () = isit::const_assert_any([
///     // 64-bit systems
///     std::mem::size_of::<Fred>() == 24,
///     // 32-bit systems
///     std::mem::size_of::<Fred>() == 12,
///     // 16-bit systems
///     std::mem::size_of::<Fred>() == 8,
/// ]);
/// ```
#[track_caller]
#[inline(always)]
pub const fn const_assert_any<const SIZE: usize>(assertions: [bool; SIZE]) {
    _ = any_condition(assertions) || panic!("Assertion failed");
}
const _: () = const_assert_any([
    false,
    false,
    true,
]);

/// Check that none of the conditions are `true` at compile time.
#[must_use]
#[inline(always)]
pub const fn none_condition<const SIZE: usize>(conditions: [bool; SIZE]) -> bool {
    let mut index = 0;
    while index < SIZE {
        _ = conditions[index] && return false;
        index += 1;
    }
    true
}

/// Assert that none of the conditions are `true` at compile time.
/// 
/// # Usage
/// ```rust
/// #[repr(C)]
/// struct Fred {
///     foo: u32,
///     bar: u64,
///     baz: bool,
/// }
/// const _: () = isit::const_assert_none([
///     // align is 8, so none of these will be true.
///     std::mem::align_of::<Fred>() == 1,
///     std::mem::align_of::<Fred>() == 2,
///     std::mem::align_of::<Fred>() == 4,
/// ]);
/// ```
#[track_caller]
#[inline(always)]
pub const fn const_assert_none<const SIZE: usize>(assertions: [bool; SIZE]) {
    _ = none_condition(assertions) || panic!("Assertion failed");
}
const _: () = const_assert_none([
    false,
    false,
    false,
]);

/// Check that the size of `T` matches `SIZE` at compile time.
#[must_use]
#[inline(always)]
pub const fn size_condition<T, const SIZE: usize>() -> bool {
    const { size_of::<T>() == SIZE }
}

/// Assert that the size of `T` matches `SIZE` at compile time.
/// 
/// # Usage
/// ```rust
/// #[repr(C)]
/// struct Fred {
///     foo: u32,
///     bar: u64,
///     baz: bool,
/// }
/// const _: () = isit::assert_size::<Fred, 24>();
/// ```
#[track_caller]
#[inline(always)]
pub const fn assert_size<T, const SIZE: usize>() {
    _ = size_condition::<T, SIZE>() || panic!("Size mismatch");
}
const _: () = assert_size::<u8, 1>();

/// Check that the size of `T` does not match `SIZE` at compile time.
#[must_use]
#[inline(always)]
pub const fn not_size_condition<T, const SIZE: usize>() -> bool {
    const { size_of::<T>() != SIZE }
}

/// Assert that the size of `T` does not match `SIZE` at compile time.
/// 
/// # Usage
/// ```rust
/// #[repr(C)]
/// struct Fred {
///     foo: u32,
///     bar: u64,
///     baz: bool,
/// }
/// // If the type were not `C` layout, and were instead `Rust`
/// // layout, the size would end up being 16 instead of 24, and
/// // this check would fail.
/// const _: () = isit::assert_not_size::<Fred, 16>();
/// ```
#[track_caller]
#[inline(always)]
pub const fn assert_not_size<T, const SIZE: usize>() {
    _=not_size_condition::<T, SIZE>() || panic!("Same size");
}
const _: () = assert_not_size::<u64, 4>();

/// Check that type `L` is the same size as type `R` at compile time.
#[must_use]
#[inline(always)]
pub const fn same_size_condition<L, R>() -> bool {
    const { size_of::<L>() == size_of::<R>() }
}

/// Assert that type `L` is the same size as type `R` at compile time.
/// 
/// # Usage
/// ```rust
/// #[repr(C)]
/// struct Fred {
///     foo: u32,
///     bar: u64,
///     baz: bool,
/// }
/// const _: () = isit::assert_same_size::<Fred, [u64; 3]>();
/// ```
#[track_caller]
#[inline(always)]
pub const fn assert_same_size<L, R>() {
    _ = same_size_condition::<L, R>() || panic!("Size mismatch");
}
const _: () = assert_same_size::<u8, i8>();

/// Check that type `L` and type `R` have different sizes at compile time.
#[must_use]
#[inline(always)]
pub const fn different_size_condition<L, R>() -> bool {
    const { size_of::<L>() != size_of::<R>() }
}

/// Assert that type `L` and type `R` have different sizes at compile time.
/// 
/// # Usage
/// ```rust
/// #[repr(C)]
/// struct Fred {
///     foo: u32,
///     bar: u64,
///     baz: bool,
/// }
/// 
/// #[repr(C)]
/// struct NotFred {
///     bar: u64,
///     foo: u32,
///     baz: bool,
/// }
/// const _: () = isit::assert_different_size::<Fred, NotFred>();
/// ```
#[track_caller]
#[inline(always)]
pub const fn assert_different_size<L, R>() {
    _ = different_size_condition::<L, R>() || panic!("Size match");
}
const _: () = assert_different_size::<u8, u16>();

/// Check that type `Min` has a size that is less than or equal to the size of `Max` at compile time.
#[must_use]
#[inline(always)]
pub const fn compatible_size_condition<Min, Max>() -> bool {
    const { size_of::<Min>() <= size_of::<Max>() }
}

/// Assert that type `Min` has a size that is less than or equal to the size of `Max` at compile time.
/// 
/// # Usage
/// ```rust
/// #[repr(C)]
/// struct Fred {
///     foo: u32,
///     bar: u64,
///     baz: bool,
/// }
/// const _: () = isit::assert_compatible_size::<[u64; 1], Fred>();
/// const _: () = isit::assert_compatible_size::<[u64; 2], Fred>();
/// const _: () = isit::assert_compatible_size::<[u64; 3], Fred>();
/// ```
#[track_caller]
#[inline(always)]
pub const fn assert_compatible_size<Min, Max>() {
    _ = compatible_size_condition::<Min, Max>() || panic!("Incompatible size");
}
const _: () = assert_compatible_size::<u8, u16>();

/// Check that type `Max` has a size that is greater than the size of type `Min` at compile time.
#[must_use]
#[inline(always)]
pub const fn incompatible_size_condition<Max, Min>() -> bool {
    const { size_of::<Max>() > size_of::<Min>() }
}

/// Assert that type `Max` has a size that is greater than the size of type `Min` at compile time.
/// 
/// # Usage
/// ```rust
/// #[repr(C)]
/// struct Fred {
///     foo: u32,
///     bar: u64,
///     baz: bool,
/// }
/// const _: () = isit::assert_incompatible_size::<Fred, [u64; 2]>();
/// ```
#[track_caller]
#[inline(always)]
pub const fn assert_incompatible_size<Max, Min>() {
    _ = incompatible_size_condition::<Max, Min>() || panic!("Compatible size");
}
const _: () = assert_incompatible_size::<u64, u32>();

/// Check that type `T` has alignment of `ALIGN` at compile time.
#[must_use]
#[inline(always)]
pub const fn align_condition<T, const ALIGN: usize>() -> bool {
    const { align_of::<T>() == ALIGN }
}

/// Assert that type `T` has alignment of `ALIGN` at compile time.
/// 
/// # Usage
/// ```rust
/// #[repr(C)]
/// struct Fred {
///     foo: u32,
///     bar: u64,
///     baz: bool,
/// }
/// const _: () = isit::assert_align::<Fred, 8>();
/// ```
#[track_caller]
#[inline(always)]
pub const fn assert_align<T, const ALIGN: usize>() {
    _=align_condition::<T, ALIGN>() || panic!("Alignment mismatch");
}
const _: () = assert_align::<u64, 8>();

/// Check that type `T` does not have the alignment of `ALIGN` at compile time.
#[must_use]
#[inline(always)]
pub const fn not_align_condition<T, const ALIGN: usize>() -> bool {
    const { align_of::<T>() != ALIGN }
}

/// Assert that type `T` does not have alignment of `ALIGN` at compile time.
/// 
/// # Usage
/// ```rust
/// #[repr(C)]
/// struct Fred {
///     foo: u32,
///     bar: u64,
///     baz: bool,
/// }
/// const _: () = isit::assert_not_align::<Fred, 4>();
/// const _: () = isit::assert_not_align::<Fred, 16>();
/// ```
#[track_caller]
#[inline(always)]
pub const fn assert_not_align<T, const ALIGN: usize>() {
    _=not_align_condition::<T, ALIGN>() || panic!("Same alignment");
}
const _: () = assert_not_align::<u64, 4>();

/// Check that type `L` has the same alignment as type `R` at compile time.
#[must_use]
#[inline(always)]
pub const fn same_align_condition<L, R>() -> bool {
    const { align_of::<L>() == align_of::<R>() }
}

/// Assert that type `L` has the same alignment as type `R` at compile time.
/// 
/// # Usage
/// ```rust
/// #[repr(C)]
/// struct Fred {
///     foo: u32,
///     bar: u64,
///     baz: bool,
/// }
/// const _: () = isit::assert_same_align::<Fred, u64>();
/// ```
#[track_caller]
#[inline(always)]
pub const fn assert_same_align<L, R>() {
    _=same_align_condition::<L, R>() || panic!("Different alignment");
}

/// Check that type `L` has a different alignment than type `R` at compile time.
#[must_use]
#[inline(always)]
pub const fn different_align_condition<L, R>() -> bool {
    const { align_of::<L>() != align_of::<R>() }
}

/// Assert that type `L` has a different alignment than type `R` at compile time.
/// 
/// # Usage
/// ```rust
/// #[repr(C)]
/// struct Fred {
///     foo: u32,
///     bar: u64,
///     baz: bool,
/// }
/// const _: () = isit::assert_different_align::<Fred, u32>();
/// const _: () = isit::assert_different_align::<Fred, bool>();
/// ```
#[track_caller]
#[inline(always)]
pub const fn assert_different_align<L, R>() {
    _=different_align_condition::<L, R>() || panic!("Same alignment");
}
const _: () = assert_different_align::<u8, u16>();

/// Check that type `Min` has an alignment that is less than or equal to the alignment of type `Max` at compile time.
#[must_use]
#[inline(always)]
pub const fn compatible_align_condition<Min, Max>() -> bool {
    const { align_of::<Min>() <= align_of::<Max>() }
}

/// Assert that type `Min` has an alignment that is less than or equal to the alignment of type `Max` at compile time.
/// 
/// # Usage
/// ```rust
/// #[repr(C)]
/// struct Fred {
///     foo: u32,
///     bar: u64,
///     baz: bool,
/// }
/// const _: () = isit::assert_compatible_align::<Fred, u64>();
/// ```
#[track_caller]
#[inline(always)]
pub const fn assert_compatible_align<Min, Max>() {
    _=compatible_align_condition::<Min, Max>() || panic!("Incompatible align");
}
const _: () = assert_compatible_align::<u8, u16>();

/// Check that type `Max` has an alignment that is greater than the alignment of type `Min` at compile time.
#[must_use]
#[inline(always)]
pub const fn incompatible_align_condition<Max, Min>() -> bool {
    const { align_of::<Max>() > align_of::<Min>() }
}

/// Assert that type `Max` has an alignment that is greater than the alignment of type `Min` at compile time.
/// 
/// # Usage
/// ```rust
/// #[repr(C)]
/// struct Fred {
///     foo: u32,
///     bar: u64,
///     baz: bool,
/// }
/// const _: () = isit::assert_incompatible_align::<Fred, u32>();
/// const _: () = isit::assert_incompatible_align::<Fred, bool>();
/// ```
#[track_caller]
#[inline(always)]
pub const fn assert_incompatible_align<Max, Min>() {
    _=incompatible_align_condition::<Max, Min>() || panic!("Compatible align");
}
const _: () = assert_incompatible_align::<u32, u16>();

/// Check that type `T` has the size of `SIZE` and the alignment of `ALIGN` at compile time.
#[track_caller]
#[inline(always)]
pub const fn size_align_condition<T, const SIZE: usize, const ALIGN: usize>() -> bool {
    const {
        size_of::<T>() == SIZE
        && align_of::<T>() == ALIGN
    }
}

/// Assert that type `T` has the size of `SIZE` and the alignment of `ALIGN` at compile time.
/// 
/// # Usage
/// ```rust
/// #[repr(C)]
/// struct Fred {
///     foo: u32,
///     bar: u64,
///     baz: bool,
/// }
/// const _: () = isit::assert_size_align::<Fred, 24, 8>();
/// ```
#[track_caller]
#[inline(always)]
pub const fn assert_size_align<T, const SIZE: usize, const ALIGN: usize>() {
    // This function doesn't use the `size_align_condition` function
    // because you can more easily customize the panic message this way.
    let size_check = const { size_of::<T>() == SIZE };
    let align_check = const { align_of::<T>() == ALIGN };
    match (size_check, align_check) {
        (false, false) => panic!("Size and alignment mismatch"),
        (true, false) => panic!("Alignment mismatch"),
        (false, true) => panic!("Size mismatch"),
        (true, true) => (),
    }
}
const _: () = assert_size_align::<u32, 4, 4>();

/// Check that type `T` has either a different size than `SIZE` and a different alignment than `ALIGN` at compile time.
#[must_use]
#[inline(always)]
pub const fn not_size_align_condition<T, const SIZE: usize, const ALIGN: usize>() -> bool {
    const {
        size_of::<T>() != SIZE
        && align_of::<T>() != ALIGN
    }
}

/// Assert that type `T` has either a different size than `SIZE` and a different alignment than `ALIGN` at compile time.
/// 
/// # Usage
/// ```rust
/// #[repr(C)]
/// struct Fred {
///     foo: u32,
///     bar: u64,
///     baz: bool,
/// }
/// const _: () = isit::assert_not_size_align::<Fred, 16, 4>();
/// ```
#[track_caller]
#[inline(always)]
pub const fn assert_not_size_align<T, const SIZE: usize, const ALIGN: usize>() {
    const {
        let not_size = size_of::<T>() != SIZE;
        let not_align = align_of::<T>() != ALIGN;
        match (not_size, not_align) {
            (true, true) => (),
            (true, false) => panic!("Same alignment"),
            (false, true) => panic!("Same size"),
            (false, false) => panic!("Same size and alignment"),
        }
    }
}
const _: () = assert_not_size_align::<u8, 2, 2>();

/// Check that type `L` has the same size and alignment as type `R` at compile time.
#[must_use]
#[inline(always)]
pub const fn same_size_align_condition<L, R>() -> bool {
    const {
        size_of::<L>() == size_of::<R>()
        && align_of::<L>() == align_of::<R>()
    }
}

/// Check that type `L` has the same size and alignment as type `R` at compile time.
/// 
/// # Usage
/// ```rust
/// #[repr(C)]
/// struct Fred {
///     foo: u32,
///     bar: u64,
///     baz: bool,
/// }
/// const _: () = isit::assert_same_size_align::<Fred, [u64; 3]>();
/// ```
#[track_caller]
#[inline(always)]
pub const fn assert_same_size_align<L, R>() {
    // This function doesn't use the `same_size_align_condition` function
    // because you can more easily customize the panic message this way.
    let size_check = const { size_of::<L>() == size_of::<R>() };
    let align_check = const { align_of::<L>() == align_of::<R>() };
    match (size_check, align_check) {
        (false, false) => panic!("Size and alignment mismatch"),
        (true, false) => panic!("Alignment mismatch"),
        (false, true) => panic!("Size mismatch"),
        (true, true) => (),
    }
}
const _: () = assert_same_size_align::<i8, u8>();

/// Check that type `L` has a different size and alignment from type `R` at compile time.
#[must_use]
#[inline(always)]
pub const fn different_size_align_condition<L, R>() -> bool {
    const {
        size_of::<L>() != size_of::<R>()
        && align_of::<L>() != align_of::<R>()
    }
}

/// Assert that type `L` has a different size and alignment from type `R` at compile time.
/// 
/// # Usage
/// ```rust
/// #[repr(C)]
/// struct Fred {
///     foo: u32,
///     bar: u64,
///     baz: bool,
/// }
/// const _: () = isit::assert_different_size_align::<Fred, [u32; 4]>();
/// ```
#[track_caller]
#[inline(always)]
pub const fn assert_different_size_align<L, R>() {
    // This function doesn't use the `different_size_align_condition` function
    // because you can more easily customize the panic message this way.
    let size_check = const { size_of::<L>() != size_of::<R>() };
    let align_check = const { align_of::<L>() != align_of::<R>() };
    match (size_check, align_check) {
        (false, false) => panic!("Same size and alignment"),
        (true, false) => panic!("Same alignment"),
        (false, true) => panic!("Same size"),
        (true, true) => (),
    }
}
const _: () = assert_different_size_align::<u8, u32>();

/// Check that the size and alignment of type `Min` is less than or equal to the size and alignment of type `Max` at compile time.
#[must_use]
#[inline(always)]
pub const fn compatible_size_align_condition<Min, Max>() -> bool {
    const {
        size_of::<Min>() <= size_of::<Max>()
        && align_of::<Min>() <= align_of::<Max>()
    }
}

/// Assert that the size and alignment of type `Min` is less than or equal to the size and alignment of type `Max` at compile time.
/// 
/// # Usage
/// ```rust
/// #[repr(C)]
/// struct Fred {
///     foo: u32,
///     bar: u64,
///     baz: bool,
/// }
/// const _: () = isit::assert_compatible_size_align::<[u32; 3], Fred>();
/// ```
#[track_caller]
#[inline(always)]
pub const fn assert_compatible_size_align<Min, Max>() {
    // This function doesn't use the `compatible_size_align_condition` function
    // because you can more easily customize the panic message this way.
    let size_check = const { size_of::<Min>() <= size_of::<Max>() };
    let align_check = const { align_of::<Min>() <= align_of::<Max>() };
    match (size_check, align_check) {
        (false, false) => panic!("Incompatible size and alignment"),
        (true, false) => panic!("Incompatible alignment"),
        (false, true) => panic!("Incompatible size"),
        (true, true) => (),
    }
}
const _: () = assert_compatible_size_align::<u8, u32>();

/// Check that size and alignment of type `Max` is greater than size and alignment of type `Min` at compile time.
#[must_use]
#[inline(always)]
pub const fn incompatible_size_align_condition<Max, Min>() -> bool {
    const {
        size_of::<Max>() > size_of::<Min>()
        || align_of::<Max>() > align_of::<Min>()
    }
}

/// Assert that size and alignment of type `Max` is greater than size and alignment of type `Min` at compile time.
/// 
/// # Usage
/// ```rust
/// #[repr(C)]
/// struct Fred {
///     foo: u32,
///     bar: u64,
///     baz: bool,
/// }
/// const _: () = isit::assert_incompatible_size_align::<Fred, u32>();
/// ```
#[track_caller]
#[inline(always)]
pub const fn assert_incompatible_size_align<Max, Min>() {
    const {
        let size_incompatible = incompatible_size_condition::<Max, Min>();
        let align_incompatible = incompatible_align_condition::<Max, Min>();
        match (size_incompatible, align_incompatible) {
            (true, true) => (),
            (true, false) => panic!("Alignment is compatible"),
            (false, true) => panic!("Size is compatible"),
            (false, false) => panic!("Size and alignment are compatible"),
        }
    }
}
const _: () = assert_incompatible_size_align::<u64, u32>();

/// Check that type `T` has the same size as a pointer at compile time.
#[must_use]
#[inline(always)]
pub const fn pointer_size_condition<T>() -> bool {
    same_size_condition::<T, *const u8>()
}

/// Assert that type `T` has the same size as a pointer at compile time.
/// 
/// # Usage
/// ```rust
/// use std::ptr::NonNull;
/// #[repr(transparent)]
/// struct Foo {
///     ptr: NonNull<u8>,
/// }
/// const _: () = isit::assert_pointer_size::<Foo>();
/// ```
#[track_caller]
#[inline(always)]
pub const fn assert_pointer_size<T>() {
    assert_same_size::<T, *const u8>();
}
#[cfg(target_pointer_width = "64")]
const _: () = assert_pointer_size::<[u8; 8]>();
#[cfg(target_pointer_width = "32")]
const _: () = assert_pointer_size::<[u8; 4]>();
#[cfg(target_pointer_width = "16")]
const _: () = assert_pointer_size::<[u8; 2]>();

/// Check that type `T` is not the same size as a pointer at compile time.
#[must_use]
#[inline(always)]
pub const fn not_pointer_size_condition<T>() -> bool {
    different_size_condition::<T, *const u8>()
}

/// Assert that type `T` is not the same size as a pointer at compile time.
/// 
/// # Usage
/// ```rust
/// #[repr(C)]
/// struct Foo {
///     a: bool,
///     b: u16,
///     c: bool,
///     d: u16,
///     e: bool,
/// }
/// const _: () = isit::assert_not_pointer_size::<Foo>();
/// ```
#[track_caller]
#[inline(always)]
pub const fn assert_not_pointer_size<T>() {
    assert_different_size::<T, *const u8>();
}
const _: () = assert_not_pointer_size::<u8>();

/// Check that type `T` has the same alignment as a pointer at compile time.
#[must_use]
#[inline(always)]
pub const fn pointer_align_condition<T>() -> bool {
    same_align_condition::<T, *const u8>()
}

/// Assert that type `T` has the same alignment as a pointer at compile time.
/// 
/// # Usage
/// ```rust
/// #[cfg_attr(target_pointer_width = "64", repr(C, align(8)))]
/// #[cfg_attr(target_pointer_width = "32", repr(C, align(4)))]
/// #[cfg_attr(target_pointer_width = "16", repr(C, align(2)))]
/// struct Foo(usize);
/// const _: () = isit::assert_pointer_align::<Foo>();
/// ```
#[track_caller]
#[inline(always)]
pub const fn assert_pointer_align<T>() {
    assert_same_size_align::<T, *const u8>();
}
#[cfg(target_pointer_width = "64")]
const _: () = assert_pointer_align::<u64>();
#[cfg(target_pointer_width = "32")]
const _: () = assert_pointer_align::<u32>();
#[cfg(target_pointer_width = "16")]
const _: () = assert_pointer_align::<u16>();

/// Check that type `T` is does not have the same alignment as a pointer at compile time.
#[must_use]
#[inline(always)]
pub const fn not_pointer_align_condition<T>() -> bool {
    different_align_condition::<T, *const u8>()
}

/// Assert that type `T` is does not have the same alignment as a pointer at compile time.
/// 
/// # Usage
/// ```rust
/// #[repr(transparent)]
/// struct Foo {
///     inner: u8,
/// }
/// const _: () = isit::assert_not_pointer_align::<Foo>();
/// ```
#[track_caller]
#[inline(always)]
pub const fn assert_not_pointer_align<T>() {
    assert_different_align::<T, *const u8>()
}
const _: () = assert_not_pointer_align::<[u8; 8]>();

/// Check that type `T` has the same size and alignment as a pointer at compile time.
#[must_use]
#[inline(always)]
pub const fn pointer_size_align_condition<T>() -> bool {
    same_size_align_condition::<T, *const u8>()
}

/// Assert that type `T` has the same size and alignment as a pointer at compile time.
/// 
/// # Usage
/// ```rust
/// use std::ptr::NonNull;
/// 
/// #[repr(transparent)]
/// struct Foo(NonNull<Foo>);
/// const _: () = isit::assert_pointer_size_align::<Foo>();
/// ```
#[track_caller]
#[inline(always)]
pub const fn assert_pointer_size_align<T>() {
    assert_same_size_align::<T, *const u8>()
}
#[cfg(target_pointer_width = "64")]
const _: () = assert_pointer_size_align::<u64>();
#[cfg(target_pointer_width = "32")]
const _: () = assert_pointer_size_align::<u32>();
#[cfg(target_pointer_width = "16")]
const _: () = assert_pointer_size_align::<u16>();

/// Check that type `T` has a different size and alignment from a pointer at compile time.
#[must_use]
#[inline(always)]
pub const fn not_pointer_size_align_condition<T>() -> bool {
    different_size_align_condition::<T, *const u8>()
}

/// Assert that type `T` has a different size and alignment from a pointer at compile time.
/// 
/// # Usage
/// ```rust
/// #[repr(transparent)]
/// struct Foo(u8);
/// const _: () = isit::assert_not_pointer_size_align::<Foo>();
/// ```
#[track_caller]
#[inline(always)]
pub const fn assert_not_pointer_size_align<T>() {
    assert_different_size_align::<T, *const u8>();
}
const _: () = assert_not_pointer_size_align::<[u8; 7]>();

/// Check that type `T` has a size and alignment that is less than or equal to the size and alignment of a pointer at compile time.
#[must_use]
#[inline(always)]
pub const fn pointer_compatible_size_align_condition<T>() -> bool {
    compatible_size_align_condition::<T, *const u8>()
}

/// Assert that type `T` has a size and alignment that is less than or equal to the size and alignment of a pointer at compile time.
/// 
/// # Usage
/// ```rust
/// #[repr(transparent)]
/// struct Foo(u8);
/// const _: () = isit::assert_pointer_compatible_size_align::<Foo>();
/// ```
#[track_caller]
#[inline(always)]
pub const fn assert_pointer_compatible_size_align<T>() {
    assert_compatible_size_align::<T, *const u8>();
}
const _: () = assert_pointer_compatible_size_align::<u8>();

/// Check that the size or alignment of type `T` is greater than the size or alignment of a pointer at compile time.
#[must_use]
#[inline(always)]
pub const fn pointer_incompatible_size_align_condition<T>() -> bool {
    incompatible_size_align_condition::<T, *const u8>()
}

/// Assert that the size or alignment of type `T` is greater than the size or alignment of a pointer at compile time.
/// 
/// # Usage
/// ```rust
/// #[repr(C, align(16))]
/// struct Fred {
///     foo: u32,
///     bar: u64,
///     baz: bool,
/// }
/// const _: () = isit::assert_pointer_incompatible_size_align::<Fred>();
/// ```
#[track_caller]
#[inline(always)]
pub const fn assert_pointer_incompatible_size_align<T>() {
    assert_incompatible_size_align::<T, *const u8>();
}

/// Checks that type `T` has the same size and alignment as `Option<Niched>` at compile time.
#[must_use]
#[inline(always)]
pub const fn t_niche_condition<T, Niched>() -> bool {
    same_size_align_condition::<T, Option<Niched>>()
}

/// Assert that type `T` has the same size and alignment as `Option<Niched>` at compile time.
/// 
/// # Usage
/// ```rust
/// use std::ptr::NonNull;
/// 
/// #[repr(transparent)]
/// struct Foo {
///     ptr: NonNull<Foo>,
/// }
/// const _: () = isit::assert_t_niche::<Foo, NonNull<Foo>>();
/// ```
#[track_caller]
#[inline(always)]
pub const fn assert_t_niche<T, Niched>() {
    assert_same_size_align::<T, Option<Niched>>();
}
const _: () = assert_t_niche::<u8, NonZero<u8>>();

/// Check that `Option<Niched>` has a greater size than type `T` at compile time.
#[must_use]
#[inline(always)]
pub const fn no_t_niche_condition<T, Niched>() -> bool {
    incompatible_size_condition::<Option<Niched>, T>()
}

/// Check that `Option<Niched>` has a greater size than type `T` at compile time.
/// 
/// # Usage
/// ```rust
/// #[repr(transparent)]
/// struct Foo {
///     foo: u32,
/// }
/// const _: () = isit::assert_no_t_niche::<Foo, u32>();
/// ```
#[track_caller]
#[inline(always)]
pub const fn assert_no_t_niche<T, Niched>() {
    assert_incompatible_size::<Option<Niched>, T>();
}
const _: () = assert_no_t_niche::<u8, u8>();

/// Check that the size and alignment of `T` is less than or equal to the size and alignment of `Option<Niched>` at compile time.
#[must_use]
#[inline(always)]
pub const fn t_niche_compatible_condition<T, Niched>() -> bool {
    compatible_size_align_condition::<T, Option<Niched>>()
}

/// Assert that the size and alignment of `T` is less than or equal to the size and alignment of `Option<Niched>` at compile time.
/// 
/// # Usage
/// ```rust
/// use std::num::NonZero;
/// #[repr(transparent)]
/// struct Foo(NonZero<u8>);
/// const _: () = isit::assert_t_niche_compatible::<Foo, NonZero<u8>>();
/// const _: () = isit::assert_t_niche_compatible::<Foo, NonZero<u16>>();
/// ```
#[track_caller]
#[inline(always)]
pub const fn assert_t_niche_compatible<T, Niched>() {
    assert_compatible_size_align::<T, Option<Niched>>();
}
const _: () = assert_t_niche_compatible::<u8, NonZero<u16>>();

/// Check that type `T` has the same size and alignment as `Option<T>` at compile time.
#[must_use]
#[inline(always)]
pub const fn niche_condition<T>() -> bool {
    t_niche_condition::<T, T>()
}

/// Assert that type `T` has the same size and alignment as `Option<T>` at compile time.
/// 
/// # Usage
/// ```rust
/// use std::ptr::NonNull;
/// 
/// #[repr(transparent)]
/// struct Foo(NonNull<Foo>);
/// const _: () = isit::assert_niche::<Foo>();
/// ```
#[track_caller]
#[inline(always)]
pub const fn assert_niche<T>() {
    assert_t_niche::<T, T>();
}
const _: () = assert_niche::<NonZero<u32>>();

/// Check that type `T` has a different size from `Option<T>` at compile time.
#[must_use]
#[inline(always)]
pub const fn no_niche_condition<T>() -> bool {
    different_size_condition::<T, Option<T>>()
}

/// Assert that type `T` has a different size from `Option<T>` at compile time.
/// 
/// # Usage
/// ```rust
/// #[repr(transparent)]
/// struct Foo(u64);
/// const _: () = isit::assert_no_niche::<Foo>();
/// ```
#[track_caller]
#[inline(always)]
pub const fn assert_no_niche<T>() {
    assert_different_size::<T, Option<T>>();
}
const _: () = assert_no_niche::<usize>();

/// Check that type `T` has the same size and alignment as `Option<T>`, but has a different size than `Option<Option<T>>` at compile time.
#[must_use]
#[inline(always)]
pub const fn single_niche_condition<T>() -> bool {
    niche_condition::<T>() && no_niche_condition::<Option<T>>()
}

/// Assert that type `T` has the same size and alignment as `Option<T>`, but has a different size than `Option<Option<T>>` at compile time.
/// 
/// # Usage
/// ```rust
/// use std::ptr::NonNull;
/// 
/// #[repr(transparent)]
/// struct Foo(NonNull<Foo>);
/// const _: () = isit::assert_single_niche::<Foo>();
/// ```
#[track_caller]
#[inline(always)]
pub const fn assert_single_niche<T>() {
    let has_niche = niche_condition::<T>();
    let has_less2 = no_niche_condition::<Option<T>>();
    match (has_niche, has_less2) {
        (true, true) => (),
        (true, false) => panic!("Has more than one niche."),
        (false, true) => panic!("Does not have at least one niche."),
        // Not possible for a type to simultaneously have
        // no niche and more than one niche.
        (false, false) => panic!("Both doesn't have a niche and has two niches. Somehow."),
    }
}
const _: () = assert_single_niche::<std::num::NonZero<u64>>();


/// Check that `Option<T>` has the same size and alignment as a pointer at compile time.
#[must_use]
#[inline(always)]
pub const fn pointer_niche_condition<T>() -> bool {
    t_niche_condition::<*const u8, T>()
}

/// Assert that `Option<T>` has the same size and alignment as a pointer at compile time.
/// 
/// # Usage
/// ```rust
/// use std::ptr::NonNull;
/// 
/// #[repr(transparent)]
/// struct Foo(NonNull<Foo>);
/// const _: () = isit::assert_pointer_niche::<Foo>();
/// ```
#[track_caller]
#[inline(always)]
pub const fn assert_pointer_niche<T>() {
    assert_t_niche::<*const u8, T>();
}
const _: () = assert_pointer_niche::<NonNull<()>>();

/// Check that `Option<T>` has a size greater than a pointer at compile time.
#[must_use]
#[inline(always)]
pub const fn no_pointer_niche_condition<T>() -> bool {
    no_t_niche_condition::<*const u8, T>()
}

/// Assert that `Option<T>` has a size greater than a pointer at compile time.
/// 
/// # Usage
/// ```rust
/// #[repr(transparent)]
/// struct Foo(usize);
/// const _: () = isit::assert_no_pointer_niche::<Foo>();
/// ```
#[track_caller]
#[inline(always)]
pub const fn assert_no_pointer_niche<T>() {
    assert_no_t_niche::<*const u8, T>();
}
const _: () = assert_no_pointer_niche::<usize>();

/// Check that the size and alignment of `Option<T>` is less or equal to the size and alignment of a pointer at compile time.
#[must_use]
#[inline(always)]
pub const fn pointer_niche_compatible_condition<T>() -> bool {
    t_niche_compatible_condition::<*const u8, T>()
}

/// Assert that the size and alignment of `Option<T>` is greater or equal to the size and alignment of a pointer at compile time.
/// 
/// # Usage
/// ```rust
/// use std::num::NonZero;
/// #[repr(transparent)]
/// struct Foo(NonZero<u128>);
/// const _: () = isit::assert_pointer_niche_compatible::<Foo>();
/// ```
#[track_caller]
#[inline(always)]
pub const fn assert_pointer_niche_compatible<T>() {
    assert_t_niche_compatible::<usize, T>();
}
const _: () = assert_pointer_niche_compatible::<NonZero<u128>>();

/// Check that the size of `Option<T>` is equal to the size of `u8` at compile time.
#[must_use]
#[inline(always)]
pub const fn u8_niche_condition<T>() -> bool {
    t_niche_condition::<u8, T>()
}

/// Assert that the size of `Option<T>` is equal to the size of `u8` at compile time.
/// 
/// # Usage
/// ```rust
/// #[repr(u8)]
/// enum Fred {
///     Foo,
///     Bar,
///     Baz,
/// }
/// const _: () = isit::assert_u8_niche::<Fred>();
/// ```
#[track_caller]
#[inline(always)]
pub const fn assert_u8_niche<T>() {
    assert_t_niche::<u8, T>();
}
const _: () = assert_u8_niche::<NonZero<u8>>();

/// Check that type `T` has the same size and alignment as a `u8` at compile time.
#[must_use]
#[inline(always)]
pub const fn u8_size_align_condition<T>() -> bool {
    same_size_align_condition::<T, u8>()
}

/// Assert that type `T` has the same size and alignment as a `u8` at compile time.
/// 
/// # Usage
/// ```rust
/// #[repr(transparent)]
/// struct Foo(u8);
/// const _: () = isit::assert_u8_size_align::<Foo>();
/// ```
#[track_caller]
#[inline(always)]
pub const fn assert_u8_size_align<T>() {
    assert_same_size_align::<T, u8>();
}
const _: () = assert_u8_size_align::<bool>();

/// Check that type `T` has a size and alignment that is less or equal to the size and alignment of `u8` at compile time.
#[must_use]
#[inline(always)]
pub const fn u8_compatible_condition<T>() -> bool {
    compatible_size_align_condition::<T, u8>()
}

/// Assert that type `T` has a size and alignment that is less or equal to the size and alignment of `u8` at compile time.
/// 
/// # Usage
/// ```rust
/// struct Zst;
/// #[repr(transparent)]
/// struct U8(u8);
/// const _: () = isit::assert_u8_compatible::<Zst>();
/// const _: () = isit::assert_u8_compatible::<U8>();
/// ```
#[track_caller]
#[inline(always)]
pub const fn assert_u8_compatible<T>() {
    assert_compatible_size_align::<T, u8>()
}

/// Check that the size or alignment of `T` is greater than the size or alignment of `u8` at compile time.
#[must_use]
#[inline(always)]
pub const fn u8_incompatible_condition<T>() -> bool {
    incompatible_size_align_condition::<T, u8>()
}

/// Assert that the size or alignment of `T` is greater than the size or alignment of `u8` at compile time.
/// 
/// # Usage
/// ```rust
/// #[repr(transparent)]
/// struct Foo(u16);
/// const _: () = isit::assert_u8_incompatible::<Foo>();
/// ```
#[track_caller]
#[inline(always)]
pub const fn assert_u8_incompatible<T>() {
    assert_incompatible_size_align::<T, u8>();
}
const _: () = assert_u8_incompatible::<u16>();

/// Check that type `T` is a ZST (zero-sized type) at compile time.
#[must_use]
#[inline(always)]
pub const fn zst_condition<T>() -> bool {
    size_condition::<T, 0>()
}

/// Assert that type `T` is a ZST (zero-sized type) at compile time.
/// 
/// # Usage
/// ```rust
/// struct Zst;
/// const _: () = isit::assert_zst::<Zst>();
/// const _: () = isit::assert_zst::<()>();
/// const _: () = isit::assert_zst::<[[u64; u16::MAX as usize]; 0]>();
/// ```
#[track_caller]
#[inline(always)]
pub const fn assert_zst<T>() {
    assert_size::<T, 0>()
}
const _: () = assert_zst::<()>();

/// Check that type `T` is not a ZST (zero-sized type) at compile time.
#[must_use]
#[inline(always)]
pub const fn not_zst_condition<T>() -> bool {
    const { !size_condition::<T, 0>() }
}

/// Assert that type `T` is not a ZST (zero-sized type) at compile time.
/// 
/// # Usage
/// ```rust
/// struct NonZst([u64; u16::MAX as usize]);
/// const _: () = isit::assert_not_zst::<NonZst>();
/// ```
#[track_caller]
#[inline(always)]
pub const fn assert_not_zst<T>() {
    assert_not_size::<T, 0>();
}

/// Check that type `T` is an uninhabited ZST (zero-sized type) at compile time.
/// 
/// https://doc.rust-lang.org/stable/reference/glossary.html?highlight=uninhabited#r-glossary.uninhabited
#[must_use]
#[inline(always)]
pub const fn uninhabited_zst_condition<T>() -> bool {
    zst_condition::<T>() && u8_niche_condition::<Niche255<T>>()
}

/// Assert that type `T` is an uninhabited ZST (zero-sized type) at compile time.
/// 
/// https://doc.rust-lang.org/stable/reference/glossary.html?highlight=uninhabited#r-glossary.uninhabited
/// 
/// # Usage
/// ```rust
/// enum Uninhabited {}
/// struct Holder<T>(T);
/// const _: () = isit::assert_uninhabited_zst::<Uninhabited>();
/// const _: () = isit::assert_uninhabited_zst::<(Uninhabited, Uninhabited)>();
/// const _: () = isit::assert_uninhabited_zst::<[Uninhabited; usize::MAX]>();
/// const _: () = isit::assert_uninhabited_zst::<Holder<Uninhabited>>();
/// const _: () = isit::assert_uninhabited_zst::<Holder<[Uninhabited; usize::MAX]>>();
/// const _: () = isit::assert_uninhabited_zst::<Holder<[[Uninhabited; usize::MAX]; usize::MAX]>>();
/// const _: () = isit::assert_uninhabited_zst::<Holder<[[[Uninhabited; usize::MAX]; usize::MAX]; usize::MAX]>>();
/// const _: () = isit::assert_uninhabited_zst::<Holder<(
///     Holder<[[[Uninhabited; usize::MAX]; usize::MAX]; usize::MAX]>,
///     Holder<[[[Uninhabited; usize::MAX]; usize::MAX]; usize::MAX]>,
/// )>>();
/// ```
#[track_caller]
#[inline(always)]
pub const fn assert_uninhabited_zst<T>() {
    _=zst_condition::<T>() || panic!("Not a zst.");
    _=u8_niche_condition::<Niche255<T>>() || panic!("Not impossible to create");
}
const _: () = assert_uninhabited_zst::<UninhabitedZst>();

/// Check that type `T` is not an uninhabited ZST (zero-sized type) at compile time.
/// 
/// https://doc.rust-lang.org/stable/reference/glossary.html?highlight=uninhabited#r-glossary.uninhabited
#[must_use]
#[inline(always)]
pub const fn not_uninhabited_zst_condition<T>() -> bool {
    !zst_condition::<T>() || !u8_niche_condition::<Niche255<T>>()
}

/// Assert that type `T` is not an uninhabited ZST (zero-sized type) at compile time.
/// 
/// https://doc.rust-lang.org/stable/reference/glossary.html?highlight=uninhabited#r-glossary.uninhabited
/// 
/// # Usage
/// ```rust
/// struct Foo(u8);
/// const _: () = isit::assert_not_uninhabited_zst::<Foo>();
/// ```
#[track_caller]
#[inline(always)]
pub const fn assert_not_uninhabited_zst<T>() {
    _ = not_uninhabited_zst_condition::<T>() || panic!("Type is an uninhabited ZST.");
}

// TODO: When const traits become available, add assert_same_type.
