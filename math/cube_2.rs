fn cube(n: u64) -> u64 { n * n * n }

fn main() {
    assert_eq!(cube(2), 8);
    println!("PASS cube_2");
}
