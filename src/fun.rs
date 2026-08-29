// Funny little things, not particularly useful

pub const fn uwuify_in_place(input: &mut str) {
	unsafe {
		let bytes = input.as_bytes_mut();
		let mut x = 0;
		while x < bytes.len() {
			if bytes[x] == b'r' || bytes[x] == b'l' {
				bytes[x] = b'w';
			} else if bytes[x] == b'R' || bytes[x] == b'L' {
				bytes[x] = b'W';
			}
			x += 1;
		}
	}
}

pub fn uwuify(input: &str) -> String {
	let mut out = input.to_owned();
	uwuify_in_place(&mut out);
	out
}

pub const fn snailiness(input: &str) -> f64 {
	let bytes = input.as_bytes();
	let mut words: u64 = 0;
	let mut snaily_words: u64 = 0;
	
	let mut x = 0;
	while x < bytes.len() {
		if bytes[x].is_ascii_alphabetic() {
			let mut y = x + 1;
			while y < bytes.len() && bytes[y].is_ascii_alphabetic() {
				y += 1;
			}
			if y - x >= 3 {
				words += 1;
				if (bytes[x] == b's' || bytes[x] == b'S')
				&& (bytes[x+1] == b'n' || bytes[x+1] == b'N') {
					snaily_words += 1;
				}
			}
			x = y;
		} else {
			x += 1;
		}
	}
	
	if words > 0 {
		snaily_words as f64 / words as f64 * 2.0 - 1.0
	} else {
		0.0
	}
}
