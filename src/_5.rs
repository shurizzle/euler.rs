use num::integer::lcm;

pub fn solve() {
    print!("{}", (11..=19).reduce(lcm).unwrap());
}
