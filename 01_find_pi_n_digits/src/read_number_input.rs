use std::io;

pub fn read_non_negative_number() -> u16 {
    let number = read_number();
    if number < 0 {
        println!("The number needs to be bigger than 0. Try again.");
        return read_non_negative_number();
    }
    return number as u16;
}

pub fn read_number() -> i32 {
    println!("Enter the number of Pi decimals: ");
    let user_input = read_input();
    match user_input.parse::<i32>() {
        Ok(n) => return n,
        Err(_err) => {
            return read_number();
        },
    }
}

pub fn read_input() -> String {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read input");
    return user_input.trim().to_string();
}
