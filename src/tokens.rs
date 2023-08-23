use proc_macro2::TokenTree;

use syn::{
	braced,
	parse::{Parse, ParseStream},
	Block, Expr, ExprAssign, LitStr, Token,
};

use crate::utils::peek_any;

#[derive(Debug, Clone)]
pub struct FormatExpression {
	pub left: Option<Expr>,
	pub right: Expr,
	pub traits: String,
}

impl Parse for FormatExpression {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let content;
		let block = if peek_any::<Block>(input, false).is_some() {
			braced!(content in input);
			&content
		} else {
			input
		};

		let assignment =
			if let Some((_, name)) = peek_any::<ExprAssign>(block, true) {
				Some(name)
			} else {
				None
			};
		let (left, right) = if let Some(assignment) = assignment {
			(Some(*assignment.left), *assignment.right)
		} else {
			(None, block.parse::<Expr>()?)
		};

		let mut traits: Vec<char> = vec![];

		if input.peek(Token![:]) {
			if input.peek2(LitStr) {
				_ = input.parse::<Token![:]>()?;
				let string = input.parse::<LitStr>()?.value();

				traits.push(':');

				for char in string.chars() {
					traits.push(char);
				}
			} else {
				while !(input.is_empty()
					|| input.peek(LitStr)
					|| input.peek(Token![,]) && input.peek2(LitStr))
				{
					let string = input.parse::<TokenTree>()?.to_string();

					for char in string.chars() {
						traits.push(char);
					}
				}
			}
		}
		let traits = traits.iter().collect();

		Ok(FormatExpression {
			left,
			right,
			traits,
		})
	}
}

#[derive(Debug, Clone)]
pub enum FormatPart {
	Expression(FormatExpression),
	String(LitStr),
}

#[derive(Debug, Clone)]
pub struct FormatStatement {
	pub stream: Option<Expr>,
	pub parts: Vec<FormatPart>,
}

impl Parse for FormatStatement {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let mut parts = vec![];

		while !input.is_empty() {
			if input.peek(LitStr) {
				parts.push(FormatPart::String(input.parse::<LitStr>()?));
			} else {
				parts.push(FormatPart::Expression(
					input.parse::<FormatExpression>()?,
				));
			}
			if input.peek(Token![,]) {
				input.parse::<Token![,]>()?;
			}
		}

		let stream = if let Some(FormatPart::Expression(part)) = parts.first()
		{
			Some(part.right.clone())
		} else {
			None
		};

		Ok(FormatStatement { stream, parts })
	}
}
