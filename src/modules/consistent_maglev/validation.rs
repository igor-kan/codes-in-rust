pub fn is_prime(n: usize) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2usize;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}
