#![doc = include_str!("../README.md")]
#![feature(box_into_inner)]

pub use fncmd_impl::fncmd;

mod result;

#[doc(hidden)]
pub use result::Result;

#[doc(hidden)]
pub use clap;