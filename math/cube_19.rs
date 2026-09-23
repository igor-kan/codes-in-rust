fn cube(n: u64) -> u64 { n * n * n }

fn main() {
    assert_eq!(cube(19), 6859);
    println!("PASS cube_19");
}
