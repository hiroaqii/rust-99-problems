/// Determine whether a given integer number is prime.
pub fn is_prime(n: u32) -> bool {
    match n {
        0 | 1 => false,
        _ => {
            for x in 2..=(n as f64).sqrt() as u32 {
                if n % x == 0 {
                    return false;
                }
            }
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(is_prime(0), false);
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(11), true);
        assert_eq!(is_prime(100), false);
        assert_eq!(is_prime(233), true);
    }
}
