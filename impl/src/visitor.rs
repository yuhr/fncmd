use syn::visit::Visit;
use syn::ItemFn;

pub struct Visitor<'ast> {
	pub functions: Vec<&'ast ItemFn>,
}

impl<'ast> Visitor<'ast> {
	pub fn new() -> Self { Self { functions: Vec::new() } }

	/// Find a main function that will be handled by `fncmd`.
	pub fn get_main_fncmd(&self) -> Option<&'ast ItemFn> {
		// Only functions are relevant here.
		self.functions
			.iter()
			.find(|&&function| {
				// Needs to be `main`.
				function.sig.ident == "main"
						// And needs to have `#[fncmd]` attribute.
							&& function
								.attrs
								.iter()
								.any(|attr| {
									// Only valid form is `#[fncmd::fncmd]` or `#[fncmd]`. Renaming is not supported at this time.
									attr.path.segments.len() <= 2
									&& attr.path.segments.iter().all(|segment| segment.ident == "fncmd")
								})
			})
			.copied()
	}
}

impl<'ast> Visit<'ast> for Visitor<'ast> {
	fn visit_item_fn(&mut self, node: &'ast ItemFn) {
		self.functions.push(node);
		// We don't need to visit nested functions.
		// visit::visit_item_fn(self, node);
	}
}