use std::io;

pub fn read_non_negative_number(title: &str) -> u16 {
	let number = read_number(title);
	if number < 0 {
		println!("The number needs to be bigger than 0. Try again.");
		return read_non_negative_number(title);
	}
	return number as u16;
}

pub fn read_number(title: &str) -> i32 {
	println!("{}", title);
	let user_input = read_input();
	match user_input.parse::<i32>() {
		Ok(n) => return n,
		Err(_err) => {
			return read_number(title);
		},
	}
}

pub fn read_input() -> String {
	let mut user_input = String::new();
	io::stdin().read_line(&mut user_input).expect("Failed to read input");
	return user_input.trim().to_string();
}

pub fn read_float(title: &str) -> f64 {
	println!("{}", title);
	let user_input = read_input();
	match user_input.parse::<f64>() {
		Ok(n) => return n,
		Err(_err) => {
			return read_float(title);
		},
	}
}
