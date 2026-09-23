fn cube(n: u64) -> u64 { n * n * n }

fn main() {
    assert_eq!(cube(5), 125);
    println!("PASS cube_5");
}
