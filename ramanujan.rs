fn is_prime(n: u64) -> bool {
    if n <= 1 { return false; }
    if n <= 3 { return true; }
    if n % 2 == 0 || n % 3 == 0 { return false; }
 
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
 }
 
 fn count_primes_in_interval(start: u64, end: u64) -> u64 {
    (start + 1..=end).filter(|&n| is_prime(n)).count() as u64
 }
 
 fn is_ramanujan_prime(number: u64, n: u64) -> bool {
    if !is_prime(number) {
        return false;
    }
 
    for x in number..=2*number {
        if count_primes_in_interval(x/2, x) < n {
            return false;
        }
    }
 
    for smaller in 2..number {
        let mut is_valid = true;
        for x in smaller..=2*smaller {
            if count_primes_in_interval(x/2, x) < n {
                is_valid = false;
                break;
            }
        }
        if is_valid && is_prime(smaller) {
            return false;
        }
    }
    true
 }
 
 fn main() {
    println!("Enter number to check:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read");
    let number: u64 = input.trim().parse().expect("Invalid number");
 
    println!("Enter n:");
    let mut input = String::new(); 
    std::io::stdin().read_line(&mut input).expect("Failed to read");
    let n: u64 = input.trim().parse().expect("Invalid number");
 
    if is_ramanujan_prime(number, n) {
        println!("{} is the {}th Ramanujan prime", number, n);
    } else {
        println!("{} is not the {}th Ramanujan prime", number, n);
    }
 }