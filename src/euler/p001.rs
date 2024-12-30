/// If we list all the natural numbers below `10` that are multiples of `3` or `5` , we get `3, 5, 6,` and `9`.
/// The sum of these multiples is `23`
/// Find the sum of all the multiples of `3` or `5` below `1000` .
pub fn solver(n: i32) -> i32 {
    let result: i32 = (0..n)
        .map(|x| match (x % 3, x % 5) {
            (0, 0) => x,
            (0, _) => x,
            (_, 0) => x,
            (_, _) => 0,
        })
        .sum();
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_return_233168() {
        assert_eq!(solver(1000), 233168);
    }
}
