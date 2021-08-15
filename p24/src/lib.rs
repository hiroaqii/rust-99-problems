use std::cmp::min;

use rand::{thread_rng, Rng};

/// Lotto: Draw N different random numbers from the set 1..M.
/// The selected numbers shall be returned in a list.

fn lotto_select(n: usize, rng: usize) -> Vec<usize> {
    let mut vec: Vec<usize> = (1..=rng).collect();
    let mut ret = Vec::with_capacity(n);
    let mut rng = thread_rng();

    for _ in 1..=min(n, vec.len()) {
        let i = rng.gen_range(0..(vec.len()));
        ret.push(vec.remove(i));
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let ret = lotto_select(6, 49);
        assert_eq!(ret.len(), 6);
        println!("{:?}", ret);
    }

    #[test]
    fn test2() {
        let ret = lotto_select(6, 6);
        assert_eq!(ret.len(), 6);
        println!("{:?}", ret);
    }

    #[test]
    fn test3() {
        let ret = lotto_select(6, 5);
        assert_eq!(ret.len(), 5);
        println!("{:?}", ret);
    }
}
