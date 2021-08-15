/// Determine the greatest common divisor of two positive integer numbers.
/// Use Euclid's algorithm.
fn gcd(a: i32, b: i32) -> i32 {
    let (mut x, mut y) = match (a.abs(), b.abs()) {
        (i, j) if i > j => (i, j),
        (i, j) => (j, i),
    };

    while y != 0 {
        let tmp = x;
        x = y;
        y = tmp % y;
    }
    x
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(gcd(36, 63), 9);
        assert_eq!(gcd(63, 36), 9);
        assert_eq!(gcd(-63, 36), 9);
        assert_eq!(gcd(63, -36), 9);
        assert_eq!(gcd(11, 7), 1);
        assert_eq!(gcd(100, 100), 100);
    }
}
