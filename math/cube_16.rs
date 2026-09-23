fn cube(n: u64) -> u64 { n * n * n }

fn main() {
    assert_eq!(cube(16), 4096);
    println!("PASS cube_16");
}
