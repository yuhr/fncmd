#![feature(termination_trait_lib, process_exitcode_placeholder, trait_alias)]
#![doc = include_str!("../README.md")]

pub use fncmd_impl::fncmd;

mod exit_code;

#[doc(hidden)]
pub use exit_code::ExitCode;
#[doc(hidden)]
pub use exit_code::IntoExitCode;

mod termination;

#[doc(hidden)]
pub use termination::Termination;

#[doc(hidden)]
pub use clap;