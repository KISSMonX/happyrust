fn main() {
	let guess: u32 = "23333".parse().expect("Invalid number");
	let gg: f64 = "23.4567890123".parse().expect("Invalid type");

	println!("guess(string to int): {}", guess);
	println!("gg(string to float64): {}", gg);
}