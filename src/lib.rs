//! Rust bindings for [libffi](https://sourceware.org/libffi/).
//!
//! The C libffi library provides two main facilities: assembling calls
//! to functions dynamically, and creating closures that can be called
//! as ordinary C functions. In Rust, the latter means that we can turn
//! a Rust lambda (or any object implementing `Fn`/`FnMut`) into an
//! ordinary C function pointer that we can pass as a callback to C.
//!
//! The easiest way to use this library is via the
//! [`high`](high/index.html) layer module, but more flexibility (and
//! less checking) is provided by the [`middle`](middle/index.html) and
//! [`low`](low/index.html) layers.
//!
//! # Usage
//!
//! It’s [on crates.io](https://crates.io/crates/libffi), but before you
//! build it, make sure you have the dependencies installed first:
//!
//!   - An up-to-date version of C [libffi](https://sourceware.org/libffi/)
//!     Version 3.2.1 is known to work. Earlier versions, such as the
//!     versions that come with Mac OS and Fedora, are known not to; neither
//!     will the version installed by Homebrew (3.0.13).
//!
//!   - [`pkg-config`](https://www.freedesktop.org/wiki/Software/pkg-config/),
//!     which you probably already have if you’re on Linux. For Mac users,
//!     the version installed by Homebrew is up to date. (I don’t know how
//!     this works on Windows; contact me if you’d like to help figure it
//!     out.)
//!
//! Then add
//!
//! ```toml
//! [dependencies]
//! libffi = "0.3.2"
//! ```
//!
//! to your `Cargo.toml` and
//!
//! ```rust
//! extern crate libffi;
//! ```
//!
//! to your crate root.
//!
//! # Organization
//!
//! This library is organized in four layers, each of which attempts to
//! provide more safety and a simpler interface than the next layer
//! down. From top to bottom:
//!
//!   - The [`high`](high/index.html) layer provides safe(?) and
//!     automatic marshalling of Rust closures into C function pointers.
//!   - The [`middle`](middle/index.html) layer provides memory-managed
//!     abstractions for assembling calls and closures, but is unsafe
//!     because it doesn’t check argument types.
//!   - The [`low`](low/index.html) layer makes no attempts at safety,
//!     but provides a more idiomatically “Rusty” API than the underlying
//!     C library.
//!   - The [`raw`](raw/index.html) layer is a re-export of the
//!     [`libffi-sys`](https://crates.io/crates/libffi-sys) crate,
//!     a direct mapping of the C libffi library into Rust, generated by
//!     [bindgen](https://crates.io/crates/bindgen).
//!
//! It should be possible to use any layer without dipping into lower
//! layers (and it will be considered a bug to the extent that it
//! isn’t).
//!
//! # Examples
//!
//! In this example, we convert a Rust lambda containing a free variable
//! into an ordinary C code pointer. The type of `fun` below is
//! `extern "C" fn(u64, u64) -> u64`.
//!
//! ```
//! use libffi::high::Closure2;
//!
//! let x = 5u64;
//! let f = |y: u64, z: u64| x + y + z;
//!
//! let closure = Closure2::new(&f);
//! let fun     = closure.code_ptr();
//!
//! assert_eq!(18, fun(6, 7));
//! ```

#![deny(missing_docs)]

#![cfg_attr(feature = "unique", feature(unique))]

#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

#[macro_use] extern crate abort_on_panic;
extern crate libc;
extern crate libffi_sys;

/// Raw definitions imported from the C library (via bindgen).
///
/// This module is generated by bindgen and undocumented. It’s intended
/// as the basis for higher-level bindings, but you can see the [C libffi
/// documentation](http://www.atmark-techno.com/~yashi/libffi.html).
pub mod raw {
    pub use libffi_sys::*;
}

pub mod high;
pub mod middle;
pub mod low;
