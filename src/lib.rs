#![feature(marker_trait_attr)]
use frunk::{Generic, HCons, HNil};
use static_assertions::*;

// `#[marker]` is needed because frunk doesn't
// implement `Generic` for primitive types.
// That's not a fundamental limitation, though.
#[marker]
pub trait FromBytes {}

impl FromBytes for u8 {}
impl FromBytes for u16 {}
impl FromBytes for f32 {} // and so on...

impl<T> FromBytes for T
where
    T: Generic,
    <T as Generic>::Repr: FromBytes
{}

// `<T as Generic>::Repr` for structs `T`,
// is `T` represented as an HList

// The first field must be `FromBytes` and
// the remaining fields must be `FromBytes`
impl<H, T> FromBytes for HCons<H, T>
where
    H: FromBytes,
    T: FromBytes,
{}

// The nullary struct is unconditionally `FromBytes`
impl FromBytes for HNil
where
{}


// Example:

#[derive(Generic)]
struct Foo<A, B, C>(A, B, C);

// All fields are `FromBytes` so `Foo` is `FromBytes`
assert_impl_all!(Foo<u8, u16, f32>: FromBytes);

// The third field isn't `FromBytes`, so `Foo` isn't `FromBytes`.
assert_not_impl_any!(Foo<u8, u16, bool>: FromBytes);
