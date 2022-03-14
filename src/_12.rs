use num::Integer;
use primes::Sieve;

use crate::util::divisors_len_s;

pub fn solve() {
    let mut i: u64 = 1;
    let mut cnt = 0;
    let mut dn1 = 2;
    let mut dn = 2;
    let mut sieve = Sieve::new();
    while cnt < 500 {
        if i.is_even() {
            dn = divisors_len_s(i + 1, &mut sieve);
        } else {
            dn1 = divisors_len_s((i + 1) / 2, &mut sieve);
        }
        cnt = dn * dn1;
        i += 1;
    }

    print!("{}", i * (i - 1) / 2);
}
