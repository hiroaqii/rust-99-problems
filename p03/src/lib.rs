///Find the K'th element of a list.
pub fn element_at<T: Copy>(lst: &[T], idx: usize) -> Option<&T> {
    if idx == 0 || lst.len() < idx {
        None
    } else {
        Some(&lst[idx - 1])
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test1() {
        let vec: Vec<i32> = Vec::new();
        assert_eq!(element_at(&vec, 1), None);
    }

    #[test]
    fn test2() {
        let vec = vec!["A", "B", "C", "D"];
        assert_eq!(element_at(&vec, 0), None);
        assert_eq!(element_at(&vec, 1), Some(&"A"));
        assert_eq!(element_at(&vec, 3), Some(&"C"));
        assert_eq!(element_at(&vec, 5), None);
    }

    #[test]
    fn test3() {
        let ary = [1, 2, 3];
        assert_eq!(element_at(&ary, 1), Some(&1));
        assert_eq!(element_at(&ary, 5), None);
    }
}
