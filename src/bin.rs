// Helper functions for constructing/reading binary data

pub struct BinReader<'a> {
	src: &'a [u8],
	idx: usize,
}

impl<'a> BinReader<'a> {
	pub fn new(src: &'a [u8]) -> Self {
		Self { src, idx: 0 }
	}
	
	pub fn take_u8(&mut self) -> Option<u8> { self.take(u8::from_le_bytes) }
	pub fn take_i8(&mut self) -> Option<i8> { self.take(i8::from_le_bytes) }
	pub fn take_le_u16(&mut self) -> Option<u16> { self.take(u16::from_le_bytes) }
	pub fn take_le_i16(&mut self) -> Option<i16> { self.take(i16::from_le_bytes) }
	pub fn take_be_u16(&mut self) -> Option<u16> { self.take(u16::from_be_bytes) }
	pub fn take_be_i16(&mut self) -> Option<i16> { self.take(i16::from_be_bytes) }
	pub fn take_le_u32(&mut self) -> Option<u32> { self.take(u32::from_le_bytes) }
	pub fn take_le_i32(&mut self) -> Option<i32> { self.take(i32::from_le_bytes) }
	pub fn take_be_u32(&mut self) -> Option<u32> { self.take(u32::from_be_bytes) }
	pub fn take_be_i32(&mut self) -> Option<i32> { self.take(i32::from_be_bytes) }
	pub fn take_le_u64(&mut self) -> Option<u64> { self.take(u64::from_le_bytes) }
	pub fn take_le_i64(&mut self) -> Option<i64> { self.take(i64::from_le_bytes) }
	pub fn take_be_u64(&mut self) -> Option<u64> { self.take(u64::from_be_bytes) }
	pub fn take_be_i64(&mut self) -> Option<i64> { self.take(i64::from_be_bytes) }
	pub fn take_le_u128(&mut self) -> Option<u128> { self.take(u128::from_le_bytes) }
	pub fn take_le_i128(&mut self) -> Option<i128> { self.take(i128::from_le_bytes) }
	pub fn take_be_u128(&mut self) -> Option<u128> { self.take(u128::from_be_bytes) }
	pub fn take_be_i128(&mut self) -> Option<i128> { self.take(i128::from_be_bytes) }
	pub fn take_le_f32(&mut self) -> Option<f32> { self.take(f32::from_le_bytes) }
	pub fn take_be_f32(&mut self) -> Option<f32> { self.take(f32::from_be_bytes) }
	pub fn take_le_f64(&mut self) -> Option<f64> { self.take(f64::from_le_bytes) }
	pub fn take_be_f64(&mut self) -> Option<f64> { self.take(f64::from_be_bytes) }
	
	pub fn take<const LEN: usize, T>(&mut self, f: fn([u8; LEN]) -> T) -> Option<T> {
		self.take_arr().map(|a| f(a))
	}
	
	pub fn take_arr<const LEN: usize>(&mut self) -> Option<[u8; LEN]> {
		self.take_slice(LEN).map(|s| s.try_into().unwrap())
	}
	
	pub fn take_str(&mut self, len: usize) -> Option<&str> {
		(self.idx + len <= self.src.len()).then(|| {
			let slice = &self.src[self.idx..(self.idx + len)];
			let utf8 = str::from_utf8(slice).ok()?;
			self.idx += len;
			Some(utf8)
		}).flatten()
	}
	
	pub fn take_slice(&mut self, len: usize) -> Option<&[u8]> {
		(self.idx + len <= self.src.len()).then(|| {
			let slice = &self.src[self.idx..(self.idx + len)];
			self.idx += len;
			slice
		})
	}
}
