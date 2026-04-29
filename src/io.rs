// Helper functions for getting user input

use std::io::{self, Write};

pub fn input_str() -> String {
	let mut input = String::new();
	io::stdin().read_line(&mut input).unwrap();
	input.truncate(input.trim_end().len());
	input
}

pub fn prompt_str(p: &str) -> String {
	print!("{p}");
	io::stdout().flush().unwrap();
	input_str()
}

pub fn input<T: std::str::FromStr>() -> Result<T, T::Err> {
	input_str().parse()
}

pub fn prompt<T: std::str::FromStr>(p: &str) -> Result<T, T::Err> {
	prompt_str(p).parse()
}
