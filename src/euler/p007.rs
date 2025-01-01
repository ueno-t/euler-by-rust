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
        return false;
    } else if n == 2 {
        return true;
    } else if n % 2 == 0 {
        return false;
    } else {
        let mut i = 3;
        // 平方根以下の数まで調べれば十分
        while i * i <= n {
            if n % i == 0 {
                return false;
            }
            i += 2;
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_is_prime() {
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(6), false);
        assert_eq!(is_prime(7), true);
        assert_eq!(is_prime(8), false);
        assert_eq!(is_prime(9), false);
        assert_eq!(is_prime(10), false);
        assert_eq!(is_prime(11), true);
        assert_eq!(is_prime(12), false);
        assert_eq!(is_prime(13), true);
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
