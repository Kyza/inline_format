use proc_macro2::{TokenStream as TokenStream2, TokenTree};

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
	pub traits: TokenStream2,
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

		let mut traits: Vec<TokenTree> = vec![];

		while !input.is_empty() && !input.peek(LitStr) {
			traits.push(input.parse::<TokenTree>()?);
		}

		Ok(FormatExpression {
			left,
			right,
			traits: TokenStream2::from_iter(traits),
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
		let stream = {
			if let Some((finput, stream)) = peek_any::<Expr>(input, false) {
				if finput.peek(Token![,]) {
					input.parse::<Expr>()?;
					input.parse::<Token![,]>()?;
					Some(stream)
				} else {
					None
				}
			} else {
				None
			}
		};

		let mut parts = vec![];

		while !input.is_empty() {
			if input.peek(LitStr) {
				parts.push(FormatPart::String(input.parse::<LitStr>()?));
			} else {
				parts.push(FormatPart::Expression(
					input.parse::<FormatExpression>()?,
				));
			}
		}

		Ok(FormatStatement { stream, parts })
	}
}
