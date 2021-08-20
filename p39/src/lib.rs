///  A list of prime numbers.
/// Given a range of integers by its lower and upper limit, construct a list of all prime numbers in that range.
pub fn prime_numbers(start: u32, end: u32) -> Vec<u32> {
    (start..=end)
        .collect::<Vec<u32>>()
        .into_iter()
        .filter(|n| p31::is_prime(*n))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(prime_numbers(4, 20), vec![5, 7, 11, 13, 17, 19]);
    }
}
