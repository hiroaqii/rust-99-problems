use rand::{thread_rng, Rng};

/// Generate a random permutation of the elements of a list.

fn rnd_permu<T: Copy>(vec: &Vec<T>) -> Vec<T> {
    let mut tmp = vec.clone();
    let mut ret = Vec::with_capacity(vec.len());
    let mut rng = thread_rng();

    for _ in 1..=vec.len() {
        let i = rng.gen_range(0..(tmp.len()));
        ret.push(tmp.remove(i));
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let vec = vec!["a", "b", "c", "d", "e", "f", "g", "h"];
        let ret = rnd_permu(&vec);
        assert_eq!(ret.len(), vec.len());
        println!("{:?}", ret);
    }
}
