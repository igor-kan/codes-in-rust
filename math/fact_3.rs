fn fact(n: u64) -> u64 { let mut r: u64 = 1; for i in 2..=n { r *= i; } r }

fn main() {
    assert_eq!(fact(3), 6);
    println!("PASS fact_3");
}
