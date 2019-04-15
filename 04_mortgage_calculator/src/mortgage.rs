fn mortgage(principal: f64, interest_rate: f64, number_of_payments: i32) -> f64 {
    let one_plus_rate = 1.0 + interest_rate;
    let one_plus_rate_pow_n = one_plus_rate.powi(number_of_payments);
    return principal * interest_rate * one_plus_rate_pow_n / (one_plus_rate_pow_n - 1.0);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_monthly_payment() {
        let monthly_payment = super::mortgage(100_000.0, 0.005, 180);
        let rounded = (monthly_payment * 100.0).round() / 100.0;
        assert_eq!(rounded, 843.86);
    }
}
