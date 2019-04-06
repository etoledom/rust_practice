/// Calculate Pi with the given number of decimal digits.
pub fn calculate(number_of_decimals: u16) -> String {
    match number_of_decimals {
        0 => String::from("3"),
        _ => {
            let pi = bbp(number_of_decimals as i32);
            return convert_to_string(pi, number_of_decimals);
        }
    }
}

fn convert_to_string(pi: f64, number_of_decimals: u16) -> String {
    let number_of_chars = number_of_decimals + 2;
    let pi_string = pi.to_string();
    return pi_string[..number_of_chars as usize].to_string();
}

/// Implementation of Bailey–Borwein–Plouffe formula
/// https://en.wikipedia.org/wiki/Bailey–Borwein–Plouffe_formula
fn bbp(until: i32) -> f64 {
    return bbp_sumation(until, 0, 0_f64);
}

/// Main summation of Bailey–Borwein–Plouffe formula.
/// 
/// * `until` - Total of iterations.
/// * `current_iteration` - The current iteration index.
/// * `pi` - Compound adition of pi (buffer).
fn bbp_sumation(until: i32, current_iteration: i32, pi: f64) -> f64 {
    match current_iteration {
        index if index == until => pi,
        _ => bbp_sumation(until, current_iteration+1, pi + bbp_sumation_iteration(current_iteration)),
    }
}

fn bbp_sumation_iteration(k: i32) -> f64 {
    let _16ek = 16_f64.powi(k);
    let _8k = 8_f64 * k as f64;
    return (
        (4_f64 / (_8k + 1_f64)) -
        (2_f64 / (_8k + 4_f64)) -
        (1_f64 / (_8k + 5_f64)) -
        (1_f64 / (_8k + 6_f64))
    ) / _16ek;
}

// Unit tests

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_pi() {
        // π = 3.1415926535897932384626433832795028841971
        assert_eq!(super::calculate(0), "3");
        assert_eq!(super::calculate(1), "3.1");
        assert_eq!(super::calculate(2), "3.14");
        assert_eq!(super::calculate(3), "3.141");
        assert_eq!(super::calculate(4), "3.1415");
        assert_eq!(super::calculate(15), "3.141592653589793");
    }
}
