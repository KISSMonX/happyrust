fn main() {
	let value: u32 = 0x1FFFF;
	let flt32: f32 = 3.141492653;

	let bytes_be = flt32.to_be_bytes();
	let bytes_le = flt32.to_le_bytes();
	println!("{:#x}", bytes_be);
	println!("{:#x}", bytes_le);

	let bytes = value.to_be_bytes();
	println!("{:?}", bytes);
}

fn transform_u32_to_array_of_u8(x: u32) -> [u8; 4] {
	let b1: u8 = ((x >> 24) & 0xff) as u8;
	let b2: u8 = ((x >> 16) & 0xff) as u8;
	let b3: u8 = ((x >> 8) & 0xff) as u8;
	let b4: u8 = (x & 0xff) as u8;
	return [b1, b2, b3, b4];
}
