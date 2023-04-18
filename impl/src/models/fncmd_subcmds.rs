use std::{collections::HashMap, path::PathBuf};

pub struct FncmdSubcmds(pub HashMap<String, (bool, PathBuf)>);

impl FncmdSubcmds {
	pub fn filter_by(mut self, self_cmd_name: &str) -> Self {
		// Remove all targets that are not prefixed with the self
		// (This step also removes the self command itself)
		self.0.retain(|name, _| is_subcommand(name, self_cmd_name));

		// Remove all subcommands that are prefixed with any other target
		let table = self.0.clone();
		self.0.retain(|name, _| !table.iter().any(|(other, _)| is_subcommand(name, other)));

		// Remove all non-`pub` targets
		self.0.retain(|_, (is_pub, _)| *is_pub);

		self
	}

	pub fn iter(&self) -> impl Iterator<Item = (&String, &(bool, PathBuf))> { self.0.iter() }
}

/// Check if `it` is a subcommand of `of`.
fn is_subcommand(it: &str, of: &str) -> bool { it.len() > of.len() && it.starts_with(of) }

impl From<HashMap<String, (bool, PathBuf)>> for FncmdSubcmds {
	fn from(from: HashMap<String, (bool, PathBuf)>) -> Self { FncmdSubcmds(from) }
}