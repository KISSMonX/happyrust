#[derive(Debug)]
struct User {
	username: String,
	email: String,
	phone: String,
	signincount: u32,
	active: bool,
}

fn main() {
	let usr = User {
		active: false,
		email: String::from("omonkerman@qq.com"),
		username: String::from("侯名"),
		signincount: 1,
		phone: String::from("17621188968"),
	};

	println!("User.active type {}", print_type_of(&User::active));
	println!("User.email type {}", print_type_of(&usr.email));
	println!("User.username type {}", print_type_of(&usr.username));
	println!("User.signincount type {}", print_type_of(&usr.signincount));
	println!("User.phone type {}", print_type_of(&usr.phone));

	println!("{:?}", usr);
}


fn print_type_of<T>(_: &T) {
	println!("{}", std::any::type_name::<T>())
}
    