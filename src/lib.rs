#![feature(termination_trait_lib, trait_alias)]
#![doc = include_str!("../README.md")]

pub use fncmd_impl::fncmd;

mod termination;

#[doc(hidden)]
pub use termination::Termination;

#[doc(hidden)]
pub use clap;