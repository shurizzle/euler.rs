pub fn solve() {
    let abc_sum = 1_000;
    let x = abc_sum / 2;
    let m = (((x as f64 / 2f64).sqrt().floor() as u64)..=((x as f64).sqrt().ceil() as u64))
        .find(|m| x % m == 0)
        .unwrap();
    let n = (x / m) - m;

    let a = 2 * m * n;
    let b = m.pow(2) - n.pow(2);
    let c = m.pow(2) + n.pow(2);

    print!("{}", a * b * c);
}
