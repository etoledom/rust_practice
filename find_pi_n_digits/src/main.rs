mod read_number_input;
mod pi;

fn main() {
    let decimals_count = read_number_input::read_non_negative_number();
    let pi = pi::calculate(decimals_count);
    println!("Pi: {}", pi);
}
