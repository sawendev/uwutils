// Helper functions for extracting data from byte slices

macro_rules! sf {
	($a:ident(mut $b:ident, $c:expr) -> $d:ty) => {
		pub fn $a(&mut self) -> Option<$d> {
			self.$b($c)
		}
	};
	($a:ident($b:ident, $c:expr) -> $d:ty) => {
		pub fn $a(&self) -> Option<$d> {
			self.$b($c)
		}
	};
}

pub struct Slut<'a> {
	bytes: &'a [u8]
}

impl<'a> Slut<'a> {
	pub fn new(src: &'a [u8]) -> Self {
		Self { bytes: src }
	}
	
	pub fn len(&self) -> usize {
		self.bytes.len()
	}
	
	pub fn take_rest(&mut self) -> &'a [u8] {
		let len = self.len();
		let bytes = self.bytes;
		self.bytes = &self.bytes[len..];
		bytes
	}
	
	pub fn peek_rest(&self) -> &'a [u8] {
		self.bytes
	}
	
	pub fn take<T, const N: usize>(&mut self, f: impl FnOnce([u8; N]) -> T) -> Option<T> {
		self.take_arr().map(|a| f(a))
	}
	
	pub fn peek<T, const N: usize>(&self, f: impl FnOnce([u8; N]) -> T) -> Option<T> {
		self.peek_arr().map(|a| f(a))
	}
	
	sf!(take_u8(mut take, u8::from_le_bytes) -> u8);
	sf!(peek_u8(    peek, u8::from_le_bytes) -> u8);
	sf!(take_i8(mut take, i8::from_le_bytes) -> i8);
	sf!(peek_i8(    peek, i8::from_le_bytes) -> i8);
	
	sf!(take_le_u16(mut take, u16::from_le_bytes) -> u16);
	sf!(peek_le_u16(    peek, u16::from_le_bytes) -> u16);
	sf!(take_le_i16(mut take, i16::from_le_bytes) -> i16);
	sf!(peek_le_i16(    peek, i16::from_le_bytes) -> i16);
	sf!(take_be_u16(mut take, u16::from_be_bytes) -> u16);
	sf!(peek_be_u16(    peek, u16::from_be_bytes) -> u16);
	sf!(take_be_i16(mut take, i16::from_be_bytes) -> i16);
	sf!(peek_be_i16(    peek, i16::from_be_bytes) -> i16);
	
	sf!(take_le_u32(mut take, u32::from_le_bytes) -> u32);
	sf!(peek_le_u32(    peek, u32::from_le_bytes) -> u32);
	sf!(take_le_i32(mut take, i32::from_le_bytes) -> i32);
	sf!(peek_le_i32(    peek, i32::from_le_bytes) -> i32);
	sf!(take_be_u32(mut take, u32::from_be_bytes) -> u32);
	sf!(peek_be_u32(    peek, u32::from_be_bytes) -> u32);
	sf!(take_be_i32(mut take, i32::from_be_bytes) -> i32);
	sf!(peek_be_i32(    peek, i32::from_be_bytes) -> i32);
	
	sf!(take_le_u64(mut take, u64::from_le_bytes) -> u64);
	sf!(peek_le_u64(    peek, u64::from_le_bytes) -> u64);
	sf!(take_le_i64(mut take, i64::from_le_bytes) -> i64);
	sf!(peek_le_i64(    peek, i64::from_le_bytes) -> i64);
	sf!(take_be_u64(mut take, u64::from_be_bytes) -> u64);
	sf!(peek_be_u64(    peek, u64::from_be_bytes) -> u64);
	sf!(take_be_i64(mut take, i64::from_be_bytes) -> i64);
	sf!(peek_be_i64(    peek, i64::from_be_bytes) -> i64);
	
	sf!(take_le_u128(mut take, u128::from_le_bytes) -> u128);
	sf!(peek_le_u128(    peek, u128::from_le_bytes) -> u128);
	sf!(take_le_i128(mut take, i128::from_le_bytes) -> i128);
	sf!(peek_le_i128(    peek, i128::from_le_bytes) -> i128);
	sf!(take_be_u128(mut take, u128::from_be_bytes) -> u128);
	sf!(peek_be_u128(    peek, u128::from_be_bytes) -> u128);
	sf!(take_be_i128(mut take, i128::from_be_bytes) -> i128);
	sf!(peek_be_i128(    peek, i128::from_be_bytes) -> i128);
	
	sf!(take_le_f32(mut take, f32::from_le_bytes) -> f32);
	sf!(peek_le_f32(    peek, f32::from_le_bytes) -> f32);
	sf!(take_be_f32(mut take, f32::from_be_bytes) -> f32);
	sf!(peek_be_f32(    peek, f32::from_be_bytes) -> f32);
	
	sf!(take_le_f64(mut take, f64::from_le_bytes) -> f64);
	sf!(peek_le_f64(    peek, f64::from_le_bytes) -> f64);
	sf!(take_be_f64(mut take, f64::from_be_bytes) -> f64);
	sf!(peek_be_f64(    peek, f64::from_be_bytes) -> f64);
	
	pub fn take_arr<const N: usize>(&mut self) -> Option<[u8; N]> {
		self.take_slice(N)?.try_into().ok()
	}
	
	pub fn peek_arr<const N: usize>(&self) -> Option<[u8; N]> {
		self.peek_slice(N)?.try_into().ok()
	}
	
	pub fn take_slice(&mut self, len: usize) -> Option<&'a [u8]> {
		self.bytes.split_at_checked(len).map(|(l, r)| { self.bytes = r; l })
	}
	
	pub fn peek_slice(&self, len: usize) -> Option<&'a [u8]> {
		self.bytes.split_at_checked(len).map(|(l, _)| l)
	}
	
	pub fn take_str(&mut self, len: usize) -> Option<&'a str> {
		self.bytes.split_at_checked(len).and_then(|(l, r)| {
			str::from_utf8(l).map(|s| { self.bytes = r; s }).ok()
		})
	}
	
	pub fn peek_str(&self, len: usize) -> Option<&'a str> {
		self.bytes.split_at_checked(len)
			.and_then(|(l, _)| str::from_utf8(l).ok())
	}
	
	pub fn take_valid_utf8(&mut self) -> &'a str {
		let s = match str::from_utf8(self.bytes) {
			Ok(s) => s,
			Err(e) => {
				let len = e.valid_up_to();
				// I FEAR NO GOD; GOD FEARS ME.
				unsafe { str::from_utf8_unchecked(&self.bytes[..len]) }
			}
		};
		self.bytes = &self.bytes[s.len()..];
		s
	}
	
	pub fn peek_valid_utf8(&self) -> &'a str {
		match str::from_utf8(self.bytes) {
			Ok(s) => s,
			Err(e) => {
				let len = e.valid_up_to();
				unsafe { str::from_utf8_unchecked(&self.bytes[..len]) }
			}
		}
	}
}
