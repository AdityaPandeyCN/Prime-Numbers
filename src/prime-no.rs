fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn generate_primes_up_to(limit: u64) -> Vec<u64> {
    (2..=limit).filter(|&n| is_prime(n)).collect()
}

fn get_first_n_primes(n: usize) -> Vec<u64> {
    let mut primes = Vec::with_capacity(n);
    let mut num = 2;
    
    while primes.len() < n {
        if is_prime(num) {
            primes.push(num);
        }
        num += 1;
    }
    primes
}

fn main() {
    println!("Prime Number Generator");
    println!("-----------------------");

    // Generate first 10 prime numbers
    let first_ten = get_first_n_primes(10);
    println!("First 10 prime numbers: {:?}", first_ten);

    // Generate primes up to 50
    let primes_to_fifty = generate_primes_up_to(50);
    println!("\nPrime numbers up to 50: {:?}", primes_to_fifty);

    // Interactive part
    println!("\nEnter a number to check if it's prime:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let number: u64 = input.trim().parse().expect("Please enter a valid number");

    if is_prime(number) {
        println!("{} is prime!", number);
    } else {
        println!("{} is not prime.", number);
    }
}