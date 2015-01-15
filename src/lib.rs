//!PhysFS bindings for Rust
#![crate_name = "physfs"]
#![crate_type = "lib"]
#![license = "zlib"]

#![deny(missing_docs)]
#![allow(dead_code)]
#![feature(unsafe_destructor)]
extern crate libc;

pub use physfs::*;
pub use physfs::file::*;

///PhysFS bindings
mod physfs;
///Definitions for the PhysFS primitives
mod primitives;