use cargo_metadata::{Metadata, MetadataCommand, Package, Target};
use once_cell::sync::{Lazy, OnceCell};
use proc_macro::Span;

pub static CONTEXT: Lazy<Context> = Lazy::new(|| {
	let metadata = MetadataCommand::new().exec().unwrap();
	Context { metadata }
});

pub struct Context {
	pub metadata: Metadata,
}

impl Context {
	/// Enumerate packages in the workspace, including the root package if exists.
	pub fn packages(&'static self) -> &'static [&'static Package] {
		static PACKAGES: OnceCell<Vec<&'static Package>> = OnceCell::new();
		PACKAGES
			.get_or_init(|| {
				self.metadata
					.root_package()
					.into_iter()
					.chain(self.metadata.workspace_packages())
					.collect::<Vec<_>>()
			})
			.as_slice()
	}

	/// Get the target and package that the given span belongs to.
	pub fn get_target_and_package_of(
		&'static self,
		span: &Span,
	) -> (&'static Target, &'static Package) {
		let source_path = span.source_file().path();
		for &package in self.packages() {
			for target in &package.targets {
				if target.src_path.ends_with(source_path.to_str().unwrap()) {
					return (target, package);
				}
			}
		}
		unreachable!()
	}
}