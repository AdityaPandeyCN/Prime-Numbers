use std::convert::TryInto;
use num::Integer; // Import the Integer trait for nth_root

fn main() {
    let n = 31; // Change this number to test for primality
    println!("Is {} prime? {}", n, aks_primality(n));
}

pub fn aks_primality(n: u64) -> bool {
    // Step 0: Handle small n and even numbers
    if n <= 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    // Step 1: Check if n is a perfect power
    if is_perfect_power(n) {
        return false;
    }

    // Step 2: Find the smallest r such that multiplicative order of n mod r > (log2(n))^2
    let log2_n = n.ilog2() as u64;
    let max_order = log2_n.pow(2);
    let r = find_r(n, max_order);

    // Step 3: Check if any a from 2 to r divides n
    for a in 2..=r {
        if a > n {
            break;
        }
        if n % a == 0 {
            return false;
        }
    }

    // Step 4: If n <= r, return true
    if n <= r {
        return true;
    }

    // Step 5: Check the polynomial congruences
    let phi_r = euler_totient(r);
    let ln_n = (n as f64).ln();
    let sqrt_phi_r = (phi_r as f64).sqrt();
    let l = (sqrt_phi_r * ln_n).floor() as u64;

    for a in 1..=l {
        if !check_congruence(a, r, n) {
            return false;
        }
    }

    // Step 6: n is prime
    true
}

// Helper function to check if n is a perfect power (n = a^b for a > 0, b > 1)
fn is_perfect_power(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    let max_exponent = n.ilog2() as u32;
    for exponent in 2..=max_exponent {
        let a = n.nth_root(exponent); // Use nth_root from the Integer trait
        if a.pow(exponent) == n {
            return true;
        }
    }
    false
}

// Helper function to find the smallest r where ord_r(n) > max_order
fn find_r(n: u64, max_order: u64) -> u64 {
    let mut r = 2;
    loop {
        if gcd(n, r) == 1 {
            let n_mod_r = n % r;
            let mut current = 1;
            let mut found = false;
            for _ in 1..=max_order {
                current = (current * n_mod_r) % r;
                if current == 1 {
                    found = true;
                    break;
                }
            }
            if !found {
                return r;
            }
        }
        r += 1;
    }
}

// Helper function to compute the greatest common divisor (gcd)
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

// Helper function to compute Euler's totient function φ(n)
fn euler_totient(mut n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut result = 1;
    // Check for factor 2
    if n % 2 == 0 {
        let mut exponent = 0;
        while n % 2 == 0 {
            n /= 2;
            exponent += 1;
        }
        result *= 2u64.pow(exponent) - 2u64.pow(exponent - 1);
    }
    // Check for odd factors
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            let mut exponent = 0;
            while n % i == 0 {
                n /= i;
                exponent += 1;
            }
            result *= i.pow(exponent) - i.pow(exponent - 1);
        }
        i += 2;
    }
    // If remaining n is a prime
    if n > 1 {
        result *= n - 1;
    }
    result
}

// Helper function to multiply two polynomials modulo (X^r - 1) and n
fn multiply_polynomials(a: &[u64], b: &[u64], r: usize, n: u64) -> Vec<u64> {
    let mut product = vec![0; r];
    for (i, &a_coeff) in a.iter().enumerate() {
        for (j, &b_coeff) in b.iter().enumerate() {
            let k = (i + j) % r;
            product[k] = (product[k] + a_coeff * b_coeff) % n;
        }
    }
    product
}

// Helper function to compute polynomial exponentiation modulo (X^r - 1) and n
fn polynomial_pow_mod(base: &[u64], exponent: u64, r: usize, n: u64) -> Vec<u64> {
    let mut result = vec![0; r];
    result[0] = 1; // Initialize as the polynomial 1
    let mut current_base = base.to_vec();
    let mut exp = exponent;
    while exp > 0 {
        if exp % 2 == 1 {
            result = multiply_polynomials(&result, &current_base, r, n);
        }
        current_base = multiply_polynomials(&current_base, &current_base, r, n);
        exp /= 2;
    }
    result
}

// Helper function to check the polynomial congruence (X + a)^n ≡ X^n + a mod (X^r - 1, n)
fn check_congruence(a: u64, r: u64, n: u64) -> bool {
    let r_usize = r.try_into().unwrap();
    let mut base = vec![0; r_usize];
    base[0] = a % n;
    base[1] = 1 % n;
    let powered = polynomial_pow_mod(&base, n, r_usize, n);
    let expected_degree = (n % r) as usize;
    let mut expected = vec![0; r_usize];
    expected[0] = a % n;
    expected[expected_degree] = 1 % n;
    powered == expected
}