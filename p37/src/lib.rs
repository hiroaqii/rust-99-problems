/// Calculate Euler's totient function phi(m) (improved).
/// See problem P34 for the definition of Euler's totient function.
/// If the list of the prime factors of a number m is known in the form of problem P36 then the function phi(m) can be efficiently calculated as follows: Let ((p1 m1) (p2 m2) (p3 m3) ...) be the list of prime factors (and their multiplicities) of a given number m. Then phi(m) can be calculated with the following formula:
/// phi(m) = (p1 - 1) * p1 ** (m1 - 1) + (p2 - 1) * p2 ** (m2 - 1) + (p3 - 1) * p3 ** (m3 - 1) + ...

pub fn phi(n: u32) -> u32 {
    p36::prime_factors_mult(n)
        .iter()
        .fold(1, |ret, (p, m)| ret * (p - 1) * p.pow(m - 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(phi(1), 1);
        assert_eq!(phi(6), 2);
        assert_eq!(phi(10), 4);
        assert_eq!(phi(12), 4);
    }
}
