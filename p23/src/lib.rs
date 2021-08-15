use std::cmp::min;

use rand::{thread_rng, Rng};

/// Extract a given number of randomly selected elements from a list.
/// The selected items shall be returned in a list.

fn rnd_select<T: Copy>(vec: &Vec<T>, n: usize) -> Vec<T> {
    let mut tmp = vec.clone();
    let mut ret = Vec::with_capacity(n);
    let mut rng = thread_rng();

    for _ in 1..=min(n, vec.len()) {
        let i = rng.gen_range(0..(tmp.len()));
        ret.push(tmp.remove(i));
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let vec = vec!["a", "b", "c", "d", "e", "f", "g", "h"];
        let ret = rnd_select(&vec, 3);
        assert_eq!(ret.len(), 3);
        println!("{:?}", ret);
    }

    #[test]
    fn test2() {
        let vec = vec!["a", "b", "c", "d", "e", "f", "g", "h"];
        let ret = rnd_select(&vec, 8);
        assert_eq!(ret.len(), 8);
        println!("{:?}", ret);
    }

    #[test]
    fn test3() {
        let vec = vec!["a", "b", "c", "d", "e", "f", "g", "h"];
        let ret = rnd_select(&vec, 100);
        assert_eq!(ret.len(), 8);
        println!("{:?}", ret);
    }
}
