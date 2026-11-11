//! The `libc` items `rav1d` uses, for `wasm32-unknown-unknown`.
//!
//! That target has no `libc` port, and `rav1d` only needs a handful of C types
//! and `errno` values, none of which require a symbol from a C library, so
//! they are defined here in plain Rust. Nothing is declared `extern`, so this
//! shim does not have to be linked against anything.
//!
//! The `errno` values are the Linux ones, i.e. the values `rav1d` has today on
//! every target it supports. They are the discriminants of `Rav1dError` and
//! are negated for the C API's `DAV1D_ERR(E*)` results, so keeping them
//! identical means error codes do not change on this target.

#![allow(non_camel_case_types)]

use core::ffi::c_int;

/// C `ptrdiff_t`: a signed pointer-width integer.
pub(crate) type ptrdiff_t = isize;

/// C `intptr_t`: a signed pointer-width integer.
pub(crate) type intptr_t = isize;

/// C `uintptr_t`: an unsigned pointer-width integer.
pub(crate) type uintptr_t = usize;

/// C `off_t`: 64-bit, as on Linux with `_FILE_OFFSET_BITS=64`.
///
/// Used only for the `offset` field of `Dav1dDataProps` and `Rav1dDataProps`;
/// this target does no file I/O.
pub(crate) type off_t = i64;

pub(crate) const ENOENT: c_int = 2;
pub(crate) const EIO: c_int = 5;
pub(crate) const EAGAIN: c_int = 11;
pub(crate) const ENOMEM: c_int = 12;
pub(crate) const EINVAL: c_int = 22;
pub(crate) const ERANGE: c_int = 34;
pub(crate) const ENOPROTOOPT: c_int = 92;
