fn cube(n: u64) -> u64 { n * n * n }

fn main() {
    assert_eq!(cube(20), 8000);
    println!("PASS cube_20");
}
