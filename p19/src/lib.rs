///  Rotate a list N places to the left.

fn rotate<T: Copy>(vec: &Vec<T>, n: i32) -> Vec<T> {
    let idx: usize = match n {
        n if n >= 0 => n as usize,
        _ => (n + vec.len() as i32) as usize,
    };

    let (v1, v2) = vec.split_at(idx);
    let mut ret: Vec<T> = v2.to_vec();
    ret.extend(v1.to_vec());
    ret
}

mod tests {
    use super::*;

    #[test]
    fn test1() {
        let vec = vec!["a", "b", "c", "d", "e", "f", "g", "h"];
        assert_eq!(
            rotate(&vec, 3),
            vec!["d", "e", "f", "g", "h", "a", "b", "c"]
        )
    }

    #[test]
    fn test2() {
        let vec = vec!["a", "b", "c", "d", "e", "f", "g", "h"];
        assert_eq!(
            rotate(&vec, -2),
            vec!["g", "h", "a", "b", "c", "d", "e", "f"]
        )
    }
}
