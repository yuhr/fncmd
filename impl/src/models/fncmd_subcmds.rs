use crate::visitor::Visitor;
use cargo_metadata::{Package, Target};
use std::fs::File;
use std::io::Read;
use std::{collections::HashMap, path::PathBuf};
use syn::parse_file;
use syn::visit::Visit;

pub struct FncmdSubcmds {
	map: HashMap<String, (bool, PathBuf)>,
}

impl FncmdSubcmds {
	pub fn iter(&self) -> impl Iterator<Item = (&String, &(bool, PathBuf))> { self.map.iter() }
}

/// Check if `it` is a subcommand of `of`.
fn is_subcommand(it: &str, of: &str) -> bool { it.len() > of.len() && it.starts_with(of) }

impl From<(&Target, &Package)> for FncmdSubcmds {
	fn from((target, package): (&Target, &Package)) -> Self {
		let name_toplevel = &target.name;

		// Enumerate targets that will become executables i.e. `bin` and `example` targets.
		let targets = package.targets.iter().filter_map(|target| {
			target.kind.iter().any(|k| k == "bin" || k == "example").then(|| target)
		});

		// Enumerate all possible subcommands.
		let mut map: HashMap<String, (bool, PathBuf)> = targets
			.filter_map(|target| {
				// Read the file.
				let content = {
					let mut file = File::open(&target.src_path).unwrap();
					let mut content = String::new();
					file.read_to_string(&mut content).unwrap();
					content
				};
				// If parsing failed, just skip hereafter.
				parse_file(&content).ok().and_then(|ast| {
					let mut visitor = Visitor::new();
					visitor.visit_file(&ast);
					visitor.get_main_fncmd().map(|function| {
						// Prepare to `collect` into a `HashMap`.
						(
							target.name.to_owned(),
							(
								matches!(function.vis, syn::Visibility::Public(_)),
								target.src_path.to_owned().into_std_path_buf(),
							),
						)
					})
				})
			})
			.collect::<HashMap<_, _>>();

		// Filter out inappropriate subcommands based on their prefixes.
		{
			// Remove all targets that are not prefixed with the toplevel name (This step also removes the toplevel target itself).
			map.retain(|name, _| is_subcommand(name, name_toplevel));

			// Remove all subcommands that are prefixed with any other target.
			let table = map.clone();
			map.retain(|name, _| {
				!table.iter().any(|(name_other, _)| is_subcommand(name, name_other))
			});

			// Remove all non-`pub` targets.
			map.retain(|_, (is_pub, _)| *is_pub);
		}

		FncmdSubcmds { map }
	}
}