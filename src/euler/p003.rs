/// The prime factors of `13195` are `5, 7, 3,` and `29` .
/// What is the largest prime factor of the number `600851475143` ?
pub fn solver(n: i64) -> i64 {
    let mut n = n;
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            n /= i;
        } else {
            i += 1;
        }
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_return_29() {
        assert_eq!(solver(13195), 29);
    }

    #[test]
    fn should_return_6857() {
        assert_eq!(solver(600851475143), 6857);
    }
}
