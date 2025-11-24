//rile r/w with freq
//twin primes
//tossing coins 
//logistic map
//cpu temp

fn is_prime(num: u64) -> bool {
    // Numbers less than or equal to 1 are not prime.
    if num <= 1 {
        return false;
    }

    // Check for divisibility by numbers from 2 up to the square root of num.
    // This optimization significantly reduces the number of checks needed.
    for i in 2..=((num as f64).sqrt() as u64) {
        if num % i == 0 {
            // If num is divisible by any number other than 1 and itself, it's not prime.
            return false;
        }
    }

    // If no divisors are found, the number is prime.
    true
}

//function to print twin primes (pairs of prime numbers that differ by exactly 2) between a range of numbers.
//fn takes, range start value (>2) and end value, and outputs an array with tuples containing the twin primes.


//Vec<T> is Rust’s growable heap-allocated vector. It’s essentially a dynamic array:
//Arrays in Rust have a fixed size known at compile time:
//Rust does not have a built-in linked list like Python or Java’s List. There is std::collections::LinkedList, but it’s rarely used:

fn twin_primes(start: u64, end: u64) -> Vec<(u32, u32)> {
    let mut result = Vec::new();
    if start < 2 {
        println!("start is less than 2");
        return
    }

    for n in start...=end-2{
        if is_prime(n) && is_prime(n + 2) {
            result.push((n, n + 2));
        }
    }
    result
}

fn main() {
    let start = 3;
    let end = 50;
    let twins = twin_primes(start, end);
    println!("{:?}", twins);
}
