use crate::util::{FindMax, Palindrome};

pub fn solve() {
    print!(
        "{}",
        (100..999u16)
            .rev()
            .flat_map(|a| (100..999u16).rev().map(move |b| (a as u64) * (b as u64)))
            .filter(Palindrome::is_palindrome)
            .find_max()
            .unwrap()
    );
}
