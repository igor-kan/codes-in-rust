fn catalan(n: u64) -> u64 { let mut c: u64 = 1; for k in 1..=n { c = c * 2 * (2 * k - 1) / (k + 1); } c }

fn main() {
    assert_eq!(catalan(4), 14);
    println!("PASS catalan_4");
}
