/// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is `13`.
/// What the 10 001st prime number?
pub fn solver(n: isize) -> isize {
    let mut count = 0;
    let mut i = 1;
    while count < n {
        i += 1;
        if is_prime(i) {
            count += 1;
        }
    }
    i
}

fn is_prime(n: isize) -> bool {
    if n < 2 {
        false
    } else if n == 2 {
        true
    } else if n % 2 == 0 {
        false
    } else {
        let mut i = 3;
        // 平方根以下の数まで調べれば十分
        while i * i <= n {
            if n % i == 0 {
                return false;
            }
            i += 2;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_is_prime() {
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(5));
        assert!(!is_prime(6));
        assert!(is_prime(7));
        assert!(!is_prime(8));
        assert!(!is_prime(9));
        assert!(!is_prime(10));
        assert!(is_prime(11));
        assert!(!is_prime(12));
        assert!(is_prime(13));
    }
    #[test]
    fn should_return_13() {
        assert_eq!(solver(6), 13);
    }
    #[test]
    fn should_return_104743() {
        assert_eq!(solver(10001), 104743);
    }
}
