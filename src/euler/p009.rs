/// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
/// a^2 + b^2 + c^2.
/// For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.
/// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
/// Find the product abc.
pub fn solver(n: isize) -> isize {
    for a in 1..n {
        for b in a..n {
            let c = n - a - b;
            if a * a + b * b == c * c {
                return a * b * c;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_return_60() {
        assert_eq!(solver(12), 60);
    }
    #[test]
    fn should_return_31875000() {
        assert_eq!(solver(1000), 31875000);
    }
}
