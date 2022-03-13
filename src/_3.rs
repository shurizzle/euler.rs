use crate::util::FindMax;

pub fn solve() {
    print!(
        "{}",
        primes::factors_uniq(600_851_475_143).find_max().unwrap()
    );
}
