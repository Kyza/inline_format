use std::io::Write;

use inline_format::{format, format_args, write};

#[test]
fn format() {
	let val = 2 + 2;

	let std = std::format!(
		"test {{}} {{}} {val:\"<5} test {x10:o} {x10} {x10}",
		x10 = 10 * 10
	);

	let inline1 = format!("test {} {} " {val}:"\"<5" " test " x10 = 10 * 10:o " " x10 " " x10);

	let inline2 = format!("test {} {} ", {val}:"\"<5", " test ", x10 = 10 * 10:o, " ", x10, " ", x10);

	assert_eq!(std, inline1);
	assert_eq!(inline1, inline2);
}

#[test]
fn format_args() {
	let val = 2 + 2;

	let std = std::format_args!(
		"test {{}} {{}} {val:,<5} test {x10:o} {x10} {x10}",
		x10 = 10 * 10
	)
	.to_string();

	let inline1 = format_args!("test {} {} " {val}:,<5 " test " x10 = 10 * 10:o " " x10 " " x10)
			.to_string();

	let inline2 = format_args!("test {} {} ", {val}:,<5, " test ", x10 = 10 * 10:o, " ", x10, " ", x10)
			.to_string();

	assert_eq!(std, inline1);
	assert_eq!(inline1, inline2);
}

#[test]
fn write() -> std::io::Result<()> {
	let mut w1 = vec![];
	let mut w2 = vec![];
	let mut w3 = vec![];

	let val = 2 + 2;

	std::write!(
		&mut w1,
		"test {{}} {{}} {val:\"<5} test {x10:o} {x10} {x10}",
		x10 = 10 * 10
	)?;
	write!(&mut w2 "test {} {} " {val}:"\"<5" " test " x10 = 10 * 10:o " " x10 " " x10)?;
	write!(&mut w3, "test {} {} ", {val}:"\"<5", " test ", x10 = 10 * 10:o, " ", x10, " ", x10)?;

	assert_eq!(w1, w2);
	assert_eq!(w2, w3);

	Ok(())
}
