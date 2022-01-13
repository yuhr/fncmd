use darling::FromMeta;
use syn::{parse_str, punctuated::Punctuated, token::Comma, FnArg, ItemFn};

#[derive(Debug, Default, FromMeta)]
#[darling(default)]
pub struct FncmdAttr {
	__item_fn: Option<String>,
}

impl FncmdAttr {
	pub fn args(&self) -> Option<Punctuated<FnArg, Comma>> {
		self.__item_fn
			.as_ref()
			.and_then(|string| parse_str::<ItemFn>(string).ok())
			.map(|item| item.sig.inputs)
	}
}