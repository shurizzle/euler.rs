pub fn solve() {
    print!(
        "{}",
        crate::util::Fibonacci::new()
            .skip(2)
            .take_while(|&x| x <= 4_000_000)
            .filter(num::Integer::is_even)
            .sum::<u128>()
    )
}
