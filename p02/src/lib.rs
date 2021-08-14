///Find the last but one box of a list.
pub fn but_last<T>(lst: &[T]) -> Option<[&T; 2]> {
    match lst {
        [] | [_] => None,
        [.., x, y] => Some([x, y]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let vec: Vec<i32> = Vec::new();
        assert_eq!(but_last(&vec), None);
    }

    #[test]
    fn test2() {
        let vec = vec!["A"];
        assert_eq!(but_last(&vec), None);
    }

    #[test]
    fn test3() {
        let vec = vec!["A", "B", "C", "D"];
        assert_eq!(but_last(&vec), Some([&"C", &"D"]));
    }

    #[test]
    fn test4() {
        let vec = vec![1, 2, 3, 4, 5];
        assert_eq!(but_last(&vec), Some([&4, &5]));
    }

    #[test]
    fn test5() {
        let ary = [1, 2, 3, 4, 5];
        assert_eq!(but_last(&ary), Some([&4, &5]));
    }
}
