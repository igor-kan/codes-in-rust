fn cube(n: u64) -> u64 { n * n * n }

fn main() {
    assert_eq!(cube(9), 729);
    println!("PASS cube_9");
}
