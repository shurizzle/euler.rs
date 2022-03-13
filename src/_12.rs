fn divisors_len(n: u64) -> u64 {
    let mut res = 0;

    for x in 1..=((n as f64).sqrt().floor() as u64) {
        if n % x == 0 {
            res += 2;
        }
    }

    res
}

pub fn solve() {
    print!(
        "{}",
        (1u64..)
            .map(|x| (1..=x).sum())
            .find(|&x| divisors_len(x) > 500)
            .unwrap()
    );
}
