fn cube(n: u64) -> u64 { n * n * n }

fn main() {
    assert_eq!(cube(6), 216);
    println!("PASS cube_6");
}
