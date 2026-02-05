// SLice UTils - Helper functions for extracting data from byte slices

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
	
	pub fn take_u8(&mut self) -> Option<u8> {
		self.take(u8::from_le_bytes)
	}
	
	pub fn peek_u8(&self) -> Option<u8> {
		self.peek(u8::from_le_bytes)
	}
	
	pub fn take_i8(&mut self) -> Option<i8> {
		self.take(i8::from_le_bytes)
	}
	
	pub fn peek_i8(&self) -> Option<i8> {
		self.peek(i8::from_le_bytes)
	}
	
	pub fn take_le_u16(&mut self) -> Option<u16> {
		self.take(u16::from_le_bytes)
	}
	
	pub fn peek_le_u16(&self) -> Option<u16> {
		self.peek(u16::from_le_bytes)
	}
	
	pub fn take_be_u16(&mut self) -> Option<u16> {
		self.take(u16::from_be_bytes)
	}
	
	pub fn peek_be_u16(&self) -> Option<u16> {
		self.peek(u16::from_be_bytes)
	}
	
	pub fn take_le_i16(&mut self) -> Option<i16> {
		self.take(i16::from_le_bytes)
	}
	
	pub fn peek_le_i16(&self) -> Option<i16> {
		self.peek(i16::from_le_bytes)
	}
	
	pub fn take_be_i16(&mut self) -> Option<i16> {
		self.take(i16::from_be_bytes)
	}
	
	pub fn peek_be_i16(&self) -> Option<i16> {
		self.peek(i16::from_be_bytes)
	}
	
	pub fn take_le_u32(&mut self) -> Option<u32> {
		self.take(u32::from_le_bytes)
	}
	
	pub fn peek_le_u32(&self) -> Option<u32> {
		self.peek(u32::from_le_bytes)
	}
	
	pub fn take_be_u32(&mut self) -> Option<u32> {
		self.take(u32::from_be_bytes)
	}
	
	pub fn peek_be_u32(&self) -> Option<u32> {
		self.peek(u32::from_be_bytes)
	}
	
	pub fn take_le_i32(&mut self) -> Option<i32> {
		self.take(i32::from_le_bytes)
	}
	
	pub fn peek_le_i32(&self) -> Option<i32> {
		self.peek(i32::from_le_bytes)
	}
	
	pub fn take_be_i32(&mut self) -> Option<i32> {
		self.take(i32::from_be_bytes)
	}
	
	pub fn peek_be_i32(&self) -> Option<i32> {
		self.peek(i32::from_be_bytes)
	}
	
	pub fn take_le_u64(&mut self) -> Option<u64> {
		self.take(u64::from_le_bytes)
	}
	
	pub fn peek_le_u64(&self) -> Option<u64> {
		self.peek(u64::from_le_bytes)
	}
	
	pub fn take_be_u64(&mut self) -> Option<u64> {
		self.take(u64::from_be_bytes)
	}
	
	pub fn peek_be_u64(&self) -> Option<u64> {
		self.peek(u64::from_be_bytes)
	}
	
	pub fn take_le_i64(&mut self) -> Option<i64> {
		self.take(i64::from_le_bytes)
	}
	
	pub fn peek_le_i64(&self) -> Option<i64> {
		self.peek(i64::from_le_bytes)
	}
	
	pub fn take_be_i64(&mut self) -> Option<i64> {
		self.take(i64::from_be_bytes)
	}
	
	pub fn peek_be_i64(&self) -> Option<i64> {
		self.peek(i64::from_be_bytes)
	}
	
	pub fn take_le_u128(&mut self) -> Option<u128> {
		self.take(u128::from_le_bytes)
	}
	
	pub fn peek_le_u128(&self) -> Option<u128> {
		self.peek(u128::from_le_bytes)
	}
	
	pub fn take_be_u128(&mut self) -> Option<u128> {
		self.take(u128::from_be_bytes)
	}
	
	pub fn peek_be_u128(&self) -> Option<u128> {
		self.peek(u128::from_be_bytes)
	}
	
	pub fn take_le_i128(&mut self) -> Option<i128> {
		self.take(i128::from_le_bytes)
	}
	
	pub fn peek_le_i128(&self) -> Option<i128> {
		self.peek(i128::from_le_bytes)
	}
	
	pub fn take_be_i128(&mut self) -> Option<i128> {
		self.take(i128::from_be_bytes)
	}
	
	pub fn peek_be_i128(&self) -> Option<i128> {
		self.peek(i128::from_be_bytes)
	}
	
	pub fn take_le_f32(&mut self) -> Option<f32> {
		self.take(f32::from_le_bytes)
	}
	
	pub fn peek_le_f32(&self) -> Option<f32> {
		self.peek(f32::from_le_bytes)
	}
	
	pub fn take_le_f64(&mut self) -> Option<f64> {
		self.take(f64::from_le_bytes)
	}
	
	pub fn peek_le_f64(&self) -> Option<f64> {
		self.peek(f64::from_le_bytes)
	}
	
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
