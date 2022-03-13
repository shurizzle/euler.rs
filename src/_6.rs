pub fn solve() {
    print!(
        "{}",
        (1..=100).sum::<u64>().pow(2) - (1..=100).map(|x: u64| x.pow(2)).sum::<u64>()
    );
}
