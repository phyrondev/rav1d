//! The C types and `errno` values that `rav1d` needs.
//!
//! This is the only place in the library that names the `libc` crate. On
//! targets with `libc` these items are re-exported from it, so the types and
//! values are exactly the platform C ones. `wasm32-unknown-unknown` has no
//! `libc` port, so there they come from a small pure-Rust shim instead,
//! which is also why `libc` is only a dependency for other targets.

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
mod wasm32;
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub(crate) use libc::*;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub(crate) use self::wasm32::*;
