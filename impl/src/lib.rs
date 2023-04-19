#![feature(proc_macro_span)]

use darling::FromMeta;
use proc_macro::{Span, TokenStream};
use proc_macro_error::proc_macro_error;
use syn::{parse_macro_input, AttributeArgs, ItemFn};

mod models;
use models::*;
mod context;
use context::*;

/// The `fncmd` macro. This can only be applied to the `main` function.
///
/// ```rust
/// /// Description of the command line tool
/// #[fncmd::fncmd]
/// pub fn main(
///   /// Argument foo
///   #[opt(short, long)]
///   foo: String,
///   /// Argument bar
///   #[opt(short, long)]
///   bar: Option<String>,
/// ) {
///   println!("{:?} {:?}", foo, bar);
/// }
/// ```
#[proc_macro_error]
#[proc_macro_attribute]
pub fn fncmd(attr: TokenStream, item: TokenStream) -> TokenStream {
	// Get information about the target and package.
	let call_site = Span::call_site();
	let (target, package) = CONTEXT.get_target_and_package_of(&call_site);
	let name = target.name.clone();
	let version = package.version.to_string();
	let subcmds = FncmdSubcmds::from((target, package));

	// Parse the input tokens.
	let attr = parse_macro_input!(attr as AttributeArgs);
	let attr = FncmdAttr::from_list(&attr).unwrap();
	let item = parse_macro_input!(item as ItemFn);
	Fncmd::parse(name, version, attr, item, subcmds).into()
}