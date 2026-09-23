fn cube(n: u64) -> u64 { n * n * n }

fn main() {
    assert_eq!(cube(18), 5832);
    println!("PASS cube_18");
}
