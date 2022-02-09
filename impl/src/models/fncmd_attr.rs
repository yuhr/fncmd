use darling::FromMeta;

#[derive(Debug, Default, FromMeta)]
#[darling(default)]
pub struct FncmdAttr {}