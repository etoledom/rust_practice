mod e;
extern crate utilities;

fn main() {
    let title = "Enter the number of decimals of e";
    let number_of_decimals = utilities::read_number_input::read_non_negative_number(title);
    let result = e::calculate(number_of_decimals);
    println!("e = {}", result);
}
