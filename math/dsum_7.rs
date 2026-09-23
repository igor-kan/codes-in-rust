fn dsum(mut n: u64) -> u64 { let mut s = 0u64; while n > 0 { s += n % 10; n /= 10; } s }

fn main() {
    assert_eq!(dsum(7), 7);
    println!("PASS dsum_7");
}
