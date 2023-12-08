fn prime_number(number: i32) -> bool {
    if number <= 1 {
        return false
    }
    let limit = ((number as f64).sqrt() as i32).max(2) + 1;
    // The limit is calculated to reduce the number of iterations in the loop.
    // It is based on the mathematical fact that a larger factor of a number is always less than the square root of the number.
    for i in 2..limit {
        if number % i == 0 {
            return false
        }
    }
    return true
}

fn next_prime(number: i32) -> i32 {
    let mut number = number;
    while !prime_number(number) {
        number += 1;
    }
    return number
}



fn main() {
    println!("{}",next_prime(1111111111));
}

