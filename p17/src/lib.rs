/// Split a list into two parts; the length of the first part is given.
/// Do not use any predefined predicates.

fn split<T: Copy>(vec: &Vec<T>, n: usize) -> (Vec<T>, Vec<T>) {
    let mut vec1: Vec<T> = vec![];
    let mut vec2: Vec<T> = vec![];
    for (i, v) in vec.iter().enumerate() {
        match i {
            i if i < n => vec1.push(*v),
            _ => vec2.push(*v),
        }
    }
    (vec1, vec2)
}

mod tests {
    use super::*;

    #[test]
    fn test1() {
        let vec = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "k"];

        assert_eq!(
            split(&vec, 3),
            (vec!["a", "b", "c"], vec!["d", "e", "f", "g", "h", "i", "k"])
        )
    }

    #[test]
    fn test2() {
        let vec = vec!["a", "b", "c", "d"];

        assert_eq!(split(&vec, 4), (vec!["a", "b", "c", "d"], vec![]))
    }

    #[test]
    fn test3() {
        let vec = vec!["a", "b", "c", "d"];

        assert_eq!(split(&vec, 100), (vec!["a", "b", "c", "d"], vec![]))
    }

    #[test]
    fn test4() {
        let vec = vec!["a", "b", "c", "d"];

        assert_eq!(split(&vec, 0), (vec![], vec!["a", "b", "c", "d"]))
    }
}
