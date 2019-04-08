pub fn sequence(n: u32) -> String {
    let mut fib_sequence = fibonacci_sequence(n);
    fib_sequence.reverse();
    let sequence_strings: Vec<String> = fib_sequence.iter().map(|n| n.to_string()).collect();
    return sequence_strings.join(" ");
}

fn fibonacci_sequence(n: u32) -> Vec<u32> {
    match n {
        0 => vec![],
        _ => {
            let mut fib = vec![fibonacci(n)];
            let mut next = fibonacci_sequence(n-1);
            // Can't we just create a new vector from two inmutable vectors?
            fib.append(&mut next);
            return fib;
        },
    }
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        2 => 1,
        _ => fibonacci(n-1) + fibonacci(n-2),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_fibonacci() {
        assert_eq!(super::fibonacci(0), 0);
        assert_eq!(super::fibonacci(1), 1);
        assert_eq!(super::fibonacci(2), 1);
        assert_eq!(super::fibonacci(3), 2);
        assert_eq!(super::fibonacci(4), 3);
        assert_eq!(super::fibonacci(8), 21);
        assert_eq!(super::fibonacci(10), 55);
    }

    #[test]
    fn test_sequence() {
        // 1 1 2 3 5 8 13 21 34 55 89 144 233 377 610 987 1597 2584 4181 6765 10946 17711 28657 46368 75025 121393 196418 317811 ...
        assert_eq!(super::sequence(1), "1");
        assert_eq!(super::sequence(2), "1 1");
        assert_eq!(super::sequence(3), "1 1 2");
        assert_eq!(super::sequence(4), "1 1 2 3");
        assert_eq!(super::sequence(5), "1 1 2 3 5");
        assert_eq!(super::sequence(6), "1 1 2 3 5 8");
        assert_eq!(super::sequence(8), "1 1 2 3 5 8 13 21");
        assert_eq!(super::sequence(25), "1 1 2 3 5 8 13 21 34 55 89 144 233 377 610 987 1597 2584 4181 6765 10946 17711 28657 46368 75025");
    }
}
