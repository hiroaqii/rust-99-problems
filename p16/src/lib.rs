///  Drop every N'th element from a list.

fn drop<T: Copy>(vec: &Vec<T>, n: usize) -> Vec<T> {
    let mut ret: Vec<T> = Vec::new();
    for (i, v) in vec.iter().enumerate() {
        if (i + 1) % n != 0 {
            ret.push(*v);
        }
    }
    ret
}

mod tests {
    use super::*;

    #[test]
    fn test() {
        let vec = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "k"];

        assert_eq!(drop(&vec, 3), vec!["a", "b", "d", "e", "g", "h", "k"])
    }
}
