#![feature(termination_trait_lib, trait_alias)]
#![doc = include_str!("../README.md")]

pub use fncmd_impl::fncmd;

mod exit_code;
#[doc(hidden)]
pub use exit_code::{ExitCode, IntoExitCode};

#[doc(hidden)]
pub use clap;