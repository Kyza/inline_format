# inline_format

A more readable collection of string formatting macros.

```rs
let val = 2 + 2;
assert_eq!(
	std::format!("text {{}} {val:04} text {:o}", 10 * 10),
	format!("text {} " val:04 " text " 10 * 10:o)
);
```

## Features

- [x] `format!` macro.
- [x] `write!` macro.
- [x] `writeln!` macro.
- [x] `print!` macro.
- [x] `println!` macro.
- [x] `eprint!` macro.
- [x] `eprintln!` macro.
- [x] `format_args!` macro.
- [x] Named parameters.

## STD Problems

The format macros in this crate work almost the same as [the standard format macros](https://doc.rust-lang.org/std/fmt/index.html).

The std macros have inline identifiers, but lack support for expressions.

You end up needing to split up your expressions and the places they go in the string. And this pattern only gets worse when you add more arguments and mix identifiers and expressions.

Unless you save expressions to variables, suddenly you need to count and keep track of which arguments go where and which formatting traits get applied.

Modifying those arguments might even lead to mistakes as well, especially when converting to and from inline with the string. `!("{val}") -> !("{}", val)`.

One built in solution is using [named parameters](https://doc.rust-lang.org/std/fmt/index.html#named-parameters) which does improve the experience, but it still has the disadvantage of being after the string.

```rs
let val = 2 + 2;
format!("text {{}} {val:04} text {}", 10 * 10)
```

## Usage

This crate solves all of that by moving the code to outside of the string, and removing the concept of separate arguments (aside from the `stream` target in `write!` and `writeln!`).

```rs
// Glob importing from it will cause a conflict with Rust's std prelude.
// If you'd like a different name there's always `inline_format::{format as iformat}`.
use inline_format::format;

let val = 2 + 2;
format!("text {} " val:04 " text " 10 * 10) // "text {} 0004 text 100"
```

Now you can see exactly where your expressions are located in the string at a glance, and what formatting traits are applied to them if any. All by continuously reading left to right.

On top of that, there's no need to escape `{}` since there's no special syntax inside of the string anymore.

Since the macros compile to the std macro equivalents, they should support the same things and do what you'd normally expect them to do.

## More Examples

### Joining two expressions.

To join multiple expressions in a row, put `""` in between them.

```rs
use inline_format::format;

format!(2 + 2 "" 10 * 10 :04) // 40100
```

### Using blocks.

If you have a longer expression you might need to close it in a block.

```rs
use inline_format::format;

format!({
	let val = 2 + 2;
	val
} "" 10 * 10 :04) // 40100
```

### Using named parameters.

If you'd like to use an expression multiple times, you can name it and reference the name.

They can be named and referenced at any point, but this isn't recommended for readability.

Format traits apply ***to the section not the variable***.

Because of how `format_args!` works the named parameters don't live long enough, so they get automatically cloned for you rather than evaluated multiple times.

```rs
use inline_format::format;

format!(x10 = 10 * 10:o " " x10) // 144 100

// Out of order still works.
// Not recommended.
format!(x10:o " " x10 = 10 * 10) // 144 100

// Named parameters can also be surrounded in blocks.
// Format traits go outside the block.
format!({x10 = 10 * 10}:o " " x10) // 144 100
```
