// Helper functions for constructing/reading binary data

pub struct BinReader<'a> {
	src: &'a [u8],
	idx: usize,
}

impl<'a> BinReader<'a> {
	pub fn new(src: &'a [u8]) -> Self {
		Self { src, idx: 0 }
	}
	
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
