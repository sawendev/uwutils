// SLice UTils - Helper functions for extracting data from byte slices

pub fn take<T, const N: usize>(bytes: &mut &[u8], f: impl FnOnce([u8; N]) -> T) -> Option<T> {
	take_arr(bytes).map(|a| f(a))
}

pub fn take_u8(bytes: &mut &[u8]) -> Option<u8> {
	take(bytes, u8::from_le_bytes)
}

pub fn take_le_u16(bytes: &mut &[u8]) -> Option<u16> {
	take(bytes, u16::from_le_bytes)
}

pub fn take_be_u16(bytes: &mut &[u8]) -> Option<u16> {
	take(bytes, u16::from_be_bytes)
}

pub fn take_le_u32(bytes: &mut &[u8]) -> Option<u32> {
	take(bytes, u32::from_le_bytes)
}

pub fn take_be_u32(bytes: &mut &[u8]) -> Option<u32> {
	take(bytes, u32::from_be_bytes)
}

pub fn take_le_u64(bytes: &mut &[u8]) -> Option<u64> {
	take(bytes, u64::from_le_bytes)
}

pub fn take_be_u64(bytes: &mut &[u8]) -> Option<u64> {
	take(bytes, u64::from_be_bytes)
}

pub fn take_le_u128(bytes: &mut &[u8]) -> Option<u128> {
	take(bytes, u128::from_le_bytes)
}

pub fn take_be_u128(bytes: &mut &[u8]) -> Option<u128> {
	take(bytes, u128::from_be_bytes)
}

pub fn take_arr<const N: usize>(bytes: &mut &[u8]) -> Option<[u8; N]> {
	take_slice(bytes, N).map(|s| s.try_into().unwrap())
}

pub fn take_slice<'a>(bytes: &mut &'a [u8], len: usize) -> Option<&'a [u8]> {
	bytes.split_at_checked(len).map(|(l, r)| {
		*bytes = r;
		l
	})
}

pub fn take_str<'a>(bytes: &mut &'a [u8], len: usize) -> Option<&'a str> {
	take_slice(bytes, len).and_then(|s| str::from_utf8(s).ok())
}
