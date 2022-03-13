use primes::{PrimeSet, Sieve};

pub fn solve() {
    print!("{}", Sieve::new().get(10_001 - 1));
}
