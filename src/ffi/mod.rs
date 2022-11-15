#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, improper_ctypes, clippy::module_inception)]
mod ffi;
pub mod constants;
pub mod events;
pub use ffi::*;

#[cfg(feature = "shm")]
pub mod shm;
#[cfg(feature = "shm")]
pub use shm::*;

#[cfg(feature = "xrandr")]
pub mod xrandr;
