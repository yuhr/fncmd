#![feature(proc_macro_span)]

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use cargo_metadata::MetadataCommand;
use darling::FromMeta;
use proc_macro::{Span, TokenStream};
use proc_macro_error::proc_macro_error;
use syn::visit::Visit;
use syn::{parse_file, parse_macro_input, AttributeArgs, ItemFn};

mod models;
use models::*;

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
#[proc_macro_attribute]
#[proc_macro_error]
pub fn fncmd(attr: TokenStream, item: TokenStream) -> TokenStream {
	// Get project metadata
	let metadata = MetadataCommand::new().exec().unwrap();
	let package = metadata.root_package().unwrap();
	let bin_targets = package
		.targets
		.iter()
		.filter_map(|target| {
			target
				.kind
				.iter()
				.any(|k| k == "bin" || k == "example")
				.then(|| target)
		})
		.collect::<Vec<_>>();
	let self_version = package.version.to_string();

	// Enumerate all possible subcommands
	let subcmds: FncmdSubcmds = bin_targets
		.iter()
		// Get subcommands
		.filter_map(|bin_target| {
			// Read the file
			let mut file = File::open(&bin_target.src_path).unwrap();
			let mut content = String::new();
			file.read_to_string(&mut content).unwrap();
			// If parsing failed, just skip hereafter
			parse_file(&content).ok().and_then(|ast| {
				let mut visitor = FncmdVisitor::new();
				visitor.visit_file(&ast);
				visitor
					// Only functions are relevant here
					.functions
					.iter()
					// Find a main function that is valid as a subcommand
					.find(|function| {
						// Needs to be `main`
						function.sig.ident == "main"
						// And needs to have `#[fncmd]` attribute
							&& function
								.attrs
								.iter()
								.any(|attr| {
									// `#[fncmd::fncmd]` or `#[fncmd]`
									attr.path.segments.len() <= 2
									&& attr.path.segments.iter().all(|segment| segment.ident == "fncmd")
								})
					})
					// If none of them are valid, just skip the file
					.map(|function| {
						// Prepare to `collect` into a `HashMap`
						(
							bin_target.name.to_owned(),
							(
								matches!(function.vis, syn::Visibility::Public(_)),
								bin_target.src_path.to_owned().into_std_path_buf(),
							),
						)
					})
			})
		})
		.collect::<HashMap<_, _>>()
		.into();

	// Filter out inappropriate subcommands based on their prefixes
	let self_src_path = Span::call_site().source_file().path();
	let self_bin_name = bin_targets
		.iter()
		.find_map(|bin_target| {
			bin_target
				.src_path
				.ends_with(self_src_path.to_str().unwrap())
				.then(|| bin_target.name.clone())
		})
		.unwrap();
	let subcmds = subcmds.filter_by(&self_bin_name);

	// Parse the input tokens
	let attr = parse_macro_input!(attr as AttributeArgs);
	let attr = FncmdAttr::from_list(&attr).unwrap();
	let item = parse_macro_input!(item as ItemFn);
	Fncmd::parse(self_bin_name, self_version, attr, item, subcmds).into()
}