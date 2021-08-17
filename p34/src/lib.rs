use fraction::{Fraction, ToPrimitive};
use std::collections::HashSet;

/// Calculate Euler's totient function phi(m).
/// Euler's so-called totient function phi(m) is defined as the number of positive integers r (1 <= r < m) that are coprime to m.
/// Example: m = 10: r = 1,3,7,9; thus phi(m) = 4. Note the special case: phi(1) = 1.
pub fn totient_phi(n: u32) -> u32 {
    let set: HashSet<u32> = p35::prime_factorize(n).into_iter().collect();
    let mut ret = Fraction::from(n);
    for x in set {
        ret *= Fraction::from(1) - Fraction::new(1 as u32, x as u32);
    }
    ret.to_u32().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(totient_phi(1), 1);
        assert_eq!(totient_phi(6), 2);
        assert_eq!(totient_phi(10), 4);
        assert_eq!(totient_phi(12), 4);
    }
}
