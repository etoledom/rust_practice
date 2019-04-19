mod mortgage;
extern crate utilities;

fn main() {
    let principal = utilities::read_number_input::read_float("principal: ");
    let interest_rate = utilities::read_number_input::read_float("interest_rate: ");
    let payments = utilities::read_number_input::read_number("payments: ");
    let result = mortgage::calculate(principal, interest_rate, payments as i32);
    println!("Mortgage: {}", result);
}
