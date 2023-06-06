#![doc = include_str!("../README.md")]
#![allow(clippy::tabs_in_doc_comments)]

use crate::utils::make_format_string;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

mod tokens;
mod utils;

#[proc_macro]
#[proc_macro_error]
/// Compiles to [`::std::format!()`](https://doc.rust-lang.org/std/macro.format.html).
///
/// ```rs
/// let val = 2 + 2;
/// assert_eq!(
/// 	std::format!("text {{}} {val:04} text {:o}", 10 * 10),
/// 	format!("text {} " val:04 " text " 10 * 10:o)
/// );
/// ```
#[doc = include_str!("../README.md")]
pub fn format(input: TokenStream) -> TokenStream {
	make_format_string("format", input, false, false)
}

#[proc_macro]
#[proc_macro_error]
/// Compiles to [`::std::write!()`](https://doc.rust-lang.org/std/macro.write.html).
///
/// ```rs
/// let val = 2 + 2;
/// assert_eq!(
/// 	std::format!("text {{}} {val:04} text {:o}", 10 * 10),
/// 	format!("text {} " val:04 " text " 10 * 10:o)
/// );
/// ```
#[doc = include_str!("../README.md")]
pub fn write(input: TokenStream) -> TokenStream {
	make_format_string("write", input, true, false)
}
#[proc_macro]
#[proc_macro_error]
/// Compiles to [`::std::writeln!()`](https://doc.rust-lang.org/std/macro.writeln.html).
///
/// ```rs
/// let val = 2 + 2;
/// assert_eq!(
/// 	std::format!("text {{}} {val:04} text {:o}", 10 * 10),
/// 	format!("text {} " val:04 " text " 10 * 10:o)
/// );
/// ```
#[doc = include_str!("../README.md")]
pub fn writeln(input: TokenStream) -> TokenStream {
	make_format_string("writeln", input, true, false)
}

#[proc_macro]
#[proc_macro_error]
/// Compiles to [`::std::print!()`](https://doc.rust-lang.org/std/macro.print.html).
///
/// ```rs
/// let val = 2 + 2;
/// assert_eq!(
/// 	std::format!("text {{}} {val:04} text {:o}", 10 * 10),
/// 	format!("text {} " val:04 " text " 10 * 10:o)
/// );
/// ```
#[doc = include_str!("../README.md")]
pub fn print(input: TokenStream) -> TokenStream {
	make_format_string("print", input, false, false)
}
#[proc_macro]
#[proc_macro_error]
/// Compiles to [`::std::println!()`](https://doc.rust-lang.org/std/macro.println.html).
///
/// ```rs
/// let val = 2 + 2;
/// assert_eq!(
/// 	std::format!("text {{}} {val:04} text {:o}", 10 * 10),
/// 	format!("text {} " val:04 " text " 10 * 10:o)
/// );
/// ```
#[doc = include_str!("../README.md")]
pub fn println(input: TokenStream) -> TokenStream {
	make_format_string("println", input, false, false)
}

#[proc_macro]
#[proc_macro_error]
/// Compiles to [`::std::eprint!()`](https://doc.rust-lang.org/std/macro.eprint.html).
///
/// ```rs
/// let val = 2 + 2;
/// assert_eq!(
/// 	std::format!("text {{}} {val:04} text {:o}", 10 * 10),
/// 	format!("text {} " val:04 " text " 10 * 10:o)
/// );
/// ```
#[doc = include_str!("../README.md")]
pub fn eprint(input: TokenStream) -> TokenStream {
	make_format_string("eprint", input, false, false)
}
#[proc_macro]
#[proc_macro_error]
/// Compiles to [`::std::eprintln!()`](https://doc.rust-lang.org/std/macro.eprintln.html).
///
/// ```rs
/// let val = 2 + 2;
/// assert_eq!(
/// 	std::format!("text {{}} {val:04} text {:o}", 10 * 10),
/// 	format!("text {} " val:04 " text " 10 * 10:o)
/// );
/// ```
#[doc = include_str!("../README.md")]
pub fn eprintln(input: TokenStream) -> TokenStream {
	make_format_string("eprintln", input, false, false)
}

#[proc_macro]
#[proc_macro_error]
/// Compiles to [`::std::format_args!()`](https://doc.rust-lang.org/std/macro.format_args.html).
///
/// ```rs
/// let val = 2 + 2;
/// assert_eq!(
/// 	std::format!("text {{}} {val:04} text {:o}", 10 * 10),
/// 	format!("text {} " val:04 " text " 10 * 10:o)
/// );
/// ```
#[doc = include_str!("../README.md")]
pub fn format_args(input: TokenStream) -> TokenStream {
	make_format_string("format_args", input, false, true)
}
