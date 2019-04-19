mod mortgage;

pub fn calculate(principal: f64, interest_rate: f64, number_of_payments: i32) -> String {
    let result = mortgage::mortgage(principal, interest_rate, number_of_payments);
    return format!("{:.2}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_calculate_mortgage() {
        let monthly_payment = super::calculate(100_000.0, 0.005, 180);
        assert_eq!(monthly_payment, "843.86");
    }
}
