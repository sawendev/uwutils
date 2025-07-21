// UwUTils - Helper functions for getting user input

use std::io::{self, Write};

pub fn input_str() -> String {
	let mut buf = String::new();
	io::stdin().read_line(&mut buf).unwrap();
	buf.trim().to_string()
}

pub fn prompt_str(p: &str) -> String {
	print!("{p}");
	io::stdout().flush().unwrap();
	input_str()
}

pub fn input<T: std::str::FromStr>() -> Result<T, T::Err>
where T::Err: std::error::Error {
	input_str().parse()
}

pub fn prompt<T: std::str::FromStr>(p: &str) -> Result<T, T::Err>
where T::Err: std::error::Error {
	prompt_str(p).parse()
}

pub fn force_input<T: std::str::FromStr>() -> T
where T::Err: std::error::Error {
	loop { match input() {
		Ok(val) => break val,
		Err(e) => eprintln!("Error: {e}. Please try again."),
	}}
}

pub fn force_prompt<T: std::str::FromStr>(p: &str) -> T
where T::Err: std::error::Error {
	loop { match prompt(p) {
		Ok(val) => break val,
		Err(e) => eprintln!("Error: {e}. Please try again."),
	}}
}

pub fn choice<'a, T>(options: &'a [(&str, T)]) -> &'a T {
	for (i, (label, _)) in options.iter().enumerate() {
		println!("[{i}] {label}");
	}
	
	loop {
		let i = force_prompt::<usize>("Choice: ");
		if i < options.len() {
			break &options[i].1
		} else {
			println!("Error: index out of bounds. Please try again.");
		}
	}
}

pub fn choice_mut<'a, T>(options: &'a mut [(&str, T)]) -> &'a mut T {
	for (i, (label, _)) in options.iter().enumerate() {
		println!("[{i}] {label}");
	}
	
	loop {
		let i = force_prompt::<usize>("Choice: ");
		if i < options.len() {
			break &mut options[i].1
		} else {
			println!("Error: index out of bounds. Please try again.");
		}
	}
}

pub fn pause() {
	input_str();
}

pub fn pause_prompt(p: &str) {
	prompt_str(p);
}

pub fn paws() { pause() }
pub fn maws(p: &str) { pause_prompt(p) }
