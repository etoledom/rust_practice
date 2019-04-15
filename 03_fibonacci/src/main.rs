mod fibonacci;
extern crate utilities;

fn main() {
	let title = "Enter the number of elements for Fibonacci sequence:";
	let number_of_elements = utilities::read_number_input::read_non_negative_number(title);
	let sequence = fibonacci::sequence(number_of_elements as u32);
	println!("Fibonacci: [{}]", sequence);
}
