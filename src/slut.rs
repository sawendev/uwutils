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
	
	pub fn take<T, const N: usize>(&mut self, f: impl FnOnce([u8; N]) -> T) -> Option<T> {
		self.take_arr().map(|a| f(a))
	}
	
	pub fn take_u8(&mut self) -> Option<u8> {
		self.take(u8::from_le_bytes)
	}
	
	pub fn take_le_u16(&mut self) -> Option<u16> {
		self.take(u16::from_le_bytes)
	}
	
	pub fn take_be_u16(&mut self) -> Option<u16> {
		self.take(u16::from_be_bytes)
	}
	
	pub fn take_le_u32(&mut self) -> Option<u32> {
		self.take(u32::from_le_bytes)
	}
	
	pub fn take_be_u32(&mut self) -> Option<u32> {
		self.take(u32::from_be_bytes)
	}
	
	pub fn take_le_u64(&mut self) -> Option<u64> {
		self.take(u64::from_le_bytes)
	}
	
	pub fn take_be_u64(&mut self) -> Option<u64> {
		self.take(u64::from_be_bytes)
	}
	
	pub fn take_le_u128(&mut self) -> Option<u128> {
		self.take(u128::from_le_bytes)
	}
	
	pub fn take_be_u128(&mut self) -> Option<u128> {
		self.take(u128::from_be_bytes)
	}
	
	pub fn take_arr<const N: usize>(&mut self) -> Option<[u8; N]> {
		self.take_slice(N).map(|s| s.try_into().unwrap())
	}
	
	pub fn take_slice(&mut self, len: usize) -> Option<&'a [u8]> {
		self.bytes.split_at_checked(len).map(|(l, r)| { self.bytes = r; l })
	}
	
	pub fn take_str(&mut self, len: usize) -> Option<&'a str> {
		self.bytes.split_at_checked(len).and_then(|(l, r)| {
			str::from_utf8(l).map(|s| { self.bytes = r; s }).ok()
		})
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
}
