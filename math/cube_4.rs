fn cube(n: u64) -> u64 { n * n * n }

fn main() {
    assert_eq!(cube(4), 64);
    println!("PASS cube_4");
}
