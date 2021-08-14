/// Reverse a list.
pub fn reverse<T: Copy>(vec: &Vec<T>) -> Vec<T> {
    let mut v = vec.clone();
    v.reverse();
    v
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let vec = vec![1, 2, 3, 4, 5];
        assert_eq!(reverse(&vec), vec![5, 4, 3, 2, 1])
    }

    #[test]
    fn test2() {
        let vec: Vec<i32> = Vec::new();
        assert_eq!(reverse(&vec), vec);
    }
}
