/// Determine whether two positive integer numbers are coprime.
/// Two numbers are coprime if their greatest common divisor equals 1.
pub fn coprime(a: i32, b: i32) -> bool {
    p32::gcd(a, b) == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(coprime(35, 64), true);
        assert_eq!(coprime(10, 5), false);
    }
}
