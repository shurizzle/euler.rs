use primes::{PrimeSet, Sieve};

pub fn solve() {
    print!(
        "{}",
        Sieve::new()
            .iter()
            .take_while(|&n| n < 2_000_000)
            .sum::<u64>()
    );
}
