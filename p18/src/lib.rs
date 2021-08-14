/// Extract a slice from a list.
/// Given two indices, I and K, the slice is the list containing the elements between the I'th and K'th element of the original list (both limits included).
/// Start counting the elements with 1.

fn slice<T: Copy>(vec: &Vec<T>, i: usize, k: usize) -> Vec<T> {
    let mut ret: Vec<T> = vec![];
    for (idx, v) in vec.iter().enumerate() {
        if i <= (idx + 1) && (idx + 1) <= k {
            ret.push(*v);
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let vec = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "k"];

        assert_eq!(slice(&vec, 3, 7), (vec!["c", "d", "e", "f", "g"]))
    }
}
