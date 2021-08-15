/// Create a list containing all integers within a given range.
/// If first argument is smaller than second, produce a list in decreasing order.

fn range(start: i32, end: i32) -> Vec<i32> {
    if start >= end {
        vec![]
    } else {
        (start..=end).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(range(4, 9), vec![4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test2() {
        assert_eq!(range(-3, 1), vec![-3, -2, -1, 0, 1]);
    }

    #[test]
    fn test3() {
        assert_eq!(range(9, 4), vec![]);
    }

    #[test]
    fn test4() {
        assert_eq!(range(1, 1), vec![]);
    }
}
