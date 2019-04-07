pub fn calculate(number_of_decimals: u16) -> String {
    match number_of_decimals {
        0 => String::from("2"),
        _ => {
            let result = e_sumation(number_of_decimals * 2 + 2);
            return convert_to_string(result, number_of_decimals);
        }
    }
}

fn convert_to_string(number: f64, number_of_decimals: u16) -> String {
    let number_of_chars = number_of_decimals + 2;
    let pi_string = number.to_string();
    return pi_string[..number_of_chars as usize].to_string();
}

fn e_sumation(until: u16) -> f64 {
    match until {
        0 => 1_f64,
        _ => e_sumation_iteration(until) + e_sumation(until - 1),
    }
}

/// Implementation of: e = 1/0! + 1/1! + 1/2! + 1/3! + 1/4! + ... 1/N!
fn e_sumation_iteration(k: u16) -> f64 {
    return 1_f64 / factorial(k as u128) as f64;
}

fn factorial(number: u128) -> u128 {
    match number {
        0 => 1_u128,
        _ => number * factorial(number - 1),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_factorial() {
        assert_eq!(super::factorial(0), 1);
        assert_eq!(super::factorial(1), 1);
        assert_eq!(super::factorial(2), 2);
        assert_eq!(super::factorial(4), 24);
        assert_eq!(super::factorial(5), 120);
    }

    #[test]
    fn test_get_e() {
        // e = 2.71828182845904523536028747135266249
        assert_eq!(super::calculate(0), "2");
        assert_eq!(super::calculate(1), "2.7");
        assert_eq!(super::calculate(2), "2.71");
        assert_eq!(super::calculate(5), "2.71828");
        assert_eq!(super::calculate(10), "2.7182818284");
        assert_eq!(super::calculate(15), "2.718281828459045");
    }
}
