/// Without this alias, users have to manually enable
/// `#![feature(termination_trait_lib)]`
pub trait Termination = std::process::Termination;