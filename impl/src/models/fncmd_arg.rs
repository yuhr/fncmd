use proc_macro2::TokenStream;
use proc_macro_error::{abort, emit_warning};
use quote::{quote, quote_spanned, ToTokens};
use syn::{parse_quote, spanned::Spanned, token::Mut, Attribute, FnArg, Ident, Pat, Type};

#[allow(dead_code)]
pub struct FncmdArg {
	pub name: Ident,
	pub documentation: Option<TokenStream>,
	pub attributes: Vec<Attribute>,
	pub ty: Type,
	pub mutability: Option<Mut>,
}

impl FncmdArg {
	pub fn parse(fnarg: &FnArg) -> Self {
		let arg = if let FnArg::Typed(arg) = fnarg {
			arg
		} else {
			abort!(fnarg.span(), "`self` receiver is invalid here");
		};

		let mut doc: Option<TokenStream> = None;
		let mut attrs = Vec::new();

		for attr in arg.attrs.iter() {
			match attr.path.get_ident().map(|ident| ident.to_string()).as_deref() {
				Some("doc") => {
					doc = Some(quote! { #attr });
				}
				Some("opt") => {
					let mut attr = attr.clone();
					let mut tokens = attr.tokens.clone();

					// If no configuration was provided, enable `long` by default.
					if attr.tokens.into_iter().count() == 0 {
						tokens.extend(quote!((long)))
					}
					attr.tokens = tokens;

					// Change `opt` into `clap`.
					let spanned_clap = quote_spanned!(attr.path.get_ident().span() => clap);
					attr.path = parse_quote! { #spanned_clap };

					attrs.push(attr);
				}
				Some(_) => {
					emit_warning!(
						attr.span(),
						"attributes other than `doc` and `opt` have no effect here"
					);
				}
				None => unreachable!(),
			}
		}

		let pat = arg.pat.as_ref();
		if let Pat::Ident(pat_ident) = pat {
			assert!(pat_ident.attrs.is_empty());
			assert!(pat_ident.by_ref.is_none());
			assert!(pat_ident.subpat.is_none());

			FncmdArg {
				name: pat_ident.ident.clone(),
				documentation: doc,
				attributes: attrs,
				ty: arg.ty.as_ref().clone(),
				mutability: pat_ident.mutability,
			}
		} else {
			abort!(pat.span(), "pattern types other than a simple `Ident` are not supported yet");
		}
	}
}

impl ToTokens for FncmdArg {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let Self { name, documentation, attributes, ty, .. } = self;
		quote! {
			#documentation
			#(#attributes)*
			#name: #ty
		}
		.to_tokens(tokens);
	}
}