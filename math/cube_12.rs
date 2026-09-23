fn cube(n: u64) -> u64 { n * n * n }

fn main() {
    assert_eq!(cube(12), 1728);
    println!("PASS cube_12");
}
