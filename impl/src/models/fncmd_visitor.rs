use syn::visit::Visit;
use syn::ItemFn;

pub struct FncmdVisitor<'ast> {
	pub functions: Vec<&'ast ItemFn>,
}

impl FncmdVisitor<'_> {
	pub fn new() -> Self {
		Self {
			functions: Vec::new(),
		}
	}
}

impl<'ast> Visit<'ast> for FncmdVisitor<'ast> {
	fn visit_item_fn(&mut self, node: &'ast ItemFn) {
		self.functions.push(node);
		// Commented out because we don't need to visit nested functions
		// visit::visit_item_fn(self, node);
	}
}