/// Remove the K'th element from a list.

fn remove_at<T: Copy>(vec: &Vec<T>, n: usize) -> Vec<T> {
    vec.iter()
        .enumerate()
        .filter_map(|(i, v)| if i + 1 == n { None } else { Some(v) })
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let vec = vec!['a', 'b', 'c', 'd'];
        assert_eq!(remove_at(&vec, 2), (vec!['a', 'c', 'd']));
    }
}
