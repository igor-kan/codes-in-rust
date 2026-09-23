fn cube(n: u64) -> u64 { n * n * n }

fn main() {
    assert_eq!(cube(14), 2744);
    println!("PASS cube_14");
}
