/// Run-length encoding of a list.
/// Use the result of problem P09 to implement the so-called run-length encoding data compression method.
/// Consecutive duplicates of elements are encoded as lists (N E) where N is the number of duplicates of the element E.
pub fn encode<T: Copy + PartialEq>(vec: &Vec<T>) -> Vec<(usize, T)> {
    let mut ret: Vec<(usize, T)> = Vec::new();
    match vec.len() {
        0 => ret,
        _ => {
            let mut n = 1;
            let mut t = &vec[0];
            for i in 1..vec.len() {
                if t == &vec[i] {
                    n += 1;
                } else {
                    ret.push((n, *t));
                    t = &vec[i];
                    n = 1;
                }
            }
            ret.push((n, *t));
            ret
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let vec: Vec<i32> = Vec::new();
        let ret: Vec<(usize, i32)> = vec![];
        assert_eq!(encode(&vec), ret);
    }

    #[test]
    fn test2() {
        let vec = vec![
            "a", "a", "a", "a", "b", "c", "c", "a", "a", "d", "e", "e", "e", "e",
        ];
        assert_eq!(
            encode(&vec),
            vec![(4, "a"), (1, "b"), (2, "c"), (2, "a"), (1, "d"), (4, "e")]
        );
    }
}
