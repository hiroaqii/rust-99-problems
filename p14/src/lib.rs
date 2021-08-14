/// Duplicate the elements of a list.

fn dupli<T: Copy>(vec: &Vec<T>) -> Vec<T> {
    let mut ret: Vec<T> = Vec::with_capacity(vec.len() * 2);
    for v in vec {
        ret.push(*v);
        ret.push(*v);
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let vec: Vec<i32> = Vec::new();
        assert_eq!(dupli(&vec), vec);
    }

    #[test]
    fn test2() {
        let vec = vec!["a", "b", "c", "c", "d"];

        assert_eq!(
            dupli(&vec),
            vec!["a", "a", "b", "b", "c", "c", "c", "c", "d", "d"]
        )
    }
}
