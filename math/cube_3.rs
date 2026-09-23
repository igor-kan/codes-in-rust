fn cube(n: u64) -> u64 { n * n * n }

fn main() {
    assert_eq!(cube(3), 27);
    println!("PASS cube_3");
}
