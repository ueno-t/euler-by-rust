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
