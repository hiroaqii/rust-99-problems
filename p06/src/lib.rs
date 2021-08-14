/// Find out whether a list is a palindrome.
/// A palindrome can be read forward or backward; e.g. (x a m a x).
pub fn palindrome<T: PartialEq>(vec: &Vec<T>) -> bool {
    let vec2 = vec.iter().rev();
    vec.iter().zip(vec2).all(|(a, b)| a == b)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        let vec = vec![1, 2, 3, 2, 1];
        assert!(palindrome(&vec));
    }

    #[test]
    fn test2() {
        let vec = vec!["A", "B", "B", "A"];
        assert!(palindrome(&vec));
    }

    #[test]
    fn test3() {
        let vec = vec![1, 2];
        assert_eq!(palindrome(&vec), false);
    }

    #[test]
    fn test4() {
        let vec: Vec<i32> = Vec::new();
        assert!(palindrome(&vec));
    }

    #[test]
    fn test5() {
        let vec = vec!["A", "B", "C", "D", "E"];
        assert!(!palindrome(&vec));
    }
}
