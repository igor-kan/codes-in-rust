fn cube(n: u64) -> u64 { n * n * n }

fn main() {
    assert_eq!(cube(13), 2197);
    println!("PASS cube_13");
}
