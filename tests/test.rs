use std::io::Write;

use inline_format::{format, format_args, write};

#[test]
fn format() {
	let val = 2 + 2;
	assert_eq!(
		std::format!(
			"test {{}} {{}} {val:04} test {x10:o} {x10} {x10}",
			x10 = 10 * 10
		),
		format!("test {} {} " {val}:04 " test " x10 = 10 * 10:o " " x10 " " x10)
	);
}

#[test]
fn format_args() {
	let val = 2 + 2;
	assert_eq!(
		std::format_args!(
			"test {{}} {{}} {val:04} test {x10:o} {x10} {x10}",
			x10 = 10 * 10
		).to_string(),
		format_args!("test {} {} " {val}:04 " test " x10 = 10 * 10:o " " x10 " " x10)
			.to_string()
	);
}

#[test]
fn write() -> std::io::Result<()> {
	let mut w1 = vec![];
	let mut w2 = vec![];

	let val = 2 + 2;

	std::write!(
		&mut w1,
		"test {{}} {{}} {val:04} test {x10:o} {x10} {x10}",
		x10 = 10 * 10
	)?;
	write!(&mut w2, "test {} {} " {val}:04 " test " x10 = 10 * 10:o " " x10 " " x10)?;

	assert_eq!(w1, w2);

	Ok(())
}
