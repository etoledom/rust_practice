use std::collections::HashMap;

pub fn sequence(n: u32) -> String {
	let fib_sequence = fibonacci_sequence(n + 1);
	return stringify_sequence(fib_sequence);
}

fn stringify_sequence(seq: [u128; 128]) -> String {
	let sequence_strings: Vec<String> = seq
		.iter()
		.filter(|&&n| n != 0)
		.map(|n| n.to_string())
		.collect();
	return sequence_strings.join(" ");
}

fn fibonacci_sequence(n: u32) -> [u128; 128] {
	let mut buff = [0; 128];
	let mut memo: HashMap<u32, u128> = HashMap::new();
	for i in 0..n {
		match memo.get(&i) {
			Some(value) => buff[i as usize] = *value,
			None => {
				buff[i as usize] = memoized_fibonacci(i, &memo);
				memo.insert(i, buff[i as usize]);
			}
		}
	}
	return buff;
}

fn memoized_fibonacci(n: u32, memo: &HashMap<u32, u128>) -> u128 {
	match memo.get(&n) {
		Some(value) => return *value,
		None => fibonacci(n),
	}
}

fn fibonacci(n: u32) -> u128 {
	match n {
		0 => 0,
		1 => 1,
		2 => 1,
		_ => fibonacci(n - 1) + fibonacci(n - 2),
	}
}

#[cfg(test)]
mod tests {
	use std::collections::HashMap;

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
	fn test_memoized_fibonacci_returns_data_from_hash() {
		let mut memo: HashMap<u32, u128> = HashMap::new();
		memo.insert(0, 1000);
		assert_eq!(super::memoized_fibonacci(0, &memo), 1000);
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
