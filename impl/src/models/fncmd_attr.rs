use darling::FromMeta;

#[derive(Debug, Default, FromMeta)]
#[darling(default)]
pub struct FncmdAttr {
	// Empty for now, but it's here for future use.
}