/// Eliminate consecutive duplicates of list elements.
/// If a list contains repeated elements they should be replaced with a single copy of the element. The order of the elements should not be changed.
pub fn compress<T: PartialEq + Copy>(vec: &Vec<T>) -> Vec<T> {
    match vec.len() {
        0 => vec![],
        _ => {
            let mut t = &vec[0];
            let mut ret = Vec::new();
            ret.push(*t);
            for x in vec {
                if x != t {
                    ret.push(*x);
                }
                t = x;
            }
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
        assert_eq!(compress(&vec), vec);
    }

    #[test]
    fn test2() {
        let vec = vec![
            "a", "a", "a", "a", "b", "c", "c", "a", "a", "d", "e", "e", "e", "e",
        ];
        assert_eq!(compress(&vec), vec!["a", "b", "c", "a", "d", "e"]);
    }
}
