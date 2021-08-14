///Find the last box of a list.
pub fn last<T>(lst: &[T]) -> Option<&T> {
    lst.last()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let vec = vec![1, 2, 3, 4, 5];
        assert_eq!(last(&vec), Some(&5));
    }

    #[test]
    fn test2() {
        let vec = vec!["A", "B", "C"];
        assert_eq!(last(&vec), Some(&"C"));
    }

    #[test]
    fn test3() {
        let vec: Vec<i32> = Vec::new();
        assert_eq!(last(&vec), None);
    }

    #[test]
    fn test4() {
        let ary = [1, 2, 3];
        assert_eq!(last(&ary), Some(&3));
    }
}