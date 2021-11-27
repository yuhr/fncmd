#![doc = include_str!("../README.md")]

pub use fncmd_impl::fncmd;

mod result;

#[doc(hidden)]
pub use result::Result;

#[doc(hidden)]
pub use clap;