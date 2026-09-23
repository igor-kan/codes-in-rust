fn cube(n: u64) -> u64 { n * n * n }

fn main() {
    assert_eq!(cube(7), 343);
    println!("PASS cube_7");
}
