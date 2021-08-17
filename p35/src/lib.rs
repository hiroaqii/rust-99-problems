///Determine the prime factors of a given positive integer.
///Construct a flat list containing the prime factors in ascending order.

pub fn prime_factorize(n: u32) -> Vec<u32> {
    let mut tmp = n;
    let mut ret: Vec<u32> = vec![];
    for i in 2..=(n / 2 + 1) {
        loop {
            if tmp % i == 0 {
                ret.push(i);
                tmp /= i;
            } else {
                break;
            }
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(prime_factorize(10), vec![2, 5]);
        assert_eq!(prime_factorize(16), vec![2, 2, 2, 2]);
        assert_eq!(prime_factorize(315), vec![3, 3, 5, 7]);
    }
}
