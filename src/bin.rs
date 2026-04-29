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
	
	pub fn take<const LEN: usize, T>(&mut self, f: impl FnOnce([u8; LEN]) -> T) -> Option<T> {
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
	
	pub fn take_rest(&mut self) -> &[u8] {
		let slice = &self.src[self.idx..];
		self.idx += slice.len();
		slice
	}
	
	pub fn peek_u8(&self) -> Option<u8> { self.peek(u8::from_le_bytes) }
	pub fn peek_i8(&self) -> Option<i8> { self.peek(i8::from_le_bytes) }
	pub fn peek_le_u16(&self) -> Option<u16> { self.peek(u16::from_le_bytes) }
	pub fn peek_le_i16(&self) -> Option<i16> { self.peek(i16::from_le_bytes) }
	pub fn peek_be_u16(&self) -> Option<u16> { self.peek(u16::from_be_bytes) }
	pub fn peek_be_i16(&self) -> Option<i16> { self.peek(i16::from_be_bytes) }
	pub fn peek_le_u32(&self) -> Option<u32> { self.peek(u32::from_le_bytes) }
	pub fn peek_le_i32(&self) -> Option<i32> { self.peek(i32::from_le_bytes) }
	pub fn peek_be_u32(&self) -> Option<u32> { self.peek(u32::from_be_bytes) }
	pub fn peek_be_i32(&self) -> Option<i32> { self.peek(i32::from_be_bytes) }
	pub fn peek_le_u64(&self) -> Option<u64> { self.peek(u64::from_le_bytes) }
	pub fn peek_le_i64(&self) -> Option<i64> { self.peek(i64::from_le_bytes) }
	pub fn peek_be_u64(&self) -> Option<u64> { self.peek(u64::from_be_bytes) }
	pub fn peek_be_i64(&self) -> Option<i64> { self.peek(i64::from_be_bytes) }
	pub fn peek_le_u128(&self) -> Option<u128> { self.peek(u128::from_le_bytes) }
	pub fn peek_le_i128(&self) -> Option<i128> { self.peek(i128::from_le_bytes) }
	pub fn peek_be_u128(&self) -> Option<u128> { self.peek(u128::from_be_bytes) }
	pub fn peek_be_i128(&self) -> Option<i128> { self.peek(i128::from_be_bytes) }
	pub fn peek_le_f32(&self) -> Option<f32> { self.peek(f32::from_le_bytes) }
	pub fn peek_be_f32(&self) -> Option<f32> { self.peek(f32::from_be_bytes) }
	pub fn peek_le_f64(&self) -> Option<f64> { self.peek(f64::from_le_bytes) }
	pub fn peek_be_f64(&self) -> Option<f64> { self.peek(f64::from_be_bytes) }
	
	pub fn peek<const LEN: usize, T>(&self, f: impl FnOnce([u8; LEN]) -> T) -> Option<T> {
		self.peek_arr().map(|a| f(a))
	}
	
	pub fn peek_arr<const LEN: usize>(&self) -> Option<[u8; LEN]> {
		self.peek_slice(LEN).map(|s| s.try_into().unwrap())
	}
	
	pub fn peek_str(&self, len: usize) -> Option<&str> {
		str::from_utf8(self.peek_slice(len)?).ok()
	}
	
	pub fn peek_slice(&self, len: usize) -> Option<&[u8]> {
		(self.idx + len <= self.src.len()).then(|| {
			let slice = &self.src[self.idx..(self.idx + len)];
			slice
		})
	}
	
	pub fn peek_rest(&self) -> &[u8] {
		let slice = &self.src[self.idx..];
		slice
	}
}

pub struct BinBuilder {
	bin: Vec<u8>,
}

impl BinBuilder {
	pub fn new() -> Self {
		Self { bin: Vec::new() }
	}
	
	pub fn build(self) -> Vec<u8> { self.bin }
	
	pub fn push_u8(&mut self, data: u8) { self.push(data, u8::to_le_bytes) }
	pub fn push_le_u16(&mut self, data: u16) { self.push(data, u16::to_le_bytes) }
	pub fn push_le_i16(&mut self, data: i16) { self.push(data, i16::to_le_bytes) }
	pub fn push_be_u16(&mut self, data: u16) { self.push(data, u16::to_be_bytes) }
	pub fn push_be_i16(&mut self, data: i16) { self.push(data, i16::to_be_bytes) }
	pub fn push_le_u32(&mut self, data: u32) { self.push(data, u32::to_le_bytes) }
	pub fn push_le_i32(&mut self, data: i32) { self.push(data, i32::to_le_bytes) }
	pub fn push_be_u32(&mut self, data: u32) { self.push(data, u32::to_be_bytes) }
	pub fn push_be_i32(&mut self, data: i32) { self.push(data, i32::to_be_bytes) }
	pub fn push_le_u64(&mut self, data: u64) { self.push(data, u64::to_le_bytes) }
	pub fn push_le_i64(&mut self, data: i64) { self.push(data, i64::to_le_bytes) }
	pub fn push_be_u64(&mut self, data: u64) { self.push(data, u64::to_be_bytes) }
	pub fn push_be_i64(&mut self, data: i64) { self.push(data, i64::to_be_bytes) }
	pub fn push_le_u128(&mut self, data: u128) { self.push(data, u128::to_le_bytes) }
	pub fn push_le_i128(&mut self, data: i128) { self.push(data, i128::to_le_bytes) }
	pub fn push_be_u128(&mut self, data: u128) { self.push(data, u128::to_be_bytes) }
	pub fn push_be_i128(&mut self, data: i128) { self.push(data, i128::to_be_bytes) }
	pub fn push_le_f32(&mut self, data: f32) { self.push(data, f32::to_le_bytes) }
	pub fn push_be_f32(&mut self, data: f32) { self.push(data, f32::to_be_bytes) }
	pub fn push_le_f64(&mut self, data: f64) { self.push(data, f64::to_le_bytes) }
	pub fn push_be_f64(&mut self, data: f64) { self.push(data, f64::to_be_bytes) }
	
	pub fn push<T, const LEN: usize>(&mut self, data: T, f: impl FnOnce(T) -> [u8; LEN]) {
		self.push_arr(f(data))
	}
	
	pub fn push_arr<const LEN: usize>(&mut self, data: [u8; LEN]) {
		self.bin.extend(data);
	}
	
	pub fn push_str(&mut self, data: &str) {
		self.push_slice(data.as_bytes())
	}
	
	pub fn push_slice(&mut self, data: &[u8]) {
		self.bin.extend_from_slice(data);
	}
	
	#[cfg(feature = "silly")] pub fn build_the_fucker(self) -> Vec<u8> { self.build() }
}
