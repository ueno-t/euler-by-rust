/// `2520` is the smallest number that can be divided by each of the numbers from `1` to `10` without any remainder.
/// What is the smallest positive number that is evenly divisible by all of the numbers from `1` to `20`?
pub fn solver(n: isize) -> isize {
    let mut result = 1;
    for i in 1..=n {
        result = lcm(result, i);
    }
    result
}

fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: isize, b: isize) -> isize {
    a * b / gcd(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_return_2520() {
        assert_eq!(solver(10), 2520);
    }

    #[test]
    fn should_return_232792560() {
        assert_eq!(solver(20), 232792560);
    }
}
