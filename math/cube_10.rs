fn cube(n: u64) -> u64 { n * n * n }

fn main() {
    assert_eq!(cube(10), 1000);
    println!("PASS cube_10");
}
