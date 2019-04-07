extern crate utilities;
mod pi;

fn main() {
    let title = "Enter the number of Pi decimals:";
    let decimals_count = utilities::read_number_input::read_non_negative_number(title);
    let pi = pi::calculate(decimals_count);
    println!("π: {}", pi);
}
