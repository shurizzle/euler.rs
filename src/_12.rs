use crate::util::Factorial;

fn divisors_len<I>(n: I) -> I
where
    I: num::Integer + num::integer::Roots + num::traits::NumAssign + Copy,
{
    let mut res = I::zero();
    let max = n.sqrt();
    let mut x = I::one();

    while x <= max {
        if n % x == I::zero() {
            res += I::one();
            res += I::one();
        }
        x += I::one();
    }

    res
}

pub fn solve() {
    print!(
        "{}",
        Factorial::<u64>::new()
            .find(|&x| divisors_len(x) > 500)
            .unwrap()
    );
}
