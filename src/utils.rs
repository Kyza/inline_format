use std::collections::HashSet;

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use proc_macro_error::abort;
use quote::{quote, ToTokens};
use syn::{
	parse::{Parse, ParseBuffer, ParseStream},
	parse_macro_input, Ident,
};

use crate::tokens::{FormatPart, FormatStatement};

pub fn peek_any<T: Parse>(
	input: ParseStream,
	parse_original: bool,
) -> Option<(ParseBuffer, T)> {
	let finput = input.fork();
	if let Ok(any) = finput.parse::<T>() {
		if parse_original {
			_ = input.parse::<T>();
		}

		Some((finput, any))
	} else {
		None
	}
}

pub fn ident_from_left(left: &TokenStream2) -> syn::Result<Ident> {
	syn::parse_str::<Ident>(
		&left
			.clone()
			.into_iter()
			.next()
			.to_token_stream()
			.to_string(),
	)
}

pub fn make_format_string(
	macro_name: &str,
	input: TokenStream,
	uses_stream: bool,
	clone_named: bool,
) -> TokenStream {
	let mut statement = parse_macro_input!(input as FormatStatement);

	let mut names: HashSet<Ident> = HashSet::new();
	let mut defined_names: HashSet<Ident> = HashSet::new();

	let mut string: String = String::new();
	let mut assignments: Vec<TokenStream2> = vec![];
	let mut args: Vec<TokenStream2> = vec![];

	let stream = if uses_stream {
		if let Some(first) = statement.stream {
			statement.parts.remove(0);
			Some(quote! { #first, })
		} else {
			None
		}
	} else {
		None
	};

	for part in &statement.parts {
		if let FormatPart::Expression(expression_part) = part {
			if let Some(left) = &expression_part.left {
				let name = ident_from_left(&left.to_token_stream());

				if name.is_err() {
					abort! { left,
						format!("failed to get the name from an assignment \"{}\"'s left", left.to_token_stream().to_string());
						help = "I don't know how you got here, or how this could\
						 possibly happen, but apparently it can. So I'm writing\
						 this message to say that you're doing something wrong\
						 here... Maybe... Or maybe I am. I have no idea.";
					}
				}

				names.insert(name.unwrap());
			}
		}
	}

	for part in statement.parts {
		match part {
			FormatPart::String(string_part) => {
				string.push_str(
					&string_part
						.value()
						.replace('{', "{{")
						.replace('}', "}}"),
				);
			}
			FormatPart::Expression(expression_part) => {
				string.push_str(&format!(
					"{{{}}}",
					expression_part.traits.replace(' ', "")
				));
				if let Some(left) = expression_part.left {
					let name = ident_from_left(&left.to_token_stream());

					if name.is_err() {
						abort! { left,
							format!("failed to get the name from a named parameter \"{}\"'s left", left.to_token_stream().to_string());
							help = "The named parameter should be an identifier.";
						}
					}
					defined_names.insert(name.clone().unwrap());

					let right = expression_part.right;
					assignments.push(quote! {
						let #left = #right;
					});

					if clone_named {
						args.push(quote! { #left.clone() });
					} else {
						args.push(left.to_token_stream());
					}
				} else {
					let name = syn::parse_str::<Ident>(
						&expression_part.right.to_token_stream().to_string(),
					);

					let arg = if let Ok(name) = name {
						if names.contains(&name) {
							// Check if it has been defined yet.
							if !defined_names.contains(&name) {
								abort! { expression_part.right,
									format!("named parameter \"{}\" was used before definition", name);
									help = "Define the named parameter the first time it's used.";
								}
							}

							quote! { #name.clone() }
						} else {
							expression_part.right.to_token_stream()
						}
					} else {
						expression_part.right.to_token_stream()
					};

					args.push(arg);
				}
			}
		}
	}

	let macro_name = Ident::new(macro_name, Span::call_site());

	quote! {
		{
			#(#assignments)*
			::std::#macro_name!(#stream #string, #(#args),*)
		}
	}
	.into()
}
