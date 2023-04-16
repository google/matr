# Matr

Matr (Metaprogramming with Associated Types in Rust, pronounced like "matter") is a metaprogramming library for Rust.

This is meant to be a sort of standard library for metaprogramming in Rust, providing basic types and algorithms,
but with the expectation that users of this crate will want to define their own types and meta-functions; the
meta-"type system" is designed to be extensible.

Apart from `bool` and `type`, all other meta-types defined here (i.e., the directories under `src/`) don't access
anything beyond what a user of this library could, so they also serve as examples for users of this library on how to
write their own meta-types.

It's also possible to define custom sub-traits of `Kind` with additional restrictions/capabilities, see
`EqualityComparableKind` and `Equals` for an example.

This library uses traits (notably `Expr<K>`) for meta-values, so that the Rust typechecker can detect type
errors in meta-functions (e.g. attempting to return an `Expr<Bool>` in a meta-function that should return an
`Expr<USize>`). Since traits can't have trait parameters (only type parameters, that could be constrained via
traits) each meta-type has an associated `Kind` struct (e.g. `Bool` for booleans).

At the time of writing (March 2023) this is still a work in progress, and requires a nightly version of rustc
due to the use of `#![feature(specialization)]` and `#![feature(const_trait_impl)]` (that are not yet stabilized).

This is not an officially supported Google product.

## Why not use `const fn`s for metaprogramming?

If your use case is covered by `const fn`s, it's simpler to use those instead of this.
However, `const fn`s can't compute types; if you're looking for a library to manipulate types you're in the right
place.

This library also provides a `ConstFn` metatype that allows to also have a `const fn` as the result of the
meta-computation. 

That is meant to be used in cases where you have some shared logic between the code used to compute a type
and to compute a `ConstFn`. If you only need the `ConstFn`, then this library is overkill for your use case, you
should just use plain `const fn`s instead.

## Why not use macros for metaprogramming?

If your use case is covered by macros, it's simpler to use those instead of this.
However, macros only have access to the AST and not to the full type information (which is possible using this library). 
