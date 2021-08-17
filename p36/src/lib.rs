/// Determine the prime factors of a given positive integer (2).
// Construct a list containing the prime factors and their multiplicity.

pub fn prime_factors_mult(n: u32) -> Vec<(u32, u32)> {
    let mut tmp = n;
    let mut ret: Vec<(u32, u32)> = vec![];
    for i in 2..=(n / 2 + 1) {
        let mut cnt = 0;
        loop {
            if tmp % i == 0 {
                cnt += 1;
                tmp /= i;
            } else {
                break;
            }
        }
        if cnt != 0 {
            ret.push((i, cnt));
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(prime_factors_mult(32), vec![(2, 5)]);
        assert_eq!(prime_factors_mult(315), vec![(3, 2), (5, 1), (7, 1)]);
    }
}
