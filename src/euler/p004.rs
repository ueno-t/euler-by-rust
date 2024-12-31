/// A palindromic number reads the same both ways. The largest palindrome made from the product of two `2`-digit numbers is `9009 = 91 x 99`.
/// Find the largest palindrome made from the product of two `3`-digit numbers.
pub fn solver(n: isize) -> isize {
    let limit = 10_isize.pow(n as u32);
    let start: isize = 10_isize.pow((n - 1) as u32);
    let mut result = 0;
    for i in start..limit {
        for j in start..limit {
            let product: isize = i * j;
            if is_parlindrome(product) && product > result {
                result = product;
            }
        }
    }
    result
}

fn is_parlindrome(n: isize) -> bool {
    let s = n.to_string();
    s == s.chars().rev().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_return_true() {
        assert!(is_parlindrome(9009));
    }

    #[test]
    fn should_return_false() {
        assert!(!is_parlindrome(890));
    }

    #[test]
    fn should_return_9009() {
        assert_eq!(solver(2), 9009);
    }

    #[test]
    fn should_return_906609() {
        assert_eq!(solver(3), 906609);
    }
}
