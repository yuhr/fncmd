#![doc = include_str!("../README.md")]

pub use fncmd_impl::fncmd;

mod exit_code;
#[doc(hidden)]
pub use exit_code::{ExitCode, IntoExitCode};

#[doc(hidden)]
pub use clap;

#[doc(hidden)]
pub use once_cell::sync::Lazy;