/// Replicate the elements of a list a given number of times.

fn repli<T: Copy>(vec: &Vec<T>, n: usize) -> Vec<T> {
    let mut ret: Vec<T> = Vec::with_capacity(vec.len() * n);
    for v in vec {
        for _ in 1..=n {
            ret.push(*v);
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let vec: Vec<i32> = Vec::new();
        assert_eq!(repli(&vec, 1), vec);
    }

    #[test]
    fn test2() {
        let vec = vec!["a", "b", "c"];
        assert_eq!(repli(&vec, 3), vec!["a", "a", "a", "b", "b", "b", "c", "c", "c"])
    }
}
