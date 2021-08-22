/// Goldbach's conjecture.
/// Goldbach's conjecture says that every positive even number greater than 2 is the sum of two prime numbers.
/// Example: 28 = 5 + 23. It is one of the most famous facts in number theory that has not been proved to be correct in the general case.
/// It has been numerically confirmed up to very large numbers (much larger than we can go with our Prolog system).
// /Write a predicate to find the two prime numbers that sum up to a given even integer.

pub fn goldbach(n: u32) -> (u32, u32) {
    for i in 2..=n {
        if p31::is_prime(i) & p31::is_prime(n - i) {
            return (i, n - i);
        }
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(goldbach(28), (5, 23));

        assert_eq!(goldbach(4), (2, 2));
        assert_eq!(goldbach(6), (3, 3));
        assert_eq!(goldbach(8), (3, 5));
        assert_eq!(goldbach(10), (3, 7));

    }
}
