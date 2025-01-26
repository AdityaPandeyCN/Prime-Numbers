use std::io;

// Sieve of Eratosthenes for efficient prime counting
fn sieve(limit: u64) -> Vec<bool> {
    let mut is_prime = vec![true; (limit + 1) as usize];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..=(limit as f64).sqrt() as u64 {
        if is_prime[i as usize] {
            for j in (i * i..=limit).step_by(i as usize) {
                is_prime[j as usize] = false;
            }
        }
    }
    is_prime
}

// Count primes in the interval (x/2, x]
fn count_primes(x: u64, sieve: &[bool]) -> u64 {
    let start = (x / 2) + 1;
    let end = x;
    (start..=end).filter(|&n| sieve[n as usize]).count() as u64
}

// Check if a number is a Ramanujan prime
fn is_ramanujan_prime(number: u64, n: u64, sieve: &[bool]) -> bool {
    if !sieve[number as usize] {
        return false;
    }

    // Check for all x >= number up to 2 * number
    for x in number..=2 * number {
        if count_primes(x, sieve) < n {
            return false;
        }
    }

    // Ensure no smaller prime satisfies the condition
    for smaller in 2..number {
        if sieve[smaller as usize] {
            let mut is_valid = true;
            for x in smaller..=2 * smaller {
                if count_primes(x, sieve) < n {
                    is_valid = false;
                    break;
                }
            }
            if is_valid {
                return false;
            }
        }
    }

    true
}

pub fn run() {
    println!("Enter number to check:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let number: u64 = input.trim().parse().expect("Please enter a valid number");

    println!("Enter n:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: u64 = input.trim().parse().expect("Please enter a valid number");

    // Generate sieve up to 2 * number for efficient prime counting
    let sieve = sieve(2 * number);

    if is_ramanujan_prime(number, n, &sieve) {
        println!("{} is the {}th Ramanujan prime", number, n);
    } else {
        println!("{} is not the {}th Ramanujan prime", number, n);
    }
}