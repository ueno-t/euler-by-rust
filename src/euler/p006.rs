/// The sum of the squares of the first ten natural numbers is, `1^2 + 2^2 + ... + 10 ^ 2 = 385`
/// The square of the sum of the first ten natural numbers is, `(1 + 2 + ... + 10) ^ 2 = 55 ^ 2 = 3025`
/// Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is `3025 - 385 = 2640`.
/// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
pub fn solver(n: isize) -> isize {
    let sum_of_squares: isize = (1..=n).map(|x| x.pow(2)).sum::<isize>();
    let suqares_of_sum: isize = (1..=n).sum::<isize>().pow(2);
    suqares_of_sum - sum_of_squares
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_return_2640() {
        assert_eq!(solver(10), 2640);
    }
    #[test]
    fn should_return_25164150() {
        assert_eq!(solver(100), 25164150);
    }
}
