use std::usize;

/// Find the number of elements of a list.
pub fn length<T>(lst: &[T]) -> usize {
    lst.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(length(&vec![1, 2, 3, 4, 5]), 5);
    }

    #[test]
    fn test2() {
        let vec: Vec<i32> = Vec::new();
        assert_eq!(length(&vec), 0);
    }
}
