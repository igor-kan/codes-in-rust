fn fib(n: u64) -> u64 { let (mut a, mut b) = (0u64, 1u64); for _ in 0..n { let t = a + b; a = b; b = t; } a }

fn main() {
    assert_eq!(fib(40), 102334155);
    println!("PASS fib_40");
}
