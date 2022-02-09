#![feature(termination_trait_lib, trait_alias)]
#![doc = include_str!("../README.md")]

pub use fncmd_impl::fncmd;

#[doc(hidden)]
pub use fncmd_impl::__inject_params;

mod exit_code;
#[doc(hidden)]
pub use exit_code::{ExitCode, IntoExitCode};

#[doc(hidden)]
pub use clap;

#[doc(hidden)]
pub use once_cell::sync::Lazy;