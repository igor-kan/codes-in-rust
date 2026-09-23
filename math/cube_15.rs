fn cube(n: u64) -> u64 { n * n * n }

fn main() {
    assert_eq!(cube(15), 3375);
    println!("PASS cube_15");
}
