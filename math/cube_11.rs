fn cube(n: u64) -> u64 { n * n * n }

fn main() {
    assert_eq!(cube(11), 1331);
    println!("PASS cube_11");
}
