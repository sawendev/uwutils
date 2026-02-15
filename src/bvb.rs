// Binary Vector Builder

macro_rules! bf {
	($a:ident($b:ty) => $c:ident($d:expr)) => {
		pub fn $a(&mut self, val: $b) {
			self.$c(val, $d);
		}
	};
}

pub struct Bvb {
	data: Vec<u8>
}

impl Bvb {
	pub fn new() -> Self {
		Self { data: Vec::new() }
	}
	
	pub fn finish(self) -> Vec<u8> {
		self.data
	}
	
	pub fn len(&self) -> usize {
		self.data.len()
	}
	
	pub fn push<T, const N: usize>(&mut self, val: T, f: impl FnOnce(T) -> [u8; N]) {
		self.data.extend(f(val));
	}
	
	pub fn push_str(&mut self, val: &str) {
		self.data.extend(val.as_bytes());
	}
	
	pub fn push_arr<const N: usize>(&mut self, val: [u8; N]) {
		self.data.extend(val);
	}
	
	pub fn push_slice(&mut self, val: &[u8]) {
		self.data.extend(val);
	}
	
	bf!(push_u8(u8) => push(u8::to_le_bytes));
	bf!(push_le_u16(u16) => push(u16::to_le_bytes));
	bf!(push_be_u16(u16) => push(u16::to_be_bytes));
	bf!(push_le_i16(i16) => push(i16::to_le_bytes));
	bf!(push_be_i16(i16) => push(i16::to_be_bytes));
	bf!(push_le_u32(u32) => push(u32::to_le_bytes));
	bf!(push_be_u32(u32) => push(u32::to_be_bytes));
	bf!(push_le_i32(i32) => push(i32::to_le_bytes));
	bf!(push_be_i32(i32) => push(i32::to_be_bytes));
	bf!(push_le_u64(u64) => push(u64::to_le_bytes));
	bf!(push_be_u64(u64) => push(u64::to_be_bytes));
	bf!(push_le_i64(i64) => push(i64::to_le_bytes));
	bf!(push_be_i64(i64) => push(i64::to_be_bytes));
	bf!(push_le_u128(u128) => push(u128::to_le_bytes));
	bf!(push_be_u128(u128) => push(u128::to_be_bytes));
	bf!(push_le_i128(i128) => push(i128::to_le_bytes));
	bf!(push_be_i128(i128) => push(i128::to_be_bytes));
	bf!(push_le_f32(f32) => push(f32::to_le_bytes));
	bf!(push_be_f32(f32) => push(f32::to_be_bytes));
	bf!(push_le_f64(f64) => push(f64::to_le_bytes));
	bf!(push_be_f64(f64) => push(f64::to_be_bytes));
}

impl From<Bvb> for Vec<u8> {
	fn from(value: Bvb) -> Self {
		value.data
	}
}
