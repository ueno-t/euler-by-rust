/// The sum of the primes below `10` is `2 + 3 + 5 + 7 = 17`.
/// Find the sum of all the primes below two million.
pub fn solver(n: usize) -> usize {
    let mut sum = 0;
    let mut i = 1;
    while i < n {
        i += 1;
        if is_prime(i) {
            sum += i;
        }
    }
    sum
}

fn is_prime(n: usize) -> bool {
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
    fn should_return_17() {
        assert_eq!(solver(10), 17);
    }
    #[test]
    fn should_return_142913828922() {
        assert_eq!(solver(2_000_000), 142913828922);
    }
}
